extern crate alloc;

use alloc::vec;

use crate::ast::Conjunction;
use crate::ast::Disjunction;
use crate::ast::Pred;
use crate::ast::Term;

use alloc::{string::String, vec::Vec};

use serde::{Deserialize, Serialize};

//#[derive(Debug)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProgramAst {
    pub schemas: Vec<Schema>,
    pub assert_rules: Vec<AssertRule>,
    pub pattern_rule: Option<PatternRule>,
    pub aggregates: Vec<AssertRule>,
    pub window_rule: Option<Window>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

pub struct PatternRule {
    pub pattern_sequence: Vec<Disjunction>,
    pub identifier: String,
}

impl PatternRule {
    pub fn new() -> PatternRule {
        return PatternRule {
            pattern_sequence: vec![Disjunction {
                conj: vec![Conjunction {
                    preds: vec![Pred {
                        lhs: Term::Int(3),
                        rhs: Term::Int(34),
                        op: crate::ast::Operator::Gr,
                    }],
                }],
            }],
            identifier: "Test_Pattern_Rule".into(),
        };
    }
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

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct TimeWindow {
    pub w_width: Term,
    pub time_unit: TimeUnit,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Serialize, Deserialize)]

pub struct CountWindow {
    pub w_width: u64,
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

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize, Clone)]
pub enum TimeUnit {
    MS,
    S,
    MIN,
    H,
    D,
}

#[derive(Debug, PartialEq, Serialize, PartialOrd, Deserialize, Clone)]
pub enum Window {
    TimeWindow(TimeWindow),
    CountWindow(CountWindow),
}
