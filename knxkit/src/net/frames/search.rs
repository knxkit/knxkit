// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use crate::net::frames::{
    dib::{device_information::DeviceInformation, supported_services::ServiceEntry},
    hpai::HPAI,
    FramePayload, ServiceType,
};

use crate::core::util::prelude::*;

// 3/8/2-7.6.1
#[derive(Debug, Clone, PartialEq)]
pub struct SearchRequest {
    pub hpai: HPAI,
}

impl FramePayload for SearchRequest {
    const SERVICE_TYPE: ServiceType = ServiceType::SearchRequest;

    fn parse(input: Input) -> Result<Self> {
        let (input, hpai) = HPAI::parse(input)?;
        Ok((input, SearchRequest { hpai }))
    }

    fn gen<W: Write>(&self) -> impl SerializeFn<W> {
        super::hpai::HPAI::gen(&self.hpai)
    }
}

// 3/8/2-7.6.2
#[derive(Debug, Clone)]
pub struct SearchResponse {
    pub hpai: HPAI,
    pub device_information: DeviceInformation,
    pub supported_services: Vec<ServiceEntry>,
}

impl FramePayload for SearchResponse {
    const SERVICE_TYPE: ServiceType = ServiceType::SearchResponse;

    fn parse(input: Input<'_>) -> Result<Self> {
        let (input, (hpai, device_information, supported_services)) = parse_tuple((
            HPAI::parse,
            DeviceInformation::parse,
            ServiceEntry::parse_many,
        ))(input)?;

        Ok((
            input,
            SearchResponse {
                hpai,
                device_information,
                supported_services,
            },
        ))
    }
}
