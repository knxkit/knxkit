// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use std::collections::HashMap;

use knxkit::project::{DatapointSubtype, Format, DPT};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub fn subtype_name(subtype: &DatapointSubtype) -> Ident {
    match subtype.dpt {
        DPT {
            main,
            sub: Some(sub),
        } => {
            quote::format_ident!("DPT_{}_{}", main, sub)
        }
        DPT { main, sub: None } => {
            quote::format_ident!("DPT_{}_x", main)
        }
    }
}

pub fn new_dpt(subtype: &DatapointSubtype) -> TokenStream {
    match subtype.dpt {
        DPT {
            main,
            sub: Some(sub),
        } => {
            quote! {DPT::new(#main, Some(#sub))}
        }
        DPT { main, sub: None } => {
            quote! {DPT::new(#main, None)}
        }
    }
}

pub fn format_name(
    subtype: &DatapointSubtype,
    format: &Format,
    name_map: &mut HashMap<String, u8>,
) -> Ident {
    let name = if subtype.formats.len() == 1 {
        subtype.text.to_owned()
    } else {
        None
    }
    .unwrap_or_else(|| match format {
        Format::Bit { name, set, .. } => name.to_owned().unwrap_or_else(|| {
            if set != "True" {
                set.to_string()
            } else {
                "bit".to_string()
            }
        }),
        Format::Enumeration { name, .. } => {
            name.to_owned().unwrap_or_else(|| "enumeration".to_string())
        }
        Format::Integer { name, .. } => name.to_owned().unwrap_or_else(|| "int".to_string()),
        Format::Float { name, .. } => name.to_owned().unwrap_or_else(|| "float".to_string()),
        Format::Reserved { .. } => "reserved".to_string(),
        Format::String { .. } => "string".to_string(),
        Format::Reference(_) => {
            panic!()
        }
    });

    let name = name
        .chars()
        .filter(|c| c.is_alphabetic() && c.is_ascii())
        .collect::<String>();

    let number = name_map.entry(name.clone()).or_insert(0);

    if *number == 0 {
        *number = 1;
        quote::format_ident!("{}", name)
    } else {
        *number += 1;
        quote::format_ident!("{}{}", name, *number)
    }
}
