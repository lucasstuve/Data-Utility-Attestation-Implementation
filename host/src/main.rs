use dnf_core::epl::ProgramAst;
use dnf_core::input_extractor::{efficient_event_extraction, grap_event_string};
use methods::{EVAL_AST_ELF, EVAL_AST_ID};
use system_core::data_consumer::{
    self, base_model_dataset, check_commitment, check_sig_alignment, generate_query_from_dnf,
    generate_query_from_textual_predicate, obtain_predicate_from_decision_tree,
    recalculate_predicate_hash, train_linfa_decision_tree, verify_attestation, recompute_image_id, validate_image_id
};
use system_core::data_consumer as al_company;
use system_core::manufacturer::{
    self, collect_batch, sign_batch, USER_BATCH_JSON, VW_BATCH_JSON,BATCH_JSON_10MB, BATCH_JSON_1000MB, BATCH_JSON_100MB
};


// DEV_MODE=1  cargo run -p host --release -- "test-data-10-MB.json"  // Run this command to test illustrate the end-to-end system with CPU

mod parser;
mod predata_processor;

use predata_processor::create_events_indexes;

use risc0_zkvm::{default_prover, ExecutorEnv};

use colored::{ColoredString, Colorize};
use core::time;
use std::env;
use std::time::Instant;
use std::{str, thread, time::Duration};
use benchmarks::performance_benchmarks::{write_results_csv, Row}; 



fn separator() {
    println!("\n{}\n", "-".repeat(94));
}

fn phase(title: &str) {
    println!("\n\n{}", "=".repeat(94));
    println!("{}", title);
    println!("{}\n", "=".repeat(94));
}

fn step(number: u32, actor: ColoredString, message: &str) {
    separator();
    println!("({}): {} {}", number, actor, message);
    separator();

    thread::sleep(Duration::from_secs(2));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // first argument selects the dataset e.g. cargo run -- "user-batch.json"

    let mut bench: Row = Row::new();

    println!(
        "\n{}{}",
        "START DATA UTILITY ATTESTATION PROTOCOL FOR DATASET:".bold(),
        &args[1]
    );

    phase("PHASE I: Batch provisioning by the MANUFACTURER");

    let mut data = "";
    if (&args[1] == &String::from("vw-batch.json")) {
        data = &VW_BATCH_JSON; 
    } else if &args[1] == &String::from("user-batch.json") {
        data = &USER_BATCH_JSON; } 
    else if &args[1] == &String::from("test-data-10-MB.json") {
        data = &BATCH_JSON_10MB; 
    }
    else if &args[1] == &String::from("test-data-100-MB.json") {
        data = &BATCH_JSON_100MB; 
    }
    else if &args[1] == &String::from("test-data-1000-MB.json") {
        data = &BATCH_JSON_1000MB; 
    }

    // For test the Manufactuerer data generation is replaced by controlled data batches.
    //  manufacturer::collect_batch(&args[1], 1);

    step(
        1,
        "Manufacturer".red(),
        &format!("collected user batch: {}", &data),
    );

    let json_batch: serde_json::Value = serde_json::from_str(&data).expect("JSON invalid.");

    println!("Loaded JSON batch:");
    println!("{:#?}", json_batch);

    let (batch_bytes, pub_key, signature) = manufacturer::sign_batch(&args[1]);

    let message = format!("signed: {}.", &data);
    step(2, "Manufacturer".red(), &message);

    println!("Signature:");
    println!("{:?}", &signature);

    step(
        3,
        "Manufacturer".red(),
        &format!("ships {}, signature, and public key to the USER.", &data),
    );

    phase("PHASE II: Utility rule generation by the Predictive-Maintenance Company");

    let data_from_base_model = al_company::base_model_dataset();

    step(4, "Predictive-Maintenance Company".green(), "trains AL base model.");

    println!(
        "Initial labelled dataset: {:?} ",
        al_company::base_model_dataset()
    );

    step(
        5,
        "Predictive-Maintenance Company".green(),
        "obtains utility rules as DNF from decision tree classifier.",
    );

    let decision_tree = al_company::train_linfa_decision_tree(data_from_base_model);
    let textual_predicate = al_company::obtain_predicate_from_decision_tree(
        decision_tree,
        "profiles.targetSOCPercentage",
        1usize,
    );

    step(
        6,
        "Predictive-Maintenance Company".green(),
        "generates query from predicate rules.",
    );

    let query = al_company::generate_query_from_textual_predicate(&textual_predicate);
    println!("Generated query:");
    println!("{}", query);

    step(
        7,
        "USER".purple(),
        "receives query and parses the Utility Attestation program.",
    );

    let dnf = parser::parse_source(&query);

    let ast: ProgramAst = match dnf {
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
        Ok(tree) => tree,
    };

    println!("{}", "Parsed Abstract Syntax Tree from CPR:".yellow());
    println!("{:#?}", ast);

    phase("PHASE III: Preprocessing by the USER");

    let batch_string = str::from_utf8(&batch_bytes).unwrap();
    let index_list = create_events_indexes(batch_string);

    step(
        8,
        "USER".purple(),
        "computes list of index pairs from B_raw.",
    );

    println!("Index list:");
    println!("{:#?}", index_list);

    let file_len: u32 = batch_bytes.len().try_into().unwrap();
    let sig_length: u32 = signature.len().try_into().unwrap();

    phase("PHASE IV: Proof generation by the USER");

    let env_timer = Instant::now();

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    step(
        9,
        "USER".purple(),
        "inputs Signature(M), Batch, AST, IndexList, and Pk_m into attested computation environment.",
    );

    let env = ExecutorEnv::builder()
        .write(&ast)
        .unwrap()
        .write(&file_len)
        .unwrap()
        .write_slice(&batch_bytes)
        .write(&index_list)
        .unwrap()
        .write(&(pub_key.len() as u32))
        .unwrap()
        .write_slice(&pub_key)
        .write(&(sig_length as usize))
        .unwrap()
        .write_slice(&signature)
        .build()
        .unwrap();

    let prover = default_prover();

    step(
        10,
        "USER".purple(),
        "initiates proof/attestation generation.",
    );

    let prove_info = prover.prove(env, EVAL_AST_ELF).unwrap();
    let receipt = prove_info.receipt;

    let proving_time = env_timer.elapsed().as_secs();

    println!("Proof generation completed.");
    println!("Proving time: {:?}", proving_time);

    step(
        11,
        "USER".purple(),
        "passes Attestation to the Predictive-Maintenance Company.",
    );

    phase("PHASE V: Verification by the Predictive-Maintenance Company");

    step(
        12,
        "Predictive-Maintenance Company".green(),
        "verifies received Attestation.",
    );

    let verify_timer = Instant::now(); 

    let (result, attestation_commit, attestation_sig) =
        al_company::verify_attestation(&receipt, EVAL_AST_ID);
       let verify_attest_time =  verify_timer.elapsed().as_millis(); 
    step(
        13,
        "Predictive-Maintenance Company".green(),
        "extracts the Utility Attestation result, commitment of parsed and hashed P, and signature.",
    );

    println!("Attestation result:");
    println!("Result: {:?}", result);
    println!("Commitment(P): {:?}", attestation_commit);
    println!("Signature: {:?}", signature);

    step(
        14,
        "Predictive-Maintenance Company".green(),
        "recomputes the commitment using the query.",
    );

    let expected_commitment = al_company::recalculate_predicate_hash(&query);

    let commit_holds = al_company::check_commitment(expected_commitment, attestation_commit);

    step(
        15,
        "Predictive-Maintenance Company".green(),
        "checks that committed P used for Attestation matches recomputed P.",
    );

    println!("Commitment holds: {:?}", commit_holds);

    let signature_correctly_used = al_company::check_sig_alignment(signature, attestation_sig);

    step(
        16,
        "Predictive-Maintenance Company".green(),
        "checks that the Signature committed in the Attestation belongs to the Manufacturer.",
    );

    println!(
        "Signature belongs to Manufacturer: {}",
        signature_correctly_used
    );

    step(
        17,
        "Predictive-Maintenance Company".green(),
        "recomputes image ID based on audited program & compares it with receipt image id.",
    );

    let recomputed_id = data_consumer::recompute_image_id(); 
    let image_id_from_receipt = data_consumer::get_image_id_from_receipt(&receipt); 
    let recomputed_id_matches  = data_consumer::validate_image_id(image_id_from_receipt , recomputed_id.unwrap()); 

    println!(
        "ZK-Attestation proofs corresponds to the provided receipt: {}",
        recomputed_id_matches
    );

    phase("PHASE VI: Final purchase decision");

    step(
        17,
        "Predictive-Maintenance Company".green(),
        "decides whether to buy the data based on Manufacturer signature, committed P, and evaluation result.",
    );

    if commit_holds && signature_correctly_used && recomputed_id_matches && result {
        println!(
            "{}: requests purchase of the batch B_raw.",
            "Predictive-Maintenance Company".green()
        );
    } else {
        println!(
            "{}: rejects purchase of the batch B_raw.",
            "Predictive-Maintenance Company".green()
        );
    }

    println!("\n{}\n", "END DATA UTILITY ATTESTATION PROTOCOL".bold());

   

    bench.set_dsl_query(&args[1]);
    bench.set_segments(prove_info.stats.segments as u32);
    bench.set_user_cycles(prove_info.stats.user_cycles as u32);
    bench.set_total_cycles(prove_info.stats.total_cycles as u32);
    bench.set_prove_time(proving_time as u32);
    bench.set_veri_time(verify_attest_time as u64 );
    bench.set_input_bytes(batch_bytes.len() as u64 );

    write_results_csv(vec![bench], "end-to-end-benchmarks.csv").unwrap(); 



}
