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

use crate::core::address::GroupAddress;
use crate::project::dpt::DPT;
use crate::project::error::Error;
use crate::project::util::{by_name, NodeExt};

use super::MasterData;

#[derive(Clone, Debug)]
pub struct Group {
    pub path: Vec<String>,
    pub name: String,
    pub address: GroupAddress,
    pub dpt: Option<DPT>,
}

#[derive(Clone, Debug)]
pub struct Groups {
    pub groups: Vec<Group>,
    pub by_name: HashMap<String, usize>,
    pub by_address: HashMap<GroupAddress, usize>,
}

impl Groups {
    pub fn parse(document: &Document, master: &MasterData) -> Result<Groups, Error> {
        let mut groups = Vec::new();

        for group_node in document.descendants().filter(by_name("GroupAddress")) {
            let address = GroupAddress::new(group_node.att("Address")?);
            let name = group_node.att("Name")?;
            let dpt = group_node
                .att_opt::<String>("DatapointType")?
                .map(|s| {
                    master.by_id(&s).map(|x| x.dpt).ok_or_else(|| {
                        Error::ParseError(format!("Unexpected DPT reference: {}", s))
                    })
                })
                .transpose()?;

            let mut path = Vec::new();
            let mut group = group_node.parent();

            loop {
                match group {
                    Some(group1) if group1.tag_name().name() == "GroupRange" => {
                        path.push(group1.att("Name")?);

                        group = group1.parent();
                    }
                    _ => {
                        break;
                    }
                }
            }

            path.reverse();

            let group = Group {
                address,
                name,
                dpt,
                path,
            };

            groups.push(group);
        }

        let mut by_name = HashMap::new();
        let mut by_address = HashMap::new();

        for (ix, g) in groups.iter().enumerate() {
            by_name.insert(g.name.clone(), ix);
            by_address.insert(g.address, ix);
        }

        Ok(Groups {
            groups,
            by_name,
            by_address,
        })
    }

    pub fn by_name(&self, name: &str) -> Option<&Group> {
        self.by_name.get(name).map(|ix| &self.groups[*ix])
    }

    pub fn by_address(&self, address: impl Borrow<GroupAddress>) -> Option<&Group> {
        self.by_address
            .get(address.borrow())
            .map(|ix| &self.groups[*ix])
    }
}
