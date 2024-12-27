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

fn encode_format(writer: &Ident, value: &TokenStream, format: &Format) -> TokenStream {
    match format {
        Format::Bit { .. } => {
            quote! {
                #writer.write_bit(#value).unwrap();
            }
        }

        Format::Integer { width, signed, .. } => {
            if *signed {
                quote! {
                    #writer.write_signed(#width as u32, #value).unwrap();
                }
            } else {
                quote! {
                    #writer.write(#width as u32, #value).unwrap();
                }
            }
        }

        Format::Float { width, .. } => {
            let converted = match width {
                16 => quote!(encode_knxf16(#value)),
                32 => quote!(f32::to_bits(#value)),

                _ => panic!("invalid width: {}", width),
            };

            quote! {
                #writer.write(#width as u32, #converted).unwrap();
            }
        }

        Format::String {
            width: 8, encoding, ..
        } => match encoding.as_str() {
            "us-ascii" => quote! {
                writer.write_from(#value as u8).unwrap();
            },
            "iso-8859-1" => quote! {
                writer.write_from(encoding::codec::singlebyte::iso_8859_1::backward(
                   #value as u32,
                )).unwrap();
            },

            _ => panic!("unsupported encoding: {}", encoding),
        },

        Format::String {
            width,
            encoding,
            variable_length: false,
            ..
        } => {
            let codec = match encoding.as_str() {
                "iso-8859-1" => quote!(ISO_8859_1),
                "us-ascii" => quote!(ASCII),
                _ => panic!("unsupported encoding: {}", encoding),
            };

            quote! {
                let mut buffer = vec![0u8; (#width / 8) as usize];
                #codec.encode_to(#value.as_str(), EncoderTrap::Replace, &mut buffer).unwrap();

                writer.write_bytes(&buffer).unwrap();
            }
        }

        Format::String {
            encoding,
            variable_length: true,
            ..
        } => {
            let codec = match encoding.as_str() {
                "iso-8859-1" => quote!(ISO_8859_1),
                "utf-8" => quote!(UTF_8),

                _ => panic!("unsupported encoding: {}", encoding),
            };

            quote! {
                let bytes = #codec
                    .encode(#value.as_str(), EncoderTrap::Replace)
                    .unwrap();

                #writer.write_bytes(&bytes).unwrap();
                #writer.write_bytes(&[0]).unwrap();
            }
        }

        Format::Enumeration { width, .. } => {
            quote! {
                #writer.write(#width as u32, #value).unwrap();
            }
        }

        Format::Reserved { width } => {
            quote! {
                #writer.write(#width as u32, 0u32).unwrap();
            }
        }

        Format::Reference { .. } => panic!("unexpected reference format"),
    }
}

pub fn encode(subtype: &DatapointSubtype) -> TokenStream {
    let size_in_bit = subtype.datapoint_type.size;

    let writer = quote::format_ident!("writer");

    let encode = match subtype.formats.as_slice() {
        [single] => encode_format(&writer, &quote!(self.0), single),

        multiple => {
            let mut name_map = HashMap::new();

            let formats = multiple.iter().map(|format| {
                let name = crate::util::format_name(subtype, format, &mut name_map);
                let value = quote!(self.#name);
                encode_format(&writer, &value, format)
            });

            quote! {
                #(#formats);*
            }
        }
    };

    let from = if size_in_bit <= 6 {
        quote! {
            let mut bytes = [0u8];
            let mut #writer = BitWriter::endian(bytes.as_mut_slice(), BigEndian);

            #encode

            DataPoint::Short(#writer.into_unwritten().1)
        }
    } else {
        quote! {
            let mut bytes = Vec::new();
            let mut #writer = BitWriter::endian(&mut bytes, BigEndian);

            #encode

            #writer.flush().unwrap();

            DataPoint::Long(bytes)
        }
    };

    quote! {
        #from
    }
}
