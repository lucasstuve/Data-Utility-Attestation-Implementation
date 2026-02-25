#![no_std]
#![no_main]
extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;

use dnf_core::{
    epl::ProgramAst,
    input_extractor::{extract_events, grap_event_string},
    interpreter::{eval_program, Event},
};

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

fn main() {
    // read AST Input for evaluation
    let epl: ProgramAst = env::read();
    let input_len: u32 = env::read();
    let mut bytes = vec![0u8; input_len as usize];
    env::read_slice(bytes.as_mut_slice());
    let index_list: Vec<(u32, u32)> = env::read();

    let mut events: Vec<Event> = Vec::new();

    let schema0 = epl
        .schemas
        .get(0)
        .expect("schema[0] is not valid or missing");

    let byte_slice = bytes.as_slice();

    for index in index_list {
        events.push(extract_events(
            schema0.clone(),
            grap_event_string(index, byte_slice).as_bytes(),
        ));
    }

    // start the evaluator to obtain the boolean result over all event data
    let result = eval_program(epl, events);
    
    // Commit the programs result to the journal
    env::commit(&result);
}
