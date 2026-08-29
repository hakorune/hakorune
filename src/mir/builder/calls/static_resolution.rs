//! Static receiver classification for qualified method calls.
//!
//! Responsibilities:
//! - Static receiver method call resolution (BoxName.method → static method)
//!
//! Key functions:
//! - resolve_static_receiver_box_name: classify BoxName.method(args) syntax

use super::super::MirBuilder;
use crate::ast::ASTNode;

impl MirBuilder {
    pub(in crate::mir::builder) fn resolve_static_receiver_box_name(
        &self,
        object: &ASTNode,
    ) -> Option<String> {
        let ASTNode::Variable { name: obj_name, .. } = object else {
            return None;
        };

        if let Some(imported_box_name) = self.comp_ctx.resolve_imported_static_box(obj_name) {
            if crate::config::env::builder_static_call_trace() {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug(&format!(
                    "[P287-DEBUG] resolve_static_receiver_box_name: imported alias {} -> {}",
                    obj_name, imported_box_name
                ));
            }
            return Some(imported_box_name.to_string());
        }

        let is_local_var = self
            .function_state
            .variable_ctx
            .variable_map
            .contains_key(obj_name);
        if crate::config::env::builder_static_call_trace() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[P287-DEBUG] resolve_static_receiver_box_name: obj_name={}, is_local_var={}",
                obj_name, is_local_var
            ));
        }
        if is_local_var {
            return None;
        }
        Some(obj_name.clone())
    }
}
