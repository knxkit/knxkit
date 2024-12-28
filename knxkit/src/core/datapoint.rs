// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum DataPoint {
    Short(u8),
    Long(Vec<u8>),
}

impl std::fmt::Display for DataPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let hex = match self {
            Self::Short(byte) => format!("${}", hex::encode([*byte])),
            Self::Long(bytes) => format!("{}", hex::encode(bytes)),
        };

        f.pad(&hex)
    }
}

impl FromStr for DataPoint {
    type Err = crate::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.trim();

        if input.starts_with('$') && input.len() == 3 {
            let byte = u8::from_str_radix(&input[1..3], 16)
                .map_err(|_| Self::Err::InvalidInput(input.to_string()))?;

            Ok(DataPoint::Short(byte))
        } else {
            if input.len() > 0 && input.len() % 2 == 0 {
                let bytes =
                    hex::decode(input).map_err(|_| Self::Err::InvalidInput(input.to_string()))?;

                Ok(DataPoint::Long(bytes))
            } else {
                Err(Self::Err::InvalidInput(input.to_string()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_short_datapoint() {
        let input = "$1a";
        let expected = DataPoint::Short(0x1a);
        let result = DataPoint::from_str(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_long_datapoint() {
        let input = "1a2b3c";
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
