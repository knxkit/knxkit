// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use num_derive::{FromPrimitive, ToPrimitive};

use crate::core::util::prelude::*;

pub mod device_information;
pub mod ip_config;
pub mod ip_current_config;
pub mod knx_addresses;
pub mod manufacturer_data;
pub mod supported_services;

// 7.5.4
#[derive(Debug, Clone, PartialEq, FromPrimitive, ToPrimitive)]
pub enum DescriptionType {
    DeviceInfo = 0x01,
    SuppSvcFamilies = 0x02,
    IpConfig = 0x03,
    IpCurConfig = 0x04,
    KNXAddresses = 0x05,
    MFRData = 0xfe,
}

// 7.5.4
#[derive(Debug, Clone, PartialEq)]
pub struct Header {
    pub length: u8,
    pub type_: DescriptionType,
}

impl Header {
    pub fn parse(input: Input) -> Result<Self> {
        let (input, length) = parse_u8(input)?;
        let (input, type_) = parse_enum(8)(input)?;

        Ok((input, Header { length, type_ }))
    }
}
