use crate::ast::{Conjunction, Disjunction, Operator, Pred, Term};
//use std::collections::HashMap;

extern crate alloc;

use alloc::{collections::BTreeMap, string::String, vec::Vec};

pub type Env = BTreeMap<String, Term>;

pub fn eval_ast(dis: &Vec<Disjunction>, env: &Env) -> bool {
    let mut result = false;

    for d in dis {
        result = result || eval_disj(&d, env);
    }

    return result;
}

fn eval_disj(dis: &Disjunction, env: &Env) -> bool {
    let mut result = false;
    for c in &dis.conj {
        result = result | eval_conj(&c, env);
    }
    return result;
}

fn eval_conj(conj: &Conjunction, env: &Env) -> bool {
    let mut result = true;

    for pred in &conj.preds {
        result = result && eval_pred(&pred, env);
    }
    return result;
}

fn resolve_term(t: &Term, env: &Env) -> Option<Term> {
    match t {
        Term::Ident(name) => env.get(name).cloned(),
        _ => Some(t.clone()),
    }
}

fn eval_pred(p: &Pred, env: &Env) -> bool {
    let lhs = match resolve_term(&p.lhs, env) {
        Some(v) => v,
        None => return false, // Variable nicht gesetzt
    };
    let rhs = match resolve_term(&p.rhs, env) {
        Some(v) => v,
        None => return false,
    };

    match (lhs, rhs) {
        (Term::Bool(a), Term::Bool(b)) => operator_to_function::<bool>(p.op)(a, b),
        (Term::Number(a), Term::Number(b)) => operator_to_function::<i64>(p.op)(a, b),
        (Term::Str(a), Term::Str(b)) => operator_to_function::<String>(p.op)(a, b),

        // Typmix (z.B. Number vs Str) => false (oder später Result/Error)
        _ => false,
    }
}
fn operator_to_function<T: PartialEq + PartialOrd>(op: Operator) -> fn(a: T, b: T) -> bool {
    match op {
        Operator::Eq => |a, b| a == b,
        Operator::NEq => |a, b| a != b,
        Operator::Gr => |a, b| a > b,
        Operator::GrEq => |a, b| a >= b,
        Operator::Sm => |a, b| a < b,
        Operator::SmEq => |a, b| a <= b,
    }
}

//pub type Env = HashMap<&str, Term>;

fn eval_bool(term: &Term) -> Option<bool> {
    match term {
        Term::Bool(b) => Some(*b),
        _ => None,
    }
}

fn eval_str<'a>(term: &'a Term) -> Option<&'a str> {
    match term {
        Term::Str(s) => Some(s),
        _ => None,
    }
}

fn eval_ident<'a>(term: &'a Term) -> Option<&'a str> {
    match term {
        Term::Ident(i) => Some(i),
        _ => None,
    }
}

fn eval_number(t: &Term) -> Option<i64> {
    match t {
        Term::Number(n) => Some(*n),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ast::{Operator, Pred, Term};

    #[test]
    fn test_eval_pred_ident_number() {
        let mut env: Env = BTreeMap::new();
        env.insert("MyValue".into(), Term::Number(11));

        let p = Pred {
            lhs: Term::Ident("MyValue".into()),
            rhs: Term::Number(10),
            op: Operator::Gr,
        };

        assert_eq!(eval_pred(&p, &env), true);
    }
}
