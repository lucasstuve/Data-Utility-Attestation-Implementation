use std::{any::Any, fmt};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Operator {
    Eq,
    Sm,
    Gr,
    GrEq,
    SmEq,
    NEq,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self {
            Operator::Eq => write!(f, "=="),
            Operator::NEq => write!(f, "!="),
            Operator::Gr => write!(f, ">"),
            Operator::Sm => write!(f, "<"),
            Operator::SmEq => write!(f, "<="),
            Operator::GrEq => write!(f, ">="),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
pub enum Term<'i> {
    Number(i64),
    Bool(bool),
    Str(&'i str),
    Ident(&'i str),
}

impl Term<'_> {
    pub fn equal_term_boolean<'i>(a: &Term, b: &Term) -> bool {
        let type_check: bool = match (a, b) {
            (Term::Bool(a_a), Term::Bool(b_b)) => true,
            _ => false,
        };

        return type_check;
    }
    pub fn equal_term_str<'i>(a: &Term, b: &Term) -> bool {
        let type_check = match (a, b) {
            (Term::Str(s1), Term::Str(s2)) => true,
            _ => false,
        };

        return type_check;
    }
    pub fn equal_term_number<'i>(a: &Term, b: &Term) -> bool {
        let type_check = match (a, b) {
            (Term::Number(n1), Term::Number(n2)) => true,
            _ => false,
        };

        return type_check;
    }
    pub fn equal_term_ident<'i>(a: &Term, b: &Term) -> bool {
        let type_check = match (a, b) {
            (Term::Ident(i1), Term::Ident(i2)) => true,
            _ => false,
        };

        return type_check;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Conjunction<'i> {
    pub preds: Vec<Pred<'i>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pred<'i> {
    pub lhs: Term<'i>,
    pub op: Operator,
    pub rhs: Term<'i>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Disjunction<'a> {
    pub conj: Vec<Conjunction<'a>>,
}
