// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use std::str::FromStr;

use crate::project::error::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DPT {
    pub main: u16,
    pub sub: Option<u16>,
}

impl DPT {
    pub const fn new(main: u16, sub: Option<u16>) -> Self {
        Self { main, sub }
    }
}

/// Formats the DPT (Data Point Type) as a string in the format "main.sub"
///
/// # Examples
///
/// ```
/// use knxkit::project::DPT;
///
/// let dpt = DPT { main: 1, sub: 1 };
/// assert_eq!(format!("{}", dpt), "1.1");
/// ```
impl std::fmt::Display for DPT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&format!(
            "{}.{}",
            self.main,
            self.sub
                .map(|sub| sub.to_string())
                .unwrap_or("x".to_string()),
        ))
    }
}

impl FromStr for DPT {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        use nom::{
            bytes::complete::tag,
            character::complete::digit1,
            combinator::{eof, map_res},
            sequence::tuple,
            Finish, IResult,
        };

        fn parse_dpt(input: &str) -> IResult<&str, (u16, u16)> {
            let (input, (main, _, sub, _)) = tuple((
                map_res(digit1, FromStr::from_str),
                tag("."),
                map_res(digit1, FromStr::from_str),
                eof,
            ))(input)?;

            Ok((input, (main, sub)))
        }

        match parse_dpt(s).finish() {
            Ok((_, (main, sub))) => Ok(DPT::new(main, Some(sub))),
            Err(_) => Err(Error::ParseError(format!("invalid DPT string: {}", s))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dpt_parsing() {
        assert_eq!("1.1".parse::<DPT>().unwrap(), DPT::new(1, Some(1)));
        assert_eq!("10.234".parse::<DPT>().unwrap(), DPT::new(10, Some(234)));
        assert_eq!("255.255".parse::<DPT>().unwrap(), DPT::new(255, Some(255)));

        assert!("1".parse::<DPT>().is_err());
        assert!("1.".parse::<DPT>().is_err());
        assert!(".1".parse::<DPT>().is_err());
        assert!("a.1".parse::<DPT>().is_err());
        assert!("1.b".parse::<DPT>().is_err());
        assert!("1.1.1".parse::<DPT>().is_err());
        assert!("65536.1".parse::<DPT>().is_err());
        assert!("1.65536".parse::<DPT>().is_err());
    }
}
