/*!
 * Grammar compatibility facade.
 *
 * The frontend grammar engine and generated tables live in
 * `hakorune-frontend-grammar`; this module preserves the historical
 * `crate::grammar::*` import path during staged frontend crate extraction.
 */

pub use hakorune_frontend_grammar::*;
