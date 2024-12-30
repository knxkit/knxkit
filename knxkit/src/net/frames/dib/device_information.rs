// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use std::net::Ipv4Addr;

use macaddr::MacAddr6;
use strum::Display;

use super::DescriptionType;
use crate::{core::address::IndividualAddress, core::util::prelude::*};

// 3/8/2-7.5.4.2
#[derive(PartialEq, Debug, Clone, ToPrimitive, FromPrimitive, Display)]
pub enum Medium {
    TP1 = 0x02,
    PL110 = 0x04,
    RF = 0x10,
    IP = 0x20,
}

// 3/8/2-7.5.4.2
#[derive(Debug, Clone, PartialEq)]
pub struct Status(pub u8);

impl Status {
    pub fn is_programming(&self) -> bool {
        self.0 & 0x01 != 0
    }
}

// 3/8/2-7.5.4.2
#[derive(Clone, PartialEq)]
pub struct Project(u16);

impl std::fmt::Debug for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Project")
            .field("project", &(self.0 >> 4))
            .field("installation", &(self.0 & 0x0f))
            .finish()
    }
}

// 3/8/2-7.5.4.2
#[derive(Debug, Clone)]
pub struct DeviceInformation {
    pub medium: Medium,
    pub address: IndividualAddress,
    pub project: Project,
    pub serial: [u8; 6],
    pub ip_multicast: Ipv4Addr,
    pub mac: MacAddr6,
    pub name: String,
    pub status: Status,
}

impl DeviceInformation {
    pub fn parse(input0: Input<'_>) -> Result<Self> {
        let (input, _) = parse_u8(input0)?;
        let (input, _) = parse_token(DescriptionType::DeviceInfo.to_u8().unwrap())(input)?;
        let (input, medium) = parse_enum(8)(input)?;
        let (input, status) = parse_u8(input).map_value(|v| Status(v))?;
        let (input, address) = IndividualAddress::parse(input)?;
        let (input, project) = parse_u16(input).map_value(|v| Project(v))?;

        let (input, serial) = parse_take(6_usize)(input).and_then_value(|name| {
            let serial = name
                .try_into()
                .map_err(|_| Err::Failure(Error::general("Incorrect length: serial", input0)))?;

            Ok(serial)
        })?;

        let (input, ip_multicast) = parse_u32(input).map_value(|v| Ipv4Addr::from(v))?;

        let (input, mac) = parse_take(6_usize)(input).and_then_value(|name| {
            let mac: [u8; 6] = name
                .try_into()
                .map_err(|_| Err::Failure(Error::general("Incorrect length: mac", input0)))?;

            Ok(MacAddr6::from(mac))
        })?;

        let (input, name) = parse_take(30_usize)(input).and_then_value(|name| {
            let name: &[u8; 30] = name.try_into().map_err(|_| {
                Err::Failure(Error::general("Incorrect length:  device name", input0))
            })?;

            let end = name.iter().position(|&c| c == b'\0').unwrap_or(name.len());
            let name = String::from_utf8_lossy(&name[..end]).into_owned();

            Ok(name)
        })?;

        let device_information = DeviceInformation {
            medium,
            address,
            status,
            project,
            serial,
            ip_multicast,
            mac,
            name,
        };

        Ok((input, device_information))
    }
}
