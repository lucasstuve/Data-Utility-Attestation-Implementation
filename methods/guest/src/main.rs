//! The RISC Zero zkVM guest program (`eval_ast`): the private computation
//! whose execution the host proves. It re-derives events from the raw
//! batch, evaluates the DSL predicate over them, verifies the
//! Manufacturer's signature, and commits the result plus a commitment to
//! the evaluated logic to the journal — without revealing the raw batch or
//! the DSL source itself.

#![no_main]
extern crate alloc;

use alloc::vec::Vec;
use alloc::{string::String, vec};

use dnf_core::{
    epl::ProgramAst,
    index_list_validator::check_list,
    input_extractor::{efficient_event_extraction, grap_event_string},
    interpreter::{eval_program, Event},
};

use risc0_zkvm::guest::env;

use rsa::signature::Verifier;
use serde::{Deserialize, Serialize};

use rsa::{
    pkcs1::DecodeRsaPublicKey,
    pkcs1v15::{Signature as RsaSignature, VerifyingKey},
    RsaPublicKey,
};

use sha2::{Digest, Sha256};

risc0_zkvm::guest::entry!(main);


/// The guest's public output, committed via `env::commit`. Its layout must
/// stay in sync with `system_core::data_consumer::Journal`, which decodes
/// it from the receipt on the verifier side.
#[derive(Serialize, Deserialize)]
pub struct Journal {
    pub evaluation_result: bool,
    pub logic_commitment: [u8; 32],
    pub signature: Vec<u8>,
    pub number_of_events: u32,
}


fn main() {
    // Read AST input for evaluation
    let epl: ProgramAst = env::read();

    let input_len: u32 = env::read();
    let mut bytes = alloc::vec![0u8; input_len as usize];
    env::read_slice(bytes.as_mut_slice());

    let index_list: Vec<(u32, u32)> = env::read();

    let pub_key_len: u32 = env::read();
    let mut pub_key_pem = alloc::vec![0u8; (pub_key_len as usize)];
    env::read_slice(&mut pub_key_pem);

    let sig_length: usize = env::read();
    let mut signature = alloc::vec![0u8; sig_length];
    env::read_slice(&mut signature);

    let byte_slice = bytes.as_slice();


    // Verify Manufacturer signature over raw batch
    let pub_key = RsaPublicKey::from_pkcs1_der(&pub_key_pem).unwrap();
    let verifying_key = VerifyingKey::<Sha256>::new(pub_key);
    let sig = RsaSignature::try_from(signature.as_slice()).unwrap();

    let sig_veri_result = verifying_key.verify(&byte_slice, &sig).is_ok();


    // Validate supplied event index list
    let index_list_valid = check_list(index_list.clone());


    // Extract events from the raw batch
    let mut events: Vec<Event> = alloc::vec![];

    let schema0 = epl
        .schemas
        .get(0)
        .expect("schema[0] is not valid or missing");

    for index in index_list.clone() {
        let event = efficient_event_extraction(
            schema0.clone(),
            grap_event_string(index, byte_slice).as_bytes(),
        );

        if event.is_some() {
            events.push(event.unwrap());
        }
    }

    let number_of_events: u32 = events.len().try_into().unwrap();


    // Evaluate the utility predicate over the extracted events
    let utility_result = eval_program(&epl, &events);


    // Compute commitment to the evaluation logic
    let digest = Sha256::digest(epl.to_bytes());
    let mut evaluation_logic_commitment = [0u8; 32];
    evaluation_logic_commitment.copy_from_slice(&digest);


    // Construct the final evaluation result
    let result = utility_result && sig_veri_result && index_list_valid;


    // Commit the program result to the journal
    let journal = Journal {
        evaluation_result: result,
        logic_commitment: evaluation_logic_commitment,
        signature,
        number_of_events,
    };

    env::commit(&journal);
}