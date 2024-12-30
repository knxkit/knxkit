// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use std::collections::HashMap;

use anyhow::{anyhow, Result};
use interpolator::Formattable;

use knxkit::{
    connection::KnxBusConnection,
    core::{
        address::DestinationAddress,
        apdu::{Service, APDU},
        cemi::CEMI,
        tpdu::TPDU,
    },
    project::{CowString, ProjectExt},
};

use knxkit_dpt::project::ProjectExtDPT;

use crate::{
    cli::{Format, GroupCommand, CLI},
    util::{connect, Defaults},
};

fn format_cemi(cemi: &CEMI, format: &Format) -> Result<Option<String>> {
    let CEMI {
        hops,
        prio,
        source,
        destination,
        npdu,
        ..
    } = cemi;

    let src = source;
    let time = time::OffsetDateTime::now_local()?.format(&format.time_format)?;

    let line = if let (DestinationAddress::Group(dst), TPDU::DataGroup(APDU { service, data })) =
        (&destination, &npdu.tpdu)
    {
        match service {
            Service::GroupValueRead => Some("GroupValueRead"),
            Service::GroupValueWrite => Some("GroupValueWrite"),
            Service::GroupValueResponse => Some("GroupValueResponse"),
            _ => None,
        }
        .map(|service| {
            let project = CLI.globals.project.as_ref();

            let dpt = project
                .group_dpt(*dst)
                .map(|dpt| CowString::from(dpt.to_string()))
                .unwrap_or_missing();
            let dpt_name = project.group_dpt_name(dst).unwrap_or_missing();
            let dpt_unit = project.group_dpt_unit(dst).unwrap_or_missing();

            let data_hex = data
                .as_ref()
                .map(|dp| CowString::from(dp.to_string()))
                .unwrap_or_missing();

            let data_value = data
                .as_ref()
                .and_then(|data| project.group_value(*dst, data, false))
                .unwrap_or_missing();

            let data = data
                .as_ref()
                .and_then(|data| project.group_value(*dst, data, true))
                .unwrap_or_missing();

            let src_name = project.device_name(src).unwrap_or_missing();
            let src_address = src.to_string();
            let src = project
                .device_name(src)
                .unwrap_or_else(|| src.to_string().into());

            let dst_name = project.group(dst).unwrap_or_missing();
            let dst_address = dst.to_string();
            let dst = project.group(dst).unwrap_or_else(|| dst.to_string().into());

            let context = [
                ("time", Formattable::display(&time)),
                //
                ("src_name", Formattable::display(&src_name)),
                ("src_address", Formattable::display(&src_address)),
                ("src", Formattable::display(&src)),
                //
                ("dst_name", Formattable::display(&dst_name)),
                ("dst_address", Formattable::display(&dst_address)),
                ("dst", Formattable::display(&dst)),
                //
                ("prio", Formattable::debug(prio)),
                ("service", Formattable::display(&service)),
                ("hops", Formattable::integer(hops)),
                //
                ("dpt", Formattable::display(&dpt)),
                ("dpt_name", Formattable::display(&dpt_name)),
                //
                ("unit", Formattable::display(&dpt_unit)),
                ("data_value", Formattable::display(&data_value)),
                ("data_hex", Formattable::display(&data_hex)),
                ("data", Formattable::display(&data)),
            ]
            .into_iter()
            .collect::<HashMap<_, _>>();

            interpolator::format(&format.line_format, &context)
        })
        .transpose()
        .map_err(|e| anyhow!("invalid format: `{}` - {}", format.line_format, e))?
    } else {
        None
    };

    Ok(line)
}

pub async fn command(command: &GroupCommand) -> Result<()> {
    crate::match_variant!(GroupCommand::Monitor { remote, format } = command => {
        let mut signal = crate::util::interrupt().await?;
        let mut tunnel = connect(&remote.remote).await.unwrap();

        loop {
            tokio::select! {
                _ = signal.recv() => {
                    break;
                }

                recv = tunnel.recv() => {
                    if let Some(cemi) = recv {
                        if let Some(line) = format_cemi(&cemi, format)? {
                            if CLI.globals.log {
                                tracing::info!("{}", line);
                            }else{
                                println!("{}", line);
                            }
                        }
                    } else {
                        break;
                    }
                }
            }
        }

        tunnel.terminate().await;

        Ok(())
    })
}
