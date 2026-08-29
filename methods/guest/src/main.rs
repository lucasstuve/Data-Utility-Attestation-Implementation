//! The RISC Zero zkVM guest program (`eval_ast`) exercised by these
//! benchmarks: re-derives events from the raw batch, verifies the
//! Manufacturer's signature, and evaluates the DSL predicate over them —
//! the same private computation as on `end-to-end-system`, but only the
//! event count is committed to the journal here, since these benchmarks
//! measure proving cost rather than the full attestation output.

#![no_main]
extern crate alloc;

use alloc::vec::Vec;
use alloc::{string::String, vec};
use dnf_core::{
    epl::ProgramAst,
    input_extractor::{efficient_event_extraction, grap_event_string},
    interpreter::{eval_program, Event},
    index_list_validator::check_list,
};
use sha2::{Digest, Sha256};

use risc0_zkvm::guest::env;
use rsa::signature::Verifier;

use rsa::{
    pkcs1::DecodeRsaPublicKey,
    pkcs1v15::{Signature as RsaSignature, VerifyingKey},
    RsaPublicKey,
};
use serde::{Deserialize, Serialize};

risc0_zkvm::guest::entry!(main);

/// Mirrors the `end-to-end-system` branch's guest journal shape for
/// parity; on this branch only `number_of_events` is actually committed
/// below (see `main`), so `evaluation_result`/`logic_commitment`/
/// `signature` here are currently unused.
#[derive(Serialize, Deserialize)]
pub struct Journal {
    pub evaluation_result: bool,
    pub logic_commitment: [u8; 32],
    pub signature: Vec<u8>,
    pub number_of_events: u32,
}

fn main() {
    // read AST Input for evaluation
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

    let mut events: Vec<Event> = alloc::vec![];

    let schema0 = epl
        .schemas
        .get(0)
        .expect("schema[0] is not valid or missing");

    let byte_slice = bytes.as_slice();

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

    

    let pub_key = RsaPublicKey::from_pkcs1_der(&pub_key_pem).unwrap();
    let verifying_key = VerifyingKey::<Sha256>::new(pub_key);
    let sig = RsaSignature::try_from(signature.as_slice()).unwrap();

    let sig_veri_result = verifying_key.verify(&byte_slice, &sig).is_ok();

        // Compute commitment to the evaluation logic
    let digest = Sha256::digest(epl.to_bytes());
    let mut evaluation_logic_commitment = [0u8; 32];
    evaluation_logic_commitment.copy_from_slice(&digest);

    // start the evaluator to obtain the boolean result over all event data
    let result = eval_program(&epl, &events) && sig_veri_result && check_list(index_list);


    // Commit the program result to the journal

    env::commit(&number_of_events);
}
