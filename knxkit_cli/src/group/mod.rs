// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use std::str::FromStr;

use anyhow::Result;
use clap::{command, Subcommand};

use knxkit::core::address::GroupAddress;

use crate::cli::{Format, Remote};

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

#[derive(clap::ValueEnum, Debug, Clone, Copy, PartialEq)]
pub enum ValueFormat {
    Raw,
    Value,
}

#[derive(Subcommand, Debug, Clone)]
pub enum GroupCommand {
    Monitor {
        #[command(flatten)]
        remote: Remote,

        #[command(flatten)]
        format: Format,
    },

    Read {
        #[command(flatten)]
        remote: Remote,

        #[arg(value_parser = GroupAddress::from_str)]
        group: GroupAddress,

        #[arg(long, value_enum, default_value = "raw")]
        format: ValueFormat,

        #[arg(long, default_value = "false")]
        unit: bool,

        #[arg(long, value_parser = parse_duration::parse, default_value = "5s")]
        timeout: std::time::Duration,
    },

    Write {
        #[command(flatten)]
        remote: Remote,

        #[arg(value_parser = GroupAddress::from_str)]
        group: GroupAddress,

        value: String,

        #[arg(long, value_enum, default_value = "raw")]
        format: ValueFormat,
    },
}
