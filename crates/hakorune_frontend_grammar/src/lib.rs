/*!
 * Hakorune frontend grammar tables and lookup engine.
 *
 * This crate owns dependency-light grammar data generated from
 * `grammar/unified-grammar.toml`. The main crate keeps `crate::grammar::*` as
 * a compatibility facade while parser/tokenizer extraction proceeds.
 */

pub mod engine;

// Generated tables from grammar/unified-grammar.toml
pub mod generated;
pub use generated::*;
