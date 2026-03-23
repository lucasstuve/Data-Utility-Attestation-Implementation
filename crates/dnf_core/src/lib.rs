#![no_std]
pub mod ast;
pub mod epl;
pub mod input_extractor;
pub mod interpreter;

mod unit_tests_interpreter;

#[cfg(test)]
extern crate std;
