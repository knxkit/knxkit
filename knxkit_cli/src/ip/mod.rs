// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use anyhow::Result;
use clap::Subcommand;

mod search;

#[derive(Subcommand, Debug, Clone)]
pub enum IpCommand {
    Search {
        #[arg(long = "timeout", value_parser = parse_duration::parse, default_value = "3s")]
        timeout: std::time::Duration,

        #[arg(
            long = "format",
            default_value = "{hpai} {multicast} {medium} {mac} {name} {serial} {status}"
        )]
        format: String,
    },
}

pub async fn command(command: &IpCommand) -> Result<()> {
    match command {
        IpCommand::Search { .. } => search::command(command).await,
    }
}
