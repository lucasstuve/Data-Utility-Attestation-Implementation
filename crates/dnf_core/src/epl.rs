use core::fmt;
extern crate alloc;
use crate::ast::Disjunction;
use crate::ast::Term;

use alloc::{string::String, vec::Vec};

use serde::{Deserialize, Serialize};

//#[derive(Debug)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProgramAst {
    pub schemas: Vec<Schema>,
    pub assert_rules: Vec<AssertRule>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Attribute {
    pub ident: Term,
    pub i_type: Itype,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttributeList {
    pub list: Vec<Attribute>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Schema {
    pub ident: Term, // should be used as Ident
    pub attribute_list: AttributeList,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssertRule {
    pub ident: Term, // should be used as Ident
    pub rule: Vec<Disjunction>,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]

pub enum Itype {
    String,
    Int,
    Bool,
    Float,
}
