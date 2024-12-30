// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use std::process;

mod cli;
mod group;
mod util;

use cli::CLI;

#[tokio::main]
async fn main() {
    if CLI.globals.log {
        use tracing_subscriber::filter::targets::Targets;
        use tracing_subscriber::prelude::*;

        let filter = Targets::default()
            .with_target("knxkit", tracing::Level::DEBUG)
            .with_target("knx", tracing::Level::DEBUG);

        let format = tracing_subscriber::fmt::layer().compact();

        tracing_subscriber::registry()
            .with(filter)
            .with(format)
            .init();
    }

    let result = match &cli::CLI.command {
        cli::Command::Group { command } => group::command(&command).await,
    };

    if let Err(error) = &result {
        eprintln!("error: {}", error)
    }

    process::exit(result.map(|_| 0).unwrap_or(-1));
}
