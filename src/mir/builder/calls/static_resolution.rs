//! Static method resolution and unresolved-call recovery.
//!
//! Responsibilities:
//! - Static receiver method call resolution (BoxName.method → static method)
//! - Unique static method recovery (undefined function → BoxName.method/Arity)
//! - Dev-only tail resolver (suffix matching with arity)
//!
//! Key functions:
//! - resolve_static_receiver_box_name: classify BoxName.method(args) syntax
//! - try_unique_static_method_recovery: find unique static method by name+arity
//! - try_tail_based_resolver: experimental dev-only suffix resolver

use super::super::{MirBuilder, ValueId};
use super::CallTarget;
use crate::ast::ASTNode;
use crate::mir::builder::callable_declaration_catalog::{
    BareStaticRecoveryDecisionV1, BareStaticRecoveryNoRecoveryReasonV1,
};

#[derive(Debug, PartialEq, Eq)]
pub(super) enum BareStaticRecoveryEmissionV1 {
    Emitted(ValueId),
    NoRecovery(BareStaticRecoveryNoRecoveryReasonV1),
}

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

    /// Try unique static method recovery (name+arity).
    ///
    /// When a function call fails to resolve, attempt to find a unique static method
    /// with matching name and arity in the complete declaration catalog.
    ///
    /// Example: foo(x, y) → BoxName.foo/2 if only one static method matches
    pub(super) fn try_unique_static_method_recovery(
        &mut self,
        name: &str,
        arg_values: &[ValueId],
    ) -> Result<BareStaticRecoveryEmissionV1, String> {
        let decision = {
            let catalog = self
                .comp_ctx
                .callable_declaration_catalog()
                .map_err(|error| error.to_string())?;
            BareStaticRecoveryDecisionV1::decide(catalog, name, arg_values.len())
                .map_err(|error| error.to_string())?
        };

        match decision {
            BareStaticRecoveryDecisionV1::Unique(key) => {
                let dst = self.next_value_id();
                let func_name = key.mir_symbol_projection();
                // Emit unified global call to the lowered static method function
                self.emit_unified_call(
                    Some(dst),
                    CallTarget::Global(func_name),
                    arg_values.to_vec(),
                )?;
                Ok(BareStaticRecoveryEmissionV1::Emitted(dst))
            }
            BareStaticRecoveryDecisionV1::NoRecovery(reason) => {
                Ok(BareStaticRecoveryEmissionV1::NoRecovery(reason))
            }
        }
    }

    /// Try the dev-only tail resolver.
    ///
    /// Experimental: Match function calls by suffix .name/arity in current module.
    /// Requires NYASH_BUILDER_TAIL_RESOLVE=1 to enable.
    ///
    /// Example: foo(x) → SomeBox.foo/1 if only one function ends with ".foo/1"
    pub(super) fn try_tail_based_resolver(
        &mut self,
        name: &str,
        arg_values: &[ValueId],
    ) -> Result<Option<ValueId>, String> {
        if crate::config::env::builder_tail_resolve() {
            if let Some(ref module) = self.current_module {
                let tail = format!(".{}{}", name, format!("/{}", arg_values.len()));
                let mut cands: Vec<String> = module
                    .functions
                    .keys()
                    .filter(|k| k.ends_with(&tail))
                    .cloned()
                    .collect();
                if cands.len() == 1 {
                    let func_name = cands.remove(0);
                    let dst = self.next_value_id();
                    self.emit_legacy_call(
                        Some(dst),
                        CallTarget::Global(func_name),
                        arg_values.to_vec(),
                    )?;
                    return Ok(Some(dst));
                }
            }
        }
        Ok(None)
    }
}
