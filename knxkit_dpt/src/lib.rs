// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use std::ops::Neg;

use knxkit::{core::DataPoint, project::DPT};
use num_traits::{Pow, Signed};
use serde::{Deserialize, Serialize};

mod generated;
pub mod opaque;
pub mod project;

pub use generated::{generic, specific, typeinfo};

pub fn decode_knxf16(repr: u16) -> f32 {
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

pub fn encode_knxf16(value: f32) -> u16 {
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

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid data point value: {0}")]
    InvalidDataPointValue(DataPoint),

    #[error("Invalid DPT: {0}")]
    InvalidDPT(DPT),

    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Serde Error: {0}")]
    SerdeError(#[from] serde_json::Error),
}

pub trait Specific: std::fmt::Display + Serialize {
    const DPT: DPT;

    fn to_data_point(&self) -> DataPoint;

    fn from_data_point(data: &DataPoint) -> Result<Self, Error>
    where
        Self: Sized;
}

pub struct DataPointTypeInfo {
    pub dpt: DPT,
    pub name: &'static str,
    pub text: Option<&'static str>,
    pub unit: Option<&'static str>,
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

    #[test]
    fn test_decode_1_1() {
        assert_eq!(
            DPT_1_1::from_data_point(&DataPoint::Short(0x01)).unwrap().0,
            true
        );
        assert_eq!(
            DPT_1_1::from_data_point(&DataPoint::Short(0x00)).unwrap().0,
            false
        );
    }

    #[test]
    fn test_encode_1_1() {
        assert_eq!(DPT_1_1(true).to_data_point(), DataPoint::Short(0x01));

        assert_eq!(DPT_1_1(false).to_data_point(), DataPoint::Short(0x00));
    }

    #[test]
    fn test_encode_json_1_1() {
        assert_eq!(serde_json::to_string(&DPT_1_1(false)).unwrap(), "false");
    }

    #[test]
    fn test_encode_json_7_1() {
        assert_eq!(serde_json::to_string(&DPT_7_1(6)).unwrap(), "6");
    }

    #[test]
    fn test_decode_generic_1_1() {
        let opaque1 = try_decode(DPT::new(1, Some(1)), &DataPoint::Short(0x01)).unwrap();

        let json = serde_json::to_string(&opaque1.to_json_value()).unwrap();
        assert_eq!(json, "true");

        let _opaque2 =
            try_decode_json(DPT::new(1, Some(1)), serde_json::from_str(&json).unwrap()).unwrap();
    }
}
