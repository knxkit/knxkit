use knxkit::core::DataPoint;
use knxkit_dpt::{generic, specific::DPT_3_7, OpaqueDataPoint, Specific};
use serde_json;

fn main() {
    let value37 = DPT_3_7 {
        Increase: true,
        StepCode: 1,
    };

    println!("DPT_3_7: {}", value37);
    // -> DPT_3_7: true/1

    let dp: DataPoint = value37.to_data_point();
    println!("DataPoint: {}", dp);
    // -> DataPoint: $09

    let json = serde_json::to_string(&value37).unwrap();
    println!("JSON: {}", json);
    // -> JSON: {"Increase":true,"StepCode":1}

    let value = DataPoint::Short(0x09);
    println!("DataPoint: {}", value);
    // -> DataPoint: $09

    let opaque: OpaqueDataPoint = generic::try_decode(DPT_3_7::DPT, &value).unwrap();
    println!("Opaque: {}", opaque);
    // -> Opaque: true/1

    let json = opaque.to_json_value();
    println!("JSON: {}", json);
    // -> JSON: {"Increase":true,"StepCode":1}

    let opaque = generic::try_decode_json(DPT_3_7::DPT, json).unwrap();
    println!("Opaque: {}", opaque);
    // -> Opaque: true/1
}
