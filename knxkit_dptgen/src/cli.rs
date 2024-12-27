// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use std::path::PathBuf;

use anyhow::Result;
use clap::{command, Parser};

use knxkit::project::Project;

fn parse_project(v: &str) -> Result<Project> {
    Ok(Project::open(v)?)
}

#[derive(clap::ValueEnum, Debug, Clone, Copy, PartialEq)]
pub enum Language {
    Rust,
}

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(long = "project", value_parser = parse_project)]
    pub project: Project,

    #[arg(long = "language", default_value = "rust")]
    pub language: Language,

    #[arg(long = "destination")]
    pub destination: PathBuf,
}
