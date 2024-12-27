// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use tokio::time::error::Elapsed;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    //#[error("Net Error: {0}")]
    //NetError(#[from] crate::net::error::Error),
    #[error("protocol error: {0}")]
    ProtocolError(#[from] crate::core::util::Error),

    #[error("tunnel error: {0}")]
    TunnelError(String),

    #[error("invalid input: {0}")]
    InvalidInput(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("timeout: {0}")]
    Timeout(#[from] Elapsed),

    #[error("general Error: {0}")]
    General(String),
}
