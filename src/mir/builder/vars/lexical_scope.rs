use crate::mir::{BindingId, ValueId};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Default, Clone)]
pub(in crate::mir::builder) struct LexicalScopeFrame {
    pub(in crate::mir::builder) declared: BTreeSet<String>,
    pub(in crate::mir::builder) restore: BTreeMap<String, Option<ValueId>>,
    /// Phase 74: Parallel BindingId restoration on scope exit
    pub(in crate::mir::builder) restore_binding: BTreeMap<String, Option<BindingId>>,
}

impl LexicalScopeFrame {
    #[allow(dead_code)]
    fn new() -> Self {
        Self::default()
    }
}

pub(in crate::mir::builder) struct LexicalScopeGuard {
    builder: *mut super::super::MirBuilder,
}

impl LexicalScopeGuard {
    pub(in crate::mir::builder) fn new(builder: &mut super::super::MirBuilder) -> Self {
        builder.push_lexical_scope();
        Self { builder }
    }
}

impl Drop for LexicalScopeGuard {
    fn drop(&mut self) {
        // Safety: LexicalScopeGuard is created from a unique `&mut MirBuilder` and its lifetime
        // is bounded by the surrounding lexical scope. Drop runs at most once.
        unsafe { &mut *self.builder }.pop_lexical_scope();
    }
}

impl super::super::MirBuilder {
    pub(in crate::mir::builder) fn push_lexical_scope(&mut self) {
        // Phase 2-4: scope_ctx is the lexical-scope stack SSOT.
        self.scope_ctx.push_lexical_scope();
    }

    pub(in crate::mir::builder) fn pop_lexical_scope(&mut self) {
        // Phase 2-4: scope_ctx is the lexical-scope stack SSOT.
        let frame = match self.scope_ctx.pop_lexical_scope() {
            Some(f) => f,
            None => {
                // Fail-fast with freeze tag (strict/dev+planner_required mode)
                let depth = self.scope_ctx.lexical_scope_stack.len();
                let func_name = self
                    .scope_ctx
                    .current_function
                    .as_ref()
                    .map(|f| f.signature.name.as_str())
                    .unwrap_or("<unknown>");
                panic!(
                    "[freeze:contract][lexical_scope/unbalanced_pop] fn={} depth={} action=pop",
                    func_name, depth
                );
            }
        };

        // Phase 287: Emit KeepAlive for all declared variables in this scope
        // This keeps values alive until scope end for PHI node inputs (liveness analysis)
        // ⚠️ Termination guard: don't emit after return/throw
        if !self.is_current_block_terminated() {
            let keepalive_values: Vec<crate::mir::ValueId> = frame
                .declared
                .iter()
                .filter_map(|name| self.variable_ctx.variable_map.get(name).copied())
                .collect();

            if !keepalive_values.is_empty() {
                let _ = self.emit_instruction(crate::mir::MirInstruction::KeepAlive {
                    values: keepalive_values,
                });
            }
        }

        // Restore ValueId mappings
        for (name, previous) in frame.restore {
            match previous {
                Some(prev_id) => {
                    self.variable_ctx.variable_map.insert(name, prev_id);
                }
                None => {
                    self.variable_ctx.variable_map.remove(&name);
                }
            }
        }

        // Phase 74: Restore BindingId mappings in parallel
        // Phase 2-5: binding_ctx is the binding-id SSOT.
        for (name, previous_binding) in frame.restore_binding {
            match previous_binding {
                Some(prev_bid) => {
                    self.binding_ctx.insert(name.clone(), prev_bid);
                }
                None => {
                    self.binding_ctx.remove(&name);
                }
            }
        }
    }

    pub(in crate::mir::builder) fn declare_local_in_current_scope(
        &mut self,
        name: &str,
        value: ValueId,
    ) -> Result<(), String> {
        let func_name = self
            .scope_ctx
            .current_function
            .as_ref()
            .map(|f| f.signature.name.clone())
            .unwrap_or_else(|| "<unknown>".to_string());
        // Phase 2-4: Use scope_ctx (SSOT)
        let Some(frame) = self.scope_ctx.current_scope_mut() else {
            return Err("COMPILER BUG: local declaration outside lexical scope".to_string());
        };

        if !frame.declared.insert(name.to_string()) {
            return Err(format!(
                "[freeze:contract][local/redeclare_same_scope] fn={} name={}",
                func_name, name
            ));
        }
        // Capture previous ValueId for restoration
        let previous = self.variable_ctx.variable_map.get(name).copied();
        frame.restore.insert(name.to_string(), previous);

        // Phase 74: Capture previous BindingId for parallel restoration
        // Phase 136 Step 4/7: Use binding_ctx for lookup
        let previous_binding = self.binding_ctx.lookup(name);
        frame
            .restore_binding
            .insert(name.to_string(), previous_binding);

        // Update both ValueId and BindingId mappings
        self.variable_ctx
            .variable_map
            .insert(name.to_string(), value);

        // Phase 74: Allocate and register new BindingId for this binding
        let binding_id = self.allocate_binding_id();
        // Phase 2-5: binding_ctx is the binding-id SSOT.
        self.binding_ctx.insert(name.to_string(), binding_id);

        Ok(())
    }
}
