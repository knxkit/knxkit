// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use std::path::Path;

use anyhow::Result;
use prettyplease as pp;
use proc_macro2::TokenStream;

use knxkit::project::MasterData;

mod decode;
mod encode;
mod generic;
mod specific;
mod typeinfo;

fn write(path: &Path, content: TokenStream) -> Result<()> {
    const HEADER: &str = "// This file is generated, don't manually edit it!";

    std::fs::write(
        path.as_os_str(),
        format!("{}\n{}", HEADER, pp::unparse(&syn::parse2(content)?)),
    )?;

    Ok(())
}

pub fn generate(master: &MasterData, destination: &Path) -> Result<()> {
    write(&destination.join("specific.rs"), specific::generate(master))?;
    write(&destination.join("generic.rs"), generic::generate(master))?;
    write(&destination.join("typeinfo.rs"), typeinfo::generate(master))?;

    Ok(())
}
