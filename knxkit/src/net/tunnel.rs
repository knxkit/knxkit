// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use std::{
    collections::VecDeque,
    net::{Ipv4Addr, SocketAddrV4},
    sync::Arc,
    time::Duration,
};

use tokio::{
    sync::{mpsc, Notify},
    task::JoinHandle,
    time::{sleep_until, timeout, Instant},
};

use tracing::{debug, trace, warn};

use crate::{
    core::{address::IndividualAddress, cemi::CEMI},
    error::Error,
    net::{
        endpoint_udp::UdpEndpoint,
        frames::{
            connect::{CRDTunnel, CRITunnel, ConnectRequest, ConnectResponse, TunnelLayer},
            connectionstate::{ConnectionStateRequest, ConnectionStateResponse, ConnectionStatus},
            disconnect::{DisconnectRequest, DisconnectResponse},
            hpai::HPAI,
            tunneling::{Connection, TunnelingACK, TunnelingRequest},
            FramePayload, ServiceType,
        },
    },
};

macro_rules! break_on_err {
    ($x:expr, $level:ident, $message:literal) => {
        match ($x) {
            Ok(ok) => ok,
            Err(err) => {
                warn!("{} {:?}", $message, err);
                break;
            }
        }
    };
}

#[macro_export]
macro_rules! break_on_none {
    ($x:expr) => {
        if let Some(some) = $x {
            some
        } else {
            break;
        }
    };
}

#[macro_export]
macro_rules! try_parse {
    ($m:ident, $x:expr) => {
        if let Ok(parsed) = $m::try_parse($x) {
            parsed
        } else {
            warn!("cannot parse {}", stringify!($m));
            break;
        }
    };
}

enum State {
    Sleep,
    Wait,
}

pub struct Heartbeat {
    deadline: Instant,
    state: State,
    channel: u8,
    count: u8,
    request: ConnectionStateRequest,
}

// 03_08_02-5.4
impl Heartbeat {
    const SHORT: Duration = Duration::from_secs(10);
    const LONG: Duration = Duration::from_secs(60);

    pub fn new(channel: u8, control: HPAI) -> Self {
        let request = ConnectionStateRequest { channel, control };

        Self {
            channel,
            request,
            state: State::Sleep,
            deadline: Instant::now() + Heartbeat::LONG,
            count: 0,
        }
    }

    pub async fn recv(&mut self) -> Option<ConnectionStateRequest> {
        match self.state {
            State::Sleep => {
                sleep_until(self.deadline).await;
                self.state = State::Wait;
                self.deadline = Instant::now() + Heartbeat::SHORT;

                Some(self.request.clone())
            }
            State::Wait => {
                sleep_until(self.deadline).await;

                if self.count < 3 {
                    self.count += 1;
                    self.deadline = Instant::now() + Heartbeat::SHORT;

                    warn!(self.channel, self.count, "request retry");

                    Some(self.request.clone())
                } else {
                    None
                }
            }
        }
    }

    pub fn response(&mut self, response: ConnectionStateResponse) -> bool {
        if response.channel == self.channel {
            trace!(self.channel, "connection state response");

            self.state = State::Sleep;
            self.deadline = Instant::now() + Heartbeat::LONG;
            self.count = 0;

            response.status == ConnectionStatus::NoError
        } else {
            true
        }
    }
}

struct Tunneling {
    channel: u8,
    sequence: u8,
}

impl Tunneling {
    fn new(channel: u8) -> Self {
        Self {
            channel,
            sequence: 0,
        }
    }

    fn request(
        &mut self,
        request: TunnelingRequest,
    ) -> Option<(Option<Vec<u8>>, Option<TunnelingACK>)> {
        let ack = TunnelingACK {
            connection: Connection {
                channel: self.channel,
                sequence: request.connection.sequence,
            },
            status: 0x00,
        };

        if request.connection.sequence == self.sequence {
            debug!(self.sequence, ?request, "tunneling request in sequence");

            self.sequence = self.sequence.wrapping_add(1);

            Some((Some(request.cemi), Some(ack)))
        } else if request.connection.sequence == self.sequence.wrapping_sub(1) {
            debug!(self.sequence, ?request, "tunneling request one behind");

            Some((None, Some(ack)))
        } else {
            warn!(self.sequence, ?request, "out of sequence");

            None
        }
    }
}

#[derive(Debug)]
struct Current {
    count: u8,
    deadline: Instant,
    request: TunnelingRequest,
    ack: Arc<Notify>,
}

pub struct Sender {
    channel: u8,
    sequence: u8,
    queue: VecDeque<Egress>,
    active: Option<Current>,
}

impl Sender {
    const DELAY: Duration = Duration::from_secs(3);

    pub fn new(channel: u8) -> Self {
        Self {
            channel,
            sequence: 0,
            queue: VecDeque::new(),
            active: None,
        }
    }

    pub async fn recv(&mut self) -> Option<TunnelingRequest> {
        if let Some(active) = &mut self.active {
            sleep_until(active.deadline).await;

            if active.count < 2 {
                active.count += 1;
                active.deadline = Instant::now() + Sender::DELAY;

                debug!(?active, "sender retry");

                Some(active.request.clone())
            } else {
                debug!(active = ?self.active, "sender timeout");

                None
            }
        } else {
            if let Some(egress) = self.queue.pop_front() {
                let request = TunnelingRequest {
                    cemi: egress.payload,
                    connection: Connection {
                        channel: self.channel,
                        sequence: self.sequence,
                    },
                };

                self.active = Some(Current {
                    request: request.clone(),
                    ack: egress.ack,
                    count: 0,
                    deadline: Instant::now() + Sender::DELAY,
                });

                self.sequence = self.sequence.wrapping_add(1);

                debug!(active = ?self.active, "sender initial");

                Some(request)
            } else {
                futures::future::pending::<()>().await;
                unreachable!()
            }
        }
    }

    fn ack(&mut self, ack: TunnelingACK) {
        if self.channel == ack.connection.channel {
            if let Some(active) = &self.active {
                if active.request.connection.sequence == ack.connection.sequence {
                    debug!(?ack, "sender ack in sequence");

                    active.ack.notify_one();

                    self.active = None;
                }
            }
        }
    }

    fn enqueue(&mut self, egress: Egress) -> Result<(), Error> {
        // fixme check overflow
        self.queue.push_back(egress);

        Ok(())
    }
}

struct Egress {
    payload: Vec<u8>,
    ack: Arc<Notify>,
}

fn spawn_tunnel(channel: u8, endpoint: UdpEndpoint) -> TunnelConnection {
    let (egress_s, mut egress_r) = mpsc::channel::<Egress>(1);
    let (ingress_s, ingress_r) = mpsc::channel::<Vec<u8>>(1);

    let mut heartbeat = Heartbeat::new(channel, endpoint.hpai());
    let mut sender = Sender::new(channel);
    let mut tunneling = Tunneling::new(channel);

    let term = Arc::new(Notify::new());
    let term_local = term.clone();

    let handle = tokio::task::spawn(async move {
        loop {
            tokio::select! {
                heartbeat = heartbeat.recv() => {
                    let frame = break_on_none!(heartbeat).into();
                    debug!(?frame, "egress frame (data)");
                    break_on_err!(endpoint.send_control(frame).await, warn, "cannot send to control endpoint");
                }

                tunelling = sender.recv() => {
                    let frame = break_on_none!(tunelling).into();
                    debug!(?frame, "egress frame (data)");
                    break_on_err!(endpoint.send_data(frame).await, warn, "cannot send to data data endpoint");
                }

                egress = egress_r.recv() => {
                    let egress = break_on_none!(egress);
                    break_on_err!(sender.enqueue(egress), warn, "cannot enqueue to sender");
                }

                frame = endpoint.recv() => {
                    let (frame, _) = break_on_err!(frame, warn, "endpoint error");
                    debug!(?frame, "ingress frame");

                    match frame.service_type {
                        ServiceType::ConnectionStateResponse => {
                            let response = try_parse!(ConnectionStateResponse, frame);
                            heartbeat.response(response);
                        },

                        ServiceType::DisconnectRequest => {
                            let request = try_parse!(DisconnectRequest, frame);
                            if request.channel == channel {
                                debug!("disconnect request received");

                                let response = DisconnectResponse {
                                    channel,
                                    status: 0x00,
                                };

                                let _ = endpoint.send_control(response.into()).await;
                                break;
                            }
                        },

                        ServiceType::TunnelingRequest => {
                            let request = try_parse!(TunnelingRequest, frame);

                            let (cemi, ack) = break_on_none!(tunneling.request(request));

                            if let Some(cemi) = cemi {
                                break_on_err!(ingress_s.send(cemi).await, warn, "cannot send to ingress channel");
                            }

                            if let Some(ack) = ack {
                                break_on_err!(endpoint.send_data(ack.into()).await, warn, "cannot send to data endpoint");
                            }
                        }

                        ServiceType::TunnelingACK => {
                            let ack = try_parse!(TunnelingACK, frame);
                            sender.ack(ack);
                        }

                        unsupported => {
                            warn!(service = ?unsupported, "unexpected frame");
                        }
                    }
                }

                _ = term_local.notified() => {
                    break;
                }
            };
        }

        endpoint
    });

    TunnelConnection {
        channel,
        term,
        handle: Some(handle),
        sender: egress_s,
        receiver: ingress_r,
    }
}

pub struct TunnelConnection {
    channel: u8,
    term: Arc<Notify>,
    handle: Option<JoinHandle<UdpEndpoint>>,
    sender: mpsc::Sender<Egress>,
    receiver: mpsc::Receiver<Vec<u8>>,
}

impl TunnelConnection {
    pub async fn start(local: Ipv4Addr, peer: SocketAddrV4) -> Result<TunnelConnection, Error> {
        Self::start_ext(local, peer, TunnelLayer::Link, true).await
    }

    pub async fn start_ext(
        local: Ipv4Addr,
        peer: SocketAddrV4,
        layer: TunnelLayer,
        nat: bool,
    ) -> Result<TunnelConnection, Error> {
        let mut endpoint = UdpEndpoint::bind(local, peer, nat).await?;

        let request = ConnectRequest {
            control: endpoint.hpai(),
            data: endpoint.hpai(),
            cri: CRITunnel { layer },
        };

        endpoint.send_control(request.into()).await?;

        let (response, from) = timeout(Duration::from_secs(10), endpoint.recv()).await??;
        //.map_err(|_| Error::TunnelError(format!("timeout")))?;
        //            .ok_or(Error::TunnelError(format!("")))?; //FIXME - recv() must return Error

        if response.service_type == ServiceType::ConnectResponse {
            match ConnectResponse::<CRDTunnel>::try_parse(response)? {
                ConnectResponse::Ok {
                    channel, mut hpai, ..
                } => {
                    debug!(?peer, channel, ?hpai, "connect response ok");

                    if hpai == HPAI::UNSPECIFIED_UDP {
                        debug!(?from, channel, "response hpai unspecified, using 'from'");
                        hpai = HPAI::new_udp(from);
                    }

                    endpoint.set_data_peer(hpai.socket_addr());
                    let tunnel_connection = spawn_tunnel(channel, endpoint);

                    return Ok(tunnel_connection);
                }

                ConnectResponse::Error(code) => {
                    warn!(?peer, ?code, "connect response error");
                    return Err(Error::TunnelError(format!("connect response: {:?}", code)).into());
                }
            }
        } else {
            Err(Error::TunnelError(format!(
                "unexpected response: {:?}",
                response.service_type
            )))
        }
    }

    pub async fn terminate(mut self) -> () {
        let join = self.handle.take().unwrap();

        self.term.notify_one();

        if let Ok(control) = join.await {
            let hpai = control.hpai();

            let disconnect = DisconnectRequest {
                channel: self.channel,
                control: hpai,
            };

            debug!(self.channel, ?hpai, "disconnect request");

            if control.send_control(disconnect.into()).await.is_ok() {
                // blindly check the next incoming frame
                if let Ok(Ok((next, _))) = timeout(Duration::from_secs(1), control.recv()).await {
                    if let Ok(response) = DisconnectResponse::try_parse(next) {
                        if response.channel == self.channel {
                            debug!(self.channel, response.status, "disconnect response");
                        }
                    }
                }
            }
        }
    }

    async fn send(&self, cemi: Vec<u8>) -> Result<Arc<Notify>, Error> {
        let notify = Arc::new(Notify::new());

        self.sender
            .send(Egress {
                payload: cemi,
                ack: notify.clone(),
            })
            .await
            .map_err(|e| Error::TunnelError(e.to_string()))?;

        Ok(notify)
    }

    async fn recv(&mut self) -> Option<Vec<u8>> {
        self.receiver.recv().await
    }
}

impl crate::connection::KnxBusConnection for TunnelConnection {
    async fn send(&self, cemi: CEMI) -> Result<Arc<Notify>, Error> {
        debug!(?cemi, "egress cemi");

        self.send(cemi.try_into()?).await.map_err(Into::into)
    }

    async fn recv(&mut self) -> Option<Arc<CEMI>> {
        loop {
            if let Some(bytes) = self.recv().await {
                match CEMI::try_from(bytes.as_slice()) {
                    Ok(cemi) => {
                        debug!(?cemi, "ingress cemi");
                        break Some(Arc::new(cemi));
                    }
                    Err(error) => {
                        warn!("cannot decode cEMI: {:?}", error)
                    }
                }
            } else {
                break None;
            }
        }
    }

    async fn terminate(self) {
        self.terminate().await;
    }

    fn address(&self) -> IndividualAddress {
        todo!()
    }
}

impl Drop for TunnelConnection {
    fn drop(&mut self) {
        if self.handle.is_some() {
            warn!("tunnel connection dropped without being terminated");
        }
    }
}
