// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use proc_macro2::TokenStream;
use quote::quote;

use knxkit::project::MasterData;

use crate::util;

pub fn generate(masterdata: &MasterData) -> TokenStream {
    let mut subtypes = masterdata.subtypes.clone();
    subtypes.sort_by_key(|s| s.dpt);

    let typeinfos = subtypes.iter().map(|subtype| {
        let name = &subtype.name;
        let text = subtype
            .text
            .as_ref()
            .map(|text| quote!(Some(#text)))
            .unwrap_or_else(|| quote!(None));
        let new_dpt = util::new_dpt(subtype);

        let unit = if subtype.formats.len() == 1 {
            subtype.formats[0].unit().map(|unit| quote!(Some(#unit)))
        } else {
            None
        }
        .unwrap_or(quote!(None));

        quote! {
            (#new_dpt, TypeInfo{dpt: #new_dpt, name: #name, text: #text, unit: #unit})
        }
    });

    quote! {
        use knxkit::project::DPT;
        use crate::typeinfo::TypeInfo;

        pub fn lookup(dpt: DPT) -> Option<&'static TypeInfo> {
            static DATAPOINTS: &'static [(DPT, TypeInfo)] =
                &[#(#typeinfos),*];

            DATAPOINTS
                .binary_search_by_key(&dpt, |x| x.0)
                .ok()
                .map(|ix| &DATAPOINTS[ix].1)
        }
    }
}
