use core::fmt;
extern crate alloc;
use crate::ast::Disjunction;
use crate::ast::Term;

use alloc::{string::String, vec::Vec};

use serde::{Deserialize, Serialize};

//#[derive(Debug)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProgramAst {
    pub schemas: Vec<Schema>,
    pub assert_rules: Vec<AssertRule>,
    pub aggregates: Vec<AssertRule>,
}
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Aggregate {
    pub ident: String,
    pub operation: AggOperation,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Attribute {
    pub ident: String,
    pub i_type: Itype,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttributeList {
    pub list: Vec<Attribute>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Schema {
    pub ident: String, // should be used as Ident
    pub attribute_list: AttributeList,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssertRule {
    pub ident: String, // should be used as Ident
    pub rule: Vec<Disjunction>,
    pub quantifier: Quantifier,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]

pub enum Itype {
    String,
    Int,
    Bool,
    Float,
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize, Clone)]
pub enum Quantifier {
    ANY,
    ALL,
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize, Clone)]
pub enum AggOperation {
    COUNT,
    AVG,
    MAX,
    MEDIAN,
    MIN,
    SUM,
    STDDEV,
    MAXEVER,
    MINEVER,
}
