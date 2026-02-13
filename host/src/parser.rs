#![allow(clippy::upper_case_acronyms, clippy::result_large_error)]

use dnf_core::ast::{Conjunction, Disjunction, Operator, Pred, Term};
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "dnf.pest"]
pub struct DnfParser;

pub fn parse_source(source: &str) -> Result<Vec<Disjunction>, pest::error::Error<Rule>> {
    let pairs = DnfParser::parse(Rule::Program, source)?;
    let mut ast: Vec<Disjunction> = Vec::new();

    for pair in pairs {
        if let Rule::Disjunction = pair.as_rule() {
            ast.push(build_dnf(pair));
        }
    }

    Ok(ast)
}

fn build_term(pair: pest::iterators::Pair<Rule>) -> Term {
    match pair.as_rule() {
        Rule::Number => Term::Number(pair.as_str().parse().unwrap()),
        Rule::Bool => Term::Bool(pair.as_str() == "true"),

        // Assuming your grammar returns quoted strings like "hi"
        Rule::Str => {
            let s = pair.as_str();
            // remove surrounding quotes
            let inner = &s[1..s.len().saturating_sub(1)];
            Term::Str(inner.to_string())
        }

        Rule::Ident => Term::Ident(pair.as_str().to_string()),

        _ => unreachable!("unexpected Term: {:?}", pair.as_rule()),
    }
}

fn build_operator(pair: pest::iterators::Pair<Rule>) -> Operator {
    match pair.as_str() {
        "==" => Operator::Eq,
        "!=" => Operator::NEq,
        ">" => Operator::Gr,
        "<" => Operator::Sm,
        "<=" => Operator::SmEq,
        ">=" => Operator::GrEq,
        _ => unreachable!("Unknown operator: {:?}", pair.as_str()),
    }
}

fn build_predicate(pair: pest::iterators::Pair<Rule>) -> Pred {
    let mut it = pair.into_inner();
    let lhs = build_term(it.next().unwrap());
    let op = build_operator(it.next().unwrap());
    let rhs = build_term(it.next().unwrap());

    Pred { lhs, op, rhs }
}

fn build_clause(pair: pest::iterators::Pair<Rule>) -> Conjunction {
    let preds = pair
        .into_inner()
        .filter(|p| p.as_rule() == Rule::Pred)
        .map(build_predicate)
        .collect();

    Conjunction { preds }
}

fn build_dnf(pair: pest::iterators::Pair<Rule>) -> Disjunction {
    let clauses = pair
        .into_inner()
        .filter(|p| p.as_rule() == Rule::Conjunction)
        .map(build_clause)
        .collect();

    Disjunction { conj: clauses }
}
