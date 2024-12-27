// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use std::{
    fmt::Debug,
    str::{self, FromStr},
};

use roxmltree::Node;

use crate::project::error::Error;

pub fn by_name(tag: &'static str) -> impl Fn(&Node) -> bool {
    move |i| i.is_element() && i.has_tag_name(tag)
}

pub trait NodeExt<'a, 'input: 'a> {
    fn child(self, name: &'static str) -> Result<Node<'a, 'input>, Error>;

    fn att<T: FromStr>(&self, name: &'static str) -> Result<T, Error>
    where
        T::Err: Debug;

    fn att_opt<T: FromStr>(&self, name: &'static str) -> Result<Option<T>, Error>
    where
        T::Err: Debug;
}

impl<'a, 'input: 'a> NodeExt<'a, 'input> for Node<'a, 'input> {
    fn child(self, name: &'static str) -> Result<Node<'a, 'input>, Error> {
        self.children()
            .find(by_name(name))
            .ok_or_else(|| Error::ParseError(format!("missing child node: {}", name)))
    }

    fn att<T: std::str::FromStr>(&self, name: &'static str) -> Result<T, Error>
    where
        T::Err: Debug,
    {
        let v = self
            .attribute(name)
            .ok_or_else(|| Error::ParseError(format!("missing required attribute: {}", name)))?;

        v.parse::<T>()
            .map_err(|_| Error::ParseError(format!("cannot parse attribute value: {}={}", name, v)))
    }

    fn att_opt<T: std::str::FromStr>(&self, name: &'static str) -> Result<Option<T>, Error>
    where
        T::Err: Debug,
    {
        self.attribute(name)
            .map(|v| {
                v.parse::<T>().map_err(|_| {
                    Error::ParseError(format!("cannot parse attribute value: {}={}", name, v))
                })
            })
            .transpose()
    }
}
