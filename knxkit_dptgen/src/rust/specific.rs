// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use knxkit::project::{DatapointSubtype, Format};

fn integer_type(width: u8, signed: bool) -> TokenStream {
    match (width, signed) {
        (32, false) => quote!(u32),
        (16, false) => quote!(u16),
        (8, false) => quote!(u8),
        (64, true) => quote!(i64),
        (32, true) => quote!(i32),
        (16, true) => quote!(i16),
        (8, true) => quote!(i8),

        (w, false) if w < 8 => quote!(u8),

        _ => panic!("invalid width: {}", width),
    }
}

fn format_type(format: &Format) -> TokenStream {
    match format {
        Format::Bit { .. } => quote!(bool),
        Format::Integer { width, signed, .. } => {
            let ntype = integer_type(*width, *signed);
            quote!(#ntype)
        }
        Format::Float { .. } => {
            quote!(f32)
        }

        Format::String { width: 8, .. } => {
            quote!(char)
        }

        Format::String { .. } => {
            quote!(String)
        }

        Format::Enumeration { width, .. } => {
            let ntype = integer_type(*width, false);
            quote!(#ntype)
        }

        Format::Reserved { .. } => quote!(Reserved),

        Format::Reference { .. } => panic!("unexpected reference format"),
    }
}

fn impl_value(sname: &Ident, subtype: &DatapointSubtype) -> TokenStream {
    let new_dpt = util::new_dpt(subtype);

    let decode_arg = quote::format_ident!("data");
    let decode = super::decode::decode(sname, &decode_arg, subtype);
    let encode = super::encode::encode(subtype);

    quote! {
        impl crate::Specific for #sname {
            const DPT: DPT = #new_dpt;

            fn to_data_point(&self) -> DataPoint {
                #encode
            }

            fn from_data_point(#decode_arg: &DataPoint) -> Result<Self, Error> {
                #decode
            }
        }
    }
}

fn format_spec(format: &Format) -> &'static str {
    match format {
        Format::Float { .. } => "{:.1}",
        _ => "{}",
    }
}

fn impl_display(sname: &Ident, subtype: &DatapointSubtype) -> TokenStream {
    let write = match subtype.formats.as_slice() {
        [single] => {
            let spec = format_spec(single);

            quote! {
                write!(f, #spec, self.0)
            }
        }

        multiple => {
            let specs = multiple
                .iter()
                .map(format_spec)
                .collect::<Vec<_>>()
                .join("/");

            let mut name_map = HashMap::new();
            let values = multiple.iter().map(|format| {
                let name = crate::util::format_name(subtype, format, &mut name_map);
                quote! {
                    self.#name
                }
            });

            quote! {
                write!(f, #specs, #(#values),*)
            }
        }
    };

    quote! {
        impl Display for #sname {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                #write
            }
        }
    }
}

fn subtype(subtype: &DatapointSubtype) -> TokenStream {
    let struct_name = crate::util::subtype_name(subtype);

    let struct_decl = match subtype.formats.as_slice() {
        [single] => {
            let type_ = format_type(single);
            quote! {
                #[allow(non_camel_case_types)]
                pub struct #struct_name(pub #type_);
            }
        }

        multiple => {
            let mut name_map = HashMap::new();
            let types = multiple.iter().map(|format| {
                let type_ = format_type(format);
                let name = crate::util::format_name(subtype, format, &mut name_map);
                quote! {pub #name: #type_}
            });

            quote! {
                #[allow(non_camel_case_types)]
                #[allow(non_snake_case)]
                pub struct #struct_name {#(#types),*}
            }
        }
    };

    let impl_value = impl_value(&struct_name, subtype);
    let impl_display = impl_display(&struct_name, subtype);

    quote! {
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]


        #struct_decl
        #impl_value
        #impl_display
    }
}

use knxkit::project::MasterData;

use crate::util;

pub fn generate(masterdata: &MasterData) -> TokenStream {
    let subtypes = masterdata.subtypes.iter().map(subtype);

    quote! {
        use std::{io::{SeekFrom, Cursor}, fmt::{Display, Debug}};
        use bitstream_io::{BitRead, BitReader, BE, BitWriter, BitWrite, BigEndian};
        use encoding::{Encoding, all::{ASCII, ISO_8859_1, UTF_8}, types::{EncoderTrap, DecoderTrap}};
        use serde::{Serialize, Deserialize};
        use knxkit::{
            project::DPT,
            core::DataPoint
        };
        use crate::{decode_knxf16, encode_knxf16, Error, Reserved};

        #(#subtypes)*
    }
}
