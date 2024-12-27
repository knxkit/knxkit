// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use crate::core::address::IndividualAddress;
use crate::core::util::prelude::*;
use crate::net::frames::hpai::HPAI;
use crate::net::frames::{FramePayload, ServiceType};

pub trait CRI {
    fn parse(_input: Input) -> Result<Self>
    where
        Self: Sized;

    fn gen<W: Write>(&self) -> impl SerializeFn<W> {
        |_| Err(GenError::NotYetImplemented)
    }
}

// 03_08_02-7.8.1
#[derive(Debug, Clone)]
pub struct ConnectRequest<C>
where
    C: CRI,
{
    pub control: HPAI,
    pub data: HPAI,
    pub cri: C,
}

impl<C: CRI> FramePayload for ConnectRequest<C> {
    const SERVICE_TYPE: ServiceType = ServiceType::ConnectRequest;

    fn parse(input: Input) -> Result<ConnectRequest<C>> {
        let (input, (control, data, cri)) =
            parse_tuple((HPAI::parse, HPAI::parse, C::parse))(input)?;

        Ok((input, ConnectRequest { control, data, cri }))
    }

    fn gen<W: std::io::Write>(&self) -> impl SerializeFn<W> {
        gen_tuple((self.control.gen(), self.data.gen(), self.cri.gen()))
    }
}

// 03_08_04-4.4.3
#[derive(PartialEq, Debug, Clone, ToPrimitive, FromPrimitive)]
pub enum TunnelLayer {
    Link = 0x02,
    Raw = 0x04,
    BusMonitor = 0x80,
}

// 03_08_04-4.4.3
#[derive(Debug, Clone)]
pub struct CRITunnel {
    pub layer: TunnelLayer,
}

impl CRI for CRITunnel {
    fn parse(input: Input) -> Result<CRITunnel> {
        let (input, (_, _, layer, _)) =
            parse_tuple((parse_u8, parse_u8, parse_enum(8), parse_u8))(input)?;

        Ok((input, CRITunnel { layer }))
    }

    fn gen<W: std::io::Write>(&self) -> impl SerializeFn<W> {
        gen_tuple((
            gen_u8(0x04),
            gen_u8(0x04),
            gen_enum(8, &self.layer),
            gen_u8(0),
        ))
    }
}
// 03_08_02-7.8.2
#[derive(PartialEq, Debug, Clone, ToPrimitive, FromPrimitive)]
pub enum ConnectStatus {
    NoError = 0x00,
    ConnectionType = 0x22,
    ConnectionOption = 0x23,
    NoMoreConnections = 0x24,
}

pub trait CRD {
    fn parse(_input: Input) -> Result<Self>
    where
        Self: Sized;

    fn gen<W: Write>(&self) -> impl SerializeFn<W> {
        |_| Err(GenError::NotYetImplemented)
    }
}

// 03_08_02-7.8.2
#[derive(Debug, Clone)]
pub enum ConnectResponse<C: CRD> {
    Ok { channel: u8, hpai: HPAI, crd: C },
    Error(ConnectStatus),
}

// 03_08_04-4.4.4
#[derive(Debug, Clone)]
pub struct CRDTunnel {
    address: IndividualAddress,
}

impl<C: CRD> FramePayload for ConnectResponse<C> {
    const SERVICE_TYPE: ServiceType = ServiceType::ConnectResponse;

    fn parse(input: Input) -> Result<ConnectResponse<C>> {
        let (input, (channel, status)) = parse_tuple((parse_u8, parse_enum(8)))(input)?;

        if status == ConnectStatus::NoError {
            let (input, (hpai, crd)) = parse_tuple((HPAI::parse, C::parse))(input)?;

            Ok((input, ConnectResponse::Ok { channel, hpai, crd }))
        } else {
            Ok((input, ConnectResponse::Error(status)))
        }
    }

    fn gen<W: std::io::Write>(&self) -> impl SerializeFn<W> {
        move |context| match self {
            ConnectResponse::Ok { channel, hpai, crd } => gen_tuple((
                gen_u8(*channel),
                gen_enum(8, &ConnectStatus::NoError),
                hpai.gen(),
                crd.gen(),
            ))(context),
            ConnectResponse::Error(code) => gen_tuple((gen_u8(0), gen_enum(8, code)))(context),
        }
    }
}

impl CRD for CRDTunnel {
    fn parse(input: Input) -> Result<CRDTunnel> {
        let (input, (_, _, address)) =
            parse_tuple((parse_u8, parse_u8, IndividualAddress::parse))(input)?;
        Ok((input, CRDTunnel { address }))
    }

    fn gen<W: std::io::Write>(&self) -> impl SerializeFn<W> {
        gen_tuple((gen_u8(0x04), gen_u8(0x04), self.address.gen()))
    }
}
