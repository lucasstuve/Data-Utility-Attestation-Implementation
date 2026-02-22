use crate::ast::Term;
use crate::epl::{Attribute, AttributeList, Itype, Schema};
use crate::interpreter::Event;
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

                Term::Number(f as i64)
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
                Term::Number(n)
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
/*

pub fn extract_value_as_str(event_str: [u8], key: &[u8]) -> &str {
    let start = event_str.to_string.find(key)? + key.iter().len();
    let rest = event_str[start..];
    let end = rest.find('"');

    return rest[..end].to_string();
}

pub fn cast_string_to_typed_term(value_str: &str, t: &Itype) -> Term {
    match t {
        Itype::Bool => Term::Bool(value_str.parse().unwrap()),
        Itype::Float => Term::Number(value_str.parse().unwrap()),
        Itype::String => Term::Str(value_str.parse().unwrap()),
        Itype::Int => Term::Number(value_str.parse().unwrap()),
    }
}

*/

/*
#[cfg(test)]
mod test {
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
*/
