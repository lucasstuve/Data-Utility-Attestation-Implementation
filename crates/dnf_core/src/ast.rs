// crates/dnf_core/src/ast.rs

//! Core term/predicate types matched by `dnf.pest`: a [`Disjunction`] of
//! [`Conjunction`]s of [`Pred`]s in disjunctive normal form (DNF), plus the
//! [`Term`] values and [`Operator`]s a predicate compares.

use core::fmt;

extern crate alloc;
use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

/// Comparison operator on the two sides of a [`Pred`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Operator {
    Eq,
    Sm,
    Gr,
    GrEq,
    SmEq,
    NEq,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operator::Eq => write!(f, "=="),
            Operator::NEq => write!(f, "!="),
            Operator::Gr => write!(f, ">"),
            Operator::Sm => write!(f, "<"),
            Operator::SmEq => write!(f, "<="),
            Operator::GrEq => write!(f, ">="),
        }
    }
}

/// A literal value, a schema-attribute reference (`Ident`), or an aggregate
/// expression, evaluated against a single [`crate::interpreter::Event`].
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Term {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
    Ident(String),

    Aggregate(crate::epl::Aggregate),
}

impl Term {
    pub fn equal_term_boolean(a: &Term, b: &Term) -> bool {
        matches!((a, b), (Term::Bool(_), Term::Bool(_)))
    }

    pub fn equal_term_str(a: &Term, b: &Term) -> bool {
        matches!((a, b), (Term::Str(_), Term::Str(_)))
    }

    pub fn equal_term_int(a: &Term, b: &Term) -> bool {
        matches!((a, b), (Term::Int(_), Term::Int(_)))
    }

    pub fn equal_term_ident(a: &Term, b: &Term) -> bool {
        matches!((a, b), (Term::Ident(_), Term::Ident(_)))
    }

    pub fn equal_term_float(a: &Term, b: &Term) -> bool {
        matches!((a, b), (Term::Float(_), Term::Float(_)))
    }
}

/// AND-ed group of predicates; true iff every [`Pred`] in `preds` is true.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Conjunction {
    pub preds: Vec<Pred>,
}

/// A single `lhs op rhs` comparison, e.g. `value > 20.0`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pred {
    pub lhs: Term,
    pub op: Operator,
    pub rhs: Term,
}

/// OR-ed group of conjunctions (the "D" in DNF); true iff any [`Conjunction`]
/// in `conj` is true.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Disjunction {
    pub conj: Vec<Conjunction>,
}
