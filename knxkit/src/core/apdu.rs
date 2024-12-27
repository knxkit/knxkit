// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use num_traits::FromPrimitive;

use super::DataPoint;
use crate::core::util::prelude::*;

#[derive(Clone, Copy, Debug, num_derive::ToPrimitive, num_derive::FromPrimitive, PartialEq)]
pub enum Service {
    /// multicast
    GroupValueRead = 0x000,
    GroupValueResponse = 0x040,
    GroupValueWrite = 0x080,

    /// broadcast
    IndividualAddressWrite = 0x0c0,
    IndividualAddressRead = 0x100,
    IndividualAddressResponse = 0x140,

    IndividualAddressSerialNumberRead = 0x3dc,
    IndividualAddressSerialNumberResponse = 0x3dd,
    IndividualAddressSerialNumberWrite = 0x3de,
    // network parameter...

    // unicast connectionless
    DeviceDescriptorRead = 0x300,
    DeviceDescriptorResponse = 0x340,
    Restart = 0x380,

    PropertyValueRead = 0x3d5,
    PropertyValueResponse = 0x3d6,
    PropertyValueWrite = 0x3d7,

    PropertyDescriptionRead = 0x3d8,
    PropertyDescriptionResponse = 0x3d9,
    // link...

    // unicast connected
    MemoryRead = 0x200,
    MemoryResponse = 0x240,
    MemoryWrite = 0x280,

    UserMemoryRead = 0x2c0,
    UserMemoryResponse = 0x2c1,
    UserMemoryWrite = 0x2c2,

    UserManufacturerInfoRead = 0x2c5,
    UserManufacturerInfoResponse = 0x2c6,

    AuthorizeRequest = 0x3d1,
    AuthorizeResponse = 0x3d2,

    KeyWrite = 0x3d3,
    KeyResponse = 0x3d4,

    ADCRead = 0x9999,
    ADCResponse = 0x9998,
}

#[derive(Debug, Clone)]
pub struct APDU {
    pub service: Service,
    pub data: Option<DataPoint>,
}

impl APDU {
    pub fn parse(prefix: u8, input: Input, npdu_length: u8) -> Result<Self> {
        let (input, octet7) = parse_u8(input)?;

        let apci = (prefix as u16) << 8 | (octet7 as u16);

        let with_data = |service, data| APDU {
            service,
            data: Some(if npdu_length == 1 {
                DataPoint::Short(data)
            } else {
                DataPoint::Long(input.into())
            }),
        };

        let apdu = match (apci >> 6, (apci & 0b00111111) as u8) {
            (0b0001, data) => with_data(Service::GroupValueResponse, data),
            (0b0010, data) => with_data(Service::GroupValueWrite, data),

            (0b0110, data) => with_data(Service::ADCRead, data),
            (0b0111, data) => with_data(Service::ADCResponse, data),

            (0b1000, data) => with_data(Service::MemoryRead, data),
            (0b1001, data) => with_data(Service::MemoryResponse, data),
            (0b1010, data) => with_data(Service::MemoryWrite, data),
            _ => {
                let service = Service::from_u16(apci).ok_or(Err::Failure(Error::general(
                    "Unexpected APCI",
                    &[prefix, octet7],
                )))?;

                APDU {
                    service,
                    data: None,
                }
            }
        };

        Ok((&[], apdu))
    }

    pub(crate) fn suffix(&self) -> u8 {
        (self.service.to_u16().unwrap() >> 8) as u8
    }

    pub fn gen<W: Write>(&self) -> impl SerializeFn<W> + use<'_, W> {
        let code = (self.service.to_u16().unwrap() & 0xff) as u8;

        move |context| {
            match &self.data {
                Some(DataPoint::Short(s)) => {
                    //
                    gen_u8((code & 0b11000000) | (s & 0b00111111))(context)
                }

                Some(DataPoint::Long(l)) => {
                    //
                    gen_tuple((gen_u8(code & 0b11000000), gen_slice(l)))(context)
                }

                None => gen_u8(code)(context),
            }
        }
    }
}
