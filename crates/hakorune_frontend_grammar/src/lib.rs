/*!
 * Hakorune frontend grammar tables and lookup engine.
 *
 * This crate owns dependency-light grammar data generated from
 * `grammar/legacy/nyash-v1.1-codegen-input.toml`. The Language v1 contract
 * reads `grammar/language-v1-registry.toml` independently. The main crate
 * keeps `crate::grammar::*` as a compatibility facade while parser/tokenizer
 * extraction proceeds.
 */

pub mod contract;
pub mod contract_corpus;
pub mod engine;
pub mod sugar_config;

pub mod generated_contract {
    include!(concat!(env!("OUT_DIR"), "/generated_contract.rs"));
}

// Generated tables from the named legacy codegen input.
pub mod generated;
pub use generated::*;
