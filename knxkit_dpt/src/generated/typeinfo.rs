// This file is generated, don't manually edit it!
use knxkit::project::DPT;
use crate::DataPointTypeInfo;
pub fn lookup(dpt: DPT) -> Option<&'static DataPointTypeInfo> {
    static DATAPOINTS: &'static [(DPT, DataPointTypeInfo)] = &[
        (
            DPT::new(1u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(1u16, None),
                name: "1.xxx",
                text: Some("1-bit"),
                unit: None,
            },
        ),
        (
            DPT::new(1u16, Some(1u16)),
            DataPointTypeInfo {
                dpt: DPT::new(1u16, Some(1u16)),
                name: "DPT_Switch",
                text: Some("switch"),
                unit: None,
            },
        ),
        (
            DPT::new(1u16, Some(2u16)),
            DataPointTypeInfo {
                dpt: DPT::new(1u16, Some(2u16)),
                name: "DPT_Bool",
                text: Some("boolean"),
                unit: None,
            },
        ),
        (
            DPT::new(1u16, Some(3u16)),
            DataPointTypeInfo {
                dpt: DPT::new(1u16, Some(3u16)),
                name: "DPT_Enable",
                text: Some("enable"),
                unit: None,
            },
        ),
        (
            DPT::new(1u16, Some(4u16)),
            DataPointTypeInfo {
                dpt: DPT::new(1u16, Some(4u16)),
                name: "DPT_Ramp",
                text: Some("ramp"),
                unit: None,
            },
        ),
        (
            DPT::new(1u16, Some(5u16)),
            DataPointTypeInfo {
                dpt: DPT::new(1u16, Some(5u16)),
                name: "DPT_Alarm",
                text: Some("alarm"),
                unit: None,
            },
        ),
        (
            DPT::new(1u16, Some(6u16)),
            DataPointTypeInfo {
                dpt: DPT::new(1u16, Some(6u16)),
                name: "DPT_BinaryValue",
                text: Some("binary value"),
                unit: None,
            },
        ),
        (
            DPT::new(1u16, Some(7u16)),
            DataPointTypeInfo {
                dpt: DPT::new(1u16, Some(7u16)),
                name: "DPT_Step",
                text: Some("step"),
                unit: None,
            },
        ),
        (
            DPT::new(1u16, Some(8u16)),
            DataPointTypeInfo {
                dpt: DPT::new(1u16, Some(8u16)),
                name: "DPT_UpDown",
                text: Some("up/down"),
                unit: None,
            },
        ),
        (
            DPT::new(1u16, Some(9u16)),
            DataPointTypeInfo {
                dpt: DPT::new(1u16, Some(9u16)),
                name: "DPT_OpenClose",
                text: Some("open/close"),
                unit: None,
            },
        ),
        (
            DPT::new(1u16, Some(10u16)),
            DataPointTypeInfo {
                dpt: DPT::new(1u16, Some(10u16)),
                name: "DPT_Start",
                text: Some("start/stop"),
                unit: None,
            },
        ),
        (
            DPT::new(1u16, Some(11u16)),
            DataPointTypeInfo {
                dpt: DPT::new(1u16, Some(11u16)),
                name: "DPT_State",
                text: Some("state"),
                unit: None,
            },
        ),
        (
            DPT::new(1u16, Some(12u16)),
            DataPointTypeInfo {
                dpt: DPT::new(1u16, Some(12u16)),
                name: "DPT_Invert",
                text: Some("invert"),
                unit: None,
            },
        ),
        (
            DPT::new(1u16, Some(13u16)),
            DataPointTypeInfo {
                dpt: DPT::new(1u16, Some(13u16)),
                name: "DPT_DimSendStyle",
                text: Some("dim send style"),
                unit: None,
            },
        ),
        (
            DPT::new(1u16, Some(14u16)),
            DataPointTypeInfo {
                dpt: DPT::new(1u16, Some(14u16)),
                name: "DPT_InputSource",
                text: Some("input source"),
                unit: None,
            },
        ),
        (
            DPT::new(1u16, Some(15u16)),
            DataPointTypeInfo {
                dpt: DPT::new(1u16, Some(15u16)),
                name: "DPT_Reset",
                text: Some("reset"),
                unit: None,
            },
        ),
        (
            DPT::new(1u16, Some(16u16)),
            DataPointTypeInfo {
                dpt: DPT::new(1u16, Some(16u16)),
                name: "DPT_Ack",
                text: Some("acknowledge"),
                unit: None,
            },
        ),
        (
            DPT::new(1u16, Some(17u16)),
            DataPointTypeInfo {
                dpt: DPT::new(1u16, Some(17u16)),
                name: "DPT_Trigger",
                text: Some("trigger"),
                unit: None,
            },
        ),
        (
            DPT::new(1u16, Some(18u16)),
            DataPointTypeInfo {
                dpt: DPT::new(1u16, Some(18u16)),
                name: "DPT_Occupancy",
                text: Some("occupancy"),
                unit: None,
            },
        ),
        (
            DPT::new(1u16, Some(19u16)),
            DataPointTypeInfo {
                dpt: DPT::new(1u16, Some(19u16)),
                name: "DPT_Window_Door",
                text: Some("window/door"),
                unit: None,
            },
        ),
        (
            DPT::new(1u16, Some(21u16)),
            DataPointTypeInfo {
                dpt: DPT::new(1u16, Some(21u16)),
                name: "DPT_LogicalFunction",
                text: Some("logical function"),
                unit: None,
            },
        ),
        (
            DPT::new(1u16, Some(22u16)),
            DataPointTypeInfo {
                dpt: DPT::new(1u16, Some(22u16)),
                name: "DPT_Scene_AB",
                text: Some("scene"),
                unit: None,
            },
        ),
        (
            DPT::new(1u16, Some(23u16)),
            DataPointTypeInfo {
                dpt: DPT::new(1u16, Some(23u16)),
                name: "DPT_ShutterBlinds_Mode",
                text: Some("shutter/blinds mode"),
                unit: None,
            },
        ),
        (
            DPT::new(1u16, Some(24u16)),
            DataPointTypeInfo {
                dpt: DPT::new(1u16, Some(24u16)),
                name: "DPT_DayNight",
                text: Some("day/night"),
                unit: None,
            },
        ),
        (
            DPT::new(1u16, Some(100u16)),
            DataPointTypeInfo {
                dpt: DPT::new(1u16, Some(100u16)),
                name: "DPT_Heat_Cool",
                text: Some("cooling/heating"),
                unit: None,
            },
        ),
        (
            DPT::new(2u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(2u16, None),
                name: "2.xxx",
                text: Some("1-bit controlled"),
                unit: None,
            },
        ),
        (
            DPT::new(2u16, Some(1u16)),
            DataPointTypeInfo {
                dpt: DPT::new(2u16, Some(1u16)),
                name: "DPT_Switch_Control",
                text: Some("switch control"),
                unit: None,
            },
        ),
        (
            DPT::new(2u16, Some(2u16)),
            DataPointTypeInfo {
                dpt: DPT::new(2u16, Some(2u16)),
                name: "DPT_Bool_Control",
                text: Some("boolean control"),
                unit: None,
            },
        ),
        (
            DPT::new(2u16, Some(3u16)),
            DataPointTypeInfo {
                dpt: DPT::new(2u16, Some(3u16)),
                name: "DPT_Enable_Control",
                text: Some("enable control"),
                unit: None,
            },
        ),
        (
            DPT::new(2u16, Some(4u16)),
            DataPointTypeInfo {
                dpt: DPT::new(2u16, Some(4u16)),
                name: "DPT_Ramp_Control",
                text: Some("ramp control"),
                unit: None,
            },
        ),
        (
            DPT::new(2u16, Some(5u16)),
            DataPointTypeInfo {
                dpt: DPT::new(2u16, Some(5u16)),
                name: "DPT_Alarm_Control",
                text: Some("alarm control"),
                unit: None,
            },
        ),
        (
            DPT::new(2u16, Some(6u16)),
            DataPointTypeInfo {
                dpt: DPT::new(2u16, Some(6u16)),
                name: "DPT_BinaryValue_Control",
                text: Some("binary value control"),
                unit: None,
            },
        ),
        (
            DPT::new(2u16, Some(7u16)),
            DataPointTypeInfo {
                dpt: DPT::new(2u16, Some(7u16)),
                name: "DPT_Step_Control",
                text: Some("step control"),
                unit: None,
            },
        ),
        (
            DPT::new(2u16, Some(8u16)),
            DataPointTypeInfo {
                dpt: DPT::new(2u16, Some(8u16)),
                name: "DPT_Direction1_Control",
                text: Some("direction control 1"),
                unit: None,
            },
        ),
        (
            DPT::new(2u16, Some(9u16)),
            DataPointTypeInfo {
                dpt: DPT::new(2u16, Some(9u16)),
                name: "DPT_Direction2_Control",
                text: Some("direction control 2"),
                unit: None,
            },
        ),
        (
            DPT::new(2u16, Some(10u16)),
            DataPointTypeInfo {
                dpt: DPT::new(2u16, Some(10u16)),
                name: "DPT_Start_Control",
                text: Some("start control"),
                unit: None,
            },
        ),
        (
            DPT::new(2u16, Some(11u16)),
            DataPointTypeInfo {
                dpt: DPT::new(2u16, Some(11u16)),
                name: "DPT_State_Control",
                text: Some("state control"),
                unit: None,
            },
        ),
        (
            DPT::new(2u16, Some(12u16)),
            DataPointTypeInfo {
                dpt: DPT::new(2u16, Some(12u16)),
                name: "DPT_Invert_Control",
                text: Some("invert control"),
                unit: None,
            },
        ),
        (
            DPT::new(3u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(3u16, None),
                name: "3.xxx",
                text: Some("3-bit controlled"),
                unit: None,
            },
        ),
        (
            DPT::new(3u16, Some(7u16)),
            DataPointTypeInfo {
                dpt: DPT::new(3u16, Some(7u16)),
                name: "DPT_Control_Dimming",
                text: Some("dimming control"),
                unit: None,
            },
        ),
        (
            DPT::new(3u16, Some(8u16)),
            DataPointTypeInfo {
                dpt: DPT::new(3u16, Some(8u16)),
                name: "DPT_Control_Blinds",
                text: Some("blind control"),
                unit: None,
            },
        ),
        (
            DPT::new(4u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(4u16, None),
                name: "4.xxx",
                text: Some("character"),
                unit: None,
            },
        ),
        (
            DPT::new(4u16, Some(1u16)),
            DataPointTypeInfo {
                dpt: DPT::new(4u16, Some(1u16)),
                name: "DPT_Char_ASCII",
                text: Some("character (ASCII)"),
                unit: None,
            },
        ),
        (
            DPT::new(4u16, Some(2u16)),
            DataPointTypeInfo {
                dpt: DPT::new(4u16, Some(2u16)),
                name: "DPT_Char_8859_1",
                text: Some("character (ISO 8859-1)"),
                unit: None,
            },
        ),
        (
            DPT::new(5u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(5u16, None),
                name: "5.xxx",
                text: Some("8-bit unsigned value"),
                unit: Some("%"),
            },
        ),
        (
            DPT::new(5u16, Some(1u16)),
            DataPointTypeInfo {
                dpt: DPT::new(5u16, Some(1u16)),
                name: "DPT_Scaling",
                text: Some("percentage (0..100%)"),
                unit: Some("%"),
            },
        ),
        (
            DPT::new(5u16, Some(3u16)),
            DataPointTypeInfo {
                dpt: DPT::new(5u16, Some(3u16)),
                name: "DPT_Angle",
                text: Some("angle (degrees)"),
                unit: Some("°"),
            },
        ),
        (
            DPT::new(5u16, Some(4u16)),
            DataPointTypeInfo {
                dpt: DPT::new(5u16, Some(4u16)),
                name: "DPT_Percent_U8",
                text: Some("percentage (0..255%)"),
                unit: Some("%"),
            },
        ),
        (
            DPT::new(5u16, Some(5u16)),
            DataPointTypeInfo {
                dpt: DPT::new(5u16, Some(5u16)),
                name: "DPT_DecimalFactor",
                text: Some("ratio (0..255)"),
                unit: Some(""),
            },
        ),
        (
            DPT::new(5u16, Some(6u16)),
            DataPointTypeInfo {
                dpt: DPT::new(5u16, Some(6u16)),
                name: "DPT_Tariff",
                text: Some("tariff (0..255)"),
                unit: Some(""),
            },
        ),
        (
            DPT::new(5u16, Some(10u16)),
            DataPointTypeInfo {
                dpt: DPT::new(5u16, Some(10u16)),
                name: "DPT_Value_1_Ucount",
                text: Some("counter pulses (0..255)"),
                unit: Some("counter pulses"),
            },
        ),
        (
            DPT::new(5u16, Some(100u16)),
            DataPointTypeInfo {
                dpt: DPT::new(5u16, Some(100u16)),
                name: "DPT_FanStage",
                text: Some("fan stage (0..255)"),
                unit: Some("fan stage"),
            },
        ),
        (
            DPT::new(6u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(6u16, None),
                name: "6.xxx",
                text: Some("8-bit signed value"),
                unit: Some("%"),
            },
        ),
        (
            DPT::new(6u16, Some(1u16)),
            DataPointTypeInfo {
                dpt: DPT::new(6u16, Some(1u16)),
                name: "DPT_Percent_V8",
                text: Some("percentage (-128..127%)"),
                unit: Some("%"),
            },
        ),
        (
            DPT::new(6u16, Some(10u16)),
            DataPointTypeInfo {
                dpt: DPT::new(6u16, Some(10u16)),
                name: "DPT_Value_1_Count",
                text: Some("counter pulses (-128..127)"),
                unit: Some("counter pulses"),
            },
        ),
        (
            DPT::new(6u16, Some(20u16)),
            DataPointTypeInfo {
                dpt: DPT::new(6u16, Some(20u16)),
                name: "DPT_Status_Mode3",
                text: Some("status with mode"),
                unit: None,
            },
        ),
        (
            DPT::new(7u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(7u16, None),
                name: "7.xxx",
                text: Some("2-byte unsigned value"),
                unit: Some("pulses"),
            },
        ),
        (
            DPT::new(7u16, Some(1u16)),
            DataPointTypeInfo {
                dpt: DPT::new(7u16, Some(1u16)),
                name: "DPT_Value_2_Ucount",
                text: Some("pulses"),
                unit: Some("pulses"),
            },
        ),
        (
            DPT::new(7u16, Some(2u16)),
            DataPointTypeInfo {
                dpt: DPT::new(7u16, Some(2u16)),
                name: "DPT_TimePeriodMsec",
                text: Some("time (ms)"),
                unit: Some("ms"),
            },
        ),
        (
            DPT::new(7u16, Some(3u16)),
            DataPointTypeInfo {
                dpt: DPT::new(7u16, Some(3u16)),
                name: "DPT_TimePeriod10Msec",
                text: Some("time (10 ms)"),
                unit: Some("ms"),
            },
        ),
        (
            DPT::new(7u16, Some(4u16)),
            DataPointTypeInfo {
                dpt: DPT::new(7u16, Some(4u16)),
                name: "DPT_TimePeriod100Msec",
                text: Some("time (100 ms)"),
                unit: Some("ms"),
            },
        ),
        (
            DPT::new(7u16, Some(5u16)),
            DataPointTypeInfo {
                dpt: DPT::new(7u16, Some(5u16)),
                name: "DPT_TimePeriodSec",
                text: Some("time (s)"),
                unit: Some("s"),
            },
        ),
        (
            DPT::new(7u16, Some(6u16)),
            DataPointTypeInfo {
                dpt: DPT::new(7u16, Some(6u16)),
                name: "DPT_TimePeriodMin",
                text: Some("time (min)"),
                unit: Some("min"),
            },
        ),
        (
            DPT::new(7u16, Some(7u16)),
            DataPointTypeInfo {
                dpt: DPT::new(7u16, Some(7u16)),
                name: "DPT_TimePeriodHrs",
                text: Some("time (h)"),
                unit: Some("h"),
            },
        ),
        (
            DPT::new(7u16, Some(10u16)),
            DataPointTypeInfo {
                dpt: DPT::new(7u16, Some(10u16)),
                name: "DPT_PropDataType",
                text: Some("property data type"),
                unit: None,
            },
        ),
        (
            DPT::new(7u16, Some(11u16)),
            DataPointTypeInfo {
                dpt: DPT::new(7u16, Some(11u16)),
                name: "DPT_Length_mm",
                text: Some("length (mm)"),
                unit: Some("mm"),
            },
        ),
        (
            DPT::new(7u16, Some(12u16)),
            DataPointTypeInfo {
                dpt: DPT::new(7u16, Some(12u16)),
                name: "DPT_UElCurrentmA",
                text: Some("current (mA)"),
                unit: Some("mA"),
            },
        ),
        (
            DPT::new(7u16, Some(13u16)),
            DataPointTypeInfo {
                dpt: DPT::new(7u16, Some(13u16)),
                name: "DPT_Brightness",
                text: Some("brightness (lux)"),
                unit: Some("lux"),
            },
        ),
        (
            DPT::new(7u16, Some(600u16)),
            DataPointTypeInfo {
                dpt: DPT::new(7u16, Some(600u16)),
                name: "DPT_Absolute_Colour_Temperature",
                text: Some("absolute colour temperature (K)"),
                unit: Some("K"),
            },
        ),
        (
            DPT::new(8u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(8u16, None),
                name: "8.xxx",
                text: Some("2-byte signed value"),
                unit: Some("pulses"),
            },
        ),
        (
            DPT::new(8u16, Some(1u16)),
            DataPointTypeInfo {
                dpt: DPT::new(8u16, Some(1u16)),
                name: "DPT_Value_2_Count",
                text: Some("pulses difference"),
                unit: Some("pulses"),
            },
        ),
        (
            DPT::new(8u16, Some(2u16)),
            DataPointTypeInfo {
                dpt: DPT::new(8u16, Some(2u16)),
                name: "DPT_DeltaTimeMsec",
                text: Some("time lag (ms)"),
                unit: Some("ms"),
            },
        ),
        (
            DPT::new(8u16, Some(3u16)),
            DataPointTypeInfo {
                dpt: DPT::new(8u16, Some(3u16)),
                name: "DPT_DeltaTime10Msec",
                text: Some("time lag(10 ms)"),
                unit: Some("ms"),
            },
        ),
        (
            DPT::new(8u16, Some(4u16)),
            DataPointTypeInfo {
                dpt: DPT::new(8u16, Some(4u16)),
                name: "DPT_DeltaTime100Msec",
                text: Some("time lag (100 ms)"),
                unit: Some("ms"),
            },
        ),
        (
            DPT::new(8u16, Some(5u16)),
            DataPointTypeInfo {
                dpt: DPT::new(8u16, Some(5u16)),
                name: "DPT_DeltaTimeSec",
                text: Some("time lag (s)"),
                unit: Some("s"),
            },
        ),
        (
            DPT::new(8u16, Some(6u16)),
            DataPointTypeInfo {
                dpt: DPT::new(8u16, Some(6u16)),
                name: "DPT_DeltaTimeMin",
                text: Some("time lag (min)"),
                unit: Some("min"),
            },
        ),
        (
            DPT::new(8u16, Some(7u16)),
            DataPointTypeInfo {
                dpt: DPT::new(8u16, Some(7u16)),
                name: "DPT_DeltaTimeHrs",
                text: Some("time lag (h)"),
                unit: Some("h"),
            },
        ),
        (
            DPT::new(8u16, Some(10u16)),
            DataPointTypeInfo {
                dpt: DPT::new(8u16, Some(10u16)),
                name: "DPT_Percent_V16",
                text: Some("percentage difference (%)"),
                unit: Some("%"),
            },
        ),
        (
            DPT::new(8u16, Some(11u16)),
            DataPointTypeInfo {
                dpt: DPT::new(8u16, Some(11u16)),
                name: "DPT_Rotation_Angle",
                text: Some("rotation angle (°)"),
                unit: Some("°"),
            },
        ),
        (
            DPT::new(8u16, Some(12u16)),
            DataPointTypeInfo {
                dpt: DPT::new(8u16, Some(12u16)),
                name: "DPT_Length_m",
                text: Some("length (m)"),
                unit: Some("m"),
            },
        ),
        (
            DPT::new(9u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(9u16, None),
                name: "9.xxx",
                text: Some("2-byte float value"),
                unit: Some("°C"),
            },
        ),
        (
            DPT::new(9u16, Some(1u16)),
            DataPointTypeInfo {
                dpt: DPT::new(9u16, Some(1u16)),
                name: "DPT_Value_Temp",
                text: Some("temperature (°C)"),
                unit: Some("°C"),
            },
        ),
        (
            DPT::new(9u16, Some(2u16)),
            DataPointTypeInfo {
                dpt: DPT::new(9u16, Some(2u16)),
                name: "DPT_Value_Tempd",
                text: Some("temperature difference (K)"),
                unit: Some("K"),
            },
        ),
        (
            DPT::new(9u16, Some(3u16)),
            DataPointTypeInfo {
                dpt: DPT::new(9u16, Some(3u16)),
                name: "DPT_Value_Tempa",
                text: Some("kelvin/hour (K/h)"),
                unit: Some("K/h"),
            },
        ),
        (
            DPT::new(9u16, Some(4u16)),
            DataPointTypeInfo {
                dpt: DPT::new(9u16, Some(4u16)),
                name: "DPT_Value_Lux",
                text: Some("lux (Lux)"),
                unit: Some("Lux"),
            },
        ),
        (
            DPT::new(9u16, Some(5u16)),
            DataPointTypeInfo {
                dpt: DPT::new(9u16, Some(5u16)),
                name: "DPT_Value_Wsp",
                text: Some("speed (m/s)"),
                unit: Some("m/s"),
            },
        ),
        (
            DPT::new(9u16, Some(6u16)),
            DataPointTypeInfo {
                dpt: DPT::new(9u16, Some(6u16)),
                name: "DPT_Value_Pres",
                text: Some("pressure (Pa)"),
                unit: Some("Pa"),
            },
        ),
        (
            DPT::new(9u16, Some(7u16)),
            DataPointTypeInfo {
                dpt: DPT::new(9u16, Some(7u16)),
                name: "DPT_Value_Humidity",
                text: Some("humidity (%)"),
                unit: Some("%"),
            },
        ),
        (
            DPT::new(9u16, Some(8u16)),
            DataPointTypeInfo {
                dpt: DPT::new(9u16, Some(8u16)),
                name: "DPT_Value_AirQuality",
                text: Some("parts/million (ppm)"),
                unit: Some("ppm"),
            },
        ),
        (
            DPT::new(9u16, Some(9u16)),
            DataPointTypeInfo {
                dpt: DPT::new(9u16, Some(9u16)),
                name: "DPT_Value_AirFlow",
                text: Some("air flow (m³/h)"),
                unit: Some("m³/h"),
            },
        ),
        (
            DPT::new(9u16, Some(10u16)),
            DataPointTypeInfo {
                dpt: DPT::new(9u16, Some(10u16)),
                name: "DPT_Value_Time1",
                text: Some("time (s)"),
                unit: Some("s"),
            },
        ),
        (
            DPT::new(9u16, Some(11u16)),
            DataPointTypeInfo {
                dpt: DPT::new(9u16, Some(11u16)),
                name: "DPT_Value_Time2",
                text: Some("time (ms)"),
                unit: Some("ms"),
            },
        ),
        (
            DPT::new(9u16, Some(20u16)),
            DataPointTypeInfo {
                dpt: DPT::new(9u16, Some(20u16)),
                name: "DPT_Value_Volt",
                text: Some("voltage (mV)"),
                unit: Some("mV"),
            },
        ),
        (
            DPT::new(9u16, Some(21u16)),
            DataPointTypeInfo {
                dpt: DPT::new(9u16, Some(21u16)),
                name: "DPT_Value_Curr",
                text: Some("current (mA)"),
                unit: Some("mA"),
            },
        ),
        (
            DPT::new(9u16, Some(22u16)),
            DataPointTypeInfo {
                dpt: DPT::new(9u16, Some(22u16)),
                name: "DPT_PowerDensity",
                text: Some("power density (W/m²)"),
                unit: Some("W/m²"),
            },
        ),
        (
            DPT::new(9u16, Some(23u16)),
            DataPointTypeInfo {
                dpt: DPT::new(9u16, Some(23u16)),
                name: "DPT_KelvinPerPercent",
                text: Some("kelvin/percent (K/%)"),
                unit: Some("K/%"),
            },
        ),
        (
            DPT::new(9u16, Some(24u16)),
            DataPointTypeInfo {
                dpt: DPT::new(9u16, Some(24u16)),
                name: "DPT_Power",
                text: Some("power (kW)"),
                unit: Some("kW"),
            },
        ),
        (
            DPT::new(9u16, Some(25u16)),
            DataPointTypeInfo {
                dpt: DPT::new(9u16, Some(25u16)),
                name: "DPT_Value_Volume_Flow",
                text: Some("volume flow (l/h)"),
                unit: Some("l/h"),
            },
        ),
        (
            DPT::new(9u16, Some(26u16)),
            DataPointTypeInfo {
                dpt: DPT::new(9u16, Some(26u16)),
                name: "DPT_Rain_Amount",
                text: Some("rain amount (l/m²)"),
                unit: Some("l/m²"),
            },
        ),
        (
            DPT::new(9u16, Some(27u16)),
            DataPointTypeInfo {
                dpt: DPT::new(9u16, Some(27u16)),
                name: "DPT_Value_Temp_F",
                text: Some("temperature (°F)"),
                unit: Some("°F"),
            },
        ),
        (
            DPT::new(9u16, Some(28u16)),
            DataPointTypeInfo {
                dpt: DPT::new(9u16, Some(28u16)),
                name: "DPT_Value_Wsp_kmh",
                text: Some("wind speed (km/h)"),
                unit: Some("km/h"),
            },
        ),
        (
            DPT::new(9u16, Some(29u16)),
            DataPointTypeInfo {
                dpt: DPT::new(9u16, Some(29u16)),
                name: "DPT_Value_Absolute_Humidity",
                text: Some("absolute humidity (g/m³)"),
                unit: Some("g/m³"),
            },
        ),
        (
            DPT::new(9u16, Some(30u16)),
            DataPointTypeInfo {
                dpt: DPT::new(9u16, Some(30u16)),
                name: "DPT_Concentration_µgm3",
                text: Some("concentration (µg/m³)"),
                unit: Some("µg/m³"),
            },
        ),
        (
            DPT::new(9u16, Some(31u16)),
            DataPointTypeInfo {
                dpt: DPT::new(9u16, Some(31u16)),
                name: "DPT_Coefficient",
                text: Some("coefficient"),
                unit: Some(""),
            },
        ),
        (
            DPT::new(10u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(10u16, None),
                name: "10.xxx",
                text: Some("time"),
                unit: None,
            },
        ),
        (
            DPT::new(10u16, Some(1u16)),
            DataPointTypeInfo {
                dpt: DPT::new(10u16, Some(1u16)),
                name: "DPT_TimeOfDay",
                text: Some("time of day"),
                unit: None,
            },
        ),
        (
            DPT::new(11u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(11u16, None),
                name: "11.xxx",
                text: Some("date"),
                unit: None,
            },
        ),
        (
            DPT::new(11u16, Some(1u16)),
            DataPointTypeInfo {
                dpt: DPT::new(11u16, Some(1u16)),
                name: "DPT_Date",
                text: Some("date"),
                unit: None,
            },
        ),
        (
            DPT::new(12u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(12u16, None),
                name: "12.xxx",
                text: Some("4-byte unsigned value"),
                unit: Some("counter pulses"),
            },
        ),
        (
            DPT::new(12u16, Some(1u16)),
            DataPointTypeInfo {
                dpt: DPT::new(12u16, Some(1u16)),
                name: "DPT_Value_4_Ucount",
                text: Some("counter pulses (unsigned)"),
                unit: Some("counter pulses"),
            },
        ),
        (
            DPT::new(12u16, Some(100u16)),
            DataPointTypeInfo {
                dpt: DPT::new(12u16, Some(100u16)),
                name: "DPT_LongTimePeriod_Sec",
                text: Some("counter timesec (s)"),
                unit: Some("s"),
            },
        ),
        (
            DPT::new(12u16, Some(101u16)),
            DataPointTypeInfo {
                dpt: DPT::new(12u16, Some(101u16)),
                name: "DPT_LongTimePeriod_Min",
                text: Some("counter timemin (min)"),
                unit: Some("min"),
            },
        ),
        (
            DPT::new(12u16, Some(102u16)),
            DataPointTypeInfo {
                dpt: DPT::new(12u16, Some(102u16)),
                name: "DPT_LongTimePeriod_Hrs",
                text: Some("counter timehrs (h)"),
                unit: Some("h"),
            },
        ),
        (
            DPT::new(12u16, Some(1200u16)),
            DataPointTypeInfo {
                dpt: DPT::new(12u16, Some(1200u16)),
                name: "DPT_VolumeLiquid_Litre",
                text: Some("volume liquid (l)"),
                unit: Some("l"),
            },
        ),
        (
            DPT::new(12u16, Some(1201u16)),
            DataPointTypeInfo {
                dpt: DPT::new(12u16, Some(1201u16)),
                name: "DPT_Volume_m³",
                text: Some("volume (m³)"),
                unit: Some("m³"),
            },
        ),
        (
            DPT::new(13u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(13u16, None),
                name: "13.xxx",
                text: Some("4-byte signed value"),
                unit: Some("counter pulses"),
            },
        ),
        (
            DPT::new(13u16, Some(1u16)),
            DataPointTypeInfo {
                dpt: DPT::new(13u16, Some(1u16)),
                name: "DPT_Value_4_Count",
                text: Some("counter pulses (signed)"),
                unit: Some("counter pulses"),
            },
        ),
        (
            DPT::new(13u16, Some(2u16)),
            DataPointTypeInfo {
                dpt: DPT::new(13u16, Some(2u16)),
                name: "DPT_FlowRate_m3/h",
                text: Some("flow rate (m³/h)"),
                unit: Some("m³/h"),
            },
        ),
        (
            DPT::new(13u16, Some(10u16)),
            DataPointTypeInfo {
                dpt: DPT::new(13u16, Some(10u16)),
                name: "DPT_ActiveEnergy",
                text: Some("active energy (Wh)"),
                unit: Some("Wh"),
            },
        ),
        (
            DPT::new(13u16, Some(11u16)),
            DataPointTypeInfo {
                dpt: DPT::new(13u16, Some(11u16)),
                name: "DPT_ApparantEnergy",
                text: Some("apparant energy (VAh)"),
                unit: Some("VAh"),
            },
        ),
        (
            DPT::new(13u16, Some(12u16)),
            DataPointTypeInfo {
                dpt: DPT::new(13u16, Some(12u16)),
                name: "DPT_ReactiveEnergy",
                text: Some("reactive energy (VARh)"),
                unit: Some("VARh"),
            },
        ),
        (
            DPT::new(13u16, Some(13u16)),
            DataPointTypeInfo {
                dpt: DPT::new(13u16, Some(13u16)),
                name: "DPT_ActiveEnergy_kWh",
                text: Some("active energy (kWh)"),
                unit: Some("kWh"),
            },
        ),
        (
            DPT::new(13u16, Some(14u16)),
            DataPointTypeInfo {
                dpt: DPT::new(13u16, Some(14u16)),
                name: "DPT_ApparantEnergy_kVAh",
                text: Some("apparant energy (kVAh)"),
                unit: Some("kVAh"),
            },
        ),
        (
            DPT::new(13u16, Some(15u16)),
            DataPointTypeInfo {
                dpt: DPT::new(13u16, Some(15u16)),
                name: "DPT_ReactiveEnergy_kVARh",
                text: Some("reactive energy (kVARh)"),
                unit: Some("kVARh"),
            },
        ),
        (
            DPT::new(13u16, Some(16u16)),
            DataPointTypeInfo {
                dpt: DPT::new(13u16, Some(16u16)),
                name: "DPT_ActiveEnergy_MWh",
                text: Some("active energy (MWh)"),
                unit: Some("MWh"),
            },
        ),
        (
            DPT::new(13u16, Some(100u16)),
            DataPointTypeInfo {
                dpt: DPT::new(13u16, Some(100u16)),
                name: "DPT_LongDeltaTimeSec",
                text: Some("time lag (s)"),
                unit: Some("s"),
            },
        ),
        (
            DPT::new(13u16, Some(1200u16)),
            DataPointTypeInfo {
                dpt: DPT::new(13u16, Some(1200u16)),
                name: "DPT_DeltaVolumeLiquid_Litre ",
                text: Some("delta volume liquid (l)"),
                unit: Some("l"),
            },
        ),
        (
            DPT::new(13u16, Some(1201u16)),
            DataPointTypeInfo {
                dpt: DPT::new(13u16, Some(1201u16)),
                name: "DPT_DeltaVolume_m³",
                text: Some("delta volume (m³)"),
                unit: Some("m³"),
            },
        ),
        (
            DPT::new(14u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, None),
                name: "14.xxx",
                text: Some("4-byte float value"),
                unit: Some("m/s²"),
            },
        ),
        (
            DPT::new(14u16, Some(0u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(0u16)),
                name: "DPT_Value_Acceleration",
                text: Some("acceleration (m/s²)"),
                unit: Some("m/s²"),
            },
        ),
        (
            DPT::new(14u16, Some(1u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(1u16)),
                name: "DPT_Value_Acceleration_Angular",
                text: Some("angular acceleration (rad/s²)"),
                unit: Some("rad/s²"),
            },
        ),
        (
            DPT::new(14u16, Some(2u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(2u16)),
                name: "DPT_Value_Activation_Energy",
                text: Some("activation energy (J/mol)"),
                unit: Some("J/mol"),
            },
        ),
        (
            DPT::new(14u16, Some(3u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(3u16)),
                name: "DPT_Value_Activity",
                text: Some("radioactive activity (1/s)"),
                unit: Some("1/s"),
            },
        ),
        (
            DPT::new(14u16, Some(4u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(4u16)),
                name: "DPT_Value_Mol",
                text: Some("amount of substance (mol)"),
                unit: Some("mol"),
            },
        ),
        (
            DPT::new(14u16, Some(5u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(5u16)),
                name: "DPT_Value_Amplitude",
                text: Some("amplitude"),
                unit: None,
            },
        ),
        (
            DPT::new(14u16, Some(6u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(6u16)),
                name: "DPT_Value_AngleRad",
                text: Some("angle (radiant)"),
                unit: Some("rad"),
            },
        ),
        (
            DPT::new(14u16, Some(7u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(7u16)),
                name: "DPT_Value_AngleDeg",
                text: Some("angle (degree)"),
                unit: Some("°"),
            },
        ),
        (
            DPT::new(14u16, Some(8u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(8u16)),
                name: "DPT_Value_Angular_Momentum",
                text: Some("angular momentum (Js)"),
                unit: Some("Js"),
            },
        ),
        (
            DPT::new(14u16, Some(9u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(9u16)),
                name: "DPT_Value_Angular_Velocity",
                text: Some("angular velocity (rad/s)"),
                unit: Some("rad/s"),
            },
        ),
        (
            DPT::new(14u16, Some(10u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(10u16)),
                name: "DPT_Value_Area",
                text: Some("area (m*m)"),
                unit: Some("m²"),
            },
        ),
        (
            DPT::new(14u16, Some(11u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(11u16)),
                name: "DPT_Value_Capacitance",
                text: Some("capacitance (F)"),
                unit: Some("F"),
            },
        ),
        (
            DPT::new(14u16, Some(12u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(12u16)),
                name: "DPT_Value_Charge_DensitySurface",
                text: Some("flux density (C/m²)"),
                unit: Some("C/m²"),
            },
        ),
        (
            DPT::new(14u16, Some(13u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(13u16)),
                name: "DPT_Value_Charge_DensityVolume",
                text: Some("charge density (C/m³)"),
                unit: Some("C/m³"),
            },
        ),
        (
            DPT::new(14u16, Some(14u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(14u16)),
                name: "DPT_Value_Compressibility",
                text: Some("compressibility (m²/N)"),
                unit: Some("m²/N"),
            },
        ),
        (
            DPT::new(14u16, Some(15u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(15u16)),
                name: "DPT_Value_Conductance",
                text: Some("conductance (S)"),
                unit: Some("S"),
            },
        ),
        (
            DPT::new(14u16, Some(16u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(16u16)),
                name: "DPT_Value_Electrical_Conductivity",
                text: Some("conductivity  (S/m)"),
                unit: Some("S/m"),
            },
        ),
        (
            DPT::new(14u16, Some(17u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(17u16)),
                name: "DPT_Value_Density",
                text: Some("density (kg/m³)"),
                unit: Some("kg/m³"),
            },
        ),
        (
            DPT::new(14u16, Some(18u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(18u16)),
                name: "DPT_Value_Electric_Charge",
                text: Some("electric charge (C)"),
                unit: Some("C"),
            },
        ),
        (
            DPT::new(14u16, Some(19u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(19u16)),
                name: "DPT_Value_Electric_Current",
                text: Some("electric current (A)"),
                unit: Some("A"),
            },
        ),
        (
            DPT::new(14u16, Some(20u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(20u16)),
                name: "DPT_Value_Electric_CurrentDensity",
                text: Some("electric current density (A/m²)"),
                unit: Some("A/m²"),
            },
        ),
        (
            DPT::new(14u16, Some(21u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(21u16)),
                name: "DPT_Value_Electric_DipoleMoment",
                text: Some("electric dipole moment (Cm)"),
                unit: Some("Cm"),
            },
        ),
        (
            DPT::new(14u16, Some(22u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(22u16)),
                name: "DPT_Value_Electric_Displacement",
                text: Some("electric displacement (C/m²)"),
                unit: Some("C/m²"),
            },
        ),
        (
            DPT::new(14u16, Some(23u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(23u16)),
                name: "DPT_Value_Electric_FieldStrength",
                text: Some("electric field strength (V/m)"),
                unit: Some("V/m"),
            },
        ),
        (
            DPT::new(14u16, Some(24u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(24u16)),
                name: "DPT_Value_Electric_Flux",
                text: Some("electric flux (C)"),
                unit: Some("C"),
            },
        ),
        (
            DPT::new(14u16, Some(25u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(25u16)),
                name: "DPT_Value_Electric_FluxDensity",
                text: Some("electric flux density (C/m²)"),
                unit: Some("C/m²"),
            },
        ),
        (
            DPT::new(14u16, Some(26u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(26u16)),
                name: "DPT_Value_Electric_Polarization",
                text: Some("electric polarization (C/m²)"),
                unit: Some("C/m²"),
            },
        ),
        (
            DPT::new(14u16, Some(27u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(27u16)),
                name: "DPT_Value_Electric_Potential",
                text: Some("electric potential (V)"),
                unit: Some("V"),
            },
        ),
        (
            DPT::new(14u16, Some(28u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(28u16)),
                name: "DPT_Value_Electric_PotentialDifference",
                text: Some("electric potential difference (V)"),
                unit: Some("V"),
            },
        ),
        (
            DPT::new(14u16, Some(29u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(29u16)),
                name: "DPT_Value_ElectromagneticMoment",
                text: Some("electromagnetic moment (Am²)"),
                unit: Some("Am²"),
            },
        ),
        (
            DPT::new(14u16, Some(30u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(30u16)),
                name: "DPT_Value_Electromotive_Force",
                text: Some("electromotive force (V)"),
                unit: Some("V"),
            },
        ),
        (
            DPT::new(14u16, Some(31u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(31u16)),
                name: "DPT_Value_Energy",
                text: Some("energy (J)"),
                unit: Some("J"),
            },
        ),
        (
            DPT::new(14u16, Some(32u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(32u16)),
                name: "DPT_Value_Force",
                text: Some("force (N)"),
                unit: Some("N"),
            },
        ),
        (
            DPT::new(14u16, Some(33u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(33u16)),
                name: "DPT_Value_Frequency",
                text: Some("frequency (Hz)"),
                unit: Some("Hz"),
            },
        ),
        (
            DPT::new(14u16, Some(34u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(34u16)),
                name: "DPT_Value_Angular_Frequency",
                text: Some("angular frequency (rad/s)"),
                unit: Some("rad/s"),
            },
        ),
        (
            DPT::new(14u16, Some(35u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(35u16)),
                name: "DPT_Value_Heat_Capacity",
                text: Some("heat capacity (J/K)"),
                unit: Some("J/K"),
            },
        ),
        (
            DPT::new(14u16, Some(36u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(36u16)),
                name: "DPT_Value_Heat_FlowRate",
                text: Some("heat flow rate (W)"),
                unit: Some("W"),
            },
        ),
        (
            DPT::new(14u16, Some(37u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(37u16)),
                name: "DPT_Value_Heat_Quantity",
                text: Some("heat quantity"),
                unit: Some("J"),
            },
        ),
        (
            DPT::new(14u16, Some(38u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(38u16)),
                name: "DPT_Value_Impedance",
                text: Some("impedance (Ω)"),
                unit: Some("Ω"),
            },
        ),
        (
            DPT::new(14u16, Some(39u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(39u16)),
                name: "DPT_Value_Length",
                text: Some("length (m)"),
                unit: Some("m"),
            },
        ),
        (
            DPT::new(14u16, Some(40u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(40u16)),
                name: "DPT_Value_Light_Quantity",
                text: Some("light quantity (J)"),
                unit: Some("J"),
            },
        ),
        (
            DPT::new(14u16, Some(41u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(41u16)),
                name: "DPT_Value_Luminance",
                text: Some("luminance (cd/m²)"),
                unit: Some("cd/m²"),
            },
        ),
        (
            DPT::new(14u16, Some(42u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(42u16)),
                name: "DPT_Value_Luminous_Flux",
                text: Some("luminous flux (lm)"),
                unit: Some("lm"),
            },
        ),
        (
            DPT::new(14u16, Some(43u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(43u16)),
                name: "DPT_Value_Luminous_Intensity",
                text: Some("luminous intensity (cd)"),
                unit: Some("cd"),
            },
        ),
        (
            DPT::new(14u16, Some(44u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(44u16)),
                name: "DPT_Value_Magnetic_FieldStrength",
                text: Some("magnetic field strength (A/m)"),
                unit: Some("A/m"),
            },
        ),
        (
            DPT::new(14u16, Some(45u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(45u16)),
                name: "DPT_Value_Magnetic_Flux",
                text: Some("magnetic flux (Wb)"),
                unit: Some("Wb"),
            },
        ),
        (
            DPT::new(14u16, Some(46u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(46u16)),
                name: "DPT_Value_Magnetic_FluxDensity",
                text: Some("magnetic flux density (T)"),
                unit: Some("T"),
            },
        ),
        (
            DPT::new(14u16, Some(47u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(47u16)),
                name: "DPT_Value_Magnetic_Moment",
                text: Some("magnetic moment (Am²)"),
                unit: Some("Am²"),
            },
        ),
        (
            DPT::new(14u16, Some(48u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(48u16)),
                name: "DPT_Value_Magnetic_Polarization",
                text: Some("magnetic polarization (T)"),
                unit: Some("T"),
            },
        ),
        (
            DPT::new(14u16, Some(49u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(49u16)),
                name: "DPT_Value_Magnetization",
                text: Some("magnetization (A/m)"),
                unit: Some("A/m"),
            },
        ),
        (
            DPT::new(14u16, Some(50u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(50u16)),
                name: "DPT_Value_MagnetomotiveForce",
                text: Some("magnetomotive force (A)"),
                unit: Some("A"),
            },
        ),
        (
            DPT::new(14u16, Some(51u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(51u16)),
                name: "DPT_Value_Mass",
                text: Some("mass (kg)"),
                unit: Some("kg"),
            },
        ),
        (
            DPT::new(14u16, Some(52u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(52u16)),
                name: "DPT_Value_MassFlux",
                text: Some("mass flux (kg/s)"),
                unit: Some("kg/s"),
            },
        ),
        (
            DPT::new(14u16, Some(53u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(53u16)),
                name: "DPT_Value_Momentum",
                text: Some("momentum (N/s)"),
                unit: Some("N/s"),
            },
        ),
        (
            DPT::new(14u16, Some(54u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(54u16)),
                name: "DPT_Value_Phase_AngleRad",
                text: Some("phase angle (rad)"),
                unit: Some("rad"),
            },
        ),
        (
            DPT::new(14u16, Some(55u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(55u16)),
                name: "DPT_Value_Phase_AngleDeg",
                text: Some("phase angle (°)"),
                unit: Some("°"),
            },
        ),
        (
            DPT::new(14u16, Some(56u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(56u16)),
                name: "DPT_Value_Power",
                text: Some("power (W)"),
                unit: Some("W"),
            },
        ),
        (
            DPT::new(14u16, Some(57u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(57u16)),
                name: "DPT_Value_Power_Factor",
                text: Some("power factor (cos Φ)"),
                unit: Some("cos Φ"),
            },
        ),
        (
            DPT::new(14u16, Some(58u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(58u16)),
                name: "DPT_Value_Pressure",
                text: Some("pressure (Pa)"),
                unit: Some("Pa"),
            },
        ),
        (
            DPT::new(14u16, Some(59u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(59u16)),
                name: "DPT_Value_Reactance",
                text: Some("reactance (Ω)"),
                unit: Some("Ω"),
            },
        ),
        (
            DPT::new(14u16, Some(60u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(60u16)),
                name: "DPT_Value_Resistance",
                text: Some("resistance (Ω)"),
                unit: Some("Ω"),
            },
        ),
        (
            DPT::new(14u16, Some(61u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(61u16)),
                name: "DPT_Value_Resistivity",
                text: Some("resistivity (Ωm)"),
                unit: Some("Ωm"),
            },
        ),
        (
            DPT::new(14u16, Some(62u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(62u16)),
                name: "DPT_Value_SelfInductance",
                text: Some("self inductance (H)"),
                unit: Some("H"),
            },
        ),
        (
            DPT::new(14u16, Some(63u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(63u16)),
                name: "DPT_Value_SolidAngle",
                text: Some("solid angle (sr)"),
                unit: Some("sr"),
            },
        ),
        (
            DPT::new(14u16, Some(64u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(64u16)),
                name: "DPT_Value_Sound_Intensity",
                text: Some("sound intensity (W/m²)"),
                unit: Some("W/m²"),
            },
        ),
        (
            DPT::new(14u16, Some(65u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(65u16)),
                name: "DPT_Value_Speed",
                text: Some("speed (m/s)"),
                unit: Some("m/s"),
            },
        ),
        (
            DPT::new(14u16, Some(66u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(66u16)),
                name: "DPT_Value_Stress",
                text: Some("stress (Pa)"),
                unit: Some("Pa"),
            },
        ),
        (
            DPT::new(14u16, Some(67u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(67u16)),
                name: "DPT_Value_Surface_Tension",
                text: Some("surface tension (N/m)"),
                unit: Some("N/m"),
            },
        ),
        (
            DPT::new(14u16, Some(68u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(68u16)),
                name: "DPT_Value_Common_Temperature",
                text: Some("temperature (°C)"),
                unit: Some("°C"),
            },
        ),
        (
            DPT::new(14u16, Some(69u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(69u16)),
                name: "DPT_Value_Absolute_Temperature",
                text: Some("temperature absolute (K)"),
                unit: Some("K"),
            },
        ),
        (
            DPT::new(14u16, Some(70u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(70u16)),
                name: "DPT_Value_TemperatureDifference",
                text: Some("temperature difference (K)"),
                unit: Some("K"),
            },
        ),
        (
            DPT::new(14u16, Some(71u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(71u16)),
                name: "DPT_Value_Thermal_Capacity",
                text: Some("thermal capacity (J/K)"),
                unit: Some("J/K"),
            },
        ),
        (
            DPT::new(14u16, Some(72u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(72u16)),
                name: "DPT_Value_Thermal_Conductivity",
                text: Some("thermal conductivity (W/mK)"),
                unit: Some("W/mK"),
            },
        ),
        (
            DPT::new(14u16, Some(73u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(73u16)),
                name: "DPT_Value_ThermoelectricPower",
                text: Some("thermoelectric power (V/K)"),
                unit: Some("V/K"),
            },
        ),
        (
            DPT::new(14u16, Some(74u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(74u16)),
                name: "DPT_Value_Time",
                text: Some("time (s)"),
                unit: Some("s"),
            },
        ),
        (
            DPT::new(14u16, Some(75u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(75u16)),
                name: "DPT_Value_Torque",
                text: Some("torque (Nm)"),
                unit: Some("Nm"),
            },
        ),
        (
            DPT::new(14u16, Some(76u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(76u16)),
                name: "DPT_Value_Volume",
                text: Some("volume (m³)"),
                unit: Some("m³"),
            },
        ),
        (
            DPT::new(14u16, Some(77u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(77u16)),
                name: "DPT_Value_Volume_Flux",
                text: Some("volume flux (m³/s)"),
                unit: Some("m³/s"),
            },
        ),
        (
            DPT::new(14u16, Some(78u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(78u16)),
                name: "DPT_Value_Weight",
                text: Some("weight (N)"),
                unit: Some("N"),
            },
        ),
        (
            DPT::new(14u16, Some(79u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(79u16)),
                name: "DPT_Value_Work",
                text: Some("work (J)"),
                unit: Some("J"),
            },
        ),
        (
            DPT::new(14u16, Some(80u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(80u16)),
                name: "DPT_Value_ApparentPower",
                text: Some("apparent power (VA)"),
                unit: Some("VA"),
            },
        ),
        (
            DPT::new(14u16, Some(1200u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(1200u16)),
                name: "DPT_Volume_Flux_Meter",
                text: Some("volume flux for meters (m³/h)"),
                unit: Some("m³/h"),
            },
        ),
        (
            DPT::new(14u16, Some(1201u16)),
            DataPointTypeInfo {
                dpt: DPT::new(14u16, Some(1201u16)),
                name: "DPT_Volume_Flux_ls",
                text: Some("volume flux for meters (1/ls)"),
                unit: Some("1/ls"),
            },
        ),
        (
            DPT::new(15u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(15u16, None),
                name: "15.xxx",
                text: Some("entrance access"),
                unit: None,
            },
        ),
        (
            DPT::new(15u16, Some(0u16)),
            DataPointTypeInfo {
                dpt: DPT::new(15u16, Some(0u16)),
                name: "DPT_Access_Data",
                text: Some("entrance access"),
                unit: None,
            },
        ),
        (
            DPT::new(16u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(16u16, None),
                name: "16.xxx",
                text: Some("character string"),
                unit: None,
            },
        ),
        (
            DPT::new(16u16, Some(0u16)),
            DataPointTypeInfo {
                dpt: DPT::new(16u16, Some(0u16)),
                name: "DPT_String_ASCII",
                text: Some("Character String (ASCII)"),
                unit: None,
            },
        ),
        (
            DPT::new(16u16, Some(1u16)),
            DataPointTypeInfo {
                dpt: DPT::new(16u16, Some(1u16)),
                name: "DPT_String_8859_1",
                text: Some("Character String (ISO 8859-1)"),
                unit: None,
            },
        ),
        (
            DPT::new(17u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(17u16, None),
                name: "17.xxx",
                text: Some("scene number"),
                unit: None,
            },
        ),
        (
            DPT::new(17u16, Some(1u16)),
            DataPointTypeInfo {
                dpt: DPT::new(17u16, Some(1u16)),
                name: "DPT_SceneNumber",
                text: Some("scene number"),
                unit: None,
            },
        ),
        (
            DPT::new(18u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(18u16, None),
                name: "18.xxx",
                text: Some("scene control"),
                unit: None,
            },
        ),
        (
            DPT::new(18u16, Some(1u16)),
            DataPointTypeInfo {
                dpt: DPT::new(18u16, Some(1u16)),
                name: "DPT_SceneControl",
                text: Some("scene control"),
                unit: None,
            },
        ),
        (
            DPT::new(19u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(19u16, None),
                name: "19.xxx",
                text: Some("Date Time"),
                unit: None,
            },
        ),
        (
            DPT::new(19u16, Some(1u16)),
            DataPointTypeInfo {
                dpt: DPT::new(19u16, Some(1u16)),
                name: "DPT_DateTime",
                text: Some("date time"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, None),
                name: "20.xxx",
                text: Some("1-byte"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(1u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(1u16)),
                name: "DPT_SCLOMode",
                text: Some("SCLO mode"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(2u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(2u16)),
                name: "DPT_BuildingMode",
                text: Some("building mode"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(3u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(3u16)),
                name: "DPT_OccMode",
                text: Some("occupied"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(4u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(4u16)),
                name: "DPT_Priority",
                text: Some("priority"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(5u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(5u16)),
                name: "DPT_LightApplicationMode",
                text: Some("light application mode"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(6u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(6u16)),
                name: "DPT_ApplicationArea",
                text: Some("light application area"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(7u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(7u16)),
                name: "DPT_AlarmClassType",
                text: Some("alarm class"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(8u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(8u16)),
                name: "DPT_PSUMode",
                text: Some("PSU mode"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(11u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(11u16)),
                name: "DPT_ErrorClass_System",
                text: Some("system error class"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(12u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(12u16)),
                name: "DPT_ErrorClass_HVAC",
                text: Some("HVAC error class"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(13u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(13u16)),
                name: "DPT_Time_Delay",
                text: Some("time delay"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(14u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(14u16)),
                name: "DPT_Beaufort_Wind_Force_Scale",
                text: Some("wind force scale (0..12)"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(17u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(17u16)),
                name: "DPT_SensorSelect",
                text: Some("sensor mode"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(20u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(20u16)),
                name: "DPT_ActuatorConnectType",
                text: Some("actuator connect type"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(21u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(21u16)),
                name: "DPT_Cloud_Cover",
                text: Some("cloud cover"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(22u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(22u16)),
                name: "DPT_PowerReturnMode",
                text: Some("power return mode"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(100u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(100u16)),
                name: "DPT_FuelType",
                text: Some("fuel type"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(101u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(101u16)),
                name: "DPT_BurnerType",
                text: Some("burner type"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(102u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(102u16)),
                name: "DPT_HVACMode",
                text: Some("HVAC mode"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(103u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(103u16)),
                name: "DPT_DHWMode",
                text: Some("DHW mode"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(104u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(104u16)),
                name: "DPT_LoadPriority",
                text: Some("load priority"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(105u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(105u16)),
                name: "DPT_HVACContrMode",
                text: Some("HVAC control mode"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(106u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(106u16)),
                name: "DPT_HVACEmergMode",
                text: Some("HVAC emergency mode"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(107u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(107u16)),
                name: "DPT_ChangeoverMode",
                text: Some("changeover mode"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(108u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(108u16)),
                name: "DPT_ValveMode",
                text: Some("valve mode"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(109u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(109u16)),
                name: "DPT_DamperMode",
                text: Some("damper mode"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(110u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(110u16)),
                name: "DPT_HeaterMode",
                text: Some("heater mode"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(111u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(111u16)),
                name: "DPT_FanMode",
                text: Some("fan mode"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(112u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(112u16)),
                name: "DPT_MasterSlaveMode",
                text: Some("master/slave mode"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(113u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(113u16)),
                name: "DPT_StatusRoomSetp",
                text: Some("status room setpoint"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(114u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(114u16)),
                name: "DPT_Metering_DeviceType",
                text: Some("metering device type"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(115u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(115u16)),
                name: "DPT_HumDehumMode",
                text: Some("hum dehum mode"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(116u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(116u16)),
                name: "DPT_EnableHCStage",
                text: Some("enable H/C stage"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(120u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(120u16)),
                name: "DPT_ADAType",
                text: Some("ADA type"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(121u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(121u16)),
                name: "DPT_BackupMode",
                text: Some("backup mode"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(122u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(122u16)),
                name: "DPT_StartSynchronization",
                text: Some("start syncronization type"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(600u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(600u16)),
                name: "DPT_Behaviour_Lock_Unlock",
                text: Some("behavior lock/unlock"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(601u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(601u16)),
                name: "DPT_Behaviour_Bus_Power_Up_Down",
                text: Some("behavior bus power up/down"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(602u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(602u16)),
                name: "DPT_DALI_Fade_Time",
                text: Some("dali fade time"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(603u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(603u16)),
                name: "DPT_BlinkingMode",
                text: Some("blink mode"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(604u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(604u16)),
                name: "DPT_LightControlMode",
                text: Some("light control mode"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(605u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(605u16)),
                name: "DPT_SwitchPBModel",
                text: Some("PB switch mode"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(606u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(606u16)),
                name: "DPT_PBAction",
                text: Some("PB action mode"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(607u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(607u16)),
                name: "DPT_DimmPBModel",
                text: Some("PB dimm mode"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(608u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(608u16)),
                name: "DPT_SwitchOnMode",
                text: Some("switch on mode"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(609u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(609u16)),
                name: "DPT_LoadTypeSet",
                text: Some("load type"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(610u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(610u16)),
                name: "DPT_LoadTypeDetected",
                text: Some("load type detection"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(611u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(611u16)),
                name: "DPT_Converter_Test_Control",
                text: Some("converter test control"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(612u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(612u16)),
                name: "DPT_Converter_Control",
                text: Some("converter control"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(801u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(801u16)),
                name: "DPT_SABExcept-Behaviour",
                text: Some("SAB except behavior"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(802u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(802u16)),
                name: "DPT_SABBehaviour_Lock_Unlock",
                text: Some("SAB behavior on lock/unlock"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(803u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(803u16)),
                name: "DPT_SSSBMode",
                text: Some("SSSB mode"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(804u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(804u16)),
                name: "DPT_BlindsControlMode",
                text: Some("blinds control mode"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(1000u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(1000u16)),
                name: "DPT_CommMode",
                text: Some("communication mode"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(1001u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(1001u16)),
                name: "DPT_AddInfoTypes",
                text: Some("additional information type"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(1002u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(1002u16)),
                name: "DPT_RF_ModeSelect",
                text: Some("RF mode selection"),
                unit: None,
            },
        ),
        (
            DPT::new(20u16, Some(1003u16)),
            DataPointTypeInfo {
                dpt: DPT::new(20u16, Some(1003u16)),
                name: "DPT_RF_FilterSelect",
                text: Some("RF filter mode selection"),
                unit: None,
            },
        ),
        (
            DPT::new(21u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(21u16, None),
                name: "21.xxx",
                text: Some("8-bit set"),
                unit: None,
            },
        ),
        (
            DPT::new(21u16, Some(1u16)),
            DataPointTypeInfo {
                dpt: DPT::new(21u16, Some(1u16)),
                name: "DPT_StatusGen",
                text: Some("general status"),
                unit: None,
            },
        ),
        (
            DPT::new(21u16, Some(2u16)),
            DataPointTypeInfo {
                dpt: DPT::new(21u16, Some(2u16)),
                name: "DPT_Device_Control",
                text: Some("device control"),
                unit: None,
            },
        ),
        (
            DPT::new(21u16, Some(100u16)),
            DataPointTypeInfo {
                dpt: DPT::new(21u16, Some(100u16)),
                name: "DPT_ForceSign",
                text: Some("forcing signal"),
                unit: None,
            },
        ),
        (
            DPT::new(21u16, Some(101u16)),
            DataPointTypeInfo {
                dpt: DPT::new(21u16, Some(101u16)),
                name: "DPT_ForceSignCool",
                text: Some("forcing signal cool"),
                unit: None,
            },
        ),
        (
            DPT::new(21u16, Some(102u16)),
            DataPointTypeInfo {
                dpt: DPT::new(21u16, Some(102u16)),
                name: "DPT_StatusRHC",
                text: Some("room heating controller status"),
                unit: None,
            },
        ),
        (
            DPT::new(21u16, Some(103u16)),
            DataPointTypeInfo {
                dpt: DPT::new(21u16, Some(103u16)),
                name: "DPT_StatusSDHWC",
                text: Some("solar DHW controller status"),
                unit: None,
            },
        ),
        (
            DPT::new(21u16, Some(104u16)),
            DataPointTypeInfo {
                dpt: DPT::new(21u16, Some(104u16)),
                name: "DPT_FuelTypeSet",
                text: Some("fuel type set"),
                unit: None,
            },
        ),
        (
            DPT::new(21u16, Some(105u16)),
            DataPointTypeInfo {
                dpt: DPT::new(21u16, Some(105u16)),
                name: "DPT_StatusRCC",
                text: Some("room cooling controller status"),
                unit: None,
            },
        ),
        (
            DPT::new(21u16, Some(106u16)),
            DataPointTypeInfo {
                dpt: DPT::new(21u16, Some(106u16)),
                name: "DPT_StatusAHU",
                text: Some("ventilation controller status"),
                unit: None,
            },
        ),
        (
            DPT::new(21u16, Some(107u16)),
            DataPointTypeInfo {
                dpt: DPT::new(21u16, Some(107u16)),
                name: "DPT_CombinedStatus_RTSM",
                text: Some("combined status RTSM"),
                unit: None,
            },
        ),
        (
            DPT::new(21u16, Some(601u16)),
            DataPointTypeInfo {
                dpt: DPT::new(21u16, Some(601u16)),
                name: "DPT_LightActuatorErrorInfo",
                text: Some("lighting actuator error information"),
                unit: None,
            },
        ),
        (
            DPT::new(21u16, Some(1000u16)),
            DataPointTypeInfo {
                dpt: DPT::new(21u16, Some(1000u16)),
                name: "DPT_RF_ModeInfo",
                text: Some("RF communication mode info"),
                unit: None,
            },
        ),
        (
            DPT::new(21u16, Some(1001u16)),
            DataPointTypeInfo {
                dpt: DPT::new(21u16, Some(1001u16)),
                name: "DPT_RF_FilterInfo",
                text: Some("cEMI server supported RF filtering modes"),
                unit: None,
            },
        ),
        (
            DPT::new(21u16, Some(1010u16)),
            DataPointTypeInfo {
                dpt: DPT::new(21u16, Some(1010u16)),
                name: "DPT_Channel_Activation_8",
                text: Some("channel activation for 8 channels"),
                unit: None,
            },
        ),
        (
            DPT::new(22u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(22u16, None),
                name: "22.xxx",
                text: Some("16-bit set"),
                unit: None,
            },
        ),
        (
            DPT::new(22u16, Some(100u16)),
            DataPointTypeInfo {
                dpt: DPT::new(22u16, Some(100u16)),
                name: "DPT_StatusDHWC",
                text: Some("DHW controller status"),
                unit: None,
            },
        ),
        (
            DPT::new(22u16, Some(101u16)),
            DataPointTypeInfo {
                dpt: DPT::new(22u16, Some(101u16)),
                name: "DPT_StatusRHCC",
                text: Some("RHCC status"),
                unit: None,
            },
        ),
        (
            DPT::new(22u16, Some(102u16)),
            DataPointTypeInfo {
                dpt: DPT::new(22u16, Some(102u16)),
                name: "DPT_CombinedStatus_HVA",
                text: Some("combined status HVA"),
                unit: None,
            },
        ),
        (
            DPT::new(22u16, Some(103u16)),
            DataPointTypeInfo {
                dpt: DPT::new(22u16, Some(103u16)),
                name: "DPT_CombinedStatus_RTC",
                text: Some("combined status RTC"),
                unit: None,
            },
        ),
        (
            DPT::new(22u16, Some(1000u16)),
            DataPointTypeInfo {
                dpt: DPT::new(22u16, Some(1000u16)),
                name: "DPT_Media",
                text: Some("media"),
                unit: None,
            },
        ),
        (
            DPT::new(22u16, Some(1010u16)),
            DataPointTypeInfo {
                dpt: DPT::new(22u16, Some(1010u16)),
                name: "DPT_Channel_Activation_16",
                text: Some("channel activation for 16 channels"),
                unit: None,
            },
        ),
        (
            DPT::new(23u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(23u16, None),
                name: "23.xxx",
                text: Some("2-bit set"),
                unit: None,
            },
        ),
        (
            DPT::new(23u16, Some(1u16)),
            DataPointTypeInfo {
                dpt: DPT::new(23u16, Some(1u16)),
                name: "DPT_OnOffAction",
                text: Some("on/off action"),
                unit: None,
            },
        ),
        (
            DPT::new(23u16, Some(2u16)),
            DataPointTypeInfo {
                dpt: DPT::new(23u16, Some(2u16)),
                name: "DPT_Alarm_Reaction",
                text: Some("alarm reaction"),
                unit: None,
            },
        ),
        (
            DPT::new(23u16, Some(3u16)),
            DataPointTypeInfo {
                dpt: DPT::new(23u16, Some(3u16)),
                name: "DPT_UpDown_Action",
                text: Some("up/down action"),
                unit: None,
            },
        ),
        (
            DPT::new(23u16, Some(102u16)),
            DataPointTypeInfo {
                dpt: DPT::new(23u16, Some(102u16)),
                name: "DPT_HVAC_PB_Action",
                text: Some("HVAC push button action"),
                unit: None,
            },
        ),
        (
            DPT::new(24u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(24u16, None),
                name: "24.xxx",
                text: None,
                unit: None,
            },
        ),
        (
            DPT::new(24u16, Some(1u16)),
            DataPointTypeInfo {
                dpt: DPT::new(24u16, Some(1u16)),
                name: "DPT_VarString_8859_1",
                text: Some("Variable-Length ISO-8859-1 String"),
                unit: None,
            },
        ),
        (
            DPT::new(25u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(25u16, None),
                name: "25.xxx",
                text: Some("2-nibble set"),
                unit: None,
            },
        ),
        (
            DPT::new(25u16, Some(1000u16)),
            DataPointTypeInfo {
                dpt: DPT::new(25u16, Some(1000u16)),
                name: "DPT_DoubleNibble",
                text: Some("busy/nak repetitions"),
                unit: None,
            },
        ),
        (
            DPT::new(26u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(26u16, None),
                name: "26.xxx",
                text: Some("8-bit set"),
                unit: None,
            },
        ),
        (
            DPT::new(26u16, Some(1u16)),
            DataPointTypeInfo {
                dpt: DPT::new(26u16, Some(1u16)),
                name: "DPT_SceneInfo",
                text: Some("scene information"),
                unit: None,
            },
        ),
        (
            DPT::new(27u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(27u16, None),
                name: "27.xxx",
                text: Some("32-bit set"),
                unit: None,
            },
        ),
        (
            DPT::new(27u16, Some(1u16)),
            DataPointTypeInfo {
                dpt: DPT::new(27u16, Some(1u16)),
                name: "DPT_CombinedInfoOnOff",
                text: Some("bit-combined info on/off"),
                unit: None,
            },
        ),
        (
            DPT::new(28u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(28u16, None),
                name: "28.xxx",
                text: None,
                unit: None,
            },
        ),
        (
            DPT::new(28u16, Some(1u16)),
            DataPointTypeInfo {
                dpt: DPT::new(28u16, Some(1u16)),
                name: "DPT_UTF-8",
                text: Some("Unicode UTF-8 String"),
                unit: None,
            },
        ),
        (
            DPT::new(29u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(29u16, None),
                name: "29.xxx",
                text: Some("electrical energy"),
                unit: Some("Wh"),
            },
        ),
        (
            DPT::new(29u16, Some(10u16)),
            DataPointTypeInfo {
                dpt: DPT::new(29u16, Some(10u16)),
                name: "DPT_ActiveEnergy_V64",
                text: Some("active energy (Wh)"),
                unit: Some("Wh"),
            },
        ),
        (
            DPT::new(29u16, Some(11u16)),
            DataPointTypeInfo {
                dpt: DPT::new(29u16, Some(11u16)),
                name: "DPT_ApparantEnergy_V64",
                text: Some("apparant energy (VAh)"),
                unit: Some("VAh"),
            },
        ),
        (
            DPT::new(29u16, Some(12u16)),
            DataPointTypeInfo {
                dpt: DPT::new(29u16, Some(12u16)),
                name: "DPT_ReactiveEnergy_V64",
                text: Some("reactive energy (VARh)"),
                unit: Some("VARh"),
            },
        ),
        (
            DPT::new(30u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(30u16, None),
                name: "30.xxx",
                text: Some("24 times channel activation"),
                unit: None,
            },
        ),
        (
            DPT::new(30u16, Some(1010u16)),
            DataPointTypeInfo {
                dpt: DPT::new(30u16, Some(1010u16)),
                name: "DPT_Channel_Activation_24",
                text: Some("activation state 0..23"),
                unit: None,
            },
        ),
        (
            DPT::new(206u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(206u16, None),
                name: "206.xxx",
                text: Some("16-bit unsigned value & 8-bit enum"),
                unit: None,
            },
        ),
        (
            DPT::new(206u16, Some(100u16)),
            DataPointTypeInfo {
                dpt: DPT::new(206u16, Some(100u16)),
                name: "DPT_HVACModeNext",
                text: Some("time delay & HVAC mode"),
                unit: None,
            },
        ),
        (
            DPT::new(206u16, Some(102u16)),
            DataPointTypeInfo {
                dpt: DPT::new(206u16, Some(102u16)),
                name: "DPT_DHWModeNext",
                text: Some("time delay & DHW mode"),
                unit: None,
            },
        ),
        (
            DPT::new(206u16, Some(104u16)),
            DataPointTypeInfo {
                dpt: DPT::new(206u16, Some(104u16)),
                name: "DPT_OccModeNext",
                text: Some("time delay & occupancy mode"),
                unit: None,
            },
        ),
        (
            DPT::new(206u16, Some(105u16)),
            DataPointTypeInfo {
                dpt: DPT::new(206u16, Some(105u16)),
                name: "DPT_BuildingModeNext",
                text: Some("time delay & building mode"),
                unit: None,
            },
        ),
        (
            DPT::new(207u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(207u16, None),
                name: "207.xxx",
                text: Some("8-bit unsigned value & 8-bit enum"),
                unit: None,
            },
        ),
        (
            DPT::new(207u16, Some(600u16)),
            DataPointTypeInfo {
                dpt: DPT::new(207u16, Some(600u16)),
                name: "DPT_StatusLightingActuator",
                text: Some("Status Lighting Actuator"),
                unit: None,
            },
        ),
        (
            DPT::new(217u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(217u16, None),
                name: "217.xxx",
                text: Some("datapoint type version"),
                unit: None,
            },
        ),
        (
            DPT::new(217u16, Some(1u16)),
            DataPointTypeInfo {
                dpt: DPT::new(217u16, Some(1u16)),
                name: "DPT_Version",
                text: Some("DPT version"),
                unit: None,
            },
        ),
        (
            DPT::new(219u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(219u16, None),
                name: "219.xxx",
                text: Some("alarm info"),
                unit: None,
            },
        ),
        (
            DPT::new(219u16, Some(1u16)),
            DataPointTypeInfo {
                dpt: DPT::new(219u16, Some(1u16)),
                name: "DPT_AlarmInfo",
                text: Some("alarm info"),
                unit: None,
            },
        ),
        (
            DPT::new(222u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(222u16, None),
                name: "222.xxx",
                text: Some("3x 2-byte float value"),
                unit: None,
            },
        ),
        (
            DPT::new(222u16, Some(100u16)),
            DataPointTypeInfo {
                dpt: DPT::new(222u16, Some(100u16)),
                name: "DPT_TempRoomSetpSetF16[3]",
                text: Some("room temperature setpoint"),
                unit: None,
            },
        ),
        (
            DPT::new(222u16, Some(101u16)),
            DataPointTypeInfo {
                dpt: DPT::new(222u16, Some(101u16)),
                name: "DPT_TempRoomSetpSetShiftF16[3]",
                text: Some("room temperature setpoint shift"),
                unit: None,
            },
        ),
        (
            DPT::new(225u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(225u16, None),
                name: "225.xxx",
                text: Some("scaling speed"),
                unit: None,
            },
        ),
        (
            DPT::new(225u16, Some(1u16)),
            DataPointTypeInfo {
                dpt: DPT::new(225u16, Some(1u16)),
                name: "Scaling Speed",
                text: Some("scaling speed"),
                unit: None,
            },
        ),
        (
            DPT::new(225u16, Some(2u16)),
            DataPointTypeInfo {
                dpt: DPT::new(225u16, Some(2u16)),
                name: "Scaling step time",
                text: Some("scaling step time"),
                unit: None,
            },
        ),
        (
            DPT::new(229u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(229u16, None),
                name: "229.xxx",
                text: Some("4-1-1 byte combined information"),
                unit: None,
            },
        ),
        (
            DPT::new(229u16, Some(1u16)),
            DataPointTypeInfo {
                dpt: DPT::new(229u16, Some(1u16)),
                name: "DPT_MeteringValue",
                text: Some("metering value (value,encoding,cmd)"),
                unit: None,
            },
        ),
        (
            DPT::new(230u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(230u16, None),
                name: "230.xxx",
                text: Some("MBus address"),
                unit: None,
            },
        ),
        (
            DPT::new(230u16, Some(1000u16)),
            DataPointTypeInfo {
                dpt: DPT::new(230u16, Some(1000u16)),
                name: "DPT_MBus_Address",
                text: Some("MBus address"),
                unit: None,
            },
        ),
        (
            DPT::new(232u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(232u16, None),
                name: "232.xxx",
                text: Some("3-byte colour RGB"),
                unit: None,
            },
        ),
        (
            DPT::new(232u16, Some(600u16)),
            DataPointTypeInfo {
                dpt: DPT::new(232u16, Some(600u16)),
                name: "DPT_Colour_RGB",
                text: Some("RGB value 3x(0..255)"),
                unit: None,
            },
        ),
        (
            DPT::new(234u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(234u16, None),
                name: "234.xxx",
                text: Some("language code ISO 639-1"),
                unit: None,
            },
        ),
        (
            DPT::new(234u16, Some(1u16)),
            DataPointTypeInfo {
                dpt: DPT::new(234u16, Some(1u16)),
                name: "DPT_LanguageCodeAlpha2_ASCII",
                text: Some("language code (ASCII)"),
                unit: None,
            },
        ),
        (
            DPT::new(235u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(235u16, None),
                name: "235.xxx",
                text: Some("Signed value with classification and validity"),
                unit: None,
            },
        ),
        (
            DPT::new(235u16, Some(1u16)),
            DataPointTypeInfo {
                dpt: DPT::new(235u16, Some(1u16)),
                name: "DPT_Tariff_ActiveEnergy",
                text: Some("electrical energy with tariff"),
                unit: None,
            },
        ),
        (
            DPT::new(236u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(236u16, None),
                name: "236.xxx",
                text: Some("Prioritised Mode Control"),
                unit: None,
            },
        ),
        (
            DPT::new(236u16, Some(1u16)),
            DataPointTypeInfo {
                dpt: DPT::new(236u16, Some(1u16)),
                name: "DPT_Prioritised_Mode_Control",
                text: Some("priority control"),
                unit: None,
            },
        ),
        (
            DPT::new(237u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(237u16, None),
                name: "237.xxx",
                text: Some("configuration/ diagnostics"),
                unit: None,
            },
        ),
        (
            DPT::new(237u16, Some(600u16)),
            DataPointTypeInfo {
                dpt: DPT::new(237u16, Some(600u16)),
                name: "DPT_DALI_Control_Gear_Diagnostic",
                text: Some("diagnostic value"),
                unit: None,
            },
        ),
        (
            DPT::new(238u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(238u16, None),
                name: "238.xxx",
                text: Some("configuration/ diagnostics"),
                unit: None,
            },
        ),
        (
            DPT::new(238u16, Some(600u16)),
            DataPointTypeInfo {
                dpt: DPT::new(238u16, Some(600u16)),
                name: "DPT_DALI_Diagnostics",
                text: Some("diagnostic value"),
                unit: None,
            },
        ),
        (
            DPT::new(240u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(240u16, None),
                name: "240.xxx",
                text: Some("positions"),
                unit: None,
            },
        ),
        (
            DPT::new(240u16, Some(800u16)),
            DataPointTypeInfo {
                dpt: DPT::new(240u16, Some(800u16)),
                name: "DPT_CombinedPosition",
                text: Some("combined position"),
                unit: None,
            },
        ),
        (
            DPT::new(241u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(241u16, None),
                name: "241.xxx",
                text: Some("status"),
                unit: None,
            },
        ),
        (
            DPT::new(241u16, Some(800u16)),
            DataPointTypeInfo {
                dpt: DPT::new(241u16, Some(800u16)),
                name: "DPT_StatusSAB",
                text: Some("status sunblind & shutter actuator"),
                unit: None,
            },
        ),
        (
            DPT::new(242u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(242u16, None),
                name: "242.xxx",
                text: Some("status"),
                unit: None,
            },
        ),
        (
            DPT::new(242u16, Some(600u16)),
            DataPointTypeInfo {
                dpt: DPT::new(242u16, Some(600u16)),
                name: "DPT_Colour_xyY",
                text: Some("colour xyY"),
                unit: None,
            },
        ),
        (
            DPT::new(244u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(244u16, None),
                name: "244.xxx",
                text: Some("Converter Status"),
                unit: None,
            },
        ),
        (
            DPT::new(244u16, Some(600u16)),
            DataPointTypeInfo {
                dpt: DPT::new(244u16, Some(600u16)),
                name: "DPT_Converter_Status",
                text: Some("DALI converter status"),
                unit: None,
            },
        ),
        (
            DPT::new(245u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(245u16, None),
                name: "245.xxx",
                text: Some("Converter test result"),
                unit: None,
            },
        ),
        (
            DPT::new(245u16, Some(600u16)),
            DataPointTypeInfo {
                dpt: DPT::new(245u16, Some(600u16)),
                name: "DPT_Converter_Test_Result",
                text: Some("DALI converter test result"),
                unit: None,
            },
        ),
        (
            DPT::new(246u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(246u16, None),
                name: "246.xxx",
                text: Some("Battery Information"),
                unit: None,
            },
        ),
        (
            DPT::new(246u16, Some(600u16)),
            DataPointTypeInfo {
                dpt: DPT::new(246u16, Some(600u16)),
                name: "DPT_Battery_Info",
                text: Some("Battery Information"),
                unit: None,
            },
        ),
        (
            DPT::new(249u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(249u16, None),
                name: "249.xxx",
                text: Some("brightness colour temperature transition"),
                unit: None,
            },
        ),
        (
            DPT::new(249u16, Some(600u16)),
            DataPointTypeInfo {
                dpt: DPT::new(249u16, Some(600u16)),
                name: "DPT_Brightness_Colour_Temperature_Transition",
                text: Some("brightness colour temperature transition"),
                unit: None,
            },
        ),
        (
            DPT::new(250u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(250u16, None),
                name: "250.xxx",
                text: Some("status"),
                unit: None,
            },
        ),
        (
            DPT::new(250u16, Some(600u16)),
            DataPointTypeInfo {
                dpt: DPT::new(250u16, Some(600u16)),
                name: "DPT_Brightness_Colour_Temperature_Control",
                text: Some("brightness colour temperature control"),
                unit: None,
            },
        ),
        (
            DPT::new(251u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(251u16, None),
                name: "251.xxx",
                text: Some("Colour RGBW"),
                unit: None,
            },
        ),
        (
            DPT::new(251u16, Some(600u16)),
            DataPointTypeInfo {
                dpt: DPT::new(251u16, Some(600u16)),
                name: "DPT_Colour_RGBW",
                text: Some("RGBW value 4x(0..100%)"),
                unit: None,
            },
        ),
        (
            DPT::new(252u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(252u16, None),
                name: "252.xxx",
                text: Some("Relative Control RGBW"),
                unit: None,
            },
        ),
        (
            DPT::new(252u16, Some(600u16)),
            DataPointTypeInfo {
                dpt: DPT::new(252u16, Some(600u16)),
                name: "DPT_Relative_Control_RGBW",
                text: Some("RGBW relative control"),
                unit: None,
            },
        ),
        (
            DPT::new(254u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(254u16, None),
                name: "254.xxx",
                text: Some("Relative Control RGB"),
                unit: None,
            },
        ),
        (
            DPT::new(254u16, Some(600u16)),
            DataPointTypeInfo {
                dpt: DPT::new(254u16, Some(600u16)),
                name: "DPT_Relative_Control_RGB",
                text: Some("RGB relative control"),
                unit: None,
            },
        ),
        (
            DPT::new(255u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(255u16, None),
                name: "255.xxx",
                text: Some("F32F32"),
                unit: None,
            },
        ),
        (
            DPT::new(255u16, Some(1u16)),
            DataPointTypeInfo {
                dpt: DPT::new(255u16, Some(1u16)),
                name: "DPT_GeographicalLocation",
                text: Some(
                    "geographical location (longitude and latitude) expressed in degrees",
                ),
                unit: None,
            },
        ),
        (
            DPT::new(275u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(275u16, None),
                name: "275.xxx",
                text: Some("F16F16F16F16"),
                unit: None,
            },
        ),
        (
            DPT::new(275u16, Some(100u16)),
            DataPointTypeInfo {
                dpt: DPT::new(275u16, Some(100u16)),
                name: "DPT_TempRoomSetpSetF16[4]",
                text: Some("Temperature setpoint setting for 4 HVAC Modes"),
                unit: None,
            },
        ),
        (
            DPT::new(275u16, Some(101u16)),
            DataPointTypeInfo {
                dpt: DPT::new(275u16, Some(101u16)),
                name: "DPT_TempRoomSetpSetShiftF16[4]",
                text: Some("Temperature setpoint shift setting for 4 HVAC Modes"),
                unit: None,
            },
        ),
        (
            DPT::new(285u16, None),
            DataPointTypeInfo {
                dpt: DPT::new(285u16, None),
                name: "285.xxx",
                text: Some("ipv6"),
                unit: None,
            },
        ),
        (
            DPT::new(285u16, Some(1u16)),
            DataPointTypeInfo {
                dpt: DPT::new(285u16, Some(1u16)),
                name: "DPT_ipv6",
                text: Some("IPv6 address encoded in the network byte order"),
                unit: None,
            },
        ),
    ];
    DATAPOINTS.binary_search_by_key(&dpt, |x| x.0).ok().map(|ix| &DATAPOINTS[ix].1)
}
