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

use knxkit::project::{DatapointSubtype, Format};

fn decode_format(format: &Format) -> TokenStream {
    match format {
        Format::Bit { .. } => {
            quote!(reader.read_bit()?)
        }

        Format::Integer { width, .. } => {
            quote!(reader.read(#width as u32)?)
        }

        Format::Float { width, .. } => {
            let (ntype, convert) = match width {
                16 => (quote!(u16), quote!(decode_knxf16(value))),
                32 => (quote!(u32), quote!(f32::from_bits(value))),

                _ => panic!("invalid width: {}", width),
            };

            quote!({
                let value: #ntype = reader.read(#width as u32)?;
                let value = #convert;
                value
            })
        }

        Format::String {
            width: 8,
            encoding,
            variable_length: false,
            ..
        } => match encoding.as_str() {
            "us-ascii" => quote! {
                reader.read_to::<u8>()? as char
            },
            "iso-8859-1" => quote! {
                char::from_u32(
                    encoding::codec::singlebyte::iso_8859_1::forward(reader.read_to()?) as u32,
                )
                .unwrap(),
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

            quote!({
                let mut value = [0u8; (#width / 8) as usize];
                reader.read_bytes(&mut value)?;
                #codec.decode(&value, DecoderTrap::Replace).unwrap()
            })
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

            quote!({
                let mut buffer = [0u8; 2048];

                reader.read_bytes(&mut buffer).unwrap();

                let end = buffer
                    .iter()
                    .position(|&c| c == b'\0')
                    .unwrap_or(buffer.len());

                #codec.decode(&buffer[0..end], DecoderTrap::Replace).unwrap()
            })
        }

        Format::Reserved { width } => {
            quote!({
                reader.skip(#width as u32)?;
                Reserved::new()
            })
        }

        Format::Enumeration { width, .. } => {
            quote!(reader.read(#width as u32)?)
        }

        Format::Reference { .. } => panic!("unexpected reference format"),
    }
}

pub fn decode(
    struct_name: &syn::Ident,
    arg_name: &syn::Ident,
    subtype: &DatapointSubtype,
) -> TokenStream {
    let size_in_bit = subtype.datapoint_type.size;

    let decode = match subtype.formats.as_slice() {
        [single] => {
            let decode = decode_format(single);

            quote! {
                #struct_name(#decode)
            }
        }

        multiple => {
            let mut name_map = HashMap::new();
            let decodes = multiple.iter().map(|format| {
                let name = crate::util::format_name(subtype, format, &mut name_map);
                let decode = decode_format(format);
                quote! {#name: #decode}
            });

            quote! {
              #struct_name{#(#decodes),*}
            }
        }
    };

    if size_in_bit <= 6 {
        quote! {
            if let DataPoint::Short(byte) = #arg_name {
                let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new([*byte]));
                reader.seek_bits(SeekFrom::End(#size_in_bit as i64)).unwrap();

                Ok(#decode)
            } else {
                Err(Error::InvalidDataPointValue(#arg_name.to_owned()))
            }
        }
    } else {
        quote! {
            if let DataPoint::Long(bytes) = #arg_name {
                let mut reader: BitReader<_, BE> = BitReader::new(Cursor::new(&bytes));
                Ok(#decode)
            } else {
                Err(Error::InvalidDataPointValue(#arg_name.to_owned()))
            }
        }
    }
}
