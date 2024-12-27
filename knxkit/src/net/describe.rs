// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use std::{future::Future, net::Ipv4Addr, net::SocketAddrV4};

use crate::{
    error::Error,
    net::{
        endpoint_udp::UdpEndpoint,
        frames::{
            describe::{DescriptionRequest, DescriptionResponse},
            FramePayload, ServiceType,
        },
    },
};

pub fn describe(
    local: Ipv4Addr,
    peer: SocketAddrV4,
) -> impl Future<Output = Result<DescriptionResponse, Error>> {
    describe_ext(local, peer, false)
}

pub async fn describe_ext(
    local: Ipv4Addr,
    peer: SocketAddrV4,
    nat: bool,
) -> Result<DescriptionResponse, Error> {
    let endpoint = UdpEndpoint::bind(local, peer, nat).await?;

    let request = DescriptionRequest {
        hpai: endpoint.hpai(),
    }
    .into();
    endpoint.send_control(request).await?;

    let (resp, _) = endpoint.recv().await.unwrap();

    if resp.service_type == ServiceType::DescriptionResponse {
        Ok(DescriptionResponse::try_parse(resp)?)
    } else {
        Err(Error::General(format!(
            "unexpected response: {:?}",
            resp.service_type
        )))
    }
}
