// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use crate::core::{
    address::{DestinationAddress, Domain},
    apdu::APDU,
    util::prelude::*,
};

#[derive(Debug, Clone)]
pub enum TPDU {
    DataBroadcast(APDU),
    DataGroup(APDU),
    DataIndividual(APDU),
    DataTagGroup(APDU),
    DataConnected(u8, APDU),

    // control
    Connect,
    Disconnect,
    Ack(u8),
    Nack(u8),
}

impl TPDU {
    pub fn parse(input: Input, destination: DestinationAddress, npdu_length: u8) -> Result<Self> {
        let (input, tpci) = parse_u8(input)?;

        let apdu = |prefix| APDU::parse(prefix, input, npdu_length).map(|(_input, apdu)| apdu);

        let parts = (
            tpci & 0b11000000 >> 6, // ctl + numbered
            tpci & 0b00111100 >> 2, // seq
            tpci & 0b00000011,
            destination.domain(),
        );

        #[rustfmt::skip]
        let tpdu = match parts {
            (0b00, 0b0000, suffix, Domain::Broadcast) => TPDU::DataBroadcast(apdu(suffix)?),
            (0b00, 0b0000, suffix, Domain::Multicast) => TPDU::DataGroup(apdu(suffix)?),
            (0b00, 0b0001, suffix, _                ) => TPDU::DataTagGroup(apdu(suffix)?),
            (0b00, 0b0000, suffix, Domain::Unicast  ) => TPDU::DataIndividual(apdu(suffix)?),
            (0b01, seq   , suffix, _                ) => TPDU::DataConnected(seq, apdu(suffix)?),

            (0b10, 0b0000, 0b00,   _) => TPDU::Connect,
            (0b10, 0b0000, 0b01,   _) => TPDU::Disconnect,
            (0b11, seq   , 0b10,   _) => TPDU::Ack(seq),
            (0b11, seq   , 0b11,   _) => TPDU::Nack(seq),

            _ => return Err(nom::Err::Failure(Error::general("unexpected TPCI", &[tpci]))),
        };

        Ok((input, tpdu))
    }

    pub fn gen<W: Write>(&self) -> impl SerializeFn<W> + use<'_, W> { 
        #[rustfmt::skip]
        let (prefix, seq, suffix, apdu) = match self {
            TPDU::DataBroadcast(apdu)      => (0b00, 0b0000,  apdu.suffix(), Some(apdu.gen())),
            TPDU::DataGroup(apdu)          => (0b00, 0b0000,  apdu.suffix(), Some(apdu.gen())),
            TPDU::DataTagGroup(apdu)       => (0b00, 0b0001,  apdu.suffix(), Some(apdu.gen())),
            TPDU::DataIndividual(apdu)     => (0b00, 0b0000,  apdu.suffix(), Some(apdu.gen())),
            TPDU::DataConnected(seq, apdu) => (0b01, *seq,    apdu.suffix(), Some(apdu.gen())),

            TPDU::Connect    => (0b10, 0b0000, 0b00, None),
            TPDU::Disconnect => (0b10, 0b0000, 0b01, None),
            TPDU::Ack(seq)   => (0b11, *seq,   0b10, None),
            TPDU::Nack(seq)  => (0b11, *seq,   0b11, None),
        };

        let apci = (prefix & 0b11 << 6) | (seq & 0b1111 << 2) | (suffix & 0b11);

        move |context| {
            if let Some(apdu) = &apdu {
                gen_tuple((gen_u8(apci), apdu))(context)
            } else {
                gen_u8(apci)(context)
            }
        }
    }
}
