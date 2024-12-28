// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use num_traits::FromPrimitive;

use crate::core::{
    address::{DestinationAddress, GroupAddress, IndividualAddress},
    npdu::NPDU,
    util::prelude::*,
};

bitflags::bitflags! {
    #[derive(Debug, Clone)]
    pub struct CEMIFlags: u8 {
        const FT = 0b10000000;
        const R =  0b00100000;
        const SB = 0b00010000;
        const A =  0b00000010;
        const C =  0b00000001;
    }
}
// 03_03_02-2.2.3
#[derive(Clone, Copy, Debug, num_derive::ToPrimitive, num_derive::FromPrimitive, PartialEq)]
pub enum Priority {
    Low = 0b11,
    Normal = 0b01,
    Urgent = 0b10,
    System = 0b00,
}

// 03_06_03-4.1.5.3
#[derive(Debug, Clone)]
pub struct CEMI {
    pub mc: u8,
    pub flags: CEMIFlags,
    pub hops: u8,
    pub prio: Priority,
    pub source: IndividualAddress,
    pub destination: DestinationAddress,
    pub npdu: NPDU,
}

impl CEMI {
    pub fn parse(input: Input) -> Result<Self>
    where
        Self: Sized,
    {
        let (input, mc) = parse_u8(input)?;

        // skip additional information
        let (input, adil_length) = parse_u8(input)?;
        let (input, _) = parse_take(adil_length)(input)?;

        let (input, ctrl_1) = parse_u8(input)?;
        let flags = CEMIFlags::from_bits_truncate(ctrl_1);
        let prio = Priority::from_u8(ctrl_1 >> 2 & 0x03).unwrap();

        let (input, ctrl_2) = parse_u8(input)?;
        let hops = (ctrl_2 >> 4) & 0x07;

        let (input, source) = IndividualAddress::parse(input)?;

        let (input, destination) = if ctrl_2 & 0b10000000_u8 > 0 {
            GroupAddress::parse(input).map_value(|a| DestinationAddress::Group(a))
        } else {
            IndividualAddress::parse(input).map_value(|a| DestinationAddress::Individual(a))
        }?;

        let (input, npdu) = NPDU::parse(input, destination)?;

        Ok((
            input,
            CEMI {
                mc,
                flags,
                prio,
                hops,
                source,
                destination,
                npdu,
            },
        ))
    }

    fn gen<W: Write>(&self) -> impl SerializeFn<W> {
        let ctrl_1 = self.flags.bits() | ((self.prio.to_u8().unwrap() & 0x03) << 2);
        let mut ctrl_2 = (self.hops & 0x07) << 4;
        if let DestinationAddress::Group(_) = self.destination {
            ctrl_2 = ctrl_2 | 0x80;
        }

        gen_tuple((
            gen_u8(self.mc),
            gen_u8(0),
            gen_u8(ctrl_1),
            gen_u8(ctrl_2),
            self.source.gen(),
            gen_u16(self.destination.as_u16()),
            self.npdu.gen(),
        ))
    }
}

impl TryFrom<CEMI> for Vec<u8> {
    type Error = Error;

    fn try_from(cemi: CEMI) -> std::result::Result<Vec<u8>, Self::Error> {
        let mut bytes = Vec::new();

        match cookie_factory::gen(cemi.gen(), &mut bytes) {
            Ok(_) => Ok(bytes),
            Err(w) => Err(w.into()),
        }
    }
}

impl TryFrom<&[u8]> for CEMI {
    type Error = Error;

    fn try_from(input: &[u8]) -> std::result::Result<CEMI, Self::Error> {
        CEMI::parse(input).finish().map(|(_, value)| value)
    }
}
