/*!
 * Result/Option prelude compatibility facade.
 *
 * Passive enum declaration construction lives in `hakorune-frontend-ast`.
 * This module preserves the historical `crate::semantics::result_option_prelude`
 * path while parser and stage1 callsites migrate.
 */

pub use hakorune_frontend_ast::result_option_prelude::*;
