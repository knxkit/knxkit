// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use std::borrow::Borrow;
use std::collections::HashMap;

use roxmltree::Document;

use crate::core::address::IndividualAddress;
use crate::project::error::Error;
use crate::project::util::{by_name, NodeExt};

#[derive(Clone, Debug)]
pub struct Device {
    pub name: String,
    pub address: IndividualAddress,
}

#[derive(Clone, Debug)]
pub struct Devices {
    pub devices: Vec<Device>,
    by_name: HashMap<String, usize>,
    by_address: HashMap<IndividualAddress, usize>,
}

impl Devices {
    pub(crate) fn parse(document: &Document) -> Result<Self, Error> {
        let mut devices = Vec::new();

        for area_node in document.descendants().filter(by_name("Area")) {
            let area = area_node.att("Address")?;
            for line_node in area_node.children().filter(by_name("Line")) {
                let line = line_node.att("Address")?;
                for device_node in line_node.descendants().filter(by_name("DeviceInstance")) {
                    let device = device_node.att("Address")?;
                    let name = device_node.att("Name")?;
                    let address = IndividualAddress::from_components((area, line, device));

                    devices.push(Device { name, address });
                }
            }
        }

        let mut by_name = HashMap::new();
        let mut by_address = HashMap::new();

        for (ix, device) in devices.iter().enumerate() {
            by_name.insert(device.name.clone(), ix);
            by_address.insert(device.address, ix);
        }

        Ok(Devices {
            devices,
            by_name,
            by_address,
        })
    }

    pub fn by_name(&self, name: &str) -> Option<&Device> {
        self.by_name.get(name).map(|ix| &self.devices[*ix])
    }

    pub fn by_address(&self, address: impl Borrow<IndividualAddress>) -> Option<&Device> {
        self.by_address
            .get(address.borrow())
            .map(|ix| &self.devices[*ix])
    }
}
