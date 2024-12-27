// This file is generated, don't manually edit it!
use serde_json::Value;
use knxkit::{core::DataPoint, project::DPT};
use crate::{Error, Specific, opaque::OpaqueDataPoint, specific::*};
pub fn try_decode(dpt: DPT, data: &DataPoint) -> Result<OpaqueDataPoint, Error> {
    static DECODERS: &'static [(
        DPT,
        fn(&DataPoint) -> Result<OpaqueDataPoint, Error>,
    )] = &[
        (
            DPT::new(1u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_1_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(1u16, Some(1u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_1_1::from_data_point(dp)?)) },
        ),
        (
            DPT::new(1u16, Some(2u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_1_2::from_data_point(dp)?)) },
        ),
        (
            DPT::new(1u16, Some(3u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_1_3::from_data_point(dp)?)) },
        ),
        (
            DPT::new(1u16, Some(4u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_1_4::from_data_point(dp)?)) },
        ),
        (
            DPT::new(1u16, Some(5u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_1_5::from_data_point(dp)?)) },
        ),
        (
            DPT::new(1u16, Some(6u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_1_6::from_data_point(dp)?)) },
        ),
        (
            DPT::new(1u16, Some(7u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_1_7::from_data_point(dp)?)) },
        ),
        (
            DPT::new(1u16, Some(8u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_1_8::from_data_point(dp)?)) },
        ),
        (
            DPT::new(1u16, Some(9u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_1_9::from_data_point(dp)?)) },
        ),
        (
            DPT::new(1u16, Some(10u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_1_10::from_data_point(dp)?)) },
        ),
        (
            DPT::new(1u16, Some(11u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_1_11::from_data_point(dp)?)) },
        ),
        (
            DPT::new(1u16, Some(12u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_1_12::from_data_point(dp)?)) },
        ),
        (
            DPT::new(1u16, Some(13u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_1_13::from_data_point(dp)?)) },
        ),
        (
            DPT::new(1u16, Some(14u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_1_14::from_data_point(dp)?)) },
        ),
        (
            DPT::new(1u16, Some(15u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_1_15::from_data_point(dp)?)) },
        ),
        (
            DPT::new(1u16, Some(16u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_1_16::from_data_point(dp)?)) },
        ),
        (
            DPT::new(1u16, Some(17u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_1_17::from_data_point(dp)?)) },
        ),
        (
            DPT::new(1u16, Some(18u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_1_18::from_data_point(dp)?)) },
        ),
        (
            DPT::new(1u16, Some(19u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_1_19::from_data_point(dp)?)) },
        ),
        (
            DPT::new(1u16, Some(21u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_1_21::from_data_point(dp)?)) },
        ),
        (
            DPT::new(1u16, Some(22u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_1_22::from_data_point(dp)?)) },
        ),
        (
            DPT::new(1u16, Some(23u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_1_23::from_data_point(dp)?)) },
        ),
        (
            DPT::new(1u16, Some(24u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_1_24::from_data_point(dp)?)) },
        ),
        (
            DPT::new(1u16, Some(100u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_1_100::from_data_point(dp)?)) },
        ),
        (
            DPT::new(2u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_2_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(2u16, Some(1u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_2_1::from_data_point(dp)?)) },
        ),
        (
            DPT::new(2u16, Some(2u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_2_2::from_data_point(dp)?)) },
        ),
        (
            DPT::new(2u16, Some(3u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_2_3::from_data_point(dp)?)) },
        ),
        (
            DPT::new(2u16, Some(4u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_2_4::from_data_point(dp)?)) },
        ),
        (
            DPT::new(2u16, Some(5u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_2_5::from_data_point(dp)?)) },
        ),
        (
            DPT::new(2u16, Some(6u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_2_6::from_data_point(dp)?)) },
        ),
        (
            DPT::new(2u16, Some(7u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_2_7::from_data_point(dp)?)) },
        ),
        (
            DPT::new(2u16, Some(8u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_2_8::from_data_point(dp)?)) },
        ),
        (
            DPT::new(2u16, Some(9u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_2_9::from_data_point(dp)?)) },
        ),
        (
            DPT::new(2u16, Some(10u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_2_10::from_data_point(dp)?)) },
        ),
        (
            DPT::new(2u16, Some(11u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_2_11::from_data_point(dp)?)) },
        ),
        (
            DPT::new(2u16, Some(12u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_2_12::from_data_point(dp)?)) },
        ),
        (
            DPT::new(3u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_3_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(3u16, Some(7u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_3_7::from_data_point(dp)?)) },
        ),
        (
            DPT::new(3u16, Some(8u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_3_8::from_data_point(dp)?)) },
        ),
        (
            DPT::new(4u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_4_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(4u16, Some(1u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_4_1::from_data_point(dp)?)) },
        ),
        (
            DPT::new(4u16, Some(2u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_4_2::from_data_point(dp)?)) },
        ),
        (
            DPT::new(5u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_5_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(5u16, Some(1u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_5_1::from_data_point(dp)?)) },
        ),
        (
            DPT::new(5u16, Some(3u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_5_3::from_data_point(dp)?)) },
        ),
        (
            DPT::new(5u16, Some(4u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_5_4::from_data_point(dp)?)) },
        ),
        (
            DPT::new(5u16, Some(5u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_5_5::from_data_point(dp)?)) },
        ),
        (
            DPT::new(5u16, Some(6u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_5_6::from_data_point(dp)?)) },
        ),
        (
            DPT::new(5u16, Some(10u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_5_10::from_data_point(dp)?)) },
        ),
        (
            DPT::new(5u16, Some(100u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_5_100::from_data_point(dp)?)) },
        ),
        (
            DPT::new(6u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_6_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(6u16, Some(1u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_6_1::from_data_point(dp)?)) },
        ),
        (
            DPT::new(6u16, Some(10u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_6_10::from_data_point(dp)?)) },
        ),
        (
            DPT::new(6u16, Some(20u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_6_20::from_data_point(dp)?)) },
        ),
        (
            DPT::new(7u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_7_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(7u16, Some(1u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_7_1::from_data_point(dp)?)) },
        ),
        (
            DPT::new(7u16, Some(2u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_7_2::from_data_point(dp)?)) },
        ),
        (
            DPT::new(7u16, Some(3u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_7_3::from_data_point(dp)?)) },
        ),
        (
            DPT::new(7u16, Some(4u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_7_4::from_data_point(dp)?)) },
        ),
        (
            DPT::new(7u16, Some(5u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_7_5::from_data_point(dp)?)) },
        ),
        (
            DPT::new(7u16, Some(6u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_7_6::from_data_point(dp)?)) },
        ),
        (
            DPT::new(7u16, Some(7u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_7_7::from_data_point(dp)?)) },
        ),
        (
            DPT::new(7u16, Some(10u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_7_10::from_data_point(dp)?)) },
        ),
        (
            DPT::new(7u16, Some(11u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_7_11::from_data_point(dp)?)) },
        ),
        (
            DPT::new(7u16, Some(12u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_7_12::from_data_point(dp)?)) },
        ),
        (
            DPT::new(7u16, Some(13u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_7_13::from_data_point(dp)?)) },
        ),
        (
            DPT::new(7u16, Some(600u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_7_600::from_data_point(dp)?)) },
        ),
        (
            DPT::new(8u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_8_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(8u16, Some(1u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_8_1::from_data_point(dp)?)) },
        ),
        (
            DPT::new(8u16, Some(2u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_8_2::from_data_point(dp)?)) },
        ),
        (
            DPT::new(8u16, Some(3u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_8_3::from_data_point(dp)?)) },
        ),
        (
            DPT::new(8u16, Some(4u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_8_4::from_data_point(dp)?)) },
        ),
        (
            DPT::new(8u16, Some(5u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_8_5::from_data_point(dp)?)) },
        ),
        (
            DPT::new(8u16, Some(6u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_8_6::from_data_point(dp)?)) },
        ),
        (
            DPT::new(8u16, Some(7u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_8_7::from_data_point(dp)?)) },
        ),
        (
            DPT::new(8u16, Some(10u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_8_10::from_data_point(dp)?)) },
        ),
        (
            DPT::new(8u16, Some(11u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_8_11::from_data_point(dp)?)) },
        ),
        (
            DPT::new(8u16, Some(12u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_8_12::from_data_point(dp)?)) },
        ),
        (
            DPT::new(9u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_9_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(9u16, Some(1u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_9_1::from_data_point(dp)?)) },
        ),
        (
            DPT::new(9u16, Some(2u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_9_2::from_data_point(dp)?)) },
        ),
        (
            DPT::new(9u16, Some(3u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_9_3::from_data_point(dp)?)) },
        ),
        (
            DPT::new(9u16, Some(4u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_9_4::from_data_point(dp)?)) },
        ),
        (
            DPT::new(9u16, Some(5u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_9_5::from_data_point(dp)?)) },
        ),
        (
            DPT::new(9u16, Some(6u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_9_6::from_data_point(dp)?)) },
        ),
        (
            DPT::new(9u16, Some(7u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_9_7::from_data_point(dp)?)) },
        ),
        (
            DPT::new(9u16, Some(8u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_9_8::from_data_point(dp)?)) },
        ),
        (
            DPT::new(9u16, Some(9u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_9_9::from_data_point(dp)?)) },
        ),
        (
            DPT::new(9u16, Some(10u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_9_10::from_data_point(dp)?)) },
        ),
        (
            DPT::new(9u16, Some(11u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_9_11::from_data_point(dp)?)) },
        ),
        (
            DPT::new(9u16, Some(20u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_9_20::from_data_point(dp)?)) },
        ),
        (
            DPT::new(9u16, Some(21u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_9_21::from_data_point(dp)?)) },
        ),
        (
            DPT::new(9u16, Some(22u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_9_22::from_data_point(dp)?)) },
        ),
        (
            DPT::new(9u16, Some(23u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_9_23::from_data_point(dp)?)) },
        ),
        (
            DPT::new(9u16, Some(24u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_9_24::from_data_point(dp)?)) },
        ),
        (
            DPT::new(9u16, Some(25u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_9_25::from_data_point(dp)?)) },
        ),
        (
            DPT::new(9u16, Some(26u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_9_26::from_data_point(dp)?)) },
        ),
        (
            DPT::new(9u16, Some(27u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_9_27::from_data_point(dp)?)) },
        ),
        (
            DPT::new(9u16, Some(28u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_9_28::from_data_point(dp)?)) },
        ),
        (
            DPT::new(9u16, Some(29u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_9_29::from_data_point(dp)?)) },
        ),
        (
            DPT::new(9u16, Some(30u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_9_30::from_data_point(dp)?)) },
        ),
        (
            DPT::new(9u16, Some(31u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_9_31::from_data_point(dp)?)) },
        ),
        (
            DPT::new(10u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_10_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(10u16, Some(1u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_10_1::from_data_point(dp)?)) },
        ),
        (
            DPT::new(11u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_11_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(11u16, Some(1u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_11_1::from_data_point(dp)?)) },
        ),
        (
            DPT::new(12u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_12_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(12u16, Some(1u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_12_1::from_data_point(dp)?)) },
        ),
        (
            DPT::new(12u16, Some(100u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_12_100::from_data_point(dp)?)) },
        ),
        (
            DPT::new(12u16, Some(101u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_12_101::from_data_point(dp)?)) },
        ),
        (
            DPT::new(12u16, Some(102u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_12_102::from_data_point(dp)?)) },
        ),
        (
            DPT::new(12u16, Some(1200u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_12_1200::from_data_point(dp)?)) },
        ),
        (
            DPT::new(12u16, Some(1201u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_12_1201::from_data_point(dp)?)) },
        ),
        (
            DPT::new(13u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_13_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(13u16, Some(1u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_13_1::from_data_point(dp)?)) },
        ),
        (
            DPT::new(13u16, Some(2u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_13_2::from_data_point(dp)?)) },
        ),
        (
            DPT::new(13u16, Some(10u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_13_10::from_data_point(dp)?)) },
        ),
        (
            DPT::new(13u16, Some(11u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_13_11::from_data_point(dp)?)) },
        ),
        (
            DPT::new(13u16, Some(12u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_13_12::from_data_point(dp)?)) },
        ),
        (
            DPT::new(13u16, Some(13u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_13_13::from_data_point(dp)?)) },
        ),
        (
            DPT::new(13u16, Some(14u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_13_14::from_data_point(dp)?)) },
        ),
        (
            DPT::new(13u16, Some(15u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_13_15::from_data_point(dp)?)) },
        ),
        (
            DPT::new(13u16, Some(16u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_13_16::from_data_point(dp)?)) },
        ),
        (
            DPT::new(13u16, Some(100u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_13_100::from_data_point(dp)?)) },
        ),
        (
            DPT::new(13u16, Some(1200u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_13_1200::from_data_point(dp)?)) },
        ),
        (
            DPT::new(13u16, Some(1201u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_13_1201::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(0u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_0::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(1u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_1::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(2u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_2::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(3u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_3::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(4u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_4::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(5u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_5::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(6u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_6::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(7u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_7::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(8u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_8::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(9u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_9::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(10u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_10::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(11u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_11::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(12u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_12::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(13u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_13::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(14u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_14::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(15u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_15::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(16u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_16::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(17u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_17::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(18u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_18::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(19u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_19::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(20u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_20::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(21u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_21::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(22u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_22::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(23u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_23::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(24u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_24::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(25u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_25::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(26u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_26::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(27u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_27::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(28u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_28::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(29u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_29::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(30u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_30::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(31u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_31::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(32u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_32::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(33u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_33::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(34u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_34::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(35u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_35::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(36u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_36::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(37u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_37::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(38u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_38::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(39u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_39::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(40u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_40::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(41u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_41::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(42u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_42::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(43u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_43::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(44u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_44::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(45u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_45::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(46u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_46::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(47u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_47::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(48u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_48::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(49u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_49::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(50u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_50::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(51u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_51::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(52u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_52::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(53u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_53::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(54u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_54::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(55u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_55::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(56u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_56::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(57u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_57::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(58u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_58::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(59u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_59::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(60u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_60::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(61u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_61::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(62u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_62::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(63u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_63::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(64u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_64::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(65u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_65::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(66u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_66::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(67u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_67::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(68u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_68::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(69u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_69::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(70u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_70::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(71u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_71::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(72u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_72::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(73u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_73::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(74u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_74::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(75u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_75::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(76u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_76::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(77u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_77::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(78u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_78::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(79u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_79::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(80u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_80::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(1200u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_1200::from_data_point(dp)?)) },
        ),
        (
            DPT::new(14u16, Some(1201u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_14_1201::from_data_point(dp)?)) },
        ),
        (
            DPT::new(15u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_15_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(15u16, Some(0u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_15_0::from_data_point(dp)?)) },
        ),
        (
            DPT::new(16u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_16_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(16u16, Some(0u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_16_0::from_data_point(dp)?)) },
        ),
        (
            DPT::new(16u16, Some(1u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_16_1::from_data_point(dp)?)) },
        ),
        (
            DPT::new(17u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_17_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(17u16, Some(1u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_17_1::from_data_point(dp)?)) },
        ),
        (
            DPT::new(18u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_18_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(18u16, Some(1u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_18_1::from_data_point(dp)?)) },
        ),
        (
            DPT::new(19u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_19_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(19u16, Some(1u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_19_1::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(1u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_1::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(2u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_2::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(3u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_3::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(4u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_4::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(5u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_5::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(6u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_6::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(7u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_7::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(8u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_8::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(11u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_11::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(12u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_12::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(13u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_13::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(14u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_14::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(17u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_17::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(20u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_20::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(21u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_21::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(22u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_22::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(100u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_100::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(101u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_101::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(102u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_102::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(103u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_103::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(104u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_104::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(105u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_105::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(106u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_106::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(107u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_107::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(108u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_108::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(109u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_109::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(110u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_110::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(111u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_111::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(112u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_112::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(113u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_113::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(114u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_114::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(115u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_115::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(116u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_116::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(120u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_120::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(121u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_121::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(122u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_122::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(600u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_600::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(601u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_601::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(602u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_602::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(603u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_603::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(604u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_604::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(605u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_605::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(606u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_606::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(607u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_607::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(608u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_608::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(609u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_609::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(610u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_610::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(611u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_611::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(612u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_612::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(801u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_801::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(802u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_802::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(803u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_803::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(804u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_804::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(1000u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_1000::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(1001u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_1001::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(1002u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_1002::from_data_point(dp)?)) },
        ),
        (
            DPT::new(20u16, Some(1003u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_20_1003::from_data_point(dp)?)) },
        ),
        (
            DPT::new(21u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_21_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(21u16, Some(1u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_21_1::from_data_point(dp)?)) },
        ),
        (
            DPT::new(21u16, Some(2u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_21_2::from_data_point(dp)?)) },
        ),
        (
            DPT::new(21u16, Some(100u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_21_100::from_data_point(dp)?)) },
        ),
        (
            DPT::new(21u16, Some(101u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_21_101::from_data_point(dp)?)) },
        ),
        (
            DPT::new(21u16, Some(102u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_21_102::from_data_point(dp)?)) },
        ),
        (
            DPT::new(21u16, Some(103u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_21_103::from_data_point(dp)?)) },
        ),
        (
            DPT::new(21u16, Some(104u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_21_104::from_data_point(dp)?)) },
        ),
        (
            DPT::new(21u16, Some(105u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_21_105::from_data_point(dp)?)) },
        ),
        (
            DPT::new(21u16, Some(106u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_21_106::from_data_point(dp)?)) },
        ),
        (
            DPT::new(21u16, Some(107u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_21_107::from_data_point(dp)?)) },
        ),
        (
            DPT::new(21u16, Some(601u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_21_601::from_data_point(dp)?)) },
        ),
        (
            DPT::new(21u16, Some(1000u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_21_1000::from_data_point(dp)?)) },
        ),
        (
            DPT::new(21u16, Some(1001u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_21_1001::from_data_point(dp)?)) },
        ),
        (
            DPT::new(21u16, Some(1010u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_21_1010::from_data_point(dp)?)) },
        ),
        (
            DPT::new(22u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_22_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(22u16, Some(100u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_22_100::from_data_point(dp)?)) },
        ),
        (
            DPT::new(22u16, Some(101u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_22_101::from_data_point(dp)?)) },
        ),
        (
            DPT::new(22u16, Some(102u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_22_102::from_data_point(dp)?)) },
        ),
        (
            DPT::new(22u16, Some(103u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_22_103::from_data_point(dp)?)) },
        ),
        (
            DPT::new(22u16, Some(1000u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_22_1000::from_data_point(dp)?)) },
        ),
        (
            DPT::new(22u16, Some(1010u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_22_1010::from_data_point(dp)?)) },
        ),
        (
            DPT::new(23u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_23_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(23u16, Some(1u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_23_1::from_data_point(dp)?)) },
        ),
        (
            DPT::new(23u16, Some(2u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_23_2::from_data_point(dp)?)) },
        ),
        (
            DPT::new(23u16, Some(3u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_23_3::from_data_point(dp)?)) },
        ),
        (
            DPT::new(23u16, Some(102u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_23_102::from_data_point(dp)?)) },
        ),
        (
            DPT::new(24u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_24_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(24u16, Some(1u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_24_1::from_data_point(dp)?)) },
        ),
        (
            DPT::new(25u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_25_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(25u16, Some(1000u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_25_1000::from_data_point(dp)?)) },
        ),
        (
            DPT::new(26u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_26_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(26u16, Some(1u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_26_1::from_data_point(dp)?)) },
        ),
        (
            DPT::new(27u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_27_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(27u16, Some(1u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_27_1::from_data_point(dp)?)) },
        ),
        (
            DPT::new(28u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_28_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(28u16, Some(1u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_28_1::from_data_point(dp)?)) },
        ),
        (
            DPT::new(29u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_29_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(29u16, Some(10u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_29_10::from_data_point(dp)?)) },
        ),
        (
            DPT::new(29u16, Some(11u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_29_11::from_data_point(dp)?)) },
        ),
        (
            DPT::new(29u16, Some(12u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_29_12::from_data_point(dp)?)) },
        ),
        (
            DPT::new(30u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_30_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(30u16, Some(1010u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_30_1010::from_data_point(dp)?)) },
        ),
        (
            DPT::new(206u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_206_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(206u16, Some(100u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_206_100::from_data_point(dp)?)) },
        ),
        (
            DPT::new(206u16, Some(102u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_206_102::from_data_point(dp)?)) },
        ),
        (
            DPT::new(206u16, Some(104u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_206_104::from_data_point(dp)?)) },
        ),
        (
            DPT::new(206u16, Some(105u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_206_105::from_data_point(dp)?)) },
        ),
        (
            DPT::new(207u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_207_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(207u16, Some(600u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_207_600::from_data_point(dp)?)) },
        ),
        (
            DPT::new(217u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_217_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(217u16, Some(1u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_217_1::from_data_point(dp)?)) },
        ),
        (
            DPT::new(219u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_219_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(219u16, Some(1u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_219_1::from_data_point(dp)?)) },
        ),
        (
            DPT::new(222u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_222_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(222u16, Some(100u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_222_100::from_data_point(dp)?)) },
        ),
        (
            DPT::new(222u16, Some(101u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_222_101::from_data_point(dp)?)) },
        ),
        (
            DPT::new(225u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_225_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(225u16, Some(1u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_225_1::from_data_point(dp)?)) },
        ),
        (
            DPT::new(225u16, Some(2u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_225_2::from_data_point(dp)?)) },
        ),
        (
            DPT::new(229u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_229_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(229u16, Some(1u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_229_1::from_data_point(dp)?)) },
        ),
        (
            DPT::new(230u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_230_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(230u16, Some(1000u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_230_1000::from_data_point(dp)?)) },
        ),
        (
            DPT::new(232u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_232_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(232u16, Some(600u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_232_600::from_data_point(dp)?)) },
        ),
        (
            DPT::new(234u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_234_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(234u16, Some(1u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_234_1::from_data_point(dp)?)) },
        ),
        (
            DPT::new(235u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_235_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(235u16, Some(1u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_235_1::from_data_point(dp)?)) },
        ),
        (
            DPT::new(236u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_236_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(236u16, Some(1u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_236_1::from_data_point(dp)?)) },
        ),
        (
            DPT::new(237u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_237_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(237u16, Some(600u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_237_600::from_data_point(dp)?)) },
        ),
        (
            DPT::new(238u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_238_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(238u16, Some(600u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_238_600::from_data_point(dp)?)) },
        ),
        (
            DPT::new(240u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_240_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(240u16, Some(800u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_240_800::from_data_point(dp)?)) },
        ),
        (
            DPT::new(241u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_241_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(241u16, Some(800u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_241_800::from_data_point(dp)?)) },
        ),
        (
            DPT::new(242u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_242_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(242u16, Some(600u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_242_600::from_data_point(dp)?)) },
        ),
        (
            DPT::new(244u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_244_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(244u16, Some(600u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_244_600::from_data_point(dp)?)) },
        ),
        (
            DPT::new(245u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_245_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(245u16, Some(600u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_245_600::from_data_point(dp)?)) },
        ),
        (
            DPT::new(246u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_246_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(246u16, Some(600u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_246_600::from_data_point(dp)?)) },
        ),
        (
            DPT::new(249u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_249_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(249u16, Some(600u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_249_600::from_data_point(dp)?)) },
        ),
        (
            DPT::new(250u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_250_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(250u16, Some(600u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_250_600::from_data_point(dp)?)) },
        ),
        (
            DPT::new(251u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_251_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(251u16, Some(600u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_251_600::from_data_point(dp)?)) },
        ),
        (
            DPT::new(252u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_252_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(252u16, Some(600u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_252_600::from_data_point(dp)?)) },
        ),
        (
            DPT::new(254u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_254_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(254u16, Some(600u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_254_600::from_data_point(dp)?)) },
        ),
        (
            DPT::new(255u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_255_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(255u16, Some(1u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_255_1::from_data_point(dp)?)) },
        ),
        (
            DPT::new(275u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_275_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(275u16, Some(100u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_275_100::from_data_point(dp)?)) },
        ),
        (
            DPT::new(275u16, Some(101u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_275_101::from_data_point(dp)?)) },
        ),
        (
            DPT::new(285u16, None),
            |dp| { Ok(OpaqueDataPoint::new(DPT_285_x::from_data_point(dp)?)) },
        ),
        (
            DPT::new(285u16, Some(1u16)),
            |dp| { Ok(OpaqueDataPoint::new(DPT_285_1::from_data_point(dp)?)) },
        ),
    ];
    DECODERS
        .binary_search_by_key(&dpt, |x| x.0)
        .map(|ix| DECODERS[ix].1(data))
        .map_err(|_| Error::InvalidDPT(dpt))?
}
pub fn try_decode_json(dpt: DPT, data: Value) -> Result<OpaqueDataPoint, Error> {
    static DECODERS: &'static [(DPT, fn(Value) -> Result<OpaqueDataPoint, Error>)] = &[
        (
            DPT::new(1u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_1_x>(value)?))
            },
        ),
        (
            DPT::new(1u16, Some(1u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_1_1>(value)?))
            },
        ),
        (
            DPT::new(1u16, Some(2u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_1_2>(value)?))
            },
        ),
        (
            DPT::new(1u16, Some(3u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_1_3>(value)?))
            },
        ),
        (
            DPT::new(1u16, Some(4u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_1_4>(value)?))
            },
        ),
        (
            DPT::new(1u16, Some(5u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_1_5>(value)?))
            },
        ),
        (
            DPT::new(1u16, Some(6u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_1_6>(value)?))
            },
        ),
        (
            DPT::new(1u16, Some(7u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_1_7>(value)?))
            },
        ),
        (
            DPT::new(1u16, Some(8u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_1_8>(value)?))
            },
        ),
        (
            DPT::new(1u16, Some(9u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_1_9>(value)?))
            },
        ),
        (
            DPT::new(1u16, Some(10u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_1_10>(value)?))
            },
        ),
        (
            DPT::new(1u16, Some(11u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_1_11>(value)?))
            },
        ),
        (
            DPT::new(1u16, Some(12u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_1_12>(value)?))
            },
        ),
        (
            DPT::new(1u16, Some(13u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_1_13>(value)?))
            },
        ),
        (
            DPT::new(1u16, Some(14u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_1_14>(value)?))
            },
        ),
        (
            DPT::new(1u16, Some(15u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_1_15>(value)?))
            },
        ),
        (
            DPT::new(1u16, Some(16u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_1_16>(value)?))
            },
        ),
        (
            DPT::new(1u16, Some(17u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_1_17>(value)?))
            },
        ),
        (
            DPT::new(1u16, Some(18u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_1_18>(value)?))
            },
        ),
        (
            DPT::new(1u16, Some(19u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_1_19>(value)?))
            },
        ),
        (
            DPT::new(1u16, Some(21u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_1_21>(value)?))
            },
        ),
        (
            DPT::new(1u16, Some(22u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_1_22>(value)?))
            },
        ),
        (
            DPT::new(1u16, Some(23u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_1_23>(value)?))
            },
        ),
        (
            DPT::new(1u16, Some(24u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_1_24>(value)?))
            },
        ),
        (
            DPT::new(1u16, Some(100u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_1_100>(value)?))
            },
        ),
        (
            DPT::new(2u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_2_x>(value)?))
            },
        ),
        (
            DPT::new(2u16, Some(1u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_2_1>(value)?))
            },
        ),
        (
            DPT::new(2u16, Some(2u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_2_2>(value)?))
            },
        ),
        (
            DPT::new(2u16, Some(3u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_2_3>(value)?))
            },
        ),
        (
            DPT::new(2u16, Some(4u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_2_4>(value)?))
            },
        ),
        (
            DPT::new(2u16, Some(5u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_2_5>(value)?))
            },
        ),
        (
            DPT::new(2u16, Some(6u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_2_6>(value)?))
            },
        ),
        (
            DPT::new(2u16, Some(7u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_2_7>(value)?))
            },
        ),
        (
            DPT::new(2u16, Some(8u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_2_8>(value)?))
            },
        ),
        (
            DPT::new(2u16, Some(9u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_2_9>(value)?))
            },
        ),
        (
            DPT::new(2u16, Some(10u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_2_10>(value)?))
            },
        ),
        (
            DPT::new(2u16, Some(11u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_2_11>(value)?))
            },
        ),
        (
            DPT::new(2u16, Some(12u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_2_12>(value)?))
            },
        ),
        (
            DPT::new(3u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_3_x>(value)?))
            },
        ),
        (
            DPT::new(3u16, Some(7u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_3_7>(value)?))
            },
        ),
        (
            DPT::new(3u16, Some(8u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_3_8>(value)?))
            },
        ),
        (
            DPT::new(4u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_4_x>(value)?))
            },
        ),
        (
            DPT::new(4u16, Some(1u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_4_1>(value)?))
            },
        ),
        (
            DPT::new(4u16, Some(2u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_4_2>(value)?))
            },
        ),
        (
            DPT::new(5u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_5_x>(value)?))
            },
        ),
        (
            DPT::new(5u16, Some(1u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_5_1>(value)?))
            },
        ),
        (
            DPT::new(5u16, Some(3u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_5_3>(value)?))
            },
        ),
        (
            DPT::new(5u16, Some(4u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_5_4>(value)?))
            },
        ),
        (
            DPT::new(5u16, Some(5u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_5_5>(value)?))
            },
        ),
        (
            DPT::new(5u16, Some(6u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_5_6>(value)?))
            },
        ),
        (
            DPT::new(5u16, Some(10u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_5_10>(value)?))
            },
        ),
        (
            DPT::new(5u16, Some(100u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_5_100>(value)?))
            },
        ),
        (
            DPT::new(6u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_6_x>(value)?))
            },
        ),
        (
            DPT::new(6u16, Some(1u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_6_1>(value)?))
            },
        ),
        (
            DPT::new(6u16, Some(10u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_6_10>(value)?))
            },
        ),
        (
            DPT::new(6u16, Some(20u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_6_20>(value)?))
            },
        ),
        (
            DPT::new(7u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_7_x>(value)?))
            },
        ),
        (
            DPT::new(7u16, Some(1u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_7_1>(value)?))
            },
        ),
        (
            DPT::new(7u16, Some(2u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_7_2>(value)?))
            },
        ),
        (
            DPT::new(7u16, Some(3u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_7_3>(value)?))
            },
        ),
        (
            DPT::new(7u16, Some(4u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_7_4>(value)?))
            },
        ),
        (
            DPT::new(7u16, Some(5u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_7_5>(value)?))
            },
        ),
        (
            DPT::new(7u16, Some(6u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_7_6>(value)?))
            },
        ),
        (
            DPT::new(7u16, Some(7u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_7_7>(value)?))
            },
        ),
        (
            DPT::new(7u16, Some(10u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_7_10>(value)?))
            },
        ),
        (
            DPT::new(7u16, Some(11u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_7_11>(value)?))
            },
        ),
        (
            DPT::new(7u16, Some(12u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_7_12>(value)?))
            },
        ),
        (
            DPT::new(7u16, Some(13u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_7_13>(value)?))
            },
        ),
        (
            DPT::new(7u16, Some(600u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_7_600>(value)?))
            },
        ),
        (
            DPT::new(8u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_8_x>(value)?))
            },
        ),
        (
            DPT::new(8u16, Some(1u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_8_1>(value)?))
            },
        ),
        (
            DPT::new(8u16, Some(2u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_8_2>(value)?))
            },
        ),
        (
            DPT::new(8u16, Some(3u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_8_3>(value)?))
            },
        ),
        (
            DPT::new(8u16, Some(4u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_8_4>(value)?))
            },
        ),
        (
            DPT::new(8u16, Some(5u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_8_5>(value)?))
            },
        ),
        (
            DPT::new(8u16, Some(6u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_8_6>(value)?))
            },
        ),
        (
            DPT::new(8u16, Some(7u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_8_7>(value)?))
            },
        ),
        (
            DPT::new(8u16, Some(10u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_8_10>(value)?))
            },
        ),
        (
            DPT::new(8u16, Some(11u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_8_11>(value)?))
            },
        ),
        (
            DPT::new(8u16, Some(12u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_8_12>(value)?))
            },
        ),
        (
            DPT::new(9u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_9_x>(value)?))
            },
        ),
        (
            DPT::new(9u16, Some(1u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_9_1>(value)?))
            },
        ),
        (
            DPT::new(9u16, Some(2u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_9_2>(value)?))
            },
        ),
        (
            DPT::new(9u16, Some(3u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_9_3>(value)?))
            },
        ),
        (
            DPT::new(9u16, Some(4u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_9_4>(value)?))
            },
        ),
        (
            DPT::new(9u16, Some(5u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_9_5>(value)?))
            },
        ),
        (
            DPT::new(9u16, Some(6u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_9_6>(value)?))
            },
        ),
        (
            DPT::new(9u16, Some(7u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_9_7>(value)?))
            },
        ),
        (
            DPT::new(9u16, Some(8u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_9_8>(value)?))
            },
        ),
        (
            DPT::new(9u16, Some(9u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_9_9>(value)?))
            },
        ),
        (
            DPT::new(9u16, Some(10u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_9_10>(value)?))
            },
        ),
        (
            DPT::new(9u16, Some(11u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_9_11>(value)?))
            },
        ),
        (
            DPT::new(9u16, Some(20u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_9_20>(value)?))
            },
        ),
        (
            DPT::new(9u16, Some(21u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_9_21>(value)?))
            },
        ),
        (
            DPT::new(9u16, Some(22u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_9_22>(value)?))
            },
        ),
        (
            DPT::new(9u16, Some(23u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_9_23>(value)?))
            },
        ),
        (
            DPT::new(9u16, Some(24u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_9_24>(value)?))
            },
        ),
        (
            DPT::new(9u16, Some(25u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_9_25>(value)?))
            },
        ),
        (
            DPT::new(9u16, Some(26u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_9_26>(value)?))
            },
        ),
        (
            DPT::new(9u16, Some(27u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_9_27>(value)?))
            },
        ),
        (
            DPT::new(9u16, Some(28u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_9_28>(value)?))
            },
        ),
        (
            DPT::new(9u16, Some(29u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_9_29>(value)?))
            },
        ),
        (
            DPT::new(9u16, Some(30u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_9_30>(value)?))
            },
        ),
        (
            DPT::new(9u16, Some(31u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_9_31>(value)?))
            },
        ),
        (
            DPT::new(10u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_10_x>(value)?))
            },
        ),
        (
            DPT::new(10u16, Some(1u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_10_1>(value)?))
            },
        ),
        (
            DPT::new(11u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_11_x>(value)?))
            },
        ),
        (
            DPT::new(11u16, Some(1u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_11_1>(value)?))
            },
        ),
        (
            DPT::new(12u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_12_x>(value)?))
            },
        ),
        (
            DPT::new(12u16, Some(1u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_12_1>(value)?))
            },
        ),
        (
            DPT::new(12u16, Some(100u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_12_100>(value)?))
            },
        ),
        (
            DPT::new(12u16, Some(101u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_12_101>(value)?))
            },
        ),
        (
            DPT::new(12u16, Some(102u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_12_102>(value)?))
            },
        ),
        (
            DPT::new(12u16, Some(1200u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_12_1200>(value)?))
            },
        ),
        (
            DPT::new(12u16, Some(1201u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_12_1201>(value)?))
            },
        ),
        (
            DPT::new(13u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_13_x>(value)?))
            },
        ),
        (
            DPT::new(13u16, Some(1u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_13_1>(value)?))
            },
        ),
        (
            DPT::new(13u16, Some(2u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_13_2>(value)?))
            },
        ),
        (
            DPT::new(13u16, Some(10u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_13_10>(value)?))
            },
        ),
        (
            DPT::new(13u16, Some(11u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_13_11>(value)?))
            },
        ),
        (
            DPT::new(13u16, Some(12u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_13_12>(value)?))
            },
        ),
        (
            DPT::new(13u16, Some(13u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_13_13>(value)?))
            },
        ),
        (
            DPT::new(13u16, Some(14u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_13_14>(value)?))
            },
        ),
        (
            DPT::new(13u16, Some(15u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_13_15>(value)?))
            },
        ),
        (
            DPT::new(13u16, Some(16u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_13_16>(value)?))
            },
        ),
        (
            DPT::new(13u16, Some(100u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_13_100>(value)?))
            },
        ),
        (
            DPT::new(13u16, Some(1200u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_13_1200>(value)?))
            },
        ),
        (
            DPT::new(13u16, Some(1201u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_13_1201>(value)?))
            },
        ),
        (
            DPT::new(14u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_x>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(0u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_0>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(1u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_1>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(2u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_2>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(3u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_3>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(4u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_4>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(5u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_5>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(6u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_6>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(7u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_7>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(8u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_8>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(9u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_9>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(10u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_10>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(11u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_11>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(12u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_12>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(13u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_13>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(14u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_14>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(15u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_15>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(16u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_16>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(17u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_17>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(18u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_18>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(19u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_19>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(20u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_20>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(21u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_21>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(22u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_22>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(23u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_23>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(24u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_24>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(25u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_25>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(26u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_26>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(27u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_27>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(28u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_28>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(29u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_29>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(30u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_30>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(31u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_31>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(32u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_32>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(33u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_33>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(34u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_34>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(35u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_35>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(36u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_36>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(37u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_37>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(38u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_38>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(39u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_39>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(40u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_40>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(41u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_41>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(42u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_42>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(43u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_43>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(44u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_44>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(45u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_45>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(46u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_46>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(47u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_47>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(48u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_48>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(49u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_49>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(50u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_50>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(51u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_51>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(52u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_52>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(53u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_53>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(54u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_54>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(55u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_55>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(56u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_56>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(57u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_57>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(58u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_58>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(59u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_59>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(60u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_60>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(61u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_61>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(62u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_62>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(63u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_63>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(64u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_64>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(65u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_65>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(66u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_66>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(67u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_67>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(68u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_68>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(69u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_69>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(70u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_70>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(71u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_71>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(72u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_72>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(73u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_73>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(74u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_74>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(75u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_75>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(76u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_76>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(77u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_77>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(78u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_78>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(79u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_79>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(80u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_80>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(1200u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_1200>(value)?))
            },
        ),
        (
            DPT::new(14u16, Some(1201u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_14_1201>(value)?))
            },
        ),
        (
            DPT::new(15u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_15_x>(value)?))
            },
        ),
        (
            DPT::new(15u16, Some(0u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_15_0>(value)?))
            },
        ),
        (
            DPT::new(16u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_16_x>(value)?))
            },
        ),
        (
            DPT::new(16u16, Some(0u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_16_0>(value)?))
            },
        ),
        (
            DPT::new(16u16, Some(1u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_16_1>(value)?))
            },
        ),
        (
            DPT::new(17u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_17_x>(value)?))
            },
        ),
        (
            DPT::new(17u16, Some(1u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_17_1>(value)?))
            },
        ),
        (
            DPT::new(18u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_18_x>(value)?))
            },
        ),
        (
            DPT::new(18u16, Some(1u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_18_1>(value)?))
            },
        ),
        (
            DPT::new(19u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_19_x>(value)?))
            },
        ),
        (
            DPT::new(19u16, Some(1u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_19_1>(value)?))
            },
        ),
        (
            DPT::new(20u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_x>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(1u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_1>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(2u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_2>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(3u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_3>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(4u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_4>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(5u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_5>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(6u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_6>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(7u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_7>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(8u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_8>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(11u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_11>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(12u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_12>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(13u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_13>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(14u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_14>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(17u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_17>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(20u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_20>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(21u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_21>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(22u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_22>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(100u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_100>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(101u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_101>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(102u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_102>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(103u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_103>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(104u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_104>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(105u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_105>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(106u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_106>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(107u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_107>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(108u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_108>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(109u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_109>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(110u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_110>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(111u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_111>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(112u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_112>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(113u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_113>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(114u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_114>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(115u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_115>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(116u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_116>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(120u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_120>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(121u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_121>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(122u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_122>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(600u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_600>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(601u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_601>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(602u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_602>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(603u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_603>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(604u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_604>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(605u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_605>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(606u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_606>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(607u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_607>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(608u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_608>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(609u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_609>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(610u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_610>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(611u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_611>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(612u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_612>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(801u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_801>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(802u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_802>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(803u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_803>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(804u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_804>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(1000u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_1000>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(1001u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_1001>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(1002u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_1002>(value)?))
            },
        ),
        (
            DPT::new(20u16, Some(1003u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_20_1003>(value)?))
            },
        ),
        (
            DPT::new(21u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_21_x>(value)?))
            },
        ),
        (
            DPT::new(21u16, Some(1u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_21_1>(value)?))
            },
        ),
        (
            DPT::new(21u16, Some(2u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_21_2>(value)?))
            },
        ),
        (
            DPT::new(21u16, Some(100u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_21_100>(value)?))
            },
        ),
        (
            DPT::new(21u16, Some(101u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_21_101>(value)?))
            },
        ),
        (
            DPT::new(21u16, Some(102u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_21_102>(value)?))
            },
        ),
        (
            DPT::new(21u16, Some(103u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_21_103>(value)?))
            },
        ),
        (
            DPT::new(21u16, Some(104u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_21_104>(value)?))
            },
        ),
        (
            DPT::new(21u16, Some(105u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_21_105>(value)?))
            },
        ),
        (
            DPT::new(21u16, Some(106u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_21_106>(value)?))
            },
        ),
        (
            DPT::new(21u16, Some(107u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_21_107>(value)?))
            },
        ),
        (
            DPT::new(21u16, Some(601u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_21_601>(value)?))
            },
        ),
        (
            DPT::new(21u16, Some(1000u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_21_1000>(value)?))
            },
        ),
        (
            DPT::new(21u16, Some(1001u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_21_1001>(value)?))
            },
        ),
        (
            DPT::new(21u16, Some(1010u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_21_1010>(value)?))
            },
        ),
        (
            DPT::new(22u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_22_x>(value)?))
            },
        ),
        (
            DPT::new(22u16, Some(100u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_22_100>(value)?))
            },
        ),
        (
            DPT::new(22u16, Some(101u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_22_101>(value)?))
            },
        ),
        (
            DPT::new(22u16, Some(102u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_22_102>(value)?))
            },
        ),
        (
            DPT::new(22u16, Some(103u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_22_103>(value)?))
            },
        ),
        (
            DPT::new(22u16, Some(1000u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_22_1000>(value)?))
            },
        ),
        (
            DPT::new(22u16, Some(1010u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_22_1010>(value)?))
            },
        ),
        (
            DPT::new(23u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_23_x>(value)?))
            },
        ),
        (
            DPT::new(23u16, Some(1u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_23_1>(value)?))
            },
        ),
        (
            DPT::new(23u16, Some(2u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_23_2>(value)?))
            },
        ),
        (
            DPT::new(23u16, Some(3u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_23_3>(value)?))
            },
        ),
        (
            DPT::new(23u16, Some(102u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_23_102>(value)?))
            },
        ),
        (
            DPT::new(24u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_24_x>(value)?))
            },
        ),
        (
            DPT::new(24u16, Some(1u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_24_1>(value)?))
            },
        ),
        (
            DPT::new(25u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_25_x>(value)?))
            },
        ),
        (
            DPT::new(25u16, Some(1000u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_25_1000>(value)?))
            },
        ),
        (
            DPT::new(26u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_26_x>(value)?))
            },
        ),
        (
            DPT::new(26u16, Some(1u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_26_1>(value)?))
            },
        ),
        (
            DPT::new(27u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_27_x>(value)?))
            },
        ),
        (
            DPT::new(27u16, Some(1u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_27_1>(value)?))
            },
        ),
        (
            DPT::new(28u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_28_x>(value)?))
            },
        ),
        (
            DPT::new(28u16, Some(1u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_28_1>(value)?))
            },
        ),
        (
            DPT::new(29u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_29_x>(value)?))
            },
        ),
        (
            DPT::new(29u16, Some(10u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_29_10>(value)?))
            },
        ),
        (
            DPT::new(29u16, Some(11u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_29_11>(value)?))
            },
        ),
        (
            DPT::new(29u16, Some(12u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_29_12>(value)?))
            },
        ),
        (
            DPT::new(30u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_30_x>(value)?))
            },
        ),
        (
            DPT::new(30u16, Some(1010u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_30_1010>(value)?))
            },
        ),
        (
            DPT::new(206u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_206_x>(value)?))
            },
        ),
        (
            DPT::new(206u16, Some(100u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_206_100>(value)?))
            },
        ),
        (
            DPT::new(206u16, Some(102u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_206_102>(value)?))
            },
        ),
        (
            DPT::new(206u16, Some(104u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_206_104>(value)?))
            },
        ),
        (
            DPT::new(206u16, Some(105u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_206_105>(value)?))
            },
        ),
        (
            DPT::new(207u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_207_x>(value)?))
            },
        ),
        (
            DPT::new(207u16, Some(600u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_207_600>(value)?))
            },
        ),
        (
            DPT::new(217u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_217_x>(value)?))
            },
        ),
        (
            DPT::new(217u16, Some(1u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_217_1>(value)?))
            },
        ),
        (
            DPT::new(219u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_219_x>(value)?))
            },
        ),
        (
            DPT::new(219u16, Some(1u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_219_1>(value)?))
            },
        ),
        (
            DPT::new(222u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_222_x>(value)?))
            },
        ),
        (
            DPT::new(222u16, Some(100u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_222_100>(value)?))
            },
        ),
        (
            DPT::new(222u16, Some(101u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_222_101>(value)?))
            },
        ),
        (
            DPT::new(225u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_225_x>(value)?))
            },
        ),
        (
            DPT::new(225u16, Some(1u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_225_1>(value)?))
            },
        ),
        (
            DPT::new(225u16, Some(2u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_225_2>(value)?))
            },
        ),
        (
            DPT::new(229u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_229_x>(value)?))
            },
        ),
        (
            DPT::new(229u16, Some(1u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_229_1>(value)?))
            },
        ),
        (
            DPT::new(230u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_230_x>(value)?))
            },
        ),
        (
            DPT::new(230u16, Some(1000u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_230_1000>(value)?))
            },
        ),
        (
            DPT::new(232u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_232_x>(value)?))
            },
        ),
        (
            DPT::new(232u16, Some(600u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_232_600>(value)?))
            },
        ),
        (
            DPT::new(234u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_234_x>(value)?))
            },
        ),
        (
            DPT::new(234u16, Some(1u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_234_1>(value)?))
            },
        ),
        (
            DPT::new(235u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_235_x>(value)?))
            },
        ),
        (
            DPT::new(235u16, Some(1u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_235_1>(value)?))
            },
        ),
        (
            DPT::new(236u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_236_x>(value)?))
            },
        ),
        (
            DPT::new(236u16, Some(1u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_236_1>(value)?))
            },
        ),
        (
            DPT::new(237u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_237_x>(value)?))
            },
        ),
        (
            DPT::new(237u16, Some(600u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_237_600>(value)?))
            },
        ),
        (
            DPT::new(238u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_238_x>(value)?))
            },
        ),
        (
            DPT::new(238u16, Some(600u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_238_600>(value)?))
            },
        ),
        (
            DPT::new(240u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_240_x>(value)?))
            },
        ),
        (
            DPT::new(240u16, Some(800u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_240_800>(value)?))
            },
        ),
        (
            DPT::new(241u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_241_x>(value)?))
            },
        ),
        (
            DPT::new(241u16, Some(800u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_241_800>(value)?))
            },
        ),
        (
            DPT::new(242u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_242_x>(value)?))
            },
        ),
        (
            DPT::new(242u16, Some(600u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_242_600>(value)?))
            },
        ),
        (
            DPT::new(244u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_244_x>(value)?))
            },
        ),
        (
            DPT::new(244u16, Some(600u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_244_600>(value)?))
            },
        ),
        (
            DPT::new(245u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_245_x>(value)?))
            },
        ),
        (
            DPT::new(245u16, Some(600u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_245_600>(value)?))
            },
        ),
        (
            DPT::new(246u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_246_x>(value)?))
            },
        ),
        (
            DPT::new(246u16, Some(600u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_246_600>(value)?))
            },
        ),
        (
            DPT::new(249u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_249_x>(value)?))
            },
        ),
        (
            DPT::new(249u16, Some(600u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_249_600>(value)?))
            },
        ),
        (
            DPT::new(250u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_250_x>(value)?))
            },
        ),
        (
            DPT::new(250u16, Some(600u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_250_600>(value)?))
            },
        ),
        (
            DPT::new(251u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_251_x>(value)?))
            },
        ),
        (
            DPT::new(251u16, Some(600u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_251_600>(value)?))
            },
        ),
        (
            DPT::new(252u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_252_x>(value)?))
            },
        ),
        (
            DPT::new(252u16, Some(600u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_252_600>(value)?))
            },
        ),
        (
            DPT::new(254u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_254_x>(value)?))
            },
        ),
        (
            DPT::new(254u16, Some(600u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_254_600>(value)?))
            },
        ),
        (
            DPT::new(255u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_255_x>(value)?))
            },
        ),
        (
            DPT::new(255u16, Some(1u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_255_1>(value)?))
            },
        ),
        (
            DPT::new(275u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_275_x>(value)?))
            },
        ),
        (
            DPT::new(275u16, Some(100u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_275_100>(value)?))
            },
        ),
        (
            DPT::new(275u16, Some(101u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_275_101>(value)?))
            },
        ),
        (
            DPT::new(285u16, None),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_285_x>(value)?))
            },
        ),
        (
            DPT::new(285u16, Some(1u16)),
            |value| {
                Ok(OpaqueDataPoint::new(serde_json::from_value::<DPT_285_1>(value)?))
            },
        ),
    ];
    DECODERS
        .binary_search_by_key(&dpt, |x| x.0)
        .map(|ix| DECODERS[ix].1(data))
        .map_err(|_| Error::InvalidDPT(dpt))?
}
