// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use crate::net::frames::{hpai::HPAI, FramePayload, ServiceType};

use crate::core::util::prelude::*;

// 03_08_02-7.8.5
#[derive(Debug, Clone)]
pub struct DisconnectRequest {
    pub channel: u8,
    pub control: HPAI,
}

impl FramePayload for DisconnectRequest {
    const SERVICE_TYPE: ServiceType = ServiceType::DisconnectRequest;

    fn parse(input: Input) -> Result<Self> {
        let (input, (channel, _reserved, control)) =
            parse_tuple((parse_u8, parse_u8, HPAI::parse))(input)?;

        Ok((input, DisconnectRequest { channel, control }))
    }

    fn gen<W: Write>(&self) -> impl SerializeFn<W> {
        gen_tuple((gen_u8(self.channel), gen_u8(0x00), self.control.gen()))
    }
}

// 03_08_02-7.8.6
#[derive(Debug, Clone)]
pub struct DisconnectResponse {
    pub channel: u8,
    pub status: u8,
}

impl FramePayload for DisconnectResponse {
    const SERVICE_TYPE: ServiceType = ServiceType::DisconnectResponse;

    fn parse(input: Input) -> Result<Self> {
        let (input, (channel, status)) = parse_tuple((parse_u8, parse_u8))(input)?;
        Ok((input, DisconnectResponse { channel, status }))
    }

    fn gen<W: Write>(&self) -> impl SerializeFn<W> {
        gen_tuple((gen_u8(self.channel), gen_u8(self.status)))
    }
}
