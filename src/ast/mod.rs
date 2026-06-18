/*!
 * Nyash AST (Abstract Syntax Tree) - compatibility facade.
 *
 * Passive frontend AST data lives in `hakorune-frontend-ast`. The main crate
 * keeps this module as the historical `crate::ast::*` import path and owns
 * only runtime bridges such as `literal_box_bridge`.
 */

pub mod literal_box_bridge;

pub use hakorune_frontend_ast::*;
