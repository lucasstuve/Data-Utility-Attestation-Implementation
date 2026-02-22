#![no_std]
#![no_main]
extern crate alloc;

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use dnf_core::{
    ast::Term,
    epl::ProgramAst,
    input_extractor::extract_events,
    interpreter::{eval_program, Event},
};

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

fn main() {
    // read AST Input for evaluation
    let mut epl: ProgramAst = env::read();
    let input_len = env::read();
    let mut bytes = vec![0u8; input_len];
    env::read_slice(&mut bytes);
    let index_list: Vec<(usize, usize)> = env::read();

    let input_event =  [br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"30","timestampUtc":"2025-12-18T16:45:03.484Z"}"#, br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"30","timestampUtc":"2025-12-18T16:45:03.484Z"}"#, br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"80","timestampUtc":"2025-12-18T16:45:03.484Z"}"#] ;

    let mut events: Vec<Event> = Vec::new();

    let schema0 = epl
        .schemas
        .get(0)
        .expect("schma[0] is not valid or missing");
    let byte_slice = bytes.as_slice();
    for index in index_list {
        events.push(extract_events(
            schema0.clone(),
            grap_event_string(index, byte_slice).as_bytes(),
        ));
    }

    let result = eval_program(epl, events);
    // Commit the result of the evaluation:
    env::commit(&result);
}

/*

pub fn fill_array() -> [&'a [u8]; 1000] {
    let mut inputs: [&[u8]; 1000];

    let mut i = 0;

    while i < 999 {

        inputs[i] = br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"40","timestampUtc":"2025-12-18T16:45:03.484Z"}"#;

        i = i + 1;
    }

    return inputs;
}

*/

fn grap_event_string<'a>(indexes: (usize, usize), bytes: &'a [u8]) -> &'a str {
    let (s, e) = indexes;
    core::str::from_utf8(&bytes[s..e]).unwrap()
}
