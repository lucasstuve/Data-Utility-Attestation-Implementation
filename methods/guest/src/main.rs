#![no_std]
#![no_main]
extern crate alloc;

use alloc::vec::Vec;

use dnf_core::{
    ast::Term,
    epl::ProgramAst,
    input_extractor::extract_events,
    interpreter::{eval_program, Event},
};

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

fn main() {
    // read AST Input for evaluation
    let mut epl: ProgramAst = env::read();

    let input_event =  [br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"30","timestampUtc":"2025-12-18T16:45:03.484Z"}"#, br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"30","timestampUtc":"2025-12-18T16:45:03.484Z"}"#, br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"80","timestampUtc":"2025-12-18T16:45:03.484Z"}"#] ;

    let inputs: [&[u8]; 100] = [
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"13","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"50","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"87","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"23","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"60","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"97","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"33","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"70","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"6","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"43","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"80","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"16","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"53","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"90","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"26","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"63","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"100","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"36","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"73","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"9","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"46","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"83","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"19","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"56","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"93","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"29","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"66","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"2","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"39","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"76","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"12","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"49","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"86","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"22","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"59","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"96","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"32","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"69","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"5","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"42","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"79","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"15","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"52","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"89","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"25","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"62","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"99","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"35","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"72","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"8","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"45","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"82","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"18","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"55","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"92","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"28","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"65","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"1","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"38","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"75","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"11","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"48","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"85","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"21","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"58","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"95","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"31","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"68","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"4","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"41","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"78","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"14","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"51","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"88","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"24","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"61","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"98","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"34","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"71","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"7","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"44","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"81","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"17","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"54","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"91","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"27","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"64","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"0","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"37","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"74","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"10","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"47","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"84","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"999","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"57","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"94","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"30","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"67","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"3","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
    br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"40","timestampUtc":"2025-12-18T16:45:03.484Z"}"#,
];
    let mut events: Vec<Event> = Vec::new();

    for input in inputs {
        events.push(extract_events(
            epl.schemas.get(0).expect("User Program has schema").clone(),
            input,
        ));
    }

    let result = eval_program(epl, events);
    // Commit the result of the evaluation:
    env::commit(&result);
}

/*

pub fn fill_array() -> [&'a [u8]; 1000] {
    let mut inputs: [&[u8]; 1000];

    let mut i = 0;

    while i < 999 {

        inputs[i] = br#"{"key":"c161867b-0566-3cc8-9f60-989848640bcc","dataFieldName":"profiles.targetSOCPercentage","value":"40","timestampUtc":"2025-12-18T16:45:03.484Z"}"#;

        i = i + 1;
    }

    return inputs;
}

*/
