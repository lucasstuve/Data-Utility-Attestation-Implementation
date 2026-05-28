use dnf_core::epl::ProgramAst;
use dnf_core::input_extractor::{efficient_event_extraction, grap_event_string};
use methods::{EVAL_AST_ELF, EVAL_AST_ID};
use system_core::data_consumer::{
    self, check_commitment, check_signature_alignment, generate_query_from_dnf,
    recalculate_predicate_hash, train_base_model, train_decision_tree_classifier,
    verify_attestation,
};
use system_core::manufacturer::{self, collect_batch, sign_batch, USER_BATCH_JSON};

mod parser;
mod predata_processor;

use predata_processor::create_events_indexes;

use risc0_zkvm::{default_prover, ExecutorEnv};

use colored::{ColoredString, Colorize};
use std::time::Instant;
use std::{str, thread, time::Duration};

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
    println!("\n{}\n", "START DATA UTILITY ATTESTATION PROTOCOL".bold());

    phase("PHASE I: Batch provisioning by the MANUFACTURER");

    manufacturer::collect_batch("user-batch.json", 1);

    step(
        1,
        "Manufacturer".red(),
        "collected user batch: user-batch.json.",
    );

    let json_batch: serde_json::Value =
        serde_json::from_str(&USER_BATCH_JSON).expect("JSON invalid.");

    println!("Loaded JSON batch:");
    println!("{:#?}", json_batch);

    let (batch_bytes, pub_key, signature) = manufacturer::sign_batch("user-batch.json");

    step(2, "Manufacturer".red(), "signed: user-batch.json.");

    println!("Signature:");
    println!("{:?}", &signature);

    step(
        3,
        "Manufacturer".red(),
        "ships user-batch.json, signature, and public key to the USER.",
    );

    phase("PHASE II: Utility rule generation by the DATA CONSUMER");

    data_consumer::train_base_model();

    step(4, "Data Consumer".green(), "trains AL base model.");

    data_consumer::train_decision_tree_classifier();

    step(
        5,
        "Data Consumer".green(),
        "obtains utility rules as DNF from decision tree classifier.",
    );

    let query = data_consumer::generate_query_from_dnf();

    step(
        6,
        "Data Consumer".green(),
        "generates query from predicate rules.",
    );

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

    let proving_time = env_timer.elapsed();

    println!("Proof generation completed.");
    println!("Proving time: {:?}", proving_time);

    step(
        11,
        "USER".purple(),
        "passes Attestation to the Data Consumer.",
    );

    phase("PHASE V: Verification by the DATA CONSUMER");

    step(
        12,
        "Data Consumer".green(),
        "verifies received Attestation.",
    );

    let (result, attestation_commit, attestation_sig) =
        data_consumer::verify_attestation(&receipt, EVAL_AST_ID);

    step(
        13,
        "Data Consumer".green(),
        "extracts the Utility Attestation result, commitment of parsed and hashed P, and signature.",
    );

    println!("Attestation result:");
    println!("Result: {:?}", result);
    println!("Commitment(P): {:?}", attestation_commit);
    println!("Signature: {:?}", signature);

    step(
        14,
        "Data Consumer".green(),
        "recomputes the commitment using the query.",
    );

    let expected_commitment = data_consumer::recalculate_predicate_hash(&query);

    let commit_holds = data_consumer::check_commitment(expected_commitment, attestation_commit);

    step(
        15,
        "Data Consumer".green(),
        "checks that committed P used for Attestation matches recomputed P.",
    );

    println!("Commitment holds: {:?}", commit_holds);

    let signature_correctly_used =
        data_consumer::check_signature_alignment(signature, attestation_sig);

    step(
        16,
        "Data Consumer".green(),
        "checks that the Signature committed in the Attestation belongs to the Manufacturer.",
    );

    println!(
        "Signature belongs to Manufacturer: {}",
        signature_correctly_used
    );

    phase("PHASE VI: Final purchase decision");

    step(
        17,
        "Data Consumer".green(),
        "decides whether to buy the data based on Manufacturer signature, committed P, and evaluation result.",
    );

    if commit_holds && signature_correctly_used && result {
        println!(
            "{}: requests purchase of the batch B_raw.",
            "Data Consumer".green()
        );
    } else {
        println!(
            "{}: rejects purchase of the batch B_raw.",
            "Data Consumer".green()
        );
    }

    println!("\n{}\n", "END DATA UTILITY ATTESTATION PROTOCOL".bold());
}
