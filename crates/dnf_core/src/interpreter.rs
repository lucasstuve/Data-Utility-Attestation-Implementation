use crate::ast::{Conjunction, Disjunction, Operator, Pred, Term};
use crate::epl::{
    AggOperation, PatternRule, ProgramAst, Quantifier, Schema, TimeUnit, TimeWindow, Window,
};
extern crate alloc;
use alloc::{collections::BTreeMap, string::String, vec::Vec};
use chrono::{DateTime, FixedOffset, NaiveDateTime, Timelike, Utc};
use libm::{nextafter, sqrt};

pub type Env = BTreeMap<String, Term>;

#[derive(Debug, PartialEq, Clone)]
pub struct Event {
    pub data: Vec<Term>,
}

pub fn eval_program(p: ProgramAst, input: Vec<Event>) -> bool {
    let schema = &p.schemas[0];
    //let mut event_data: Vec<Term> = Vec::new();
    let mut filter_result = true;

    // Create sub-set of event data (fullfills ASSERT rule)
    for r in &p.assert_rules {
        if r.quantifier == Quantifier::ALL {
            filter_result = input.iter().all(|e| {
                let env = type_input(
                    Event {
                        data: e.data.clone(),
                    },
                    schema,
                );
                eval_filter_dnf(&r.rule, &env)
            });
        } else if r.quantifier == Quantifier::ANY {
            filter_result = input.iter().any(|e| {
                let env = type_input(
                    Event {
                        data: e.data.clone(),
                    },
                    schema,
                );
                eval_filter_dnf(&r.rule, &env)
            });
        }
    }

    let mut pattern_result = true;
    //let mut env: BTreeMap<String, Term> = BTreeMap::new();
    // PATTERN evaluation:
    if p.pattern_rule != None {
        let mut event_envs: Vec<BTreeMap<String, Term>> = Vec::new();
        for e in &input {
            event_envs.push(type_input(
                Event {
                    data: e.data.clone(),
                },
                schema,
            ));
        }

        pattern_result = eval_pattern_rule(event_envs, p.pattern_rule);
    }

    let mut window_eval_flag: bool = p.window_rule.is_some();

    // Collect the event data that passes the filter criteria
    let dnf_filter = &p.assert_rules[0].rule;
    let mut event_data = Vec::new();
    for e in &input {
        let env = type_input(
            Event {
                data: e.data.clone(),
            },
            schema,
        );
        if (&p.aggregates.len() > &(0 as usize)) && eval_filter_dnf(dnf_filter, &env) {
            collect_event_data(&env, &p.aggregates[0].ident, &mut event_data);
        }
    }

    let mut agg_pred_result = false;

    // Compute aggregate on filtered data

    let mut single_window_eval = false;

    if window_eval_flag && p.aggregates.is_empty() {
        let windows = eval_window_rule(p.window_rule.clone().unwrap(), input.clone(), 2 as usize);
        if windows.is_empty() {
            single_window_eval = false;
        }
    }

    if window_eval_flag && !p.aggregates.is_empty() {
        let ident = get_aggreage_assert_ident(&p.aggregates[0].rule).unwrap();
        let aggregate = &p.aggregates[0];
        let op = get_aggreage_assert_op(&p.aggregates[0].rule).unwrap();
        let pred = &p.aggregates[0].rule[0].conj[0].preds[0];
        let mut aggregate_env: BTreeMap<String, Term> = BTreeMap::new();
        //let window_rule = p.window_rule.clone().unwrap();

        let windows = eval_window_rule(p.window_rule.clone().unwrap(), input.clone(), 2 as usize);

        //  let window_pred_result = eval_pred(pred, &aggreate_env);

        agg_pred_result = windows.into_iter().all(|window| {
            let mut data = Vec::new();

            for e in window {
                let env = type_input(e, schema);
                if eval_filter_dnf(dnf_filter, &env) {
                    collect_event_data(&env, &aggregate.ident, &mut data);
                }
            }
            let Some(agg_value) = eval_agg_data(op.clone(), data) else {
                return false;
            };
            let mut aggregate_env = BTreeMap::new();
            aggregate_env.insert(ident.clone(), agg_value);

            eval_pred(pred, &aggregate_env)
        });
    } else {
        if p.aggregates.len() != 0 {
            let op = get_aggreage_assert_op(&p.aggregates[0].rule).unwrap();
            let aggregation_result = eval_agg_data(op, event_data).unwrap();

            // Evaluate predicate on aggregation function result
            let ident = get_aggreage_assert_ident(&p.aggregates[0].rule).unwrap();
            let mut aggregate_env = BTreeMap::new();
            aggregate_env.insert(ident, aggregation_result);

            let pred = &p.aggregates[0].rule[0].conj[0].preds[0];
            agg_pred_result = eval_pred(pred, &aggregate_env);
        }
    }

    return filter_result && ((agg_pred_result && single_window_eval) || pattern_result);
}

pub fn collect_event_data(event: &BTreeMap<String, Term>, ident: &String, out: &mut Vec<Term>) {
    if let Some(t) = event.get(ident) {
        out.push(t.clone());
    }
}

pub fn eval_pattern_rule(events: Vec<BTreeMap<String, Term>>, p_rule: Option<PatternRule>) -> bool {
    let pattern_rule = p_rule.unwrap();
    let pattern = pattern_rule.pattern_sequence;

    return events.windows(pattern.len()).any(|sequence| {
        sequence
            .iter()
            .zip(pattern.iter())
            .all(|(event_env, disj)| {
                return eval_disj(disj, event_env);
            })
    });
}

pub fn flatten_window(windows: Vec<Vec<Event>>) -> Vec<Vec<Term>> {
    let mut flatted_windows = Vec::new();

    for w in windows {
        let mut window = Vec::new();
        for e in w {
            window.extend(e.data);
        }
        flatted_windows.push(window);
    }
    return flatted_windows;
}

pub fn eval_window_rule(w: Window, mut events: Vec<Event>, utc_pos: usize) -> Vec<Vec<Event>> {
    match w {
        Window::CountWindow(count_win) => {
            let mut c_windows = Vec::new();

            while events.len() > count_win.w_width as usize {
                let rest = events.split_off(count_win.w_width as usize);

                c_windows.push(events);
                events = rest;
            }

            return c_windows;
        }
        Window::TimeWindow(TimeWindow { time_unit, w_width }) => {
            let window_in_ms = get_time_window_ms(time_unit, w_width);
            let mut tstemp_events: Vec<(Event, DateTime<FixedOffset>)> = events
                .into_iter()
                .map(|e| {
                    let dt = match &e.data[utc_pos] {
                        Term::Str(s) => {
                            /*  NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")
                            .unwrap_or_else(|err| {
                                panic!("Timestamp not valid!{:?}, {:?}", s, err)
                            })
                            .and_utc() */
                            DateTime::parse_from_rfc3339(s).unwrap()
                        }
                        _ => panic!("Expected (TimeStampFormat might vary) string"),
                    };
                    (e, dt)
                })
                .collect();

            tstemp_events.sort_by_key(|(_, d)| *d);

            let mut window: Vec<Event> = Vec::new();
            let mut win_vec: Vec<Vec<Event>> = Vec::new();

            if tstemp_events.is_empty() {
                return win_vec;
            }

            let mut window_start = tstemp_events[0].1.timestamp_millis();

            for (e, d) in tstemp_events {
                if d.timestamp_millis() < window_start + window_in_ms {
                    window.push(e);
                } else {
                    win_vec.push(window.clone());
                    window.clear();
                    window.push(e);
                    window_start = d.timestamp_millis();
                }
            }

            if !window.clone().is_empty() {
                win_vec.push(window);
            }

            return win_vec;
        }
    }
}

pub fn get_time_window_ms(time_unit: TimeUnit, w_width: Term) -> i64 {
    match w_width {
        Term::Int(i) => match time_unit {
            TimeUnit::MS => i as i64,
            TimeUnit::S => (i * 1000) as i64,
            TimeUnit::MIN => (60 * i * 1000) as i64,
            TimeUnit::H => (60 * 60 * i * 1000) as i64,
            TimeUnit::D => (24 * 60 * 60 * i * 1000) as i64,
            _ => unimplemented!("Please choose a supported time unit: MS, S, MIN, H, D"),
        },
        Term::Float(f) => match time_unit {
            TimeUnit::MS => f as i64,
            TimeUnit::S => (f * 1000.0) as i64,
            TimeUnit::MIN => (60_f64 * f * 1000.0) as i64,
            TimeUnit::H => (60.0 * 60.0 * f * 1000.0) as i64,
            TimeUnit::D => (24.0 * 60.0 * 60.0 * f * 1000.0) as i64,
            _ => unimplemented!("Please choose a supported time unit: MS, S, MIN, H, D"),
        },
        _ => unimplemented!("Only Int or Float a valid types to specify the TimeWindow!"),
    }
}

pub fn eval_agg_data(op: AggOperation, data: Vec<Term>) -> Option<Term> {
    let result = match op {
        AggOperation::AVG => avg(&data),
        AggOperation::COUNT => count(&data),
        AggOperation::SUM => sum(&data),
        AggOperation::MEDIAN => median(&data),
        AggOperation::MIN => min(&data),
        AggOperation::MAX => max(&data),
        AggOperation::STDDEV => stddev(&data), //TODO think about MINEVER, MAXEVER function

        _ => unimplemented!("No valid aggregation operation, or not yet implemented!"),
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
    return Some(Term::Int(data.len() as i64));
}

pub fn stddev(data: &[Term]) -> Option<Term> {
    let n = data.len();

    let mean_f64: f64 = match avg(&data).unwrap() {
        Term::Float(f) => f,
        Term::Int(i) => i as f64,
        _ => return None,
    };

    let inner_term: f64 = data
        .iter()
        .map(|t| match *t {
            Term::Float(f) => {
                let d = f - mean_f64;
                d * d
            }
            Term::Int(i) => {
                let d = (i as f64) - mean_f64;
                (d * d) as f64
            }
            _ => unreachable!("STDDEV is only implemented for numerics (Int, Float)."),
        })
        .sum();

    let var = inner_term / ((n - 1) as f64);
    let sd = sqrt(var);

    return Some(Term::Float(sd));
}
pub fn max(data: &[Term]) -> Option<Term> {
    let mut float_values = Vec::new();
    let mut int_values = Vec::new();

    //let mut int_values = [i64];
    let mut is_int = true;

    for t in data {
        match *t {
            Term::Float(f) => {
                float_values.push(f);
                is_int = false;
            }
            Term::Int(i) => int_values.push(i),
            _ => unreachable!("Median is only implemented for numerics (int, float)!"),
        }
    }

    if is_int {
        return int_values.iter().copied().max().map(Term::Int);
    } else {
        return float_values
            .iter()
            .copied()
            .max_by(|a, b| a.total_cmp(b))
            .map(Term::Float);
    }
}

pub fn min(data: &[Term]) -> Option<Term> {
    let mut float_values = Vec::new();
    let mut int_values = Vec::new();

    //let mut int_values = [i64];
    let mut is_int = true;

    for t in data {
        match *t {
            Term::Float(f) => {
                float_values.push(f);
                is_int = false;
            }
            Term::Int(i) => int_values.push(i),
            _ => unreachable!("Median is only implemented for numerics (int, float)!"),
        }
    }

    if is_int {
        return int_values.iter().copied().min().map(Term::Int);
    } else {
        return float_values
            .iter()
            .copied()
            .min_by(|a, b| a.total_cmp(b))
            .map(Term::Float);
    }
}

pub fn sum(data: &[Term]) -> Option<Term> {
    let mut result_f: f64 = 0.0;
    let mut result_i: i64 = 0;
    let mut is_int = true;

    for t in data {
        match t {
            Term::Float(f) => {
                result_f = result_f + f;
                is_int = false;
            }
            Term::Int(i) => {
                result_i = result_i + i;
                is_int = true;
            }
            _ => unreachable!("Only for numeric types int, float implemented!"),
        }
    }

    if is_int {
        return Some(Term::Int(result_i));
    } else {
        return Some(Term::Float(result_f));
    }
}

pub fn median(data: &[Term]) -> Option<Term> {
    let mut float_values = Vec::new();
    let mut int_values = Vec::new();

    //let mut int_values = [i64];
    let mut is_int = true;

    for t in data {
        match *t {
            Term::Float(f) => {
                float_values.push(f);
                is_int = false;
            }
            Term::Int(i) => int_values.push(i),
            _ => unreachable!("Median is only implemented for numerics (int, float)!"),
        }
    }
    let mid_f = float_values.len() / 2;
    let mid_i = int_values.len() / 2;

    float_values.sort_by(|a, b| a.total_cmp(b));
    int_values.sort();

    if is_int {
        return Some(Term::Int(int_values[mid_i]));
    } else {
        return Some(Term::Float(float_values[mid_f]));
    }
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
        Term::Aggregate(a) => env.get(&a.ident).cloned(),
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

#[cfg(test)]
mod test_interpreter {
    use crate::{
        ast::Term,
        interpreter::{avg, count, max, median, min, stddev, sum},
    };

    #[test]
    fn test_median_int_odd_number() {
        let int_terms = [
            Term::Int(23),
            Term::Int(234),
            Term::Int(2),
            Term::Int(56),
            Term::Int(45),
        ];

        assert_eq!(median(&int_terms).unwrap(), Term::Int(45));
    }

    #[test]
    fn test_median_int_even_number() {
        let int_terms = [Term::Int(23), Term::Int(234), Term::Int(56), Term::Int(45)];

        assert_eq!(median(&int_terms).unwrap(), Term::Int(56)); // means => Index 2,5 => 3
    }
    #[test]
    fn test_median_float_odd_number() {
        let int_terms = [
            Term::Float(23.0),
            Term::Float(234.0),
            Term::Float(56.0),
            Term::Float(45.0),
            Term::Float(12.0),
        ];

        assert_eq!(median(&int_terms).unwrap(), Term::Float(45.0));
    }

    #[test]
    fn test_avg_int_odd_number() {
        let int_terms = [
            Term::Int(23),
            Term::Int(234),
            Term::Int(2),
            Term::Int(56),
            Term::Int(45),
        ];

        assert_eq!(avg(&int_terms).unwrap(), Term::Int(72));
    }
    #[test]
    fn test_avg_int_even_number() {
        let int_terms = [Term::Int(23), Term::Int(234), Term::Int(56), Term::Int(45)];

        assert_eq!(avg(&int_terms).unwrap(), Term::Int(89)); //TODO change return for Int input => Float value  // Or cast correctly
    }

    #[test]
    fn test_avg_float_even_number() {
        let int_terms = [
            Term::Float(23.0),
            Term::Float(234.0),
            Term::Float(56.0),
            Term::Float(45.0),
        ];

        assert_eq!(avg(&int_terms).unwrap(), Term::Float(89.5));
    }

    #[test]
    fn test_max_float_even_number() {
        let int_terms = [
            Term::Float(23.0),
            Term::Float(234.0),
            Term::Float(56.0),
            Term::Float(45.0),
        ];

        assert_eq!(max(&int_terms).unwrap(), Term::Float(234.0));
    }

    #[test]
    fn test_max_float_odd_number() {
        let int_terms = [
            Term::Float(23.0),
            Term::Float(234.0),
            Term::Float(56.0),
            Term::Float(45.0),
            Term::Float(550.0),
        ];

        assert_eq!(max(&int_terms).unwrap(), Term::Float(550.0));
    }

    #[test]
    fn test_sum_float_odd_number() {
        let int_terms = [
            Term::Float(23.0),
            Term::Float(234.0),
            Term::Float(56.0),
            Term::Float(45.0),
            Term::Float(550.0),
        ];

        assert_eq!(sum(&int_terms).unwrap(), Term::Float(908.0));
    }

    #[test]
    fn test_stddev_float_odd_number() {
        let int_terms = [
            Term::Float(23.0),
            Term::Float(234.0),
            Term::Float(56.0),
            Term::Float(45.0),
            Term::Float(550.0),
        ];

        assert_eq!(stddev(&int_terms).unwrap(), Term::Float(222.51584213264456));
    }

    #[test]
    fn test_stddev_int_odd_number() {
        let int_terms = [
            Term::Int(23),
            Term::Int(234),
            Term::Int(56),
            Term::Int(45),
            Term::Int(550),
        ];

        assert_eq!(stddev(&int_terms).unwrap(), Term::Float(222.51685329430669));
        // TODO left Float, where right is Int()
    }

    #[test]
    fn test_count_int_odd_number() {
        let int_terms = [
            Term::Int(23),
            Term::Int(234),
            Term::Int(56),
            Term::Int(45),
            Term::Int(550),
        ];

        assert_eq!(count(&int_terms).unwrap(), Term::Int(5)); // TODO left Float, where right is Int()
    }

    #[test]
    fn test_count_int_even_number() {
        let int_terms = [
            Term::Int(23),
            Term::Int(234),
            Term::Int(56),
            Term::Int(45),
            Term::Int(550),
            Term::Int(342),
        ];

        assert_eq!(count(&int_terms).unwrap(), Term::Int(6)); // TODO left Float, where right is Int()
    }

    #[test]
    fn test_count_flaot_even_number() {
        let int_terms = [
            Term::Float(23.0),
            Term::Float(234.0),
            Term::Float(56.0),
            Term::Float(45.0),
            Term::Float(550.0),
            Term::Float(342.0),
        ];

        assert_eq!(count(&int_terms).unwrap(), Term::Int(6)); // TODO left Float, where right is Int()
    }
}
