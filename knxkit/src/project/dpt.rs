// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use std::str::FromStr;

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
/// let dpt = DPT { main: 1, sub: Some(1) };
/// assert_eq!(format!("{}", dpt), "1.001");
/// ```
impl std::fmt::Display for DPT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self.sub {
            Some(sub) => format!("{}.{:03}", self.main, sub),
            None => format!("{}.x", self.main),
        };

        f.pad(&s)
    }
}

impl FromStr for DPT {
    type Err = crate::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        use nom::{
            branch::alt,
            bytes::complete::tag,
            character::complete::digit1,
            combinator::{eof, map_res},
            sequence::tuple,
            Finish,
        };

        type NomErr<'a> = nom::error::Error<&'a str>;

        let s = s.trim();

        let (_, (main, _, sub, _)) = tuple((
            map_res(digit1::<_, NomErr>, &str::parse::<u16>),
            tag("."),
            alt((
                map_res(tag("x"), |_| Ok::<_, NomErr>(None)),
                map_res(digit1, |s: &str| s.parse::<u16>().map(Some)),
            )),
            eof,
        ))(s)
        .finish()
        .map_err(|_| Self::Err::InvalidInput(s.to_string()))?;

        Ok(DPT { main, sub })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dpt_parsing() {
        assert_eq!("1.001".parse::<DPT>().unwrap(), DPT::new(1, Some(1)));
        assert_eq!("3.x".parse::<DPT>().unwrap(), DPT::new(3, None));

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
