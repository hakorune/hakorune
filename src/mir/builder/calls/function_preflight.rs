//! Function-call preflight gate.
//!
//! Handles special source-level call forms before generic argument
//! materialization and route selection kick in.

use super::super::{MirBuilder, ValueId};
use crate::ast::ASTNode;

impl MirBuilder {
    pub(super) fn try_handle_function_preflight(
        &mut self,
        name: &str,
        args: &[ASTNode],
    ) -> Result<Option<ValueId>, String> {
        // Phase 285W-Syntax-0.1: Reject weak(...) function call syntax.
        // SSOT: docs/reference/language/lifecycle.md - weak <expr> is the ONLY valid syntax.
        if name == "weak" {
            let ring0 = crate::runtime::get_global_ring0();
            ring0
                .log
                .error("[Phase285W-0.1] Rejecting weak(...) function call");
            return Err(
                "Invalid syntax: weak(...). Use unary operator: weak <expr>\n\
                 Help: Change 'weak(obj)' to 'weak obj' (unary operator, no parentheses)\n\
                 SSOT: docs/reference/language/lifecycle.md"
                    .to_string(),
            );
        }

        if name == "externcall" {
            return self.build_explicit_extern_call(args.to_vec()).map(Some);
        }

        if self.comp_ctx.is_brand_declared(name) {
            return self
                .build_brand_constructor_call(name.to_string(), args.to_vec())
                .map(Some);
        }

        if let Some(result) = self.try_build_typeop_function(name, args)? {
            return Ok(Some(result));
        }

        if let Some(result) = self.try_handle_math_function(name, args.to_vec()) {
            return result.map(Some);
        }

        Ok(None)
    }
}
