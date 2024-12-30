// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use anyhow::Result;

use knxkit::{
    connection::{remote::RemoteSpec, KnxBusConnection},
    project::CowString,
};
use tokio::signal::unix::{signal, Signal, SignalKind};

use crate::CLI;

pub async fn interrupt() -> std::io::Result<Signal> {
    signal(SignalKind::interrupt())
}

pub async fn connect(remote: &RemoteSpec) -> Result<impl KnxBusConnection> {
    Ok(knxkit::connection::remote::connect(CLI.globals.local_address, remote).await?)
}

pub trait Defaults<'a> {
    fn unwrap_or_missing(self) -> CowString<'a>;
    fn unwrap_or_empty(self) -> CowString<'a>;
}

impl<'a> Defaults<'a> for Option<CowString<'a>> {
    fn unwrap_or_missing(self) -> CowString<'a> {
        self.unwrap_or_else(|| CowString::Ref("-"))
    }

    fn unwrap_or_empty(self) -> CowString<'a> {
        self.unwrap_or_else(|| CowString::Ref(""))
    }
}

#[macro_export]
macro_rules! match_variant {
    ($p:pat = $e:expr => $s:stmt) => {
        if let $p = $e {
            $s
        } else {
            panic!("invalid pattern")
        }
    };
}
