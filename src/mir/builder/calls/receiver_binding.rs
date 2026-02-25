//! Receiver ('me'/'this') normalization and binding
//!
//! Responsibilities:
//! - Normalize 'this'/'me' in method calls to proper call forms
//! - Handle 'this' in static vs instance context
//! - Handle 'me' with/without module context
//!
//! Key functions:
//! - try_normalize_this_me_method_call: Main entry point for this/me normalization
//!
//! Design notes:
//! - Uses comp_ctx.current_static_box to determine static context
//! - Uses scope_ctx.current_function for instance method detection
//! - Falls back to static_resolution for static method calls

use super::super::{MirBuilder, ValueId};
use crate::ast::ASTNode;

impl MirBuilder {
    /// Phase 269 P1.2: ReceiverNormalizeBox - MethodCall 共通入口 SSOT
    ///
    /// Normalizes this/me method calls to appropriate call forms:
    /// - Static box context: this.method() → BoxName.method() (compile-time)
    /// - Instance context: me.method() → instance method call (runtime)
    pub(super) fn try_normalize_this_me_method_call(
        &mut self,
        object: &ASTNode,
        method: &str,
        arguments: &[ASTNode],
    ) -> Result<Option<ValueId>, String> {
        if !matches!(object, ASTNode::This { .. } | ASTNode::Me { .. }) {
            return Ok(None);
        }

        // Priority 1: static box → compile-time static call normalization
        if let Some(box_name) = self.comp_ctx.current_static_box.clone() {
            if crate::config::env::builder_trace_normalize() {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug(&format!(
                    "[trace:normalize] this.{}() → {}.{}() (static call)",
                    method, box_name, method
                ));
            }
            // this.method(args) → current_static_box.method/arity(args)
            // Delegate to static_resolution module for static method handling
            return Ok(Some(self.handle_static_method_call(
                &box_name, method, arguments,
            )?));
        }

        // Instance method fallback (requires variable_map["me"])
        self.handle_me_method_call(method, arguments)
    }
}
