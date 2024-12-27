// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use super::DescriptionType;
use crate::core::{address::IndividualAddress, util::prelude::*};

// 3/8/2-7.5.4.6
#[derive(Clone, Debug)]
pub struct KNXAddresses {
    pub address: IndividualAddress,
    pub additional: Vec<IndividualAddress>,
}

impl KNXAddresses {
    pub fn parse(input: Input<'_>) -> Result<Self> {
        let (input, length) = parse_u8(input)?;
        let (input, _) = parse_token(DescriptionType::KNXAddresses.to_u8().unwrap())(input)?;

        let (input, address) = parse_u16(input).map_value(IndividualAddress::new)?;
        let (input, additional) =
            parse_count(IndividualAddress::parse, ((length - 4) / 2) as usize)(input)?;

        Ok((
            input,
            KNXAddresses {
                address,
                additional,
            },
        ))
    }
}
