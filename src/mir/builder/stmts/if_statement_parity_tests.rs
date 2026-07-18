//! IF0-P0: exact parity against the retired pre-I0 statement-If surface.
//!
//! This module is the sole test-only reference owner. It deliberately repeats
//! the retired orchestration instead of calling the selected If driver. The
//! existing IfForm, expression, FastMem verifier/fact, and Void emitters remain
//! shared semantic owners.

use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::builder::vars::lexical_scope::LexicalScopeGuard;
use crate::mir::exact_numeric_value_facts::{ExactNumericConstFact, ExactNumericValueFact};
use crate::mir::function::{
    FastMemBranchConditionFact, FastMemBranchConditionProofKind, FastMemRegionMetadata,
    FastMemRegionOrigin,
};
use crate::mir::instruction::FastMemRegionId;
use crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout;
use crate::mir::value_kind::MirValueKind;
use crate::mir::{
    BasicBlockId, BindingId, EffectMask, MirBuilder, MirInstruction, MirType, ValueId,
};

#[derive(Debug, PartialEq)]
struct BlockSnapshotV1 {
    id: BasicBlockId,
    instructions: Vec<MirInstruction>,
    instruction_spans: Vec<Span>,
    terminator: Option<MirInstruction>,
    terminator_span: Option<Span>,
    predecessors: Vec<BasicBlockId>,
    successors: Vec<BasicBlockId>,
    effects: EffectMask,
    reachable: bool,
    sealed: bool,
    return_env: Option<Vec<ValueId>>,
    return_env_layout: Option<JumpArgsLayout>,
}

#[derive(Debug, PartialEq)]
struct ScopeFrameSnapshotV1 {
    declared: Vec<String>,
    restore: Vec<(String, Option<ValueId>)>,
    restore_binding: Vec<(String, Option<BindingId>)>,
}

#[derive(Debug, PartialEq)]
struct IfStatementParitySnapshotV1 {
    result: Result<ValueId, String>,
    blocks: Vec<BlockSnapshotV1>,
    locals: Vec<MirType>,
    value_types: Vec<(ValueId, MirType)>,
    value_kinds: Vec<(ValueId, MirValueKind)>,
    value_origins: Vec<(ValueId, String)>,
    string_literals: Vec<(ValueId, String)>,
    exact_numeric_const_facts: Vec<(ValueId, ExactNumericConstFact)>,
    exact_numeric_value_facts: Vec<(ValueId, ExactNumericValueFact)>,
    variable_map: Vec<(String, ValueId)>,
    bindings: Vec<(String, Option<BindingId>)>,
    scope_frames: Vec<ScopeFrameSnapshotV1>,
    loop_header_stack: Vec<BasicBlockId>,
    loop_exit_stack: Vec<BasicBlockId>,
    if_merge_stack: Vec<BasicBlockId>,
    debug_scope_stack: Vec<String>,
    fastmem_region_stack: Vec<FastMemRegionId>,
    fastmem_regions: Vec<FastMemRegionMetadata>,
    fastmem_branch_condition_facts: Vec<FastMemBranchConditionFact>,
    pending_phis: Vec<(BasicBlockId, ValueId, String)>,
    pin_slots: Vec<(ValueId, String)>,
    local_ssa_map: Vec<((BasicBlockId, ValueId, u8), ValueId)>,
    schedule_mat_map: Vec<((BasicBlockId, ValueId), ValueId)>,
    current_block: Option<BasicBlockId>,
    next_value_id: u32,
    next_core_value: ValueId,
    next_core_block: BasicBlockId,
    next_binding_id: u32,
    temp_slot_counter: u32,
    debug_join_counter: u32,
    recursion_depth: usize,
    current_span: Span,
    in_cleanup_block: bool,
    cleanup_allow_return: bool,
    return_defer_active: bool,
    return_defer_slot: Option<ValueId>,
    return_defer_target: Option<BasicBlockId>,
    return_deferred_emitted: bool,
}

fn int_at(value: i64, span: Span) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span,
    }
}

fn int(value: i64) -> ASTNode {
    int_at(value, Span::unknown())
}

fn boolean(value: bool) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Bool(value),
        span: Span::unknown(),
    }
}

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_string(),
        span: Span::unknown(),
    }
}

fn local(name: &str, value: i64) -> ASTNode {
    ASTNode::Local {
        variables: vec![name.to_string()],
        initial_values: vec![Some(Box::new(int(value)))],
        declared_type_names: vec![None],
        span: Span::unknown(),
    }
}

fn assignment(name: &str, value: i64) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(variable(name)),
        value: Box::new(int(value)),
        span: Span::unknown(),
    }
}

fn return_int(value: i64) -> ASTNode {
    ASTNode::Return {
        value: Some(Box::new(int(value))),
        span: Span::unknown(),
    }
}

fn statement_if(
    condition: ASTNode,
    then_body: Vec<ASTNode>,
    else_body: Option<Vec<ASTNode>>,
) -> ASTNode {
    statement_if_at(condition, then_body, else_body, Span::unknown())
}

fn statement_if_at(
    condition: ASTNode,
    then_body: Vec<ASTNode>,
    else_body: Option<Vec<ASTNode>>,
    span: Span,
) -> ASTNode {
    ASTNode::If {
        condition: Box::new(condition),
        then_body,
        else_body,
        span,
    }
}

fn type_check(receiver: ASTNode) -> ASTNode {
    ASTNode::MethodCall {
        object: Box::new(receiver),
        method: "is".to_string(),
        arguments: vec![ASTNode::Literal {
            value: LiteralValue::String("Integer".to_string()),
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    }
}

fn owner_eq_condition() -> ASTNode {
    ASTNode::FunctionCall {
        name: "mem.ownerEq".to_string(),
        arguments: vec![int(1), int(1)],
        span: Span::unknown(),
    }
}

fn builder(name: &str) -> MirBuilder {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test(name.to_string());
    builder
}

fn legacy_branch_program(statements: Vec<ASTNode>) -> ASTNode {
    ASTNode::Program {
        statements,
        span: Span::unknown(),
    }
}

fn lower_selected(builder: &mut MirBuilder, statement: ASTNode) -> Result<ValueId, String> {
    super::block_stmt::build_statement(builder, statement)
}

fn lower_pre_i0_statement_if_reference(
    builder: &mut MirBuilder,
    condition: ASTNode,
    then_body: Vec<ASTNode>,
    else_body: Option<Vec<ASTNode>>,
) -> Result<(), String> {
    if let Some(region) = builder.current_fastmem_region() {
        let condition_value = builder.build_expression(condition.clone())?;
        crate::mir::builder::fastmem::branch::ensure_fastmem_owner_eq_condition(
            builder,
            region,
            condition_value,
        )?;
        builder.add_fastmem_branch_condition_fact(
            region,
            condition_value,
            FastMemBranchConditionProofKind::SourceAssumeOwnerEq,
            true,
        )?;
        debug_assert_eq!(builder.current_fastmem_region(), Some(region));

        builder.lower_if_form_with_condition_value(
            condition_value,
            Some(condition),
            legacy_branch_program(then_body),
            else_body.map(legacy_branch_program),
        )?;
        return Ok(());
    }

    builder.cf_if(
        condition,
        legacy_branch_program(then_body),
        else_body.map(legacy_branch_program),
    )?;
    Ok(())
}

fn lower_pre_i0_statement_surface_reference(
    builder: &mut MirBuilder,
    statement: ASTNode,
) -> Result<ValueId, String> {
    builder.metadata_ctx.set_current_span(statement.span());
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = statement
    else {
        return Err("IF0-P0 reference requires statement If".to_string());
    };
    lower_pre_i0_statement_if_reference(builder, *condition, then_body, else_body)?;
    crate::mir::builder::emission::constant::emit_void(builder)
}

fn sorted_map<K: Ord, V>(iter: impl Iterator<Item = (K, V)>) -> Vec<(K, V)> {
    let mut rows = iter.collect::<Vec<_>>();
    rows.sort_by(|left, right| left.0.cmp(&right.0));
    rows
}

fn snapshot(builder: &MirBuilder, result: Result<ValueId, String>) -> IfStatementParitySnapshotV1 {
    let function = builder
        .scope_ctx
        .current_function
        .as_ref()
        .expect("current IF0-P0 function");
    let mut blocks = function
        .blocks
        .values()
        .map(|block| BlockSnapshotV1 {
            id: block.id,
            instructions: block.instructions.clone(),
            instruction_spans: block.instruction_spans.clone(),
            terminator: block.terminator.clone(),
            terminator_span: block.terminator_span,
            predecessors: block.predecessors.iter().copied().collect(),
            successors: block.successors.iter().copied().collect(),
            effects: block.effects,
            reachable: block.reachable,
            sealed: block.sealed,
            return_env: block.return_env.clone(),
            return_env_layout: block.return_env_layout,
        })
        .collect::<Vec<_>>();
    blocks.sort_by_key(|block| block.id);

    let variable_map = sorted_map(
        builder
            .variable_ctx
            .variable_map
            .iter()
            .map(|(name, value)| (name.clone(), *value)),
    );
    let bindings = variable_map
        .iter()
        .map(|(name, _)| (name.clone(), builder.binding_ctx.lookup(name)))
        .collect();
    let scope_frames = builder
        .scope_ctx
        .lexical_scope_stack
        .iter()
        .map(|frame| ScopeFrameSnapshotV1 {
            declared: frame.declared.iter().cloned().collect(),
            restore: frame
                .restore
                .iter()
                .map(|(name, value)| (name.clone(), *value))
                .collect(),
            restore_binding: frame
                .restore_binding
                .iter()
                .map(|(name, binding)| (name.clone(), *binding))
                .collect(),
        })
        .collect();

    IfStatementParitySnapshotV1 {
        result,
        blocks,
        locals: function.locals.clone(),
        value_types: sorted_map(
            builder
                .type_ctx
                .value_types
                .iter()
                .map(|(value, ty)| (*value, ty.clone())),
        ),
        value_kinds: sorted_map(
            builder
                .type_ctx
                .value_kinds
                .iter()
                .map(|(value, kind)| (*value, *kind)),
        ),
        value_origins: sorted_map(
            builder
                .type_ctx
                .value_origin_newbox
                .iter()
                .map(|(value, owner)| (*value, owner.clone())),
        ),
        string_literals: sorted_map(
            builder
                .type_ctx
                .string_literals
                .iter()
                .map(|(value, text)| (*value, text.clone())),
        ),
        exact_numeric_const_facts: sorted_map(
            function
                .metadata
                .exact_numeric_const_facts
                .iter()
                .map(|(value, fact)| (*value, fact.clone())),
        ),
        exact_numeric_value_facts: sorted_map(
            function
                .metadata
                .exact_numeric_value_facts
                .iter()
                .map(|(value, fact)| (*value, fact.clone())),
        ),
        variable_map,
        bindings,
        scope_frames,
        loop_header_stack: builder.scope_ctx.loop_header_stack.clone(),
        loop_exit_stack: builder.scope_ctx.loop_exit_stack.clone(),
        if_merge_stack: builder.scope_ctx.if_merge_stack.clone(),
        debug_scope_stack: builder.scope_ctx.debug_scope_stack.clone(),
        fastmem_region_stack: builder.scope_ctx.fastmem_region_stack.clone(),
        fastmem_regions: function.metadata.fastmem_regions.clone(),
        fastmem_branch_condition_facts: function.metadata.fastmem_branch_condition_facts.clone(),
        pending_phis: builder.pending_phis.clone(),
        pin_slots: sorted_map(
            builder
                .pin_slot_names
                .iter()
                .map(|(value, name)| (*value, name.clone())),
        ),
        local_ssa_map: sorted_map(
            builder
                .local_ssa_map
                .iter()
                .map(|(key, value)| (*key, *value)),
        ),
        schedule_mat_map: sorted_map(
            builder
                .schedule_mat_map
                .iter()
                .map(|(key, value)| (*key, *value)),
        ),
        current_block: builder.current_block,
        next_value_id: function.next_value_id,
        next_core_value: builder.core_ctx.peek_next_value(),
        next_core_block: builder.core_ctx.peek_next_block(),
        next_binding_id: builder.core_ctx.next_binding_id,
        temp_slot_counter: builder.core_ctx.temp_slot_counter,
        debug_join_counter: builder.core_ctx.debug_join_counter,
        recursion_depth: builder.recursion_depth,
        current_span: builder.metadata_ctx.current_span(),
        in_cleanup_block: builder.in_cleanup_block,
        cleanup_allow_return: builder.cleanup_allow_return,
        return_defer_active: builder.return_defer_active,
        return_defer_slot: builder.return_defer_slot,
        return_defer_target: builder.return_defer_target,
        return_deferred_emitted: builder.return_deferred_emitted,
    }
}

fn assert_parity(statement: ASTNode) {
    let mut selected = builder("if_statement_parity/0");
    let mut reference = builder("if_statement_parity/0");
    let selected_result = lower_selected(&mut selected, statement.clone());
    let reference_result = lower_pre_i0_statement_surface_reference(&mut reference, statement);
    assert_eq!(
        snapshot(&selected, selected_result),
        snapshot(&reference, reference_result)
    );
}

fn assert_parity_and_reuse(failure: ASTNode) {
    let mut selected = builder("if_statement_failure_reuse/0");
    let mut reference = builder("if_statement_failure_reuse/0");
    let selected_result = lower_selected(&mut selected, failure.clone());
    let reference_result = lower_pre_i0_statement_surface_reference(&mut reference, failure);
    assert!(selected_result.is_err());
    assert_eq!(
        snapshot(&selected, selected_result),
        snapshot(&reference, reference_result)
    );

    let recovery = statement_if(boolean(true), vec![int(1)], Some(vec![int(2)]));
    let selected_result = lower_selected(&mut selected, recovery.clone());
    let reference_result = lower_pre_i0_statement_surface_reference(&mut reference, recovery);
    assert!(selected_result.is_ok());
    assert!(reference_result.is_ok());
    assert_eq!(
        snapshot(&selected, selected_result),
        snapshot(&reference, reference_result)
    );
}

#[test]
fn if_statement_parity_explicit_else_phis_and_child_expression_families() {
    for condition in [
        ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(int(1)),
            right: Box::new(int(2)),
            span: Span::unknown(),
        },
        ASTNode::BinaryOp {
            operator: BinaryOperator::And,
            left: Box::new(boolean(true)),
            right: Box::new(boolean(false)),
            span: Span::unknown(),
        },
        type_check(int(1)),
    ] {
        let mut selected = builder("if_statement_phis/0");
        let mut reference = builder("if_statement_phis/0");
        let _selected_scope = LexicalScopeGuard::new(&mut selected);
        let _reference_scope = LexicalScopeGuard::new(&mut reference);
        lower_selected(&mut selected, local("x", 0)).unwrap();
        lower_selected(&mut reference, local("x", 0)).unwrap();
        let statement = statement_if(
            condition,
            vec![assignment("x", 1)],
            Some(vec![assignment("x", 2)]),
        );
        let selected_result = lower_selected(&mut selected, statement.clone());
        let reference_result = lower_pre_i0_statement_surface_reference(&mut reference, statement);
        assert_eq!(
            snapshot(&selected, selected_result),
            snapshot(&reference, reference_result)
        );
    }
}

#[test]
fn if_statement_parity_implicit_else_and_termination_matrix() {
    assert_parity(statement_if(boolean(true), vec![int(1)], None));
    for (then_returns, else_returns) in [(false, false), (true, false), (false, true), (true, true)]
    {
        let then_body = if then_returns {
            vec![return_int(1)]
        } else {
            vec![int(1)]
        };
        let else_body = if else_returns {
            vec![return_int(2)]
        } else {
            vec![int(2)]
        };
        assert_parity(statement_if(boolean(true), then_body, Some(else_body)));
    }
}

#[test]
fn if_statement_parity_condition_then_else_failures_and_reuse() {
    assert_parity_and_reuse(statement_if(
        variable("missing_condition"),
        vec![int(1)],
        None,
    ));
    assert_parity_and_reuse(statement_if(
        boolean(true),
        vec![variable("missing_then")],
        Some(vec![int(2)]),
    ));
    assert_parity_and_reuse(statement_if(
        boolean(true),
        vec![int(1)],
        Some(vec![variable("missing_else")]),
    ));
}

fn configure_fastmem(builder: &mut MirBuilder) -> FastMemRegionId {
    let region = FastMemRegionId(0);
    builder
        .scope_ctx
        .current_function
        .as_mut()
        .unwrap()
        .metadata
        .fastmem_regions
        .push(FastMemRegionMetadata {
            id: region,
            contract: "IfStatementParityV1".to_string(),
            source_span: Span::unknown(),
            origin: FastMemRegionOrigin::SourceFastMemBlock,
            body_statement_count: 1,
            emitted_memop_count: 0,
        });
    builder.push_fastmem_region(region);
    region
}

#[test]
fn if_statement_parity_fastmem_positive_negative_and_reuse() {
    for (condition, should_succeed) in [(boolean(true), false), (owner_eq_condition(), true)] {
        let mut selected = builder("if_statement_fastmem/0");
        let mut reference = builder("if_statement_fastmem/0");
        configure_fastmem(&mut selected);
        configure_fastmem(&mut reference);
        let statement = statement_if(condition, vec![int(1)], Some(vec![int(2)]));
        let selected_result = lower_selected(&mut selected, statement.clone());
        let reference_result = lower_pre_i0_statement_surface_reference(&mut reference, statement);
        assert_eq!(selected_result.is_ok(), should_succeed);
        assert_eq!(reference_result.is_ok(), should_succeed);
        assert_eq!(
            snapshot(&selected, selected_result),
            snapshot(&reference, reference_result)
        );

        let recovery = statement_if(owner_eq_condition(), vec![int(3)], Some(vec![int(4)]));
        let selected_result = lower_selected(&mut selected, recovery.clone());
        let reference_result = lower_pre_i0_statement_surface_reference(&mut reference, recovery);
        assert!(selected_result.is_ok());
        assert!(reference_result.is_ok());
        assert_eq!(
            snapshot(&selected, selected_result),
            snapshot(&reference, reference_result)
        );
    }
}

#[test]
fn if_statement_parity_recursion_boundaries_restore_exact_state() {
    for depth in [198, 199, 200] {
        let mut selected = builder("if_statement_recursion/0");
        let mut reference = builder("if_statement_recursion/0");
        selected.recursion_depth = depth;
        reference.recursion_depth = depth;
        let statement = statement_if(boolean(true), vec![int(1)], Some(vec![int(2)]));
        let selected_result = lower_selected(&mut selected, statement.clone());
        let reference_result = lower_pre_i0_statement_surface_reference(&mut reference, statement);
        assert_eq!(selected_result.is_ok(), depth == 198);
        assert_eq!(reference_result.is_ok(), depth == 198);
        if depth >= 199 {
            assert!(selected_result
                .as_ref()
                .unwrap_err()
                .contains("Recursion depth exceeded"));
            assert!(reference_result
                .as_ref()
                .unwrap_err()
                .contains("Recursion depth exceeded"));
        }
        assert_eq!(
            snapshot(&selected, selected_result),
            snapshot(&reference, reference_result)
        );
        assert_eq!(selected.recursion_depth, depth);
        assert_eq!(reference.recursion_depth, depth);
    }
}

#[test]
fn if_statement_parity_preserves_outer_and_branch_program_spans() {
    let outer = Span::new(1, 1, 1, 20);
    let condition = Span::new(1, 4, 1, 8);
    let then_span = Span::new(2, 3, 2, 4);
    let else_span = Span::new(4, 3, 4, 4);
    for (else_body, expected_span) in [
        (vec![int_at(2, else_span)], else_span),
        (Vec::new(), Span::unknown()),
    ] {
        let mut selected = builder("if_statement_span/0");
        let mut reference = builder("if_statement_span/0");
        let statement = statement_if_at(
            ASTNode::Literal {
                value: LiteralValue::Bool(true),
                span: condition,
            },
            vec![int_at(1, then_span)],
            Some(else_body),
            outer,
        );
        let selected_result = lower_selected(&mut selected, statement.clone());
        let reference_result = lower_pre_i0_statement_surface_reference(&mut reference, statement);
        assert!(selected_result.is_ok());
        assert!(reference_result.is_ok());
        assert_eq!(selected.metadata_ctx.current_span(), expected_span);
        assert_eq!(reference.metadata_ctx.current_span(), expected_span);
        assert_eq!(
            snapshot(&selected, selected_result),
            snapshot(&reference, reference_result)
        );
    }
}
