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
                data: vec![Term::Int(8), Term::Bool(true), Term::Str("low".into() ), Term::Float(43.00) ],
            },
            Event {
                data: vec![Term::Int(6), Term::Bool(true), Term::Str("low".into()), Term::Float(4.00)],
            },
            Event {
                data: vec![Term::Int(8), Term::Bool(true), Term::Str("high".into()), Term::Float(456.00)],
            },
            Event {
                data: vec![Term::Int(2), Term::Bool(false), Term::Str("low".into()), Term::Float(93.00)],
            },
            Event {
                data: vec![Term::Int(1), Term::Bool(true), Term::Str("medium".into()), Term::Float(3.50)],
            },
            Event {
                data: vec![Term::Int(5), Term::Bool(true), Term::Str("low".into()), Term::Float(3.55)],
            },
            Event {
                data: vec![Term::Int(6), Term::Bool(false), Term::Str("high".into()), Term::Float(30.20)],
            },
            Event {
                data: vec![Term::Int(9), Term::Bool(true), Term::Str("low".into()), Term::Float(4.77)],
            },
            Event {
                data: vec![Term::Int(5), Term::Bool(true), Term::Str("high".into()), Term::Float(4.80)],
            },
            Event {
                data: vec![Term::Int(10), Term::Bool(true), Term::Str("low".into()), Term::Float(9.10)],
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

        //Explaination of expected result: The tenth event fulfills the value == 10 and is_on == true.

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

        //Explaination of expected result: Event with value = 2 and is_on = false satisfies right side of the disjunction. 

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

         //Explaination of expected result: this must be true, because all events are either energy = low, medium or high.

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

         //Explaination of expected result: one event with energy = medium is present in the set. 

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

          //Explaination of expected result: some events have energy = low, or high. 

        // Comparision of expected with actual evaluation result.
        assert_eq!(eval_result, false);
    }

    #[test]
    pub fn qc03_int_true(){
            // Query
        let query = r#"CREATE SCHEMA Event(value int, is_on bool, energy string); assert ANY Event(value == 6); 
"#;
        // Parsed Program
        let evaluation_program = h1_query_to_epl(&query);
        // Evaluated Events:
        let events = events_1();
        // Evaluation Result:
        let eval_result = eval_program(&evaluation_program.unwrap(), &events);

            //Explaination of expected result: two events have the value 6. 

        // Comparision of expected with actual evaluation result.
        assert_eq!(eval_result, true);

    }
     #[test]
    pub fn qc03_int_false(){
            // Query
        let query = r#"CREATE SCHEMA Event(value int, is_on bool, energy string); assert ANY Event(value == 99); 
"#;
        // Parsed Program
        let evaluation_program = h1_query_to_epl(&query);
        // Evaluated Events:
        let events = events_1();
        // Evaluation Result:
        let eval_result = eval_program(&evaluation_program.unwrap(), &events);

        //Explaination of expected result: no event has value the value 99. 

        // Comparision of expected with actual evaluation result.
        assert_eq!(eval_result, false);

    }


    #[test]
    pub fn qc03_float_false(){
             // Query
        let query = r#"CREATE SCHEMA Event(value int, is_on bool, energy string, amount float); assert ANY Event(amount == 93.10); 
"#;
        // Parsed Program
        let evaluation_program = h1_query_to_epl(&query);
        // Evaluated Events:
        let events = events_1();
        // Evaluation Result:
        let eval_result = eval_program(&evaluation_program.unwrap(), &events);

        //Explaination of expected result: there is no event with amount 93.10 

        // Comparision of expected with actual evaluation result.
        assert_eq!(eval_result, false);
    }

    #[test]
    pub fn qc03_float_true(){
              // Query
        let query = r#"CREATE SCHEMA Event(value int, is_on bool, energy string, amount float); assert ANY Event(amount == 93.00); 
"#;
        // Parsed Program
        let evaluation_program = h1_query_to_epl(&query);
        // Evaluated Events:
        let events = events_1();
        // Evaluation Result:
        let eval_result = eval_program(&evaluation_program.unwrap(), &events);

        //Explaination of expected result: there is one event with amount 93.00.

        // Comparision of expected with actual evaluation result.
        assert_eq!(eval_result, true);

    }
     

      #[test]
    pub fn qc03_bool_true(){
                 // Query
        let query = r#"CREATE SCHEMA Event(value int, is_on bool, energy string, amount float); assert ANY Event(is_on == true); 
"#;
        // Parsed Program
        let evaluation_program = h1_query_to_epl(&query);
        // Evaluated Events:
        let events = events_1();
        // Evaluation Result:
        let eval_result = eval_program(&evaluation_program.unwrap(), &events);

        // Comparision of expected with actual evaluation result.
        assert_eq!(eval_result, true);


        //Explaination of expected result: there are several events where is_on = true. 


    }
    #[test]
    pub fn qc03_bool_false(){
                         // Query
        let query = r#"CREATE SCHEMA Event(value int, is_on bool, energy string, amount float); assert ANY Event(is_on != true AND is_on != false); 
"#;
        // Parsed Program
        let evaluation_program = h1_query_to_epl(&query);
        // Evaluated Events:
        let events = events_1();
        // Evaluation Result:
        let eval_result = eval_program(&evaluation_program.unwrap(), &events);

        // Comparision of expected with actual evaluation result.
        assert_eq!(eval_result, false);


        //Explaination of expected result: there are no other optionen beyond true, or false. 

        
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

        //Explaination of expected result: there are events with energy = medium or low. 
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

        //Explaination of expected result: some events have energy = high.  
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

        //Explaination of expected result: there is only one event with value = 10, but it has is_on = true.
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

        
        //Explaination of expected result: all events have values between 1-10 . 
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

        //Explaination of expected result: event with value = 10 present. 

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

        //Explaination of expected result: for example the event with the value = 6, is higher 2 and lower than 8. 
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

        //Explaination of expected result: there is no event with the value = 7. 
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

        //Explaination of expected result: there are exact 10 events with the value >= 1. 
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

        //Explaination of expected result: because count is exact 10, the last statement is false. 
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

        //Explaination of expected result: values greater than 7: 8,8,9,10 => only 4. 
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

        //Explaination of expected result: there are exact 10 events with values >= 1.
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

        //Explaination of expected result: 8+6+8+2+1+5+6+9+5+10 = 60. 
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

        //Explaination of expected result: the event with value = 10 has highest value in the set. 
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

        //Explaination of expected result: the smallest value of an event is indeed 1. 
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

        //Explaination of expected result: sorted => 1, 2, 5, 5, 6, 6, 8, 8, 9, 10  => median = 6 
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

        //Explaination of expected result: sum(value) = 60;  60 / 10 events => avg = 6 
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

        //Explaination of expected result: 2.9059326290271157 with calculator. 
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

        //Explaination of expected result:  3 events are in the 60s window. 
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

        //Explaination of expected result: 3 events are in the 60s window, so it is false. 
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

        //Explaination of expected result: the count window contains 2 events, so it is true. 
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

        //Explaination of expected result: the count window contains only two events. 
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

        //Explaination of expected result: the event sequence contains 6 -> 8 -> 1, so session pattern is found. 
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

        //Explaination of expected result: the sequence 1 -> 2 -> 3 -> 4 does not occur. 
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

        //Explaination of expected result: two events with value = 8 occur in the required order. 
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

        //Explaination of expected result: the filtered sequence does (is_on = true), does not contain the sequence 8 -> 6. 
    }
}
    