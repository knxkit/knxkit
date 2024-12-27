// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

/*
use std::sync::Arc;

use super::KnxBusConnection;
use crate::core::{address::IndividualAddress, cemi::CEMI};

pub struct Multiplexer<T: KnxBusConnection> {
    connection: T,
}

impl<T: KnxBusConnection> Multiplexer<T> {
    pub fn new(connection: T) -> Self {
        Self { connection }
    }
}

struct MultiplexerConnection {
    //
}

impl KnxBusConnection for MultiplexerConnection {
    async fn recv(&mut self) -> Option<Result<Arc<CEMI>, Box<dyn std::error::Error>>> {
        todo!()
    }

    async fn send(&self, cemi: CEMI) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    async fn terminate(self) -> () {
        todo!()
    }

    fn address(&self) -> IndividualAddress {
        todo!()
    }
}
*/
