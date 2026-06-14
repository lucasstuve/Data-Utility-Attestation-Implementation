#[cfg(test)]
pub mod lang_expressiveness_tests {
    extern crate alloc;
    use ::dnf_core::ast::Term;
    use dnf_core::epl::ProgramAst;
    use dnf_core::interpreter::{eval_program, Event};
    use system_core::parser;

    use crate::parser::parse_source;

    fn h1_query_to_epl(query: &str) -> Option<ProgramAst> {
        let program_ast = parser::parse_source(&query);
        let ast: ProgramAst = match program_ast {
            Err(e) => {
                println!("Error: {}", e);
                return None;
            }
            Ok(tree) => return Some(tree),
        };
    }

    fn events_1() -> Vec<Event> {
        let events = vec![
            Event {
                data: vec![Term::Int(8), Term::Bool(true), Term::Str("low".into())],
            },
            Event {
                data: vec![Term::Int(6), Term::Bool(true), Term::Str("low".into())],
            },
            Event {
                data: vec![Term::Int(8), Term::Bool(true), Term::Str("high".into())],
            },
            Event {
                data: vec![Term::Int(2), Term::Bool(false), Term::Str("low".into())],
            },
            Event {
                data: vec![Term::Int(1), Term::Bool(true), Term::Str("medium".into())],
            },
            Event {
                data: vec![Term::Int(5), Term::Bool(true), Term::Str("low".into())],
            },
            Event {
                data: vec![Term::Int(6), Term::Bool(false), Term::Str("high".into())],
            },
            Event {
                data: vec![Term::Int(9), Term::Bool(true), Term::Str("low".into())],
            },
            Event {
                data: vec![Term::Int(5), Term::Bool(true), Term::Str("high".into())],
            },
            Event {
                data: vec![Term::Int(10), Term::Bool(true), Term::Str("low".into())],
            },
        ];
        return events;
    }
    fn events_2() -> Vec<Event> {
        let events = vec![
            Event {
                data: vec![
                    Term::Int(8),
                    Term::Bool(true),
                    Term::Str("2026-06-11T10:00:00+00:00".into()),
                ],
            },
            Event {
                data: vec![
                    Term::Int(6),
                    Term::Bool(false),
                    Term::Str("2026-06-11T10:00:10+00:00".into()),
                ],
            },
            Event {
                data: vec![
                    Term::Int(8),
                    Term::Bool(true),
                    Term::Str("2026-06-11T10:00:20+00:00".into()),
                ],
            },
            Event {
                data: vec![
                    Term::Int(2),
                    Term::Bool(false),
                    Term::Str("2026-06-11T10:01:05+00:00".into()),
                ],
            },
            Event {
                data: vec![
                    Term::Int(1),
                    Term::Bool(true),
                    Term::Str("2026-06-11T10:01:20+00:00".into()),
                ],
            },
            Event {
                data: vec![
                    Term::Int(5),
                    Term::Bool(false),
                    Term::Str("2026-06-11T10:02:00+00:00".into()),
                ],
            },
            Event {
                data: vec![
                    Term::Int(5),
                    Term::Bool(false),
                    Term::Str("2026-06-11T10:02:00+00:00".into()),
                ],
            },
            Event {
                data: vec![
                    Term::Int(5),
                    Term::Bool(false),
                    Term::Str("2026-06-11T10:02:00+00:00".into()),
                ],
            },
            Event {
                data: vec![
                    Term::Int(5),
                    Term::Bool(false),
                    Term::Str("2026-06-11T10:02:00+00:00".into()),
                ],
            },
            Event {
                data: vec![
                    Term::Int(5),
                    Term::Bool(false),
                    Term::Str("2026-06-11T10:02:00+00:00".into()),
                ],
            },
        ];
        return events;
    }

    // r#"CREATE SCHEMA VehicleData (dataFieldName string, value float); assert ALL VehicleData (dataFieldName == "profiles.targetSOCPercentage" AND value > 20.0 ); assert (COUNT(value) > 1);"#;

    #[test]
    pub fn qc01_any_rare_event_true() {
        // Query
        let query = r#"CREATE SCHEMA Event(value int, is_on bool); assert ANY Event(value == 10 AND is_on == true); 
"#;
        // Parsed Program
        let evaluation_program = h1_query_to_epl(&query);
        // Evaluated Events:
        let events = events_1();
        // Evaluation Result:
        let eval_result = eval_program(&evaluation_program.unwrap(), &events);

        // Comparision of expected with actual evaluation result.
        assert_eq!(eval_result, true);
    }
    //  data: vec![Term::Int(2), Term::Bool(false), Term::Str("low".into())],
    #[test]
    pub fn qc04_dnf_complex_true() {
        // Query
        let query = r#"CREATE SCHEMA Event(value int, is_on bool, energy string); assert ANY Event(value > 1 AND value < 3 AND is_on == true AND energy == "low" OR value > 1 AND value < 3 AND is_on == false );"#;
        // Parsed Program
        let evaluation_program = h1_query_to_epl(&query);
        // Evaluated Events:
        let events = events_1();
        // Evaluation Result:
        let eval_result = eval_program(&evaluation_program.unwrap(), &events);

        // Comparision of expected with actual evaluation result.
        assert_eq!(eval_result, true);
    }
    #[test]
    pub fn qc03_string_all_true() {
        // Query
        let query = r#"CREATE SCHEMA Event(value int, is_on bool, energy string); assert ALL Event(energy == "medium" OR energy == "low" OR energy == "high"); 
"#;
        // Parsed Program
        let evaluation_program = h1_query_to_epl(&query);
        // Evaluated Events:
        let events = events_1();
        // Evaluation Result:
        let eval_result = eval_program(&evaluation_program.unwrap(), &events);

        // Comparision of expected with actual evaluation result.
        assert_eq!(eval_result, true);
    }

    #[test]
    pub fn qc03_string_any_true() {
        // Query
        let query = r#"CREATE SCHEMA Event(value int, is_on bool, energy string); assert ANY Event(energy == "medium"); 
"#;
        // Parsed Program
        let evaluation_program = h1_query_to_epl(&query);
        // Evaluated Events:
        let events = events_1();
        // Evaluation Result:
        let eval_result = eval_program(&evaluation_program.unwrap(), &events);

        // Comparision of expected with actual evaluation result.
        assert_eq!(eval_result, true);
    }
    #[test]
    pub fn qc03_string_all_false() {
        // Query
        let query = r#"CREATE SCHEMA Event(value int, is_on bool, energy string); assert ALL Event(energy == "medium"); 
"#;
        // Parsed Program
        let evaluation_program = h1_query_to_epl(&query);
        // Evaluated Events:
        let events = events_1();
        // Evaluation Result:
        let eval_result = eval_program(&evaluation_program.unwrap(), &events);

        // Comparision of expected with actual evaluation result.
        assert_eq!(eval_result, false);
    }
    #[test]
    pub fn qc04_string_or_any_true() {
        // Query
        let query = r#"CREATE SCHEMA Event(value int, is_on bool, energy string); assert ANY Event(energy == "medium" OR energy == "low"); 
"#;
        // Parsed Program
        let evaluation_program = h1_query_to_epl(&query);
        // Evaluated Events:
        let events = events_1();
        // Evaluation Result:
        let eval_result = eval_program(&evaluation_program.unwrap(), &events);

        // Comparision of expected with actual evaluation result.
        assert_eq!(eval_result, true);
    }
    #[test]
    pub fn qc04_string_or_all_false() {
        // Query
        let query = r#"CREATE SCHEMA Event(value int, is_on bool, energy string); assert ALL Event(energy == "medium" OR energy == "low"); 
"#;
        // Parsed Program
        let evaluation_program = h1_query_to_epl(&query);
        // Evaluated Events:
        let events = events_1();
        // Evaluation Result:
        let eval_result = eval_program(&evaluation_program.unwrap(), &events);

        // Comparision of expected with actual evaluation result.
        assert_eq!(eval_result, false);
    }

    #[test]
    pub fn qc01_any_rare_event_false() {
        // Query
        let query = r#"CREATE SCHEMA Event(value int, is_on bool); assert ANY Event(value == 10 AND is_on == false); 
"#;
        // Parsed Program
        let evaluation_program = h1_query_to_epl(&query);
        // Evaluated Events:
        let events = events_1();
        // Evaluation Result:
        let eval_result = eval_program(&evaluation_program.unwrap(), &events);

        // Comparision of expected with actual evaluation result.
        assert_eq!(eval_result, false);
    }

    #[test]
    pub fn qc02_all_range_true() {
        // Query
        let query = r#"CREATE SCHEMA Event(value int, is_on bool); assert ALL Event(value < 11 AND value > 0); 
"#;
        // Parsed Program
        let evaluation_program = h1_query_to_epl(&query);
        // Evaluated Events:
        let events = events_1();
        // Evaluation Result:
        let eval_result = eval_program(&evaluation_program.unwrap(), &events);

        // Comparision of expected with actual evaluation result.
        assert_eq!(eval_result, true);
    }
    #[test]
    pub fn qc02_all_range_false() {
        // Query
        let query = r#"CREATE SCHEMA Event(value int, is_on bool); assert ALL Event(value < 10 AND value > 0); 
"#;
        // Parsed Program
        let evaluation_program = h1_query_to_epl(&query);
        // Evaluated Events:
        let events: Vec<Event> = events_1();
        // Evaluation Result:
        let eval_result = eval_program(&evaluation_program.unwrap(), &events);

        // Comparision of expected with actual evaluation result.
        assert_eq!(eval_result, false);

        //Explaination of expected result: no Event with value 7, ( 6 < 7 < 8 )
    }

    #[test]
    pub fn qc01_any_range_true() {
        // Query
        let query = r#"CREATE SCHEMA Event(value int, is_on bool); assert ANY Event(value < 8 AND value > 2); 
"#;
        // Parsed Program
        let evaluation_program = h1_query_to_epl(&query);
        // Evaluated Events:
        let events = events_1();
        // Evaluation Result:
        let eval_result = eval_program(&evaluation_program.unwrap(), &events);

        // Comparision of expected with actual evaluation result.
        assert_eq!(eval_result, true);

        //Explaination of expected result: no Event with value 7, ( 6 < 7 < 8 )
    }

    #[test]
    pub fn qc01_any_range_false() {
        // Query
        let query = r#"CREATE SCHEMA Event(value int, is_on bool); assert ANY Event(value < 8 AND value > 6); 
"#;
        // Parsed Program
        let evaluation_program = h1_query_to_epl(&query);
        // Evaluated Events:
        let events = events_1();
        // Evaluation Result:
        let eval_result = eval_program(&evaluation_program.unwrap(), &events);

        // Comparision of expected with actual evaluation result.
        assert_eq!(eval_result, false);

        //Explaination of expected result: no Event with value 7, ( 6 < 7 < 8 )
    }
    #[test]
    pub fn qc05_count_eq_true() {
        // Query
        let query = r#"CREATE SCHEMA Event(value int, is_on bool); assert ANY Event(value >= 1); assert (COUNT(value) == 10);
"#;
        // Parsed Program
        let evaluation_program = h1_query_to_epl(&query);
        // Evaluated Events:
        let events = events_1();
        // Evaluation Result:
        let eval_result = eval_program(&evaluation_program.unwrap(), &events);

        // Comparision of expected with actual evaluation result.
        assert_eq!(eval_result, true);

        //Explaination of expected result: no Event with value 7, ( 6 < 7 < 8 )
    }

    #[test]
    pub fn qc05_count_lt_false() {
        // Query
        let query = r#"CREATE SCHEMA Event(value int, is_on bool); assert ANY Event(value >= 1); assert (COUNT(value) < 10);
"#;
        // Parsed Program
        let evaluation_program = h1_query_to_epl(&query);
        // Evaluated Events:
        let events = events_1();
        // Evaluation Result:
        let eval_result = eval_program(&evaluation_program.unwrap(), &events);

        // Comparision of expected with actual evaluation result.
        assert_eq!(eval_result, false);

        //Explaination of expected result: no Event with value 7, ( 6 < 7 < 8 )
    }
    #[test]
    pub fn qc05_count_after_any_false() {
        // Query
        let query = r#"CREATE SCHEMA Event(value int, is_on bool); assert ANY Event(value > 7); assert (COUNT(value) == 5);
"#;
        // Parsed Program
        let evaluation_program = h1_query_to_epl(&query);
        // Evaluated Events:
        let events = events_1();
        // Evaluation Result:
        let eval_result = eval_program(&evaluation_program.unwrap(), &events);

        // Comparision of expected with actual evaluation result.
        assert_eq!(eval_result, false);

        //Explaination of expected result: no Event with value 7, ( 6 < 7 < 8 )
    }

    #[test]
    pub fn qc05_count_ne_false() {
        // Query
        let query = r#"CREATE SCHEMA Event(value int, is_on bool); assert ANY Event(value >= 1); assert (COUNT(value) != 10);
"#;
        // Parsed Program
        let evaluation_program = h1_query_to_epl(&query);
        // Evaluated Events:
        let events = events_1();
        // Evaluation Result:
        let eval_result = eval_program(&evaluation_program.unwrap(), &events);

        // Comparision of expected with actual evaluation result.
        assert_eq!(eval_result, false);

        //Explaination of expected result: no Event with value 7, ( 6 < 7 < 8 )
    }

    #[test]
    pub fn qc05_sum_eq_true() {
        // Query
        let query = r#"CREATE SCHEMA Event(value int, is_on bool); assert ALL Event(value >= 1); assert (SUM (value) == 60 );
"#;
        // Parsed Program
        let evaluation_program = h1_query_to_epl(&query);
        // Evaluated Events:
        let events: Vec<Event> = events_1();
        // Evaluation Result:
        let eval_result = eval_program(&evaluation_program.unwrap(), &events);

        // Comparision of expected with actual evaluation result.
        assert_eq!(eval_result, true);

        //Explaination of expected result: no Event with value 7, ( 6 < 7 < 8 )
    }

    #[test]
    pub fn qc05_max_eq_true() {
        // Query
        let query = r#"CREATE SCHEMA Event(value int, is_on bool); assert ALL Event(value >= 1); assert (MAX (value) == 10 );
"#;
        // Parsed Program
        let evaluation_program = h1_query_to_epl(&query);
        // Evaluated Events:
        let events = events_1();
        // Evaluation Result:
        let eval_result = eval_program(&evaluation_program.unwrap(), &events);

        // Comparision of expected with actual evaluation result.
        assert_eq!(eval_result, true);

        //Explaination of expected result: no Event with value 7, ( 6 < 7 < 8 )
    }

    #[test]
    pub fn qc05_min_eq_true() {
        // Query
        let query = r#"CREATE SCHEMA Event(value int, is_on bool); assert ALL Event(value >= 1); assert (MIN (value) == 1 );
"#;
        // Parsed Program
        let evaluation_program = h1_query_to_epl(&query);
        // Evaluated Events:
        let events = events_1();
        // Evaluation Result:
        let eval_result = eval_program(&evaluation_program.unwrap(), &events);

        // Comparision of expected with actual evaluation result.
        assert_eq!(eval_result, true);

        //Explaination of expected result: no Event with value 7, ( 6 < 7 < 8 )
    }

    #[test]
    pub fn qc05_median_eq_true() {
        // Query
        let query = r#"CREATE SCHEMA Event(value int, is_on bool); assert ALL Event(value >= 1); assert (MEDIAN(value) == 6 );
"#;
        // Parsed Program
        let evaluation_program = h1_query_to_epl(&query);
        // Evaluated Events:
        let events: Vec<Event> = events_1();
        // Evaluation Result:
        let eval_result = eval_program(&evaluation_program.unwrap(), &events);

        // Comparision of expected with actual evaluation result.
        assert_eq!(eval_result, true);

        //Explaination of expected result: no Event with value 7, ( 6 < 7 < 8 )
    }
    #[test]
    pub fn qc05_avg_eq_true() {
        // Query
        let query = r#"CREATE SCHEMA Event(value int, is_on bool); assert ALL Event(value >= 1); assert (AVG(value) == 6 );
"#;
        // Parsed Program
        let evaluation_program = h1_query_to_epl(&query);
        // Evaluated Events:
        let events: Vec<Event> = events_1();
        // Evaluation Result:
        let eval_result = eval_program(&evaluation_program.unwrap(), &events);

        // Comparision of expected with actual evaluation result.
        assert_eq!(eval_result, true);

        //Explaination of expected result: no Event with value 7, ( 6 < 7 < 8 )
    }
    #[test]
    pub fn qc05_stddev_eq_true() {
        // Query
        let query = r#"CREATE SCHEMA Event(value int, is_on bool); assert ALL Event(value >= 1); assert (STDDEV(value) == 2.9059326290271157);
"#;
        // Parsed Program
        let evaluation_program = h1_query_to_epl(&query);
        // Evaluated Events:
        let events: Vec<Event> = events_1();
        // Evaluation Result:
        let eval_result = eval_program(&evaluation_program.unwrap(), &events);

        // Comparision of expected with actual evaluation result.
        assert_eq!(eval_result, true);

        //Explaination of expected result: no Event with value 7, ( 6 < 7 < 8 )
    }

    #[test]
    pub fn qc06_time_window_count_true() {
        // Query
        let query = r#"CREATE SCHEMA Event( value int, is_on bool, timestamp string); assert ALL Event(value > 0); assert Event#win:time(60s); assert (COUNT(value) > 2) ;
"#;
        // Parsed Program
        let evaluation_program = h1_query_to_epl(&query);
        // Evaluated Events:
        let events = events_2();
        // Evaluation Result:
        let eval_result = eval_program(&evaluation_program.unwrap(), &events);

        // Comparision of expected with actual evaluation result.
        assert_eq!(eval_result, true);

        //Explaination of expected result: no Event with value 7, ( 6 < 7 < 8 )
    }

    #[test]
    pub fn qc06_time_window_count_false() {
        // Query
        let query = r#"CREATE SCHEMA Event( value int, is_on bool, timestamp string); assert ALL Event(value > 0); assert Event#win:time(60s); assert (COUNT(value) > 3) ;
"#;
        // Parsed Program
        let evaluation_program = h1_query_to_epl(&query);
        // Evaluated Events:
        let events = events_2();
        // Evaluation Result:
        let eval_result = eval_program(&evaluation_program.unwrap(), &events);

        // Comparision of expected with actual evaluation result.
        assert_eq!(eval_result, false);

        //Explaination of expected result: no Event with value 7, ( 6 < 7 < 8 )
    }

    #[test]
    pub fn qc07_count_window_count_true() {
        // Query
        let query = r#"CREATE SCHEMA Event( value int, is_on bool, timestamp string); assert ALL Event(value > 0); assert Event#win:count(2); assert (COUNT(value) == 2) ;
"#;
        // Parsed Program
        let evaluation_program = h1_query_to_epl(&query);
        // Evaluated Events:
        let events = events_2();
        // Evaluation Result:
        let eval_result = eval_program(&evaluation_program.unwrap(), &events);

        // Comparision of expected with actual evaluation result.
        assert_eq!(eval_result, true);

        //Explaination of expected result: no Event with value 7, ( 6 < 7 < 8 )
    }
    #[test]
    pub fn qc07_count_window_count_false() {
        // Query
        let query = r#"CREATE SCHEMA Event( value int, is_on bool, timestamp string); assert ALL Event(value > 0); assert Event#win:count(2); assert (COUNT(value) > 2) ;
"#;
        // Parsed Program
        let evaluation_program = h1_query_to_epl(&query);
        // Evaluated Events:
        let events = events_2();
        // Evaluation Result:
        let eval_result = eval_program(&evaluation_program.unwrap(), &events);

        // Comparision of expected with actual evaluation result.
        assert_eq!(eval_result, false);

        //Explaination of expected result: no Event with value 7, ( 6 < 7 < 8 )
    }

    #[test]
    pub fn qc08_session_true() {
        // Query
        let query = r#"CREATE SCHEMA Event( value int, is_on bool, timestamp string); assert ALL Event(value > 0); assert SESSION(
        Event:start(value == 6) -> Event(value==8)  -> Event:end(value == 1) ) ;
"#;
        // Parsed Program
        let evaluation_program = h1_query_to_epl(&query);
        // Evaluated Events:
        let events = events_2();
        // Evaluation Result:
        let eval_result = eval_program(&evaluation_program.unwrap(), &events);

        // Comparision of expected with actual evaluation result.
        assert_eq!(eval_result, true);

        //Explaination of expected result: no Event with value 7, ( 6 < 7 < 8 )
    }
    #[test]
    pub fn qc08_session_false() {
        // Query
        let query = r#"CREATE SCHEMA Event( value int, is_on bool, timestamp string); assert ALL Event(value > 0); assert SESSION(
        Event:start(value == 1) -> Event(value==2) -> Event(value == 3 ) -> Event:end(value == 4) ) ;
"#;
        // Parsed Program
        let evaluation_program = h1_query_to_epl(&query);
        // Evaluated Events:
        let events = events_2();
        // Evaluation Result:
        let eval_result = eval_program(&evaluation_program.unwrap(), &events);

        // Comparision of expected with actual evaluation result.
        assert_eq!(eval_result, false);

        //Explaination of expected result: no Event with value 7, ( 6 < 7 < 8 )
    }
    #[test]
    pub fn qc09_pattern_true() {
        // Query
        let query = r#"CREATE SCHEMA Event( value int, is_on bool, timestamp string); assert ANY Event(is_on == true); assert PATTERN(Event(value == 8) -> Event(value == 8));
"#;
        // Parsed Program
        let evaluation_program = h1_query_to_epl(&query);
        // Evaluated Events:
        let events = events_2();
        // Evaluation Result:
        let eval_result = eval_program(&evaluation_program.unwrap(), &events);

        // Comparision of expected with actual evaluation result.
        assert_eq!(eval_result, true);

        //Explaination of expected result: no Event with value 7, ( 6 < 7 < 8 )
    }

    #[test]
    fn qc09_pattern_false() {
        // Query
        let query = r#"CREATE SCHEMA Event( value int, is_on bool, timestamp string); assert ANY Event(is_on == true); assert PATTERN(Event(value == 8) -> Event(value == 6));
"#;
        // Parsed Program
        let evaluation_program = h1_query_to_epl(&query);
        // Evaluated Events:
        let events = events_2();
        // Evaluation Result:
        let eval_result = eval_program(&evaluation_program.unwrap(), &events);

        // Comparision of expected with actual evaluation result.
        assert_eq!(eval_result, false);

        //Explaination of expected result: no Event with value 7, ( 6 < 7 < 8 )
    }
}
