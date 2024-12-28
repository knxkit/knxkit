// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use anyhow::{anyhow, Result};

use knxkit::{
    connection::{ops::GroupOps, KnxBusConnection},
    core::address::GroupAddress,
};

use knxkit_dpt::project::ProjectExtDPT;

use crate::{
    cli::{Remote, ValueFormat, CLI},
    util::connect,
};

pub async fn command(
    remote: &Remote,
    group: GroupAddress,
    format: ValueFormat,
    unit: bool,
) -> Result<()> {
    let mut connection = connect(&remote.remote).await.unwrap();

    let dp = connection.group_read(group).await?;

    connection.terminate().await;

    if format == ValueFormat::Raw {
        println!("{}", dp);
    } else {
        let project = CLI.globals.project.as_ref();

        if let Some(display) = project.group_value(group, &dp, unit) {
            println!("{}", display);
        } else {
            return Err(anyhow!(
                "value cannot be decoded (project not set or unknown group)"
            ));
        }
    }

    Ok(())
}
