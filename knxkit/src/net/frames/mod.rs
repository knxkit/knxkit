// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

pub mod connect;
pub mod connectionstate;
pub mod describe;
pub mod dib;
pub mod disconnect;
pub mod hpai;
pub mod search;
pub mod tunneling;

use crate::core::util::prelude::*;

#[derive(PartialEq, Debug, Clone, num_derive::ToPrimitive, num_derive::FromPrimitive)]
pub enum ServiceType {
    SearchRequest = 0x0201,
    SearchResponse = 0x0202,
    DescriptionRequest = 0x0203,
    DescriptionResponse = 0x0204,
    ConnectRequest = 0x0205,
    ConnectResponse = 0x0206,
    ConnectionStateRequest = 0x0207,
    ConnectionStateResponse = 0x0208,
    DisconnectRequest = 0x0209,
    DisconnectResponse = 0x020a,

    TunnelingRequest = 0x0420,
    TunnelingACK = 0x0421,
}

#[derive(Debug)]
pub struct Frame {
    pub service_type: ServiceType,
    pub payload: Vec<u8>,
}

const KNXNET_VERSION: u8 = 0x10;
const LENGTH: u8 = 0x06;

impl Frame {
    fn parse(input: Input) -> Result<Frame> {
        let (input, length0) = parse_u8(input)?;
        let (input, _version) = parse_u8(input)?;
        // FIXME check length and version

        let (input, service_type) = parse_enum(16)(input)?;
        let (input, length) = parse_u16(input)?;

        let (_input, payload) = parse_take(length - length0 as u16)(input)?;

        Ok((
            input,
            Frame {
                service_type,
                payload: payload.into(),
            },
        ))
    }

    fn gen<W: Write>(&self) -> impl SerializeFn<W> + use<'_, W> {
        gen_tuple((
            gen_u8(LENGTH),
            gen_u8(KNXNET_VERSION),
            gen_enum(16, &self.service_type),
            gen_u16(self.payload.len() as u16 + LENGTH as u16),
            gen_slice(&self.payload),
        ))
    }
}

impl TryFrom<&[u8]> for Frame {
    type Error = Error;

    fn try_from(input: &[u8]) -> std::result::Result<Frame, Self::Error> {
        Frame::parse(input).finish().map(|(_, frame)| frame)
    }
}

impl TryFrom<Frame> for Vec<u8> {
    type Error = Error;

    fn try_from(frame: Frame) -> std::result::Result<Vec<u8>, Self::Error> {
        let mut bytes = Vec::new();

        match cookie_factory::gen(frame.gen(), &mut bytes) {
            Ok(_) => Ok(bytes),
            Err(w) => Err(w.into()),
        }
    }
}

impl<P: FramePayload> From<P> for Frame {
    fn from(input: P) -> Self {
        let mut payload = Vec::new();

        if let Err(error) = cookie_factory::gen(input.gen(), &mut payload) {
            panic!("encode error: {:?}", error);
        }

        Frame {
            service_type: P::SERVICE_TYPE,
            payload,
        }
    }
}

pub trait FramePayload {
    const SERVICE_TYPE: ServiceType;

    fn parse(_input: Input) -> Result<Self>
    where
        Self: Sized;

    fn gen<W: Write>(&self) -> impl SerializeFn<W> {
        |_| Err(GenError::NotYetImplemented)
    }

    fn try_parse(frame: Frame) -> std::result::Result<Self, Error>
    where
        Self: Sized,
    {
        if frame.service_type != Self::SERVICE_TYPE {
            return Err(Error::general("invalid service type", frame.payload));
        }

        Self::parse(&frame.payload).finish().map(|(_, v)| v)
    }
}

mod test {
    #[test]
    fn test() {
        let input = &hex::decode("06100206001411000801c0a808020e57040411fb").unwrap();
        let _parsed = super::Frame::try_from(input.as_slice()).expect("parse");
    }
}
