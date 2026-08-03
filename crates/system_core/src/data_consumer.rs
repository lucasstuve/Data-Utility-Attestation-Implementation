use std::fmt::format;
use std::panic::panic_any;

use crate::parser;
use dnf_core::epl::ProgramAst;
use linfa::prelude::*;
use linfa::DatasetBase;
use linfa_trees::DecisionTree;
use ndarray::array;
use ndarray::{Array1, Array2};
use risc0_binfmt::Digestible;
use risc0_zkvm::Receipt;
use risc0_zkvm::compute_image_id;
use risc0_zkvm::{default_prover, ExecutorEnv, sha::{Digest as Risc0Digest,Impl}};
use serde::{Deserialize, Serialize};
use sha2::{digest, Digest, Sha256};
use methods::{EVAL_AST_ELF};




pub type ALDataset = DatasetBase<Array2<f64>, Array1<usize>>;

#[derive(Serialize, Deserialize)]
pub struct Journal {
    pub evaluation_result: bool,
    pub logic_commitment: [u8; 32],
    pub signature: Vec<u8>,
    pub number_of_events: u32,
}

pub fn base_model_dataset() -> ALDataset {
    println!("Mock: Training the AL model..................");
    //
    let soc_percentage_values = array![
        [5.0],
        [12.0],
        [18.0],
        [24.0],
        [31.0],
        [46.0],
        [46.0],
        [66.0],
        [73.0],
        [80.0]
    ];

    // Labels: noting whether the AL model considered the label or not.

    let labels =
        array![0usize, 0usize, 0usize, 0usize, 0usize, 0usize, 0usize, 1usize, 1usize, 1usize,];

    let data_set = Dataset::new(soc_percentage_values, labels);

    return data_set;
}


pub fn train_linfa_decision_tree(dataset: ALDataset) -> DecisionTree<f64, usize> {
    let tree = DecisionTree::params().fit(&dataset).unwrap();
    let accuracy = tree
        .predict(&dataset)
        .confusion_matrix(&dataset)
        .unwrap()
        .accuracy();
    assert!(accuracy > 0.6);

    println!("Traning AL-specific decision tree ... ");

    return tree;
}

pub fn obtain_predicate_from_decision_tree(
    tree: DecisionTree<f64, usize>,
    feature_name: &str,
    positve_label: usize,
) -> String {
    let root = tree.root_node();
    let (feature_index, thres_hold, impurity_decrease) = root.split();

    if root.is_leaf() {
        panic!("Decision tree has no splig, no threshold can be extracted.");
    }

    let children = root.children();

    let left_child = children[0].as_ref().expect("Left child in decision tree.");
    let right_child = children[1].as_ref().expect("Right child in decision tree.");

    let operator = if left_child.is_leaf() && left_child.prediction() == Some(positve_label) {
        "<="
    } else if right_child.is_leaf() && right_child.prediction() == Some(positve_label) {
        ">"
    } else {
        panic!("Operator could not be derived from decision tree.")
    };

    let predicate = format!(
        r#"dataFieldName == "{}" AND value {} {:.6}"#,
        feature_name, operator, thres_hold
    );

    println!("Deriving predicate rule...: {}", &predicate);

    return predicate;
}

pub fn generate_query_from_dnf() -> String {
    let query = r#"CREATE SCHEMA VehicleData (dataFieldName string, value float); assert ANY VehicleData (dataFieldName == "profiles.targetSOCPercentage" AND value > 20.0 ); assert (COUNT(value) > 1);"#;
    return query.to_string();
}
pub fn generate_query_from_textual_predicate(textual_predicate: &str) -> String {
    let query = format!(
        r#"CREATE SCHEMA VehicleData (dataFieldName string, value float); assert ANY VehicleData ({});"#,
        textual_predicate
    );

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

pub fn check_sig_alignment(issued: Vec<u8>, commited: Vec<u8>) -> bool {
    issued == commited
}

pub fn recompute_image_id() -> Result<Risc0Digest, Box<dyn std::error::Error>> {
    Ok(compute_image_id(EVAL_AST_ELF)?) 
}
pub fn validate_image_id(prover_id: Risc0Digest, recomputed_id: Risc0Digest) -> bool{
    return prover_id == recomputed_id;

}

pub fn get_image_id_from_receipt(receipt: &Receipt ) -> Risc0Digest{

  receipt.claim()
        .unwrap()
        .as_value()
        .unwrap()
        .pre
        .digest::<Impl>()
        .as_words()
        .try_into()
        .unwrap()

}

#[test]
fn hash_generation() {
    let query = r#"'CREATE SCHEMA VehicleData (dataFieldName string, value float); assert ALL VehicleData (dataFieldName == "profiles.targetSOCPercentage" AND value < 50.0 ); assert (COUNT(value) > 20);' "#;

    let hash = recalculate_predicate_hash(query);
    assert_eq!(hash, hash);
}

#[test]
fn generate_al_model() {
    let data = base_model_dataset();
    let al_decision_tree = train_linfa_decision_tree(data);
}
#[test]
fn test_textual_query_specification() {
    let data = base_model_dataset();
    let al_decision_tree = train_linfa_decision_tree(data);
    let predicate = obtain_predicate_from_decision_tree(
        al_decision_tree,
        "profiles.targetSOCPercentage",
        1usize,
    );
    let query = generate_query_from_textual_predicate(predicate.as_str());

    println!("{}", query);
}
