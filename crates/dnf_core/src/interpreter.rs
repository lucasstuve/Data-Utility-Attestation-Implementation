//use std::process::id;

use crate::ast::{Conjunction, Disjunction, Operator, Pred, Term};
use crate::epl::{AggOperation, ProgramAst, Schema};
extern crate alloc;
use alloc::{collections::BTreeMap, string::String, vec::Vec};

pub type Env = BTreeMap<String, Term>;

#[derive(Debug)]
pub struct Event {
    pub data: Vec<Term>,
}

pub fn eval_program(p: ProgramAst, input: Vec<Event>) -> bool {
    let schema = &p.schemas[0];
    let mut event_data: Vec<Term> = Vec::new();
    p.assert_rules.iter().all(|r| {
        input.iter().any(|e| {
            let env = type_input(
                Event {
                    data: e.data.clone(),
                },
                schema,
            );
            //let counter_result = eval_aggreagtor();

            let filter = eval_filter_dnf(&r.rule, &env);

            if filter {
                collect_event_data(&env, &p.aggregates[0].ident, &mut event_data);
            };

            let agg = eval_agg_data(
                get_aggreage_assert_op(&p.aggregates[0].rule.clone()).unwrap(),
                event_data.clone(),
            );

            let mut aggregate_env: BTreeMap<String, Term> = BTreeMap::new();
            let ident = get_aggreage_assert_ident(&p.aggregates[0].rule.clone()).unwrap();
            aggregate_env.insert(ident, agg.unwrap());
            let aggreage_rule = eval_filter_dnf(&p.aggregates[0].rule, &aggregate_env);

            return filter && aggreage_rule;
        })
    })
}

pub fn collect_event_data(event: &BTreeMap<String, Term>, ident: &String, out: &mut Vec<Term>) {
    if let Some(t) = event.get(ident) {
        out.push(t.clone());
    }
}

pub fn eval_agg_data(op: AggOperation, data: Vec<Term>) -> Option<Term> {
    let result = match op {
        AggOperation::AVG => avg(&data),
        AggOperation::COUNT => count(&data),
        _ => unimplemented!("other analytics will be implemented!"),
    };

    return result;
}

pub fn avg(data: &[Term]) -> Option<Term> {
    let (sum, count, is_int) =
        data.iter()
            .fold((0.0f64, 0f64, true), |(s, c, is_int), t| match t {
                Term::Int(i) => (s + *i as f64, c + (1 as f64), true),
                Term::Float(f) => (s + f, c + (1 as f64), false),
                _ => (s, c, is_int),
            });

    if count == 0 as f64 {
        None
    } else if is_int {
        Some(Term::Int((sum / count) as i64))
    } else {
        Some(Term::Float(sum / count as f64))
    }
}

pub fn count(data: &[Term]) -> Option<Term> {
    let count = data.iter().count();

    return Some(Term::Float(count as f64));
}

pub fn as_f64(t: &Term) -> Option<f64> {
    match t {
        Term::Float(f) => Some(*f),
        _ => unimplemented!("Must be Float."),
    }
}

pub fn as_i64(t: &Term) -> Option<i64> {
    match t {
        Term::Int(i) => Some(*i),
        _ => unimplemented!("Must be Int."),
    }
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

fn get_aggreage_assert_ident(dnf: &Vec<Disjunction>) -> Option<String> {
    match &dnf.first()?.conj.first()?.preds.first()?.lhs {
        Term::Aggregate(a) => Some(a.ident.clone()),
        _ => None,
    }
}

fn get_aggreage_assert_op(dnf: &Vec<Disjunction>) -> Option<AggOperation> {
    match &dnf.first()?.conj.first()?.preds.first()?.lhs {
        Term::Aggregate(a) => Some(a.operation.clone()),
        _ => None,
    }
}
