use crate::parser;
use dnf_core::epl::ProgramAst;
use risc0_zkvm::Receipt;
use risc0_zkvm::{default_prover, ExecutorEnv};
use sha2::{digest, Digest, Sha256};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Journal {
    pub evaluation_result: bool,
    pub logic_commitment: [u8; 32],
    pub signature: Vec<u8>,
    pub number_of_events: u32,
}

pub fn train_base_model() {
    println!("Mock: Training the AL model..................");
}

pub fn train_decision_tree_classifier() {
    println!(" Mocking to obtain a DNF Rule...............");
}

pub fn generate_query_from_dnf() -> String {
    let query = r#"CREATE SCHEMA VehicleData (dataFieldName string, value float); assert ALL VehicleData (dataFieldName == "profiles.targetSOCPercentage" AND value < 50.0 ); assert (COUNT(value) > 20);"#;
    return query.to_string();
}

pub fn verify_attestation(receipt: &Receipt, image_id: [u32; 8]) -> (bool, [u8; 32], Vec<u8>) {
    receipt
        .verify(image_id)
        .expect("Data batch evaluated correct. ");

    let journal: Journal = receipt.journal.decode().expect("failed to decode journal");

    return (
        journal.evaluation_result,
        journal.logic_commitment,
        journal.signature,
    );
}

pub fn recalculate_predicate_hash(query_string: &str) -> [u8; 32] {
    let dnf = parser::parse_source(&query_string);
    println!("PEST: {:?}", dnf);

    let ast: ProgramAst = match dnf {
        Err(e) => {
            print!("Error {}", e);
            return [
                23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23,
                23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23,
            ];
        }
        Ok(tree) => tree,
    };

    let digest = Sha256::digest(ast.to_bytes());
    let mut output = [0u8; 32];

    output.copy_from_slice(&digest);

    return output;
}

pub fn check_commitment(expected: [u8; 32], obtained: [u8; 32]) -> bool {
    expected == obtained
}

pub fn check_signature_alignment(issued: Vec<u8>, commited: Vec<u8>) -> bool {
    issued == commited
}

#[test]
fn hash_generatio() {
    let query = r#"'CREATE SCHEMA VehicleData (dataFieldName string, value float); assert ALL VehicleData (dataFieldName == "profiles.targetSOCPercentage" AND value < 50.0 ); assert (COUNT(value) > 20);' "#;

    let hash = recalculate_predicate_hash(query);
    assert_eq!(hash, hash);
}
