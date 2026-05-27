/*#![no_main]
extern crate alloc;

use risc0_zkvm::guest::env;
//use rsa::{pkcs1::DecodeRsaPublicKey, pkcs8::DecodePublicKey, RsaPublicKey};
//use rsa::pkcs1v15::VerifyingKey;
//use rsa::pkcs1v15::Signature as RsaSignature;
//use rsa::pkcs1::DecodeRsaPublicKey;
//use rsa::RsaPublicKey;

risc0_zkvm::guest::entry!(main);

fn main() {
    let ok = true;
    env::commit(&ok);
}

*/

//#![no_std]
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

use rsa::{
    pkcs1::DecodeRsaPublicKey,
    pkcs1v15::{Signature as RsaSignature, VerifyingKey},
    RsaPublicKey,
};
use sha2::Sha256;

risc0_zkvm::guest::entry!(main);

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

    // start the evaluator to obtain the boolean result over all event data
    let result = eval_program(&epl, &events) && sig_veri_result && check_list(index_list);

    // Commit the programs result to the journal

    //  env::commit(&result);
    env::commit(&number_of_events);
}
