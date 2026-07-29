use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::builder::recursive_child_lowering::{
    drive_raw_legacy_expression_v1, with_legacy_expression_recursion_guard_v1,
    RawLegacyChildLoweringPortV1,
};
use crate::mir::exact_numeric_value_facts::{ExactNumericConstFact, ExactNumericValueFact};
use crate::mir::value_kind::MirValueKind;
use crate::mir::{BasicBlockId, BindingId, MirBuilder, MirInstruction, MirType, ValueId};

use super::return_stmt::{
    build_void_return_statement, emit_return_from_value, try_apply_match_return_optimization,
};
use super::{drive_value_return_statement_v1, RawLegacyValueReturnInputV1};

#[derive(Debug, PartialEq)]
struct ScopeFrameSnapshotV1 {
    declared: Vec<String>,
    restore: Vec<(String, Option<ValueId>)>,
    restore_binding: Vec<(String, Option<BindingId>)>,
}

#[derive(Debug, PartialEq)]
struct ReturnParitySnapshotV1 {
    result: Result<ValueId, String>,
    blocks: Vec<(BasicBlockId, Vec<MirInstruction>, Option<MirInstruction>)>,
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
    pin_slots: Vec<(ValueId, String)>,
    local_ssa_map: Vec<((BasicBlockId, ValueId, u8), ValueId)>,
    schedule_mat_map: Vec<((BasicBlockId, ValueId), ValueId)>,
    current_block: Option<BasicBlockId>,
    next_value_id: u32,
    next_core_value: ValueId,
    next_core_block: BasicBlockId,
    next_binding_id: u32,
    temp_slot_counter: u32,
    recursion_depth: usize,
    current_span: Span,
    in_cleanup_block: bool,
    cleanup_allow_return: bool,
    return_defer_active: bool,
    return_defer_slot: Option<ValueId>,
    return_defer_target: Option<BasicBlockId>,
    return_deferred_emitted: bool,
}

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::new(10, 1, 10, 2),
    }
}

fn boolean(value: bool) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Bool(value),
        span: Span::new(11, 1, 11, 2),
    }
}

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_string(),
        span: Span::new(12, 1, 12, 2),
    }
}

fn binary(operator: BinaryOperator, left: ASTNode, right: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator,
        left: Box::new(left),
        right: Box::new(right),
        span: Span::new(16, 1, 16, 2),
    }
}

fn type_check(receiver: ASTNode) -> ASTNode {
    ASTNode::MethodCall {
        object: Box::new(receiver),
        method: "is".to_string(),
        arguments: vec![ASTNode::Literal {
            value: LiteralValue::String("Integer".to_string()),
            span: Span::new(13, 1, 13, 2),
        }],
        span: Span::new(14, 1, 14, 2),
    }
}

fn value_return(value: ASTNode) -> ASTNode {
    ASTNode::Return {
        value: Some(Box::new(value)),
        span: Span::new(20, 1, 20, 2),
    }
}

fn void_return() -> ASTNode {
    ASTNode::Return {
        value: None,
        span: Span::new(21, 1, 21, 2),
    }
}

fn accepted_match() -> ASTNode {
    ASTNode::MatchExpr {
        scrutinee: Box::new(integer(2)),
        arms: vec![
            (LiteralValue::Integer(1), integer(10)),
            (LiteralValue::Integer(2), integer(20)),
        ],
        else_expr: Box::new(integer(30)),
        span: Span::new(15, 1, 15, 2),
    }
}

fn builder(name: &str) -> MirBuilder {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test(name.to_string());
    builder
}

fn lower_selected(builder: &mut MirBuilder, expression: ASTNode) -> Result<ValueId, String> {
    let span = expression.span();
    let node_kind = std::mem::discriminant(&expression);
    let ASTNode::Return { value, .. } = expression else {
        return Err("RET0-I0 selected owner requires Return".to_string());
    };
    with_legacy_expression_recursion_guard_v1(builder, node_kind, move |builder| {
        builder.metadata_ctx.set_current_span(span);
        match value {
            Some(value) => {
                let input = RawLegacyValueReturnInputV1::new(*value);
                let mut port = RawLegacyChildLoweringPortV1;
                drive_value_return_statement_v1(builder, &mut port, &input)
            }
            None => build_void_return_statement(builder),
        }
    })
}

fn lower_pre_i0_return_reference(
    builder: &mut MirBuilder,
    expression: ASTNode,
) -> Result<ValueId, String> {
    let span = expression.span();
    let node_kind = std::mem::discriminant(&expression);
    let ASTNode::Return { value, .. } = expression else {
        return Err("RET0-P0 reference requires Return".to_string());
    };

    with_legacy_expression_recursion_guard_v1(builder, node_kind, move |builder| {
        builder.metadata_ctx.set_current_span(span);
        if builder.function_state.in_cleanup_block && !builder.function_state.cleanup_allow_return {
            return Err("return is not allowed inside cleanup block (enable NYASH_CLEANUP_ALLOW_RETURN=1 to permit)".to_string());
        }
        if let Some(return_value) =
            try_apply_match_return_optimization(builder, value.as_deref(), true)?
        {
            return Ok(return_value);
        }

        let return_value = if let Some(expr) = value {
            drive_raw_legacy_expression_v1(builder, *expr)?
        } else {
            crate::mir::builder::emission::constant::emit_void(builder)?
        };
        emit_return_from_value(builder, return_value)
    })
}

fn snapshot(builder: &MirBuilder, result: Result<ValueId, String>) -> ReturnParitySnapshotV1 {
    let function = builder
        .function_state
        .current_function
        .as_ref()
        .expect("current RET0-P0 function");
    let mut blocks = function
        .blocks
        .iter()
        .map(|(id, block)| (*id, block.instructions.clone(), block.terminator.clone()))
        .collect::<Vec<_>>();
    blocks.sort_by_key(|(id, _, _)| *id);

    let mut value_kinds = builder
        .function_state
        .type_ctx
        .value_kinds
        .iter()
        .map(|(value, kind)| (*value, *kind))
        .collect::<Vec<_>>();
    value_kinds.sort_by_key(|(value, _)| *value);

    let mut value_origins = builder
        .function_state
        .type_ctx
        .value_origin_newbox
        .iter()
        .map(|(value, owner)| (*value, owner.clone()))
        .collect::<Vec<_>>();
    value_origins.sort_by_key(|(value, _)| *value);

    let mut string_literals = builder
        .function_state
        .type_ctx
        .string_literals
        .iter()
        .map(|(value, text)| (*value, text.clone()))
        .collect::<Vec<_>>();
    string_literals.sort_by_key(|(value, _)| *value);

    let mut variable_map = builder
        .function_state
        .variable_ctx
        .variable_map
        .iter()
        .map(|(name, value)| (name.clone(), *value))
        .collect::<Vec<_>>();
    variable_map.sort_by(|left, right| left.0.cmp(&right.0));

    let bindings = variable_map
        .iter()
        .map(|(name, _)| {
            (
                name.clone(),
                builder.function_state.binding_ctx.lookup(name),
            )
        })
        .collect();

    let scope_frames = builder
        .function_state
        .scope
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

    let mut pin_slots = builder
        .function_state
        .pin_slot_names
        .iter()
        .map(|(value, name)| (*value, name.clone()))
        .collect::<Vec<_>>();
    pin_slots.sort_by_key(|(value, _)| *value);

    let mut local_ssa_map = builder
        .function_state
        .local_ssa_map
        .iter()
        .map(|(key, value)| (*key, *value))
        .collect::<Vec<_>>();
    local_ssa_map.sort_by_key(|(key, value)| (*key, *value));

    let mut schedule_mat_map = builder
        .function_state
        .schedule_mat_map
        .iter()
        .map(|(key, value)| (*key, *value))
        .collect::<Vec<_>>();
    schedule_mat_map.sort_by_key(|(key, value)| (*key, *value));

    let mut value_types = builder
        .function_state
        .type_ctx
        .value_types
        .iter()
        .map(|(value, ty)| (*value, ty.clone()))
        .collect::<Vec<_>>();
    value_types.sort_by_key(|(value, _)| *value);

    let mut exact_numeric_const_facts = function
        .metadata
        .exact_numeric_const_facts
        .iter()
        .map(|(value, fact)| (*value, fact.clone()))
        .collect::<Vec<_>>();
    exact_numeric_const_facts.sort_by_key(|(value, _)| *value);

    let mut exact_numeric_value_facts = function
        .metadata
        .exact_numeric_value_facts
        .iter()
        .map(|(value, fact)| (*value, fact.clone()))
        .collect::<Vec<_>>();
    exact_numeric_value_facts.sort_by_key(|(value, _)| *value);

    ReturnParitySnapshotV1 {
        result,
        blocks,
        locals: function.locals.clone(),
        value_types,
        value_kinds,
        value_origins,
        string_literals,
        exact_numeric_const_facts,
        exact_numeric_value_facts,
        variable_map,
        bindings,
        scope_frames,
        pin_slots,
        local_ssa_map,
        schedule_mat_map,
        current_block: builder.function_state.current_block,
        next_value_id: function.next_value_id,
        next_core_value: builder.core_ctx.peek_next_value(),
        next_core_block: builder.core_ctx.peek_next_block(),
        next_binding_id: builder.core_ctx.next_binding_id,
        temp_slot_counter: builder.core_ctx.temp_slot_counter,
        recursion_depth: builder.recursion_depth,
        current_span: builder.metadata_ctx.current_span(),
        in_cleanup_block: builder.function_state.in_cleanup_block,
        cleanup_allow_return: builder.function_state.cleanup_allow_return,
        return_defer_active: builder.function_state.return_defer_active,
        return_defer_slot: builder.function_state.return_defer_slot,
        return_defer_target: builder.function_state.return_defer_target,
        return_deferred_emitted: builder.function_state.return_deferred_emitted,
    }
}

fn assert_parity(expression: ASTNode) {
    let mut selected = builder("return_parity/0");
    let mut reference = builder("return_parity/0");

    let selected_result = lower_selected(&mut selected, expression.clone());
    let reference_result = lower_pre_i0_return_reference(&mut reference, expression);

    assert_eq!(
        snapshot(&selected, selected_result),
        snapshot(&reference, reference_result)
    );
}

#[test]
fn literal_binary_short_circuit_and_method_call_have_exact_pre_i0_parity() {
    for value in [
        integer(9),
        binary(BinaryOperator::Add, integer(2), integer(3)),
        binary(BinaryOperator::And, boolean(true), boolean(false)),
        type_check(integer(8)),
    ] {
        assert_parity(value_return(value));
    }
}

#[test]
fn void_return_has_exact_pre_i0_parity() {
    assert_parity(void_return());
}

#[test]
fn selected_match_return_has_exact_pre_i0_parity() {
    assert_parity(value_return(accepted_match()));
}

#[test]
fn configured_defer_has_exact_pre_i0_parity() {
    let mut selected = builder("return_parity_defer/0");
    let mut reference = builder("return_parity_defer/0");
    let selected_slot = selected.next_value_id();
    let reference_slot = reference.next_value_id();
    let selected_target = selected.next_block_id();
    let reference_target = reference.next_block_id();
    assert_eq!(selected_slot, reference_slot);
    assert_eq!(selected_target, reference_target);

    for builder in [&mut selected, &mut reference] {
        builder.function_state.return_defer_active = true;
        builder.function_state.return_defer_slot = Some(selected_slot);
        builder.function_state.return_defer_target = Some(selected_target);
    }

    let expression = value_return(type_check(integer(8)));
    let selected_result = lower_selected(&mut selected, expression.clone());
    let reference_result = lower_pre_i0_return_reference(&mut reference, expression);
    assert_eq!(
        snapshot(&selected, selected_result),
        snapshot(&reference, reference_result)
    );
}

#[test]
fn cleanup_and_child_failures_plus_same_builder_reuse_have_exact_pre_i0_parity() {
    let mut selected = builder("return_parity_cleanup/0");
    let mut reference = builder("return_parity_cleanup/0");
    for builder in [&mut selected, &mut reference] {
        builder.function_state.in_cleanup_block = true;
        builder.function_state.cleanup_allow_return = false;
    }

    let cleanup = value_return(type_check(integer(8)));
    let selected_result = lower_selected(&mut selected, cleanup.clone());
    let reference_result = lower_pre_i0_return_reference(&mut reference, cleanup);
    assert_eq!(
        snapshot(&selected, selected_result),
        snapshot(&reference, reference_result)
    );

    selected.function_state.in_cleanup_block = false;
    reference.function_state.in_cleanup_block = false;
    let recovery = value_return(integer(1));
    let selected_result = lower_selected(&mut selected, recovery.clone());
    let reference_result = lower_pre_i0_return_reference(&mut reference, recovery);
    assert_eq!(
        snapshot(&selected, selected_result),
        snapshot(&reference, reference_result)
    );

    let mut selected = builder("return_parity_child_failure/0");
    let mut reference = builder("return_parity_child_failure/0");
    let failure = value_return(variable("missing"));
    let selected_result = lower_selected(&mut selected, failure.clone());
    let reference_result = lower_pre_i0_return_reference(&mut reference, failure);
    assert_eq!(
        snapshot(&selected, selected_result),
        snapshot(&reference, reference_result)
    );

    let recovery = value_return(integer(2));
    let selected_result = lower_selected(&mut selected, recovery.clone());
    let reference_result = lower_pre_i0_return_reference(&mut reference, recovery);
    assert_eq!(
        snapshot(&selected, selected_result),
        snapshot(&reference, reference_result)
    );
}
