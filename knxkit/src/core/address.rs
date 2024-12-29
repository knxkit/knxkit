// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{eof, map_res, verify},
    sequence::tuple,
    Finish, IResult,
};

use crate::core::util::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct IndividualAddress(u16);

impl IndividualAddress {
    pub fn new(a: u16) -> Self {
        Self(a)
    }

    // 0.0.0
    pub fn new_zero() -> Self {
        Self(0)
    }

    pub fn from_components((area, line, device): (u8, u8, u8)) -> Self {
        let a = (area as u16) << 12 | (line as u16 & 0x0f) << 8 | device as u16;
        Self(a)
    }

    pub fn as_u16(&self) -> u16 {
        self.0
    }
}

impl IndividualAddress {
    pub fn parse(input: Input) -> Result<Self> {
        let (input, address) = parse_u16(input)?;

        Ok((input, IndividualAddress(address)))
    }

    pub fn gen<W: std::io::Write>(&self) -> impl cookie_factory::SerializeFn<W> {
        gen_u16(self.0)
    }
}

impl FromStr for IndividualAddress {
    type Err = crate::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        use nom::{
            bytes::complete::tag,
            character::complete::digit1,
            combinator::{map_res, verify},
            sequence::tuple,
            Finish, IResult,
        };

        fn parse_individual_address(input: &str) -> IResult<&str, (u8, u8, u8)> {
            let (input, (area, _, line, _, device, _)) = tuple((
                verify(map_res(digit1, FromStr::from_str), |n| *n <= 15),
                tag("."),
                verify(map_res(digit1, FromStr::from_str), |n| *n <= 15),
                tag("."),
                map_res(digit1, FromStr::from_str),
                eof,
            ))(input)?;

            Ok((input, (area, line, device)))
        }

        match parse_individual_address(s).finish() {
            Ok((_, (area, line, device))) => {
                Ok(IndividualAddress::from_components((area, line, device)))
            }
            Err(_) => Err(crate::Error::InvalidInput(s.to_string())),
        }
    }
}

impl Display for IndividualAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let a = self.0;
        f.write_str(&format!(
            "{}.{}.{}",
            (a & 0xf000) >> 12,
            (a & 0x0f00) >> 8,
            a & 0x00ff
        ))
    }
}

impl Debug for IndividualAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct GroupAddress(u16);

impl GroupAddress {
    pub fn new(a: u16) -> Self {
        Self(a)
    }

    pub fn from_components((main, middle, sub): (u8, u8, u8)) -> Self {
        Self((main as u16) << 11 | (middle as u16 & 0x07) << 8 | sub as u16)
    }

    pub fn as_u16(&self) -> u16 {
        self.0
    }
}

impl GroupAddress {
    pub fn parse(input: Input) -> Result<Self> {
        let (input, address) = parse_u16(input)?;

        Ok((input, GroupAddress(address)))
    }

    pub fn gen<W: std::io::Write>(&self) -> impl cookie_factory::SerializeFn<W> {
        gen_u16(self.0)
    }
}

impl Display for GroupAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let a = self.0;

        f.write_str(&format!(
            "{}/{}/{}",
            (a & 0xfc00) >> 11,
            (a & 0x0380) >> 8,
            (a & 0x007f)
        ))
    }
}

impl FromStr for GroupAddress {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        fn parse_group_address(input: &str) -> IResult<&str, (u8, u8, u8)> {
            let (input, (main, _, middle, _, sub, _)) = tuple((
                verify(map_res(digit1, FromStr::from_str), |n| *n <= 30),
                tag("/"),
                verify(map_res(digit1, FromStr::from_str), |n| *n <= 7),
                tag("/"),
                map_res(digit1, FromStr::from_str),
                eof,
            ))(input)?;

            Ok((input, (main, middle, sub)))
        }

        match parse_group_address(s).finish() {
            Ok((_, (main, middle, sub))) => Ok(GroupAddress::from_components((main, middle, sub))),
            Err(_) => Err(crate::Error::InvalidInput(s.to_string())),
        }
    }
}

impl Debug for GroupAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

#[derive(Clone, PartialEq)]
pub enum Domain {
    Broadcast,
    Multicast,
    Unicast,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum DestinationAddress {
    Individual(IndividualAddress),
    Group(GroupAddress),
}

impl DestinationAddress {
    pub fn as_u16(&self) -> u16 {
        match self {
            DestinationAddress::Individual(i) => i.as_u16(),
            DestinationAddress::Group(g) => g.as_u16(),
        }
    }

    pub fn domain(&self) -> Domain {
        match self {
            Self::Group(GroupAddress(0)) => Domain::Broadcast,
            Self::Group(_) => Domain::Multicast,
            Self::Individual(_) => Domain::Unicast,
        }
    }

    pub fn is_group(&self) -> bool {
        matches!(self, Self::Group(_))
    }

    pub fn is_individual(&self) -> bool {
        matches!(self, Self::Individual(_))
    }

    pub fn as_group(&self) -> &GroupAddress {
        if let Self::Group(g) = self {
            g
        } else {
            panic!("Not a group address");
        }
    }

    pub fn as_individual(&self) -> &IndividualAddress {
        if let Self::Individual(i) = self {
            i
        } else {
            panic!("Not an individual address");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_individual_address_from_str() {
        // Valid addresses
        assert_eq!(
            "1.2.3".parse::<IndividualAddress>().unwrap(),
            IndividualAddress::from_components((1, 2, 3))
        );
        assert_eq!(
            "15.15.255".parse::<IndividualAddress>().unwrap(),
            IndividualAddress::from_components((15, 15, 255))
        );
        assert_eq!(
            "0.0.0".parse::<IndividualAddress>().unwrap(),
            IndividualAddress::from_components((0, 0, 0))
        );

        // Invalid formats
        assert!("1.2".parse::<IndividualAddress>().is_err());
        assert!("1.2.3.4".parse::<IndividualAddress>().is_err());
        assert!("a.2.3".parse::<IndividualAddress>().is_err());
        assert!("1/2/3".parse::<IndividualAddress>().is_err());
        assert!("".parse::<IndividualAddress>().is_err());

        // Out of range values
        assert!("16.15.255".parse::<IndividualAddress>().is_err());
        assert!("15.16.255".parse::<IndividualAddress>().is_err());
        assert!("15.15.256".parse::<IndividualAddress>().is_err());

        // Edge cases
        assert!(".1.2".parse::<IndividualAddress>().is_err());
        assert!("1..2".parse::<IndividualAddress>().is_err());
        assert!("1.2.".parse::<IndividualAddress>().is_err());
    }

    #[test]
    fn test_group_address_from_str() {
        // Valid addresses
        assert_eq!(
            "1/2/3".parse::<GroupAddress>().unwrap(),
            GroupAddress::from_components((1, 2, 3))
        );
        assert_eq!(
            "30/7/255".parse::<GroupAddress>().unwrap(),
            GroupAddress::from_components((30, 7, 255))
        );
        assert_eq!(
            "0/0/0".parse::<GroupAddress>().unwrap(),
            GroupAddress::from_components((0, 0, 0))
        );

        // Invalid formats
        assert!("1/2".parse::<GroupAddress>().is_err());
        assert!("1/2/3/4".parse::<GroupAddress>().is_err());
        assert!("a/2/3".parse::<GroupAddress>().is_err());
        assert!("1.2.3".parse::<GroupAddress>().is_err());
        assert!("".parse::<GroupAddress>().is_err());

        // Out of range values
        assert!("31/7/255".parse::<GroupAddress>().is_err());
        assert!("30/8/255".parse::<GroupAddress>().is_err());
        assert!("256/7/255".parse::<GroupAddress>().is_err());

        // Edge cases
        assert!("/1/2".parse::<GroupAddress>().is_err());
        assert!("1//2".parse::<GroupAddress>().is_err());
        assert!("1/2/".parse::<GroupAddress>().is_err());
    }
}
