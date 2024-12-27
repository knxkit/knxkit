// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use std::{
    net::{IpAddr, Ipv4Addr},
    str::FromStr,
    sync::LazyLock,
};

use anyhow::{bail, Result};
use clap::{command, Parser, Subcommand};

use knxkit::{connection::remote, core::address::GroupAddress, project::Project};

fn parse_local(v: &str) -> Result<Ipv4Addr> {
    let local = if v.len() > 0 {
        v.parse::<Ipv4Addr>()?
    } else {
        if let Ok(IpAddr::V4(v4)) = local_ip_address::local_ip() {
            v4
        } else {
            bail!("cannot identify local ip v4 address")
        }
    };

    Ok(local)
}

fn parse_project(v: &str) -> Result<Project> {
    Ok(Project::open(v)?)
}

#[derive(Parser, Debug, Clone)]
pub struct Remote {
    #[arg(short = 'r', long)]
    #[arg(value_parser = remote::parse_remote)]
    pub remote: remote::RemoteSpec,
}

#[derive(Parser, Debug, Clone)]
pub struct Format {
    #[arg(long = "format")]
    #[arg(
        default_value = "{time} {prio} {src} {src_name} {dst} {dst_name} {service} {hops} {dpt} {dpt_name} {data_raw} {data} {unit}"
    )]
    pub line_format: String,

    #[arg(value_parser = time::format_description::parse_strftime_owned)]
    #[arg(default_value = "%a, %d %b %Y %T %z")]
    pub time_format: time::format_description::OwnedFormatItem,
}

#[derive(Parser, Debug, Clone)]
pub struct Globals {
    #[arg(short = 'l', long = "local")]
    #[arg(value_parser = parse_local, default_value="")]
    pub local_address: Ipv4Addr,

    #[arg(long = "log")]
    pub log: bool,

    #[arg(long = "project", value_parser = parse_project)]
    pub project: Option<Project>,
}

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(flatten)]
    pub globals: Globals,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    Group {
        #[command(subcommand)]
        command: GroupCommand,
    },
}

#[derive(clap::ValueEnum, Debug, Clone, Copy, PartialEq)]
pub enum ReadValueFormat {
    Raw,
    Value,
    Hex,
}

#[derive(clap::ValueEnum, Debug, Clone, Copy, PartialEq)]
pub enum WriteValueFormat {
    Raw,
    Hex,
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

        #[arg(value_enum, default_value = "value", long = "format")]
        format: ReadValueFormat,
    },

    Write {
        #[command(flatten)]
        remote: Remote,

        #[arg(value_parser = GroupAddress::from_str)]
        group: GroupAddress,

        value: String,

        #[arg(value_enum, default_value = "hex", long = "format")]
        format: WriteValueFormat,
    },
}

pub static CLI: LazyLock<Cli> = LazyLock::new(Cli::parse);
