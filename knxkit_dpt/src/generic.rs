// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use std::cell::LazyCell;

use serde_json::Value;

use knxkit::{core::DataPoint, project::DPT};

use crate::specific::SpecificDataPoint;

pub use super::generated::generic::*;

type Lazy<T> = LazyCell<T, Box<dyn FnOnce() -> T>>;

/// Untyped datapioint representation
pub struct GenericDataPoint {
    dpt: DPT,
    raw: Lazy<DataPoint>,
    json: Lazy<Value>,
    display: Lazy<String>,
}

impl GenericDataPoint {
    pub fn new<T>(value: T) -> Self
    where
        T: SpecificDataPoint + std::fmt::Display + 'static,
    {
        let value = std::sync::Arc::new(value);
        let display = value.clone();
        let json = value.clone();

        GenericDataPoint {
            dpt: T::DPT,
            raw: LazyCell::new(Box::new(move || value.as_ref().to_data_point())),
            display: LazyCell::new(Box::new(move || display.to_string())),
            json: LazyCell::new(Box::new(move || {
                serde_json::to_value(json.as_ref()).unwrap()
            })),
        }
    }

    /// Get the DPT identifier  of the data point
    pub fn dpt(&self) -> DPT {
        self.dpt
    }

    /// Convert to json value
    pub fn to_json_value(&self) -> Value {
        self.json.clone()
    }

    /// Convert to json string
    pub fn to_json_string(&self) -> String {
        self.json.to_string()
    }

    /// Convert to DataPoint
    pub fn to_data_point(&self) -> DataPoint {
        self.raw.clone()
    }
}

impl std::fmt::Display for GenericDataPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.display)
    }
}
