use crate::ast::{Conjunction, Disjunction, Operator, Pred, Term};
use crate::epl::{
    AggOperation, PatternRule, ProgramAst, Quantifier, Schema, Session, TimeUnit, TimeWindow,
    Window,
};
extern crate alloc;
use alloc::{collections::BTreeMap, string::String, vec::Vec};
use chrono::{DateTime, FixedOffset};
use libm::sqrt;
use serde::{Deserialize, Serialize};

pub type Env = BTreeMap<String, Term>;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Event {
    pub data: Vec<Term>,
}

pub fn eval_program(p: &ProgramAst, input: &Vec<Event>) -> bool {
    let has_assrt: bool = !&p.assert_rules.is_empty();
    let has_patt: bool = !(p.pattern_rule == None);
    let has_aggr: bool = !&p.aggregates.is_empty();
    let has_wind: bool = !(p.window_rule == None);
    let has_session: bool = !(p.session == None);

    let dnf_filter = &p.assert_rules[0].rule;

    let mut session_result: bool = false;

    let schema = &p.schemas[0];
    let mapping = mapping_from_schema(&schema);

    let mut event_data = Vec::new();
    for e in input.iter() {
        if eval_filter_dnf(dnf_filter, &mapping, &e) {
            event_data.push(e.clone());
        }
    }

    //let mut event_data: Vec<Term> = Vec::new();
    let mut filter_result = true;

    // Create sub-set of event data (fullfills ASSERT rule)

    for r in &p.assert_rules {
        let rule_result: bool = if r.quantifier == Quantifier::ALL {
            input.iter().all(|e| eval_filter_dnf(&r.rule, &mapping, &e))
        } else {
            input.iter().any(|e| eval_filter_dnf(&r.rule, &mapping, &e))
        };
        filter_result = filter_result && rule_result;
    }

    if has_session {
        session_result = eval_session(&event_data, &mapping, &p.session);
    }

    let mut pattern_result = true;

    // PATTERN evaluation:
    if p.pattern_rule != None {
        pattern_result = eval_pattern_rule(&event_data, &mapping, &p.pattern_rule);
    }

    let window_eval_flag: bool = p.window_rule.is_some();

    // Collect the event data that passes the filter criteria

    let mut agg_pred_result = false;

    // Compute aggregate on filtered data

    let mut single_window_eval: bool = false;

    if window_eval_flag && p.aggregates.is_empty() {
        let windows = eval_window_rule(
            &p.window_rule.as_ref().unwrap(),
            event_data.clone(),
            &2usize,
        );
        single_window_eval = !windows.is_empty();
    }

    // AGGREGATE EVALUATUION: START
    if window_eval_flag && !p.aggregates.is_empty() {
        let ident = get_aggreage_assert_ident(&p.aggregates[0].rule).unwrap();

        let op = get_aggreage_assert_op(&p.aggregates[0].rule).unwrap();
        let pred = &p.aggregates[0].rule[0].conj[0].preds[0];
        //let window_rule = p.window_rule.clone().unwrap();

        let windows = eval_window_rule(&p.window_rule.as_ref().unwrap(), event_data, &2usize);

        //  let window_pred_result = eval_pred(pred, &aggreate_env);

        agg_pred_result = windows.into_iter().all(|window| {
            let mut data = Vec::new();

            let mapped_index = *mapping.get(&ident).unwrap();

            for e in window {
                data.push(e.data[mapped_index].clone());
            }
            let Some(agg_value) = eval_agg_data(&op, &data) else {
                return false;
            };
            let mut e_data = alloc::vec![Term::Int(0); mapping.len()];
            e_data[mapped_index] = agg_value.clone();
            let artificial_event = Event { data: e_data };
            eval_pred(pred, &mapping, &artificial_event)
        });
    } else if !p.aggregates.is_empty() {
        let op = get_aggreage_assert_op(&p.aggregates[0].rule).unwrap();
        let ident = get_aggreage_assert_ident(&p.aggregates[0].rule).unwrap();
        let mapped_index = *mapping.get(&ident).unwrap();
        let pred = &p.aggregates[0].rule[0].conj[0].preds[0];
        let mut data = Vec::new();

        for e in event_data.clone() {
            data.push(e.data[mapped_index].clone());
        }

        let Some(aggregation_result) = eval_agg_data(&op, &data) else {
            return false;
        };

        let mut e_data = alloc::vec![Term::Int(0); mapping.len()];
        e_data[mapped_index] = aggregation_result;
        let artificial_event = Event { data: e_data };

        agg_pred_result = eval_pred(pred, &mapping, &artificial_event);
    }

    if !has_aggr && has_assrt && !has_patt && !has_wind && has_session {
        return filter_result && session_result;
    } else if has_aggr && has_assrt && !has_patt && !has_wind {
        return agg_pred_result;
    } else if !has_aggr && has_assrt && !has_patt && has_wind {
        return filter_result && single_window_eval;
    } else if has_aggr && has_assrt && !has_patt && has_wind {
        return filter_result && agg_pred_result;
    } else if !has_aggr && has_assrt && has_patt && !has_wind {
        return filter_result && pattern_result;
    } else if !has_aggr && has_assrt && !has_patt && !has_wind && !has_session {
        return filter_result;
    } else {
        return false;
    }
}

pub fn eval_session(
    events: &Vec<Event>,
    mapping: &BTreeMap<String, usize>,
    session_def: &Option<Session>,
) -> bool {
    let session = session_def.as_ref().unwrap();
    let sequence = &session.session_sequence;

    let mut seq_counter = 0;

    for event in events {
        if seq_counter > 0 && eval_disj(&sequence[0], &mapping, &event) {
            seq_counter = 1;
            continue;
        }

        if eval_disj(&sequence[seq_counter], &mapping, &event) {
            seq_counter = seq_counter + 1;

            if seq_counter == sequence.len() {
                return true;
            }
        }
    }

    return false;
}

pub fn eval_pattern_rule(
    events: &Vec<Event>,
    mapping: &BTreeMap<String, usize>,
    p_rule: &Option<PatternRule>,
) -> bool {
    let pattern_rule = p_rule.as_ref().unwrap();
    let pattern = &pattern_rule.pattern_sequence;

    return events.windows(pattern.len()).any(|sequence| {
        sequence.iter().zip(pattern.iter()).all(|(event, disj)| {
            return eval_disj(disj, &mapping, event);
        })
    });
}

pub fn flatten_window(windows: &Vec<Vec<Event>>) -> Vec<Vec<Term>> {
    let mut flatted_windows = Vec::new();

    for w in windows {
        let mut window = Vec::new();
        for e in w {
            window.extend(e.data.clone());
        }
        flatted_windows.push(window);
    }
    return flatted_windows;
}

pub fn eval_window_rule(w: &Window, mut events: Vec<Event>, utc_pos: &usize) -> Vec<Vec<Event>> {
    match w {
        Window::CountWindow(count_win) => {
            let mut c_windows = Vec::new();

            while events.len() >= count_win.w_width as usize {
                let rest = events.split_off(count_win.w_width as usize).clone();

                c_windows.push(events.clone());
                events = rest;
            }

            return c_windows;
        }
        Window::TimeWindow(TimeWindow { time_unit, w_width }) => {
            let window_in_ms = get_time_window_ms(time_unit, w_width.clone());
            let mut tstemp_events: Vec<(Event, DateTime<FixedOffset>)> = events
                .into_iter()
                .map(|e: Event| {
                    let dt: DateTime<FixedOffset> = match &e.data[*utc_pos] {
                        Term::Str(s) => DateTime::parse_from_rfc3339(s).unwrap(),
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
                    window.push(e.clone());
                } else {
                    win_vec.push(window.clone());
                    window.clear();
                    window.push(e.clone());
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

pub fn get_time_window_ms(time_unit: &TimeUnit, w_width: Term) -> i64 {
    match w_width {
        Term::Int(i) => match time_unit {
            TimeUnit::MS => i as i64,
            TimeUnit::S => (i * 1000) as i64,
            TimeUnit::MIN => (60 * i * 1000) as i64,
            TimeUnit::H => (60 * 60 * i * 1000) as i64,
            TimeUnit::D => (24 * 60 * 60 * i * 1000) as i64,
        },
        Term::Float(f) => match time_unit {
            TimeUnit::MS => f as i64,
            TimeUnit::S => (f * 1000.0) as i64,
            TimeUnit::MIN => (60_f64 * f * 1000.0) as i64,
            TimeUnit::H => (60.0 * 60.0 * f * 1000.0) as i64,
            TimeUnit::D => (24.0 * 60.0 * 60.0 * f * 1000.0) as i64,
        },
        _ => unimplemented!("Only Int or Float a valid types to specify the TimeWindow!"),
    }
}

pub fn eval_agg_data(op: &AggOperation, data: &Vec<Term>) -> Option<Term> {
    let result = match op {
        AggOperation::AVG => avg(&data),
        AggOperation::COUNT => count(&data),
        AggOperation::SUM => sum(&data),
        AggOperation::MEDIAN => median(&data),
        AggOperation::MIN => min(&data),
        AggOperation::MAX => max(&data),
        AggOperation::STDDEV => stddev(&data),

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

pub fn mapping_from_schema(s: &Schema) -> BTreeMap<String, usize> {
    let mut mapping: BTreeMap<String, usize> = BTreeMap::new();
    let mut counter = 0;

    for attr in s.attribute_list.list.iter() {
        mapping.insert(attr.ident.clone(), counter);
        counter = counter + 1;
    }
    return mapping;
}

pub fn eval_filter_dnf(
    dis: &Vec<Disjunction>,
    mapping: &BTreeMap<String, usize>,
    event: &Event,
) -> bool {
    let mut result = false;

    for d in dis {
        result = result || eval_disj(&d, mapping, event);
    }

    return result;
}

fn eval_disj(dis: &Disjunction, mapping: &BTreeMap<String, usize>, event: &Event) -> bool {
    let mut result = false;
    for c in &dis.conj {
        result = result || eval_conj(&c, &mapping, &event);
    }
    return result;
}

fn eval_conj(conj: &Conjunction, mapping: &BTreeMap<String, usize>, event: &Event) -> bool {
    let mut result = true;

    for pred in &conj.preds {
        result = result && eval_pred(&pred, &mapping, &event);
    }
    return result;
}

fn resolve_term<'k>(
    t: &'k Term,
    mapping: &'k BTreeMap<String, usize>,
    event: &'k Event,
) -> Option<&'k Term> {
    match t {
        Term::Ident(name) => {
            let index = *mapping.get(name)?;
            Some(&event.data[index])
        }
        Term::Aggregate(a) => {
            let index = *mapping.get(&a.ident)?;
            Some(&event.data[index])
        }
        _ => Some(t),
    }
}

pub fn eval_pred(p: &Pred, env: &BTreeMap<String, usize>, event: &Event) -> bool {
    let lhs = match resolve_term(&p.lhs, env, event) {
        Some(v) => v,
        None => return false, // Variable nicht gesetzt
    };
    let rhs = match resolve_term(&p.rhs, env, event) {
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
pub fn operator_to_function<T: PartialEq + PartialOrd>(op: Operator) -> fn(a: &T, b: &T) -> bool {
    match op {
        Operator::Eq => |a, b| a == b,
        Operator::NEq => |a, b| a != b,
        Operator::Gr => |a, b| a > b,
        Operator::GrEq => |a, b| a >= b,
        Operator::Sm => |a, b| a < b,
        Operator::SmEq => |a, b| a <= b,
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
