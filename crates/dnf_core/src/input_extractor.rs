use crate::ast::Term;
use crate::epl::{Attribute, AttributeList, Itype, Schema};
use crate::interpreter::Event;
use core::str::*;
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use serde_json::{Result, Value};

pub fn extract_events(schema: Schema, raw_event: &[u8]) -> Event {
    let json: Value = serde_json::from_slice(raw_event).unwrap();
    let mut event_data: Vec<Term> = Vec::new();

    for attr in schema.attribute_list.list {
        let term = match attr.i_type {
            Itype::Bool => {
                let v = json.get(&attr.ident).unwrap();
                let b = v
                    .as_bool()
                    .or_else(|| v.as_str().and_then(|s| s.parse::<bool>().ok()))
                    .unwrap();
                Term::Bool(b)
            }
            Itype::Float => {
                let v = json.get(&attr.ident).unwrap();
                let f = v
                    .as_f64()
                    .or_else(|| v.as_str().and_then(|s| s.parse::<f64>().ok()))
                    .unwrap_or_else(|| panic!("float parsing failed!"));

                Term::Float(f)
            }
            Itype::String => {
                let v = json.get(&attr.ident).unwrap();
                Term::Str(v.as_str().unwrap().into())
            }
            Itype::Int => {
                let v = json.get(&attr.ident).unwrap();
                let n = v
                    .as_i64()
                    .or_else(|| v.as_str().and_then(|s| s.parse::<i64>().ok()))
                    .unwrap();
                Term::Int(n)
            }
        };

        event_data.push(term);
    }

    return Event { data: event_data };
}

pub fn grap_event_string<'a>(indexes: (u32, u32), bytes: &'a [u8]) -> &'a str {
    let (s, e) = indexes;
    let s_usize: usize = s as usize;
    let e_usize: usize = e as usize;
    core::str::from_utf8(&bytes[s_usize..e_usize]).unwrap()
}

//TODO implement more efficient data extraction based on byte,string evaluation.

pub fn efficient_event_extraction(schema: Schema, raw_event: &[u8]) -> Event {
    let attributes = schema.attribute_list.list;
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
        let attr_parsed = bytes_to_term(a.i_type, &raw_event[start..abs_end]);
        e_data.push(attr_parsed);
    }

    return Event { data: e_data };
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
mod extract_events_test {
    use super::*;
    extern crate std;
    use std::println;

    #[test]
    fn test_event_extraction() {
        use super::*;
        extern crate std;
        use std::println;
        let mut attribute_list = Vec::new();
        let a1: Attribute = Attribute {
            ident: "dataFieldName".into(),
            i_type: Itype::String,
        };
        let a2: Attribute = Attribute {
            ident: "value".into(),
            i_type: Itype::Int,
        };

        attribute_list.push(a1);
        attribute_list.push(a2);

        let s: Schema = Schema {
            ident: "VehicleData".into(),
            attribute_list: AttributeList {
                list: attribute_list,
            },
        };

        let example_input = br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"80","timestampUtc":"2025-12-18T16:45:03.484Z"}"#;

        let event = extract_events(s, example_input);

        println!("{:?}", event);
        println!("Number of Events: {}", event.data.len());

        assert_eq!(event.data.len(), 2);
    }
}

#[cfg(test)]
mod efficient_event_extraction_test {
    use super::*;
    extern crate std;

    #[test]
    pub fn test_event_01() {
        let mut attribute_list = Vec::new();
        let a1: Attribute = Attribute {
            ident: "dataFieldName".into(),
            i_type: Itype::String,
        };
        let a2: Attribute = Attribute {
            ident: "value".into(),
            i_type: Itype::Int,
        };

        attribute_list.push(a1);
        attribute_list.push(a2);

        let s: Schema = Schema {
            ident: "VehicleData".into(),
            attribute_list: AttributeList {
                list: attribute_list,
            },
        };

        let example_input = br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"80","timestampUtc":"2025-12-18T16:45:03.484Z"}"#;

        let expected_event = Event {
            data: alloc::vec![
                Term::Str("profiles.targetSOCPercentage".into()),
                Term::Int(80)
            ],
        };

        let result_event = efficient_event_extraction(s, example_input);

        assert_eq!(expected_event == result_event, true);
    }

    #[test]
    pub fn test_event_02() {
        let mut attribute_list = Vec::new();
        let a1: Attribute = Attribute {
            ident: "dataFieldName".into(),
            i_type: Itype::String,
        };
        let a2: Attribute = Attribute {
            ident: "value".into(),
            i_type: Itype::Int,
        };
        let a3: Attribute = Attribute {
            ident: "key".into(),
            i_type: Itype::String,
        };

        let a4: Attribute = Attribute {
            ident: "timestampUtc".into(),
            i_type: Itype::String,
        };

        attribute_list.push(a1);
        attribute_list.push(a2);
        attribute_list.push(a3);
        attribute_list.push(a4);

        let s: Schema = Schema {
            ident: "VehicleData".into(),
            attribute_list: AttributeList {
                list: attribute_list,
            },
        };

        let example_input = br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"80","timestampUtc":"2025-12-18T16:45:03.484Z"}"#;

        let expected_event = Event {
            data: alloc::vec![
                Term::Str("profiles.targetSOCPercentage".into()),
                Term::Int(80),
                Term::Str("c161867b-0566-3cc8-9f60-989848640bcc".into()),
                Term::Str("2025-12-18T16:45:03.484Z".into())
            ],
        };

        let result_event = efficient_event_extraction(s, example_input);

        assert_eq!(expected_event == result_event, true);
    }
}
