// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use std::borrow::Borrow;

use crate::{
    core::address::{GroupAddress, IndividualAddress},
    project::Project,
};

use super::DPT;

#[derive(Debug, Clone)]
pub enum CowString<'a> {
    Owned(String),
    Ref(&'a str),
}

impl CowString<'_> {
    pub fn as_str(&self) -> &str {
        match self {
            CowString::Owned(string) => string.as_str(),
            CowString::Ref(string) => string,
        }
    }
}

impl<'a> From<&'a str> for CowString<'a> {
    fn from(value: &'a str) -> Self {
        Self::Ref(value)
    }
}

impl From<String> for CowString<'_> {
    fn from(value: String) -> Self {
        Self::Owned(value)
    }
}

impl std::fmt::Display for CowString<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(self.as_str())
    }
}

pub trait ProjectExt {
    fn device_name(&self, address: impl Borrow<IndividualAddress>) -> Option<CowString>;
    fn group(&self, address: impl Borrow<GroupAddress>) -> Option<CowString>;
    fn group_dpt(&self, address: GroupAddress) -> Option<DPT>;
    //    fn decode_hex(&self, address: GroupAddress, hex: &str) -> Result<DataPoint, Error>;
}

impl ProjectExt for Option<&Project> {
    fn device_name(&self, address: impl Borrow<IndividualAddress>) -> Option<CowString> {
        self.and_then(|project| {
            project
                .devices
                .by_address(address)
                .map(|d| d.name.as_str().into())
        })
    }

    fn group(&self, address: impl Borrow<GroupAddress>) -> Option<CowString> {
        self.and_then(|project| {
            project
                .groups
                .by_address(address)
                .map(|g| g.name.as_str().into())
        })
    }

    fn group_dpt(&self, address: GroupAddress) -> Option<DPT> {
        self.and_then(|project| project.groups.by_address(address).and_then(|g| g.dpt))
    }

    /*
    fn decode_hex(&self, address: GroupAddress, hex: &str) -> Result<DataPoint, Error> {
        // self.and_then(|project|)

        unimplemented!();
    }*/
}
