use crate::ast::{Conjunction, Disjunction, Operator, Pred, Term};
use crate::epl::{ProgramAst, Schema};
extern crate alloc;
use alloc::{collections::BTreeMap, string::String, vec::Vec};

pub type Env = BTreeMap<String, Term>;

#[derive(Debug)]
pub struct Event {
    pub data: Vec<Term>,
}

pub fn eval_program(p: ProgramAst, input: Vec<Event>) -> bool {
    let schema = &p.schemas[0];

    p.assert_rules.iter().all(|r| {
        input.iter().any(|e| {
            let env = type_input(
                Event {
                    data: e.data.clone(),
                },
                schema,
            );
            eval_filter_dnf(&r.rule, &env)
        })
    })
}

pub fn type_input(e: Event, s: &Schema) -> BTreeMap<String, Term> {
    let mut data_args: BTreeMap<String, Term> = BTreeMap::new();

    for (idx, attr) in s.attribute_list.list.iter().enumerate() {
        data_args.insert(attr.ident.clone(), e.data[idx].clone());
    }

    return data_args;
}

pub fn eval_filter_dnf(dis: &Vec<Disjunction>, env: &Env) -> bool {
    let mut result = false;

    for d in dis {
        result = result || eval_disj(&d, env);
    }

    return result;
}

fn eval_disj(dis: &Disjunction, env: &Env) -> bool {
    let mut result = false;
    for c in &dis.conj {
        result = result || eval_conj(&c, env);
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
        (Term::Int(a), Term::Int(b)) => operator_to_function::<i64>(p.op)(a, b),
        (Term::Float(a), Term::Float(b)) => operator_to_function::<f64>(p.op)(a, b),
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::ast::{Operator, Pred, Term};

    #[test]
    fn test_eval_pred_ident_number() {
        let mut env: Env = BTreeMap::new();
        env.insert("MyValue".into(), Term::Int(11));

        let p = Pred {
            lhs: Term::Ident("MyValue".into()),
            rhs: Term::Int(10),
            op: Operator::Gr,
        };

        assert_eq!(eval_pred(&p, &env), true);
    }
}
