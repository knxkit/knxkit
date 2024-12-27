// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use std::{future::Future, sync::Arc};

use tokio::sync::Notify;

use crate::{
    core::{address::IndividualAddress, cemi::CEMI},
    error::Error,
};

pub mod multiplex;
pub mod ops;
pub mod remote;

pub use remote::{connect, parse_remote, RemoteSpec};

pub trait KnxBusConnection {
    fn send(&self, cemi: CEMI) -> impl Future<Output = Result<Arc<Notify>, Error>>;
    fn recv(&mut self) -> impl Future<Output = Option<Arc<CEMI>>>;
    fn terminate(self) -> impl Future<Output = ()>;
    fn address(&self) -> IndividualAddress;
}
