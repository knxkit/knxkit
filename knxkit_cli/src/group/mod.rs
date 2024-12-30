// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use anyhow::Result;

use crate::cli::GroupCommand;

mod monitor;
mod read;
mod write;

pub async fn command(command: &GroupCommand) -> Result<()> {
    match command {
        GroupCommand::Monitor { .. } => monitor::command(command).await,
        GroupCommand::Read { .. } => read::command(command).await,
        GroupCommand::Write { .. } => write::command(command).await,
    }
}
