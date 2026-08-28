# Data Utility Attestation Implementation

<!-- TODO: one-line thesis title / author / university -->

This repository's evaluation work lives on two branches, not `main`:

- **[`end-to-end-system`](../../tree/end-to-end-system)** — unit tests and
  an end-to-end demo of the data utility attestation protocol.
- **[`dsl-performance-benchmarks`](../../tree/dsl-performance-benchmarks)** —
  performance benchmarks for the EPL DSL query evaluation.


Quick link for Data Utility Predicate Language Implementation: 
- **[DSL implementation](#dsl-implementation)** — parser, AST and
  interpreter breakdown for the language, further down this page.

Check out the branch you want; each has its own `README.md` with full
build/run instructions (Docker included).

## DSL implementation

The custom DSL referenced above — internally called **EPL** — is implemented
on the [`end-to-end-system`](../../tree/end-to-end-system) branch, under
[`crates/system_core`](../../tree/end-to-end-system/crates/system_core) (parsing)
and [`crates/dnf_core`](../../tree/end-to-end-system/crates/dnf_core) (AST +
interpreter):

| Component | Path | Description |
| --- | --- | --- |
| Grammar | [`epl-dsl.pest`](../../blob/end-to-end-system/crates/system_core/src/epl-dsl.pest), [`dnf.pest`](../../blob/end-to-end-system/crates/system_core/src/dnf.pest), [`common.pest`](../../blob/end-to-end-system/crates/system_core/src/common.pest) | [PEST](https://pest.rs) grammar rules defining the DSL surface syntax. |
| Parser | [`parser.rs`](../../blob/end-to-end-system/crates/system_core/src/parser.rs) (`parse_source`) | Combines the PEST lexical parsing of the grammar above with the AST transformation into a `ProgramAst`. |
| Data model — program/rules | [`epl.rs`](../../blob/end-to-end-system/crates/dnf_core/src/epl.rs) | The `ProgramAst` structure plus schema, assertion/pattern rule, window and session types. |
| Data model — terms/predicates | [`ast.rs`](../../blob/end-to-end-system/crates/dnf_core/src/ast.rs) | Core term/predicate types (`Operator`, `Term`, `Pred`) and the DNF (disjunctive normal form) `Conjunction`/`Disjunction` structures matched by `dnf.pest`. |
| Interpreter | [`interpreter.rs`](../../blob/end-to-end-system/crates/dnf_core/src/interpreter.rs) (`eval_program`) | Evaluates a parsed `ProgramAst` (the canonical utility predicate) against an event batch `Vec<Event>`. |

`eval_program(p: &ProgramAst, input: &Vec<Event>) -> bool` is the interpreter's
entry point: it takes the AST produced by `parse_source` and a batch of
`Event`s, and returns whether the batch satisfies the encoded utility
predicate. The diagram below shows how a query flows from the language
frontend (grammar → PEG parser → AST transformer) into the interpreter's
evaluation stages:

![EPL language architecture](crates/dnf_core/language_architecture.svg)

## Acknowledgments
Built on the [RISC Zero](https://github.com/risc0/risc0) zkVM and its
project template (Apache License 2.0).

> **Note:** the project was supported partly through AI tools. More
> information can be requested from the author.
