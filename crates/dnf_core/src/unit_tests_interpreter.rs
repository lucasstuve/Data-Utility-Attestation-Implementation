#[cfg(test)]
mod interpreter_tests {
    extern crate alloc;
    use core::time;

    use crate::ast::*;
    use crate::epl::{
        AggOperation, Aggregate, AssertRule, Attribute, AttributeList, CountWindow, Itype,
        PatternRule, ProgramAst, Quantifier, Schema, TimeUnit, TimeWindow, Window,
    };
    use crate::interpreter::*;
    use alloc::{string::String, vec::Vec};

    //TODO Test eval_program

    // Helper functions for data preparations:

    fn attribute_list_builder(attr: Vec<(String, Itype)>) -> AttributeList {
        let mut attributes = Vec::new();
        for a in attr {
            attributes.push(Attribute {
                ident: a.0,
                i_type: a.1,
            });
        }
        return AttributeList { list: attributes };
    }

    fn schema_builder(s: String, a: AttributeList) -> Schema {
        return Schema {
            ident: s,
            attribute_list: a,
        };
    }

    fn disj_builder(conjs: Vec<Conjunction>) -> Disjunction {
        return Disjunction { conj: conjs };
    }

    fn conj_builder(preds: Vec<Pred>) -> Conjunction {
        return Conjunction { preds };
    }

    fn pred_builder(lhs: Term, op: Operator, rhs: Term) -> Pred {
        return Pred { lhs, op, rhs };
    }

    fn pattern_rule_builder(disj: Vec<Disjunction>, ident: String) -> PatternRule {
        return PatternRule {
            pattern_sequence: disj,
            identifier: ident,
        };
    }

    fn assert_rule_builder(q: Quantifier, ident: String, rules: Vec<Disjunction>) -> AssertRule {
        return AssertRule {
            ident,
            rule: rules,
            quantifier: q,
        };
    }
    fn time_window_builder(t: Term, u: TimeUnit) -> Window {
        return Window::TimeWindow(TimeWindow {
            w_width: t,
            time_unit: u,
        });
    }
    fn count_window_builder(t: u64) -> Window {
        return Window::CountWindow(CountWindow { w_width: t });
    }

    #[test]
    fn test_assert_smeq_01() {
        let attribute_list =
            alloc::vec![("id".into(), Itype::String), ("speed".into(), Itype::Int)];
        let schema = schema_builder("Event".into(), attribute_list_builder(attribute_list));
        let pred = pred_builder(Term::Ident("speed".into()), Operator::SmEq, Term::Int(60));
        let conj = conj_builder(alloc::vec![pred]);
        let disj = disj_builder(alloc::vec![conj]);
        let assert_rule = assert_rule_builder(Quantifier::ALL, "speed".into(), alloc::vec![disj]);
        let ast = ProgramAst {
            schemas: alloc::vec![schema],
            assert_rules: alloc::vec![assert_rule],
            pattern_rule: None,
            aggregates: alloc::vec![],
            window_rule: None,
            session: None,
        };

        let input_events = alloc::vec![
            Event {
                data: alloc::vec![Term::Str("id_001".into()), Term::Int(30)]
            },
            Event {
                data: alloc::vec![Term::Str("id_002".into()), Term::Int(50)]
            },
            Event {
                data: alloc::vec![Term::Str("id_002".into()), Term::Int(60)]
            },
            Event {
                data: alloc::vec![Term::Str("id_003".into()), Term::Int(20)]
            },
        ];

        assert_eq!(eval_program(ast, input_events), true);
    }

    #[test]
    fn test_assert_smeq_02() {
        let attribute_list =
            alloc::vec![("id".into(), Itype::String), ("speed".into(), Itype::Int)];
        let schema = schema_builder("Event".into(), attribute_list_builder(attribute_list));
        let pred = pred_builder(Term::Ident("speed".into()), Operator::SmEq, Term::Int(60));
        let conj = conj_builder(alloc::vec![pred]);
        let disj = disj_builder(alloc::vec![conj]);
        let assert_rule = assert_rule_builder(Quantifier::ALL, "speed".into(), alloc::vec![disj]);
        let ast = ProgramAst {
            schemas: alloc::vec![schema],
            assert_rules: alloc::vec![assert_rule],
            pattern_rule: None,
            aggregates: alloc::vec![],
            window_rule: None,
            session: None,
        };

        let input_events = alloc::vec![
            Event {
                data: alloc::vec![Term::Str("id_001".into()), Term::Int(30)]
            },
            Event {
                data: alloc::vec![Term::Str("id_002".into()), Term::Int(70)]
            },
            Event {
                data: alloc::vec![Term::Str("id_002".into()), Term::Int(60)]
            },
            Event {
                data: alloc::vec![Term::Str("id_003".into()), Term::Int(20)]
            },
        ];

        assert_eq!(eval_program(ast, input_events), false);
    }
    #[test]
    fn test_assert_greq_01() {
        let attribute_list =
            alloc::vec![("id".into(), Itype::String), ("speed".into(), Itype::Int)];
        let schema = schema_builder("Event".into(), attribute_list_builder(attribute_list));
        let pred = pred_builder(Term::Ident("speed".into()), Operator::GrEq, Term::Int(10));
        let conj = conj_builder(alloc::vec![pred]);
        let disj = disj_builder(alloc::vec![conj]);
        let assert_rule = assert_rule_builder(Quantifier::ALL, "speed".into(), alloc::vec![disj]);
        let ast = ProgramAst {
            schemas: alloc::vec![schema],
            assert_rules: alloc::vec![assert_rule],
            pattern_rule: None,
            aggregates: alloc::vec![],
            window_rule: None,
            session: None,
        };

        let input_events = alloc::vec![
            Event {
                data: alloc::vec![Term::Str("id_001".into()), Term::Int(30)]
            },
            Event {
                data: alloc::vec![Term::Str("id_002".into()), Term::Int(70)]
            },
            Event {
                data: alloc::vec![Term::Str("id_002".into()), Term::Int(60)]
            },
            Event {
                data: alloc::vec![Term::Str("id_003".into()), Term::Int(20)]
            },
        ];

        assert_eq!(eval_program(ast, input_events), true);
    }

    #[test]
    fn test_assert_greq_02() {
        let attribute_list =
            alloc::vec![("id".into(), Itype::String), ("speed".into(), Itype::Int)];
        let schema = schema_builder("Event".into(), attribute_list_builder(attribute_list));
        let pred = pred_builder(Term::Ident("speed".into()), Operator::GrEq, Term::Int(100));
        let conj = conj_builder(alloc::vec![pred]);
        let disj = disj_builder(alloc::vec![conj]);
        let assert_rule = assert_rule_builder(Quantifier::ALL, "speed".into(), alloc::vec![disj]);
        let ast = ProgramAst {
            schemas: alloc::vec![schema],
            assert_rules: alloc::vec![assert_rule],
            pattern_rule: None,
            aggregates: alloc::vec![],
            window_rule: None,
            session: None,
        };

        let input_events = alloc::vec![
            Event {
                data: alloc::vec![Term::Str("id_001".into()), Term::Int(30)]
            },
            Event {
                data: alloc::vec![Term::Str("id_002".into()), Term::Int(70)]
            },
            Event {
                data: alloc::vec![Term::Str("id_002".into()), Term::Int(60)]
            },
            Event {
                data: alloc::vec![Term::Str("id_003".into()), Term::Int(20)]
            },
        ];

        assert_eq!(eval_program(ast, input_events), false);
    }

    #[test]
    fn test_assert_all_Eq_01() {
        let attribute_list =
            alloc::vec![("id".into(), Itype::String), ("speed".into(), Itype::Int)];
        let schema = schema_builder("Event".into(), attribute_list_builder(attribute_list));
        let pred = pred_builder(Term::Ident("speed".into()), Operator::Eq, Term::Int(10));
        let conj = conj_builder(alloc::vec![pred]);
        let disj = disj_builder(alloc::vec![conj]);
        let assert_rule = assert_rule_builder(Quantifier::ALL, "speed".into(), alloc::vec![disj]);
        let ast = ProgramAst {
            schemas: alloc::vec![schema],
            assert_rules: alloc::vec![assert_rule],
            pattern_rule: None,
            aggregates: alloc::vec![],
            window_rule: None,
            session: None,
        };

        let input_events = alloc::vec![
            Event {
                data: alloc::vec![Term::Str("id_001".into()), Term::Int(30)]
            },
            Event {
                data: alloc::vec![Term::Str("id_002".into()), Term::Int(70)]
            },
            Event {
                data: alloc::vec![Term::Str("id_002".into()), Term::Int(60)]
            },
            Event {
                data: alloc::vec![Term::Str("id_003".into()), Term::Int(20)]
            },
        ];

        assert_eq!(eval_program(ast, input_events), false);
    }

    #[test]
    fn test_assert_all_Eq_02() {
        let attribute_list =
            alloc::vec![("id".into(), Itype::String), ("speed".into(), Itype::Int)];
        let schema = schema_builder("Event".into(), attribute_list_builder(attribute_list));
        let pred = pred_builder(Term::Ident("speed".into()), Operator::Eq, Term::Int(10));
        let conj = conj_builder(alloc::vec![pred]);
        let disj = disj_builder(alloc::vec![conj]);
        let assert_rule = assert_rule_builder(Quantifier::ALL, "speed".into(), alloc::vec![disj]);
        let ast = ProgramAst {
            schemas: alloc::vec![schema],
            assert_rules: alloc::vec![assert_rule],
            pattern_rule: None,
            aggregates: alloc::vec![],
            window_rule: None,
            session: None,
        };

        let input_events = alloc::vec![
            Event {
                data: alloc::vec![Term::Str("id_001".into()), Term::Int(10)]
            },
            Event {
                data: alloc::vec![Term::Str("id_002".into()), Term::Int(10)]
            },
            Event {
                data: alloc::vec![Term::Str("id_002".into()), Term::Int(10)]
            },
            Event {
                data: alloc::vec![Term::Str("id_003".into()), Term::Int(10)]
            },
        ];

        assert_eq!(eval_program(ast, input_events), true);
    }

    #[test]
    fn test_assert_all_Sm_01() {
        let attribute_list =
            alloc::vec![("id".into(), Itype::String), ("speed".into(), Itype::Int)];
        let schema = schema_builder("Event".into(), attribute_list_builder(attribute_list));
        let pred = pred_builder(Term::Ident("speed".into()), Operator::Sm, Term::Int(100));
        let conj = conj_builder(alloc::vec![pred]);
        let disj = disj_builder(alloc::vec![conj]);
        let assert_rule = assert_rule_builder(Quantifier::ALL, "speed".into(), alloc::vec![disj]);
        let ast = ProgramAst {
            schemas: alloc::vec![schema],
            assert_rules: alloc::vec![assert_rule],
            pattern_rule: None,
            aggregates: alloc::vec![],
            window_rule: None,
            session: None,
        };

        let input_events = alloc::vec![
            Event {
                data: alloc::vec![Term::Str("id_001".into()), Term::Int(30)]
            },
            Event {
                data: alloc::vec![Term::Str("id_002".into()), Term::Int(70)]
            },
            Event {
                data: alloc::vec![Term::Str("id_002".into()), Term::Int(60)]
            },
            Event {
                data: alloc::vec![Term::Str("id_003".into()), Term::Int(20)]
            },
        ];

        assert_eq!(eval_program(ast, input_events), true);
    }

    #[test]
    fn test_assert_all_Sm_02() {
        let attribute_list =
            alloc::vec![("id".into(), Itype::String), ("speed".into(), Itype::Int)];
        let schema = schema_builder("Event".into(), attribute_list_builder(attribute_list));
        let pred = pred_builder(Term::Ident("speed".into()), Operator::Sm, Term::Int(50));
        let conj = conj_builder(alloc::vec![pred]);
        let disj = disj_builder(alloc::vec![conj]);
        let assert_rule = assert_rule_builder(Quantifier::ALL, "speed".into(), alloc::vec![disj]);
        let ast = ProgramAst {
            schemas: alloc::vec![schema],
            assert_rules: alloc::vec![assert_rule],
            pattern_rule: None,
            aggregates: alloc::vec![],
            window_rule: None,
            session: None,
        };

        let input_events = alloc::vec![
            Event {
                data: alloc::vec![Term::Str("id_001".into()), Term::Int(30)]
            },
            Event {
                data: alloc::vec![Term::Str("id_002".into()), Term::Int(70)]
            },
            Event {
                data: alloc::vec![Term::Str("id_002".into()), Term::Int(60)]
            },
            Event {
                data: alloc::vec![Term::Str("id_003".into()), Term::Int(20)]
            },
        ];

        assert_eq!(eval_program(ast, input_events), false);
    }

    #[test]
    fn test_assert_all_nEq_01() {
        let attribute_list =
            alloc::vec![("id".into(), Itype::String), ("speed".into(), Itype::Int)];
        let schema = schema_builder("Event".into(), attribute_list_builder(attribute_list));
        let pred = pred_builder(Term::Ident("speed".into()), Operator::NEq, Term::Int(50));
        let conj = conj_builder(alloc::vec![pred]);
        let disj = disj_builder(alloc::vec![conj]);
        let assert_rule = assert_rule_builder(Quantifier::ALL, "speed".into(), alloc::vec![disj]);
        let ast = ProgramAst {
            schemas: alloc::vec![schema],
            assert_rules: alloc::vec![assert_rule],
            pattern_rule: None,
            aggregates: alloc::vec![],
            window_rule: None,
            session: None,
        };

        let input_events = alloc::vec![
            Event {
                data: alloc::vec![Term::Str("id_001".into()), Term::Int(30)]
            },
            Event {
                data: alloc::vec![Term::Str("id_002".into()), Term::Int(70)]
            },
            Event {
                data: alloc::vec![Term::Str("id_002".into()), Term::Int(60)]
            },
            Event {
                data: alloc::vec![Term::Str("id_003".into()), Term::Int(20)]
            },
        ];

        assert_eq!(eval_program(ast, input_events), true);
    }

    #[test]
    fn test_assert_all_nEq_02() {
        let attribute_list =
            alloc::vec![("id".into(), Itype::String), ("speed".into(), Itype::Int)];
        let schema = schema_builder("Event".into(), attribute_list_builder(attribute_list));
        let pred = pred_builder(Term::Ident("speed".into()), Operator::NEq, Term::Int(50));
        let conj = conj_builder(alloc::vec![pred]);
        let disj = disj_builder(alloc::vec![conj]);
        let assert_rule = assert_rule_builder(Quantifier::ALL, "speed".into(), alloc::vec![disj]);
        let ast = ProgramAst {
            schemas: alloc::vec![schema],
            assert_rules: alloc::vec![assert_rule],
            pattern_rule: None,
            aggregates: alloc::vec![],
            window_rule: None,
            session: None,
        };

        let input_events = alloc::vec![
            Event {
                data: alloc::vec![Term::Str("id_001".into()), Term::Int(30)]
            },
            Event {
                data: alloc::vec![Term::Str("id_002".into()), Term::Int(50)]
            },
            Event {
                data: alloc::vec![Term::Str("id_002".into()), Term::Int(60)]
            },
            Event {
                data: alloc::vec![Term::Str("id_003".into()), Term::Int(20)]
            },
        ];

        assert_eq!(eval_program(ast, input_events), false);
    }

    // Some test cases with the ANY-Quantifier and assert (filter rule)

    #[test]
    fn test_assert_any_nEq_01() {
        let attribute_list =
            alloc::vec![("id".into(), Itype::String), ("speed".into(), Itype::Int)];
        let schema = schema_builder("Event".into(), attribute_list_builder(attribute_list));
        let pred = pred_builder(Term::Ident("speed".into()), Operator::NEq, Term::Int(50));
        let conj = conj_builder(alloc::vec![pred]);
        let disj = disj_builder(alloc::vec![conj]);
        let assert_rule = assert_rule_builder(Quantifier::ANY, "speed".into(), alloc::vec![disj]);
        let ast = ProgramAst {
            schemas: alloc::vec![schema],
            assert_rules: alloc::vec![assert_rule],
            pattern_rule: None,
            aggregates: alloc::vec![],
            window_rule: None,
            session: None,
        };

        let input_events = alloc::vec![
            Event {
                data: alloc::vec![Term::Str("id_001".into()), Term::Int(30)]
            },
            Event {
                data: alloc::vec![Term::Str("id_002".into()), Term::Int(50)]
            },
            Event {
                data: alloc::vec![Term::Str("id_002".into()), Term::Int(60)]
            },
            Event {
                data: alloc::vec![Term::Str("id_003".into()), Term::Int(20)]
            },
        ];

        assert_eq!(eval_program(ast, input_events), true);
    }

    #[test]
    fn test_assert_any_nEq_02() {
        let attribute_list =
            alloc::vec![("id".into(), Itype::String), ("speed".into(), Itype::Int)];
        let schema = schema_builder("Event".into(), attribute_list_builder(attribute_list));
        let pred = pred_builder(Term::Ident("speed".into()), Operator::NEq, Term::Int(50));
        let conj = conj_builder(alloc::vec![pred]);
        let disj = disj_builder(alloc::vec![conj]);
        let assert_rule = assert_rule_builder(Quantifier::ANY, "speed".into(), alloc::vec![disj]);
        let ast = ProgramAst {
            schemas: alloc::vec![schema],
            assert_rules: alloc::vec![assert_rule],
            pattern_rule: None,
            aggregates: alloc::vec![],
            window_rule: None,
            session: None,
        };

        let input_events = alloc::vec![
            Event {
                data: alloc::vec![Term::Str("id_001".into()), Term::Int(50)]
            },
            Event {
                data: alloc::vec![Term::Str("id_002".into()), Term::Int(50)]
            },
            Event {
                data: alloc::vec![Term::Str("id_002".into()), Term::Int(50)]
            },
            Event {
                data: alloc::vec![Term::Str("id_003".into()), Term::Int(50)]
            },
        ];

        assert_eq!(eval_program(ast, input_events), false);
    }

    #[test]
    fn test_assert_any_Gr_01() {
        let attribute_list =
            alloc::vec![("id".into(), Itype::String), ("speed".into(), Itype::Int)];
        let schema = schema_builder("Event".into(), attribute_list_builder(attribute_list));
        let pred = pred_builder(Term::Ident("speed".into()), Operator::Gr, Term::Int(50));
        let conj = conj_builder(alloc::vec![pred]);
        let disj = disj_builder(alloc::vec![conj]);
        let assert_rule = assert_rule_builder(Quantifier::ANY, "speed".into(), alloc::vec![disj]);
        let ast = ProgramAst {
            schemas: alloc::vec![schema],
            assert_rules: alloc::vec![assert_rule],
            pattern_rule: None,
            aggregates: alloc::vec![],
            window_rule: None,
            session: None,
        };

        let input_events = alloc::vec![
            Event {
                data: alloc::vec![Term::Str("id_001".into()), Term::Int(30)]
            },
            Event {
                data: alloc::vec![Term::Str("id_002".into()), Term::Int(50)]
            },
            Event {
                data: alloc::vec![Term::Str("id_002".into()), Term::Int(60)]
            },
            Event {
                data: alloc::vec![Term::Str("id_003".into()), Term::Int(20)]
            },
        ];

        assert_eq!(eval_program(ast, input_events), true);
    }

    #[test]
    fn test_assert_any_Gr_02() {
        let attribute_list =
            alloc::vec![("id".into(), Itype::String), ("speed".into(), Itype::Int)];
        let schema = schema_builder("Event".into(), attribute_list_builder(attribute_list));
        let pred = pred_builder(Term::Ident("speed".into()), Operator::Gr, Term::Int(50));
        let conj = conj_builder(alloc::vec![pred]);
        let disj = disj_builder(alloc::vec![conj]);
        let assert_rule = assert_rule_builder(Quantifier::ANY, "speed".into(), alloc::vec![disj]);
        let ast = ProgramAst {
            schemas: alloc::vec![schema],
            assert_rules: alloc::vec![assert_rule],
            pattern_rule: None,
            aggregates: alloc::vec![],
            window_rule: None,
            session: None,
        };

        let input_events = alloc::vec![
            Event {
                data: alloc::vec![Term::Str("id_001".into()), Term::Int(30)]
            },
            Event {
                data: alloc::vec![Term::Str("id_002".into()), Term::Int(50)]
            },
            Event {
                data: alloc::vec![Term::Str("id_002".into()), Term::Int(50)]
            },
            Event {
                data: alloc::vec![Term::Str("id_003".into()), Term::Int(20)]
            },
        ];

        assert_eq!(eval_program(ast, input_events), false);
    }

    #[test]
    fn test_assert_all_Gr_aggregate_COUNT_01() {
        let attribute_list =
            alloc::vec![("id".into(), Itype::String), ("speed".into(), Itype::Int)];

        let aggregation = AssertRule {
            quantifier: Quantifier::ALL,
            ident: "speed".into(),
            rule: alloc::vec![disj_builder(alloc::vec![conj_builder(alloc::vec![
                pred_builder(
                    Term::Aggregate(Aggregate {
                        ident: "speed".into(),
                        operation: AggOperation::COUNT
                    }),
                    Operator::Eq,
                    Term::Int(4)
                )
            ])],)],
        };
        let schema = schema_builder("Event".into(), attribute_list_builder(attribute_list));
        let pred = pred_builder(Term::Ident("speed".into()), Operator::Gr, Term::Int(10));
        let conj = conj_builder(alloc::vec![pred]);
        let disj = disj_builder(alloc::vec![conj]);
        let assert_rule = assert_rule_builder(Quantifier::ALL, "speed".into(), alloc::vec![disj]);
        let ast = ProgramAst {
            schemas: alloc::vec![schema],
            assert_rules: alloc::vec![assert_rule],
            pattern_rule: None,
            aggregates: alloc::vec![aggregation],
            window_rule: None,
            session: None,
        };

        let input_events = alloc::vec![
            Event {
                data: alloc::vec![Term::Str("id_001".into()), Term::Int(30)]
            },
            Event {
                data: alloc::vec![Term::Str("id_002".into()), Term::Int(50)]
            },
            Event {
                data: alloc::vec![Term::Str("id_002".into()), Term::Int(60)]
            },
            Event {
                data: alloc::vec![Term::Str("id_003".into()), Term::Int(64)]
            },
        ];

        assert_eq!(eval_program(ast, input_events), true);
    }

    #[test]
    fn test_assert_all_Gr_aggregate_COUNT_02() {
        let attribute_list =
            alloc::vec![("id".into(), Itype::String), ("speed".into(), Itype::Int)];

        let aggregation = AssertRule {
            quantifier: Quantifier::ALL,
            ident: "speed".into(),
            rule: alloc::vec![disj_builder(alloc::vec![conj_builder(alloc::vec![
                pred_builder(
                    Term::Aggregate(Aggregate {
                        ident: "speed".into(),
                        operation: AggOperation::COUNT
                    }),
                    Operator::Eq,
                    Term::Int(3)
                )
            ])],)],
        };
        let schema = schema_builder("Event".into(), attribute_list_builder(attribute_list));
        let pred = pred_builder(Term::Ident("speed".into()), Operator::Gr, Term::Int(30));
        let conj = conj_builder(alloc::vec![pred]);
        let disj = disj_builder(alloc::vec![conj]);
        let assert_rule = assert_rule_builder(Quantifier::ALL, "speed".into(), alloc::vec![disj]);
        let ast = ProgramAst {
            schemas: alloc::vec![schema],
            assert_rules: alloc::vec![assert_rule],
            pattern_rule: None,
            aggregates: alloc::vec![aggregation],
            window_rule: None,
            session: None,
        };

        let input_events = alloc::vec![
            Event {
                data: alloc::vec![Term::Str("id_001".into()), Term::Int(30)]
            },
            Event {
                data: alloc::vec![Term::Str("id_002".into()), Term::Int(50)]
            },
            Event {
                data: alloc::vec![Term::Str("id_002".into()), Term::Int(60)]
            },
            Event {
                data: alloc::vec![Term::Str("id_003".into()), Term::Int(64)]
            },
        ];

        assert_eq!(eval_program(ast, input_events), true);
    }

    #[test]
    fn test_assert_all_gr_aggregate_sum_01() {
        let attribute_list =
            alloc::vec![("id".into(), Itype::String), ("speed".into(), Itype::Int)];

        let aggregation = AssertRule {
            quantifier: Quantifier::ALL,
            ident: "speed".into(),
            rule: alloc::vec![disj_builder(alloc::vec![conj_builder(alloc::vec![
                pred_builder(
                    Term::Aggregate(Aggregate {
                        ident: "speed".into(),
                        operation: AggOperation::SUM
                    }),
                    Operator::Eq,
                    Term::Int(174)
                )
            ])],)],
        };
        let schema = schema_builder("Event".into(), attribute_list_builder(attribute_list));
        let pred = pred_builder(Term::Ident("speed".into()), Operator::Gr, Term::Int(30));
        let conj = conj_builder(alloc::vec![pred]);
        let disj = disj_builder(alloc::vec![conj]);
        let assert_rule = assert_rule_builder(Quantifier::ALL, "speed".into(), alloc::vec![disj]);
        let ast = ProgramAst {
            schemas: alloc::vec![schema],
            assert_rules: alloc::vec![assert_rule],
            pattern_rule: None,
            aggregates: alloc::vec![aggregation],
            window_rule: None,
            session: None,
        };

        let input_events = alloc::vec![
            Event {
                data: alloc::vec![Term::Str("id_001".into()), Term::Int(30)]
            },
            Event {
                data: alloc::vec![Term::Str("id_002".into()), Term::Int(50)]
            },
            Event {
                data: alloc::vec![Term::Str("id_002".into()), Term::Int(60)]
            },
            Event {
                data: alloc::vec![Term::Str("id_003".into()), Term::Int(64)]
            },
        ];

        assert_eq!(eval_program(ast, input_events), true);
    }

    #[test]
    fn test_assert_all_gr_aggregate_sum_02() {
        let attribute_list =
            alloc::vec![("id".into(), Itype::String), ("speed".into(), Itype::Int)];

        let aggregation = AssertRule {
            quantifier: Quantifier::ALL,
            ident: "speed".into(),
            rule: alloc::vec![disj_builder(alloc::vec![conj_builder(alloc::vec![
                pred_builder(
                    Term::Aggregate(Aggregate {
                        ident: "speed".into(),
                        operation: AggOperation::SUM
                    }),
                    Operator::GrEq,
                    Term::Int(400)
                )
            ])],)],
        };
        let schema = schema_builder("Event".into(), attribute_list_builder(attribute_list));
        let pred = pred_builder(Term::Ident("speed".into()), Operator::Gr, Term::Int(30));
        let conj = conj_builder(alloc::vec![pred]);
        let disj = disj_builder(alloc::vec![conj]);
        let assert_rule = assert_rule_builder(Quantifier::ALL, "speed".into(), alloc::vec![disj]);
        let ast = ProgramAst {
            schemas: alloc::vec![schema],
            assert_rules: alloc::vec![assert_rule],
            pattern_rule: None,
            aggregates: alloc::vec![aggregation],
            window_rule: None,
            session: None,
        };

        let input_events = alloc::vec![
            Event {
                data: alloc::vec![Term::Str("id_001".into()), Term::Int(30)]
            },
            Event {
                data: alloc::vec![Term::Str("id_002".into()), Term::Int(300)]
            },
            Event {
                data: alloc::vec![Term::Str("id_002".into()), Term::Int(60)]
            },
            Event {
                data: alloc::vec![Term::Str("id_003".into()), Term::Int(40)]
            },
        ];

        assert_eq!(eval_program(ast, input_events), true);
    }

    #[test]
    fn test_assert_all_gr_aggregate_avg_01() {
        let attribute_list =
            alloc::vec![("id".into(), Itype::String), ("speed".into(), Itype::Int)];

        let aggregation = AssertRule {
            quantifier: Quantifier::ALL,
            ident: "speed".into(),
            rule: alloc::vec![disj_builder(alloc::vec![conj_builder(alloc::vec![
                pred_builder(
                    Term::Aggregate(Aggregate {
                        ident: "speed".into(),
                        operation: AggOperation::AVG
                    }),
                    Operator::SmEq,
                    Term::Int(134) // should be 133.333... (400/3)
                )
            ])],)],
        };
        let schema = schema_builder("Event".into(), attribute_list_builder(attribute_list));
        let pred = pred_builder(Term::Ident("speed".into()), Operator::Gr, Term::Int(30));
        let conj = conj_builder(alloc::vec![pred]);
        let disj = disj_builder(alloc::vec![conj]);
        let assert_rule = assert_rule_builder(Quantifier::ALL, "speed".into(), alloc::vec![disj]);
        let ast = ProgramAst {
            schemas: alloc::vec![schema],
            assert_rules: alloc::vec![assert_rule],
            pattern_rule: None,
            aggregates: alloc::vec![aggregation],
            window_rule: None,
            session: None,
        };

        let input_events = alloc::vec![
            Event {
                data: alloc::vec![Term::Str("id_001".into()), Term::Int(30)]
            },
            Event {
                data: alloc::vec![Term::Str("id_002".into()), Term::Int(300)]
            },
            Event {
                data: alloc::vec![Term::Str("id_002".into()), Term::Int(60)]
            },
            Event {
                data: alloc::vec![Term::Str("id_003".into()), Term::Int(40)]
            },
        ];

        assert_eq!(eval_program(ast, input_events), true);
    }

    #[test]
    fn test_assert_all_gr_aggregate_avg_02() {
        let attribute_list =
            alloc::vec![("id".into(), Itype::String), ("speed".into(), Itype::Float)];

        let aggregation = AssertRule {
            quantifier: Quantifier::ALL,
            ident: "speed".into(),
            rule: alloc::vec![disj_builder(alloc::vec![conj_builder(alloc::vec![
                pred_builder(
                    Term::Aggregate(Aggregate {
                        ident: "speed".into(),
                        operation: AggOperation::AVG
                    }),
                    Operator::SmEq,
                    Term::Float(133.3334) // should be 133.333... (400/3)
                )
            ])],)],
        };
        let schema = schema_builder("Event".into(), attribute_list_builder(attribute_list));
        let pred = pred_builder(Term::Ident("speed".into()), Operator::Gr, Term::Float(30.0));
        let conj = conj_builder(alloc::vec![pred]);
        let disj = disj_builder(alloc::vec![conj]);
        let assert_rule = assert_rule_builder(Quantifier::ALL, "speed".into(), alloc::vec![disj]);
        let ast = ProgramAst {
            schemas: alloc::vec![schema],
            assert_rules: alloc::vec![assert_rule],
            pattern_rule: None,
            aggregates: alloc::vec![aggregation],
            window_rule: None,
            session: None,
        };

        let input_events = alloc::vec![
            Event {
                data: alloc::vec![Term::Str("id_001".into()), Term::Float(30.0)]
            },
            Event {
                data: alloc::vec![Term::Str("id_002".into()), Term::Float(300.0)]
            },
            Event {
                data: alloc::vec![Term::Str("id_002".into()), Term::Float(60.0)]
            },
            Event {
                data: alloc::vec![Term::Str("id_003".into()), Term::Float(40.0)]
            },
        ];

        assert_eq!(eval_program(ast, input_events), true);
    }

    #[test]
    fn test_assert_all_gr_aggregate_avg_03() {
        let attribute_list =
            alloc::vec![("id".into(), Itype::String), ("speed".into(), Itype::Float)];

        let aggregation = AssertRule {
            quantifier: Quantifier::ALL,
            ident: "speed".into(),
            rule: alloc::vec![disj_builder(alloc::vec![conj_builder(alloc::vec![
                pred_builder(
                    Term::Aggregate(Aggregate {
                        ident: "speed".into(),
                        operation: AggOperation::AVG
                    }),
                    Operator::Eq,
                    Term::Float(133.33333333) // should be 133.333... (400/3)
                )
            ])],)],
        };
        let schema = schema_builder("Event".into(), attribute_list_builder(attribute_list));
        let pred = pred_builder(Term::Ident("speed".into()), Operator::Gr, Term::Float(30.0));
        let conj = conj_builder(alloc::vec![pred]);
        let disj = disj_builder(alloc::vec![conj]);
        let assert_rule = assert_rule_builder(Quantifier::ALL, "speed".into(), alloc::vec![disj]);
        let ast = ProgramAst {
            schemas: alloc::vec![schema],
            assert_rules: alloc::vec![assert_rule],
            pattern_rule: None,
            aggregates: alloc::vec![aggregation],
            window_rule: None,
            session: None,
        };

        let input_events = alloc::vec![
            Event {
                data: alloc::vec![Term::Str("id_001".into()), Term::Float(30.0)]
            },
            Event {
                data: alloc::vec![Term::Str("id_002".into()), Term::Float(300.0)]
            },
            Event {
                data: alloc::vec![Term::Str("id_002".into()), Term::Float(60.0)]
            },
            Event {
                data: alloc::vec![Term::Str("id_003".into()), Term::Float(40.0)]
            },
        ];

        assert_eq!(eval_program(ast, input_events), false);
    }

    #[test]
    fn test_assert_all_gr_aggregate_median_01() {
        let attribute_list =
            alloc::vec![("id".into(), Itype::String), ("speed".into(), Itype::Float)];

        let aggregation = AssertRule {
            quantifier: Quantifier::ALL,
            ident: "speed".into(),
            rule: alloc::vec![disj_builder(alloc::vec![conj_builder(alloc::vec![
                pred_builder(
                    Term::Aggregate(Aggregate {
                        ident: "speed".into(),
                        operation: AggOperation::MEDIAN
                    }),
                    Operator::Eq,
                    Term::Float(60.0)
                )
            ])],)],
        };
        let schema = schema_builder("Event".into(), attribute_list_builder(attribute_list));
        let pred = pred_builder(Term::Ident("speed".into()), Operator::Gr, Term::Float(20.0));
        let conj = conj_builder(alloc::vec![pred]);
        let disj = disj_builder(alloc::vec![conj]);
        let assert_rule = assert_rule_builder(Quantifier::ALL, "speed".into(), alloc::vec![disj]);
        let ast = ProgramAst {
            schemas: alloc::vec![schema],
            assert_rules: alloc::vec![assert_rule],
            pattern_rule: None,
            aggregates: alloc::vec![aggregation],
            window_rule: None,
            session: None,
        };

        let input_events = alloc::vec![
            Event {
                data: alloc::vec![Term::Str("id_001".into()), Term::Float(30.0)]
            },
            Event {
                data: alloc::vec![Term::Str("id_002".into()), Term::Float(300.0)]
            },
            Event {
                data: alloc::vec![Term::Str("id_002".into()), Term::Float(60.0)]
            },
            Event {
                data: alloc::vec![Term::Str("id_003".into()), Term::Float(40.0)]
            },
            Event {
                data: alloc::vec![Term::Str("id_003".into()), Term::Float(90.0)]
            },
        ];

        assert_eq!(eval_program(ast, input_events), true);
    }

    #[test]
    fn test_assert_all_gr_aggregate_median_02() {
        let attribute_list =
            alloc::vec![("id".into(), Itype::String), ("speed".into(), Itype::Int)];

        let aggregation = AssertRule {
            quantifier: Quantifier::ALL,
            ident: "speed".into(),
            rule: alloc::vec![disj_builder(alloc::vec![conj_builder(alloc::vec![
                pred_builder(
                    Term::Aggregate(Aggregate {
                        ident: "speed".into(),
                        operation: AggOperation::MEDIAN
                    }),
                    Operator::Eq,
                    Term::Int(60)
                )
            ])],)],
        };
        let schema = schema_builder("Event".into(), attribute_list_builder(attribute_list));
        let pred = pred_builder(Term::Ident("speed".into()), Operator::Gr, Term::Int(20));
        let conj = conj_builder(alloc::vec![pred]);
        let disj = disj_builder(alloc::vec![conj]);
        let assert_rule = assert_rule_builder(Quantifier::ALL, "speed".into(), alloc::vec![disj]);
        let ast = ProgramAst {
            schemas: alloc::vec![schema],
            assert_rules: alloc::vec![assert_rule],
            pattern_rule: None,
            aggregates: alloc::vec![aggregation],
            window_rule: None,
            session: None,
        };

        let input_events = alloc::vec![
            Event {
                data: alloc::vec![Term::Str("id_001".into()), Term::Int(30)]
            },
            Event {
                data: alloc::vec![Term::Str("id_002".into()), Term::Int(300)]
            },
            Event {
                data: alloc::vec![Term::Str("id_002".into()), Term::Int(60)]
            },
            Event {
                data: alloc::vec![Term::Str("id_003".into()), Term::Int(40)]
            },
            Event {
                data: alloc::vec![Term::Str("id_003".into()), Term::Int(90)]
            },
        ];

        assert_eq!(eval_program(ast, input_events), true);
    }

    #[test]
    fn test_assert_all_gr_window_count_01() {
        let window = count_window_builder(2);
        let attribute_list =
            alloc::vec![("id".into(), Itype::String), ("speed".into(), Itype::Int)];

        let schema = schema_builder("Event".into(), attribute_list_builder(attribute_list));
        let pred = pred_builder(Term::Ident("speed".into()), Operator::Gr, Term::Int(20));
        let conj = conj_builder(alloc::vec![pred]);
        let disj = disj_builder(alloc::vec![conj]);
        let assert_rule = assert_rule_builder(Quantifier::ALL, "speed".into(), alloc::vec![disj]);

        let ast = ProgramAst {
            schemas: alloc::vec![schema],
            assert_rules: alloc::vec![assert_rule],
            pattern_rule: None,
            aggregates: alloc::vec![],
            window_rule: Some(window),
            session: None,
        };

        let input_events = alloc::vec![
            Event {
                data: alloc::vec![Term::Str("id_001".into()), Term::Int(30)]
            },
            Event {
                data: alloc::vec![Term::Str("id_002".into()), Term::Int(300)]
            },
            Event {
                data: alloc::vec![Term::Str("id_002".into()), Term::Int(60)]
            },
            Event {
                data: alloc::vec![Term::Str("id_003".into()), Term::Int(40)]
            },
            Event {
                data: alloc::vec![Term::Str("id_003".into()), Term::Int(90)]
            },
        ];

        assert_eq!(eval_program(ast, input_events), true);
    }

    #[test]
    fn test_assert_all_gr_window_count_02() {
        let window = count_window_builder(6);
        let attribute_list =
            alloc::vec![("id".into(), Itype::String), ("speed".into(), Itype::Int)];

        let schema = schema_builder("Event".into(), attribute_list_builder(attribute_list));
        let pred = pred_builder(Term::Ident("speed".into()), Operator::Gr, Term::Int(20));
        let conj = conj_builder(alloc::vec![pred]);
        let disj = disj_builder(alloc::vec![conj]);
        let assert_rule = assert_rule_builder(Quantifier::ALL, "speed".into(), alloc::vec![disj]);

        let ast = ProgramAst {
            schemas: alloc::vec![schema],
            assert_rules: alloc::vec![assert_rule],
            pattern_rule: None,
            aggregates: alloc::vec![],
            window_rule: Some(window),
            session: None,
        };

        let input_events = alloc::vec![
            Event {
                data: alloc::vec![Term::Str("id_001".into()), Term::Int(30)]
            },
            Event {
                data: alloc::vec![Term::Str("id_002".into()), Term::Int(300)]
            },
            Event {
                data: alloc::vec![Term::Str("id_002".into()), Term::Int(60)]
            },
            Event {
                data: alloc::vec![Term::Str("id_003".into()), Term::Int(40)]
            },
            Event {
                data: alloc::vec![Term::Str("id_003".into()), Term::Int(90)]
            },
        ];

        assert_eq!(eval_program(ast, input_events), false);
    }

    #[test]
    fn test_assert_all_gr_window_time_01() {
        let window = time_window_builder(Term::Int(3), TimeUnit::S);
        let attribute_list =
            alloc::vec![("id".into(), Itype::String), ("speed".into(), Itype::Int)];

        let schema = schema_builder("Event".into(), attribute_list_builder(attribute_list));
        let pred = pred_builder(Term::Ident("speed".into()), Operator::Gr, Term::Int(20));
        let conj = conj_builder(alloc::vec![pred]);
        let disj = disj_builder(alloc::vec![conj]);
        let assert_rule = assert_rule_builder(Quantifier::ALL, "speed".into(), alloc::vec![disj]);

        let ast = ProgramAst {
            schemas: alloc::vec![schema],
            assert_rules: alloc::vec![assert_rule],
            pattern_rule: None,
            aggregates: alloc::vec![],
            window_rule: Some(window),
            session: None,
        };

        let input_events = alloc::vec![
            Event {
                data: alloc::vec![
                    Term::Str("id_001".into()),
                    Term::Int(30),
                    Term::Str("2026-03-23T10:00:00+00:00".into())
                ]
            },
            Event {
                data: alloc::vec![
                    Term::Str("id_002".into()),
                    Term::Int(300),
                    Term::Str("2026-03-23T10:00:02+00:00".into())
                ]
            },
            Event {
                data: alloc::vec![
                    Term::Str("id_002".into()),
                    Term::Int(60),
                    Term::Str("2026-03-23T10:00:05+00:00".into())
                ]
            },
            Event {
                data: alloc::vec![
                    Term::Str("id_003".into()),
                    Term::Int(40),
                    Term::Str("2026-03-23T10:00:10+00:00".into())
                ]
            },
            Event {
                data: alloc::vec![
                    Term::Str("id_003".into()),
                    Term::Int(90),
                    Term::Str("2026-03-23T10:00:11+00:00".into())
                ]
            },
        ];

        assert_eq!(eval_program(ast, input_events), true);
    }

    #[test]
    fn test_assert_all_gr_window_time_02() {
        let window = time_window_builder(Term::Int(1), TimeUnit::H);
        let attribute_list =
            alloc::vec![("id".into(), Itype::String), ("speed".into(), Itype::Int)];

        let schema = schema_builder("Event".into(), attribute_list_builder(attribute_list));
        let pred = pred_builder(Term::Ident("speed".into()), Operator::Gr, Term::Int(20));
        let conj = conj_builder(alloc::vec![pred]);
        let disj = disj_builder(alloc::vec![conj]);
        let assert_rule = assert_rule_builder(Quantifier::ALL, "speed".into(), alloc::vec![disj]);

        let ast = ProgramAst {
            schemas: alloc::vec![schema],
            assert_rules: alloc::vec![assert_rule],
            pattern_rule: None,
            aggregates: alloc::vec![],
            window_rule: Some(window),
            session: None,
        };

        let input_events = alloc::vec![
            Event {
                data: alloc::vec![
                    Term::Str("id_001".into()),
                    Term::Int(30),
                    Term::Str("2026-03-23T10:00:00+00:00".into())
                ]
            },
            Event {
                data: alloc::vec![
                    Term::Str("id_002".into()),
                    Term::Int(300),
                    Term::Str("2026-03-23T10:00:02+00:00".into())
                ]
            },
            Event {
                data: alloc::vec![
                    Term::Str("id_003".into()),
                    Term::Int(60),
                    Term::Str("2026-03-23T10:00:05+00:00".into())
                ]
            },
            Event {
                data: alloc::vec![
                    Term::Str("id_004".into()),
                    Term::Int(40),
                    Term::Str("2026-03-23T10:00:10+00:00".into())
                ]
            },
            Event {
                data: alloc::vec![
                    Term::Str("id_005".into()),
                    Term::Int(90),
                    Term::Str("2026-03-23T10:00:11+00:00".into())
                ]
            },
        ];

        assert_eq!(eval_program(ast, input_events), true);
    }

    #[test]
    fn test_assert_all_gr_window_time_aggregate_01() {
        let window = time_window_builder(Term::Int(1), TimeUnit::H);
        let attribute_list =
            alloc::vec![("id".into(), Itype::String), ("speed".into(), Itype::Int)];

        let aggregation = AssertRule {
            quantifier: Quantifier::ALL,
            ident: "speed".into(),
            rule: alloc::vec![disj_builder(alloc::vec![conj_builder(alloc::vec![
                pred_builder(
                    Term::Aggregate(Aggregate {
                        ident: "speed".into(),
                        operation: AggOperation::COUNT
                    }),
                    Operator::Eq,
                    Term::Int(5)
                )
            ])],)],
        };

        let schema = schema_builder("Event".into(), attribute_list_builder(attribute_list));
        let pred = pred_builder(Term::Ident("speed".into()), Operator::Gr, Term::Int(20));
        let conj = conj_builder(alloc::vec![pred]);
        let disj = disj_builder(alloc::vec![conj]);
        let assert_rule = assert_rule_builder(Quantifier::ALL, "speed".into(), alloc::vec![disj]);

        let ast = ProgramAst {
            schemas: alloc::vec![schema],
            assert_rules: alloc::vec![assert_rule],
            pattern_rule: None,
            aggregates: alloc::vec![aggregation],
            window_rule: Some(window),
            session: None,
        };

        let input_events = alloc::vec![
            Event {
                data: alloc::vec![
                    Term::Str("id_001".into()),
                    Term::Int(30),
                    Term::Str("2026-03-23T10:00:00+00:00".into())
                ]
            },
            Event {
                data: alloc::vec![
                    Term::Str("id_002".into()),
                    Term::Int(300),
                    Term::Str("2026-03-23T10:00:02+00:00".into())
                ]
            },
            Event {
                data: alloc::vec![
                    Term::Str("id_003".into()),
                    Term::Int(60),
                    Term::Str("2026-03-23T10:00:05+00:00".into())
                ]
            },
            Event {
                data: alloc::vec![
                    Term::Str("id_004".into()),
                    Term::Int(40),
                    Term::Str("2026-03-23T10:00:10+00:00".into())
                ]
            },
            Event {
                data: alloc::vec![
                    Term::Str("id_005".into()),
                    Term::Int(90),
                    Term::Str("2026-03-23T10:00:11+00:00".into())
                ]
            },
        ];

        assert_eq!(eval_program(ast, input_events), true);
    }

    #[test]
    fn test_assert_all_gr_window_time_aggregate_02() {
        let window = time_window_builder(Term::Int(3), TimeUnit::S);
        let attribute_list =
            alloc::vec![("id".into(), Itype::String), ("speed".into(), Itype::Int)];

        let aggregation = AssertRule {
            quantifier: Quantifier::ALL,
            ident: "speed".into(),
            rule: alloc::vec![disj_builder(alloc::vec![conj_builder(alloc::vec![
                pred_builder(
                    Term::Aggregate(Aggregate {
                        ident: "speed".into(),
                        operation: AggOperation::COUNT
                    }),
                    Operator::GrEq,
                    Term::Int(2)
                )
            ])],)],
        };

        let schema = schema_builder("Event".into(), attribute_list_builder(attribute_list));
        let pred = pred_builder(Term::Ident("speed".into()), Operator::Gr, Term::Int(20));
        let conj = conj_builder(alloc::vec![pred]);
        let disj = disj_builder(alloc::vec![conj]);
        let assert_rule = assert_rule_builder(Quantifier::ALL, "speed".into(), alloc::vec![disj]);

        let ast = ProgramAst {
            schemas: alloc::vec![schema],
            assert_rules: alloc::vec![assert_rule],
            pattern_rule: None,
            aggregates: alloc::vec![aggregation],
            window_rule: Some(window),
            session: None,
        };

        let input_events = alloc::vec![
            Event {
                data: alloc::vec![
                    Term::Str("id_001".into()),
                    Term::Int(30),
                    Term::Str("2026-03-23T10:00:00+00:00".into())
                ]
            },
            Event {
                data: alloc::vec![
                    Term::Str("id_002".into()),
                    Term::Int(300),
                    Term::Str("2026-03-23T10:00:02+00:00".into())
                ]
            },
            Event {
                data: alloc::vec![
                    Term::Str("id_003".into()),
                    Term::Int(60),
                    Term::Str("2026-03-23T10:00:05+00:00".into())
                ]
            },
            Event {
                data: alloc::vec![
                    Term::Str("id_004".into()),
                    Term::Int(40),
                    Term::Str("2026-03-23T10:00:10+00:00".into())
                ]
            },
            Event {
                data: alloc::vec![
                    Term::Str("id_005".into()),
                    Term::Int(90),
                    Term::Str("2026-03-23T10:00:11+00:00".into())
                ]
            },
        ];

        assert_eq!(eval_program(ast, input_events), true);
    }

    // test AGGREGATOR Operations:
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
