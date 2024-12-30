// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use std::net::Ipv4Addr;

use futures::stream::Stream;

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

pub async fn search(
    local: Ipv4Addr,
    nat: bool,
) -> Result<impl Stream<Item = SearchResponse>, Error> {
    let endpoint = UdpEndpoint::bind(local, UdpEndpoint::multicast_peer(), nat).await?;

    let request = SearchRequest {
        hpai: endpoint.hpai(),
    };

    endpoint.send_control(request.into()).await?;

    let stream = async_stream::stream! {
        loop {
            if let Ok((response, _)) = endpoint.recv().await  {
                if response.service_type == ServiceType::SearchResponse {
                    if let Ok(response) = SearchResponse::try_parse(response) {
                        yield response;
                    }
                }
            }else{
                break;
            }
        }
    };

    Ok(stream)
}
