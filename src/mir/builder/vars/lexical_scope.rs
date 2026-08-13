use crate::mir::{BindingId, LocalSlotId, ValueId};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::panic::{catch_unwind, resume_unwind, AssertUnwindSafe};

/// Atomic snapshot of the current SSA publication and lexical identity views.
pub(in crate::mir::builder) struct LocalBindingStateSnapshot {
    variable_map: BTreeMap<String, ValueId>,
    binding_context: hakorune_mir_builder::BindingContext,
}

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

#[derive(Debug)]
pub(in crate::mir::builder) enum LexicalScopeCloseErrorV1 {
    Unbalanced { function: Box<str>, depth: usize },
    KeepAlive(String),
}

impl fmt::Display for LexicalScopeCloseErrorV1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unbalanced { function, depth } => {
                write!(
                    f,
                    "lexical scope close is unbalanced: fn={function} depth={depth}"
                )
            }
            Self::KeepAlive(error) => write!(f, "lexical scope KeepAlive failed: {error}"),
        }
    }
}

#[derive(Debug)]
pub(in crate::mir::builder) enum LexicalScopeTransactionErrorV1<E> {
    Body(E),
    Close(LexicalScopeCloseErrorV1),
    BodyAndClose {
        body: E,
        close: LexicalScopeCloseErrorV1,
    },
}

#[cfg(test)]
mod test_guard;

#[cfg(test)]
pub(in crate::mir::builder) use test_guard::LexicalScopeGuard;

impl<E: fmt::Display> fmt::Display for LexicalScopeTransactionErrorV1<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Body(error) => write!(f, "lexical scope body failed: {error}"),
            Self::Close(error) => write!(f, "{error}"),
            Self::BodyAndClose { body, close } => {
                write!(
                    f,
                    "lexical scope body failed: {body}; close failed: {close}"
                )
            }
        }
    }
}

/// Run one body while the lexical scope is open, then close it exactly once.
///
/// The callback owns the only mutable builder borrow.  This prevents a scope
/// owner from escaping its builder and lets us close/restore after both normal
/// returns and typed body errors.  A body panic is resumed after restoration;
/// a close error during unwinding is intentionally secondary to that panic.
pub(in crate::mir::builder) fn try_with_lexical_scope<T, E, F>(
    builder: &mut super::super::MirBuilder,
    body: F,
) -> Result<T, LexicalScopeTransactionErrorV1<E>>
where
    F: FnOnce(&mut super::super::MirBuilder) -> Result<T, E>,
{
    builder.push_lexical_scope();
    let body_result = catch_unwind(AssertUnwindSafe(|| body(builder)));
    match body_result {
        Err(payload) => {
            let _ = catch_unwind(AssertUnwindSafe(|| builder.close_lexical_scope()));
            resume_unwind(payload)
        }
        Ok(body_result) => {
            let close_result = catch_unwind(AssertUnwindSafe(|| builder.close_lexical_scope()));
            match close_result {
                Err(payload) => resume_unwind(payload),
                Ok(close_result) => match (body_result, close_result) {
                    (Ok(value), Ok(())) => Ok(value),
                    (Err(body), Ok(())) => Err(LexicalScopeTransactionErrorV1::Body(body)),
                    (Ok(_), Err(close)) => Err(LexicalScopeTransactionErrorV1::Close(close)),
                    (Err(body), Err(close)) => {
                        Err(LexicalScopeTransactionErrorV1::BodyAndClose { body, close })
                    }
                },
            }
        }
    }
}

impl super::super::MirBuilder {
    pub(in crate::mir::builder) fn snapshot_local_binding_state(
        &self,
    ) -> LocalBindingStateSnapshot {
        LocalBindingStateSnapshot {
            variable_map: self.function_state.variable_ctx.variable_map.clone(),
            binding_context: self.function_state.binding_ctx.snapshot(),
        }
    }

    pub(in crate::mir::builder) fn restore_local_binding_state(
        &mut self,
        snapshot: LocalBindingStateSnapshot,
    ) {
        self.function_state.variable_ctx.variable_map = snapshot.variable_map;
        self.function_state
            .binding_ctx
            .restore(snapshot.binding_context);
    }

    fn push_lexical_scope(&mut self) {
        // Phase 2-4: scope_ctx is the lexical-scope stack SSOT.
        self.function_state.scope.push_lexical_scope();
    }

    #[cfg(test)]
    pub(in crate::mir::builder) fn push_lexical_scope_for_test(&mut self) {
        self.push_lexical_scope();
    }

    #[cfg(test)]
    pub(in crate::mir::builder) fn pop_lexical_scope_for_test(&mut self) {
        let _ = self.close_lexical_scope();
    }

    fn close_lexical_scope(&mut self) -> Result<(), LexicalScopeCloseErrorV1> {
        // Phase 2-4: scope_ctx is the lexical-scope stack SSOT.
        let Some(frame) = self.function_state.scope.pop_lexical_scope() else {
            let depth = self.function_state.scope.lexical_scope_stack.len();
            let function = self
                .function_state
                .current_function
                .as_ref()
                .map(|f| f.signature.name.clone().into_boxed_str())
                .unwrap_or_else(|| "<unknown>".into());
            return Err(LexicalScopeCloseErrorV1::Unbalanced { function, depth });
        };

        // Phase 287: Emit KeepAlive for all declared variables in this scope
        // This keeps values alive until scope end for PHI node inputs (liveness analysis)
        // ⚠️ Termination guard: don't emit after return/throw
        let keepalive_result = if !self.is_current_block_terminated() {
            let keepalive_values: Vec<crate::mir::ValueId> = frame
                .declared
                .iter()
                .filter_map(|name| {
                    self.function_state
                        .variable_ctx
                        .variable_map
                        .get(name)
                        .copied()
                })
                .collect();

            if keepalive_values.is_empty() {
                Ok(())
            } else {
                match catch_unwind(AssertUnwindSafe(|| {
                    self.emit_instruction(crate::mir::MirInstruction::KeepAlive {
                        values: keepalive_values,
                    })
                })) {
                    Ok(result) => result.map_err(LexicalScopeCloseErrorV1::KeepAlive),
                    Err(payload) => {
                        self.restore_lexical_scope_frame(frame);
                        resume_unwind(payload)
                    }
                }
            }
        } else {
            Ok(())
        };

        self.restore_lexical_scope_frame(frame);
        keepalive_result
    }

    fn restore_lexical_scope_frame(&mut self, frame: LexicalScopeFrame) {
        // Restore ValueId mappings
        for (name, previous) in frame.restore {
            match previous {
                Some(prev_id) => {
                    self.function_state
                        .variable_ctx
                        .variable_map
                        .insert(name, prev_id);
                }
                None => {
                    self.function_state.variable_ctx.variable_map.remove(&name);
                }
            }
        }

        // Phase 74: Restore BindingId mappings in parallel
        // Phase 2-5: binding_ctx is the binding-id SSOT.
        for (name, previous_binding) in frame.restore_binding {
            match previous_binding {
                Some(prev_bid) => {
                    self.function_state
                        .binding_ctx
                        .insert(name.clone(), prev_bid);
                }
                None => {
                    self.function_state.binding_ctx.remove(&name);
                }
            }
        }
    }

    pub(in crate::mir::builder) fn declare_local_in_current_scope(
        &mut self,
        name: &str,
        value: ValueId,
    ) -> Result<LocalSlotId, String> {
        self.ensure_local_name_available(name)?;
        let binding_id = self.allocate_binding_id()?;
        self.publish_local_binding(name, value, binding_id)
    }

    fn ensure_local_name_available(&self, name: &str) -> Result<(), String> {
        let func_name = self
            .function_state
            .current_function
            .as_ref()
            .map(|f| f.signature.name.clone())
            .unwrap_or_else(|| "<unknown>".to_string());
        // Phase 2-4: Use scope_ctx (SSOT)
        let Some(frame) = self.function_state.scope.lexical_scope_stack.last() else {
            return Err("COMPILER BUG: local declaration outside lexical scope".to_string());
        };
        if frame.declared.contains(name) {
            return Err(format!(
                "[freeze:contract][local/redeclare_same_scope] fn={} name={}",
                func_name, name
            ));
        }
        Ok(())
    }

    fn publish_local_binding(
        &mut self,
        name: &str,
        value: ValueId,
        binding_id: BindingId,
    ) -> Result<LocalSlotId, String> {
        let frame = self
            .function_state
            .scope
            .current_scope_mut()
            .expect("local availability check requires current scope");
        assert!(frame.declared.insert(name.to_string()));
        // Capture previous ValueId for restoration
        let previous = self
            .function_state
            .variable_ctx
            .variable_map
            .get(name)
            .copied();
        frame.restore.insert(name.to_string(), previous);

        // Phase 74: Capture previous BindingId for parallel restoration
        // Phase 136 Step 4/7: Use binding_ctx for lookup
        let previous_binding = self.function_state.binding_ctx.lookup(name);
        frame
            .restore_binding
            .insert(name.to_string(), previous_binding);

        // Update both ValueId and BindingId mappings
        self.function_state
            .variable_ctx
            .variable_map
            .insert(name.to_string(), value);

        // Phase 2-5: binding_ctx is the binding-id SSOT.
        self.function_state
            .binding_ctx
            .insert(name.to_string(), binding_id);

        Ok(LocalSlotId::from(binding_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::MirBuilder;

    fn scope_depth(builder: &MirBuilder) -> usize {
        builder.function_state.scope.lexical_scope_stack.len()
    }

    #[test]
    fn declaration_and_shadowing_use_one_binding_allocator() {
        let mut builder = MirBuilder::new();
        builder.push_lexical_scope();
        let outer = builder
            .declare_local_in_current_scope("x", ValueId::new(1))
            .expect("outer declaration");

        builder.push_lexical_scope();
        let inner = builder
            .declare_local_in_current_scope("x", ValueId::new(2))
            .expect("inner declaration");
        assert_ne!(inner, outer);
        assert_eq!(
            builder.function_state.binding_ctx.lookup("x"),
            Some(inner.binding_id())
        );

        builder.pop_lexical_scope_for_test();
        assert_eq!(
            builder.function_state.binding_ctx.lookup("x"),
            Some(outer.binding_id())
        );
        assert_eq!(
            builder.function_state.variable_ctx.variable_map.get("x"),
            Some(&ValueId::new(1))
        );
        builder.pop_lexical_scope_for_test();
    }

    #[test]
    fn local_binding_snapshot_restores_values_and_identity_together() {
        let mut builder = MirBuilder::new();
        builder.push_lexical_scope();
        let slot = builder
            .declare_local_in_current_scope("x", ValueId::new(1))
            .expect("declaration");
        let snapshot = builder.snapshot_local_binding_state();

        builder
            .function_state
            .variable_ctx
            .variable_map
            .insert("x".to_string(), ValueId::new(2));
        builder.function_state.binding_ctx.remove("x");
        builder.restore_local_binding_state(snapshot);

        assert_eq!(
            builder.function_state.variable_ctx.variable_map.get("x"),
            Some(&ValueId::new(1))
        );
        assert_eq!(
            builder.function_state.binding_ctx.lookup("x"),
            Some(slot.binding_id())
        );
        builder.pop_lexical_scope_for_test();
    }

    #[test]
    fn scoped_transaction_restores_after_success_and_emits_keepalive() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("lexical_scope_success/0".to_owned());

        let result = try_with_lexical_scope(&mut builder, |builder| -> Result<ValueId, String> {
            builder.declare_local_in_current_scope("x", ValueId::new(7))?;
            Ok(ValueId::new(7))
        });

        assert_eq!(result.expect("scope success"), ValueId::new(7));
        assert_eq!(scope_depth(&builder), 0);
        assert!(!builder.function_state.binding_ctx.contains("x"));
        assert!(!builder
            .function_state
            .variable_ctx
            .variable_map
            .contains_key("x"));
        let block = builder
            .function_state
            .current_block
            .expect("function test harness has a current block");
        let keepalives = builder
            .function_state
            .current_function
            .as_ref()
            .and_then(|function| function.blocks.get(&block))
            .map(|block| {
                block
                    .instructions
                    .iter()
                    .filter(|instruction| {
                        matches!(instruction, crate::mir::MirInstruction::KeepAlive { .. })
                    })
                    .count()
            })
            .unwrap_or(0);
        assert_eq!(keepalives, 1);
    }

    #[test]
    fn scoped_transaction_returns_body_error_and_restores() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("lexical_scope_body_error/0".to_owned());
        let result = try_with_lexical_scope(&mut builder, |builder| -> Result<(), String> {
            builder.declare_local_in_current_scope("x", ValueId::new(8))?;
            Err("body failure".to_owned())
        });

        assert!(matches!(
            result,
            Err(LexicalScopeTransactionErrorV1::Body(error)) if error == "body failure"
        ));
        assert_eq!(scope_depth(&builder), 0);
        assert!(!builder.function_state.binding_ctx.contains("x"));
        assert!(!builder
            .function_state
            .variable_ctx
            .variable_map
            .contains_key("x"));
    }

    #[test]
    fn scoped_transaction_surfaces_keepalive_failure_after_restoration() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("lexical_scope_close_failure/0".to_owned());

        let result = try_with_lexical_scope(&mut builder, |builder| -> Result<(), String> {
            builder.declare_local_in_current_scope("x", ValueId::new(9))?;
            builder.function_state.current_block = None;
            Ok(())
        });

        assert!(matches!(
            result,
            Err(LexicalScopeTransactionErrorV1::Close(
                LexicalScopeCloseErrorV1::KeepAlive(_)
            ))
        ));
        assert_eq!(scope_depth(&builder), 0);
        assert!(!builder.function_state.binding_ctx.contains("x"));
        assert!(!builder
            .function_state
            .variable_ctx
            .variable_map
            .contains_key("x"));
    }

    #[test]
    fn scoped_transaction_restores_before_resuming_body_panic() {
        let mut builder = MirBuilder::new();
        let panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _: Result<(), LexicalScopeTransactionErrorV1<String>> =
                try_with_lexical_scope(&mut builder, |builder| -> Result<(), String> {
                    builder.declare_local_in_current_scope("x", ValueId::new(10))?;
                    panic!("body panic");
                });
        }));

        assert!(panic.is_err());
        assert_eq!(scope_depth(&builder), 0);
        assert!(!builder.function_state.binding_ctx.contains("x"));
        assert!(!builder
            .function_state
            .variable_ctx
            .variable_map
            .contains_key("x"));
    }
}
