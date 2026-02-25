//! Static method resolution and fallback logic
//!
//! Responsibilities:
//! - Static receiver method call resolution (BoxName.method → static method)
//! - Static method fallback (undefined function → unique static method)
//! - Tail-based fallback (suffix matching with arity)
//!
//! Key functions:
//! - try_build_static_receiver_method_call: Handle BoxName.method(args) syntax
//! - try_build_static_method_call: Resolve static method calls
//! - try_static_method_fallback: Find unique static method by name+arity
//! - try_tail_based_fallback: Experimental suffix-based resolution

use super::super::{MirBuilder, ValueId};
use super::CallTarget;
use crate::ast::ASTNode;

impl MirBuilder {
    /// Try static receiver method call: BoxName.method(args)
    ///
    /// Phase 287 P4: Fix toString() method resolution bug
    /// Guard: If object is a local variable, don't treat as static box name
    pub(super) fn try_build_static_receiver_method_call(
        &mut self,
        object: &ASTNode,
        method: &str,
        arguments: &[ASTNode],
    ) -> Result<Option<ValueId>, String> {
        let ASTNode::Variable { name: obj_name, .. } = object else {
            return Ok(None);
        };

        // Phase 287 P4: Fix toString() method resolution bug
        // Guard: If this is a local variable, don't treat as static box name
        let is_local_var = self.variable_ctx.variable_map.contains_key(obj_name);

        if crate::config::env::builder_static_call_trace() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!("[P287-DEBUG] try_build_static_receiver_method_call: obj_name={}, method={}, is_local_var={}", obj_name, method, is_local_var));
            ring0.log.debug(&format!("[P287-DEBUG] variable_map keys: {:?}", self.variable_ctx.variable_map.keys().collect::<Vec<_>>()));
        }

        if is_local_var {
            // This is a variable reference (primitive or box instance), not a static box name
            // Let it flow through to handle_standard_method_call (line 147 in build_method_call_impl)
            if crate::config::env::builder_static_call_trace() {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug("[P287-DEBUG] -> Returning None (local var, will use method call)");
            }
            return Ok(None);
        }

        // Only treat as static box method call if obj_name is NOT a local variable
        if crate::config::env::builder_static_call_trace() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug("[P287-DEBUG] -> Calling try_build_static_method_call (not a local var)");
        }
        self.try_build_static_method_call(obj_name, method, arguments)
    }

    /// Try static method call: BoxName.method(args)
    ///
    /// Phase 15.5: Treat unknown identifiers in receiver position as static type names
    fn try_build_static_method_call(
        &mut self,
        obj_name: &str,
        method: &str,
        arguments: &[ASTNode],
    ) -> Result<Option<ValueId>, String> {
        let is_local_var = self.variable_ctx.variable_map.contains_key(obj_name);

        // Debug trace
        if crate::config::env::builder_static_call_trace() {
            let trace = crate::mir::builder::control_flow::joinir::trace::trace();
            trace.stderr_if(
                &format!(
                    "[DEBUG] try_build_static_method_call: obj_name={}, method={}",
                    obj_name, method
                ),
                true,
            );
            trace.stderr_if(&format!("[DEBUG]   is_local_var={}", is_local_var), true);
            if is_local_var {
                trace.stderr_if(
                    &format!(
                        "[DEBUG]   variable_map contains '{}' - treating as local variable, will use method call",
                        obj_name
                    ),
                    true,
                );
                trace.stderr_if(
                    &format!(
                        "[DEBUG]   variable_map keys: {:?}",
                        self.variable_ctx.variable_map.keys().collect::<Vec<_>>()
                    ),
                    true,
                );
            } else {
                trace.stderr_if(
                    &format!(
                        "[DEBUG]   '{}' not in variable_map - treating as static box, will use global call",
                        obj_name
                    ),
                    true,
                );
            }
        }

        // Phase 15.5: Treat unknown identifiers in receiver position as static type names
        if !is_local_var {
            let result = self.handle_static_method_call(obj_name, method, arguments)?;
            return Ok(Some(result));
        }
        Ok(None)
    }

    /// Try static method fallback (name+arity)
    ///
    /// When a function call fails to resolve, attempt to find a unique static method
    /// with matching name and arity in comp_ctx.static_method_index.
    ///
    /// Example: foo(x, y) → BoxName.foo/2 if only one static method matches
    pub(super) fn try_static_method_fallback(
        &mut self,
        name: &str,
        arg_values: &[ValueId],
    ) -> Result<Option<ValueId>, String> {
        if let Some(cands) = self.comp_ctx.static_method_index.get(name) {
            let mut matches: Vec<(String, usize)> = cands
                .iter()
                .cloned()
                .filter(|(_, ar)| *ar == arg_values.len())
                .collect();
            if matches.len() == 1 {
                let (bx, _arity) = matches.remove(0);
                let dst = self.next_value_id();
                let func_name = format!("{}.{}{}", bx, name, format!("/{}", arg_values.len()));
                // Emit unified global call to the lowered static method function
                self.emit_unified_call(
                    Some(dst),
                    CallTarget::Global(func_name),
                    arg_values.to_vec(),
                )?;
                return Ok(Some(dst));
            }
        }
        Ok(None)
    }

    /// Try tail-based fallback (disabled by default)
    ///
    /// Experimental: Match function calls by suffix .name/arity in current module.
    /// Requires NYASH_BUILDER_TAIL_RESOLVE=1 to enable.
    ///
    /// Example: foo(x) → SomeBox.foo/1 if only one function ends with ".foo/1"
    pub(super) fn try_tail_based_fallback(
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
