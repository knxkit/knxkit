// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use futures::Future;
use std::{sync::Arc, time::Duration};
use tokio::sync::Notify;

use crate::{
    connection::KnxBusConnection,
    core::{
        address::{DestinationAddress, GroupAddress, IndividualAddress},
        apdu::{Service, APDU},
        cemi::{CEMIFlags, CEMI},
        npdu::NPDU,
        tpdu::TPDU,
        DataPoint,
    },
    error::Error,
};

pub trait GroupOps {
    fn group_read(
        &mut self,
        group: GroupAddress,
        timeout: Duration,
    ) -> impl Future<Output = Result<DataPoint, Error>>;

    fn group_write(
        &mut self,
        group: GroupAddress,
        dp: DataPoint,
    ) -> impl Future<Output = Result<Arc<Notify>, Error>>;
}

fn cemi(destination: DestinationAddress, apdu: APDU) -> CEMI {
    CEMI {
        mc: 0x11,
        flags: CEMIFlags::FT | CEMIFlags::R | CEMIFlags::SB,
        hops: 0x06,
        prio: 0x03,
        source: IndividualAddress::new_zero(),
        destination,
        npdu: NPDU {
            tpdu: TPDU::DataGroup(apdu),
        },
    }
}

impl<T: KnxBusConnection> GroupOps for T {
    async fn group_read(
        &mut self,
        group: GroupAddress,
        timeout: Duration,
    ) -> Result<DataPoint, Error> {
        let apdu = APDU {
            service: crate::core::apdu::Service::GroupValueRead,
            data: None,
        };

        let destination = DestinationAddress::Group(group);
        let cemi = cemi(destination, apdu);

        self.send(cemi).await?;
        self.fish_for(timeout, |cemi| match &cemi.npdu.tpdu {
            TPDU::DataGroup(APDU {
                service: Service::GroupValueResponse,
                data,
                ..
            }) if cemi.destination == destination => Some(data.clone()),

            _ => None,
        })
        .await
        .ok_or_else(|| Error::General(format!("timeout")))?
        .ok_or_else(|| Error::General(format!("no data")))
    }

    async fn group_write(
        &mut self,
        group: GroupAddress,
        data: DataPoint,
    ) -> Result<Arc<Notify>, Error> {
        let apdu = APDU {
            service: crate::core::apdu::Service::GroupValueWrite,
            data: Some(data),
        };

        let destination = DestinationAddress::Group(group);
        let cemi = cemi(destination, apdu);

        let notify = self.send(cemi).await?;

        Ok(notify)
    }
}

pub trait FishOps {
    fn fish_for<T, P: Fn(&CEMI) -> Option<T> + Send>(
        &mut self,
        timeout: std::time::Duration,
        p: P,
    ) -> impl Future<Output = Option<T>>;
}

impl<C: KnxBusConnection> FishOps for C {
    async fn fish_for<T, P: Fn(&CEMI) -> Option<T> + Send>(
        &mut self,
        timeout: std::time::Duration,
        p: P,
    ) -> Option<T> {
        let deadline = tokio::time::Instant::now() + timeout;

        loop {
            tokio::select! {
                _ = tokio::time::sleep_until(deadline) => {
                    break None
                }

                frame = self.recv() => {
                    if let Some(cemi) = frame {
                        let result = p(&cemi);
                        if result.is_some() {
                            break result;
                        }
                    }else{
                        break None;
                    }
                },

            };
        }
    }
}
