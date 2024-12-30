// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use num_traits::{Pow, Signed};
use serde::{Deserialize, Serialize};
use std::ops::Neg;

use knxkit::{core::DataPoint, project::DPT};

use crate::Error;

pub use super::generated::specific::*;

pub trait SpecificDataPoint: std::fmt::Display + Serialize {
    const DPT: DPT;

    fn to_data_point(&self) -> DataPoint;

    fn from_data_point(data: &DataPoint) -> Result<Self, Error>
    where
        Self: Sized;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reserved;

impl Reserved {
    pub fn new() -> Self {
        Reserved {}
    }
}

impl std::fmt::Display for Reserved {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "-")
    }
}

pub(crate) fn decode_knxf16(repr: u16) -> f32 {
    if repr != 0x7fff {
        let exp = ((repr & 0x7800) >> 11) as f32;
        let man = (repr & 0x07ff) as f32;
        let sign = repr & 0x8000 != 0;

        let exp_f = 2.0f32.powf(exp);
        let man_f = (man) * 0.01;

        let mut value = man_f * exp_f;
        if sign {
            value = value.neg();
        }

        value
    } else {
        f32::NAN
    }
}

pub(crate) fn encode_knxf16(value: f32) -> u16 {
    let negative = value.is_negative();
    let mut value = value.abs();

    if !value.is_nan() && value <= 671_088.64_f32 {
        value *= 100.0;

        let exponent = if value > 2048_f32 {
            (value.log2() - 11.0).ceil() as u16
        } else {
            0
        };

        let mantissa = (value / 2.0_f32.pow(exponent)).round() as u16;

        let mut result = (exponent << 11) | mantissa;

        if negative {
            result |= 0x8000;
        }

        result
    } else {
        0x7fff
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generic::*;
    use crate::specific::*;

    #[test]
    fn test_knx_f16() {
        use super::{decode_knxf16, encode_knxf16};

        assert_eq!(decode_knxf16(0x0708), 18.0);
        assert_eq!(encode_knxf16(18.0), 0x0708);

        assert_eq!(decode_knxf16(0x8708), -18.0);
        assert_eq!(encode_knxf16(-18.0), 0x8708);

        assert!(decode_knxf16(0x7fff).is_nan());

        assert_eq!(encode_knxf16(f32::NAN), 0x7fff);
        assert_ne!(encode_knxf16(671_088.64), 0x7fff);
        assert_eq!(encode_knxf16(671_088.66), 0x7fff);
    }
}
