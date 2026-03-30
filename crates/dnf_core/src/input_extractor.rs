use crate::ast::Term;
use crate::epl::{Itype, Schema};
use crate::interpreter::Event;
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
//use serde_json::Value;

/*
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
    */

pub fn grap_event_string<'a>(indexes: (u32, u32), bytes: &'a [u8]) -> &'a str {
    let (s, e) = indexes;
    let s_usize: usize = s as usize;
    let e_usize: usize = e as usize;
    core::str::from_utf8(&bytes[s_usize..e_usize]).unwrap()
}

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
