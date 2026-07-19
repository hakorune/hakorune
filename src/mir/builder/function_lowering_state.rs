//! Private FunctionLoweringState vocabulary for the FSESSION0 storage cutover.
//!
//! This module deliberately owns no live `MirBuilder` storage in S0a. Its only
//! job is to freeze the FunctionOwned partition before S0b moves fields out of
//! the mixed Builder contexts. No constructor, accessor, `Deref`, or session
//! lifecycle API belongs here until that physical cutover.

use std::collections::{HashMap, HashSet};

use hakorune_mir_builder::BindingContext;

use crate::ast::{ASTNode, Span};
use crate::mir::builder::compilation_context::RecordLocalValue;
use crate::mir::builder::control_flow::edgecfg::api::FragEmitSession;
use crate::mir::builder::scope_context::LexicalScopeFrame;
use crate::mir::builder::type_context::TypeContext;
use crate::mir::builder::variable_context::VariableContext;
use crate::mir::builder::vars::resolved_binding_state::ResolvedBindingLoweringStateV1;
use crate::mir::instruction::FastMemRegionId;
use crate::mir::{BasicBlockId, MirFunction, ValueId};

/// The future sole physical owner of FunctionOwned Builder state.
///
/// It is intentionally not installed in `MirBuilder` during S0a. `core_ctx`,
/// observation state, module state, and legacy compatibility state stay out of
/// this vocabulary.
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
