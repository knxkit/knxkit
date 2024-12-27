// Copyright (c) 2024 Alexey Aristov <aav@acm.org> and others
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at http://
// www.eclipse.org/legal/epl-2.0, or the GNU General Public License, version 3
// which is available at https://www.gnu.org/licenses/gpl-3.0.en.html.
//
// SPDX-License-Identifier: EPL-2.0 OR GPL-3.0

use std::{collections::HashMap, fmt::Debug, rc::Rc};

use roxmltree::{Document, Node};

use crate::project::{
    dpt::DPT,
    error::Error,
    util::{by_name, NodeExt},
};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DatapointType {
    pub id: String,
    pub dpt: DPT,

    pub name: String,
    pub text: Option<String>,

    pub size: u16,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DatapointSubtype {
    pub datapoint_type: Rc<DatapointType>,

    pub id: String,
    pub dpt: DPT,

    pub name: String,
    pub text: Option<String>,

    pub formats: Vec<Format>,
    pub default: Option<String>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct EnumerationValue {
    pub id: String,
    pub value: u8,
    pub text: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Format {
    Bit {
        name: Option<String>,
        cleared: String,
        set: String,
    },

    Integer {
        name: Option<String>,
        width: u8,
        signed: bool,
        min_inclusive: Option<i64>,
        max_inclusive: Option<i64>,
        coefficient: Option<f64>,
        unit: Option<String>,
    },

    Float {
        name: Option<String>,
        width: u8,
        min_value: Option<f64>,
        max_value: Option<f64>,
        unit: Option<String>,
    },

    String {
        encoding: String,
        width: u16,
        variable_length: bool,
        null_terminated: bool,
    },

    Enumeration {
        name: Option<String>,
        width: u8,
        values: Vec<EnumerationValue>,
    },

    Reserved {
        width: u8,
    },

    Reference(String),
}

impl Format {
    pub fn unit(&self) -> Option<&String> {
        match self {
            Format::Integer { unit, .. } => unit.as_ref(),
            Format::Float { unit, .. } => unit.as_ref(),
            _ => None,
        }
    }
}

fn parse_format(f: Node) -> Result<(Option<String>, Format), Error> {
    let ftype = f.tag_name().name();

    match ftype {
        "Bit" => Ok((
            Some(f.att("Id")?),
            Format::Bit {
                name: f.att_opt("Name")?,
                cleared: f.att("Cleared")?,
                set: f.att("Set")?,
            },
        )),

        "UnsignedInteger" => Ok((
            Some(f.att("Id")?),
            Format::Integer {
                name: f.att_opt("Name")?,
                width: f.att("Width")?,
                signed: false,
                min_inclusive: f.att_opt("MinInclusive")?,
                max_inclusive: f.att_opt("MaxInclusive")?,
                coefficient: f.att_opt("Coefficient")?,
                unit: f.att_opt("Unit")?,
            },
        )),

        "SignedInteger" => Ok((
            Some(f.att("Id")?),
            Format::Integer {
                name: f.att_opt("Name")?,
                width: f.att("Width")?,
                signed: true,
                min_inclusive: f.att_opt("MinInclusive")?,
                max_inclusive: f.att_opt("MaxInclusive")?,
                coefficient: f.att_opt("Coefficient")?,
                unit: f.att_opt("Unit")?,
            },
        )),

        "Float" => Ok((
            Some(f.att("Id")?),
            Format::Float {
                name: f.att_opt("Name")?,
                width: f.att("Width")?,
                min_value: f.att_opt("MinValue")?,
                max_value: f.att_opt("MaxValue")?,
                unit: f.att_opt("Unit")?,
            },
        )),

        "String" => Ok((
            Some(f.att("Id")?),
            Format::String {
                encoding: f.att("Encoding")?,
                width: f.att("Width")?,
                variable_length: f.att_opt("VariableLength")?.unwrap_or(false),
                null_terminated: f.att_opt("NullTerminated")?.unwrap_or(false),
            },
        )),

        "Enumeration" => Ok((
            Some(f.att("Id")?),
            Format::Enumeration {
                name: f.att_opt("Name")?,
                width: f.att("Width")?,
                values: {
                    let mut values = Vec::new();

                    for v in f.children().filter(by_name("EnumValue")) {
                        values.push(EnumerationValue {
                            id: v.att("Id")?,
                            value: v.att("Value")?,
                            text: v.att("Text")?,
                        })
                    }

                    values
                },
            },
        )),

        "Reserved" => Ok((
            None,
            Format::Reserved {
                width: f.att("Width")?,
            },
        )),

        "RefType" => Ok((None, Format::Reference(f.att("RefId")?))),

        unexpected => Err(Error::ParseError(format!(
            "unexpected format: {}",
            unexpected
        ))),
    }
}
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct MasterData {
    pub version: String,
    pub subtypes: Vec<DatapointSubtype>,
    pub by_id: HashMap<String, usize>,
    pub by_dpt: HashMap<DPT, usize>,
}

impl MasterData {
    pub fn parse(document: &Document) -> Result<MasterData, Error> {
        let masterdata = document.root().child("KNX")?.child("MasterData")?;
        let version = masterdata.att::<String>("Version")?;

        let mut subtypes = Vec::new();
        let mut formats_map = HashMap::new();

        for type_node in masterdata
            .child("DatapointTypes")?
            .children()
            .filter(by_name("DatapointType"))
        {
            let datapoint_type = Rc::new(DatapointType {
                id: type_node.att("Id")?,
                dpt: DPT::new(type_node.att("Number")?, None),
                name: type_node.att("Name")?,
                text: type_node.att_opt("Text")?,
                size: type_node.att("SizeInBit")?,
            });

            let mut type_formats = None;

            for subtype_node in type_node
                .child("DatapointSubtypes")?
                .children()
                .filter(by_name("DatapointSubtype"))
            {
                let mut formats = Vec::new();

                for format_node in subtype_node
                    .child("Format")?
                    .children()
                    .filter(Node::is_element)
                {
                    let (id, mut format) = parse_format(format_node)?;

                    if let Some(id) = &id {
                        if !formats_map.contains_key(id) {
                            formats_map.insert(id.clone(), format.clone());
                        }
                    }

                    if let Format::Reference(id) = format {
                        format = formats_map
                            .get(&id)
                            .ok_or_else(|| {
                                Error::ParseError(format!("undefined format reference: {}", id))
                            })?
                            .clone();
                    }

                    formats.push(format);
                }

                if type_formats.is_none() {
                    type_formats = Some(formats.clone());
                }

                let subtype = DatapointSubtype {
                    datapoint_type: datapoint_type.clone(),
                    id: subtype_node.att("Id")?,
                    dpt: DPT::new(datapoint_type.dpt.main, Some(subtype_node.att("Number")?)),

                    name: subtype_node.att("Name")?,
                    text: subtype_node.att_opt("Text")?,
                    default: subtype_node.att_opt("Default")?,
                    formats,
                };

                subtypes.push(subtype);
            }

            let generic = DatapointSubtype {
                datapoint_type: datapoint_type.clone(),
                id: datapoint_type.id.clone(),
                dpt: DPT::new(datapoint_type.dpt.main, None),
                name: datapoint_type.name.clone(),
                text: datapoint_type.text.clone(),
                default: type_node.att_opt("Default")?,
                formats: type_formats.unwrap(),
            };

            subtypes.push(generic);
        }

        let mut by_id = HashMap::new();
        let mut by_dpt = HashMap::new();

        for (ix, subtype) in subtypes.iter().enumerate() {
            by_id.insert(subtype.id.clone(), ix);
            by_dpt.insert(subtype.dpt, ix);
        }

        Ok(MasterData {
            version,
            subtypes,
            by_id,
            by_dpt,
        })
    }

    pub fn by_id(&self, id: &str) -> Option<&DatapointSubtype> {
        self.by_id.get(id).map(|ix| &self.subtypes[*ix])
    }

    pub fn by_dpt(&self, dpt: DPT) -> Option<&DatapointSubtype> {
        self.by_dpt.get(&dpt).map(|ix| &self.subtypes[*ix])
    }
}
