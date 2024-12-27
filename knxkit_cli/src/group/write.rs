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

use knxkit::{
    connection::{ops::GroupOps, KnxBusConnection},
    core::{address::GroupAddress, DataPoint},
};

use crate::{
    cli::{Remote, WriteValueFormat},
    util::connect,
};

pub async fn command(
    remote: &Remote,
    group: GroupAddress,
    value: &String,
    format: WriteValueFormat,
) -> Result<()> {
    let mut connection = connect(&remote.remote).await.unwrap();

    let data = match format {
        WriteValueFormat::Hex => DataPoint::from_str(&value)?,
        WriteValueFormat::Raw => {
            unimplemented!()
        }
    };

    let notify = connection.group_write(group, data).await?;

    notify.notified().await;

    connection.terminate().await;

    Ok(())
}
