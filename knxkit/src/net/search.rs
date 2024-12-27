// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use std::{future::Future, net::Ipv4Addr, time::Duration};

use tokio::time::Instant;

use crate::{
    error::Error,
    net::{
        endpoint_udp::UdpEndpoint,
        frames::{
            search::{SearchRequest, SearchResponse},
            FramePayload, ServiceType,
        },
    },
};

pub fn search(local: Ipv4Addr) -> impl Future<Output = Result<Vec<SearchResponse>, Error>> {
    search_ext(local, false, Duration::from_secs(5))
}

pub async fn search_ext(
    local: Ipv4Addr,
    nat: bool,
    wait: Duration,
) -> Result<Vec<SearchResponse>, Error> {
    let endpoint = UdpEndpoint::bind(local, UdpEndpoint::multicast_peer(), nat).await?;

    let request = SearchRequest {
        hpai: endpoint.hpai(),
    }
    .into();
    endpoint.send_control(request).await?;

    let mut found = Vec::new();

    let deadline = Instant::now() + wait;

    loop {
        tokio::select! {
            rec = endpoint.recv() => {
                let (response, _) = rec?;
                if response.service_type == ServiceType::SearchResponse {
                    let response = SearchResponse::try_parse(response)?;
                    found.push(response);
                }
            },

            _ = tokio::time::sleep_until(deadline) => {
                break;
            }
        }
    }

    Ok(found)
}
