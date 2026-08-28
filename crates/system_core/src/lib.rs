//! Protocol-side logic around the DSL: batch generation/signing
//! ([`manufacturer`]), predicate derivation and attestation verification
//! ([`data_consumer`]), and the DSL parser ([`parser`]).

pub mod data_consumer;
pub mod manufacturer;
pub mod parser;
