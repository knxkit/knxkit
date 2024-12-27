// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use crate::core::{address::DestinationAddress, tpdu::TPDU, util::prelude::*};

#[derive(Debug, Clone)]
pub struct NPDU {
    // pub length: u8,
    pub tpdu: TPDU,
}

impl NPDU {
    pub fn parse(input: Input, destination: DestinationAddress) -> Result<Self> {
        // 03_03_03-2.1 (octet 5)
        let (input, length) = parse_u8(input)?;

        // 1 MSB bit and 4 LSB bits belong to L2 ?????
        let _address_type = (length & 0x80) >> 7;
        let _length = length & 0x0f;

        let (input, tpdu) = TPDU::parse(input, destination, length)?;

        Ok((input, NPDU { tpdu }))
    }

    pub fn gen<W: Write>(&self) -> impl SerializeFn<W> {
        let tpdu = gen_simple(self.tpdu.gen(), Vec::new()).expect("cannot generate TPDU");

        gen_tuple((gen_u8((tpdu.len() - 1) as u8), gen_slice(tpdu)))
    }
}
