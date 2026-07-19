//! Private FunctionOwned storage for the FSESSION0 physical cutover.
//!
//! `MirBuilder` owns exactly one `FunctionLoweringStateV1`. This module exposes
//! neither a compatibility facade nor a fresh-session API: S0b only moves
//! storage, while S0c owns lifecycle transaction consolidation.

use std::collections::{HashMap, HashSet};

use hakorune_mir_builder::BindingContext;

use crate::ast::{ASTNode, Span};
use crate::mir::builder::compilation_context::RecordLocalValue;
use crate::mir::builder::control_flow::edgecfg::api::FragEmitSession;
use crate::mir::builder::type_context::TypeContext;
use crate::mir::builder::variable_context::VariableContext;
use crate::mir::builder::vars::lexical_scope::LexicalScopeFrame;
use crate::mir::builder::vars::resolved_binding_state::ResolvedBindingLoweringStateV1;
use crate::mir::instruction::FastMemRegionId;
use crate::mir::{BasicBlockId, MirFunction, ValueId};

/// The sole physical owner of FunctionOwned Builder state.
///
/// `core_ctx`, observation state, module state, and legacy compatibility state
/// intentionally stay outside this component.
#[derive(Debug, Default)]
pub(super) struct FunctionLoweringStateV1 {
    pub(super) current_function: Option<MirFunction>,
    pub(super) current_block: Option<BasicBlockId>,
    pub(super) variable_ctx: VariableContext,
    pub(super) type_ctx: TypeContext,
    pub(super) binding_ctx: BindingContext,
    pub(super) resolved_binding_state: ResolvedBindingLoweringStateV1,
    pub(super) scope: FunctionScopeStateV1,
    pub(super) compilation: FunctionCompilationScratchV1,
    pub(super) value_origins: FunctionValueOriginFactsV1,
    pub(super) pending_phis: Vec<(BasicBlockId, ValueId, String)>,
    pub(super) local_ssa_map: HashMap<(BasicBlockId, ValueId, u8), ValueId>,
    pub(super) schedule_mat_map: HashMap<(BasicBlockId, ValueId), ValueId>,
    pub(super) pin_slot_names: HashMap<ValueId, String>,
    pub(super) frag_emit_session: FragEmitSession,
    pub(super) return_defer_active: bool,
    pub(super) return_defer_slot: Option<ValueId>,
    pub(super) return_defer_target: Option<BasicBlockId>,
    pub(super) return_deferred_emitted: bool,
    pub(super) in_cleanup_block: bool,
    pub(super) cleanup_allow_return: bool,
    pub(super) cleanup_allow_throw: bool,
    pub(super) suppress_pin_entry_copy_next: bool,
    pub(super) in_unified_boxcall_fallback: bool,
}

/// FunctionOwned leaves split from the mixed `ScopeContext`.
///
/// `debug_scope_stack` remains an ObservationBorrow and is intentionally absent.
#[derive(Debug, Default)]
pub(super) struct FunctionScopeStateV1 {
    pub(super) lexical_scope_stack: Vec<LexicalScopeFrame>,
    pub(super) loop_header_stack: Vec<BasicBlockId>,
    pub(super) loop_exit_stack: Vec<BasicBlockId>,
    pub(super) if_merge_stack: Vec<BasicBlockId>,
    pub(super) function_param_names: HashSet<String>,
    pub(super) fastmem_region_stack: Vec<FastMemRegionId>,
}

impl FunctionScopeStateV1 {
    #[inline]
    pub(super) fn push_lexical_scope(&mut self) {
        self.lexical_scope_stack.push(LexicalScopeFrame::default());
    }

    #[inline]
    pub(super) fn pop_lexical_scope(&mut self) -> Option<LexicalScopeFrame> {
        self.lexical_scope_stack.pop()
    }

    #[inline]
    pub(super) fn current_scope_mut(&mut self) -> Option<&mut LexicalScopeFrame> {
        self.lexical_scope_stack.last_mut()
    }

    #[inline]
    pub(super) fn push_fastmem_region(&mut self, region: FastMemRegionId) {
        self.fastmem_region_stack.push(region);
    }

    #[inline]
    pub(super) fn pop_fastmem_region(&mut self) -> Option<FastMemRegionId> {
        self.fastmem_region_stack.pop()
    }

    #[inline]
    pub(super) fn current_fastmem_region(&self) -> Option<FastMemRegionId> {
        self.fastmem_region_stack.last().copied()
    }

    pub(super) fn clear_for_function_entry(&mut self) {
        self.lexical_scope_stack.clear();
        self.loop_header_stack.clear();
        self.loop_exit_stack.clear();
        self.if_merge_stack.clear();
        self.fastmem_region_stack.clear();
    }
}

impl FunctionCompilationScratchV1 {
    #[inline]
    pub(super) fn is_reserved_value_id(&self, id: ValueId) -> bool {
        self.reserved_value_ids.contains(&id)
    }

    #[inline]
    pub(super) fn reserve_value_id(&mut self, id: ValueId) {
        self.reserved_value_ids.insert(id);
    }

    pub(super) fn register_record_local_value(
        &mut self,
        value: ValueId,
        record_name: String,
        fields: Vec<crate::mir::builder::compilation_context::RecordLocalFieldValue>,
    ) {
        self.record_local_values.insert(
            value,
            RecordLocalValue {
                record_name,
                fields,
            },
        );
    }

    #[inline]
    pub(super) fn record_local_value(&self, value: ValueId) -> Option<&RecordLocalValue> {
        self.record_local_values.get(&value)
    }

    pub(super) fn propagate_record_local_value(&mut self, src: ValueId, dst: ValueId) {
        if let Some(record) = self.record_local_values.get(&src).cloned() {
            self.record_local_values.insert(dst, record);
        }
    }

    pub(super) fn propagate_record_local_value_from_phi(
        &mut self,
        inputs: &[(BasicBlockId, ValueId)],
        dst: ValueId,
    ) {
        let mut records = inputs
            .iter()
            .filter_map(|(_, value)| self.record_local_values.get(value));
        let Some(first) = records.next().cloned() else {
            return;
        };
        if records.all(|record| {
            record.record_name == first.record_name
                && record.fields.len() == first.fields.len()
                && record.fields.iter().zip(first.fields.iter()).all(|(a, b)| {
                    a.name == b.name
                        && a.declared_type_name == b.declared_type_name
                        && a.value == b.value
                })
        }) {
            self.record_local_values.insert(dst, first);
        }
    }

    #[inline]
    pub(super) fn clear_record_local_values(&mut self) {
        self.record_local_values.clear();
    }
}

/// FunctionOwned leaves split from the mixed `CompilationContext`.
#[derive(Debug, Default)]
pub(super) struct FunctionCompilationScratchV1 {
    pub(super) reserved_value_ids: HashSet<ValueId>,
    pub(super) fn_body_ast: Option<Vec<ASTNode>>,
    pub(super) record_local_values: HashMap<ValueId, RecordLocalValue>,
}

/// ValueId-keyed metadata facts currently missing from the legacy snapshot.
///
/// `value_origin_newbox` stays in `TypeContext`; it must not be duplicated here.
#[derive(Debug, Default)]
pub(super) struct FunctionValueOriginFactsV1 {
    pub(super) value_origin_spans: HashMap<ValueId, Span>,
    pub(super) value_origin_callers: HashMap<ValueId, String>,
}

impl FunctionValueOriginFactsV1 {
    #[inline]
    pub(super) fn record_span(&mut self, value: ValueId, span: Span) {
        self.value_origin_spans.insert(value, span);
    }

    #[inline]
    pub(super) fn span(&self, value: ValueId) -> Option<Span> {
        self.value_origin_spans.get(&value).copied()
    }

    #[inline]
    pub(super) fn record_caller(
        &mut self,
        value: ValueId,
        caller: &'static std::panic::Location<'static>,
    ) {
        self.value_origin_callers.insert(
            value,
            format!("{}:{}:{}", caller.file(), caller.line(), caller.column()),
        );
    }

    #[inline]
    pub(super) fn caller(&self, value: ValueId) -> Option<&str> {
        self.value_origin_callers.get(&value).map(String::as_str)
    }

    pub(super) fn caller_rows(&self) -> Vec<(ValueId, String)> {
        self.value_origin_callers
            .iter()
            .map(|(value, caller)| (*value, caller.clone()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vocabulary_defaults_to_an_unpopulated_function_state() {
        let state = FunctionLoweringStateV1::default();

        assert!(state.current_function.is_none());
        assert!(state.current_block.is_none());
        assert!(state.variable_ctx.variable_map.is_empty());
        assert!(state.type_ctx.value_types.is_empty());
        assert!(state.binding_ctx.is_empty());
        assert!(state.scope.lexical_scope_stack.is_empty());
        assert!(state.compilation.reserved_value_ids.is_empty());
        assert!(state.value_origins.value_origin_spans.is_empty());
        assert!(state.pending_phis.is_empty());
        assert!(!state.in_unified_boxcall_fallback);
    }
}
