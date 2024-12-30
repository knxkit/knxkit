// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

pub use super::generated::typeinfo::*;

use knxkit::project::DPT;

pub struct TypeInfo {
    pub dpt: DPT,
    pub name: &'static str,
    pub text: Option<&'static str>,
    pub unit: Option<&'static str>,
}
