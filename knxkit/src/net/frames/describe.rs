// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use super::dib::{
    device_information::DeviceInformation, ip_config::IpConfig, ip_current_config::IpCurrentConfig,
    knx_addresses::KNXAddresses, manufacturer_data::ManufacturerData,
    supported_services::ServiceEntry, DescriptionType, Header,
};
use crate::core::util::prelude::*;
use crate::net::frames::hpai::HPAI;
use crate::net::frames::{FramePayload, ServiceType};

// 3/8/2-7.7.1
#[derive(Debug, Clone, PartialEq)]
pub struct DescriptionRequest {
    pub hpai: HPAI,
}

impl FramePayload for DescriptionRequest {
    const SERVICE_TYPE: ServiceType = ServiceType::DescriptionRequest;

    fn parse(input: Input) -> Result<Self> {
        let (input, hpai) = HPAI::parse(input)?;
        Ok((input, DescriptionRequest { hpai }))
    }

    fn gen<W: Write>(&self) -> impl SerializeFn<W> {
        super::hpai::HPAI::gen(&self.hpai)
    }
}

// 3/8/2-7.7.1
#[derive(Debug, Clone)]
pub struct DescriptionResponse {
    pub device_information: DeviceInformation,
    pub supported_services: Vec<ServiceEntry>,
    pub ip_config: Option<IpConfig>,
    pub ip_cur_config: Option<IpCurrentConfig>,
    pub knx_addresses: Option<KNXAddresses>,
    pub mfr_data: Option<ManufacturerData>,
}

impl FramePayload for DescriptionResponse {
    const SERVICE_TYPE: ServiceType = ServiceType::DescriptionResponse;

    fn parse(input: Input<'_>) -> Result<Self> {
        let (input, (device_information, supported_services)) =
            parse_tuple((DeviceInformation::parse, ServiceEntry::parse_many))(input)?;

        let mut response = DescriptionResponse {
            device_information,
            supported_services,
            ip_config: None,
            ip_cur_config: None,
            knx_addresses: None,
            mfr_data: None,
        };

        let mut input = input;
        while input.len() > 0 {
            let (_, header) = Header::parse(input)?;

            input = match header.type_ {
                DescriptionType::IpConfig => {
                    let (input, ip_config) = IpConfig::parse(input)?;
                    response.ip_config = Some(ip_config);
                    input
                }

                DescriptionType::IpCurConfig => {
                    let (input, ip_cur_config) = IpCurrentConfig::parse(input)?;
                    response.ip_cur_config = Some(ip_cur_config);
                    input
                }

                DescriptionType::KNXAddresses => {
                    let (input, knx_addresses) = KNXAddresses::parse(input)?;
                    response.knx_addresses = Some(knx_addresses);
                    input
                }

                DescriptionType::MFRData => {
                    let (input, mfr_data) = ManufacturerData::parse(input)?;
                    response.mfr_data = Some(mfr_data);
                    input
                }

                _ => {
                    // unknown DIB, skip bytes
                    parse_take(header.length)(input)?.0
                }
            };
        }

        Ok((input, response))
    }
}
