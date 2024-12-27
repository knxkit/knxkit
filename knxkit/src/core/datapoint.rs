// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use std::str::FromStr;

use nom::Finish;

#[derive(Debug, Clone, PartialEq)]
pub enum DataPoint {
    Short(u8),
    Long(Vec<u8>),
}

impl DataPoint {
    pub fn to_hex_string(&self) -> String {
        match self {
            Self::Short(byte) => hex::encode([*byte]),
            Self::Long(bytes) => hex::encode(bytes),
        }
    }
}

impl std::fmt::Display for DataPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let hex = match self {
            Self::Short(byte) => format!("{}", hex::encode([*byte])),
            Self::Long(bytes) => format!("[{}]", hex::encode(bytes)),
        };

        f.pad(&hex)
    }
}

impl FromStr for DataPoint {
    type Err = crate::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use nom::{
            bytes::complete::take_while_m_n,
            character::complete::char,
            combinator::{eof, map_res},
            multi::many1,
            sequence::{delimited, tuple},
            IResult,
        };

        fn parse_hex_byte(input: &str) -> IResult<&str, u8> {
            map_res(take_while_m_n(2, 2, |c: char| c.is_digit(16)), |h| {
                u8::from_str_radix(h, 16)
            })(input)
        }

        if input.starts_with('[') {
            map_res(
                tuple((delimited(char('['), many1(parse_hex_byte), char(']')), eof)),
                |(bytes, _)| Ok::<_, nom::error::Error<&str>>(DataPoint::Long(bytes)),
            )(input)
            .finish()
        } else {
            map_res(tuple((parse_hex_byte, eof)), |(byte, _)| {
                Ok::<_, nom::error::Error<&str>>(DataPoint::Short(byte))
            })(input)
            .finish()
        }
        .map(|(_, result)| result)
        .map_err(|_| crate::Error::InvalidInput(input.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_short_datapoint() {
        let input = "1a";
        let expected = DataPoint::Short(0x1a);
        let result = DataPoint::from_str(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_long_datapoint() {
        let input = "[1a2b3c]";
        let expected = DataPoint::Long(vec![0x1a, 0x2b, 0x3c]);
        let result = DataPoint::from_str(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_invalid_datapoint() {
        assert!(DataPoint::from_str("invalid").is_err());
        assert!(DataPoint::from_str("").is_err());
        assert!(DataPoint::from_str("  ").is_err());
        assert!(DataPoint::from_str("a").is_err());
        assert!(DataPoint::from_str("az").is_err());
        assert!(DataPoint::from_str("[ac1]").is_err());
        assert!(DataPoint::from_str("[ac1]1").is_err());
    }
}
