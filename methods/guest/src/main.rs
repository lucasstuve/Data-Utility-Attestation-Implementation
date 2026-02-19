#![no_std]
#![no_main]
extern crate alloc;

use alloc::vec::Vec;

use dnf_core::{
    ast::Term,
    epl::ProgramAst,
    interpreter::{eval_program, Event},
};

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

fn main() {
    // read AST Input for evaluation
    let epl: ProgramAst = env::read();

    let mut data = Vec::new();
    data.insert(0, Term::Number(2392));
    data.insert(1, Term::Bool(true));

    let event = Event { data: data };

    let mut events: Vec<Event> = Vec::new();
    events.insert(0, event);

    let result = eval_program(epl, events);
    // Commit the result of the evaluation:
    env::commit(&result);
}
