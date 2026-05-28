use benchmarks::data_generator::{
    generate_data_by_frequency, generate_test_data, generate_test_data_large,
};
use rand::rngs::OsRng;
use rsa::signature::{RandomizedSigner, SignatureEncoding};
use rsa::{
    pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey, LineEnding},
    pkcs1v15::SigningKey,
    RsaPrivateKey, RsaPublicKey,
};
use sha2::Sha256;

use rsa::pkcs1::DecodeRsaPublicKey;
use rsa::pkcs1v15::{Signature, VerifyingKey};
use rsa::signature::Verifier;

use std::env;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;
use std::time::Instant;
pub fn collect_batch(file: &str, size: u64) -> &str {
    let output_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("test_data");
    generate_data_by_frequency(0.05, file, size, output_dir);
    return file;
}
pub const USER_BATCH_JSON: &str = include_str!("../test_data/user-batch.json");

pub fn sign_batch(file: &str) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let base = Path::new(env!("CARGO_MANIFEST_DIR")).join("test_data");
    let path = base.join(file);

    let mut rng = OsRng;

    let file_name = path; // r"host/src/example_vw_event_data_window_test.json";

    let file_bytes: Vec<u8> = fs::read(file_name).unwrap();

    let private_key = RsaPrivateKey::new(&mut rng, 2048).unwrap();
    let public_key = RsaPublicKey::from(&private_key);

    let priv_pem = private_key
        .to_pkcs1_pem(LineEnding::LF)
        .unwrap()
        .to_string();

    let pub_pem = public_key.to_pkcs1_pem(LineEnding::LF).unwrap().to_string();

    fs::write("./private_key.pem", &priv_pem).unwrap();
    fs::write("./public_key.pem", &pub_pem).unwrap();

    let singing_key = SigningKey::<Sha256>::new(private_key.clone());

    let mut rng = OsRng;

    let public_key = RsaPublicKey::from(&private_key);
    let public_key_pem = public_key.to_pkcs1_der().unwrap().as_bytes().to_vec();

    let signature = singing_key.sign_with_rng(&mut rng, &file_bytes).to_vec();
    let sig_length: u32 = signature.len().try_into().unwrap();

    return (file_bytes, public_key_pem, signature);
}

#[test]
fn test_sign_batch_signature_verifies() {
    let (file_bytes, public_key_der, signature_bytes) = sign_batch("test-data-1KB.json");

    let public_key = RsaPublicKey::from_pkcs1_der(&public_key_der).unwrap();

    let verifying_key = VerifyingKey::<Sha256>::new(public_key);

    let signature = Signature::try_from(signature_bytes.as_slice()).unwrap();

    assert!(verifying_key.verify(&file_bytes, &signature).is_ok());
}
#[test]
fn test_issuance_batch() {
    collect_batch("test-batch.json", 1);
    assert!(true);
}
