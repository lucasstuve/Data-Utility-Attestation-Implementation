use crate::ast::Term;
use crate::epl::{Itype, Schema};
use crate::interpreter::Event;
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;


pub fn grap_event_string<'a>(indexes: (u32, u32), bytes: &'a [u8]) -> &'a str {
    let (s, e) = indexes;
    let s_usize: usize = s as usize;
    let e_usize: usize = e as usize;
    core::str::from_utf8(&bytes[s_usize..e_usize]).unwrap()
}

pub fn efficient_event_extraction(schema: Schema, raw_event: &[u8]) -> Option<Event> {
    let attributes = schema.clone().attribute_list.list;
    let mut e_data = Vec::new();

    for a in attributes {
        let start = &raw_event
            .windows(a.ident.len())
            .position(|w| w == a.ident.as_bytes())
            .unwrap()
            + 3 as usize
            + a.ident.len();
        let end = &raw_event[start..]
            .iter()
            .position(|&e| e == b'"')
            .expect("Closing delimmiter not found.");

        let abs_end = end + start;

        let s = core::str::from_utf8(&&raw_event[start..abs_end]).unwrap();

        /*
        if s.len() == 0 || s == "" {
            return None;
        } */

        match a.i_type {
            Itype::Float => {
                if s.chars().all(|c| c.is_ascii_digit() || c == '.') {
                    let attr_parsed = bytes_to_term(a.i_type, &raw_event[start..abs_end]);
                    e_data.push(attr_parsed);
                }
            }
            Itype::Int => {
                if s.chars().all(|c| c.is_ascii_digit()) {
                    let attr_parsed = bytes_to_term(a.i_type, &raw_event[start..abs_end]);
                    e_data.push(attr_parsed);
                }
            }
            Itype::Bool => {
                if s == "true" || s == "false" {
                    let attr_parsed = bytes_to_term(a.i_type, &raw_event[start..abs_end]);
                    e_data.push(attr_parsed);
                }
            }
            Itype::String => {
                let attr_parsed = bytes_to_term(a.i_type, &raw_event[start..abs_end]);
                e_data.push(attr_parsed);
            }
        }
    }

    if e_data.len() == schema.attribute_list.list.len() {
        return Some(Event { data: e_data });
    } else {
        return None;
    }
}

pub fn valid_schema_types<'a>(s: &'a Schema) -> Vec<&'a str> {
    let list = &s.attribute_list.list;
    let mut typed_data_fields: Vec<&'a str> = Vec::new();
    for a in list {
        typed_data_fields.push(&a.ident);
    }

    return typed_data_fields;
}

// event String: "{"key":"4b26efad-ee19-305a-add0-a7a422d4e719","dataFieldName":"profiles.targetSOCPercentage","value":"30","timestampUtc":"2025-12-18T16:45:03.484Z"}"

pub fn bytes_to_term(itype: Itype, b: &[u8]) -> Term {
    let s = core::str::from_utf8(&b).unwrap();

    match itype {
        Itype::Bool => Term::Bool(s.parse::<bool>().unwrap()),
        Itype::Float => Term::Float(s.parse::<f64>().unwrap()),
        Itype::Int => Term::Int(s.parse::<i64>().unwrap()),
        Itype::String => Term::Str(s.parse::<String>().unwrap()),
    }
}

#[cfg(test)]

pub mod efficient_event_extraction_test {
    use super::*;
    use crate::epl::{Attribute, AttributeList, Schema};

    #[test]
    pub fn event_extraction_test() {
        let event_string = r#"{"key":"e5334c84-a1ed-4dde-930d-a23c145d021a","dataFieldName":"mileage","value":"47.01","timestampUtc":"2026-01-01T00:03:00.000Z"}"#.as_bytes();
        let event_string_2 = r#"{"key":"e5334c84-a1ed-4dde-930d-a23c145d021a","dataFieldName":"OTHER_FIELD","value":"HereIdeclare.","timestampUtc":"2026-01-01T00:03:00.000Z"}"#.as_bytes();

        let schema: Schema = Schema {
            ident: "VehicleData".into(),
            attribute_list: AttributeList {
                list: alloc::vec![
                    Attribute {
                        ident: "value".into(),
                        i_type: Itype::Float
                    },
                    Attribute {
                        ident: "dataFieldName".into(),
                        i_type: Itype::String
                    }
                ],
            },
        };
        let not_empty_event = Event {
            data: alloc::vec![Term::Float(47.01), Term::Str("mileage".into())],
        };
        let empty_event = Event {
            data: alloc::vec![],
        };
        assert_eq!(
            efficient_event_extraction(schema.clone(), event_string).unwrap(),
            not_empty_event
        );
        assert_eq!(efficient_event_extraction(schema, event_string_2), None);
    }
}
