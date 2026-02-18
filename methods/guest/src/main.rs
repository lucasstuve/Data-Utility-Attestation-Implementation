#![no_std]
#![no_main]
extern crate alloc;

use alloc::{collections::BTreeMap, string::String, vec::Vec};

use dnf_core::{
    ast::{Disjunction, Term},
    epl::{AssertRule, Attribute, AttributeList, ProgramAst},
    interpreter::eval_ast,
};

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

fn main() {
    // read AST Input for evaluation
    let ast: ProgramAst = env::read();

    let mut my_data_args: BTreeMap<String, Term> = BTreeMap::new();
    my_data_args.insert("speed".into(), Term::Number(11));
    my_data_args.insert("open".into(), Term::Bool(true));

    let mut result = true;

    for rule in &ast.assert_rules {
        result = eval_ast(&rule.rule, &my_data_args) & result;
    }

    // Commit the result of the evaluation:
    env::commit(&result);
}
