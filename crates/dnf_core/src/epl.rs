//! Program-level AST: the [`ProgramAst`] produced by [`crate`]-external
//! parsing (`host::parser::parse_source`) and evaluated by
//! [`crate::interpreter::eval_program`], plus its schema/rule/window/session
//! building blocks.

extern crate alloc;

use alloc::vec;

use crate::ast::Conjunction;
use crate::ast::Disjunction;
use crate::ast::Pred;
use crate::ast::Term;

use alloc::{string::String, vec::Vec};

use serde::{Deserialize, Serialize};

/// The parsed representation of one EPL program: the canonical utility
/// predicate that `eval_program` evaluates against an event batch.
//#[derive(Debug)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProgramAst {
    pub schemas: Vec<Schema>,
    pub assert_rules: Vec<AssertRule>,
    pub pattern_rule: Option<PatternRule>,
    pub aggregates: Vec<AssertRule>,
    pub window_rule: Option<Window>,
    pub session: Option<Session>,
}

impl ProgramAst {
    /// Debug-formats the AST to bytes; hashed to commit to the evaluated
    /// logic without revealing the underlying DSL source (see the zkVM
    /// guest's `logic_commitment`).
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = alloc::format!("{:?}", self).into_bytes();
        return bytes;
    }
}


/// A named sequence of event conditions that must occur in order (with a
/// wraparound back to the first condition allowed between matches).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Session {
    pub identifier: String,
    pub session_sequence: Vec<Disjunction>,
}

/// A named sequence of event conditions that must occur in strict,
/// consecutive order within the event batch.
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

/// Declares the shape of one event: a name plus its typed attribute list,
/// used to map raw event bytes onto [`crate::interpreter::Event`] fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Schema {
    pub ident: String, // should be used as Ident
    pub attribute_list: AttributeList,
}

/// A DNF condition (`rule`) that events are filtered/selected by, combined
/// with a [`Quantifier`] deciding whether `ANY` or `ALL` events must match.
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
