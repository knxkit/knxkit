// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use std::collections::HashMap;

use anyhow::Result;
use futures_util::pin_mut;
use interpolator::Formattable;
use tokio::time::sleep_until;

use crate::cli::CLI;

use futures_util::stream::StreamExt;

use super::IpCommand;
pub async fn command(command: &IpCommand) -> Result<()> {
    crate::match_variant!(IpCommand::Search { timeout, format } = command => {
        let results = knxkit::net::search::search(CLI.globals.local_address, CLI.globals.nat).await?;
        pin_mut!(results);

        let deadline = tokio::time::Instant::now() + *timeout;

        loop {
            tokio::select! {
                _ = sleep_until(deadline) => {
                    break;
                }

                response = results.next() => {
                    if let Some(response) = response {
                        let serial = hex::encode(&response.device_information.serial);
                        let context = [
                            ("hpai", Formattable::display(&response.hpai)),
                            ("medium", Formattable::display(&response.device_information.medium)),
                            ("mac", Formattable::display(&response.device_information.mac)),
                            ("name", Formattable::display(&response.device_information.name)),
                            ("multicast", Formattable::display(&response.device_information.ip_multicast)),
                            ("serial", Formattable::display(&serial)),
                            ("status", Formattable::integer(&response.device_information.status.0))
                        ]
                        .into_iter()
                        .collect::<HashMap<_, _>>();

                        let line = interpolator::format(format, &context)?;

                        println!("{}", line);
                    } else {
                        break;
                    }
                }
            }
        }

        Ok(())
    })
}
