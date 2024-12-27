// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use roxmltree::Document;
use std::{io::Read, path::Path};

use crate::project::{devices::Devices, error::Error, groups::Groups, master};

use super::MasterData;

#[derive(Clone, Debug)]
pub struct Project {
    pub devices: Devices,
    pub groups: Groups,
    pub master: MasterData,
}

impl Project {
    pub fn open(file_name: impl AsRef<Path>) -> Result<Project, Error> {
        let file = std::fs::File::open(file_name)?;
        let mut zip = zip::ZipArchive::new(file)?;

        let master = {
            let mut master = zip.by_name("knx_master.xml")?;
            let mut contents = String::new();
            master.read_to_string(&mut contents)?;

            master::MasterData::parse(&Document::parse(&contents)?)
        }?;

        let regex = regex::Regex::new(r"^P-(.*).signature$").expect("invalid regex");

        let pfile = 'a: {
            for name in zip.file_names() {
                if let Some(capture) = regex.captures(name) {
                    break 'a Some(format!("P-{}/0.xml", &capture[1]).to_string());
                }
            }
            None
        }
        .ok_or(Error::ProjectError("cannot identify project id"))?;

        let mut pfile = zip.by_name(&pfile)?;
        let mut contents = String::new();
        pfile.read_to_string(&mut contents)?;
        let pfile = Document::parse(&contents)?;

        let devices = Devices::parse(&pfile)?;
        let groups = Groups::parse(&pfile, &master)?;

        Ok(Project {
            devices,
            groups,
            master,
        })
    }
}
