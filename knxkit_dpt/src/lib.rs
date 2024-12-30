// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

//! Datapoint structures for [knxkit](https://crates.io/crates/knxkit)
//!
//! Create a specific datapoint and convert it to a binary representation
//!```rust
//!  use knxkit_dpt::specific::{SpecificDataPoint, DPT_3_7};   
//!
//!  let value37 = DPT_3_7 {
//!    Increase: true,
//!    StepCode: 1,
//!  };
//!
//!  let datapoint = value37.to_data_point();
//!     
//!  assert_eq!(datapoint, knxkit::core::DataPoint::Short(0x09));
//!```
//!
//! Decode a binary representation to a specific datapoint
//! ```rust
//!  use knxkit_dpt::specific::{SpecificDataPoint, DPT_3_7};   
//!
//!  let datapoint = knxkit::core::DataPoint::Short(0x09);
//!  let value37 = DPT_3_7::from_data_point(&datapoint).unwrap();
//!
//!  assert_eq!(value37, DPT_3_7 {Increase: true, StepCode: 1});
//! ```
//!
//! Decode a binary representation to a generic datapoint and then to JSON string
//!```rust
//!  use knxkit_dpt::specific::{SpecificDataPoint, DPT_3_7};
//!  use knxkit_dpt::generic;
//!
//!  let datapoint = knxkit::core::DataPoint::Short(0x09);
//!  let generic = generic::try_decode(DPT_3_7::DPT, &datapoint).unwrap();
//!  let json = generic.to_json_string();
//!  assert_eq!(json, "{\"Increase\":true,\"StepCode\":1}");
//!```
//! Lookup a datapoint info by DPT
//! ```rust
//!  use knxkit_dpt::specific::{SpecificDataPoint, DPT_3_7};
//!
//!  let info = knxkit_dpt::typeinfo::lookup(DPT_3_7::DPT).unwrap();   
//!
//!  assert_eq!(info.name, "DPT_Control_Dimming");
//!  assert_eq!(info.text, Some("dimming control"));
//!  assert_eq!(info.unit, None);
//! ```

mod error;
mod generated;
pub mod generic;
pub mod project;
pub mod specific;
pub mod typeinfo;

pub use error::Error;

#[cfg(test)]
mod tests {
    use crate::generic::*;
    use crate::specific::*;
    use knxkit::{core::DataPoint, project::DPT};

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
