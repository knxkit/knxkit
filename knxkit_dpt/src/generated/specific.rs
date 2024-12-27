// This file is generated, don't manually edit it!
use std::{
    io::{SeekFrom, Cursor},
    fmt::{Display, Debug},
};
use bitstream_io::{BitRead, BitReader, BE, BitWriter, BitWrite, BigEndian};
use encoding::{
    Encoding, all::{ASCII, ISO_8859_1, UTF_8},
    types::{EncoderTrap, DecoderTrap},
};
use serde::{Serialize, Deserialize};
use knxkit::{project::DPT, core::DataPoint};
use crate::{decode_knxf16, encode_knxf16, Error, Reserved};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_1_1(pub bool);
impl crate::Specific for DPT_1_1 {
    const DPT: DPT = DPT::new(1u16, Some(1u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = [0u8];
        let mut writer = BitWriter::endian(bytes.as_mut_slice(), BigEndian);
        writer.write_bit(self.0).unwrap();
        DataPoint::Short(writer.into_unwritten().1)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Short(byte) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new([*byte]));
            reader.seek_bits(SeekFrom::End(1u16 as i64)).unwrap();
            Ok(DPT_1_1(reader.read_bit()?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_1_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_1_2(pub bool);
impl crate::Specific for DPT_1_2 {
    const DPT: DPT = DPT::new(1u16, Some(2u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = [0u8];
        let mut writer = BitWriter::endian(bytes.as_mut_slice(), BigEndian);
        writer.write_bit(self.0).unwrap();
        DataPoint::Short(writer.into_unwritten().1)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Short(byte) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new([*byte]));
            reader.seek_bits(SeekFrom::End(1u16 as i64)).unwrap();
            Ok(DPT_1_2(reader.read_bit()?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_1_2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_1_3(pub bool);
impl crate::Specific for DPT_1_3 {
    const DPT: DPT = DPT::new(1u16, Some(3u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = [0u8];
        let mut writer = BitWriter::endian(bytes.as_mut_slice(), BigEndian);
        writer.write_bit(self.0).unwrap();
        DataPoint::Short(writer.into_unwritten().1)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Short(byte) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new([*byte]));
            reader.seek_bits(SeekFrom::End(1u16 as i64)).unwrap();
            Ok(DPT_1_3(reader.read_bit()?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_1_3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_1_4(pub bool);
impl crate::Specific for DPT_1_4 {
    const DPT: DPT = DPT::new(1u16, Some(4u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = [0u8];
        let mut writer = BitWriter::endian(bytes.as_mut_slice(), BigEndian);
        writer.write_bit(self.0).unwrap();
        DataPoint::Short(writer.into_unwritten().1)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Short(byte) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new([*byte]));
            reader.seek_bits(SeekFrom::End(1u16 as i64)).unwrap();
            Ok(DPT_1_4(reader.read_bit()?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_1_4 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_1_5(pub bool);
impl crate::Specific for DPT_1_5 {
    const DPT: DPT = DPT::new(1u16, Some(5u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = [0u8];
        let mut writer = BitWriter::endian(bytes.as_mut_slice(), BigEndian);
        writer.write_bit(self.0).unwrap();
        DataPoint::Short(writer.into_unwritten().1)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Short(byte) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new([*byte]));
            reader.seek_bits(SeekFrom::End(1u16 as i64)).unwrap();
            Ok(DPT_1_5(reader.read_bit()?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_1_5 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_1_6(pub bool);
impl crate::Specific for DPT_1_6 {
    const DPT: DPT = DPT::new(1u16, Some(6u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = [0u8];
        let mut writer = BitWriter::endian(bytes.as_mut_slice(), BigEndian);
        writer.write_bit(self.0).unwrap();
        DataPoint::Short(writer.into_unwritten().1)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Short(byte) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new([*byte]));
            reader.seek_bits(SeekFrom::End(1u16 as i64)).unwrap();
            Ok(DPT_1_6(reader.read_bit()?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_1_6 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_1_7(pub bool);
impl crate::Specific for DPT_1_7 {
    const DPT: DPT = DPT::new(1u16, Some(7u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = [0u8];
        let mut writer = BitWriter::endian(bytes.as_mut_slice(), BigEndian);
        writer.write_bit(self.0).unwrap();
        DataPoint::Short(writer.into_unwritten().1)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Short(byte) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new([*byte]));
            reader.seek_bits(SeekFrom::End(1u16 as i64)).unwrap();
            Ok(DPT_1_7(reader.read_bit()?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_1_7 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_1_8(pub bool);
impl crate::Specific for DPT_1_8 {
    const DPT: DPT = DPT::new(1u16, Some(8u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = [0u8];
        let mut writer = BitWriter::endian(bytes.as_mut_slice(), BigEndian);
        writer.write_bit(self.0).unwrap();
        DataPoint::Short(writer.into_unwritten().1)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Short(byte) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new([*byte]));
            reader.seek_bits(SeekFrom::End(1u16 as i64)).unwrap();
            Ok(DPT_1_8(reader.read_bit()?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_1_8 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_1_9(pub bool);
impl crate::Specific for DPT_1_9 {
    const DPT: DPT = DPT::new(1u16, Some(9u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = [0u8];
        let mut writer = BitWriter::endian(bytes.as_mut_slice(), BigEndian);
        writer.write_bit(self.0).unwrap();
        DataPoint::Short(writer.into_unwritten().1)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Short(byte) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new([*byte]));
            reader.seek_bits(SeekFrom::End(1u16 as i64)).unwrap();
            Ok(DPT_1_9(reader.read_bit()?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_1_9 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_1_10(pub bool);
impl crate::Specific for DPT_1_10 {
    const DPT: DPT = DPT::new(1u16, Some(10u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = [0u8];
        let mut writer = BitWriter::endian(bytes.as_mut_slice(), BigEndian);
        writer.write_bit(self.0).unwrap();
        DataPoint::Short(writer.into_unwritten().1)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Short(byte) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new([*byte]));
            reader.seek_bits(SeekFrom::End(1u16 as i64)).unwrap();
            Ok(DPT_1_10(reader.read_bit()?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_1_10 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_1_11(pub bool);
impl crate::Specific for DPT_1_11 {
    const DPT: DPT = DPT::new(1u16, Some(11u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = [0u8];
        let mut writer = BitWriter::endian(bytes.as_mut_slice(), BigEndian);
        writer.write_bit(self.0).unwrap();
        DataPoint::Short(writer.into_unwritten().1)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Short(byte) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new([*byte]));
            reader.seek_bits(SeekFrom::End(1u16 as i64)).unwrap();
            Ok(DPT_1_11(reader.read_bit()?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_1_11 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_1_12(pub bool);
impl crate::Specific for DPT_1_12 {
    const DPT: DPT = DPT::new(1u16, Some(12u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = [0u8];
        let mut writer = BitWriter::endian(bytes.as_mut_slice(), BigEndian);
        writer.write_bit(self.0).unwrap();
        DataPoint::Short(writer.into_unwritten().1)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Short(byte) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new([*byte]));
            reader.seek_bits(SeekFrom::End(1u16 as i64)).unwrap();
            Ok(DPT_1_12(reader.read_bit()?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_1_12 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_1_13(pub bool);
impl crate::Specific for DPT_1_13 {
    const DPT: DPT = DPT::new(1u16, Some(13u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = [0u8];
        let mut writer = BitWriter::endian(bytes.as_mut_slice(), BigEndian);
        writer.write_bit(self.0).unwrap();
        DataPoint::Short(writer.into_unwritten().1)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Short(byte) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new([*byte]));
            reader.seek_bits(SeekFrom::End(1u16 as i64)).unwrap();
            Ok(DPT_1_13(reader.read_bit()?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_1_13 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_1_14(pub bool);
impl crate::Specific for DPT_1_14 {
    const DPT: DPT = DPT::new(1u16, Some(14u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = [0u8];
        let mut writer = BitWriter::endian(bytes.as_mut_slice(), BigEndian);
        writer.write_bit(self.0).unwrap();
        DataPoint::Short(writer.into_unwritten().1)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Short(byte) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new([*byte]));
            reader.seek_bits(SeekFrom::End(1u16 as i64)).unwrap();
            Ok(DPT_1_14(reader.read_bit()?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_1_14 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_1_15(pub bool);
impl crate::Specific for DPT_1_15 {
    const DPT: DPT = DPT::new(1u16, Some(15u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = [0u8];
        let mut writer = BitWriter::endian(bytes.as_mut_slice(), BigEndian);
        writer.write_bit(self.0).unwrap();
        DataPoint::Short(writer.into_unwritten().1)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Short(byte) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new([*byte]));
            reader.seek_bits(SeekFrom::End(1u16 as i64)).unwrap();
            Ok(DPT_1_15(reader.read_bit()?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_1_15 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_1_16(pub bool);
impl crate::Specific for DPT_1_16 {
    const DPT: DPT = DPT::new(1u16, Some(16u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = [0u8];
        let mut writer = BitWriter::endian(bytes.as_mut_slice(), BigEndian);
        writer.write_bit(self.0).unwrap();
        DataPoint::Short(writer.into_unwritten().1)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Short(byte) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new([*byte]));
            reader.seek_bits(SeekFrom::End(1u16 as i64)).unwrap();
            Ok(DPT_1_16(reader.read_bit()?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_1_16 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_1_17(pub bool);
impl crate::Specific for DPT_1_17 {
    const DPT: DPT = DPT::new(1u16, Some(17u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = [0u8];
        let mut writer = BitWriter::endian(bytes.as_mut_slice(), BigEndian);
        writer.write_bit(self.0).unwrap();
        DataPoint::Short(writer.into_unwritten().1)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Short(byte) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new([*byte]));
            reader.seek_bits(SeekFrom::End(1u16 as i64)).unwrap();
            Ok(DPT_1_17(reader.read_bit()?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_1_17 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_1_18(pub bool);
impl crate::Specific for DPT_1_18 {
    const DPT: DPT = DPT::new(1u16, Some(18u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = [0u8];
        let mut writer = BitWriter::endian(bytes.as_mut_slice(), BigEndian);
        writer.write_bit(self.0).unwrap();
        DataPoint::Short(writer.into_unwritten().1)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Short(byte) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new([*byte]));
            reader.seek_bits(SeekFrom::End(1u16 as i64)).unwrap();
            Ok(DPT_1_18(reader.read_bit()?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_1_18 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_1_19(pub bool);
impl crate::Specific for DPT_1_19 {
    const DPT: DPT = DPT::new(1u16, Some(19u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = [0u8];
        let mut writer = BitWriter::endian(bytes.as_mut_slice(), BigEndian);
        writer.write_bit(self.0).unwrap();
        DataPoint::Short(writer.into_unwritten().1)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Short(byte) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new([*byte]));
            reader.seek_bits(SeekFrom::End(1u16 as i64)).unwrap();
            Ok(DPT_1_19(reader.read_bit()?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_1_19 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_1_21(pub bool);
impl crate::Specific for DPT_1_21 {
    const DPT: DPT = DPT::new(1u16, Some(21u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = [0u8];
        let mut writer = BitWriter::endian(bytes.as_mut_slice(), BigEndian);
        writer.write_bit(self.0).unwrap();
        DataPoint::Short(writer.into_unwritten().1)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Short(byte) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new([*byte]));
            reader.seek_bits(SeekFrom::End(1u16 as i64)).unwrap();
            Ok(DPT_1_21(reader.read_bit()?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_1_21 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_1_22(pub bool);
impl crate::Specific for DPT_1_22 {
    const DPT: DPT = DPT::new(1u16, Some(22u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = [0u8];
        let mut writer = BitWriter::endian(bytes.as_mut_slice(), BigEndian);
        writer.write_bit(self.0).unwrap();
        DataPoint::Short(writer.into_unwritten().1)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Short(byte) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new([*byte]));
            reader.seek_bits(SeekFrom::End(1u16 as i64)).unwrap();
            Ok(DPT_1_22(reader.read_bit()?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_1_22 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_1_23(pub bool);
impl crate::Specific for DPT_1_23 {
    const DPT: DPT = DPT::new(1u16, Some(23u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = [0u8];
        let mut writer = BitWriter::endian(bytes.as_mut_slice(), BigEndian);
        writer.write_bit(self.0).unwrap();
        DataPoint::Short(writer.into_unwritten().1)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Short(byte) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new([*byte]));
            reader.seek_bits(SeekFrom::End(1u16 as i64)).unwrap();
            Ok(DPT_1_23(reader.read_bit()?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_1_23 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_1_24(pub bool);
impl crate::Specific for DPT_1_24 {
    const DPT: DPT = DPT::new(1u16, Some(24u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = [0u8];
        let mut writer = BitWriter::endian(bytes.as_mut_slice(), BigEndian);
        writer.write_bit(self.0).unwrap();
        DataPoint::Short(writer.into_unwritten().1)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Short(byte) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new([*byte]));
            reader.seek_bits(SeekFrom::End(1u16 as i64)).unwrap();
            Ok(DPT_1_24(reader.read_bit()?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_1_24 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_1_100(pub bool);
impl crate::Specific for DPT_1_100 {
    const DPT: DPT = DPT::new(1u16, Some(100u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = [0u8];
        let mut writer = BitWriter::endian(bytes.as_mut_slice(), BigEndian);
        writer.write_bit(self.0).unwrap();
        DataPoint::Short(writer.into_unwritten().1)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Short(byte) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new([*byte]));
            reader.seek_bits(SeekFrom::End(1u16 as i64)).unwrap();
            Ok(DPT_1_100(reader.read_bit()?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_1_100 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_1_x(pub bool);
impl crate::Specific for DPT_1_x {
    const DPT: DPT = DPT::new(1u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = [0u8];
        let mut writer = BitWriter::endian(bytes.as_mut_slice(), BigEndian);
        writer.write_bit(self.0).unwrap();
        DataPoint::Short(writer.into_unwritten().1)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Short(byte) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new([*byte]));
            reader.seek_bits(SeekFrom::End(1u16 as i64)).unwrap();
            Ok(DPT_1_x(reader.read_bit()?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_1_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_2_1 {
    pub control: bool,
    pub On: bool,
}
impl crate::Specific for DPT_2_1 {
    const DPT: DPT = DPT::new(2u16, Some(1u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = [0u8];
        let mut writer = BitWriter::endian(bytes.as_mut_slice(), BigEndian);
        writer.write_bit(self.control).unwrap();
        writer.write_bit(self.On).unwrap();
        DataPoint::Short(writer.into_unwritten().1)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Short(byte) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new([*byte]));
            reader.seek_bits(SeekFrom::End(2u16 as i64)).unwrap();
            Ok(DPT_2_1 {
                control: reader.read_bit()?,
                On: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_2_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.control, self.On)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_2_2 {
    pub control: bool,
    pub bit: bool,
}
impl crate::Specific for DPT_2_2 {
    const DPT: DPT = DPT::new(2u16, Some(2u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = [0u8];
        let mut writer = BitWriter::endian(bytes.as_mut_slice(), BigEndian);
        writer.write_bit(self.control).unwrap();
        writer.write_bit(self.bit).unwrap();
        DataPoint::Short(writer.into_unwritten().1)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Short(byte) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new([*byte]));
            reader.seek_bits(SeekFrom::End(2u16 as i64)).unwrap();
            Ok(DPT_2_2 {
                control: reader.read_bit()?,
                bit: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_2_2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.control, self.bit)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_2_3 {
    pub control: bool,
    pub Enable: bool,
}
impl crate::Specific for DPT_2_3 {
    const DPT: DPT = DPT::new(2u16, Some(3u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = [0u8];
        let mut writer = BitWriter::endian(bytes.as_mut_slice(), BigEndian);
        writer.write_bit(self.control).unwrap();
        writer.write_bit(self.Enable).unwrap();
        DataPoint::Short(writer.into_unwritten().1)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Short(byte) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new([*byte]));
            reader.seek_bits(SeekFrom::End(2u16 as i64)).unwrap();
            Ok(DPT_2_3 {
                control: reader.read_bit()?,
                Enable: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_2_3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.control, self.Enable)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_2_4 {
    pub control: bool,
    pub Ramp: bool,
}
impl crate::Specific for DPT_2_4 {
    const DPT: DPT = DPT::new(2u16, Some(4u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = [0u8];
        let mut writer = BitWriter::endian(bytes.as_mut_slice(), BigEndian);
        writer.write_bit(self.control).unwrap();
        writer.write_bit(self.Ramp).unwrap();
        DataPoint::Short(writer.into_unwritten().1)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Short(byte) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new([*byte]));
            reader.seek_bits(SeekFrom::End(2u16 as i64)).unwrap();
            Ok(DPT_2_4 {
                control: reader.read_bit()?,
                Ramp: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_2_4 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.control, self.Ramp)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_2_5 {
    pub control: bool,
    pub Alarm: bool,
}
impl crate::Specific for DPT_2_5 {
    const DPT: DPT = DPT::new(2u16, Some(5u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = [0u8];
        let mut writer = BitWriter::endian(bytes.as_mut_slice(), BigEndian);
        writer.write_bit(self.control).unwrap();
        writer.write_bit(self.Alarm).unwrap();
        DataPoint::Short(writer.into_unwritten().1)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Short(byte) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new([*byte]));
            reader.seek_bits(SeekFrom::End(2u16 as i64)).unwrap();
            Ok(DPT_2_5 {
                control: reader.read_bit()?,
                Alarm: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_2_5 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.control, self.Alarm)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_2_6 {
    pub control: bool,
    pub High: bool,
}
impl crate::Specific for DPT_2_6 {
    const DPT: DPT = DPT::new(2u16, Some(6u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = [0u8];
        let mut writer = BitWriter::endian(bytes.as_mut_slice(), BigEndian);
        writer.write_bit(self.control).unwrap();
        writer.write_bit(self.High).unwrap();
        DataPoint::Short(writer.into_unwritten().1)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Short(byte) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new([*byte]));
            reader.seek_bits(SeekFrom::End(2u16 as i64)).unwrap();
            Ok(DPT_2_6 {
                control: reader.read_bit()?,
                High: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_2_6 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.control, self.High)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_2_7 {
    pub control: bool,
    pub Increase: bool,
}
impl crate::Specific for DPT_2_7 {
    const DPT: DPT = DPT::new(2u16, Some(7u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = [0u8];
        let mut writer = BitWriter::endian(bytes.as_mut_slice(), BigEndian);
        writer.write_bit(self.control).unwrap();
        writer.write_bit(self.Increase).unwrap();
        DataPoint::Short(writer.into_unwritten().1)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Short(byte) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new([*byte]));
            reader.seek_bits(SeekFrom::End(2u16 as i64)).unwrap();
            Ok(DPT_2_7 {
                control: reader.read_bit()?,
                Increase: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_2_7 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.control, self.Increase)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_2_8 {
    pub control: bool,
    pub Down: bool,
}
impl crate::Specific for DPT_2_8 {
    const DPT: DPT = DPT::new(2u16, Some(8u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = [0u8];
        let mut writer = BitWriter::endian(bytes.as_mut_slice(), BigEndian);
        writer.write_bit(self.control).unwrap();
        writer.write_bit(self.Down).unwrap();
        DataPoint::Short(writer.into_unwritten().1)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Short(byte) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new([*byte]));
            reader.seek_bits(SeekFrom::End(2u16 as i64)).unwrap();
            Ok(DPT_2_8 {
                control: reader.read_bit()?,
                Down: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_2_8 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.control, self.Down)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_2_9 {
    pub control: bool,
    pub Close: bool,
}
impl crate::Specific for DPT_2_9 {
    const DPT: DPT = DPT::new(2u16, Some(9u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = [0u8];
        let mut writer = BitWriter::endian(bytes.as_mut_slice(), BigEndian);
        writer.write_bit(self.control).unwrap();
        writer.write_bit(self.Close).unwrap();
        DataPoint::Short(writer.into_unwritten().1)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Short(byte) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new([*byte]));
            reader.seek_bits(SeekFrom::End(2u16 as i64)).unwrap();
            Ok(DPT_2_9 {
                control: reader.read_bit()?,
                Close: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_2_9 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.control, self.Close)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_2_10 {
    pub control: bool,
    pub Start: bool,
}
impl crate::Specific for DPT_2_10 {
    const DPT: DPT = DPT::new(2u16, Some(10u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = [0u8];
        let mut writer = BitWriter::endian(bytes.as_mut_slice(), BigEndian);
        writer.write_bit(self.control).unwrap();
        writer.write_bit(self.Start).unwrap();
        DataPoint::Short(writer.into_unwritten().1)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Short(byte) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new([*byte]));
            reader.seek_bits(SeekFrom::End(2u16 as i64)).unwrap();
            Ok(DPT_2_10 {
                control: reader.read_bit()?,
                Start: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_2_10 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.control, self.Start)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_2_11 {
    pub control: bool,
    pub Active: bool,
}
impl crate::Specific for DPT_2_11 {
    const DPT: DPT = DPT::new(2u16, Some(11u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = [0u8];
        let mut writer = BitWriter::endian(bytes.as_mut_slice(), BigEndian);
        writer.write_bit(self.control).unwrap();
        writer.write_bit(self.Active).unwrap();
        DataPoint::Short(writer.into_unwritten().1)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Short(byte) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new([*byte]));
            reader.seek_bits(SeekFrom::End(2u16 as i64)).unwrap();
            Ok(DPT_2_11 {
                control: reader.read_bit()?,
                Active: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_2_11 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.control, self.Active)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_2_12 {
    pub control: bool,
    pub Inverted: bool,
}
impl crate::Specific for DPT_2_12 {
    const DPT: DPT = DPT::new(2u16, Some(12u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = [0u8];
        let mut writer = BitWriter::endian(bytes.as_mut_slice(), BigEndian);
        writer.write_bit(self.control).unwrap();
        writer.write_bit(self.Inverted).unwrap();
        DataPoint::Short(writer.into_unwritten().1)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Short(byte) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new([*byte]));
            reader.seek_bits(SeekFrom::End(2u16 as i64)).unwrap();
            Ok(DPT_2_12 {
                control: reader.read_bit()?,
                Inverted: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_2_12 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.control, self.Inverted)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_2_x {
    pub control: bool,
    pub On: bool,
}
impl crate::Specific for DPT_2_x {
    const DPT: DPT = DPT::new(2u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = [0u8];
        let mut writer = BitWriter::endian(bytes.as_mut_slice(), BigEndian);
        writer.write_bit(self.control).unwrap();
        writer.write_bit(self.On).unwrap();
        DataPoint::Short(writer.into_unwritten().1)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Short(byte) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new([*byte]));
            reader.seek_bits(SeekFrom::End(2u16 as i64)).unwrap();
            Ok(DPT_2_x {
                control: reader.read_bit()?,
                On: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_2_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.control, self.On)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_3_7 {
    pub Increase: bool,
    pub StepCode: u8,
}
impl crate::Specific for DPT_3_7 {
    const DPT: DPT = DPT::new(3u16, Some(7u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = [0u8];
        let mut writer = BitWriter::endian(bytes.as_mut_slice(), BigEndian);
        writer.write_bit(self.Increase).unwrap();
        writer.write(3u8 as u32, self.StepCode).unwrap();
        DataPoint::Short(writer.into_unwritten().1)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Short(byte) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new([*byte]));
            reader.seek_bits(SeekFrom::End(4u16 as i64)).unwrap();
            Ok(DPT_3_7 {
                Increase: reader.read_bit()?,
                StepCode: reader.read(3u8 as u32)?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_3_7 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.Increase, self.StepCode)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_3_8 {
    pub Down: bool,
    pub StepCode: u8,
}
impl crate::Specific for DPT_3_8 {
    const DPT: DPT = DPT::new(3u16, Some(8u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = [0u8];
        let mut writer = BitWriter::endian(bytes.as_mut_slice(), BigEndian);
        writer.write_bit(self.Down).unwrap();
        writer.write(3u8 as u32, self.StepCode).unwrap();
        DataPoint::Short(writer.into_unwritten().1)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Short(byte) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new([*byte]));
            reader.seek_bits(SeekFrom::End(4u16 as i64)).unwrap();
            Ok(DPT_3_8 {
                Down: reader.read_bit()?,
                StepCode: reader.read(3u8 as u32)?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_3_8 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.Down, self.StepCode)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_3_x {
    pub Increase: bool,
    pub StepCode: u8,
}
impl crate::Specific for DPT_3_x {
    const DPT: DPT = DPT::new(3u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = [0u8];
        let mut writer = BitWriter::endian(bytes.as_mut_slice(), BigEndian);
        writer.write_bit(self.Increase).unwrap();
        writer.write(3u8 as u32, self.StepCode).unwrap();
        DataPoint::Short(writer.into_unwritten().1)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Short(byte) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new([*byte]));
            reader.seek_bits(SeekFrom::End(4u16 as i64)).unwrap();
            Ok(DPT_3_x {
                Increase: reader.read_bit()?,
                StepCode: reader.read(3u8 as u32)?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_3_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.Increase, self.StepCode)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_4_1(pub char);
impl crate::Specific for DPT_4_1 {
    const DPT: DPT = DPT::new(4u16, Some(1u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_from(self.0 as u8).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_4_1(reader.read_to::<u8>()? as char))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_4_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_4_2(pub char);
impl crate::Specific for DPT_4_2 {
    const DPT: DPT = DPT::new(4u16, Some(2u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer
            .write_from(encoding::codec::singlebyte::iso_8859_1::backward(self.0 as u32))
            .unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_4_2(
                    char::from_u32(
                            encoding::codec::singlebyte::iso_8859_1::forward(
                                reader.read_to()?,
                            ) as u32,
                        )
                        .unwrap(),
                ),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_4_2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_4_x(pub char);
impl crate::Specific for DPT_4_x {
    const DPT: DPT = DPT::new(4u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_from(self.0 as u8).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_4_x(reader.read_to::<u8>()? as char))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_4_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_5_1(pub u8);
impl crate::Specific for DPT_5_1 {
    const DPT: DPT = DPT::new(5u16, Some(1u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_5_1(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_5_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_5_3(pub u8);
impl crate::Specific for DPT_5_3 {
    const DPT: DPT = DPT::new(5u16, Some(3u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_5_3(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_5_3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_5_4(pub u8);
impl crate::Specific for DPT_5_4 {
    const DPT: DPT = DPT::new(5u16, Some(4u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_5_4(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_5_4 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_5_5(pub u8);
impl crate::Specific for DPT_5_5 {
    const DPT: DPT = DPT::new(5u16, Some(5u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_5_5(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_5_5 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_5_6(pub u8);
impl crate::Specific for DPT_5_6 {
    const DPT: DPT = DPT::new(5u16, Some(6u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_5_6(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_5_6 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_5_10(pub u8);
impl crate::Specific for DPT_5_10 {
    const DPT: DPT = DPT::new(5u16, Some(10u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_5_10(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_5_10 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_5_100(pub u8);
impl crate::Specific for DPT_5_100 {
    const DPT: DPT = DPT::new(5u16, Some(100u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_5_100(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_5_100 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_5_x(pub u8);
impl crate::Specific for DPT_5_x {
    const DPT: DPT = DPT::new(5u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_5_x(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_5_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_6_1(pub i8);
impl crate::Specific for DPT_6_1 {
    const DPT: DPT = DPT::new(6u16, Some(1u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_signed(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_6_1(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_6_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_6_10(pub i8);
impl crate::Specific for DPT_6_10 {
    const DPT: DPT = DPT::new(6u16, Some(10u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_signed(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_6_10(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_6_10 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_6_20 {
    pub StatusA: bool,
    pub StatusB: bool,
    pub StatusC: bool,
    pub StatusD: bool,
    pub StatusE: bool,
    pub enumeration: u8,
}
impl crate::Specific for DPT_6_20 {
    const DPT: DPT = DPT::new(6u16, Some(20u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_bit(self.StatusA).unwrap();
        writer.write_bit(self.StatusB).unwrap();
        writer.write_bit(self.StatusC).unwrap();
        writer.write_bit(self.StatusD).unwrap();
        writer.write_bit(self.StatusE).unwrap();
        writer.write(3u8 as u32, self.enumeration).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_6_20 {
                StatusA: reader.read_bit()?,
                StatusB: reader.read_bit()?,
                StatusC: reader.read_bit()?,
                StatusD: reader.read_bit()?,
                StatusE: reader.read_bit()?,
                enumeration: reader.read(3u8 as u32)?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_6_20 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}", self.StatusA, self.StatusB, self.StatusC, self
            .StatusD, self.StatusE, self.enumeration
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_6_x(pub i8);
impl crate::Specific for DPT_6_x {
    const DPT: DPT = DPT::new(6u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_signed(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_6_x(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_6_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_7_1(pub u16);
impl crate::Specific for DPT_7_1 {
    const DPT: DPT = DPT::new(7u16, Some(1u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_7_1(reader.read(16u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_7_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_7_2(pub u16);
impl crate::Specific for DPT_7_2 {
    const DPT: DPT = DPT::new(7u16, Some(2u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_7_2(reader.read(16u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_7_2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_7_3(pub u16);
impl crate::Specific for DPT_7_3 {
    const DPT: DPT = DPT::new(7u16, Some(3u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_7_3(reader.read(16u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_7_3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_7_4(pub u16);
impl crate::Specific for DPT_7_4 {
    const DPT: DPT = DPT::new(7u16, Some(4u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_7_4(reader.read(16u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_7_4 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_7_5(pub u16);
impl crate::Specific for DPT_7_5 {
    const DPT: DPT = DPT::new(7u16, Some(5u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_7_5(reader.read(16u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_7_5 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_7_6(pub u16);
impl crate::Specific for DPT_7_6 {
    const DPT: DPT = DPT::new(7u16, Some(6u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_7_6(reader.read(16u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_7_6 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_7_7(pub u16);
impl crate::Specific for DPT_7_7 {
    const DPT: DPT = DPT::new(7u16, Some(7u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_7_7(reader.read(16u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_7_7 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_7_10(pub u16);
impl crate::Specific for DPT_7_10 {
    const DPT: DPT = DPT::new(7u16, Some(10u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_7_10(reader.read(16u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_7_10 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_7_11(pub u16);
impl crate::Specific for DPT_7_11 {
    const DPT: DPT = DPT::new(7u16, Some(11u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_7_11(reader.read(16u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_7_11 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_7_12(pub u16);
impl crate::Specific for DPT_7_12 {
    const DPT: DPT = DPT::new(7u16, Some(12u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_7_12(reader.read(16u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_7_12 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_7_13(pub u16);
impl crate::Specific for DPT_7_13 {
    const DPT: DPT = DPT::new(7u16, Some(13u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_7_13(reader.read(16u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_7_13 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_7_600(pub u16);
impl crate::Specific for DPT_7_600 {
    const DPT: DPT = DPT::new(7u16, Some(600u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_7_600(reader.read(16u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_7_600 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_7_x(pub u16);
impl crate::Specific for DPT_7_x {
    const DPT: DPT = DPT::new(7u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_7_x(reader.read(16u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_7_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_8_1(pub i16);
impl crate::Specific for DPT_8_1 {
    const DPT: DPT = DPT::new(8u16, Some(1u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_signed(16u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_8_1(reader.read(16u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_8_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_8_2(pub i16);
impl crate::Specific for DPT_8_2 {
    const DPT: DPT = DPT::new(8u16, Some(2u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_signed(16u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_8_2(reader.read(16u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_8_2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_8_3(pub i16);
impl crate::Specific for DPT_8_3 {
    const DPT: DPT = DPT::new(8u16, Some(3u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_signed(16u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_8_3(reader.read(16u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_8_3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_8_4(pub i16);
impl crate::Specific for DPT_8_4 {
    const DPT: DPT = DPT::new(8u16, Some(4u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_signed(16u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_8_4(reader.read(16u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_8_4 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_8_5(pub i16);
impl crate::Specific for DPT_8_5 {
    const DPT: DPT = DPT::new(8u16, Some(5u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_signed(16u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_8_5(reader.read(16u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_8_5 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_8_6(pub i16);
impl crate::Specific for DPT_8_6 {
    const DPT: DPT = DPT::new(8u16, Some(6u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_signed(16u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_8_6(reader.read(16u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_8_6 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_8_7(pub i16);
impl crate::Specific for DPT_8_7 {
    const DPT: DPT = DPT::new(8u16, Some(7u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_signed(16u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_8_7(reader.read(16u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_8_7 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_8_10(pub i16);
impl crate::Specific for DPT_8_10 {
    const DPT: DPT = DPT::new(8u16, Some(10u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_signed(16u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_8_10(reader.read(16u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_8_10 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_8_11(pub i16);
impl crate::Specific for DPT_8_11 {
    const DPT: DPT = DPT::new(8u16, Some(11u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_signed(16u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_8_11(reader.read(16u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_8_11 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_8_12(pub i16);
impl crate::Specific for DPT_8_12 {
    const DPT: DPT = DPT::new(8u16, Some(12u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_signed(16u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_8_12(reader.read(16u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_8_12 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_8_x(pub i16);
impl crate::Specific for DPT_8_x {
    const DPT: DPT = DPT::new(8u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_signed(16u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_8_x(reader.read(16u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_8_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_9_1(pub f32);
impl crate::Specific for DPT_9_1 {
    const DPT: DPT = DPT::new(9u16, Some(1u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, encode_knxf16(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_9_1({
                    let value: u16 = reader.read(16u8 as u32)?;
                    let value = decode_knxf16(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_9_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_9_2(pub f32);
impl crate::Specific for DPT_9_2 {
    const DPT: DPT = DPT::new(9u16, Some(2u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, encode_knxf16(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_9_2({
                    let value: u16 = reader.read(16u8 as u32)?;
                    let value = decode_knxf16(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_9_2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_9_3(pub f32);
impl crate::Specific for DPT_9_3 {
    const DPT: DPT = DPT::new(9u16, Some(3u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, encode_knxf16(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_9_3({
                    let value: u16 = reader.read(16u8 as u32)?;
                    let value = decode_knxf16(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_9_3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_9_4(pub f32);
impl crate::Specific for DPT_9_4 {
    const DPT: DPT = DPT::new(9u16, Some(4u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, encode_knxf16(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_9_4({
                    let value: u16 = reader.read(16u8 as u32)?;
                    let value = decode_knxf16(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_9_4 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_9_5(pub f32);
impl crate::Specific for DPT_9_5 {
    const DPT: DPT = DPT::new(9u16, Some(5u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, encode_knxf16(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_9_5({
                    let value: u16 = reader.read(16u8 as u32)?;
                    let value = decode_knxf16(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_9_5 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_9_6(pub f32);
impl crate::Specific for DPT_9_6 {
    const DPT: DPT = DPT::new(9u16, Some(6u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, encode_knxf16(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_9_6({
                    let value: u16 = reader.read(16u8 as u32)?;
                    let value = decode_knxf16(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_9_6 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_9_7(pub f32);
impl crate::Specific for DPT_9_7 {
    const DPT: DPT = DPT::new(9u16, Some(7u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, encode_knxf16(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_9_7({
                    let value: u16 = reader.read(16u8 as u32)?;
                    let value = decode_knxf16(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_9_7 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_9_8(pub f32);
impl crate::Specific for DPT_9_8 {
    const DPT: DPT = DPT::new(9u16, Some(8u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, encode_knxf16(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_9_8({
                    let value: u16 = reader.read(16u8 as u32)?;
                    let value = decode_knxf16(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_9_8 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_9_9(pub f32);
impl crate::Specific for DPT_9_9 {
    const DPT: DPT = DPT::new(9u16, Some(9u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, encode_knxf16(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_9_9({
                    let value: u16 = reader.read(16u8 as u32)?;
                    let value = decode_knxf16(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_9_9 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_9_10(pub f32);
impl crate::Specific for DPT_9_10 {
    const DPT: DPT = DPT::new(9u16, Some(10u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, encode_knxf16(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_9_10({
                    let value: u16 = reader.read(16u8 as u32)?;
                    let value = decode_knxf16(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_9_10 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_9_11(pub f32);
impl crate::Specific for DPT_9_11 {
    const DPT: DPT = DPT::new(9u16, Some(11u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, encode_knxf16(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_9_11({
                    let value: u16 = reader.read(16u8 as u32)?;
                    let value = decode_knxf16(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_9_11 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_9_20(pub f32);
impl crate::Specific for DPT_9_20 {
    const DPT: DPT = DPT::new(9u16, Some(20u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, encode_knxf16(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_9_20({
                    let value: u16 = reader.read(16u8 as u32)?;
                    let value = decode_knxf16(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_9_20 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_9_21(pub f32);
impl crate::Specific for DPT_9_21 {
    const DPT: DPT = DPT::new(9u16, Some(21u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, encode_knxf16(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_9_21({
                    let value: u16 = reader.read(16u8 as u32)?;
                    let value = decode_knxf16(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_9_21 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_9_22(pub f32);
impl crate::Specific for DPT_9_22 {
    const DPT: DPT = DPT::new(9u16, Some(22u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, encode_knxf16(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_9_22({
                    let value: u16 = reader.read(16u8 as u32)?;
                    let value = decode_knxf16(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_9_22 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_9_23(pub f32);
impl crate::Specific for DPT_9_23 {
    const DPT: DPT = DPT::new(9u16, Some(23u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, encode_knxf16(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_9_23({
                    let value: u16 = reader.read(16u8 as u32)?;
                    let value = decode_knxf16(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_9_23 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_9_24(pub f32);
impl crate::Specific for DPT_9_24 {
    const DPT: DPT = DPT::new(9u16, Some(24u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, encode_knxf16(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_9_24({
                    let value: u16 = reader.read(16u8 as u32)?;
                    let value = decode_knxf16(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_9_24 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_9_25(pub f32);
impl crate::Specific for DPT_9_25 {
    const DPT: DPT = DPT::new(9u16, Some(25u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, encode_knxf16(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_9_25({
                    let value: u16 = reader.read(16u8 as u32)?;
                    let value = decode_knxf16(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_9_25 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_9_26(pub f32);
impl crate::Specific for DPT_9_26 {
    const DPT: DPT = DPT::new(9u16, Some(26u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, encode_knxf16(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_9_26({
                    let value: u16 = reader.read(16u8 as u32)?;
                    let value = decode_knxf16(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_9_26 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_9_27(pub f32);
impl crate::Specific for DPT_9_27 {
    const DPT: DPT = DPT::new(9u16, Some(27u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, encode_knxf16(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_9_27({
                    let value: u16 = reader.read(16u8 as u32)?;
                    let value = decode_knxf16(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_9_27 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_9_28(pub f32);
impl crate::Specific for DPT_9_28 {
    const DPT: DPT = DPT::new(9u16, Some(28u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, encode_knxf16(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_9_28({
                    let value: u16 = reader.read(16u8 as u32)?;
                    let value = decode_knxf16(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_9_28 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_9_29(pub f32);
impl crate::Specific for DPT_9_29 {
    const DPT: DPT = DPT::new(9u16, Some(29u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, encode_knxf16(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_9_29({
                    let value: u16 = reader.read(16u8 as u32)?;
                    let value = decode_knxf16(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_9_29 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_9_30(pub f32);
impl crate::Specific for DPT_9_30 {
    const DPT: DPT = DPT::new(9u16, Some(30u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, encode_knxf16(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_9_30({
                    let value: u16 = reader.read(16u8 as u32)?;
                    let value = decode_knxf16(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_9_30 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_9_31(pub f32);
impl crate::Specific for DPT_9_31 {
    const DPT: DPT = DPT::new(9u16, Some(31u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, encode_knxf16(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_9_31({
                    let value: u16 = reader.read(16u8 as u32)?;
                    let value = decode_knxf16(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_9_31 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_9_x(pub f32);
impl crate::Specific for DPT_9_x {
    const DPT: DPT = DPT::new(9u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, encode_knxf16(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_9_x({
                    let value: u16 = reader.read(16u8 as u32)?;
                    let value = decode_knxf16(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_9_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_10_1 {
    pub Day: u8,
    pub Hour: u8,
    pub reserved: Reserved,
    pub Minutes: u8,
    pub reserved2: Reserved,
    pub Seconds: u8,
}
impl crate::Specific for DPT_10_1 {
    const DPT: DPT = DPT::new(10u16, Some(1u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(3u8 as u32, self.Day).unwrap();
        writer.write(5u8 as u32, self.Hour).unwrap();
        writer.write(2u8 as u32, 0u32).unwrap();
        writer.write(6u8 as u32, self.Minutes).unwrap();
        writer.write(2u8 as u32, 0u32).unwrap();
        writer.write(6u8 as u32, self.Seconds).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_10_1 {
                Day: reader.read(3u8 as u32)?,
                Hour: reader.read(5u8 as u32)?,
                reserved: {
                    reader.skip(2u8 as u32)?;
                    Reserved::new()
                },
                Minutes: reader.read(6u8 as u32)?,
                reserved2: {
                    reader.skip(2u8 as u32)?;
                    Reserved::new()
                },
                Seconds: reader.read(6u8 as u32)?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_10_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}", self.Day, self.Hour, self.reserved, self.Minutes,
            self.reserved2, self.Seconds
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_10_x {
    pub Day: u8,
    pub Hour: u8,
    pub reserved: Reserved,
    pub Minutes: u8,
    pub reserved2: Reserved,
    pub Seconds: u8,
}
impl crate::Specific for DPT_10_x {
    const DPT: DPT = DPT::new(10u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(3u8 as u32, self.Day).unwrap();
        writer.write(5u8 as u32, self.Hour).unwrap();
        writer.write(2u8 as u32, 0u32).unwrap();
        writer.write(6u8 as u32, self.Minutes).unwrap();
        writer.write(2u8 as u32, 0u32).unwrap();
        writer.write(6u8 as u32, self.Seconds).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_10_x {
                Day: reader.read(3u8 as u32)?,
                Hour: reader.read(5u8 as u32)?,
                reserved: {
                    reader.skip(2u8 as u32)?;
                    Reserved::new()
                },
                Minutes: reader.read(6u8 as u32)?,
                reserved2: {
                    reader.skip(2u8 as u32)?;
                    Reserved::new()
                },
                Seconds: reader.read(6u8 as u32)?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_10_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}", self.Day, self.Hour, self.reserved, self.Minutes,
            self.reserved2, self.Seconds
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_11_1 {
    pub reserved: Reserved,
    pub int: u8,
    pub reserved2: Reserved,
    pub int2: u8,
    pub reserved3: Reserved,
    pub int3: u8,
}
impl crate::Specific for DPT_11_1 {
    const DPT: DPT = DPT::new(11u16, Some(1u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(3u8 as u32, 0u32).unwrap();
        writer.write(5u8 as u32, self.int).unwrap();
        writer.write(4u8 as u32, 0u32).unwrap();
        writer.write(4u8 as u32, self.int2).unwrap();
        writer.write(1u8 as u32, 0u32).unwrap();
        writer.write(7u8 as u32, self.int3).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_11_1 {
                reserved: {
                    reader.skip(3u8 as u32)?;
                    Reserved::new()
                },
                int: reader.read(5u8 as u32)?,
                reserved2: {
                    reader.skip(4u8 as u32)?;
                    Reserved::new()
                },
                int2: reader.read(4u8 as u32)?,
                reserved3: {
                    reader.skip(1u8 as u32)?;
                    Reserved::new()
                },
                int3: reader.read(7u8 as u32)?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_11_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}", self.reserved, self.int, self.reserved2, self.int2,
            self.reserved3, self.int3
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_11_x {
    pub reserved: Reserved,
    pub int: u8,
    pub reserved2: Reserved,
    pub int2: u8,
    pub reserved3: Reserved,
    pub int3: u8,
}
impl crate::Specific for DPT_11_x {
    const DPT: DPT = DPT::new(11u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(3u8 as u32, 0u32).unwrap();
        writer.write(5u8 as u32, self.int).unwrap();
        writer.write(4u8 as u32, 0u32).unwrap();
        writer.write(4u8 as u32, self.int2).unwrap();
        writer.write(1u8 as u32, 0u32).unwrap();
        writer.write(7u8 as u32, self.int3).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_11_x {
                reserved: {
                    reader.skip(3u8 as u32)?;
                    Reserved::new()
                },
                int: reader.read(5u8 as u32)?,
                reserved2: {
                    reader.skip(4u8 as u32)?;
                    Reserved::new()
                },
                int2: reader.read(4u8 as u32)?,
                reserved3: {
                    reader.skip(1u8 as u32)?;
                    Reserved::new()
                },
                int3: reader.read(7u8 as u32)?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_11_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}", self.reserved, self.int, self.reserved2, self.int2,
            self.reserved3, self.int3
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_12_1(pub u32);
impl crate::Specific for DPT_12_1 {
    const DPT: DPT = DPT::new(12u16, Some(1u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_12_1(reader.read(32u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_12_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_12_100(pub u32);
impl crate::Specific for DPT_12_100 {
    const DPT: DPT = DPT::new(12u16, Some(100u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_12_100(reader.read(32u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_12_100 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_12_101(pub u32);
impl crate::Specific for DPT_12_101 {
    const DPT: DPT = DPT::new(12u16, Some(101u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_12_101(reader.read(32u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_12_101 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_12_102(pub u32);
impl crate::Specific for DPT_12_102 {
    const DPT: DPT = DPT::new(12u16, Some(102u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_12_102(reader.read(32u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_12_102 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_12_1200(pub u32);
impl crate::Specific for DPT_12_1200 {
    const DPT: DPT = DPT::new(12u16, Some(1200u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_12_1200(reader.read(32u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_12_1200 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_12_1201(pub u32);
impl crate::Specific for DPT_12_1201 {
    const DPT: DPT = DPT::new(12u16, Some(1201u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_12_1201(reader.read(32u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_12_1201 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_12_x(pub u32);
impl crate::Specific for DPT_12_x {
    const DPT: DPT = DPT::new(12u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_12_x(reader.read(32u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_12_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_13_1(pub i32);
impl crate::Specific for DPT_13_1 {
    const DPT: DPT = DPT::new(13u16, Some(1u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_signed(32u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_13_1(reader.read(32u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_13_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_13_2(pub i32);
impl crate::Specific for DPT_13_2 {
    const DPT: DPT = DPT::new(13u16, Some(2u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_signed(32u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_13_2(reader.read(32u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_13_2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_13_10(pub i32);
impl crate::Specific for DPT_13_10 {
    const DPT: DPT = DPT::new(13u16, Some(10u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_signed(32u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_13_10(reader.read(32u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_13_10 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_13_11(pub i32);
impl crate::Specific for DPT_13_11 {
    const DPT: DPT = DPT::new(13u16, Some(11u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_signed(32u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_13_11(reader.read(32u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_13_11 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_13_12(pub i32);
impl crate::Specific for DPT_13_12 {
    const DPT: DPT = DPT::new(13u16, Some(12u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_signed(32u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_13_12(reader.read(32u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_13_12 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_13_13(pub i32);
impl crate::Specific for DPT_13_13 {
    const DPT: DPT = DPT::new(13u16, Some(13u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_signed(32u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_13_13(reader.read(32u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_13_13 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_13_14(pub i32);
impl crate::Specific for DPT_13_14 {
    const DPT: DPT = DPT::new(13u16, Some(14u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_signed(32u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_13_14(reader.read(32u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_13_14 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_13_15(pub i32);
impl crate::Specific for DPT_13_15 {
    const DPT: DPT = DPT::new(13u16, Some(15u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_signed(32u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_13_15(reader.read(32u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_13_15 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_13_16(pub i32);
impl crate::Specific for DPT_13_16 {
    const DPT: DPT = DPT::new(13u16, Some(16u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_signed(32u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_13_16(reader.read(32u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_13_16 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_13_100(pub i32);
impl crate::Specific for DPT_13_100 {
    const DPT: DPT = DPT::new(13u16, Some(100u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_signed(32u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_13_100(reader.read(32u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_13_100 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_13_1200(pub i32);
impl crate::Specific for DPT_13_1200 {
    const DPT: DPT = DPT::new(13u16, Some(1200u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_signed(32u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_13_1200(reader.read(32u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_13_1200 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_13_1201(pub i32);
impl crate::Specific for DPT_13_1201 {
    const DPT: DPT = DPT::new(13u16, Some(1201u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_signed(32u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_13_1201(reader.read(32u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_13_1201 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_13_x(pub i32);
impl crate::Specific for DPT_13_x {
    const DPT: DPT = DPT::new(13u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_signed(32u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_13_x(reader.read(32u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_13_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_0(pub f32);
impl crate::Specific for DPT_14_0 {
    const DPT: DPT = DPT::new(14u16, Some(0u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_0({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_0 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_1(pub f32);
impl crate::Specific for DPT_14_1 {
    const DPT: DPT = DPT::new(14u16, Some(1u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_1({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_2(pub f32);
impl crate::Specific for DPT_14_2 {
    const DPT: DPT = DPT::new(14u16, Some(2u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_2({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_3(pub f32);
impl crate::Specific for DPT_14_3 {
    const DPT: DPT = DPT::new(14u16, Some(3u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_3({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_4(pub f32);
impl crate::Specific for DPT_14_4 {
    const DPT: DPT = DPT::new(14u16, Some(4u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_4({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_4 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_5(pub f32);
impl crate::Specific for DPT_14_5 {
    const DPT: DPT = DPT::new(14u16, Some(5u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_5({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_5 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_6(pub f32);
impl crate::Specific for DPT_14_6 {
    const DPT: DPT = DPT::new(14u16, Some(6u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_6({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_6 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_7(pub f32);
impl crate::Specific for DPT_14_7 {
    const DPT: DPT = DPT::new(14u16, Some(7u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_7({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_7 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_8(pub f32);
impl crate::Specific for DPT_14_8 {
    const DPT: DPT = DPT::new(14u16, Some(8u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_8({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_8 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_9(pub f32);
impl crate::Specific for DPT_14_9 {
    const DPT: DPT = DPT::new(14u16, Some(9u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_9({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_9 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_10(pub f32);
impl crate::Specific for DPT_14_10 {
    const DPT: DPT = DPT::new(14u16, Some(10u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_10({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_10 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_11(pub f32);
impl crate::Specific for DPT_14_11 {
    const DPT: DPT = DPT::new(14u16, Some(11u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_11({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_11 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_12(pub f32);
impl crate::Specific for DPT_14_12 {
    const DPT: DPT = DPT::new(14u16, Some(12u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_12({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_12 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_13(pub f32);
impl crate::Specific for DPT_14_13 {
    const DPT: DPT = DPT::new(14u16, Some(13u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_13({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_13 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_14(pub f32);
impl crate::Specific for DPT_14_14 {
    const DPT: DPT = DPT::new(14u16, Some(14u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_14({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_14 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_15(pub f32);
impl crate::Specific for DPT_14_15 {
    const DPT: DPT = DPT::new(14u16, Some(15u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_15({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_15 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_16(pub f32);
impl crate::Specific for DPT_14_16 {
    const DPT: DPT = DPT::new(14u16, Some(16u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_16({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_16 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_17(pub f32);
impl crate::Specific for DPT_14_17 {
    const DPT: DPT = DPT::new(14u16, Some(17u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_17({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_17 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_18(pub f32);
impl crate::Specific for DPT_14_18 {
    const DPT: DPT = DPT::new(14u16, Some(18u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_18({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_18 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_19(pub f32);
impl crate::Specific for DPT_14_19 {
    const DPT: DPT = DPT::new(14u16, Some(19u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_19({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_19 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_20(pub f32);
impl crate::Specific for DPT_14_20 {
    const DPT: DPT = DPT::new(14u16, Some(20u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_20({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_20 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_21(pub f32);
impl crate::Specific for DPT_14_21 {
    const DPT: DPT = DPT::new(14u16, Some(21u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_21({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_21 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_22(pub f32);
impl crate::Specific for DPT_14_22 {
    const DPT: DPT = DPT::new(14u16, Some(22u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_22({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_22 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_23(pub f32);
impl crate::Specific for DPT_14_23 {
    const DPT: DPT = DPT::new(14u16, Some(23u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_23({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_23 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_24(pub f32);
impl crate::Specific for DPT_14_24 {
    const DPT: DPT = DPT::new(14u16, Some(24u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_24({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_24 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_25(pub f32);
impl crate::Specific for DPT_14_25 {
    const DPT: DPT = DPT::new(14u16, Some(25u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_25({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_25 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_26(pub f32);
impl crate::Specific for DPT_14_26 {
    const DPT: DPT = DPT::new(14u16, Some(26u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_26({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_26 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_27(pub f32);
impl crate::Specific for DPT_14_27 {
    const DPT: DPT = DPT::new(14u16, Some(27u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_27({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_27 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_28(pub f32);
impl crate::Specific for DPT_14_28 {
    const DPT: DPT = DPT::new(14u16, Some(28u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_28({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_28 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_29(pub f32);
impl crate::Specific for DPT_14_29 {
    const DPT: DPT = DPT::new(14u16, Some(29u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_29({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_29 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_30(pub f32);
impl crate::Specific for DPT_14_30 {
    const DPT: DPT = DPT::new(14u16, Some(30u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_30({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_30 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_31(pub f32);
impl crate::Specific for DPT_14_31 {
    const DPT: DPT = DPT::new(14u16, Some(31u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_31({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_31 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_32(pub f32);
impl crate::Specific for DPT_14_32 {
    const DPT: DPT = DPT::new(14u16, Some(32u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_32({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_32 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_33(pub f32);
impl crate::Specific for DPT_14_33 {
    const DPT: DPT = DPT::new(14u16, Some(33u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_33({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_33 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_34(pub f32);
impl crate::Specific for DPT_14_34 {
    const DPT: DPT = DPT::new(14u16, Some(34u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_34({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_34 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_35(pub f32);
impl crate::Specific for DPT_14_35 {
    const DPT: DPT = DPT::new(14u16, Some(35u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_35({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_35 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_36(pub f32);
impl crate::Specific for DPT_14_36 {
    const DPT: DPT = DPT::new(14u16, Some(36u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_36({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_36 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_37(pub f32);
impl crate::Specific for DPT_14_37 {
    const DPT: DPT = DPT::new(14u16, Some(37u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_37({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_37 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_38(pub f32);
impl crate::Specific for DPT_14_38 {
    const DPT: DPT = DPT::new(14u16, Some(38u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_38({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_38 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_39(pub f32);
impl crate::Specific for DPT_14_39 {
    const DPT: DPT = DPT::new(14u16, Some(39u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_39({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_39 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_40(pub f32);
impl crate::Specific for DPT_14_40 {
    const DPT: DPT = DPT::new(14u16, Some(40u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_40({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_40 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_41(pub f32);
impl crate::Specific for DPT_14_41 {
    const DPT: DPT = DPT::new(14u16, Some(41u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_41({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_41 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_42(pub f32);
impl crate::Specific for DPT_14_42 {
    const DPT: DPT = DPT::new(14u16, Some(42u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_42({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_42 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_43(pub f32);
impl crate::Specific for DPT_14_43 {
    const DPT: DPT = DPT::new(14u16, Some(43u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_43({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_43 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_44(pub f32);
impl crate::Specific for DPT_14_44 {
    const DPT: DPT = DPT::new(14u16, Some(44u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_44({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_44 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_45(pub f32);
impl crate::Specific for DPT_14_45 {
    const DPT: DPT = DPT::new(14u16, Some(45u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_45({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_45 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_46(pub f32);
impl crate::Specific for DPT_14_46 {
    const DPT: DPT = DPT::new(14u16, Some(46u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_46({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_46 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_47(pub f32);
impl crate::Specific for DPT_14_47 {
    const DPT: DPT = DPT::new(14u16, Some(47u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_47({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_47 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_48(pub f32);
impl crate::Specific for DPT_14_48 {
    const DPT: DPT = DPT::new(14u16, Some(48u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_48({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_48 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_49(pub f32);
impl crate::Specific for DPT_14_49 {
    const DPT: DPT = DPT::new(14u16, Some(49u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_49({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_49 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_50(pub f32);
impl crate::Specific for DPT_14_50 {
    const DPT: DPT = DPT::new(14u16, Some(50u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_50({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_50 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_51(pub f32);
impl crate::Specific for DPT_14_51 {
    const DPT: DPT = DPT::new(14u16, Some(51u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_51({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_51 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_52(pub f32);
impl crate::Specific for DPT_14_52 {
    const DPT: DPT = DPT::new(14u16, Some(52u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_52({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_52 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_53(pub f32);
impl crate::Specific for DPT_14_53 {
    const DPT: DPT = DPT::new(14u16, Some(53u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_53({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_53 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_54(pub f32);
impl crate::Specific for DPT_14_54 {
    const DPT: DPT = DPT::new(14u16, Some(54u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_54({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_54 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_55(pub f32);
impl crate::Specific for DPT_14_55 {
    const DPT: DPT = DPT::new(14u16, Some(55u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_55({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_55 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_56(pub f32);
impl crate::Specific for DPT_14_56 {
    const DPT: DPT = DPT::new(14u16, Some(56u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_56({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_56 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_57(pub f32);
impl crate::Specific for DPT_14_57 {
    const DPT: DPT = DPT::new(14u16, Some(57u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_57({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_57 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_58(pub f32);
impl crate::Specific for DPT_14_58 {
    const DPT: DPT = DPT::new(14u16, Some(58u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_58({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_58 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_59(pub f32);
impl crate::Specific for DPT_14_59 {
    const DPT: DPT = DPT::new(14u16, Some(59u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_59({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_59 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_60(pub f32);
impl crate::Specific for DPT_14_60 {
    const DPT: DPT = DPT::new(14u16, Some(60u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_60({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_60 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_61(pub f32);
impl crate::Specific for DPT_14_61 {
    const DPT: DPT = DPT::new(14u16, Some(61u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_61({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_61 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_62(pub f32);
impl crate::Specific for DPT_14_62 {
    const DPT: DPT = DPT::new(14u16, Some(62u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_62({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_62 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_63(pub f32);
impl crate::Specific for DPT_14_63 {
    const DPT: DPT = DPT::new(14u16, Some(63u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_63({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_63 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_64(pub f32);
impl crate::Specific for DPT_14_64 {
    const DPT: DPT = DPT::new(14u16, Some(64u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_64({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_64 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_65(pub f32);
impl crate::Specific for DPT_14_65 {
    const DPT: DPT = DPT::new(14u16, Some(65u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_65({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_65 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_66(pub f32);
impl crate::Specific for DPT_14_66 {
    const DPT: DPT = DPT::new(14u16, Some(66u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_66({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_66 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_67(pub f32);
impl crate::Specific for DPT_14_67 {
    const DPT: DPT = DPT::new(14u16, Some(67u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_67({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_67 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_68(pub f32);
impl crate::Specific for DPT_14_68 {
    const DPT: DPT = DPT::new(14u16, Some(68u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_68({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_68 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_69(pub f32);
impl crate::Specific for DPT_14_69 {
    const DPT: DPT = DPT::new(14u16, Some(69u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_69({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_69 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_70(pub f32);
impl crate::Specific for DPT_14_70 {
    const DPT: DPT = DPT::new(14u16, Some(70u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_70({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_70 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_71(pub f32);
impl crate::Specific for DPT_14_71 {
    const DPT: DPT = DPT::new(14u16, Some(71u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_71({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_71 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_72(pub f32);
impl crate::Specific for DPT_14_72 {
    const DPT: DPT = DPT::new(14u16, Some(72u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_72({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_72 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_73(pub f32);
impl crate::Specific for DPT_14_73 {
    const DPT: DPT = DPT::new(14u16, Some(73u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_73({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_73 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_74(pub f32);
impl crate::Specific for DPT_14_74 {
    const DPT: DPT = DPT::new(14u16, Some(74u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_74({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_74 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_75(pub f32);
impl crate::Specific for DPT_14_75 {
    const DPT: DPT = DPT::new(14u16, Some(75u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_75({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_75 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_76(pub f32);
impl crate::Specific for DPT_14_76 {
    const DPT: DPT = DPT::new(14u16, Some(76u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_76({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_76 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_77(pub f32);
impl crate::Specific for DPT_14_77 {
    const DPT: DPT = DPT::new(14u16, Some(77u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_77({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_77 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_78(pub f32);
impl crate::Specific for DPT_14_78 {
    const DPT: DPT = DPT::new(14u16, Some(78u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_78({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_78 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_79(pub f32);
impl crate::Specific for DPT_14_79 {
    const DPT: DPT = DPT::new(14u16, Some(79u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_79({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_79 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_80(pub f32);
impl crate::Specific for DPT_14_80 {
    const DPT: DPT = DPT::new(14u16, Some(80u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_80({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_80 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_1200(pub f32);
impl crate::Specific for DPT_14_1200 {
    const DPT: DPT = DPT::new(14u16, Some(1200u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_1200({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_1200 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_1201(pub f32);
impl crate::Specific for DPT_14_1201 {
    const DPT: DPT = DPT::new(14u16, Some(1201u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_1201({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_1201 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_14_x(pub f32);
impl crate::Specific for DPT_14_x {
    const DPT: DPT = DPT::new(14u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.0)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_14_x({
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_14_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_15_0 {
    pub int: u8,
    pub int2: u8,
    pub int3: u8,
    pub int4: u8,
    pub int5: u8,
    pub int6: u8,
    pub Detectionerror: bool,
    pub Permission: bool,
    pub Readdirection: bool,
    pub Encryptionofaccessinformation: bool,
    pub Indexofaccessidentificationcode: u8,
}
impl crate::Specific for DPT_15_0 {
    const DPT: DPT = DPT::new(15u16, Some(0u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(4u8 as u32, self.int).unwrap();
        writer.write(4u8 as u32, self.int2).unwrap();
        writer.write(4u8 as u32, self.int3).unwrap();
        writer.write(4u8 as u32, self.int4).unwrap();
        writer.write(4u8 as u32, self.int5).unwrap();
        writer.write(4u8 as u32, self.int6).unwrap();
        writer.write_bit(self.Detectionerror).unwrap();
        writer.write_bit(self.Permission).unwrap();
        writer.write_bit(self.Readdirection).unwrap();
        writer.write_bit(self.Encryptionofaccessinformation).unwrap();
        writer.write(4u8 as u32, self.Indexofaccessidentificationcode).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_15_0 {
                int: reader.read(4u8 as u32)?,
                int2: reader.read(4u8 as u32)?,
                int3: reader.read(4u8 as u32)?,
                int4: reader.read(4u8 as u32)?,
                int5: reader.read(4u8 as u32)?,
                int6: reader.read(4u8 as u32)?,
                Detectionerror: reader.read_bit()?,
                Permission: reader.read_bit()?,
                Readdirection: reader.read_bit()?,
                Encryptionofaccessinformation: reader.read_bit()?,
                Indexofaccessidentificationcode: reader.read(4u8 as u32)?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_15_0 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}", self.int, self.int2, self.int3, self
            .int4, self.int5, self.int6, self.Detectionerror, self.Permission, self
            .Readdirection, self.Encryptionofaccessinformation, self
            .Indexofaccessidentificationcode
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_15_x {
    pub int: u8,
    pub int2: u8,
    pub int3: u8,
    pub int4: u8,
    pub int5: u8,
    pub int6: u8,
    pub Detectionerror: bool,
    pub Permission: bool,
    pub Readdirection: bool,
    pub Encryptionofaccessinformation: bool,
    pub Indexofaccessidentificationcode: u8,
}
impl crate::Specific for DPT_15_x {
    const DPT: DPT = DPT::new(15u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(4u8 as u32, self.int).unwrap();
        writer.write(4u8 as u32, self.int2).unwrap();
        writer.write(4u8 as u32, self.int3).unwrap();
        writer.write(4u8 as u32, self.int4).unwrap();
        writer.write(4u8 as u32, self.int5).unwrap();
        writer.write(4u8 as u32, self.int6).unwrap();
        writer.write_bit(self.Detectionerror).unwrap();
        writer.write_bit(self.Permission).unwrap();
        writer.write_bit(self.Readdirection).unwrap();
        writer.write_bit(self.Encryptionofaccessinformation).unwrap();
        writer.write(4u8 as u32, self.Indexofaccessidentificationcode).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_15_x {
                int: reader.read(4u8 as u32)?,
                int2: reader.read(4u8 as u32)?,
                int3: reader.read(4u8 as u32)?,
                int4: reader.read(4u8 as u32)?,
                int5: reader.read(4u8 as u32)?,
                int6: reader.read(4u8 as u32)?,
                Detectionerror: reader.read_bit()?,
                Permission: reader.read_bit()?,
                Readdirection: reader.read_bit()?,
                Encryptionofaccessinformation: reader.read_bit()?,
                Indexofaccessidentificationcode: reader.read(4u8 as u32)?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_15_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}", self.int, self.int2, self.int3, self
            .int4, self.int5, self.int6, self.Detectionerror, self.Permission, self
            .Readdirection, self.Encryptionofaccessinformation, self
            .Indexofaccessidentificationcode
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_16_0(pub String);
impl crate::Specific for DPT_16_0 {
    const DPT: DPT = DPT::new(16u16, Some(0u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        let mut buffer = vec![0u8; (112u16 / 8) as usize];
        ASCII.encode_to(self.0.as_str(), EncoderTrap::Replace, &mut buffer).unwrap();
        writer.write_bytes(&buffer).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_16_0({
                    let mut value = [0u8; (112u16 / 8) as usize];
                    reader.read_bytes(&mut value)?;
                    ASCII.decode(&value, DecoderTrap::Replace).unwrap()
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_16_0 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_16_1(pub String);
impl crate::Specific for DPT_16_1 {
    const DPT: DPT = DPT::new(16u16, Some(1u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        let mut buffer = vec![0u8; (112u16 / 8) as usize];
        ISO_8859_1
            .encode_to(self.0.as_str(), EncoderTrap::Replace, &mut buffer)
            .unwrap();
        writer.write_bytes(&buffer).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_16_1({
                    let mut value = [0u8; (112u16 / 8) as usize];
                    reader.read_bytes(&mut value)?;
                    ISO_8859_1.decode(&value, DecoderTrap::Replace).unwrap()
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_16_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_16_x(pub String);
impl crate::Specific for DPT_16_x {
    const DPT: DPT = DPT::new(16u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        let mut buffer = vec![0u8; (112u16 / 8) as usize];
        ASCII.encode_to(self.0.as_str(), EncoderTrap::Replace, &mut buffer).unwrap();
        writer.write_bytes(&buffer).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_16_x({
                    let mut value = [0u8; (112u16 / 8) as usize];
                    reader.read_bytes(&mut value)?;
                    ASCII.decode(&value, DecoderTrap::Replace).unwrap()
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_16_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_17_1 {
    pub reserved: Reserved,
    pub Scenenumber: u8,
}
impl crate::Specific for DPT_17_1 {
    const DPT: DPT = DPT::new(17u16, Some(1u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(2u8 as u32, 0u32).unwrap();
        writer.write(6u8 as u32, self.Scenenumber).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_17_1 {
                reserved: {
                    reader.skip(2u8 as u32)?;
                    Reserved::new()
                },
                Scenenumber: reader.read(6u8 as u32)?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_17_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.reserved, self.Scenenumber)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_17_x {
    pub reserved: Reserved,
    pub Scenenumber: u8,
}
impl crate::Specific for DPT_17_x {
    const DPT: DPT = DPT::new(17u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(2u8 as u32, 0u32).unwrap();
        writer.write(6u8 as u32, self.Scenenumber).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_17_x {
                reserved: {
                    reader.skip(2u8 as u32)?;
                    Reserved::new()
                },
                Scenenumber: reader.read(6u8 as u32)?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_17_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.reserved, self.Scenenumber)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_18_1 {
    pub learnthescenecorrespondingtothefieldSceneNumber: bool,
    pub reserved: Reserved,
    pub Scenenumber: u8,
}
impl crate::Specific for DPT_18_1 {
    const DPT: DPT = DPT::new(18u16, Some(1u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_bit(self.learnthescenecorrespondingtothefieldSceneNumber).unwrap();
        writer.write(1u8 as u32, 0u32).unwrap();
        writer.write(6u8 as u32, self.Scenenumber).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_18_1 {
                learnthescenecorrespondingtothefieldSceneNumber: reader.read_bit()?,
                reserved: {
                    reader.skip(1u8 as u32)?;
                    Reserved::new()
                },
                Scenenumber: reader.read(6u8 as u32)?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_18_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}", self.learnthescenecorrespondingtothefieldSceneNumber, self
            .reserved, self.Scenenumber
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_18_x {
    pub learnthescenecorrespondingtothefieldSceneNumber: bool,
    pub reserved: Reserved,
    pub Scenenumber: u8,
}
impl crate::Specific for DPT_18_x {
    const DPT: DPT = DPT::new(18u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_bit(self.learnthescenecorrespondingtothefieldSceneNumber).unwrap();
        writer.write(1u8 as u32, 0u32).unwrap();
        writer.write(6u8 as u32, self.Scenenumber).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_18_x {
                learnthescenecorrespondingtothefieldSceneNumber: reader.read_bit()?,
                reserved: {
                    reader.skip(1u8 as u32)?;
                    Reserved::new()
                },
                Scenenumber: reader.read(6u8 as u32)?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_18_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}", self.learnthescenecorrespondingtothefieldSceneNumber, self
            .reserved, self.Scenenumber
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_19_1 {
    pub Year: u8,
    pub reserved: Reserved,
    pub Month: u8,
    pub reserved2: Reserved,
    pub DayOfMonth: u8,
    pub DayOfWeek: u8,
    pub HourOfDay: u8,
    pub reserved3: Reserved,
    pub Minutes: u8,
    pub reserved4: Reserved,
    pub Seconds: u8,
    pub Fault: bool,
    pub WorkingDay: bool,
    pub NoWD: bool,
    pub NoYear: bool,
    pub NoDate: bool,
    pub NoDayofWeek: bool,
    pub NoTime: bool,
    pub StandardSummerTime: bool,
    pub Qualityofclock: bool,
    pub reserved5: Reserved,
}
impl crate::Specific for DPT_19_1 {
    const DPT: DPT = DPT::new(19u16, Some(1u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.Year).unwrap();
        writer.write(4u8 as u32, 0u32).unwrap();
        writer.write(4u8 as u32, self.Month).unwrap();
        writer.write(3u8 as u32, 0u32).unwrap();
        writer.write(5u8 as u32, self.DayOfMonth).unwrap();
        writer.write(3u8 as u32, self.DayOfWeek).unwrap();
        writer.write(5u8 as u32, self.HourOfDay).unwrap();
        writer.write(2u8 as u32, 0u32).unwrap();
        writer.write(6u8 as u32, self.Minutes).unwrap();
        writer.write(2u8 as u32, 0u32).unwrap();
        writer.write(6u8 as u32, self.Seconds).unwrap();
        writer.write_bit(self.Fault).unwrap();
        writer.write_bit(self.WorkingDay).unwrap();
        writer.write_bit(self.NoWD).unwrap();
        writer.write_bit(self.NoYear).unwrap();
        writer.write_bit(self.NoDate).unwrap();
        writer.write_bit(self.NoDayofWeek).unwrap();
        writer.write_bit(self.NoTime).unwrap();
        writer.write_bit(self.StandardSummerTime).unwrap();
        writer.write_bit(self.Qualityofclock).unwrap();
        writer.write(7u8 as u32, 0u32).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_19_1 {
                Year: reader.read(8u8 as u32)?,
                reserved: {
                    reader.skip(4u8 as u32)?;
                    Reserved::new()
                },
                Month: reader.read(4u8 as u32)?,
                reserved2: {
                    reader.skip(3u8 as u32)?;
                    Reserved::new()
                },
                DayOfMonth: reader.read(5u8 as u32)?,
                DayOfWeek: reader.read(3u8 as u32)?,
                HourOfDay: reader.read(5u8 as u32)?,
                reserved3: {
                    reader.skip(2u8 as u32)?;
                    Reserved::new()
                },
                Minutes: reader.read(6u8 as u32)?,
                reserved4: {
                    reader.skip(2u8 as u32)?;
                    Reserved::new()
                },
                Seconds: reader.read(6u8 as u32)?,
                Fault: reader.read_bit()?,
                WorkingDay: reader.read_bit()?,
                NoWD: reader.read_bit()?,
                NoYear: reader.read_bit()?,
                NoDate: reader.read_bit()?,
                NoDayofWeek: reader.read_bit()?,
                NoTime: reader.read_bit()?,
                StandardSummerTime: reader.read_bit()?,
                Qualityofclock: reader.read_bit()?,
                reserved5: {
                    reader.skip(7u8 as u32)?;
                    Reserved::new()
                },
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_19_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}", self
            .Year, self.reserved, self.Month, self.reserved2, self.DayOfMonth, self
            .DayOfWeek, self.HourOfDay, self.reserved3, self.Minutes, self.reserved4,
            self.Seconds, self.Fault, self.WorkingDay, self.NoWD, self.NoYear, self
            .NoDate, self.NoDayofWeek, self.NoTime, self.StandardSummerTime, self
            .Qualityofclock, self.reserved5
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_19_x {
    pub Year: u8,
    pub reserved: Reserved,
    pub Month: u8,
    pub reserved2: Reserved,
    pub DayOfMonth: u8,
    pub DayOfWeek: u8,
    pub HourOfDay: u8,
    pub reserved3: Reserved,
    pub Minutes: u8,
    pub reserved4: Reserved,
    pub Seconds: u8,
    pub Fault: bool,
    pub WorkingDay: bool,
    pub NoWD: bool,
    pub NoYear: bool,
    pub NoDate: bool,
    pub NoDayofWeek: bool,
    pub NoTime: bool,
    pub StandardSummerTime: bool,
    pub Qualityofclock: bool,
    pub reserved5: Reserved,
}
impl crate::Specific for DPT_19_x {
    const DPT: DPT = DPT::new(19u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.Year).unwrap();
        writer.write(4u8 as u32, 0u32).unwrap();
        writer.write(4u8 as u32, self.Month).unwrap();
        writer.write(3u8 as u32, 0u32).unwrap();
        writer.write(5u8 as u32, self.DayOfMonth).unwrap();
        writer.write(3u8 as u32, self.DayOfWeek).unwrap();
        writer.write(5u8 as u32, self.HourOfDay).unwrap();
        writer.write(2u8 as u32, 0u32).unwrap();
        writer.write(6u8 as u32, self.Minutes).unwrap();
        writer.write(2u8 as u32, 0u32).unwrap();
        writer.write(6u8 as u32, self.Seconds).unwrap();
        writer.write_bit(self.Fault).unwrap();
        writer.write_bit(self.WorkingDay).unwrap();
        writer.write_bit(self.NoWD).unwrap();
        writer.write_bit(self.NoYear).unwrap();
        writer.write_bit(self.NoDate).unwrap();
        writer.write_bit(self.NoDayofWeek).unwrap();
        writer.write_bit(self.NoTime).unwrap();
        writer.write_bit(self.StandardSummerTime).unwrap();
        writer.write_bit(self.Qualityofclock).unwrap();
        writer.write(7u8 as u32, 0u32).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_19_x {
                Year: reader.read(8u8 as u32)?,
                reserved: {
                    reader.skip(4u8 as u32)?;
                    Reserved::new()
                },
                Month: reader.read(4u8 as u32)?,
                reserved2: {
                    reader.skip(3u8 as u32)?;
                    Reserved::new()
                },
                DayOfMonth: reader.read(5u8 as u32)?,
                DayOfWeek: reader.read(3u8 as u32)?,
                HourOfDay: reader.read(5u8 as u32)?,
                reserved3: {
                    reader.skip(2u8 as u32)?;
                    Reserved::new()
                },
                Minutes: reader.read(6u8 as u32)?,
                reserved4: {
                    reader.skip(2u8 as u32)?;
                    Reserved::new()
                },
                Seconds: reader.read(6u8 as u32)?,
                Fault: reader.read_bit()?,
                WorkingDay: reader.read_bit()?,
                NoWD: reader.read_bit()?,
                NoYear: reader.read_bit()?,
                NoDate: reader.read_bit()?,
                NoDayofWeek: reader.read_bit()?,
                NoTime: reader.read_bit()?,
                StandardSummerTime: reader.read_bit()?,
                Qualityofclock: reader.read_bit()?,
                reserved5: {
                    reader.skip(7u8 as u32)?;
                    Reserved::new()
                },
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_19_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}", self
            .Year, self.reserved, self.Month, self.reserved2, self.DayOfMonth, self
            .DayOfWeek, self.HourOfDay, self.reserved3, self.Minutes, self.reserved4,
            self.Seconds, self.Fault, self.WorkingDay, self.NoWD, self.NoYear, self
            .NoDate, self.NoDayofWeek, self.NoTime, self.StandardSummerTime, self
            .Qualityofclock, self.reserved5
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_1(pub u8);
impl crate::Specific for DPT_20_1 {
    const DPT: DPT = DPT::new(20u16, Some(1u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_1(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_2(pub u8);
impl crate::Specific for DPT_20_2 {
    const DPT: DPT = DPT::new(20u16, Some(2u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_2(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_3(pub u8);
impl crate::Specific for DPT_20_3 {
    const DPT: DPT = DPT::new(20u16, Some(3u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_3(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_4(pub u8);
impl crate::Specific for DPT_20_4 {
    const DPT: DPT = DPT::new(20u16, Some(4u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_4(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_4 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_5(pub u8);
impl crate::Specific for DPT_20_5 {
    const DPT: DPT = DPT::new(20u16, Some(5u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_5(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_5 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_6(pub u8);
impl crate::Specific for DPT_20_6 {
    const DPT: DPT = DPT::new(20u16, Some(6u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_6(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_6 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_7(pub u8);
impl crate::Specific for DPT_20_7 {
    const DPT: DPT = DPT::new(20u16, Some(7u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_7(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_7 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_8(pub u8);
impl crate::Specific for DPT_20_8 {
    const DPT: DPT = DPT::new(20u16, Some(8u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_8(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_8 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_11(pub u8);
impl crate::Specific for DPT_20_11 {
    const DPT: DPT = DPT::new(20u16, Some(11u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_11(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_11 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_12(pub u8);
impl crate::Specific for DPT_20_12 {
    const DPT: DPT = DPT::new(20u16, Some(12u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_12(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_12 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_13(pub u8);
impl crate::Specific for DPT_20_13 {
    const DPT: DPT = DPT::new(20u16, Some(13u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_13(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_13 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_14(pub u8);
impl crate::Specific for DPT_20_14 {
    const DPT: DPT = DPT::new(20u16, Some(14u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_14(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_14 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_17(pub u8);
impl crate::Specific for DPT_20_17 {
    const DPT: DPT = DPT::new(20u16, Some(17u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_17(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_17 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_20(pub u8);
impl crate::Specific for DPT_20_20 {
    const DPT: DPT = DPT::new(20u16, Some(20u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_20(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_20 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_21(pub u8);
impl crate::Specific for DPT_20_21 {
    const DPT: DPT = DPT::new(20u16, Some(21u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_21(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_21 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_22(pub u8);
impl crate::Specific for DPT_20_22 {
    const DPT: DPT = DPT::new(20u16, Some(22u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_22(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_22 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_100(pub u8);
impl crate::Specific for DPT_20_100 {
    const DPT: DPT = DPT::new(20u16, Some(100u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_100(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_100 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_101(pub u8);
impl crate::Specific for DPT_20_101 {
    const DPT: DPT = DPT::new(20u16, Some(101u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_101(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_101 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_102(pub u8);
impl crate::Specific for DPT_20_102 {
    const DPT: DPT = DPT::new(20u16, Some(102u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_102(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_102 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_103(pub u8);
impl crate::Specific for DPT_20_103 {
    const DPT: DPT = DPT::new(20u16, Some(103u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_103(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_103 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_104(pub u8);
impl crate::Specific for DPT_20_104 {
    const DPT: DPT = DPT::new(20u16, Some(104u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_104(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_104 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_105(pub u8);
impl crate::Specific for DPT_20_105 {
    const DPT: DPT = DPT::new(20u16, Some(105u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_105(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_105 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_106(pub u8);
impl crate::Specific for DPT_20_106 {
    const DPT: DPT = DPT::new(20u16, Some(106u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_106(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_106 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_107(pub u8);
impl crate::Specific for DPT_20_107 {
    const DPT: DPT = DPT::new(20u16, Some(107u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_107(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_107 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_108(pub u8);
impl crate::Specific for DPT_20_108 {
    const DPT: DPT = DPT::new(20u16, Some(108u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_108(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_108 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_109(pub u8);
impl crate::Specific for DPT_20_109 {
    const DPT: DPT = DPT::new(20u16, Some(109u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_109(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_109 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_110(pub u8);
impl crate::Specific for DPT_20_110 {
    const DPT: DPT = DPT::new(20u16, Some(110u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_110(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_110 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_111(pub u8);
impl crate::Specific for DPT_20_111 {
    const DPT: DPT = DPT::new(20u16, Some(111u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_111(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_111 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_112(pub u8);
impl crate::Specific for DPT_20_112 {
    const DPT: DPT = DPT::new(20u16, Some(112u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_112(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_112 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_113(pub u8);
impl crate::Specific for DPT_20_113 {
    const DPT: DPT = DPT::new(20u16, Some(113u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_113(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_113 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_114(pub u8);
impl crate::Specific for DPT_20_114 {
    const DPT: DPT = DPT::new(20u16, Some(114u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_114(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_114 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_115(pub u8);
impl crate::Specific for DPT_20_115 {
    const DPT: DPT = DPT::new(20u16, Some(115u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_115(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_115 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_116(pub u8);
impl crate::Specific for DPT_20_116 {
    const DPT: DPT = DPT::new(20u16, Some(116u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_116(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_116 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_120(pub u8);
impl crate::Specific for DPT_20_120 {
    const DPT: DPT = DPT::new(20u16, Some(120u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_120(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_120 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_121(pub u8);
impl crate::Specific for DPT_20_121 {
    const DPT: DPT = DPT::new(20u16, Some(121u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_121(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_121 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_122(pub u8);
impl crate::Specific for DPT_20_122 {
    const DPT: DPT = DPT::new(20u16, Some(122u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_122(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_122 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_600(pub u8);
impl crate::Specific for DPT_20_600 {
    const DPT: DPT = DPT::new(20u16, Some(600u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_600(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_600 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_601(pub u8);
impl crate::Specific for DPT_20_601 {
    const DPT: DPT = DPT::new(20u16, Some(601u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_601(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_601 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_602(pub u8);
impl crate::Specific for DPT_20_602 {
    const DPT: DPT = DPT::new(20u16, Some(602u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_602(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_602 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_603(pub u8);
impl crate::Specific for DPT_20_603 {
    const DPT: DPT = DPT::new(20u16, Some(603u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_603(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_603 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_604(pub u8);
impl crate::Specific for DPT_20_604 {
    const DPT: DPT = DPT::new(20u16, Some(604u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_604(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_604 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_605(pub u8);
impl crate::Specific for DPT_20_605 {
    const DPT: DPT = DPT::new(20u16, Some(605u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_605(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_605 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_606(pub u8);
impl crate::Specific for DPT_20_606 {
    const DPT: DPT = DPT::new(20u16, Some(606u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_606(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_606 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_607(pub u8);
impl crate::Specific for DPT_20_607 {
    const DPT: DPT = DPT::new(20u16, Some(607u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_607(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_607 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_608(pub u8);
impl crate::Specific for DPT_20_608 {
    const DPT: DPT = DPT::new(20u16, Some(608u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_608(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_608 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_609(pub u8);
impl crate::Specific for DPT_20_609 {
    const DPT: DPT = DPT::new(20u16, Some(609u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_609(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_609 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_610(pub u8);
impl crate::Specific for DPT_20_610 {
    const DPT: DPT = DPT::new(20u16, Some(610u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_610(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_610 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_611(pub u8);
impl crate::Specific for DPT_20_611 {
    const DPT: DPT = DPT::new(20u16, Some(611u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_611(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_611 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_612(pub u8);
impl crate::Specific for DPT_20_612 {
    const DPT: DPT = DPT::new(20u16, Some(612u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_612(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_612 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_801(pub u8);
impl crate::Specific for DPT_20_801 {
    const DPT: DPT = DPT::new(20u16, Some(801u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_801(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_801 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_802(pub u8);
impl crate::Specific for DPT_20_802 {
    const DPT: DPT = DPT::new(20u16, Some(802u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_802(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_802 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_803(pub u8);
impl crate::Specific for DPT_20_803 {
    const DPT: DPT = DPT::new(20u16, Some(803u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_803(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_803 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_804(pub u8);
impl crate::Specific for DPT_20_804 {
    const DPT: DPT = DPT::new(20u16, Some(804u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_804(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_804 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_1000(pub u8);
impl crate::Specific for DPT_20_1000 {
    const DPT: DPT = DPT::new(20u16, Some(1000u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_1000(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_1000 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_1001(pub u8);
impl crate::Specific for DPT_20_1001 {
    const DPT: DPT = DPT::new(20u16, Some(1001u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_1001(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_1001 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_1002(pub u8);
impl crate::Specific for DPT_20_1002 {
    const DPT: DPT = DPT::new(20u16, Some(1002u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_1002(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_1002 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_1003(pub u8);
impl crate::Specific for DPT_20_1003 {
    const DPT: DPT = DPT::new(20u16, Some(1003u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_1003(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_1003 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_20_x(pub u8);
impl crate::Specific for DPT_20_x {
    const DPT: DPT = DPT::new(20u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_20_x(reader.read(8u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_20_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_21_1 {
    pub reserved: Reserved,
    pub alarmstatusofcorrespondingDatapointisnotacknowledged: bool,
    pub correspondingDatapointisinalarm: bool,
    pub correspondingDatapointMainvalueisoverridden: bool,
    pub correspondingDatapointMainvalueiscorruptedduetofailure: bool,
    pub correspondingDatapointvalueisoutofservice: bool,
}
impl crate::Specific for DPT_21_1 {
    const DPT: DPT = DPT::new(21u16, Some(1u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(3u8 as u32, 0u32).unwrap();
        writer
            .write_bit(self.alarmstatusofcorrespondingDatapointisnotacknowledged)
            .unwrap();
        writer.write_bit(self.correspondingDatapointisinalarm).unwrap();
        writer.write_bit(self.correspondingDatapointMainvalueisoverridden).unwrap();
        writer
            .write_bit(self.correspondingDatapointMainvalueiscorruptedduetofailure)
            .unwrap();
        writer.write_bit(self.correspondingDatapointvalueisoutofservice).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_21_1 {
                reserved: {
                    reader.skip(3u8 as u32)?;
                    Reserved::new()
                },
                alarmstatusofcorrespondingDatapointisnotacknowledged: reader.read_bit()?,
                correspondingDatapointisinalarm: reader.read_bit()?,
                correspondingDatapointMainvalueisoverridden: reader.read_bit()?,
                correspondingDatapointMainvalueiscorruptedduetofailure: reader
                    .read_bit()?,
                correspondingDatapointvalueisoutofservice: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_21_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}", self.reserved, self
            .alarmstatusofcorrespondingDatapointisnotacknowledged, self
            .correspondingDatapointisinalarm, self
            .correspondingDatapointMainvalueisoverridden, self
            .correspondingDatapointMainvalueiscorruptedduetofailure, self
            .correspondingDatapointvalueisoutofservice
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_21_2 {
    pub reserved: Reserved,
    pub VerifyModeison: bool,
    pub AdatagramwiththeownIndividualAddressasSourceAddresshasbeenreceived: bool,
    pub Theuserapplicationisstopped: bool,
}
impl crate::Specific for DPT_21_2 {
    const DPT: DPT = DPT::new(21u16, Some(2u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(5u8 as u32, 0u32).unwrap();
        writer.write_bit(self.VerifyModeison).unwrap();
        writer
            .write_bit(
                self.AdatagramwiththeownIndividualAddressasSourceAddresshasbeenreceived,
            )
            .unwrap();
        writer.write_bit(self.Theuserapplicationisstopped).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_21_2 {
                reserved: {
                    reader.skip(5u8 as u32)?;
                    Reserved::new()
                },
                VerifyModeison: reader.read_bit()?,
                AdatagramwiththeownIndividualAddressasSourceAddresshasbeenreceived: reader
                    .read_bit()?,
                Theuserapplicationisstopped: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_21_2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}", self.reserved, self.VerifyModeison, self
            .AdatagramwiththeownIndividualAddressasSourceAddresshasbeenreceived, self
            .Theuserapplicationisstopped
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_21_100 {
    pub RoomHMax: bool,
    pub RoomHConf: bool,
    pub DHWLegio: bool,
    pub DHWNorm: bool,
    pub Overrun: bool,
    pub Oversupply: bool,
    pub Protection: bool,
    pub ForceRequest: bool,
}
impl crate::Specific for DPT_21_100 {
    const DPT: DPT = DPT::new(21u16, Some(100u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_bit(self.RoomHMax).unwrap();
        writer.write_bit(self.RoomHConf).unwrap();
        writer.write_bit(self.DHWLegio).unwrap();
        writer.write_bit(self.DHWNorm).unwrap();
        writer.write_bit(self.Overrun).unwrap();
        writer.write_bit(self.Oversupply).unwrap();
        writer.write_bit(self.Protection).unwrap();
        writer.write_bit(self.ForceRequest).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_21_100 {
                RoomHMax: reader.read_bit()?,
                RoomHConf: reader.read_bit()?,
                DHWLegio: reader.read_bit()?,
                DHWNorm: reader.read_bit()?,
                Overrun: reader.read_bit()?,
                Oversupply: reader.read_bit()?,
                Protection: reader.read_bit()?,
                ForceRequest: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_21_100 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}/{}/{}", self.RoomHMax, self.RoomHConf, self.DHWLegio,
            self.DHWNorm, self.Overrun, self.Oversupply, self.Protection, self
            .ForceRequest
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_21_101 {
    pub reserved: Reserved,
    pub ForceRequest: bool,
}
impl crate::Specific for DPT_21_101 {
    const DPT: DPT = DPT::new(21u16, Some(101u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(7u8 as u32, 0u32).unwrap();
        writer.write_bit(self.ForceRequest).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_21_101 {
                reserved: {
                    reader.skip(7u8 as u32)?;
                    Reserved::new()
                },
                ForceRequest: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_21_101 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.reserved, self.ForceRequest)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_21_102 {
    pub SummerMode: bool,
    pub StatusStopOptim: bool,
    pub StatusStartOptim: bool,
    pub StatusMorningBoost: bool,
    pub TempReturnLimit: bool,
    pub TempFlowLimit: bool,
    pub SatusECO: bool,
    pub Fault: bool,
}
impl crate::Specific for DPT_21_102 {
    const DPT: DPT = DPT::new(21u16, Some(102u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_bit(self.SummerMode).unwrap();
        writer.write_bit(self.StatusStopOptim).unwrap();
        writer.write_bit(self.StatusStartOptim).unwrap();
        writer.write_bit(self.StatusMorningBoost).unwrap();
        writer.write_bit(self.TempReturnLimit).unwrap();
        writer.write_bit(self.TempFlowLimit).unwrap();
        writer.write_bit(self.SatusECO).unwrap();
        writer.write_bit(self.Fault).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_21_102 {
                SummerMode: reader.read_bit()?,
                StatusStopOptim: reader.read_bit()?,
                StatusStartOptim: reader.read_bit()?,
                StatusMorningBoost: reader.read_bit()?,
                TempReturnLimit: reader.read_bit()?,
                TempFlowLimit: reader.read_bit()?,
                SatusECO: reader.read_bit()?,
                Fault: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_21_102 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}/{}/{}", self.SummerMode, self.StatusStopOptim, self
            .StatusStartOptim, self.StatusMorningBoost, self.TempReturnLimit, self
            .TempFlowLimit, self.SatusECO, self.Fault
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_21_103 {
    pub reserved: Reserved,
    pub SolarLoadSufficient: bool,
    pub SDHWLoadActive: bool,
    pub Fault: bool,
}
impl crate::Specific for DPT_21_103 {
    const DPT: DPT = DPT::new(21u16, Some(103u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(5u8 as u32, 0u32).unwrap();
        writer.write_bit(self.SolarLoadSufficient).unwrap();
        writer.write_bit(self.SDHWLoadActive).unwrap();
        writer.write_bit(self.Fault).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_21_103 {
                reserved: {
                    reader.skip(5u8 as u32)?;
                    Reserved::new()
                },
                SolarLoadSufficient: reader.read_bit()?,
                SDHWLoadActive: reader.read_bit()?,
                Fault: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_21_103 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}", self.reserved, self.SolarLoadSufficient, self
            .SDHWLoadActive, self.Fault
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_21_104 {
    pub reserved: Reserved,
    pub SolidState: bool,
    pub Gas: bool,
    pub Oil: bool,
}
impl crate::Specific for DPT_21_104 {
    const DPT: DPT = DPT::new(21u16, Some(104u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(5u8 as u32, 0u32).unwrap();
        writer.write_bit(self.SolidState).unwrap();
        writer.write_bit(self.Gas).unwrap();
        writer.write_bit(self.Oil).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_21_104 {
                reserved: {
                    reader.skip(5u8 as u32)?;
                    Reserved::new()
                },
                SolidState: reader.read_bit()?,
                Gas: reader.read_bit()?,
                Oil: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_21_104 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}/{}/{}", self.reserved, self.SolidState, self.Gas, self.Oil)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_21_105 {
    pub reserved: Reserved,
    pub Fault: bool,
}
impl crate::Specific for DPT_21_105 {
    const DPT: DPT = DPT::new(21u16, Some(105u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(7u8 as u32, 0u32).unwrap();
        writer.write_bit(self.Fault).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_21_105 {
                reserved: {
                    reader.skip(7u8 as u32)?;
                    Reserved::new()
                },
                Fault: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_21_105 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.reserved, self.Fault)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_21_106 {
    pub reserved: Reserved,
    pub Cool: bool,
    pub Heat: bool,
    pub FanActive: bool,
    pub Fault: bool,
}
impl crate::Specific for DPT_21_106 {
    const DPT: DPT = DPT::new(21u16, Some(106u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(4u8 as u32, 0u32).unwrap();
        writer.write_bit(self.Cool).unwrap();
        writer.write_bit(self.Heat).unwrap();
        writer.write_bit(self.FanActive).unwrap();
        writer.write_bit(self.Fault).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_21_106 {
                reserved: {
                    reader.skip(4u8 as u32)?;
                    Reserved::new()
                },
                Cool: reader.read_bit()?,
                Heat: reader.read_bit()?,
                FanActive: reader.read_bit()?,
                Fault: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_21_106 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}", self.reserved, self.Cool, self.Heat, self.FanActive,
            self.Fault
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_21_107 {
    pub reserved: Reserved,
    pub StatusofHVACModeUser: bool,
    pub StatusofcomfortprolongationUser: bool,
    pub Effectivevalueofthecomfortpushbutton: bool,
    pub Effectivevalueofthepresencestatus: bool,
    pub Effectivevalueofthewindowstatus: bool,
}
impl crate::Specific for DPT_21_107 {
    const DPT: DPT = DPT::new(21u16, Some(107u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(3u8 as u32, 0u32).unwrap();
        writer.write_bit(self.StatusofHVACModeUser).unwrap();
        writer.write_bit(self.StatusofcomfortprolongationUser).unwrap();
        writer.write_bit(self.Effectivevalueofthecomfortpushbutton).unwrap();
        writer.write_bit(self.Effectivevalueofthepresencestatus).unwrap();
        writer.write_bit(self.Effectivevalueofthewindowstatus).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_21_107 {
                reserved: {
                    reader.skip(3u8 as u32)?;
                    Reserved::new()
                },
                StatusofHVACModeUser: reader.read_bit()?,
                StatusofcomfortprolongationUser: reader.read_bit()?,
                Effectivevalueofthecomfortpushbutton: reader.read_bit()?,
                Effectivevalueofthepresencestatus: reader.read_bit()?,
                Effectivevalueofthewindowstatus: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_21_107 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}", self.reserved, self.StatusofHVACModeUser, self
            .StatusofcomfortprolongationUser, self.Effectivevalueofthecomfortpushbutton,
            self.Effectivevalueofthepresencestatus, self.Effectivevalueofthewindowstatus
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_21_601 {
    pub reserved: Reserved,
    pub Overheat: bool,
    pub LampFailure: bool,
    pub DefectiveLoad: bool,
    pub Underload: bool,
    pub Overcurrent: bool,
    pub Undervoltage: bool,
    pub LoadDetectionError: bool,
}
impl crate::Specific for DPT_21_601 {
    const DPT: DPT = DPT::new(21u16, Some(601u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(1u8 as u32, 0u32).unwrap();
        writer.write_bit(self.Overheat).unwrap();
        writer.write_bit(self.LampFailure).unwrap();
        writer.write_bit(self.DefectiveLoad).unwrap();
        writer.write_bit(self.Underload).unwrap();
        writer.write_bit(self.Overcurrent).unwrap();
        writer.write_bit(self.Undervoltage).unwrap();
        writer.write_bit(self.LoadDetectionError).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_21_601 {
                reserved: {
                    reader.skip(1u8 as u32)?;
                    Reserved::new()
                },
                Overheat: reader.read_bit()?,
                LampFailure: reader.read_bit()?,
                DefectiveLoad: reader.read_bit()?,
                Underload: reader.read_bit()?,
                Overcurrent: reader.read_bit()?,
                Undervoltage: reader.read_bit()?,
                LoadDetectionError: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_21_601 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}/{}/{}", self.reserved, self.Overheat, self.LampFailure,
            self.DefectiveLoad, self.Underload, self.Overcurrent, self.Undervoltage, self
            .LoadDetectionError
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_21_1000 {
    pub reserved: Reserved,
    pub BiBatSlave: bool,
    pub BiBatMaster: bool,
    pub Asynchronous: bool,
}
impl crate::Specific for DPT_21_1000 {
    const DPT: DPT = DPT::new(21u16, Some(1000u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(5u8 as u32, 0u32).unwrap();
        writer.write_bit(self.BiBatSlave).unwrap();
        writer.write_bit(self.BiBatMaster).unwrap();
        writer.write_bit(self.Asynchronous).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_21_1000 {
                reserved: {
                    reader.skip(5u8 as u32)?;
                    Reserved::new()
                },
                BiBatSlave: reader.read_bit()?,
                BiBatMaster: reader.read_bit()?,
                Asynchronous: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_21_1000 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}", self.reserved, self.BiBatSlave, self.BiBatMaster, self
            .Asynchronous
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_21_1001 {
    pub reserved: Reserved,
    pub DoA: bool,
    pub KNXSN: bool,
    pub DoAandKNXSN: bool,
}
impl crate::Specific for DPT_21_1001 {
    const DPT: DPT = DPT::new(21u16, Some(1001u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(5u8 as u32, 0u32).unwrap();
        writer.write_bit(self.DoA).unwrap();
        writer.write_bit(self.KNXSN).unwrap();
        writer.write_bit(self.DoAandKNXSN).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_21_1001 {
                reserved: {
                    reader.skip(5u8 as u32)?;
                    Reserved::new()
                },
                DoA: reader.read_bit()?,
                KNXSN: reader.read_bit()?,
                DoAandKNXSN: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_21_1001 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}/{}/{}", self.reserved, self.DoA, self.KNXSN, self.DoAandKNXSN)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_21_1010 {
    pub Activationstateofchannel: bool,
    pub Activationstateofchannel2: bool,
    pub Activationstateofchannel3: bool,
    pub Activationstateofchannel4: bool,
    pub Activationstateofchannel5: bool,
    pub Activationstateofchannel6: bool,
    pub Activationstateofchannel7: bool,
    pub Activationstateofchannel8: bool,
}
impl crate::Specific for DPT_21_1010 {
    const DPT: DPT = DPT::new(21u16, Some(1010u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_bit(self.Activationstateofchannel).unwrap();
        writer.write_bit(self.Activationstateofchannel2).unwrap();
        writer.write_bit(self.Activationstateofchannel3).unwrap();
        writer.write_bit(self.Activationstateofchannel4).unwrap();
        writer.write_bit(self.Activationstateofchannel5).unwrap();
        writer.write_bit(self.Activationstateofchannel6).unwrap();
        writer.write_bit(self.Activationstateofchannel7).unwrap();
        writer.write_bit(self.Activationstateofchannel8).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_21_1010 {
                Activationstateofchannel: reader.read_bit()?,
                Activationstateofchannel2: reader.read_bit()?,
                Activationstateofchannel3: reader.read_bit()?,
                Activationstateofchannel4: reader.read_bit()?,
                Activationstateofchannel5: reader.read_bit()?,
                Activationstateofchannel6: reader.read_bit()?,
                Activationstateofchannel7: reader.read_bit()?,
                Activationstateofchannel8: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_21_1010 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}/{}/{}", self.Activationstateofchannel, self
            .Activationstateofchannel2, self.Activationstateofchannel3, self
            .Activationstateofchannel4, self.Activationstateofchannel5, self
            .Activationstateofchannel6, self.Activationstateofchannel7, self
            .Activationstateofchannel8
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_21_x {
    pub reserved: Reserved,
    pub alarmstatusofcorrespondingDatapointisnotacknowledged: bool,
    pub correspondingDatapointisinalarm: bool,
    pub correspondingDatapointMainvalueisoverridden: bool,
    pub correspondingDatapointMainvalueiscorruptedduetofailure: bool,
    pub correspondingDatapointvalueisoutofservice: bool,
}
impl crate::Specific for DPT_21_x {
    const DPT: DPT = DPT::new(21u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(3u8 as u32, 0u32).unwrap();
        writer
            .write_bit(self.alarmstatusofcorrespondingDatapointisnotacknowledged)
            .unwrap();
        writer.write_bit(self.correspondingDatapointisinalarm).unwrap();
        writer.write_bit(self.correspondingDatapointMainvalueisoverridden).unwrap();
        writer
            .write_bit(self.correspondingDatapointMainvalueiscorruptedduetofailure)
            .unwrap();
        writer.write_bit(self.correspondingDatapointvalueisoutofservice).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_21_x {
                reserved: {
                    reader.skip(3u8 as u32)?;
                    Reserved::new()
                },
                alarmstatusofcorrespondingDatapointisnotacknowledged: reader.read_bit()?,
                correspondingDatapointisinalarm: reader.read_bit()?,
                correspondingDatapointMainvalueisoverridden: reader.read_bit()?,
                correspondingDatapointMainvalueiscorruptedduetofailure: reader
                    .read_bit()?,
                correspondingDatapointvalueisoutofservice: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_21_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}", self.reserved, self
            .alarmstatusofcorrespondingDatapointisnotacknowledged, self
            .correspondingDatapointisinalarm, self
            .correspondingDatapointMainvalueisoverridden, self
            .correspondingDatapointMainvalueiscorruptedduetofailure, self
            .correspondingDatapointvalueisoutofservice
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_22_100 {
    pub reserved: Reserved,
    pub TempOptimShiftActive: bool,
    pub SolarEnergySupport: bool,
    pub SolarEnergyOnly: bool,
    pub OtherEnergySourceActive: bool,
    pub DHWPushActive: bool,
    pub LegioProtActive: bool,
    pub DHWLoadActive: bool,
    pub Fault: bool,
}
impl crate::Specific for DPT_22_100 {
    const DPT: DPT = DPT::new(22u16, Some(100u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, 0u32).unwrap();
        writer.write_bit(self.TempOptimShiftActive).unwrap();
        writer.write_bit(self.SolarEnergySupport).unwrap();
        writer.write_bit(self.SolarEnergyOnly).unwrap();
        writer.write_bit(self.OtherEnergySourceActive).unwrap();
        writer.write_bit(self.DHWPushActive).unwrap();
        writer.write_bit(self.LegioProtActive).unwrap();
        writer.write_bit(self.DHWLoadActive).unwrap();
        writer.write_bit(self.Fault).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_22_100 {
                reserved: {
                    reader.skip(8u8 as u32)?;
                    Reserved::new()
                },
                TempOptimShiftActive: reader.read_bit()?,
                SolarEnergySupport: reader.read_bit()?,
                SolarEnergyOnly: reader.read_bit()?,
                OtherEnergySourceActive: reader.read_bit()?,
                DHWPushActive: reader.read_bit()?,
                LegioProtActive: reader.read_bit()?,
                DHWLoadActive: reader.read_bit()?,
                Fault: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_22_100 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}/{}/{}/{}", self.reserved, self.TempOptimShiftActive,
            self.SolarEnergySupport, self.SolarEnergyOnly, self.OtherEnergySourceActive,
            self.DHWPushActive, self.LegioProtActive, self.DHWLoadActive, self.Fault
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_22_101 {
    pub reserved: Reserved,
    pub OverheatAlarm: bool,
    pub FrostAlarm: bool,
    pub DewPointStatus: bool,
    pub CoolingDisabled: bool,
    pub StatusPreCool: bool,
    pub StatusEcoC: bool,
    pub HeatCoolMode: bool,
    pub HeatingDiabled: bool,
    pub StatusStopOptim: bool,
    pub StatusStartOptim: bool,
    pub StatusMorningBoostH: bool,
    pub TempFlowReturnLimit: bool,
    pub TempFlowLimit: bool,
    pub StatusEcoH: bool,
    pub Fault: bool,
}
impl crate::Specific for DPT_22_101 {
    const DPT: DPT = DPT::new(22u16, Some(101u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(1u8 as u32, 0u32).unwrap();
        writer.write_bit(self.OverheatAlarm).unwrap();
        writer.write_bit(self.FrostAlarm).unwrap();
        writer.write_bit(self.DewPointStatus).unwrap();
        writer.write_bit(self.CoolingDisabled).unwrap();
        writer.write_bit(self.StatusPreCool).unwrap();
        writer.write_bit(self.StatusEcoC).unwrap();
        writer.write_bit(self.HeatCoolMode).unwrap();
        writer.write_bit(self.HeatingDiabled).unwrap();
        writer.write_bit(self.StatusStopOptim).unwrap();
        writer.write_bit(self.StatusStartOptim).unwrap();
        writer.write_bit(self.StatusMorningBoostH).unwrap();
        writer.write_bit(self.TempFlowReturnLimit).unwrap();
        writer.write_bit(self.TempFlowLimit).unwrap();
        writer.write_bit(self.StatusEcoH).unwrap();
        writer.write_bit(self.Fault).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_22_101 {
                reserved: {
                    reader.skip(1u8 as u32)?;
                    Reserved::new()
                },
                OverheatAlarm: reader.read_bit()?,
                FrostAlarm: reader.read_bit()?,
                DewPointStatus: reader.read_bit()?,
                CoolingDisabled: reader.read_bit()?,
                StatusPreCool: reader.read_bit()?,
                StatusEcoC: reader.read_bit()?,
                HeatCoolMode: reader.read_bit()?,
                HeatingDiabled: reader.read_bit()?,
                StatusStopOptim: reader.read_bit()?,
                StatusStartOptim: reader.read_bit()?,
                StatusMorningBoostH: reader.read_bit()?,
                TempFlowReturnLimit: reader.read_bit()?,
                TempFlowLimit: reader.read_bit()?,
                StatusEcoH: reader.read_bit()?,
                Fault: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_22_101 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}", self.reserved, self
            .OverheatAlarm, self.FrostAlarm, self.DewPointStatus, self.CoolingDisabled,
            self.StatusPreCool, self.StatusEcoC, self.HeatCoolMode, self.HeatingDiabled,
            self.StatusStopOptim, self.StatusStartOptim, self.StatusMorningBoostH, self
            .TempFlowReturnLimit, self.TempFlowLimit, self.StatusEcoH, self.Fault
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_22_102 {
    pub reserved: Reserved,
    pub CalibrationMode: bool,
    pub LockedPosition: bool,
    pub ForcedPosition: bool,
    pub Manuaoperationoverridden: bool,
    pub Servicemode: bool,
    pub Valvekick: bool,
    pub Overload: bool,
    pub ShortCircuit: bool,
    pub CurrentValveposition: bool,
}
impl crate::Specific for DPT_22_102 {
    const DPT: DPT = DPT::new(22u16, Some(102u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(7u8 as u32, 0u32).unwrap();
        writer.write_bit(self.CalibrationMode).unwrap();
        writer.write_bit(self.LockedPosition).unwrap();
        writer.write_bit(self.ForcedPosition).unwrap();
        writer.write_bit(self.Manuaoperationoverridden).unwrap();
        writer.write_bit(self.Servicemode).unwrap();
        writer.write_bit(self.Valvekick).unwrap();
        writer.write_bit(self.Overload).unwrap();
        writer.write_bit(self.ShortCircuit).unwrap();
        writer.write_bit(self.CurrentValveposition).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_22_102 {
                reserved: {
                    reader.skip(7u8 as u32)?;
                    Reserved::new()
                },
                CalibrationMode: reader.read_bit()?,
                LockedPosition: reader.read_bit()?,
                ForcedPosition: reader.read_bit()?,
                Manuaoperationoverridden: reader.read_bit()?,
                Servicemode: reader.read_bit()?,
                Valvekick: reader.read_bit()?,
                Overload: reader.read_bit()?,
                ShortCircuit: reader.read_bit()?,
                CurrentValveposition: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_22_102 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}/{}/{}/{}/{}", self.reserved, self.CalibrationMode, self
            .LockedPosition, self.ForcedPosition, self.Manuaoperationoverridden, self
            .Servicemode, self.Valvekick, self.Overload, self.ShortCircuit, self
            .CurrentValveposition
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_22_103 {
    pub reserved: Reserved,
    pub CoolingModeEnabled: bool,
    pub HeatingModeEnabled: bool,
    pub AdditionalheatingcoolingstageStage: bool,
    pub Controllerinactive: bool,
    pub OverheatAlarm: bool,
    pub FrostAlarm: bool,
    pub DewPointStatus: bool,
    pub ActiveMode: bool,
    pub Generalfailureinformation: bool,
}
impl crate::Specific for DPT_22_103 {
    const DPT: DPT = DPT::new(22u16, Some(103u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(7u8 as u32, 0u32).unwrap();
        writer.write_bit(self.CoolingModeEnabled).unwrap();
        writer.write_bit(self.HeatingModeEnabled).unwrap();
        writer.write_bit(self.AdditionalheatingcoolingstageStage).unwrap();
        writer.write_bit(self.Controllerinactive).unwrap();
        writer.write_bit(self.OverheatAlarm).unwrap();
        writer.write_bit(self.FrostAlarm).unwrap();
        writer.write_bit(self.DewPointStatus).unwrap();
        writer.write_bit(self.ActiveMode).unwrap();
        writer.write_bit(self.Generalfailureinformation).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_22_103 {
                reserved: {
                    reader.skip(7u8 as u32)?;
                    Reserved::new()
                },
                CoolingModeEnabled: reader.read_bit()?,
                HeatingModeEnabled: reader.read_bit()?,
                AdditionalheatingcoolingstageStage: reader.read_bit()?,
                Controllerinactive: reader.read_bit()?,
                OverheatAlarm: reader.read_bit()?,
                FrostAlarm: reader.read_bit()?,
                DewPointStatus: reader.read_bit()?,
                ActiveMode: reader.read_bit()?,
                Generalfailureinformation: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_22_103 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}/{}/{}/{}/{}", self.reserved, self.CoolingModeEnabled,
            self.HeatingModeEnabled, self.AdditionalheatingcoolingstageStage, self
            .Controllerinactive, self.OverheatAlarm, self.FrostAlarm, self
            .DewPointStatus, self.ActiveMode, self.Generalfailureinformation
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_22_1000 {
    pub reserved: Reserved,
    pub KNXIP: bool,
    pub RF: bool,
    pub reserved2: Reserved,
    pub PL: bool,
    pub TP: bool,
    pub reserved3: Reserved,
}
impl crate::Specific for DPT_22_1000 {
    const DPT: DPT = DPT::new(22u16, Some(1000u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(10u8 as u32, 0u32).unwrap();
        writer.write_bit(self.KNXIP).unwrap();
        writer.write_bit(self.RF).unwrap();
        writer.write(1u8 as u32, 0u32).unwrap();
        writer.write_bit(self.PL).unwrap();
        writer.write_bit(self.TP).unwrap();
        writer.write(1u8 as u32, 0u32).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_22_1000 {
                reserved: {
                    reader.skip(10u8 as u32)?;
                    Reserved::new()
                },
                KNXIP: reader.read_bit()?,
                RF: reader.read_bit()?,
                reserved2: {
                    reader.skip(1u8 as u32)?;
                    Reserved::new()
                },
                PL: reader.read_bit()?,
                TP: reader.read_bit()?,
                reserved3: {
                    reader.skip(1u8 as u32)?;
                    Reserved::new()
                },
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_22_1000 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}/{}", self.reserved, self.KNXIP, self.RF, self
            .reserved2, self.PL, self.TP, self.reserved3
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_22_1010 {
    pub Activationstateofchannel: bool,
    pub Activationstateofchannel2: bool,
    pub Activationstateofchannel3: bool,
    pub Activationstateofchannel4: bool,
    pub Activationstateofchannel5: bool,
    pub Activationstateofchannel6: bool,
    pub Activationstateofchannel7: bool,
    pub Activationstateofchannel8: bool,
    pub Activationstateofchannel9: bool,
    pub Activationstateofchannel10: bool,
    pub Activationstateofchannel11: bool,
    pub Activationstateofchannel12: bool,
    pub Activationstateofchannel13: bool,
    pub Activationstateofchannel14: bool,
    pub Activationstateofchannel15: bool,
    pub Activationstateofchannel16: bool,
}
impl crate::Specific for DPT_22_1010 {
    const DPT: DPT = DPT::new(22u16, Some(1010u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_bit(self.Activationstateofchannel).unwrap();
        writer.write_bit(self.Activationstateofchannel2).unwrap();
        writer.write_bit(self.Activationstateofchannel3).unwrap();
        writer.write_bit(self.Activationstateofchannel4).unwrap();
        writer.write_bit(self.Activationstateofchannel5).unwrap();
        writer.write_bit(self.Activationstateofchannel6).unwrap();
        writer.write_bit(self.Activationstateofchannel7).unwrap();
        writer.write_bit(self.Activationstateofchannel8).unwrap();
        writer.write_bit(self.Activationstateofchannel9).unwrap();
        writer.write_bit(self.Activationstateofchannel10).unwrap();
        writer.write_bit(self.Activationstateofchannel11).unwrap();
        writer.write_bit(self.Activationstateofchannel12).unwrap();
        writer.write_bit(self.Activationstateofchannel13).unwrap();
        writer.write_bit(self.Activationstateofchannel14).unwrap();
        writer.write_bit(self.Activationstateofchannel15).unwrap();
        writer.write_bit(self.Activationstateofchannel16).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_22_1010 {
                Activationstateofchannel: reader.read_bit()?,
                Activationstateofchannel2: reader.read_bit()?,
                Activationstateofchannel3: reader.read_bit()?,
                Activationstateofchannel4: reader.read_bit()?,
                Activationstateofchannel5: reader.read_bit()?,
                Activationstateofchannel6: reader.read_bit()?,
                Activationstateofchannel7: reader.read_bit()?,
                Activationstateofchannel8: reader.read_bit()?,
                Activationstateofchannel9: reader.read_bit()?,
                Activationstateofchannel10: reader.read_bit()?,
                Activationstateofchannel11: reader.read_bit()?,
                Activationstateofchannel12: reader.read_bit()?,
                Activationstateofchannel13: reader.read_bit()?,
                Activationstateofchannel14: reader.read_bit()?,
                Activationstateofchannel15: reader.read_bit()?,
                Activationstateofchannel16: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_22_1010 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}", self
            .Activationstateofchannel, self.Activationstateofchannel2, self
            .Activationstateofchannel3, self.Activationstateofchannel4, self
            .Activationstateofchannel5, self.Activationstateofchannel6, self
            .Activationstateofchannel7, self.Activationstateofchannel8, self
            .Activationstateofchannel9, self.Activationstateofchannel10, self
            .Activationstateofchannel11, self.Activationstateofchannel12, self
            .Activationstateofchannel13, self.Activationstateofchannel14, self
            .Activationstateofchannel15, self.Activationstateofchannel16
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_22_x {
    pub reserved: Reserved,
    pub TempOptimShiftActive: bool,
    pub SolarEnergySupport: bool,
    pub SolarEnergyOnly: bool,
    pub OtherEnergySourceActive: bool,
    pub DHWPushActive: bool,
    pub LegioProtActive: bool,
    pub DHWLoadActive: bool,
    pub Fault: bool,
}
impl crate::Specific for DPT_22_x {
    const DPT: DPT = DPT::new(22u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, 0u32).unwrap();
        writer.write_bit(self.TempOptimShiftActive).unwrap();
        writer.write_bit(self.SolarEnergySupport).unwrap();
        writer.write_bit(self.SolarEnergyOnly).unwrap();
        writer.write_bit(self.OtherEnergySourceActive).unwrap();
        writer.write_bit(self.DHWPushActive).unwrap();
        writer.write_bit(self.LegioProtActive).unwrap();
        writer.write_bit(self.DHWLoadActive).unwrap();
        writer.write_bit(self.Fault).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_22_x {
                reserved: {
                    reader.skip(8u8 as u32)?;
                    Reserved::new()
                },
                TempOptimShiftActive: reader.read_bit()?,
                SolarEnergySupport: reader.read_bit()?,
                SolarEnergyOnly: reader.read_bit()?,
                OtherEnergySourceActive: reader.read_bit()?,
                DHWPushActive: reader.read_bit()?,
                LegioProtActive: reader.read_bit()?,
                DHWLoadActive: reader.read_bit()?,
                Fault: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_22_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}/{}/{}/{}", self.reserved, self.TempOptimShiftActive,
            self.SolarEnergySupport, self.SolarEnergyOnly, self.OtherEnergySourceActive,
            self.DHWPushActive, self.LegioProtActive, self.DHWLoadActive, self.Fault
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_23_1(pub u8);
impl crate::Specific for DPT_23_1 {
    const DPT: DPT = DPT::new(23u16, Some(1u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = [0u8];
        let mut writer = BitWriter::endian(bytes.as_mut_slice(), BigEndian);
        writer.write(2u8 as u32, self.0).unwrap();
        DataPoint::Short(writer.into_unwritten().1)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Short(byte) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new([*byte]));
            reader.seek_bits(SeekFrom::End(2u16 as i64)).unwrap();
            Ok(DPT_23_1(reader.read(2u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_23_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_23_2(pub u8);
impl crate::Specific for DPT_23_2 {
    const DPT: DPT = DPT::new(23u16, Some(2u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = [0u8];
        let mut writer = BitWriter::endian(bytes.as_mut_slice(), BigEndian);
        writer.write(2u8 as u32, self.0).unwrap();
        DataPoint::Short(writer.into_unwritten().1)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Short(byte) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new([*byte]));
            reader.seek_bits(SeekFrom::End(2u16 as i64)).unwrap();
            Ok(DPT_23_2(reader.read(2u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_23_2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_23_3(pub u8);
impl crate::Specific for DPT_23_3 {
    const DPT: DPT = DPT::new(23u16, Some(3u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = [0u8];
        let mut writer = BitWriter::endian(bytes.as_mut_slice(), BigEndian);
        writer.write(2u8 as u32, self.0).unwrap();
        DataPoint::Short(writer.into_unwritten().1)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Short(byte) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new([*byte]));
            reader.seek_bits(SeekFrom::End(2u16 as i64)).unwrap();
            Ok(DPT_23_3(reader.read(2u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_23_3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_23_102(pub u8);
impl crate::Specific for DPT_23_102 {
    const DPT: DPT = DPT::new(23u16, Some(102u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = [0u8];
        let mut writer = BitWriter::endian(bytes.as_mut_slice(), BigEndian);
        writer.write(2u8 as u32, self.0).unwrap();
        DataPoint::Short(writer.into_unwritten().1)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Short(byte) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new([*byte]));
            reader.seek_bits(SeekFrom::End(2u16 as i64)).unwrap();
            Ok(DPT_23_102(reader.read(2u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_23_102 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_23_x(pub u8);
impl crate::Specific for DPT_23_x {
    const DPT: DPT = DPT::new(23u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = [0u8];
        let mut writer = BitWriter::endian(bytes.as_mut_slice(), BigEndian);
        writer.write(2u8 as u32, self.0).unwrap();
        DataPoint::Short(writer.into_unwritten().1)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Short(byte) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new([*byte]));
            reader.seek_bits(SeekFrom::End(2u16 as i64)).unwrap();
            Ok(DPT_23_x(reader.read(2u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_23_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_24_1(pub String);
impl crate::Specific for DPT_24_1 {
    const DPT: DPT = DPT::new(24u16, Some(1u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        let bytes = ISO_8859_1.encode(self.0.as_str(), EncoderTrap::Replace).unwrap();
        writer.write_bytes(&bytes).unwrap();
        writer.write_bytes(&[0]).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_24_1({
                    let mut buffer = [0u8; 2048];
                    reader.read_bytes(&mut buffer).unwrap();
                    let end = buffer
                        .iter()
                        .position(|&c| c == b'\0')
                        .unwrap_or(buffer.len());
                    ISO_8859_1.decode(&buffer[0..end], DecoderTrap::Replace).unwrap()
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_24_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_24_x(pub String);
impl crate::Specific for DPT_24_x {
    const DPT: DPT = DPT::new(24u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        let bytes = ISO_8859_1.encode(self.0.as_str(), EncoderTrap::Replace).unwrap();
        writer.write_bytes(&bytes).unwrap();
        writer.write_bytes(&[0]).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_24_x({
                    let mut buffer = [0u8; 2048];
                    reader.read_bytes(&mut buffer).unwrap();
                    let end = buffer
                        .iter()
                        .position(|&c| c == b'\0')
                        .unwrap_or(buffer.len());
                    ISO_8859_1.decode(&buffer[0..end], DecoderTrap::Replace).unwrap()
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_24_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_25_1000 {
    pub Busy: u8,
    pub Nak: u8,
}
impl crate::Specific for DPT_25_1000 {
    const DPT: DPT = DPT::new(25u16, Some(1000u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(4u8 as u32, self.Busy).unwrap();
        writer.write(4u8 as u32, self.Nak).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_25_1000 {
                Busy: reader.read(4u8 as u32)?,
                Nak: reader.read(4u8 as u32)?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_25_1000 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.Busy, self.Nak)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_25_x {
    pub Busy: u8,
    pub Nak: u8,
}
impl crate::Specific for DPT_25_x {
    const DPT: DPT = DPT::new(25u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(4u8 as u32, self.Busy).unwrap();
        writer.write(4u8 as u32, self.Nak).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_25_x {
                Busy: reader.read(4u8 as u32)?,
                Nak: reader.read(4u8 as u32)?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_25_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.Busy, self.Nak)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_26_1 {
    pub reserved: Reserved,
    pub sceneisinactive: bool,
    pub SceneNumber: u8,
}
impl crate::Specific for DPT_26_1 {
    const DPT: DPT = DPT::new(26u16, Some(1u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(1u8 as u32, 0u32).unwrap();
        writer.write_bit(self.sceneisinactive).unwrap();
        writer.write(6u8 as u32, self.SceneNumber).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_26_1 {
                reserved: {
                    reader.skip(1u8 as u32)?;
                    Reserved::new()
                },
                sceneisinactive: reader.read_bit()?,
                SceneNumber: reader.read(6u8 as u32)?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_26_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}/{}", self.reserved, self.sceneisinactive, self.SceneNumber)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_26_x {
    pub reserved: Reserved,
    pub sceneisinactive: bool,
    pub SceneNumber: u8,
}
impl crate::Specific for DPT_26_x {
    const DPT: DPT = DPT::new(26u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(1u8 as u32, 0u32).unwrap();
        writer.write_bit(self.sceneisinactive).unwrap();
        writer.write(6u8 as u32, self.SceneNumber).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_26_x {
                reserved: {
                    reader.skip(1u8 as u32)?;
                    Reserved::new()
                },
                sceneisinactive: reader.read_bit()?,
                SceneNumber: reader.read(6u8 as u32)?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_26_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}/{}", self.reserved, self.sceneisinactive, self.SceneNumber)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_27_1 {
    pub MaskBitInfoOnOffOutput: bool,
    pub MaskBitInfoOnOffOutput2: bool,
    pub MaskBitInfoOnOffOutput3: bool,
    pub MaskBitInfoOnOffOutput4: bool,
    pub MaskBitInfoOnOffOutput5: bool,
    pub MaskBitInfoOnOffOutput6: bool,
    pub MaskBitInfoOnOffOutput7: bool,
    pub MaskBitInfoOnOffOutput8: bool,
    pub MaskBitInfoOnOffOutput9: bool,
    pub MaskBitInfoOnOffOutput10: bool,
    pub MaskBitInfoOnOffOutput11: bool,
    pub MaskBitInfoOnOffOutput12: bool,
    pub MaskBitInfoOnOffOutput13: bool,
    pub MaskBitInfoOnOffOutput14: bool,
    pub MaskBitInfoOnOffOutput15: bool,
    pub MaskBitInfoOnOffOutput16: bool,
    pub InfoOnOffOutput: bool,
    pub InfoOnOffOutput2: bool,
    pub InfoOnOffOutput3: bool,
    pub InfoOnOffOutput4: bool,
    pub InfoOnOffOutput5: bool,
    pub InfoOnOffOutput6: bool,
    pub InfoOnOffOutput7: bool,
    pub InfoOnOffOutput8: bool,
    pub InfoOnOffOutput9: bool,
    pub InfoOnOffOutput10: bool,
    pub InfoOnOffOutput11: bool,
    pub InfoOnOffOutput12: bool,
    pub InfoOnOffOutput13: bool,
    pub InfoOnOffOutput14: bool,
    pub InfoOnOffOutput15: bool,
    pub InfoOnOffOutput16: bool,
}
impl crate::Specific for DPT_27_1 {
    const DPT: DPT = DPT::new(27u16, Some(1u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_bit(self.MaskBitInfoOnOffOutput).unwrap();
        writer.write_bit(self.MaskBitInfoOnOffOutput2).unwrap();
        writer.write_bit(self.MaskBitInfoOnOffOutput3).unwrap();
        writer.write_bit(self.MaskBitInfoOnOffOutput4).unwrap();
        writer.write_bit(self.MaskBitInfoOnOffOutput5).unwrap();
        writer.write_bit(self.MaskBitInfoOnOffOutput6).unwrap();
        writer.write_bit(self.MaskBitInfoOnOffOutput7).unwrap();
        writer.write_bit(self.MaskBitInfoOnOffOutput8).unwrap();
        writer.write_bit(self.MaskBitInfoOnOffOutput9).unwrap();
        writer.write_bit(self.MaskBitInfoOnOffOutput10).unwrap();
        writer.write_bit(self.MaskBitInfoOnOffOutput11).unwrap();
        writer.write_bit(self.MaskBitInfoOnOffOutput12).unwrap();
        writer.write_bit(self.MaskBitInfoOnOffOutput13).unwrap();
        writer.write_bit(self.MaskBitInfoOnOffOutput14).unwrap();
        writer.write_bit(self.MaskBitInfoOnOffOutput15).unwrap();
        writer.write_bit(self.MaskBitInfoOnOffOutput16).unwrap();
        writer.write_bit(self.InfoOnOffOutput).unwrap();
        writer.write_bit(self.InfoOnOffOutput2).unwrap();
        writer.write_bit(self.InfoOnOffOutput3).unwrap();
        writer.write_bit(self.InfoOnOffOutput4).unwrap();
        writer.write_bit(self.InfoOnOffOutput5).unwrap();
        writer.write_bit(self.InfoOnOffOutput6).unwrap();
        writer.write_bit(self.InfoOnOffOutput7).unwrap();
        writer.write_bit(self.InfoOnOffOutput8).unwrap();
        writer.write_bit(self.InfoOnOffOutput9).unwrap();
        writer.write_bit(self.InfoOnOffOutput10).unwrap();
        writer.write_bit(self.InfoOnOffOutput11).unwrap();
        writer.write_bit(self.InfoOnOffOutput12).unwrap();
        writer.write_bit(self.InfoOnOffOutput13).unwrap();
        writer.write_bit(self.InfoOnOffOutput14).unwrap();
        writer.write_bit(self.InfoOnOffOutput15).unwrap();
        writer.write_bit(self.InfoOnOffOutput16).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_27_1 {
                MaskBitInfoOnOffOutput: reader.read_bit()?,
                MaskBitInfoOnOffOutput2: reader.read_bit()?,
                MaskBitInfoOnOffOutput3: reader.read_bit()?,
                MaskBitInfoOnOffOutput4: reader.read_bit()?,
                MaskBitInfoOnOffOutput5: reader.read_bit()?,
                MaskBitInfoOnOffOutput6: reader.read_bit()?,
                MaskBitInfoOnOffOutput7: reader.read_bit()?,
                MaskBitInfoOnOffOutput8: reader.read_bit()?,
                MaskBitInfoOnOffOutput9: reader.read_bit()?,
                MaskBitInfoOnOffOutput10: reader.read_bit()?,
                MaskBitInfoOnOffOutput11: reader.read_bit()?,
                MaskBitInfoOnOffOutput12: reader.read_bit()?,
                MaskBitInfoOnOffOutput13: reader.read_bit()?,
                MaskBitInfoOnOffOutput14: reader.read_bit()?,
                MaskBitInfoOnOffOutput15: reader.read_bit()?,
                MaskBitInfoOnOffOutput16: reader.read_bit()?,
                InfoOnOffOutput: reader.read_bit()?,
                InfoOnOffOutput2: reader.read_bit()?,
                InfoOnOffOutput3: reader.read_bit()?,
                InfoOnOffOutput4: reader.read_bit()?,
                InfoOnOffOutput5: reader.read_bit()?,
                InfoOnOffOutput6: reader.read_bit()?,
                InfoOnOffOutput7: reader.read_bit()?,
                InfoOnOffOutput8: reader.read_bit()?,
                InfoOnOffOutput9: reader.read_bit()?,
                InfoOnOffOutput10: reader.read_bit()?,
                InfoOnOffOutput11: reader.read_bit()?,
                InfoOnOffOutput12: reader.read_bit()?,
                InfoOnOffOutput13: reader.read_bit()?,
                InfoOnOffOutput14: reader.read_bit()?,
                InfoOnOffOutput15: reader.read_bit()?,
                InfoOnOffOutput16: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_27_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}",
            self.MaskBitInfoOnOffOutput, self.MaskBitInfoOnOffOutput2, self
            .MaskBitInfoOnOffOutput3, self.MaskBitInfoOnOffOutput4, self
            .MaskBitInfoOnOffOutput5, self.MaskBitInfoOnOffOutput6, self
            .MaskBitInfoOnOffOutput7, self.MaskBitInfoOnOffOutput8, self
            .MaskBitInfoOnOffOutput9, self.MaskBitInfoOnOffOutput10, self
            .MaskBitInfoOnOffOutput11, self.MaskBitInfoOnOffOutput12, self
            .MaskBitInfoOnOffOutput13, self.MaskBitInfoOnOffOutput14, self
            .MaskBitInfoOnOffOutput15, self.MaskBitInfoOnOffOutput16, self
            .InfoOnOffOutput, self.InfoOnOffOutput2, self.InfoOnOffOutput3, self
            .InfoOnOffOutput4, self.InfoOnOffOutput5, self.InfoOnOffOutput6, self
            .InfoOnOffOutput7, self.InfoOnOffOutput8, self.InfoOnOffOutput9, self
            .InfoOnOffOutput10, self.InfoOnOffOutput11, self.InfoOnOffOutput12, self
            .InfoOnOffOutput13, self.InfoOnOffOutput14, self.InfoOnOffOutput15, self
            .InfoOnOffOutput16
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_27_x {
    pub MaskBitInfoOnOffOutput: bool,
    pub MaskBitInfoOnOffOutput2: bool,
    pub MaskBitInfoOnOffOutput3: bool,
    pub MaskBitInfoOnOffOutput4: bool,
    pub MaskBitInfoOnOffOutput5: bool,
    pub MaskBitInfoOnOffOutput6: bool,
    pub MaskBitInfoOnOffOutput7: bool,
    pub MaskBitInfoOnOffOutput8: bool,
    pub MaskBitInfoOnOffOutput9: bool,
    pub MaskBitInfoOnOffOutput10: bool,
    pub MaskBitInfoOnOffOutput11: bool,
    pub MaskBitInfoOnOffOutput12: bool,
    pub MaskBitInfoOnOffOutput13: bool,
    pub MaskBitInfoOnOffOutput14: bool,
    pub MaskBitInfoOnOffOutput15: bool,
    pub MaskBitInfoOnOffOutput16: bool,
    pub InfoOnOffOutput: bool,
    pub InfoOnOffOutput2: bool,
    pub InfoOnOffOutput3: bool,
    pub InfoOnOffOutput4: bool,
    pub InfoOnOffOutput5: bool,
    pub InfoOnOffOutput6: bool,
    pub InfoOnOffOutput7: bool,
    pub InfoOnOffOutput8: bool,
    pub InfoOnOffOutput9: bool,
    pub InfoOnOffOutput10: bool,
    pub InfoOnOffOutput11: bool,
    pub InfoOnOffOutput12: bool,
    pub InfoOnOffOutput13: bool,
    pub InfoOnOffOutput14: bool,
    pub InfoOnOffOutput15: bool,
    pub InfoOnOffOutput16: bool,
}
impl crate::Specific for DPT_27_x {
    const DPT: DPT = DPT::new(27u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_bit(self.MaskBitInfoOnOffOutput).unwrap();
        writer.write_bit(self.MaskBitInfoOnOffOutput2).unwrap();
        writer.write_bit(self.MaskBitInfoOnOffOutput3).unwrap();
        writer.write_bit(self.MaskBitInfoOnOffOutput4).unwrap();
        writer.write_bit(self.MaskBitInfoOnOffOutput5).unwrap();
        writer.write_bit(self.MaskBitInfoOnOffOutput6).unwrap();
        writer.write_bit(self.MaskBitInfoOnOffOutput7).unwrap();
        writer.write_bit(self.MaskBitInfoOnOffOutput8).unwrap();
        writer.write_bit(self.MaskBitInfoOnOffOutput9).unwrap();
        writer.write_bit(self.MaskBitInfoOnOffOutput10).unwrap();
        writer.write_bit(self.MaskBitInfoOnOffOutput11).unwrap();
        writer.write_bit(self.MaskBitInfoOnOffOutput12).unwrap();
        writer.write_bit(self.MaskBitInfoOnOffOutput13).unwrap();
        writer.write_bit(self.MaskBitInfoOnOffOutput14).unwrap();
        writer.write_bit(self.MaskBitInfoOnOffOutput15).unwrap();
        writer.write_bit(self.MaskBitInfoOnOffOutput16).unwrap();
        writer.write_bit(self.InfoOnOffOutput).unwrap();
        writer.write_bit(self.InfoOnOffOutput2).unwrap();
        writer.write_bit(self.InfoOnOffOutput3).unwrap();
        writer.write_bit(self.InfoOnOffOutput4).unwrap();
        writer.write_bit(self.InfoOnOffOutput5).unwrap();
        writer.write_bit(self.InfoOnOffOutput6).unwrap();
        writer.write_bit(self.InfoOnOffOutput7).unwrap();
        writer.write_bit(self.InfoOnOffOutput8).unwrap();
        writer.write_bit(self.InfoOnOffOutput9).unwrap();
        writer.write_bit(self.InfoOnOffOutput10).unwrap();
        writer.write_bit(self.InfoOnOffOutput11).unwrap();
        writer.write_bit(self.InfoOnOffOutput12).unwrap();
        writer.write_bit(self.InfoOnOffOutput13).unwrap();
        writer.write_bit(self.InfoOnOffOutput14).unwrap();
        writer.write_bit(self.InfoOnOffOutput15).unwrap();
        writer.write_bit(self.InfoOnOffOutput16).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_27_x {
                MaskBitInfoOnOffOutput: reader.read_bit()?,
                MaskBitInfoOnOffOutput2: reader.read_bit()?,
                MaskBitInfoOnOffOutput3: reader.read_bit()?,
                MaskBitInfoOnOffOutput4: reader.read_bit()?,
                MaskBitInfoOnOffOutput5: reader.read_bit()?,
                MaskBitInfoOnOffOutput6: reader.read_bit()?,
                MaskBitInfoOnOffOutput7: reader.read_bit()?,
                MaskBitInfoOnOffOutput8: reader.read_bit()?,
                MaskBitInfoOnOffOutput9: reader.read_bit()?,
                MaskBitInfoOnOffOutput10: reader.read_bit()?,
                MaskBitInfoOnOffOutput11: reader.read_bit()?,
                MaskBitInfoOnOffOutput12: reader.read_bit()?,
                MaskBitInfoOnOffOutput13: reader.read_bit()?,
                MaskBitInfoOnOffOutput14: reader.read_bit()?,
                MaskBitInfoOnOffOutput15: reader.read_bit()?,
                MaskBitInfoOnOffOutput16: reader.read_bit()?,
                InfoOnOffOutput: reader.read_bit()?,
                InfoOnOffOutput2: reader.read_bit()?,
                InfoOnOffOutput3: reader.read_bit()?,
                InfoOnOffOutput4: reader.read_bit()?,
                InfoOnOffOutput5: reader.read_bit()?,
                InfoOnOffOutput6: reader.read_bit()?,
                InfoOnOffOutput7: reader.read_bit()?,
                InfoOnOffOutput8: reader.read_bit()?,
                InfoOnOffOutput9: reader.read_bit()?,
                InfoOnOffOutput10: reader.read_bit()?,
                InfoOnOffOutput11: reader.read_bit()?,
                InfoOnOffOutput12: reader.read_bit()?,
                InfoOnOffOutput13: reader.read_bit()?,
                InfoOnOffOutput14: reader.read_bit()?,
                InfoOnOffOutput15: reader.read_bit()?,
                InfoOnOffOutput16: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_27_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}",
            self.MaskBitInfoOnOffOutput, self.MaskBitInfoOnOffOutput2, self
            .MaskBitInfoOnOffOutput3, self.MaskBitInfoOnOffOutput4, self
            .MaskBitInfoOnOffOutput5, self.MaskBitInfoOnOffOutput6, self
            .MaskBitInfoOnOffOutput7, self.MaskBitInfoOnOffOutput8, self
            .MaskBitInfoOnOffOutput9, self.MaskBitInfoOnOffOutput10, self
            .MaskBitInfoOnOffOutput11, self.MaskBitInfoOnOffOutput12, self
            .MaskBitInfoOnOffOutput13, self.MaskBitInfoOnOffOutput14, self
            .MaskBitInfoOnOffOutput15, self.MaskBitInfoOnOffOutput16, self
            .InfoOnOffOutput, self.InfoOnOffOutput2, self.InfoOnOffOutput3, self
            .InfoOnOffOutput4, self.InfoOnOffOutput5, self.InfoOnOffOutput6, self
            .InfoOnOffOutput7, self.InfoOnOffOutput8, self.InfoOnOffOutput9, self
            .InfoOnOffOutput10, self.InfoOnOffOutput11, self.InfoOnOffOutput12, self
            .InfoOnOffOutput13, self.InfoOnOffOutput14, self.InfoOnOffOutput15, self
            .InfoOnOffOutput16
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_28_1(pub String);
impl crate::Specific for DPT_28_1 {
    const DPT: DPT = DPT::new(28u16, Some(1u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        let bytes = UTF_8.encode(self.0.as_str(), EncoderTrap::Replace).unwrap();
        writer.write_bytes(&bytes).unwrap();
        writer.write_bytes(&[0]).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_28_1({
                    let mut buffer = [0u8; 2048];
                    reader.read_bytes(&mut buffer).unwrap();
                    let end = buffer
                        .iter()
                        .position(|&c| c == b'\0')
                        .unwrap_or(buffer.len());
                    UTF_8.decode(&buffer[0..end], DecoderTrap::Replace).unwrap()
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_28_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_28_x(pub String);
impl crate::Specific for DPT_28_x {
    const DPT: DPT = DPT::new(28u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        let bytes = UTF_8.encode(self.0.as_str(), EncoderTrap::Replace).unwrap();
        writer.write_bytes(&bytes).unwrap();
        writer.write_bytes(&[0]).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_28_x({
                    let mut buffer = [0u8; 2048];
                    reader.read_bytes(&mut buffer).unwrap();
                    let end = buffer
                        .iter()
                        .position(|&c| c == b'\0')
                        .unwrap_or(buffer.len());
                    UTF_8.decode(&buffer[0..end], DecoderTrap::Replace).unwrap()
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_28_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_29_10(pub i64);
impl crate::Specific for DPT_29_10 {
    const DPT: DPT = DPT::new(29u16, Some(10u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_signed(64u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_29_10(reader.read(64u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_29_10 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_29_11(pub i64);
impl crate::Specific for DPT_29_11 {
    const DPT: DPT = DPT::new(29u16, Some(11u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_signed(64u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_29_11(reader.read(64u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_29_11 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_29_12(pub i64);
impl crate::Specific for DPT_29_12 {
    const DPT: DPT = DPT::new(29u16, Some(12u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_signed(64u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_29_12(reader.read(64u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_29_12 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_29_x(pub i64);
impl crate::Specific for DPT_29_x {
    const DPT: DPT = DPT::new(29u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_signed(64u8 as u32, self.0).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_29_x(reader.read(64u8 as u32)?))
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_29_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_30_1010 {
    pub Activationstateofchannel: bool,
    pub Activationstateofchannel2: bool,
    pub Activationstateofchannel3: bool,
    pub Activationstateofchannel4: bool,
    pub Activationstateofchannel5: bool,
    pub Activationstateofchannel6: bool,
    pub Activationstateofchannel7: bool,
    pub Activationstateofchannel8: bool,
    pub Activationstateofchannel9: bool,
    pub Activationstateofchannel10: bool,
    pub Activationstateofchannel11: bool,
    pub Activationstateofchannel12: bool,
    pub Activationstateofchannel13: bool,
    pub Activationstateofchannel14: bool,
    pub Activationstateofchannel15: bool,
    pub Activationstateofchannel16: bool,
    pub Activationstateofchannel17: bool,
    pub Activationstateofchannel18: bool,
    pub Activationstateofchannel19: bool,
    pub Activationstateofchannel20: bool,
    pub Activationstateofchannel21: bool,
    pub Activationstateofchannel22: bool,
    pub Activationstateofchannel23: bool,
    pub Activationstateofchannel24: bool,
}
impl crate::Specific for DPT_30_1010 {
    const DPT: DPT = DPT::new(30u16, Some(1010u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_bit(self.Activationstateofchannel).unwrap();
        writer.write_bit(self.Activationstateofchannel2).unwrap();
        writer.write_bit(self.Activationstateofchannel3).unwrap();
        writer.write_bit(self.Activationstateofchannel4).unwrap();
        writer.write_bit(self.Activationstateofchannel5).unwrap();
        writer.write_bit(self.Activationstateofchannel6).unwrap();
        writer.write_bit(self.Activationstateofchannel7).unwrap();
        writer.write_bit(self.Activationstateofchannel8).unwrap();
        writer.write_bit(self.Activationstateofchannel9).unwrap();
        writer.write_bit(self.Activationstateofchannel10).unwrap();
        writer.write_bit(self.Activationstateofchannel11).unwrap();
        writer.write_bit(self.Activationstateofchannel12).unwrap();
        writer.write_bit(self.Activationstateofchannel13).unwrap();
        writer.write_bit(self.Activationstateofchannel14).unwrap();
        writer.write_bit(self.Activationstateofchannel15).unwrap();
        writer.write_bit(self.Activationstateofchannel16).unwrap();
        writer.write_bit(self.Activationstateofchannel17).unwrap();
        writer.write_bit(self.Activationstateofchannel18).unwrap();
        writer.write_bit(self.Activationstateofchannel19).unwrap();
        writer.write_bit(self.Activationstateofchannel20).unwrap();
        writer.write_bit(self.Activationstateofchannel21).unwrap();
        writer.write_bit(self.Activationstateofchannel22).unwrap();
        writer.write_bit(self.Activationstateofchannel23).unwrap();
        writer.write_bit(self.Activationstateofchannel24).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_30_1010 {
                Activationstateofchannel: reader.read_bit()?,
                Activationstateofchannel2: reader.read_bit()?,
                Activationstateofchannel3: reader.read_bit()?,
                Activationstateofchannel4: reader.read_bit()?,
                Activationstateofchannel5: reader.read_bit()?,
                Activationstateofchannel6: reader.read_bit()?,
                Activationstateofchannel7: reader.read_bit()?,
                Activationstateofchannel8: reader.read_bit()?,
                Activationstateofchannel9: reader.read_bit()?,
                Activationstateofchannel10: reader.read_bit()?,
                Activationstateofchannel11: reader.read_bit()?,
                Activationstateofchannel12: reader.read_bit()?,
                Activationstateofchannel13: reader.read_bit()?,
                Activationstateofchannel14: reader.read_bit()?,
                Activationstateofchannel15: reader.read_bit()?,
                Activationstateofchannel16: reader.read_bit()?,
                Activationstateofchannel17: reader.read_bit()?,
                Activationstateofchannel18: reader.read_bit()?,
                Activationstateofchannel19: reader.read_bit()?,
                Activationstateofchannel20: reader.read_bit()?,
                Activationstateofchannel21: reader.read_bit()?,
                Activationstateofchannel22: reader.read_bit()?,
                Activationstateofchannel23: reader.read_bit()?,
                Activationstateofchannel24: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_30_1010 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}",
            self.Activationstateofchannel, self.Activationstateofchannel2, self
            .Activationstateofchannel3, self.Activationstateofchannel4, self
            .Activationstateofchannel5, self.Activationstateofchannel6, self
            .Activationstateofchannel7, self.Activationstateofchannel8, self
            .Activationstateofchannel9, self.Activationstateofchannel10, self
            .Activationstateofchannel11, self.Activationstateofchannel12, self
            .Activationstateofchannel13, self.Activationstateofchannel14, self
            .Activationstateofchannel15, self.Activationstateofchannel16, self
            .Activationstateofchannel17, self.Activationstateofchannel18, self
            .Activationstateofchannel19, self.Activationstateofchannel20, self
            .Activationstateofchannel21, self.Activationstateofchannel22, self
            .Activationstateofchannel23, self.Activationstateofchannel24
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_30_x {
    pub Activationstateofchannel: bool,
    pub Activationstateofchannel2: bool,
    pub Activationstateofchannel3: bool,
    pub Activationstateofchannel4: bool,
    pub Activationstateofchannel5: bool,
    pub Activationstateofchannel6: bool,
    pub Activationstateofchannel7: bool,
    pub Activationstateofchannel8: bool,
    pub Activationstateofchannel9: bool,
    pub Activationstateofchannel10: bool,
    pub Activationstateofchannel11: bool,
    pub Activationstateofchannel12: bool,
    pub Activationstateofchannel13: bool,
    pub Activationstateofchannel14: bool,
    pub Activationstateofchannel15: bool,
    pub Activationstateofchannel16: bool,
    pub Activationstateofchannel17: bool,
    pub Activationstateofchannel18: bool,
    pub Activationstateofchannel19: bool,
    pub Activationstateofchannel20: bool,
    pub Activationstateofchannel21: bool,
    pub Activationstateofchannel22: bool,
    pub Activationstateofchannel23: bool,
    pub Activationstateofchannel24: bool,
}
impl crate::Specific for DPT_30_x {
    const DPT: DPT = DPT::new(30u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_bit(self.Activationstateofchannel).unwrap();
        writer.write_bit(self.Activationstateofchannel2).unwrap();
        writer.write_bit(self.Activationstateofchannel3).unwrap();
        writer.write_bit(self.Activationstateofchannel4).unwrap();
        writer.write_bit(self.Activationstateofchannel5).unwrap();
        writer.write_bit(self.Activationstateofchannel6).unwrap();
        writer.write_bit(self.Activationstateofchannel7).unwrap();
        writer.write_bit(self.Activationstateofchannel8).unwrap();
        writer.write_bit(self.Activationstateofchannel9).unwrap();
        writer.write_bit(self.Activationstateofchannel10).unwrap();
        writer.write_bit(self.Activationstateofchannel11).unwrap();
        writer.write_bit(self.Activationstateofchannel12).unwrap();
        writer.write_bit(self.Activationstateofchannel13).unwrap();
        writer.write_bit(self.Activationstateofchannel14).unwrap();
        writer.write_bit(self.Activationstateofchannel15).unwrap();
        writer.write_bit(self.Activationstateofchannel16).unwrap();
        writer.write_bit(self.Activationstateofchannel17).unwrap();
        writer.write_bit(self.Activationstateofchannel18).unwrap();
        writer.write_bit(self.Activationstateofchannel19).unwrap();
        writer.write_bit(self.Activationstateofchannel20).unwrap();
        writer.write_bit(self.Activationstateofchannel21).unwrap();
        writer.write_bit(self.Activationstateofchannel22).unwrap();
        writer.write_bit(self.Activationstateofchannel23).unwrap();
        writer.write_bit(self.Activationstateofchannel24).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_30_x {
                Activationstateofchannel: reader.read_bit()?,
                Activationstateofchannel2: reader.read_bit()?,
                Activationstateofchannel3: reader.read_bit()?,
                Activationstateofchannel4: reader.read_bit()?,
                Activationstateofchannel5: reader.read_bit()?,
                Activationstateofchannel6: reader.read_bit()?,
                Activationstateofchannel7: reader.read_bit()?,
                Activationstateofchannel8: reader.read_bit()?,
                Activationstateofchannel9: reader.read_bit()?,
                Activationstateofchannel10: reader.read_bit()?,
                Activationstateofchannel11: reader.read_bit()?,
                Activationstateofchannel12: reader.read_bit()?,
                Activationstateofchannel13: reader.read_bit()?,
                Activationstateofchannel14: reader.read_bit()?,
                Activationstateofchannel15: reader.read_bit()?,
                Activationstateofchannel16: reader.read_bit()?,
                Activationstateofchannel17: reader.read_bit()?,
                Activationstateofchannel18: reader.read_bit()?,
                Activationstateofchannel19: reader.read_bit()?,
                Activationstateofchannel20: reader.read_bit()?,
                Activationstateofchannel21: reader.read_bit()?,
                Activationstateofchannel22: reader.read_bit()?,
                Activationstateofchannel23: reader.read_bit()?,
                Activationstateofchannel24: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_30_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}",
            self.Activationstateofchannel, self.Activationstateofchannel2, self
            .Activationstateofchannel3, self.Activationstateofchannel4, self
            .Activationstateofchannel5, self.Activationstateofchannel6, self
            .Activationstateofchannel7, self.Activationstateofchannel8, self
            .Activationstateofchannel9, self.Activationstateofchannel10, self
            .Activationstateofchannel11, self.Activationstateofchannel12, self
            .Activationstateofchannel13, self.Activationstateofchannel14, self
            .Activationstateofchannel15, self.Activationstateofchannel16, self
            .Activationstateofchannel17, self.Activationstateofchannel18, self
            .Activationstateofchannel19, self.Activationstateofchannel20, self
            .Activationstateofchannel21, self.Activationstateofchannel22, self
            .Activationstateofchannel23, self.Activationstateofchannel24
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_206_100 {
    pub delaytimemin: u16,
    pub HVACmode: u8,
}
impl crate::Specific for DPT_206_100 {
    const DPT: DPT = DPT::new(206u16, Some(100u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, self.delaytimemin).unwrap();
        writer.write(8u8 as u32, self.HVACmode).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_206_100 {
                delaytimemin: reader.read(16u8 as u32)?,
                HVACmode: reader.read(8u8 as u32)?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_206_100 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.delaytimemin, self.HVACmode)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_206_102 {
    pub delaytimemin: u16,
    pub DHWmode: u8,
}
impl crate::Specific for DPT_206_102 {
    const DPT: DPT = DPT::new(206u16, Some(102u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, self.delaytimemin).unwrap();
        writer.write(8u8 as u32, self.DHWmode).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_206_102 {
                delaytimemin: reader.read(16u8 as u32)?,
                DHWmode: reader.read(8u8 as u32)?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_206_102 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.delaytimemin, self.DHWmode)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_206_104 {
    pub delaytimemin: u16,
    pub occupancymode: u8,
}
impl crate::Specific for DPT_206_104 {
    const DPT: DPT = DPT::new(206u16, Some(104u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, self.delaytimemin).unwrap();
        writer.write(8u8 as u32, self.occupancymode).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_206_104 {
                delaytimemin: reader.read(16u8 as u32)?,
                occupancymode: reader.read(8u8 as u32)?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_206_104 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.delaytimemin, self.occupancymode)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_206_105 {
    pub delaytimemin: u16,
    pub buildingmode: u8,
}
impl crate::Specific for DPT_206_105 {
    const DPT: DPT = DPT::new(206u16, Some(105u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, self.delaytimemin).unwrap();
        writer.write(8u8 as u32, self.buildingmode).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_206_105 {
                delaytimemin: reader.read(16u8 as u32)?,
                buildingmode: reader.read(8u8 as u32)?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_206_105 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.delaytimemin, self.buildingmode)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_206_x {
    pub delaytimemin: u16,
    pub HVACmode: u8,
}
impl crate::Specific for DPT_206_x {
    const DPT: DPT = DPT::new(206u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, self.delaytimemin).unwrap();
        writer.write(8u8 as u32, self.HVACmode).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_206_x {
                delaytimemin: reader.read(16u8 as u32)?,
                HVACmode: reader.read(8u8 as u32)?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_206_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.delaytimemin, self.HVACmode)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_207_600 {
    pub ActualValue: u8,
    pub Failure: bool,
    pub LocalOverride: bool,
    pub Dimming: bool,
    pub StaircaseLightingFunction: bool,
    pub NightModeActive: bool,
    pub Forced: bool,
    pub Locked: bool,
    pub ValidActualValue: bool,
}
impl crate::Specific for DPT_207_600 {
    const DPT: DPT = DPT::new(207u16, Some(600u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.ActualValue).unwrap();
        writer.write_bit(self.Failure).unwrap();
        writer.write_bit(self.LocalOverride).unwrap();
        writer.write_bit(self.Dimming).unwrap();
        writer.write_bit(self.StaircaseLightingFunction).unwrap();
        writer.write_bit(self.NightModeActive).unwrap();
        writer.write_bit(self.Forced).unwrap();
        writer.write_bit(self.Locked).unwrap();
        writer.write_bit(self.ValidActualValue).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_207_600 {
                ActualValue: reader.read(8u8 as u32)?,
                Failure: reader.read_bit()?,
                LocalOverride: reader.read_bit()?,
                Dimming: reader.read_bit()?,
                StaircaseLightingFunction: reader.read_bit()?,
                NightModeActive: reader.read_bit()?,
                Forced: reader.read_bit()?,
                Locked: reader.read_bit()?,
                ValidActualValue: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_207_600 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}/{}/{}/{}", self.ActualValue, self.Failure, self
            .LocalOverride, self.Dimming, self.StaircaseLightingFunction, self
            .NightModeActive, self.Forced, self.Locked, self.ValidActualValue
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_207_x {
    pub ActualValue: u8,
    pub Failure: bool,
    pub LocalOverride: bool,
    pub Dimming: bool,
    pub StaircaseLightingFunction: bool,
    pub NightModeActive: bool,
    pub Forced: bool,
    pub Locked: bool,
    pub ValidActualValue: bool,
}
impl crate::Specific for DPT_207_x {
    const DPT: DPT = DPT::new(207u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.ActualValue).unwrap();
        writer.write_bit(self.Failure).unwrap();
        writer.write_bit(self.LocalOverride).unwrap();
        writer.write_bit(self.Dimming).unwrap();
        writer.write_bit(self.StaircaseLightingFunction).unwrap();
        writer.write_bit(self.NightModeActive).unwrap();
        writer.write_bit(self.Forced).unwrap();
        writer.write_bit(self.Locked).unwrap();
        writer.write_bit(self.ValidActualValue).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_207_x {
                ActualValue: reader.read(8u8 as u32)?,
                Failure: reader.read_bit()?,
                LocalOverride: reader.read_bit()?,
                Dimming: reader.read_bit()?,
                StaircaseLightingFunction: reader.read_bit()?,
                NightModeActive: reader.read_bit()?,
                Forced: reader.read_bit()?,
                Locked: reader.read_bit()?,
                ValidActualValue: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_207_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}/{}/{}/{}", self.ActualValue, self.Failure, self
            .LocalOverride, self.Dimming, self.StaircaseLightingFunction, self
            .NightModeActive, self.Forced, self.Locked, self.ValidActualValue
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_217_1 {
    pub MagicNumber: u8,
    pub VersionNumber: u8,
    pub RevisionNumber: u8,
}
impl crate::Specific for DPT_217_1 {
    const DPT: DPT = DPT::new(217u16, Some(1u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(5u8 as u32, self.MagicNumber).unwrap();
        writer.write(5u8 as u32, self.VersionNumber).unwrap();
        writer.write(6u8 as u32, self.RevisionNumber).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_217_1 {
                MagicNumber: reader.read(5u8 as u32)?,
                VersionNumber: reader.read(5u8 as u32)?,
                RevisionNumber: reader.read(6u8 as u32)?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_217_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}/{}", self.MagicNumber, self.VersionNumber, self.RevisionNumber)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_217_x {
    pub MagicNumber: u8,
    pub VersionNumber: u8,
    pub RevisionNumber: u8,
}
impl crate::Specific for DPT_217_x {
    const DPT: DPT = DPT::new(217u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(5u8 as u32, self.MagicNumber).unwrap();
        writer.write(5u8 as u32, self.VersionNumber).unwrap();
        writer.write(6u8 as u32, self.RevisionNumber).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_217_x {
                MagicNumber: reader.read(5u8 as u32)?,
                VersionNumber: reader.read(5u8 as u32)?,
                RevisionNumber: reader.read(6u8 as u32)?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_217_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}/{}", self.MagicNumber, self.VersionNumber, self.RevisionNumber)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_219_1 {
    pub LogNumber: u8,
    pub AlarmPriority: u8,
    pub ApplicationArea: u8,
    pub ErrorClass: u8,
    pub reserved: Reserved,
    pub ErrorCodeSup: bool,
    pub AlarmTextSup: bool,
    pub TimeStampSup: bool,
    pub AckSup: bool,
    pub reserved2: Reserved,
    pub Locked: bool,
    pub AlarmUnAck: bool,
    pub InAlarm: bool,
}
impl crate::Specific for DPT_219_1 {
    const DPT: DPT = DPT::new(219u16, Some(1u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.LogNumber).unwrap();
        writer.write(8u8 as u32, self.AlarmPriority).unwrap();
        writer.write(8u8 as u32, self.ApplicationArea).unwrap();
        writer.write(8u8 as u32, self.ErrorClass).unwrap();
        writer.write(4u8 as u32, 0u32).unwrap();
        writer.write_bit(self.ErrorCodeSup).unwrap();
        writer.write_bit(self.AlarmTextSup).unwrap();
        writer.write_bit(self.TimeStampSup).unwrap();
        writer.write_bit(self.AckSup).unwrap();
        writer.write(5u8 as u32, 0u32).unwrap();
        writer.write_bit(self.Locked).unwrap();
        writer.write_bit(self.AlarmUnAck).unwrap();
        writer.write_bit(self.InAlarm).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_219_1 {
                LogNumber: reader.read(8u8 as u32)?,
                AlarmPriority: reader.read(8u8 as u32)?,
                ApplicationArea: reader.read(8u8 as u32)?,
                ErrorClass: reader.read(8u8 as u32)?,
                reserved: {
                    reader.skip(4u8 as u32)?;
                    Reserved::new()
                },
                ErrorCodeSup: reader.read_bit()?,
                AlarmTextSup: reader.read_bit()?,
                TimeStampSup: reader.read_bit()?,
                AckSup: reader.read_bit()?,
                reserved2: {
                    reader.skip(5u8 as u32)?;
                    Reserved::new()
                },
                Locked: reader.read_bit()?,
                AlarmUnAck: reader.read_bit()?,
                InAlarm: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_219_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}", self.LogNumber, self
            .AlarmPriority, self.ApplicationArea, self.ErrorClass, self.reserved, self
            .ErrorCodeSup, self.AlarmTextSup, self.TimeStampSup, self.AckSup, self
            .reserved2, self.Locked, self.AlarmUnAck, self.InAlarm
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_219_x {
    pub LogNumber: u8,
    pub AlarmPriority: u8,
    pub ApplicationArea: u8,
    pub ErrorClass: u8,
    pub reserved: Reserved,
    pub ErrorCodeSup: bool,
    pub AlarmTextSup: bool,
    pub TimeStampSup: bool,
    pub AckSup: bool,
    pub reserved2: Reserved,
    pub Locked: bool,
    pub AlarmUnAck: bool,
    pub InAlarm: bool,
}
impl crate::Specific for DPT_219_x {
    const DPT: DPT = DPT::new(219u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.LogNumber).unwrap();
        writer.write(8u8 as u32, self.AlarmPriority).unwrap();
        writer.write(8u8 as u32, self.ApplicationArea).unwrap();
        writer.write(8u8 as u32, self.ErrorClass).unwrap();
        writer.write(4u8 as u32, 0u32).unwrap();
        writer.write_bit(self.ErrorCodeSup).unwrap();
        writer.write_bit(self.AlarmTextSup).unwrap();
        writer.write_bit(self.TimeStampSup).unwrap();
        writer.write_bit(self.AckSup).unwrap();
        writer.write(5u8 as u32, 0u32).unwrap();
        writer.write_bit(self.Locked).unwrap();
        writer.write_bit(self.AlarmUnAck).unwrap();
        writer.write_bit(self.InAlarm).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_219_x {
                LogNumber: reader.read(8u8 as u32)?,
                AlarmPriority: reader.read(8u8 as u32)?,
                ApplicationArea: reader.read(8u8 as u32)?,
                ErrorClass: reader.read(8u8 as u32)?,
                reserved: {
                    reader.skip(4u8 as u32)?;
                    Reserved::new()
                },
                ErrorCodeSup: reader.read_bit()?,
                AlarmTextSup: reader.read_bit()?,
                TimeStampSup: reader.read_bit()?,
                AckSup: reader.read_bit()?,
                reserved2: {
                    reader.skip(5u8 as u32)?;
                    Reserved::new()
                },
                Locked: reader.read_bit()?,
                AlarmUnAck: reader.read_bit()?,
                InAlarm: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_219_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}", self.LogNumber, self
            .AlarmPriority, self.ApplicationArea, self.ErrorClass, self.reserved, self
            .ErrorCodeSup, self.AlarmTextSup, self.TimeStampSup, self.AckSup, self
            .reserved2, self.Locked, self.AlarmUnAck, self.InAlarm
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_222_100 {
    pub TempSetpComf: f32,
    pub TempSetpStdby: f32,
    pub TempSetpEco: f32,
}
impl crate::Specific for DPT_222_100 {
    const DPT: DPT = DPT::new(222u16, Some(100u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, encode_knxf16(self.TempSetpComf)).unwrap();
        writer.write(16u8 as u32, encode_knxf16(self.TempSetpStdby)).unwrap();
        writer.write(16u8 as u32, encode_knxf16(self.TempSetpEco)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_222_100 {
                TempSetpComf: {
                    let value: u16 = reader.read(16u8 as u32)?;
                    let value = decode_knxf16(value);
                    value
                },
                TempSetpStdby: {
                    let value: u16 = reader.read(16u8 as u32)?;
                    let value = decode_knxf16(value);
                    value
                },
                TempSetpEco: {
                    let value: u16 = reader.read(16u8 as u32)?;
                    let value = decode_knxf16(value);
                    value
                },
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_222_100 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{:.1}/{:.1}/{:.1}", self.TempSetpComf, self.TempSetpStdby, self
            .TempSetpEco
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_222_101 {
    pub TempSetpShiftComf: f32,
    pub TempSetpShiftStdby: f32,
    pub TempSetpShiftEco: f32,
}
impl crate::Specific for DPT_222_101 {
    const DPT: DPT = DPT::new(222u16, Some(101u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, encode_knxf16(self.TempSetpShiftComf)).unwrap();
        writer.write(16u8 as u32, encode_knxf16(self.TempSetpShiftStdby)).unwrap();
        writer.write(16u8 as u32, encode_knxf16(self.TempSetpShiftEco)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_222_101 {
                TempSetpShiftComf: {
                    let value: u16 = reader.read(16u8 as u32)?;
                    let value = decode_knxf16(value);
                    value
                },
                TempSetpShiftStdby: {
                    let value: u16 = reader.read(16u8 as u32)?;
                    let value = decode_knxf16(value);
                    value
                },
                TempSetpShiftEco: {
                    let value: u16 = reader.read(16u8 as u32)?;
                    let value = decode_knxf16(value);
                    value
                },
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_222_101 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{:.1}/{:.1}/{:.1}", self.TempSetpShiftComf, self.TempSetpShiftStdby, self
            .TempSetpShiftEco
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_222_x {
    pub TempSetpComf: f32,
    pub TempSetpStdby: f32,
    pub TempSetpEco: f32,
}
impl crate::Specific for DPT_222_x {
    const DPT: DPT = DPT::new(222u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, encode_knxf16(self.TempSetpComf)).unwrap();
        writer.write(16u8 as u32, encode_knxf16(self.TempSetpStdby)).unwrap();
        writer.write(16u8 as u32, encode_knxf16(self.TempSetpEco)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_222_x {
                TempSetpComf: {
                    let value: u16 = reader.read(16u8 as u32)?;
                    let value = decode_knxf16(value);
                    value
                },
                TempSetpStdby: {
                    let value: u16 = reader.read(16u8 as u32)?;
                    let value = decode_knxf16(value);
                    value
                },
                TempSetpEco: {
                    let value: u16 = reader.read(16u8 as u32)?;
                    let value = decode_knxf16(value);
                    value
                },
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_222_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{:.1}/{:.1}/{:.1}", self.TempSetpComf, self.TempSetpStdby, self
            .TempSetpEco
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_225_1 {
    pub timeperiod: u16,
    pub percent: u8,
}
impl crate::Specific for DPT_225_1 {
    const DPT: DPT = DPT::new(225u16, Some(1u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, self.timeperiod).unwrap();
        writer.write(8u8 as u32, self.percent).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_225_1 {
                timeperiod: reader.read(16u8 as u32)?,
                percent: reader.read(8u8 as u32)?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_225_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.timeperiod, self.percent)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_225_2 {
    pub timeperiod: u16,
    pub percent: u8,
}
impl crate::Specific for DPT_225_2 {
    const DPT: DPT = DPT::new(225u16, Some(2u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, self.timeperiod).unwrap();
        writer.write(8u8 as u32, self.percent).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_225_2 {
                timeperiod: reader.read(16u8 as u32)?,
                percent: reader.read(8u8 as u32)?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_225_2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.timeperiod, self.percent)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_225_x {
    pub timeperiod: u16,
    pub percent: u8,
}
impl crate::Specific for DPT_225_x {
    const DPT: DPT = DPT::new(225u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, self.timeperiod).unwrap();
        writer.write(8u8 as u32, self.percent).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_225_x {
                timeperiod: reader.read(16u8 as u32)?,
                percent: reader.read(8u8 as u32)?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_225_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.timeperiod, self.percent)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_229_1 {
    pub CountVal: i32,
    pub ValInfField: u8,
    pub reserved: Reserved,
    pub AlarmUnAck: bool,
    pub InAlarm: bool,
    pub Overridden: bool,
    pub Fault: bool,
    pub OutOfService: bool,
}
impl crate::Specific for DPT_229_1 {
    const DPT: DPT = DPT::new(229u16, Some(1u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_signed(32u8 as u32, self.CountVal).unwrap();
        writer.write(8u8 as u32, self.ValInfField).unwrap();
        writer.write(3u8 as u32, 0u32).unwrap();
        writer.write_bit(self.AlarmUnAck).unwrap();
        writer.write_bit(self.InAlarm).unwrap();
        writer.write_bit(self.Overridden).unwrap();
        writer.write_bit(self.Fault).unwrap();
        writer.write_bit(self.OutOfService).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_229_1 {
                CountVal: reader.read(32u8 as u32)?,
                ValInfField: reader.read(8u8 as u32)?,
                reserved: {
                    reader.skip(3u8 as u32)?;
                    Reserved::new()
                },
                AlarmUnAck: reader.read_bit()?,
                InAlarm: reader.read_bit()?,
                Overridden: reader.read_bit()?,
                Fault: reader.read_bit()?,
                OutOfService: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_229_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}/{}/{}", self.CountVal, self.ValInfField, self.reserved,
            self.AlarmUnAck, self.InAlarm, self.Overridden, self.Fault, self.OutOfService
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_229_x {
    pub CountVal: i32,
    pub ValInfField: u8,
    pub reserved: Reserved,
    pub AlarmUnAck: bool,
    pub InAlarm: bool,
    pub Overridden: bool,
    pub Fault: bool,
    pub OutOfService: bool,
}
impl crate::Specific for DPT_229_x {
    const DPT: DPT = DPT::new(229u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_signed(32u8 as u32, self.CountVal).unwrap();
        writer.write(8u8 as u32, self.ValInfField).unwrap();
        writer.write(3u8 as u32, 0u32).unwrap();
        writer.write_bit(self.AlarmUnAck).unwrap();
        writer.write_bit(self.InAlarm).unwrap();
        writer.write_bit(self.Overridden).unwrap();
        writer.write_bit(self.Fault).unwrap();
        writer.write_bit(self.OutOfService).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_229_x {
                CountVal: reader.read(32u8 as u32)?,
                ValInfField: reader.read(8u8 as u32)?,
                reserved: {
                    reader.skip(3u8 as u32)?;
                    Reserved::new()
                },
                AlarmUnAck: reader.read_bit()?,
                InAlarm: reader.read_bit()?,
                Overridden: reader.read_bit()?,
                Fault: reader.read_bit()?,
                OutOfService: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_229_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}/{}/{}", self.CountVal, self.ValInfField, self.reserved,
            self.AlarmUnAck, self.InAlarm, self.Overridden, self.Fault, self.OutOfService
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_230_1000 {
    pub ManufactID: u16,
    pub IdentNumber: u32,
    pub Version: u8,
    pub Medium: u8,
}
impl crate::Specific for DPT_230_1000 {
    const DPT: DPT = DPT::new(230u16, Some(1000u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, self.ManufactID).unwrap();
        writer.write(32u8 as u32, self.IdentNumber).unwrap();
        writer.write(8u8 as u32, self.Version).unwrap();
        writer.write(8u8 as u32, self.Medium).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_230_1000 {
                ManufactID: reader.read(16u8 as u32)?,
                IdentNumber: reader.read(32u8 as u32)?,
                Version: reader.read(8u8 as u32)?,
                Medium: reader.read(8u8 as u32)?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_230_1000 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}", self.ManufactID, self.IdentNumber, self.Version, self
            .Medium
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_230_x {
    pub ManufactID: u16,
    pub IdentNumber: u32,
    pub Version: u8,
    pub Medium: u8,
}
impl crate::Specific for DPT_230_x {
    const DPT: DPT = DPT::new(230u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, self.ManufactID).unwrap();
        writer.write(32u8 as u32, self.IdentNumber).unwrap();
        writer.write(8u8 as u32, self.Version).unwrap();
        writer.write(8u8 as u32, self.Medium).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_230_x {
                ManufactID: reader.read(16u8 as u32)?,
                IdentNumber: reader.read(32u8 as u32)?,
                Version: reader.read(8u8 as u32)?,
                Medium: reader.read(8u8 as u32)?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_230_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}", self.ManufactID, self.IdentNumber, self.Version, self
            .Medium
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_232_600 {
    pub R: u8,
    pub G: u8,
    pub B: u8,
}
impl crate::Specific for DPT_232_600 {
    const DPT: DPT = DPT::new(232u16, Some(600u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.R).unwrap();
        writer.write(8u8 as u32, self.G).unwrap();
        writer.write(8u8 as u32, self.B).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_232_600 {
                R: reader.read(8u8 as u32)?,
                G: reader.read(8u8 as u32)?,
                B: reader.read(8u8 as u32)?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_232_600 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}/{}", self.R, self.G, self.B)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_232_x {
    pub R: u8,
    pub G: u8,
    pub B: u8,
}
impl crate::Specific for DPT_232_x {
    const DPT: DPT = DPT::new(232u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.R).unwrap();
        writer.write(8u8 as u32, self.G).unwrap();
        writer.write(8u8 as u32, self.B).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_232_x {
                R: reader.read(8u8 as u32)?,
                G: reader.read(8u8 as u32)?,
                B: reader.read(8u8 as u32)?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_232_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}/{}", self.R, self.G, self.B)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_234_1(pub String);
impl crate::Specific for DPT_234_1 {
    const DPT: DPT = DPT::new(234u16, Some(1u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        let mut buffer = vec![0u8; (16u16 / 8) as usize];
        ASCII.encode_to(self.0.as_str(), EncoderTrap::Replace, &mut buffer).unwrap();
        writer.write_bytes(&buffer).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_234_1({
                    let mut value = [0u8; (16u16 / 8) as usize];
                    reader.read_bytes(&mut value)?;
                    ASCII.decode(&value, DecoderTrap::Replace).unwrap()
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_234_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct DPT_234_x(pub String);
impl crate::Specific for DPT_234_x {
    const DPT: DPT = DPT::new(234u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        let mut buffer = vec![0u8; (16u16 / 8) as usize];
        ASCII.encode_to(self.0.as_str(), EncoderTrap::Replace, &mut buffer).unwrap();
        writer.write_bytes(&buffer).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(
                DPT_234_x({
                    let mut value = [0u8; (16u16 / 8) as usize];
                    reader.read_bytes(&mut value)?;
                    ASCII.decode(&value, DecoderTrap::Replace).unwrap()
                }),
            )
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_234_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_235_1 {
    pub ActiveElectricalEnergy: i32,
    pub Tariff: u8,
    pub reserved: Reserved,
    pub ElectricalEngergyValidity: bool,
    pub TariffValidity: bool,
}
impl crate::Specific for DPT_235_1 {
    const DPT: DPT = DPT::new(235u16, Some(1u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_signed(32u8 as u32, self.ActiveElectricalEnergy).unwrap();
        writer.write(8u8 as u32, self.Tariff).unwrap();
        writer.write(6u8 as u32, 0u32).unwrap();
        writer.write_bit(self.ElectricalEngergyValidity).unwrap();
        writer.write_bit(self.TariffValidity).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_235_1 {
                ActiveElectricalEnergy: reader.read(32u8 as u32)?,
                Tariff: reader.read(8u8 as u32)?,
                reserved: {
                    reader.skip(6u8 as u32)?;
                    Reserved::new()
                },
                ElectricalEngergyValidity: reader.read_bit()?,
                TariffValidity: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_235_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}", self.ActiveElectricalEnergy, self.Tariff, self.reserved,
            self.ElectricalEngergyValidity, self.TariffValidity
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_235_x {
    pub ActiveElectricalEnergy: i32,
    pub Tariff: u8,
    pub reserved: Reserved,
    pub ElectricalEngergyValidity: bool,
    pub TariffValidity: bool,
}
impl crate::Specific for DPT_235_x {
    const DPT: DPT = DPT::new(235u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_signed(32u8 as u32, self.ActiveElectricalEnergy).unwrap();
        writer.write(8u8 as u32, self.Tariff).unwrap();
        writer.write(6u8 as u32, 0u32).unwrap();
        writer.write_bit(self.ElectricalEngergyValidity).unwrap();
        writer.write_bit(self.TariffValidity).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_235_x {
                ActiveElectricalEnergy: reader.read(32u8 as u32)?,
                Tariff: reader.read(8u8 as u32)?,
                reserved: {
                    reader.skip(6u8 as u32)?;
                    Reserved::new()
                },
                ElectricalEngergyValidity: reader.read_bit()?,
                TariffValidity: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_235_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}", self.ActiveElectricalEnergy, self.Tariff, self.reserved,
            self.ElectricalEngergyValidity, self.TariffValidity
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_236_1 {
    pub deactivationofpriority: bool,
    pub prioritylevel: u8,
    pub modelevel: u8,
}
impl crate::Specific for DPT_236_1 {
    const DPT: DPT = DPT::new(236u16, Some(1u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_bit(self.deactivationofpriority).unwrap();
        writer.write(3u8 as u32, self.prioritylevel).unwrap();
        writer.write(4u8 as u32, self.modelevel).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_236_1 {
                deactivationofpriority: reader.read_bit()?,
                prioritylevel: reader.read(3u8 as u32)?,
                modelevel: reader.read(4u8 as u32)?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_236_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}", self.deactivationofpriority, self.prioritylevel, self
            .modelevel
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_236_x {
    pub deactivationofpriority: bool,
    pub prioritylevel: u8,
    pub modelevel: u8,
}
impl crate::Specific for DPT_236_x {
    const DPT: DPT = DPT::new(236u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_bit(self.deactivationofpriority).unwrap();
        writer.write(3u8 as u32, self.prioritylevel).unwrap();
        writer.write(4u8 as u32, self.modelevel).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_236_x {
                deactivationofpriority: reader.read_bit()?,
                prioritylevel: reader.read(3u8 as u32)?,
                modelevel: reader.read(4u8 as u32)?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_236_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}", self.deactivationofpriority, self.prioritylevel, self
            .modelevel
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_237_600 {
    pub reserved: Reserved,
    pub Convertorerror: bool,
    pub BallastFailure: bool,
    pub LampFailure: bool,
    pub ReadorResponse: bool,
    pub AddressIndicator: bool,
    pub DALIDeviceAddressorDALIGroupAddress: u8,
}
impl crate::Specific for DPT_237_600 {
    const DPT: DPT = DPT::new(237u16, Some(600u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(5u8 as u32, 0u32).unwrap();
        writer.write_bit(self.Convertorerror).unwrap();
        writer.write_bit(self.BallastFailure).unwrap();
        writer.write_bit(self.LampFailure).unwrap();
        writer.write_bit(self.ReadorResponse).unwrap();
        writer.write_bit(self.AddressIndicator).unwrap();
        writer.write(6u8 as u32, self.DALIDeviceAddressorDALIGroupAddress).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_237_600 {
                reserved: {
                    reader.skip(5u8 as u32)?;
                    Reserved::new()
                },
                Convertorerror: reader.read_bit()?,
                BallastFailure: reader.read_bit()?,
                LampFailure: reader.read_bit()?,
                ReadorResponse: reader.read_bit()?,
                AddressIndicator: reader.read_bit()?,
                DALIDeviceAddressorDALIGroupAddress: reader.read(6u8 as u32)?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_237_600 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}/{}", self.reserved, self.Convertorerror, self
            .BallastFailure, self.LampFailure, self.ReadorResponse, self
            .AddressIndicator, self.DALIDeviceAddressorDALIGroupAddress
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_237_x {
    pub reserved: Reserved,
    pub Convertorerror: bool,
    pub BallastFailure: bool,
    pub LampFailure: bool,
    pub ReadorResponse: bool,
    pub AddressIndicator: bool,
    pub DALIDeviceAddressorDALIGroupAddress: u8,
}
impl crate::Specific for DPT_237_x {
    const DPT: DPT = DPT::new(237u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(5u8 as u32, 0u32).unwrap();
        writer.write_bit(self.Convertorerror).unwrap();
        writer.write_bit(self.BallastFailure).unwrap();
        writer.write_bit(self.LampFailure).unwrap();
        writer.write_bit(self.ReadorResponse).unwrap();
        writer.write_bit(self.AddressIndicator).unwrap();
        writer.write(6u8 as u32, self.DALIDeviceAddressorDALIGroupAddress).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_237_x {
                reserved: {
                    reader.skip(5u8 as u32)?;
                    Reserved::new()
                },
                Convertorerror: reader.read_bit()?,
                BallastFailure: reader.read_bit()?,
                LampFailure: reader.read_bit()?,
                ReadorResponse: reader.read_bit()?,
                AddressIndicator: reader.read_bit()?,
                DALIDeviceAddressorDALIGroupAddress: reader.read(6u8 as u32)?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_237_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}/{}", self.reserved, self.Convertorerror, self
            .BallastFailure, self.LampFailure, self.ReadorResponse, self
            .AddressIndicator, self.DALIDeviceAddressorDALIGroupAddress
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_238_600 {
    pub BallastFailure: bool,
    pub LampFailure: bool,
    pub DeviceAddress: u8,
}
impl crate::Specific for DPT_238_600 {
    const DPT: DPT = DPT::new(238u16, Some(600u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_bit(self.BallastFailure).unwrap();
        writer.write_bit(self.LampFailure).unwrap();
        writer.write(6u8 as u32, self.DeviceAddress).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_238_600 {
                BallastFailure: reader.read_bit()?,
                LampFailure: reader.read_bit()?,
                DeviceAddress: reader.read(6u8 as u32)?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_238_600 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}/{}", self.BallastFailure, self.LampFailure, self.DeviceAddress)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_238_x {
    pub BallastFailure: bool,
    pub LampFailure: bool,
    pub DeviceAddress: u8,
}
impl crate::Specific for DPT_238_x {
    const DPT: DPT = DPT::new(238u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write_bit(self.BallastFailure).unwrap();
        writer.write_bit(self.LampFailure).unwrap();
        writer.write(6u8 as u32, self.DeviceAddress).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_238_x {
                BallastFailure: reader.read_bit()?,
                LampFailure: reader.read_bit()?,
                DeviceAddress: reader.read(6u8 as u32)?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_238_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}/{}", self.BallastFailure, self.LampFailure, self.DeviceAddress)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_240_800 {
    pub HeightPosition: u8,
    pub SlatsPosition: u8,
    pub reserved: Reserved,
    pub ValiditySlatsPosition: bool,
    pub ValidityHeightPosition: bool,
}
impl crate::Specific for DPT_240_800 {
    const DPT: DPT = DPT::new(240u16, Some(800u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.HeightPosition).unwrap();
        writer.write(8u8 as u32, self.SlatsPosition).unwrap();
        writer.write(6u8 as u32, 0u32).unwrap();
        writer.write_bit(self.ValiditySlatsPosition).unwrap();
        writer.write_bit(self.ValidityHeightPosition).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_240_800 {
                HeightPosition: reader.read(8u8 as u32)?,
                SlatsPosition: reader.read(8u8 as u32)?,
                reserved: {
                    reader.skip(6u8 as u32)?;
                    Reserved::new()
                },
                ValiditySlatsPosition: reader.read_bit()?,
                ValidityHeightPosition: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_240_800 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}", self.HeightPosition, self.SlatsPosition, self.reserved,
            self.ValiditySlatsPosition, self.ValidityHeightPosition
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_240_x {
    pub HeightPosition: u8,
    pub SlatsPosition: u8,
    pub reserved: Reserved,
    pub ValiditySlatsPosition: bool,
    pub ValidityHeightPosition: bool,
}
impl crate::Specific for DPT_240_x {
    const DPT: DPT = DPT::new(240u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.HeightPosition).unwrap();
        writer.write(8u8 as u32, self.SlatsPosition).unwrap();
        writer.write(6u8 as u32, 0u32).unwrap();
        writer.write_bit(self.ValiditySlatsPosition).unwrap();
        writer.write_bit(self.ValidityHeightPosition).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_240_x {
                HeightPosition: reader.read(8u8 as u32)?,
                SlatsPosition: reader.read(8u8 as u32)?,
                reserved: {
                    reader.skip(6u8 as u32)?;
                    Reserved::new()
                },
                ValiditySlatsPosition: reader.read_bit()?,
                ValidityHeightPosition: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_240_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}", self.HeightPosition, self.SlatsPosition, self.reserved,
            self.ValiditySlatsPosition, self.ValidityHeightPosition
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_241_800 {
    pub HeightPosition: u8,
    pub SlatsPosition: u8,
    pub Validityslatspos: bool,
    pub Validityheightpos: bool,
    pub reserved: Reserved,
    pub Generalfailureoftheactuatororthedrive: bool,
    pub Actuatorsetvalueislocallyoverriddenegviaalocaluserinterface: bool,
    pub MovementislockedegbyDeviceLockedinput: bool,
    pub UpdownpositionisforcedbyMoveUpDownForcedinput: bool,
    pub AtleastoneoftheinputsWindRainFrostAlarmisinalarm: bool,
    pub Restrictionofslatsheightposposcannotbereached: bool,
    pub Restrictionoftargetheightposposcannotbereached: bool,
    pub Targetposdrive: bool,
    pub Lowerpredefposreachedtypheightslatsangle: bool,
    pub Lowerendposreached: bool,
    pub Upperendposreached: bool,
}
impl crate::Specific for DPT_241_800 {
    const DPT: DPT = DPT::new(241u16, Some(800u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.HeightPosition).unwrap();
        writer.write(8u8 as u32, self.SlatsPosition).unwrap();
        writer.write_bit(self.Validityslatspos).unwrap();
        writer.write_bit(self.Validityheightpos).unwrap();
        writer.write(3u8 as u32, 0u32).unwrap();
        writer.write_bit(self.Generalfailureoftheactuatororthedrive).unwrap();
        writer
            .write_bit(self.Actuatorsetvalueislocallyoverriddenegviaalocaluserinterface)
            .unwrap();
        writer.write_bit(self.MovementislockedegbyDeviceLockedinput).unwrap();
        writer.write_bit(self.UpdownpositionisforcedbyMoveUpDownForcedinput).unwrap();
        writer.write_bit(self.AtleastoneoftheinputsWindRainFrostAlarmisinalarm).unwrap();
        writer.write_bit(self.Restrictionofslatsheightposposcannotbereached).unwrap();
        writer.write_bit(self.Restrictionoftargetheightposposcannotbereached).unwrap();
        writer.write_bit(self.Targetposdrive).unwrap();
        writer.write_bit(self.Lowerpredefposreachedtypheightslatsangle).unwrap();
        writer.write_bit(self.Lowerendposreached).unwrap();
        writer.write_bit(self.Upperendposreached).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_241_800 {
                HeightPosition: reader.read(8u8 as u32)?,
                SlatsPosition: reader.read(8u8 as u32)?,
                Validityslatspos: reader.read_bit()?,
                Validityheightpos: reader.read_bit()?,
                reserved: {
                    reader.skip(3u8 as u32)?;
                    Reserved::new()
                },
                Generalfailureoftheactuatororthedrive: reader.read_bit()?,
                Actuatorsetvalueislocallyoverriddenegviaalocaluserinterface: reader
                    .read_bit()?,
                MovementislockedegbyDeviceLockedinput: reader.read_bit()?,
                UpdownpositionisforcedbyMoveUpDownForcedinput: reader.read_bit()?,
                AtleastoneoftheinputsWindRainFrostAlarmisinalarm: reader.read_bit()?,
                Restrictionofslatsheightposposcannotbereached: reader.read_bit()?,
                Restrictionoftargetheightposposcannotbereached: reader.read_bit()?,
                Targetposdrive: reader.read_bit()?,
                Lowerpredefposreachedtypheightslatsangle: reader.read_bit()?,
                Lowerendposreached: reader.read_bit()?,
                Upperendposreached: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_241_800 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}", self.HeightPosition,
            self.SlatsPosition, self.Validityslatspos, self.Validityheightpos, self
            .reserved, self.Generalfailureoftheactuatororthedrive, self
            .Actuatorsetvalueislocallyoverriddenegviaalocaluserinterface, self
            .MovementislockedegbyDeviceLockedinput, self
            .UpdownpositionisforcedbyMoveUpDownForcedinput, self
            .AtleastoneoftheinputsWindRainFrostAlarmisinalarm, self
            .Restrictionofslatsheightposposcannotbereached, self
            .Restrictionoftargetheightposposcannotbereached, self.Targetposdrive, self
            .Lowerpredefposreachedtypheightslatsangle, self.Lowerendposreached, self
            .Upperendposreached
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_241_x {
    pub HeightPosition: u8,
    pub SlatsPosition: u8,
    pub Validityslatspos: bool,
    pub Validityheightpos: bool,
    pub reserved: Reserved,
    pub Generalfailureoftheactuatororthedrive: bool,
    pub Actuatorsetvalueislocallyoverriddenegviaalocaluserinterface: bool,
    pub MovementislockedegbyDeviceLockedinput: bool,
    pub UpdownpositionisforcedbyMoveUpDownForcedinput: bool,
    pub AtleastoneoftheinputsWindRainFrostAlarmisinalarm: bool,
    pub Restrictionofslatsheightposposcannotbereached: bool,
    pub Restrictionoftargetheightposposcannotbereached: bool,
    pub Targetposdrive: bool,
    pub Lowerpredefposreachedtypheightslatsangle: bool,
    pub Lowerendposreached: bool,
    pub Upperendposreached: bool,
}
impl crate::Specific for DPT_241_x {
    const DPT: DPT = DPT::new(241u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.HeightPosition).unwrap();
        writer.write(8u8 as u32, self.SlatsPosition).unwrap();
        writer.write_bit(self.Validityslatspos).unwrap();
        writer.write_bit(self.Validityheightpos).unwrap();
        writer.write(3u8 as u32, 0u32).unwrap();
        writer.write_bit(self.Generalfailureoftheactuatororthedrive).unwrap();
        writer
            .write_bit(self.Actuatorsetvalueislocallyoverriddenegviaalocaluserinterface)
            .unwrap();
        writer.write_bit(self.MovementislockedegbyDeviceLockedinput).unwrap();
        writer.write_bit(self.UpdownpositionisforcedbyMoveUpDownForcedinput).unwrap();
        writer.write_bit(self.AtleastoneoftheinputsWindRainFrostAlarmisinalarm).unwrap();
        writer.write_bit(self.Restrictionofslatsheightposposcannotbereached).unwrap();
        writer.write_bit(self.Restrictionoftargetheightposposcannotbereached).unwrap();
        writer.write_bit(self.Targetposdrive).unwrap();
        writer.write_bit(self.Lowerpredefposreachedtypheightslatsangle).unwrap();
        writer.write_bit(self.Lowerendposreached).unwrap();
        writer.write_bit(self.Upperendposreached).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_241_x {
                HeightPosition: reader.read(8u8 as u32)?,
                SlatsPosition: reader.read(8u8 as u32)?,
                Validityslatspos: reader.read_bit()?,
                Validityheightpos: reader.read_bit()?,
                reserved: {
                    reader.skip(3u8 as u32)?;
                    Reserved::new()
                },
                Generalfailureoftheactuatororthedrive: reader.read_bit()?,
                Actuatorsetvalueislocallyoverriddenegviaalocaluserinterface: reader
                    .read_bit()?,
                MovementislockedegbyDeviceLockedinput: reader.read_bit()?,
                UpdownpositionisforcedbyMoveUpDownForcedinput: reader.read_bit()?,
                AtleastoneoftheinputsWindRainFrostAlarmisinalarm: reader.read_bit()?,
                Restrictionofslatsheightposposcannotbereached: reader.read_bit()?,
                Restrictionoftargetheightposposcannotbereached: reader.read_bit()?,
                Targetposdrive: reader.read_bit()?,
                Lowerpredefposreachedtypheightslatsangle: reader.read_bit()?,
                Lowerendposreached: reader.read_bit()?,
                Upperendposreached: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_241_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}", self.HeightPosition,
            self.SlatsPosition, self.Validityslatspos, self.Validityheightpos, self
            .reserved, self.Generalfailureoftheactuatororthedrive, self
            .Actuatorsetvalueislocallyoverriddenegviaalocaluserinterface, self
            .MovementislockedegbyDeviceLockedinput, self
            .UpdownpositionisforcedbyMoveUpDownForcedinput, self
            .AtleastoneoftheinputsWindRainFrostAlarmisinalarm, self
            .Restrictionofslatsheightposposcannotbereached, self
            .Restrictionoftargetheightposposcannotbereached, self.Targetposdrive, self
            .Lowerpredefposreachedtypheightslatsangle, self.Lowerendposreached, self
            .Upperendposreached
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_242_600 {
    pub xaxis: u16,
    pub yaxis: u16,
    pub brightness: u8,
    pub reserved: Reserved,
    pub Validityxy: bool,
    pub Validitybrightness: bool,
}
impl crate::Specific for DPT_242_600 {
    const DPT: DPT = DPT::new(242u16, Some(600u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, self.xaxis).unwrap();
        writer.write(16u8 as u32, self.yaxis).unwrap();
        writer.write(8u8 as u32, self.brightness).unwrap();
        writer.write(6u8 as u32, 0u32).unwrap();
        writer.write_bit(self.Validityxy).unwrap();
        writer.write_bit(self.Validitybrightness).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_242_600 {
                xaxis: reader.read(16u8 as u32)?,
                yaxis: reader.read(16u8 as u32)?,
                brightness: reader.read(8u8 as u32)?,
                reserved: {
                    reader.skip(6u8 as u32)?;
                    Reserved::new()
                },
                Validityxy: reader.read_bit()?,
                Validitybrightness: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_242_600 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}", self.xaxis, self.yaxis, self.brightness, self
            .reserved, self.Validityxy, self.Validitybrightness
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_242_x {
    pub xaxis: u16,
    pub yaxis: u16,
    pub brightness: u8,
    pub reserved: Reserved,
    pub Validityxy: bool,
    pub Validitybrightness: bool,
}
impl crate::Specific for DPT_242_x {
    const DPT: DPT = DPT::new(242u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, self.xaxis).unwrap();
        writer.write(16u8 as u32, self.yaxis).unwrap();
        writer.write(8u8 as u32, self.brightness).unwrap();
        writer.write(6u8 as u32, 0u32).unwrap();
        writer.write_bit(self.Validityxy).unwrap();
        writer.write_bit(self.Validitybrightness).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_242_x {
                xaxis: reader.read(16u8 as u32)?,
                yaxis: reader.read(16u8 as u32)?,
                brightness: reader.read(8u8 as u32)?,
                reserved: {
                    reader.skip(6u8 as u32)?;
                    Reserved::new()
                },
                Validityxy: reader.read_bit()?,
                Validitybrightness: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_242_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}", self.xaxis, self.yaxis, self.brightness, self
            .reserved, self.Validityxy, self.Validitybrightness
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_244_600 {
    pub ConverterModeaccordingtotheDALIconverterstatemachine: u8,
    pub reserved: Reserved,
    pub HS: bool,
    pub HS2: bool,
    pub FunctionTestPending: u8,
    pub DurationTestPending: u8,
    pub PartialDurationTestPending: u8,
    pub ConverterFailure: u8,
}
impl crate::Specific for DPT_244_600 {
    const DPT: DPT = DPT::new(244u16, Some(600u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer
            .write(4u8 as u32, self.ConverterModeaccordingtotheDALIconverterstatemachine)
            .unwrap();
        writer.write(2u8 as u32, 0u32).unwrap();
        writer.write_bit(self.HS).unwrap();
        writer.write_bit(self.HS2).unwrap();
        writer.write(2u8 as u32, self.FunctionTestPending).unwrap();
        writer.write(2u8 as u32, self.DurationTestPending).unwrap();
        writer.write(2u8 as u32, self.PartialDurationTestPending).unwrap();
        writer.write(2u8 as u32, self.ConverterFailure).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_244_600 {
                ConverterModeaccordingtotheDALIconverterstatemachine: reader
                    .read(4u8 as u32)?,
                reserved: {
                    reader.skip(2u8 as u32)?;
                    Reserved::new()
                },
                HS: reader.read_bit()?,
                HS2: reader.read_bit()?,
                FunctionTestPending: reader.read(2u8 as u32)?,
                DurationTestPending: reader.read(2u8 as u32)?,
                PartialDurationTestPending: reader.read(2u8 as u32)?,
                ConverterFailure: reader.read(2u8 as u32)?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_244_600 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}/{}/{}", self
            .ConverterModeaccordingtotheDALIconverterstatemachine, self.reserved, self
            .HS, self.HS2, self.FunctionTestPending, self.DurationTestPending, self
            .PartialDurationTestPending, self.ConverterFailure
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_244_x {
    pub ConverterModeaccordingtotheDALIconverterstatemachine: u8,
    pub reserved: Reserved,
    pub HS: bool,
    pub HS2: bool,
    pub FunctionTestPending: u8,
    pub DurationTestPending: u8,
    pub PartialDurationTestPending: u8,
    pub ConverterFailure: u8,
}
impl crate::Specific for DPT_244_x {
    const DPT: DPT = DPT::new(244u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer
            .write(4u8 as u32, self.ConverterModeaccordingtotheDALIconverterstatemachine)
            .unwrap();
        writer.write(2u8 as u32, 0u32).unwrap();
        writer.write_bit(self.HS).unwrap();
        writer.write_bit(self.HS2).unwrap();
        writer.write(2u8 as u32, self.FunctionTestPending).unwrap();
        writer.write(2u8 as u32, self.DurationTestPending).unwrap();
        writer.write(2u8 as u32, self.PartialDurationTestPending).unwrap();
        writer.write(2u8 as u32, self.ConverterFailure).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_244_x {
                ConverterModeaccordingtotheDALIconverterstatemachine: reader
                    .read(4u8 as u32)?,
                reserved: {
                    reader.skip(2u8 as u32)?;
                    Reserved::new()
                },
                HS: reader.read_bit()?,
                HS2: reader.read_bit()?,
                FunctionTestPending: reader.read(2u8 as u32)?,
                DurationTestPending: reader.read(2u8 as u32)?,
                PartialDurationTestPending: reader.read(2u8 as u32)?,
                ConverterFailure: reader.read(2u8 as u32)?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_244_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}/{}/{}", self
            .ConverterModeaccordingtotheDALIconverterstatemachine, self.reserved, self
            .HS, self.HS2, self.FunctionTestPending, self.DurationTestPending, self
            .PartialDurationTestPending, self.ConverterFailure
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_245_600 {
    pub LTRF: u8,
    pub LTRD: u8,
    pub LTRP: u8,
    pub reserved: Reserved,
    pub SF: u8,
    pub SD: u8,
    pub SP: u8,
    pub reserved2: Reserved,
    pub LDTR: u16,
    pub LPDTR: u8,
}
impl crate::Specific for DPT_245_600 {
    const DPT: DPT = DPT::new(245u16, Some(600u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(4u8 as u32, self.LTRF).unwrap();
        writer.write(4u8 as u32, self.LTRD).unwrap();
        writer.write(4u8 as u32, self.LTRP).unwrap();
        writer.write(4u8 as u32, 0u32).unwrap();
        writer.write(2u8 as u32, self.SF).unwrap();
        writer.write(2u8 as u32, self.SD).unwrap();
        writer.write(2u8 as u32, self.SP).unwrap();
        writer.write(2u8 as u32, 0u32).unwrap();
        writer.write(16u8 as u32, self.LDTR).unwrap();
        writer.write(8u8 as u32, self.LPDTR).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_245_600 {
                LTRF: reader.read(4u8 as u32)?,
                LTRD: reader.read(4u8 as u32)?,
                LTRP: reader.read(4u8 as u32)?,
                reserved: {
                    reader.skip(4u8 as u32)?;
                    Reserved::new()
                },
                SF: reader.read(2u8 as u32)?,
                SD: reader.read(2u8 as u32)?,
                SP: reader.read(2u8 as u32)?,
                reserved2: {
                    reader.skip(2u8 as u32)?;
                    Reserved::new()
                },
                LDTR: reader.read(16u8 as u32)?,
                LPDTR: reader.read(8u8 as u32)?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_245_600 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}/{}/{}/{}/{}", self.LTRF, self.LTRD, self.LTRP, self
            .reserved, self.SF, self.SD, self.SP, self.reserved2, self.LDTR, self.LPDTR
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_245_x {
    pub LTRF: u8,
    pub LTRD: u8,
    pub LTRP: u8,
    pub reserved: Reserved,
    pub SF: u8,
    pub SD: u8,
    pub SP: u8,
    pub reserved2: Reserved,
    pub LDTR: u16,
    pub LPDTR: u8,
}
impl crate::Specific for DPT_245_x {
    const DPT: DPT = DPT::new(245u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(4u8 as u32, self.LTRF).unwrap();
        writer.write(4u8 as u32, self.LTRD).unwrap();
        writer.write(4u8 as u32, self.LTRP).unwrap();
        writer.write(4u8 as u32, 0u32).unwrap();
        writer.write(2u8 as u32, self.SF).unwrap();
        writer.write(2u8 as u32, self.SD).unwrap();
        writer.write(2u8 as u32, self.SP).unwrap();
        writer.write(2u8 as u32, 0u32).unwrap();
        writer.write(16u8 as u32, self.LDTR).unwrap();
        writer.write(8u8 as u32, self.LPDTR).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_245_x {
                LTRF: reader.read(4u8 as u32)?,
                LTRD: reader.read(4u8 as u32)?,
                LTRP: reader.read(4u8 as u32)?,
                reserved: {
                    reader.skip(4u8 as u32)?;
                    Reserved::new()
                },
                SF: reader.read(2u8 as u32)?,
                SD: reader.read(2u8 as u32)?,
                SP: reader.read(2u8 as u32)?,
                reserved2: {
                    reader.skip(2u8 as u32)?;
                    Reserved::new()
                },
                LDTR: reader.read(16u8 as u32)?,
                LPDTR: reader.read(8u8 as u32)?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_245_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}/{}/{}/{}/{}", self.LTRF, self.LTRD, self.LTRP, self
            .reserved, self.SF, self.SD, self.SP, self.reserved2, self.LDTR, self.LPDTR
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_246_600 {
    pub reserved: Reserved,
    pub BatteryFullyCharged: bool,
    pub BatteryDurationFailure: bool,
    pub BatteryFailure: bool,
    pub BatteryChargeLevel: u8,
}
impl crate::Specific for DPT_246_600 {
    const DPT: DPT = DPT::new(246u16, Some(600u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(5u8 as u32, 0u32).unwrap();
        writer.write_bit(self.BatteryFullyCharged).unwrap();
        writer.write_bit(self.BatteryDurationFailure).unwrap();
        writer.write_bit(self.BatteryFailure).unwrap();
        writer.write(8u8 as u32, self.BatteryChargeLevel).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_246_600 {
                reserved: {
                    reader.skip(5u8 as u32)?;
                    Reserved::new()
                },
                BatteryFullyCharged: reader.read_bit()?,
                BatteryDurationFailure: reader.read_bit()?,
                BatteryFailure: reader.read_bit()?,
                BatteryChargeLevel: reader.read(8u8 as u32)?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_246_600 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}", self.reserved, self.BatteryFullyCharged, self
            .BatteryDurationFailure, self.BatteryFailure, self.BatteryChargeLevel
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_246_x {
    pub reserved: Reserved,
    pub BatteryFullyCharged: bool,
    pub BatteryDurationFailure: bool,
    pub BatteryFailure: bool,
    pub BatteryChargeLevel: u8,
}
impl crate::Specific for DPT_246_x {
    const DPT: DPT = DPT::new(246u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(5u8 as u32, 0u32).unwrap();
        writer.write_bit(self.BatteryFullyCharged).unwrap();
        writer.write_bit(self.BatteryDurationFailure).unwrap();
        writer.write_bit(self.BatteryFailure).unwrap();
        writer.write(8u8 as u32, self.BatteryChargeLevel).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_246_x {
                reserved: {
                    reader.skip(5u8 as u32)?;
                    Reserved::new()
                },
                BatteryFullyCharged: reader.read_bit()?,
                BatteryDurationFailure: reader.read_bit()?,
                BatteryFailure: reader.read_bit()?,
                BatteryChargeLevel: reader.read(8u8 as u32)?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_246_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}", self.reserved, self.BatteryFullyCharged, self
            .BatteryDurationFailure, self.BatteryFailure, self.BatteryChargeLevel
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_249_600 {
    pub int: u16,
    pub int2: u16,
    pub int3: u8,
    pub reserved: Reserved,
    pub validityoftheTimePeriod: bool,
    pub validityoftheAbsoluteColourTemperature: bool,
    pub validityoftheabsolutebrightness: bool,
}
impl crate::Specific for DPT_249_600 {
    const DPT: DPT = DPT::new(249u16, Some(600u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, self.int).unwrap();
        writer.write(16u8 as u32, self.int2).unwrap();
        writer.write(8u8 as u32, self.int3).unwrap();
        writer.write(5u8 as u32, 0u32).unwrap();
        writer.write_bit(self.validityoftheTimePeriod).unwrap();
        writer.write_bit(self.validityoftheAbsoluteColourTemperature).unwrap();
        writer.write_bit(self.validityoftheabsolutebrightness).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_249_600 {
                int: reader.read(16u8 as u32)?,
                int2: reader.read(16u8 as u32)?,
                int3: reader.read(8u8 as u32)?,
                reserved: {
                    reader.skip(5u8 as u32)?;
                    Reserved::new()
                },
                validityoftheTimePeriod: reader.read_bit()?,
                validityoftheAbsoluteColourTemperature: reader.read_bit()?,
                validityoftheabsolutebrightness: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_249_600 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}/{}", self.int, self.int2, self.int3, self.reserved,
            self.validityoftheTimePeriod, self.validityoftheAbsoluteColourTemperature,
            self.validityoftheabsolutebrightness
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_249_x {
    pub int: u16,
    pub int2: u16,
    pub int3: u8,
    pub reserved: Reserved,
    pub validityoftheTimePeriod: bool,
    pub validityoftheAbsoluteColourTemperature: bool,
    pub validityoftheabsolutebrightness: bool,
}
impl crate::Specific for DPT_249_x {
    const DPT: DPT = DPT::new(249u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(16u8 as u32, self.int).unwrap();
        writer.write(16u8 as u32, self.int2).unwrap();
        writer.write(8u8 as u32, self.int3).unwrap();
        writer.write(5u8 as u32, 0u32).unwrap();
        writer.write_bit(self.validityoftheTimePeriod).unwrap();
        writer.write_bit(self.validityoftheAbsoluteColourTemperature).unwrap();
        writer.write_bit(self.validityoftheabsolutebrightness).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_249_x {
                int: reader.read(16u8 as u32)?,
                int2: reader.read(16u8 as u32)?,
                int3: reader.read(8u8 as u32)?,
                reserved: {
                    reader.skip(5u8 as u32)?;
                    Reserved::new()
                },
                validityoftheTimePeriod: reader.read_bit()?,
                validityoftheAbsoluteColourTemperature: reader.read_bit()?,
                validityoftheabsolutebrightness: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_249_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}/{}", self.int, self.int2, self.int3, self.reserved,
            self.validityoftheTimePeriod, self.validityoftheAbsoluteColourTemperature,
            self.validityoftheabsolutebrightness
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_250_600 {
    pub reserved: Reserved,
    pub CCT: bool,
    pub StepCodeColourTemperature: u8,
    pub reserved2: Reserved,
    pub CB: bool,
    pub StepCodeBrightness: u8,
    pub reserved3: Reserved,
    pub CCTandStepCodeColourValidity: bool,
    pub CBandStepCodeBrightnessValidity: bool,
}
impl crate::Specific for DPT_250_600 {
    const DPT: DPT = DPT::new(250u16, Some(600u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(4u8 as u32, 0u32).unwrap();
        writer.write_bit(self.CCT).unwrap();
        writer.write(3u8 as u32, self.StepCodeColourTemperature).unwrap();
        writer.write(4u8 as u32, 0u32).unwrap();
        writer.write_bit(self.CB).unwrap();
        writer.write(3u8 as u32, self.StepCodeBrightness).unwrap();
        writer.write(6u8 as u32, 0u32).unwrap();
        writer.write_bit(self.CCTandStepCodeColourValidity).unwrap();
        writer.write_bit(self.CBandStepCodeBrightnessValidity).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_250_600 {
                reserved: {
                    reader.skip(4u8 as u32)?;
                    Reserved::new()
                },
                CCT: reader.read_bit()?,
                StepCodeColourTemperature: reader.read(3u8 as u32)?,
                reserved2: {
                    reader.skip(4u8 as u32)?;
                    Reserved::new()
                },
                CB: reader.read_bit()?,
                StepCodeBrightness: reader.read(3u8 as u32)?,
                reserved3: {
                    reader.skip(6u8 as u32)?;
                    Reserved::new()
                },
                CCTandStepCodeColourValidity: reader.read_bit()?,
                CBandStepCodeBrightnessValidity: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_250_600 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}/{}/{}/{}", self.reserved, self.CCT, self
            .StepCodeColourTemperature, self.reserved2, self.CB, self.StepCodeBrightness,
            self.reserved3, self.CCTandStepCodeColourValidity, self
            .CBandStepCodeBrightnessValidity
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_250_x {
    pub reserved: Reserved,
    pub CCT: bool,
    pub StepCodeColourTemperature: u8,
    pub reserved2: Reserved,
    pub CB: bool,
    pub StepCodeBrightness: u8,
    pub reserved3: Reserved,
    pub CCTandStepCodeColourValidity: bool,
    pub CBandStepCodeBrightnessValidity: bool,
}
impl crate::Specific for DPT_250_x {
    const DPT: DPT = DPT::new(250u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(4u8 as u32, 0u32).unwrap();
        writer.write_bit(self.CCT).unwrap();
        writer.write(3u8 as u32, self.StepCodeColourTemperature).unwrap();
        writer.write(4u8 as u32, 0u32).unwrap();
        writer.write_bit(self.CB).unwrap();
        writer.write(3u8 as u32, self.StepCodeBrightness).unwrap();
        writer.write(6u8 as u32, 0u32).unwrap();
        writer.write_bit(self.CCTandStepCodeColourValidity).unwrap();
        writer.write_bit(self.CBandStepCodeBrightnessValidity).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_250_x {
                reserved: {
                    reader.skip(4u8 as u32)?;
                    Reserved::new()
                },
                CCT: reader.read_bit()?,
                StepCodeColourTemperature: reader.read(3u8 as u32)?,
                reserved2: {
                    reader.skip(4u8 as u32)?;
                    Reserved::new()
                },
                CB: reader.read_bit()?,
                StepCodeBrightness: reader.read(3u8 as u32)?,
                reserved3: {
                    reader.skip(6u8 as u32)?;
                    Reserved::new()
                },
                CCTandStepCodeColourValidity: reader.read_bit()?,
                CBandStepCodeBrightnessValidity: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_250_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}/{}/{}/{}", self.reserved, self.CCT, self
            .StepCodeColourTemperature, self.reserved2, self.CB, self.StepCodeBrightness,
            self.reserved3, self.CCTandStepCodeColourValidity, self
            .CBandStepCodeBrightnessValidity
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_251_600 {
    pub ColourLevelRed: u8,
    pub ColourLevelGreen: u8,
    pub ColourLevelBlue: u8,
    pub ColourLevelWhite: u8,
    pub reserved: Reserved,
    pub reserved2: Reserved,
    pub mR: bool,
    pub mG: bool,
    pub mB: bool,
    pub mW: bool,
}
impl crate::Specific for DPT_251_600 {
    const DPT: DPT = DPT::new(251u16, Some(600u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.ColourLevelRed).unwrap();
        writer.write(8u8 as u32, self.ColourLevelGreen).unwrap();
        writer.write(8u8 as u32, self.ColourLevelBlue).unwrap();
        writer.write(8u8 as u32, self.ColourLevelWhite).unwrap();
        writer.write(8u8 as u32, 0u32).unwrap();
        writer.write(4u8 as u32, 0u32).unwrap();
        writer.write_bit(self.mR).unwrap();
        writer.write_bit(self.mG).unwrap();
        writer.write_bit(self.mB).unwrap();
        writer.write_bit(self.mW).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_251_600 {
                ColourLevelRed: reader.read(8u8 as u32)?,
                ColourLevelGreen: reader.read(8u8 as u32)?,
                ColourLevelBlue: reader.read(8u8 as u32)?,
                ColourLevelWhite: reader.read(8u8 as u32)?,
                reserved: {
                    reader.skip(8u8 as u32)?;
                    Reserved::new()
                },
                reserved2: {
                    reader.skip(4u8 as u32)?;
                    Reserved::new()
                },
                mR: reader.read_bit()?,
                mG: reader.read_bit()?,
                mB: reader.read_bit()?,
                mW: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_251_600 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}/{}/{}/{}/{}", self.ColourLevelRed, self
            .ColourLevelGreen, self.ColourLevelBlue, self.ColourLevelWhite, self
            .reserved, self.reserved2, self.mR, self.mG, self.mB, self.mW
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_251_x {
    pub ColourLevelRed: u8,
    pub ColourLevelGreen: u8,
    pub ColourLevelBlue: u8,
    pub ColourLevelWhite: u8,
    pub reserved: Reserved,
    pub reserved2: Reserved,
    pub mR: bool,
    pub mG: bool,
    pub mB: bool,
    pub mW: bool,
}
impl crate::Specific for DPT_251_x {
    const DPT: DPT = DPT::new(251u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.ColourLevelRed).unwrap();
        writer.write(8u8 as u32, self.ColourLevelGreen).unwrap();
        writer.write(8u8 as u32, self.ColourLevelBlue).unwrap();
        writer.write(8u8 as u32, self.ColourLevelWhite).unwrap();
        writer.write(8u8 as u32, 0u32).unwrap();
        writer.write(4u8 as u32, 0u32).unwrap();
        writer.write_bit(self.mR).unwrap();
        writer.write_bit(self.mG).unwrap();
        writer.write_bit(self.mB).unwrap();
        writer.write_bit(self.mW).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_251_x {
                ColourLevelRed: reader.read(8u8 as u32)?,
                ColourLevelGreen: reader.read(8u8 as u32)?,
                ColourLevelBlue: reader.read(8u8 as u32)?,
                ColourLevelWhite: reader.read(8u8 as u32)?,
                reserved: {
                    reader.skip(8u8 as u32)?;
                    Reserved::new()
                },
                reserved2: {
                    reader.skip(4u8 as u32)?;
                    Reserved::new()
                },
                mR: reader.read_bit()?,
                mG: reader.read_bit()?,
                mB: reader.read_bit()?,
                mW: reader.read_bit()?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_251_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}/{}/{}/{}/{}", self.ColourLevelRed, self
            .ColourLevelGreen, self.ColourLevelBlue, self.ColourLevelWhite, self
            .reserved, self.reserved2, self.mR, self.mG, self.mB, self.mW
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_252_600 {
    pub reserved: Reserved,
    pub MaskCw: bool,
    pub MaskCb: bool,
    pub MaskCg: bool,
    pub MaskCr: bool,
    pub reserved2: Reserved,
    pub Cw: bool,
    pub StepCodeColourWhite: u8,
    pub reserved3: Reserved,
    pub Cb: bool,
    pub StepCodeColourBlue: u8,
    pub reserved4: Reserved,
    pub Cg: bool,
    pub StepCodeColourGreen: u8,
    pub reserved5: Reserved,
    pub Cr: bool,
    pub StepCodeColourRed: u8,
}
impl crate::Specific for DPT_252_600 {
    const DPT: DPT = DPT::new(252u16, Some(600u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(4u8 as u32, 0u32).unwrap();
        writer.write_bit(self.MaskCw).unwrap();
        writer.write_bit(self.MaskCb).unwrap();
        writer.write_bit(self.MaskCg).unwrap();
        writer.write_bit(self.MaskCr).unwrap();
        writer.write(4u8 as u32, 0u32).unwrap();
        writer.write_bit(self.Cw).unwrap();
        writer.write(3u8 as u32, self.StepCodeColourWhite).unwrap();
        writer.write(4u8 as u32, 0u32).unwrap();
        writer.write_bit(self.Cb).unwrap();
        writer.write(3u8 as u32, self.StepCodeColourBlue).unwrap();
        writer.write(4u8 as u32, 0u32).unwrap();
        writer.write_bit(self.Cg).unwrap();
        writer.write(3u8 as u32, self.StepCodeColourGreen).unwrap();
        writer.write(4u8 as u32, 0u32).unwrap();
        writer.write_bit(self.Cr).unwrap();
        writer.write(3u8 as u32, self.StepCodeColourRed).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_252_600 {
                reserved: {
                    reader.skip(4u8 as u32)?;
                    Reserved::new()
                },
                MaskCw: reader.read_bit()?,
                MaskCb: reader.read_bit()?,
                MaskCg: reader.read_bit()?,
                MaskCr: reader.read_bit()?,
                reserved2: {
                    reader.skip(4u8 as u32)?;
                    Reserved::new()
                },
                Cw: reader.read_bit()?,
                StepCodeColourWhite: reader.read(3u8 as u32)?,
                reserved3: {
                    reader.skip(4u8 as u32)?;
                    Reserved::new()
                },
                Cb: reader.read_bit()?,
                StepCodeColourBlue: reader.read(3u8 as u32)?,
                reserved4: {
                    reader.skip(4u8 as u32)?;
                    Reserved::new()
                },
                Cg: reader.read_bit()?,
                StepCodeColourGreen: reader.read(3u8 as u32)?,
                reserved5: {
                    reader.skip(4u8 as u32)?;
                    Reserved::new()
                },
                Cr: reader.read_bit()?,
                StepCodeColourRed: reader.read(3u8 as u32)?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_252_600 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}", self.reserved, self
            .MaskCw, self.MaskCb, self.MaskCg, self.MaskCr, self.reserved2, self.Cw, self
            .StepCodeColourWhite, self.reserved3, self.Cb, self.StepCodeColourBlue, self
            .reserved4, self.Cg, self.StepCodeColourGreen, self.reserved5, self.Cr, self
            .StepCodeColourRed
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_252_x {
    pub reserved: Reserved,
    pub MaskCw: bool,
    pub MaskCb: bool,
    pub MaskCg: bool,
    pub MaskCr: bool,
    pub reserved2: Reserved,
    pub Cw: bool,
    pub StepCodeColourWhite: u8,
    pub reserved3: Reserved,
    pub Cb: bool,
    pub StepCodeColourBlue: u8,
    pub reserved4: Reserved,
    pub Cg: bool,
    pub StepCodeColourGreen: u8,
    pub reserved5: Reserved,
    pub Cr: bool,
    pub StepCodeColourRed: u8,
}
impl crate::Specific for DPT_252_x {
    const DPT: DPT = DPT::new(252u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(4u8 as u32, 0u32).unwrap();
        writer.write_bit(self.MaskCw).unwrap();
        writer.write_bit(self.MaskCb).unwrap();
        writer.write_bit(self.MaskCg).unwrap();
        writer.write_bit(self.MaskCr).unwrap();
        writer.write(4u8 as u32, 0u32).unwrap();
        writer.write_bit(self.Cw).unwrap();
        writer.write(3u8 as u32, self.StepCodeColourWhite).unwrap();
        writer.write(4u8 as u32, 0u32).unwrap();
        writer.write_bit(self.Cb).unwrap();
        writer.write(3u8 as u32, self.StepCodeColourBlue).unwrap();
        writer.write(4u8 as u32, 0u32).unwrap();
        writer.write_bit(self.Cg).unwrap();
        writer.write(3u8 as u32, self.StepCodeColourGreen).unwrap();
        writer.write(4u8 as u32, 0u32).unwrap();
        writer.write_bit(self.Cr).unwrap();
        writer.write(3u8 as u32, self.StepCodeColourRed).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_252_x {
                reserved: {
                    reader.skip(4u8 as u32)?;
                    Reserved::new()
                },
                MaskCw: reader.read_bit()?,
                MaskCb: reader.read_bit()?,
                MaskCg: reader.read_bit()?,
                MaskCr: reader.read_bit()?,
                reserved2: {
                    reader.skip(4u8 as u32)?;
                    Reserved::new()
                },
                Cw: reader.read_bit()?,
                StepCodeColourWhite: reader.read(3u8 as u32)?,
                reserved3: {
                    reader.skip(4u8 as u32)?;
                    Reserved::new()
                },
                Cb: reader.read_bit()?,
                StepCodeColourBlue: reader.read(3u8 as u32)?,
                reserved4: {
                    reader.skip(4u8 as u32)?;
                    Reserved::new()
                },
                Cg: reader.read_bit()?,
                StepCodeColourGreen: reader.read(3u8 as u32)?,
                reserved5: {
                    reader.skip(4u8 as u32)?;
                    Reserved::new()
                },
                Cr: reader.read_bit()?,
                StepCodeColourRed: reader.read(3u8 as u32)?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_252_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}", self.reserved, self
            .MaskCw, self.MaskCb, self.MaskCg, self.MaskCr, self.reserved2, self.Cw, self
            .StepCodeColourWhite, self.reserved3, self.Cb, self.StepCodeColourBlue, self
            .reserved4, self.Cg, self.StepCodeColourGreen, self.reserved5, self.Cr, self
            .StepCodeColourRed
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_254_600 {
    pub reserved: Reserved,
    pub Cb: bool,
    pub StepCodeColourBlue: u8,
    pub reserved2: Reserved,
    pub Cg: bool,
    pub StepCodeColourGreen: u8,
    pub reserved3: Reserved,
    pub Cr: bool,
    pub StepCodeColourRed: u8,
}
impl crate::Specific for DPT_254_600 {
    const DPT: DPT = DPT::new(254u16, Some(600u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(4u8 as u32, 0u32).unwrap();
        writer.write_bit(self.Cb).unwrap();
        writer.write(3u8 as u32, self.StepCodeColourBlue).unwrap();
        writer.write(4u8 as u32, 0u32).unwrap();
        writer.write_bit(self.Cg).unwrap();
        writer.write(3u8 as u32, self.StepCodeColourGreen).unwrap();
        writer.write(4u8 as u32, 0u32).unwrap();
        writer.write_bit(self.Cr).unwrap();
        writer.write(3u8 as u32, self.StepCodeColourRed).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_254_600 {
                reserved: {
                    reader.skip(4u8 as u32)?;
                    Reserved::new()
                },
                Cb: reader.read_bit()?,
                StepCodeColourBlue: reader.read(3u8 as u32)?,
                reserved2: {
                    reader.skip(4u8 as u32)?;
                    Reserved::new()
                },
                Cg: reader.read_bit()?,
                StepCodeColourGreen: reader.read(3u8 as u32)?,
                reserved3: {
                    reader.skip(4u8 as u32)?;
                    Reserved::new()
                },
                Cr: reader.read_bit()?,
                StepCodeColourRed: reader.read(3u8 as u32)?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_254_600 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}/{}/{}/{}", self.reserved, self.Cb, self
            .StepCodeColourBlue, self.reserved2, self.Cg, self.StepCodeColourGreen, self
            .reserved3, self.Cr, self.StepCodeColourRed
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_254_x {
    pub reserved: Reserved,
    pub Cb: bool,
    pub StepCodeColourBlue: u8,
    pub reserved2: Reserved,
    pub Cg: bool,
    pub StepCodeColourGreen: u8,
    pub reserved3: Reserved,
    pub Cr: bool,
    pub StepCodeColourRed: u8,
}
impl crate::Specific for DPT_254_x {
    const DPT: DPT = DPT::new(254u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(4u8 as u32, 0u32).unwrap();
        writer.write_bit(self.Cb).unwrap();
        writer.write(3u8 as u32, self.StepCodeColourBlue).unwrap();
        writer.write(4u8 as u32, 0u32).unwrap();
        writer.write_bit(self.Cg).unwrap();
        writer.write(3u8 as u32, self.StepCodeColourGreen).unwrap();
        writer.write(4u8 as u32, 0u32).unwrap();
        writer.write_bit(self.Cr).unwrap();
        writer.write(3u8 as u32, self.StepCodeColourRed).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_254_x {
                reserved: {
                    reader.skip(4u8 as u32)?;
                    Reserved::new()
                },
                Cb: reader.read_bit()?,
                StepCodeColourBlue: reader.read(3u8 as u32)?,
                reserved2: {
                    reader.skip(4u8 as u32)?;
                    Reserved::new()
                },
                Cg: reader.read_bit()?,
                StepCodeColourGreen: reader.read(3u8 as u32)?,
                reserved3: {
                    reader.skip(4u8 as u32)?;
                    Reserved::new()
                },
                Cr: reader.read_bit()?,
                StepCodeColourRed: reader.read(3u8 as u32)?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_254_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}/{}/{}/{}", self.reserved, self.Cb, self
            .StepCodeColourBlue, self.reserved2, self.Cg, self.StepCodeColourGreen, self
            .reserved3, self.Cr, self.StepCodeColourRed
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_255_1 {
    pub Longitude: f32,
    pub Latitude: f32,
}
impl crate::Specific for DPT_255_1 {
    const DPT: DPT = DPT::new(255u16, Some(1u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.Longitude)).unwrap();
        writer.write(32u8 as u32, f32::to_bits(self.Latitude)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_255_1 {
                Longitude: {
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                },
                Latitude: {
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                },
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_255_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}/{:.1}", self.Longitude, self.Latitude)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_255_x {
    pub Longitude: f32,
    pub Latitude: f32,
}
impl crate::Specific for DPT_255_x {
    const DPT: DPT = DPT::new(255u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(32u8 as u32, f32::to_bits(self.Longitude)).unwrap();
        writer.write(32u8 as u32, f32::to_bits(self.Latitude)).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_255_x {
                Longitude: {
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                },
                Latitude: {
                    let value: u32 = reader.read(32u8 as u32)?;
                    let value = f32::from_bits(value);
                    value
                },
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_255_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.1}/{:.1}", self.Longitude, self.Latitude)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_275_100 {
    pub roomtemperaturesetpointcomfort: f32,
    pub roomtemperaturesetpointstandby: f32,
    pub roomtemperaturesetpointeconomy: f32,
    pub roomtemperaturesetpointbuildingprotection: f32,
}
impl crate::Specific for DPT_275_100 {
    const DPT: DPT = DPT::new(275u16, Some(100u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer
            .write(16u8 as u32, encode_knxf16(self.roomtemperaturesetpointcomfort))
            .unwrap();
        writer
            .write(16u8 as u32, encode_knxf16(self.roomtemperaturesetpointstandby))
            .unwrap();
        writer
            .write(16u8 as u32, encode_knxf16(self.roomtemperaturesetpointeconomy))
            .unwrap();
        writer
            .write(
                16u8 as u32,
                encode_knxf16(self.roomtemperaturesetpointbuildingprotection),
            )
            .unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_275_100 {
                roomtemperaturesetpointcomfort: {
                    let value: u16 = reader.read(16u8 as u32)?;
                    let value = decode_knxf16(value);
                    value
                },
                roomtemperaturesetpointstandby: {
                    let value: u16 = reader.read(16u8 as u32)?;
                    let value = decode_knxf16(value);
                    value
                },
                roomtemperaturesetpointeconomy: {
                    let value: u16 = reader.read(16u8 as u32)?;
                    let value = decode_knxf16(value);
                    value
                },
                roomtemperaturesetpointbuildingprotection: {
                    let value: u16 = reader.read(16u8 as u32)?;
                    let value = decode_knxf16(value);
                    value
                },
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_275_100 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{:.1}/{:.1}/{:.1}/{:.1}", self.roomtemperaturesetpointcomfort, self
            .roomtemperaturesetpointstandby, self.roomtemperaturesetpointeconomy, self
            .roomtemperaturesetpointbuildingprotection
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_275_101 {
    pub roomtemperaturesetpointshiftcomfort: f32,
    pub roomtemperaturesetpointshiftstandby: f32,
    pub roomtemperaturesetpointshifteconomy: f32,
    pub roomtemperaturesetpointshiftbuildingprotection: f32,
}
impl crate::Specific for DPT_275_101 {
    const DPT: DPT = DPT::new(275u16, Some(101u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer
            .write(16u8 as u32, encode_knxf16(self.roomtemperaturesetpointshiftcomfort))
            .unwrap();
        writer
            .write(16u8 as u32, encode_knxf16(self.roomtemperaturesetpointshiftstandby))
            .unwrap();
        writer
            .write(16u8 as u32, encode_knxf16(self.roomtemperaturesetpointshifteconomy))
            .unwrap();
        writer
            .write(
                16u8 as u32,
                encode_knxf16(self.roomtemperaturesetpointshiftbuildingprotection),
            )
            .unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_275_101 {
                roomtemperaturesetpointshiftcomfort: {
                    let value: u16 = reader.read(16u8 as u32)?;
                    let value = decode_knxf16(value);
                    value
                },
                roomtemperaturesetpointshiftstandby: {
                    let value: u16 = reader.read(16u8 as u32)?;
                    let value = decode_knxf16(value);
                    value
                },
                roomtemperaturesetpointshifteconomy: {
                    let value: u16 = reader.read(16u8 as u32)?;
                    let value = decode_knxf16(value);
                    value
                },
                roomtemperaturesetpointshiftbuildingprotection: {
                    let value: u16 = reader.read(16u8 as u32)?;
                    let value = decode_knxf16(value);
                    value
                },
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_275_101 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{:.1}/{:.1}/{:.1}/{:.1}", self.roomtemperaturesetpointshiftcomfort, self
            .roomtemperaturesetpointshiftstandby, self
            .roomtemperaturesetpointshifteconomy, self
            .roomtemperaturesetpointshiftbuildingprotection
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_275_x {
    pub roomtemperaturesetpointcomfort: f32,
    pub roomtemperaturesetpointstandby: f32,
    pub roomtemperaturesetpointeconomy: f32,
    pub roomtemperaturesetpointbuildingprotection: f32,
}
impl crate::Specific for DPT_275_x {
    const DPT: DPT = DPT::new(275u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer
            .write(16u8 as u32, encode_knxf16(self.roomtemperaturesetpointcomfort))
            .unwrap();
        writer
            .write(16u8 as u32, encode_knxf16(self.roomtemperaturesetpointstandby))
            .unwrap();
        writer
            .write(16u8 as u32, encode_knxf16(self.roomtemperaturesetpointeconomy))
            .unwrap();
        writer
            .write(
                16u8 as u32,
                encode_knxf16(self.roomtemperaturesetpointbuildingprotection),
            )
            .unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_275_x {
                roomtemperaturesetpointcomfort: {
                    let value: u16 = reader.read(16u8 as u32)?;
                    let value = decode_knxf16(value);
                    value
                },
                roomtemperaturesetpointstandby: {
                    let value: u16 = reader.read(16u8 as u32)?;
                    let value = decode_knxf16(value);
                    value
                },
                roomtemperaturesetpointeconomy: {
                    let value: u16 = reader.read(16u8 as u32)?;
                    let value = decode_knxf16(value);
                    value
                },
                roomtemperaturesetpointbuildingprotection: {
                    let value: u16 = reader.read(16u8 as u32)?;
                    let value = decode_knxf16(value);
                    value
                },
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_275_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{:.1}/{:.1}/{:.1}/{:.1}", self.roomtemperaturesetpointcomfort, self
            .roomtemperaturesetpointstandby, self.roomtemperaturesetpointeconomy, self
            .roomtemperaturesetpointbuildingprotection
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_285_1 {
    pub int: u8,
    pub int2: u8,
    pub int3: u8,
    pub int4: u8,
    pub int5: u8,
    pub int6: u8,
    pub int7: u8,
    pub int8: u8,
    pub int9: u8,
    pub int10: u8,
    pub int11: u8,
    pub int12: u8,
    pub int13: u8,
    pub int14: u8,
    pub int15: u8,
    pub int16: u8,
}
impl crate::Specific for DPT_285_1 {
    const DPT: DPT = DPT::new(285u16, Some(1u16));
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.int).unwrap();
        writer.write(8u8 as u32, self.int2).unwrap();
        writer.write(8u8 as u32, self.int3).unwrap();
        writer.write(8u8 as u32, self.int4).unwrap();
        writer.write(8u8 as u32, self.int5).unwrap();
        writer.write(8u8 as u32, self.int6).unwrap();
        writer.write(8u8 as u32, self.int7).unwrap();
        writer.write(8u8 as u32, self.int8).unwrap();
        writer.write(8u8 as u32, self.int9).unwrap();
        writer.write(8u8 as u32, self.int10).unwrap();
        writer.write(8u8 as u32, self.int11).unwrap();
        writer.write(8u8 as u32, self.int12).unwrap();
        writer.write(8u8 as u32, self.int13).unwrap();
        writer.write(8u8 as u32, self.int14).unwrap();
        writer.write(8u8 as u32, self.int15).unwrap();
        writer.write(8u8 as u32, self.int16).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_285_1 {
                int: reader.read(8u8 as u32)?,
                int2: reader.read(8u8 as u32)?,
                int3: reader.read(8u8 as u32)?,
                int4: reader.read(8u8 as u32)?,
                int5: reader.read(8u8 as u32)?,
                int6: reader.read(8u8 as u32)?,
                int7: reader.read(8u8 as u32)?,
                int8: reader.read(8u8 as u32)?,
                int9: reader.read(8u8 as u32)?,
                int10: reader.read(8u8 as u32)?,
                int11: reader.read(8u8 as u32)?,
                int12: reader.read(8u8 as u32)?,
                int13: reader.read(8u8 as u32)?,
                int14: reader.read(8u8 as u32)?,
                int15: reader.read(8u8 as u32)?,
                int16: reader.read(8u8 as u32)?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_285_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}", self.int, self.int2,
            self.int3, self.int4, self.int5, self.int6, self.int7, self.int8, self.int9,
            self.int10, self.int11, self.int12, self.int13, self.int14, self.int15, self
            .int16
        )
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct DPT_285_x {
    pub int: u8,
    pub int2: u8,
    pub int3: u8,
    pub int4: u8,
    pub int5: u8,
    pub int6: u8,
    pub int7: u8,
    pub int8: u8,
    pub int9: u8,
    pub int10: u8,
    pub int11: u8,
    pub int12: u8,
    pub int13: u8,
    pub int14: u8,
    pub int15: u8,
    pub int16: u8,
}
impl crate::Specific for DPT_285_x {
    const DPT: DPT = DPT::new(285u16, None);
    fn to_data_point(&self) -> DataPoint {
        let mut bytes = Vec::new();
        let mut writer = BitWriter::endian(&mut bytes, BigEndian);
        writer.write(8u8 as u32, self.int).unwrap();
        writer.write(8u8 as u32, self.int2).unwrap();
        writer.write(8u8 as u32, self.int3).unwrap();
        writer.write(8u8 as u32, self.int4).unwrap();
        writer.write(8u8 as u32, self.int5).unwrap();
        writer.write(8u8 as u32, self.int6).unwrap();
        writer.write(8u8 as u32, self.int7).unwrap();
        writer.write(8u8 as u32, self.int8).unwrap();
        writer.write(8u8 as u32, self.int9).unwrap();
        writer.write(8u8 as u32, self.int10).unwrap();
        writer.write(8u8 as u32, self.int11).unwrap();
        writer.write(8u8 as u32, self.int12).unwrap();
        writer.write(8u8 as u32, self.int13).unwrap();
        writer.write(8u8 as u32, self.int14).unwrap();
        writer.write(8u8 as u32, self.int15).unwrap();
        writer.write(8u8 as u32, self.int16).unwrap();
        writer.flush().unwrap();
        DataPoint::Long(bytes)
    }
    fn from_data_point(data: &DataPoint) -> Result<Self, Error> {
        if let DataPoint::Long(bytes) = data {
            let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
            Ok(DPT_285_x {
                int: reader.read(8u8 as u32)?,
                int2: reader.read(8u8 as u32)?,
                int3: reader.read(8u8 as u32)?,
                int4: reader.read(8u8 as u32)?,
                int5: reader.read(8u8 as u32)?,
                int6: reader.read(8u8 as u32)?,
                int7: reader.read(8u8 as u32)?,
                int8: reader.read(8u8 as u32)?,
                int9: reader.read(8u8 as u32)?,
                int10: reader.read(8u8 as u32)?,
                int11: reader.read(8u8 as u32)?,
                int12: reader.read(8u8 as u32)?,
                int13: reader.read(8u8 as u32)?,
                int14: reader.read(8u8 as u32)?,
                int15: reader.read(8u8 as u32)?,
                int16: reader.read(8u8 as u32)?,
            })
        } else {
            Err(Error::InvalidDataPointValue(data.to_owned()))
        }
    }
}
impl Display for DPT_285_x {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}", self.int, self.int2,
            self.int3, self.int4, self.int5, self.int6, self.int7, self.int8, self.int9,
            self.int10, self.int11, self.int12, self.int13, self.int14, self.int15, self
            .int16
        )
    }
}
