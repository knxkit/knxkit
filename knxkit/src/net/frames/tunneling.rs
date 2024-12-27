// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use crate::net::frames::{FramePayload, ServiceType};

use crate::core::util::prelude::*;

// 03_08_04-4.4.5
#[derive(Debug, Clone)]
pub struct Connection {
    // length and status are handled by TunnelingRequest and TunnelingACK
    pub channel: u8,
    pub sequence: u8,
}

impl Connection {
    pub fn parse(input: Input) -> Result<Self> {
        let (input, (channel, sequence)) = parse_tuple((parse_u8, parse_u8))(input)?;

        Ok((input, Connection { channel, sequence }))
    }

    pub fn gen<W: Write>(&self) -> impl SerializeFn<W> {
        gen_tuple((gen_u8(self.channel), gen_u8(self.sequence)))
    }
}

// 03_08_04-4.4.6
#[derive(Debug, Clone)]
pub struct TunnelingRequest {
    pub connection: Connection,
    pub cemi: Vec<u8>,
}

impl FramePayload for TunnelingRequest {
    const SERVICE_TYPE: ServiceType = ServiceType::TunnelingRequest;

    fn parse(input: Input) -> Result<Self> {
        let (input, (_length, connection, _reserved, cemi)) = parse_tuple((
            parse_u8,
            Connection::parse,
            parse_u8,
            parse_take(input.len() - 4),
        ))(input)?;

        Ok((
            input,
            TunnelingRequest {
                connection,
                cemi: cemi.into(),
            },
        ))
    }

    fn gen<W: Write>(&self) -> impl SerializeFn<W> {
        gen_tuple((
            // gen_u8(4 + self.cemi.len() as u8),
            gen_u8(4),
            self.connection.gen(),
            gen_u8(0x00),
            gen_slice(&self.cemi),
        ))
    }
}

// 03_08_04-4.4.7
#[derive(Debug, Clone)]
pub struct TunnelingACK {
    pub connection: Connection,
    pub status: u8,
}

impl FramePayload for TunnelingACK {
    const SERVICE_TYPE: ServiceType = ServiceType::TunnelingACK;

    fn parse(input: Input) -> Result<Self> {
        let (input, (_length, connection, status)) =
            parse_tuple((parse_u8, Connection::parse, parse_u8))(input)?;

        Ok((input, TunnelingACK { connection, status }))
    }

    fn gen<W: Write>(&self) -> impl SerializeFn<W> {
        gen_tuple((gen_u8(0x04), self.connection.gen(), gen_u8(self.status)))
    }
}
