// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

pub type Input<'a> = &'a [u8];
pub type Result<'a, R> = IResult<Input<'a>, R, Error>;

pub mod prelude {
    pub use super::{gen_enum, parse_enum, parse_token, Error, Input, Result, ResultExt};
    pub use std::io::Write;

    pub use cookie_factory::{
        bytes::{be_u16 as gen_u16, be_u8 as gen_u8},
        combinator::slice as gen_slice,
        gen_simple,
        sequence::tuple as gen_tuple,
        GenError, SerializeFn,
    };
    pub use nom::{
        bytes::complete::take as parse_take,
        error::{ErrorKind, ParseError},
        multi::count as parse_count,
        number::complete::{be_u16 as parse_u16, be_u32 as parse_u32, be_u8 as parse_u8},
        sequence::tuple as parse_tuple,
        Err, Finish, IResult,
    };

    pub use num_derive::{FromPrimitive, ToPrimitive};
    pub use num_traits::ToPrimitive;
}

use prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("General error: {0}")]
    General(String, Vec<u8>),

    #[error("Parse error: {0:?}")]
    ParseError(ErrorKind, Vec<u8>),

    #[error("Gen error")]
    GenError(#[from] cookie_factory::GenError),
}

impl Error {
    pub fn general(s: impl ToString, b: impl Into<Vec<u8>>) -> Self {
        Self::General(s.to_string(), b.into())
    }
}

impl ParseError<Input<'_>> for Error {
    fn from_error_kind(input: Input<'_>, kind: ErrorKind) -> Self {
        Error::ParseError(kind, input.into())
    }

    fn append(_: Input<'_>, _: ErrorKind, other: Self) -> Self {
        other
    }
}

pub fn parse_token(expected: u8) -> impl Fn(Input) -> Result<()> {
    move |input0| {
        let (input, token) = parse_u8(input0)?;

        if token != expected {
            Err(nom::Err::Failure(Error::general(
                "Unexpected token",
                input0,
            )))
        } else {
            Ok((input, ()))
        }
    }
}

pub fn parse_enum<T: num_traits::FromPrimitive>(size: u8) -> impl Fn(Input) -> Result<T> {
    move |input: Input| {
        match size {
            8 => {
                let (input, value) = parse_u8(input)?;
                T::from_u8(value).map(|x| (input, x))
            }

            16 => {
                let (input, value) = parse_u16(input)?;
                T::from_u16(value).map(|x| (input, x))
            }
            _ => {
                panic!()
            }
        }
        .ok_or(nom::Err::Failure(Error::general(
            "Enumeration error",
            input,
        )))
    }
}

pub fn gen_enum<'a, T: num_traits::ToPrimitive, W: std::io::Write>(
    size: u8,
    value: &'a T,
) -> impl cookie_factory::SerializeFn<W> + 'a {
    move |context| match size {
        8 => {
            let value = value.to_u8().unwrap();
            gen_u8(value)(context)
        }

        16 => {
            let value = value.to_u16().unwrap();
            gen_u16(value)(context)
        }
        _ => {
            panic!("unexpected size")
        }
    }
}

pub trait ResultExt<'a, T> {
    fn map_value<U>(self, f: impl Fn(T) -> U) -> Result<'a, U>;
    fn and_then_value<U>(
        self,
        f: impl Fn(T) -> std::result::Result<U, nom::Err<Error>>,
    ) -> Result<'a, U>;
}

impl<'a, T> ResultExt<'a, T> for Result<'a, T> {
    fn map_value<U>(self, f: impl FnOnce(T) -> U) -> Result<'a, U> {
        Result::map(self, |(i, o)| (i, f(o)))
    }

    fn and_then_value<U>(
        self,
        f: impl FnOnce(T) -> std::result::Result<U, nom::Err<Error>>,
    ) -> Result<'a, U> {
        self.and_then(|(i, t)| f(t).map(|u| (i, u)))
    }
}
