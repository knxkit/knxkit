// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use knxkit::project::MasterData;
use proc_macro2::TokenStream;
use quote::quote;

use crate::util;

pub fn generate(masterdata: &MasterData) -> TokenStream {
    let mut subtypes = masterdata.subtypes.clone();
    subtypes.sort_by_key(|s| s.dpt);

    let datapoint_decoders = subtypes.iter().map(|subtype| {
        let sname = crate::util::subtype_name(subtype);
        let new_dpt = util::new_dpt(subtype);

        quote! {
            (#new_dpt, |dp| {
                Ok(GenericDataPoint::new(#sname::from_data_point(dp)?))
            })
        }
    });

    let serde_decoders = subtypes.iter().map(|subtype| {
        let sname = crate::util::subtype_name(subtype);
        let new_dpt = util::new_dpt(subtype);

        quote! {
            (#new_dpt, |value| {
                Ok(GenericDataPoint::new(serde_json::from_value::<#sname>(
                    value,
                )?))
            })
        }
    });

    let lookup = quote! {
        DECODERS
        .binary_search_by_key(&dpt, |x| x.0)
        .map(|ix| DECODERS[ix].1(data))
        .map_err(|_| Error::InvalidDPT(dpt))?
    };

    quote! {
        use serde_json::Value;
        use knxkit::{core::DataPoint, project::DPT};
        use crate::{Error, specific::SpecificDataPoint, generic::GenericDataPoint, specific::*};

        pub fn try_decode(dpt: DPT, data: &DataPoint) -> Result<GenericDataPoint, Error> {
            static DECODERS: &'static [(DPT, fn(&DataPoint) -> Result<GenericDataPoint, Error>)] =
                &[#(#datapoint_decoders),*];

            #lookup
        }

        pub fn try_decode_json(dpt: DPT, data: Value) -> Result<GenericDataPoint, Error> {
            static DECODERS: &'static [(DPT, fn(Value) -> Result<GenericDataPoint, Error>)] =
                &[#(#serde_decoders),*];

            #lookup
        }
    }
}
