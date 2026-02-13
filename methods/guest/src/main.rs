#![no_std]
#![no_main]
extern crate alloc;

use alloc::{collections::BTreeMap, string::String, vec::Vec};

use dnf_core::{
    ast::{Disjunction, Term},
    interpreter::eval_ast,
};
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

fn main() {
    // read AST Input for evaluation
    let ast: Vec<Disjunction> = env::read();

    let mut my_data_args: BTreeMap<String, Term> = BTreeMap::new();
    my_data_args.insert("DataValue".into(), Term::Number(11));
    my_data_args.insert("OtherValue.".into(), Term::Bool(true));

    let ast_result = eval_ast(&ast, &my_data_args);

    // Commit the result of the evaluation:
    env::commit(&ast_result);
}
