// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use std::cell::LazyCell;

use knxkit::core::DataPoint;
use serde_json::Value;

use crate::{Specific, DPT};

type Lazy<T> = LazyCell<T, Box<dyn FnOnce() -> T>>;

pub struct OpaqueDataPoint {
    dpt: DPT,
    raw: Lazy<DataPoint>,
    json: Lazy<Value>,
    display: Lazy<String>,
}

impl OpaqueDataPoint {
    pub fn new<T>(value: T) -> Self
    where
        T: Specific + std::fmt::Display + 'static,
    {
        let value = std::sync::Arc::new(value);
        let display = value.clone();
        let json = value.clone();

        OpaqueDataPoint {
            dpt: T::DPT,
            raw: LazyCell::new(Box::new(move || value.as_ref().to_data_point())),
            display: LazyCell::new(Box::new(move || display.to_string())),
            json: LazyCell::new(Box::new(move || {
                serde_json::to_value(json.as_ref()).unwrap()
            })),
        }
    }

    pub fn dpt(&self) -> DPT {
        self.dpt
    }

    pub fn to_json_value(&self) -> Value {
        self.json.clone()
    }

    pub fn to_json_string(&self) -> String {
        self.json.to_string()
    }

    pub fn to_data_point(&self) -> DataPoint {
        self.raw.clone()
    }
}

impl std::fmt::Display for OpaqueDataPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.display)
    }
}
