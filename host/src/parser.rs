#![allow(clippy::upper_case_acronyms, clippy::result_large_error)]

use std::sync::mpsc::RecvTimeoutError;

use dnf_core::ast::{Conjunction, Disjunction, Operator, Pred, Term};
use dnf_core::epl::{AssertRule, Attribute, AttributeList, Itype, ProgramAst, Schema};
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "epl-dsl.pest"]
#[grammar = "common.pest"]
#[grammar = "dnf.pest"]
pub struct DnfParser;

pub fn parse_source(source: &str) -> Result<ProgramAst, pest::error::Error<Rule>> {
    let mut pairs = DnfParser::parse(Rule::program, source)?;
    //let ast = build_dnf(pairs);
    // Ok(ast)

    let program_pair = pairs.next().unwrap();
    Ok(build_program(program_pair))
}

fn build_program(program: pest::iterators::Pair<Rule>) -> ProgramAst {
    let mut schemas = Vec::new();
    let mut assert_rules = Vec::new();

    for p in program.into_inner() {
        match p.as_rule() {
            Rule::schema => schemas.push(create_schema(p)),
            Rule::assert_rule => assert_rules.push(create_assert_rule(p)),
            Rule::EOI => {}
            _ => {}
        }
    }

    return ProgramAst {
        schemas,
        assert_rules,
    };
}

fn create_schema(pair: pest::iterators::Pair<Rule>) -> Schema {
    match pair.as_rule() {
        Rule::schema => {
            let mut it = pair.into_inner();
            let ident = build_ident(it.next().unwrap());
            let attr_list = build_attribute_list(it.next().unwrap());

            return Schema {
                ident,
                attribute_list: attr_list,
            };
        }
        _ => unreachable!("Only the Schema Rule is expected here"),
    }
}

fn create_assert_rule(pair: pest::iterators::Pair<Rule>) -> AssertRule {
    match pair.as_rule() {
        Rule::assert_rule => {
            let mut it = pair.into_inner();
            let ident = build_ident(it.next().unwrap());
            let dnf_pair = it.next().unwrap();

            let dnf: Vec<Disjunction> = match dnf_pair.as_rule() {
                Rule::Disjunction => vec![build_disjunction(dnf_pair)],
                Rule::dnf_statement => build_dnf(dnf_pair.into_inner()),
                other => unreachable!("Not expected other execution path!"),
            };

            return AssertRule { ident, rule: dnf };
        }
        _ => unreachable!("Rule::assert_rule is expected here!"),
    }
}

fn build_attribute_list(pair: pest::iterators::Pair<Rule>) -> AttributeList {
    match pair.as_rule() {
        Rule::attribute_list => {
            //let mut it = pair.into_inner();

            let mut list: Vec<Attribute> = Vec::new();

            for inner in pair.into_inner() {
                list.push(build_attribute(inner));
            }
            return AttributeList { list };
        }
        _ => unreachable!("Should be an attribute list rule!"),
    }
}

fn build_attribute(pair: pest::iterators::Pair<Rule>) -> Attribute {
    match pair.as_rule() {
        Rule::attribute => {
            let mut it = pair.into_inner();
            let ident = build_ident(it.next().unwrap());
            let t: Itype = build_type(it.next().unwrap());

            return Attribute {
                i_type: t,
                ident: ident,
            };
        }

        _ => unreachable!("unexpected Term: {:?}", pair.as_rule()),
    }
}

fn build_ident(pair: pest::iterators::Pair<Rule>) -> Term {
    match pair.as_rule() {
        Rule::Ident => Term::Ident(pair.as_str().to_string()),
        _ => unreachable!("Must be Identfier!"),
    }
}

fn build_type(pair: pest::iterators::Pair<Rule>) -> Itype {
    match pair.as_str() {
        "string" => Itype::String,
        "int" => Itype::Int,
        "bool" => Itype::Bool,
        "float" => Itype::Float,
        _ => unreachable!(
            "Per Definition limited number of supported types (int, string, bool, float)"
        ),
    }
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

fn build_disjunction(pair: pest::iterators::Pair<Rule>) -> Disjunction {
    let clauses = pair
        .into_inner()
        .filter(|p| p.as_rule() == Rule::Conjunction)
        .map(build_clause)
        .collect();

    Disjunction { conj: clauses }
}

fn build_dnf(pairs: pest::iterators::Pairs<Rule>) -> Vec<Disjunction> {
    let mut ast: Vec<Disjunction> = Vec::new();

    for p in pairs {
        if let Rule::Disjunction = p.as_rule() {
            ast.push(build_disjunction(p));
        }
    }

    return ast;
}
