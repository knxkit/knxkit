// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use std::net::Ipv4Addr;

use super::DescriptionType;
use crate::core::util::prelude::*;

// 3/8/2-7.5.4.5
#[derive(Debug, Clone)]
pub struct IpCurrentConfig {
    pub address: Ipv4Addr,
    pub netmask: u32,
    pub gateway: Ipv4Addr,
    pub dhcp_server: Ipv4Addr,
    pub assignment_method: u8,
}

impl IpCurrentConfig {
    pub fn parse(input: Input<'_>) -> Result<Self> {
        let (input, _length) = parse_u8(input)?;
        let (input, _) = parse_token(DescriptionType::IpCurConfig.to_u8().unwrap())(input)?;

        let (input, (address, netmask, gateway, dhcp_server, assignment_method, _)) =
            parse_tuple((
                parse_u32, parse_u32, parse_u32, parse_u32, parse_u8, parse_u8,
            ))(input)?;

        Ok((
            input,
            IpCurrentConfig {
                address: Ipv4Addr::from_bits(address),
                netmask,
                gateway: Ipv4Addr::from_bits(gateway),
                dhcp_server: Ipv4Addr::from_bits(dhcp_server),
                assignment_method,
            },
        ))
    }
}
