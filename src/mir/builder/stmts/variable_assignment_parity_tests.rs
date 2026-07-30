use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::builder::recursive_child_lowering::{
    drive_raw_legacy_expression_v1, with_legacy_expression_recursion_guard_v1,
    RawLegacyChildLoweringPortV1,
};
use crate::mir::builder::vars::assignment_resolver::AssignmentResolverBox;
use crate::mir::builder::vars::lexical_scope::LexicalScopeGuard;
use crate::mir::exact_numeric_value_facts::{ExactNumericConstFact, ExactNumericValueFact};
use crate::mir::function::{
    ArrayElementWriteWitness, ArrayStateTerm, LocalIdentityEvidence, LocalSlotContract,
    TypedArrayContractSource, TypedArrayElementContract,
};
use crate::mir::region::function_slot_registry::FunctionSlotRegistry;
use crate::mir::region::RefSlotKind;
use crate::mir::value_kind::MirValueKind;
use crate::mir::{BasicBlockId, BindingId, MirBuilder, MirInstruction, MirType, ValueId};

use super::{
    drive_local_statement_v1, drive_variable_assignment_v1, RawLegacyLocalInputV1,
    RawLegacyVariableAssignmentInputV1,
};

#[derive(Debug, PartialEq)]
struct ScopeFrameSnapshotV1 {
    declared: Vec<String>,
    restore: Vec<(String, Option<ValueId>)>,
    restore_binding: Vec<(String, Option<BindingId>)>,
}

#[derive(Debug, PartialEq)]
struct AssignmentParitySnapshotV1 {
    result: Result<ValueId, String>,
    blocks: Vec<(BasicBlockId, Vec<MirInstruction>, Option<MirInstruction>)>,
    locals: Vec<MirType>,
    value_types: Vec<(ValueId, MirType)>,
    value_kinds: Vec<(ValueId, MirValueKind)>,
    value_origins: Vec<(ValueId, String)>,
    string_literals: Vec<(ValueId, String)>,
    map_value_types: Vec<(ValueId, MirType)>,
    map_literal_value_types: Vec<((ValueId, String), MirType)>,
    variable_map: Vec<(String, ValueId)>,
    bindings: Vec<(String, Option<BindingId>)>,
    scope_frames: Vec<ScopeFrameSnapshotV1>,
    pin_slots: Vec<(ValueId, String)>,
    local_slot_contracts: Vec<LocalSlotContract>,
    local_identity_evidence: Vec<LocalIdentityEvidence>,
    array_element_write_witnesses: Vec<ArrayElementWriteWitness>,
    array_state_terms: Vec<ArrayStateTerm>,
    typed_array_contract_sources: Vec<TypedArrayContractSource>,
    typed_array_element_contracts: Vec<TypedArrayElementContract>,
    exact_numeric_const_facts: Vec<(ValueId, ExactNumericConstFact)>,
    exact_numeric_value_facts: Vec<(ValueId, ExactNumericValueFact)>,
    slot_registry: Vec<(String, Option<MirType>, Option<RefSlotKind>)>,
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
}

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
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

fn binary(operator: BinaryOperator, left: ASTNode, right: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator,
        left: Box::new(left),
        right: Box::new(right),
        span: Span::unknown(),
    }
}

fn array(elements: Vec<ASTNode>) -> ASTNode {
    ASTNode::ArrayLiteral {
        elements,
        span: Span::unknown(),
    }
}

fn local(name: &str, initial: ASTNode, declared_type: Option<&str>) -> ASTNode {
    ASTNode::Local {
        variables: vec![name.to_string()],
        initial_values: vec![Some(Box::new(initial))],
        declared_type_names: vec![declared_type.map(str::to_string)],
        span: Span::unknown(),
    }
}

fn assignment(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(variable(name)),
        value: Box::new(value),
        span: Span::unknown(),
    }
}

fn grouped_assignment(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::GroupedAssignmentExpr {
        lhs: name.to_string(),
        rhs: Box::new(value),
        span: Span::unknown(),
    }
}

fn builder(name: &str) -> MirBuilder {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test(name.to_string());
    builder.comp_ctx.current_slot_registry = Some(FunctionSlotRegistry::new());
    builder
}

fn lower_selected(builder: &mut MirBuilder, expression: ASTNode) -> Result<ValueId, String> {
    let span = expression.span();
    let node_kind = std::mem::discriminant(&expression);
    let ASTNode::Assignment { target, value, .. } = expression else {
        return Err("ASN0-I0 selected owner requires Assignment".to_string());
    };
    let ASTNode::Variable { name, .. } = *target else {
        return Err("ASN0-I0 selected owner requires exact Variable target".to_string());
    };
    with_legacy_expression_recursion_guard_v1(builder, node_kind, move |builder| {
        builder.metadata_ctx.set_current_span(span);
        let input = RawLegacyVariableAssignmentInputV1::new(name, *value);
        let mut port = RawLegacyChildLoweringPortV1;
        drive_variable_assignment_v1(builder, &mut port, &input)
    })
}

fn lower_local_seed(builder: &mut MirBuilder, expression: ASTNode) -> Result<ValueId, String> {
    let span = expression.span();
    let node_kind = std::mem::discriminant(&expression);
    if !matches!(&expression, ASTNode::Local { .. }) {
        return Err("ASN0-P0 seed requires Local".to_string());
    }
    with_legacy_expression_recursion_guard_v1(builder, node_kind, move |builder| {
        builder.metadata_ctx.set_current_span(span);
        let input = RawLegacyLocalInputV1::new(expression);
        let mut port = RawLegacyChildLoweringPortV1;
        drive_local_statement_v1(builder, &mut port, input)
    })
}

fn lower_pre_i0_assignment_reference(
    builder: &mut MirBuilder,
    expression: ASTNode,
) -> Result<ValueId, String> {
    let span = expression.span();
    let node_kind = std::mem::discriminant(&expression);
    let ASTNode::Assignment { target, value, .. } = expression else {
        return Err("ASN0-P0 reference requires Assignment".to_string());
    };
    let ASTNode::Variable { name, .. } = *target else {
        return Err("ASN0-P0 reference requires exact Variable target".to_string());
    };
    with_legacy_expression_recursion_guard_v1(builder, node_kind, move |builder| {
        builder.metadata_ctx.set_current_span(span);
        AssignmentResolverBox::ensure_declared(builder, &name)?;
        let value = drive_raw_legacy_expression_v1(builder, *value)?;
        builder.build_assignment_from_value(name, value)
    })
}

fn snapshot(
    builder: &MirBuilder,
    result: Result<ValueId, String>,
    observed_names: &[&str],
) -> AssignmentParitySnapshotV1 {
    let function = builder
        .function_state
        .current_function
        .as_ref()
        .expect("current ASN0-P0 function");
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

    let mut names = observed_names
        .iter()
        .map(|name| (*name).to_string())
        .collect::<Vec<_>>();
    names.extend(
        builder
            .function_state
            .variable_ctx
            .variable_map
            .keys()
            .cloned(),
    );
    names.sort();
    names.dedup();

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

    let slot_registry = builder
        .comp_ctx
        .current_slot_registry
        .as_ref()
        .expect("ASN0-P0 slot registry")
        .iter_slots()
        .map(|slot| (slot.name.clone(), slot.ty.clone(), slot.ref_kind))
        .collect();

    AssignmentParitySnapshotV1 {
        result,
        blocks,
        locals: function.locals.clone(),
        value_types: builder
            .function_state
            .type_ctx
            .value_types
            .iter()
            .map(|(value, ty)| (*value, ty.clone()))
            .collect(),
        value_kinds,
        value_origins: builder
            .function_state
            .type_ctx
            .value_origin_newbox
            .iter()
            .map(|(value, owner)| (*value, owner.clone()))
            .collect(),
        string_literals: builder
            .function_state
            .type_ctx
            .string_literals
            .iter()
            .map(|(value, text)| (*value, text.clone()))
            .collect(),
        map_value_types: builder
            .function_state
            .type_ctx
            .map_value_types
            .iter()
            .map(|(value, ty)| (*value, ty.clone()))
            .collect(),
        map_literal_value_types: builder
            .function_state
            .type_ctx
            .map_literal_value_types
            .iter()
            .map(|(key, ty)| (key.clone(), ty.clone()))
            .collect(),
        variable_map: builder
            .function_state
            .variable_ctx
            .variable_map
            .iter()
            .map(|(name, value)| (name.clone(), *value))
            .collect(),
        bindings: names
            .into_iter()
            .map(|name| {
                let binding = builder.function_state.binding_ctx.lookup(&name);
                (name, binding)
            })
            .collect(),
        scope_frames: builder
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
            .collect(),
        pin_slots,
        local_slot_contracts: function.metadata.local_slot_contracts.clone(),
        local_identity_evidence: function.metadata.local_identity_evidence.clone(),
        array_element_write_witnesses: function.metadata.array_element_write_witnesses.clone(),
        array_state_terms: function.metadata.array_state_terms.clone(),
        typed_array_contract_sources: function.metadata.typed_array_contract_sources.clone(),
        typed_array_element_contracts: function.metadata.typed_array_element_contracts.clone(),
        exact_numeric_const_facts: function
            .metadata
            .exact_numeric_const_facts
            .iter()
            .map(|(value, fact)| (*value, fact.clone()))
            .collect(),
        exact_numeric_value_facts: function
            .metadata
            .exact_numeric_value_facts
            .iter()
            .map(|(value, fact)| (*value, fact.clone()))
            .collect(),
        slot_registry,
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
    }
}

fn assert_parity(seed: Option<ASTNode>, expression: ASTNode, observed_names: &[&str]) {
    let mut selected = builder("assignment_parity/0");
    let _selected_scope = LexicalScopeGuard::new(&mut selected);
    let mut reference = builder("assignment_parity/0");
    let _reference_scope = LexicalScopeGuard::new(&mut reference);

    if let Some(seed) = seed {
        let selected_seed = lower_local_seed(&mut selected, seed.clone());
        let reference_seed = lower_local_seed(&mut reference, seed);
        assert_eq!(
            snapshot(&selected, selected_seed, observed_names),
            snapshot(&reference, reference_seed, observed_names)
        );
    }

    let selected_result = lower_selected(&mut selected, expression.clone());
    let reference_result = lower_pre_i0_assignment_reference(&mut reference, expression);
    assert_eq!(
        snapshot(&selected, selected_result, observed_names),
        snapshot(&reference, reference_result, observed_names)
    );
}

fn assert_failure_and_reuse_parity(
    seed: Option<ASTNode>,
    failure: ASTNode,
    recovery_seed: Option<ASTNode>,
    recovery: ASTNode,
    observed_names: &[&str],
) {
    let mut selected = builder("assignment_parity_reuse/0");
    let _selected_scope = LexicalScopeGuard::new(&mut selected);
    let mut reference = builder("assignment_parity_reuse/0");
    let _reference_scope = LexicalScopeGuard::new(&mut reference);

    if let Some(seed) = seed {
        lower_local_seed(&mut selected, seed.clone()).unwrap();
        lower_local_seed(&mut reference, seed).unwrap();
    }

    let selected_failure = lower_selected(&mut selected, failure.clone());
    let reference_failure = lower_pre_i0_assignment_reference(&mut reference, failure);
    assert_eq!(
        snapshot(&selected, selected_failure, observed_names),
        snapshot(&reference, reference_failure, observed_names)
    );

    if let Some(seed) = recovery_seed {
        let selected_seed = lower_local_seed(&mut selected, seed.clone());
        let reference_seed = lower_local_seed(&mut reference, seed);
        assert_eq!(
            snapshot(&selected, selected_seed, observed_names),
            snapshot(&reference, reference_seed, observed_names)
        );
    }

    let selected_recovery = lower_selected(&mut selected, recovery.clone());
    let reference_recovery = lower_pre_i0_assignment_reference(&mut reference, recovery);
    assert_eq!(
        snapshot(&selected, selected_recovery, observed_names),
        snapshot(&reference, reference_recovery, observed_names)
    );
}

#[test]
fn literal_binary_and_short_circuit_rhs_have_exact_pre_i0_snapshot_parity() {
    for rhs in [
        integer(9),
        binary(BinaryOperator::Add, integer(2), integer(3)),
        binary(BinaryOperator::And, boolean(true), boolean(false)),
    ] {
        assert_parity(
            Some(local("x", integer(1), None)),
            assignment("x", rhs),
            &["x"],
        );
    }
}

#[test]
fn exact_local_contract_reassignment_has_exact_pre_i0_snapshot_parity() {
    assert_parity(
        Some(local("x", integer(1), Some("i64"))),
        assignment(
            "x",
            binary(BinaryOperator::Multiply, integer(6), integer(7)),
        ),
        &["x"],
    );
}

#[test]
fn typed_array_reassignment_has_exact_pre_i0_snapshot_parity() {
    assert_parity(
        Some(local(
            "xs",
            array(vec![integer(1), integer(2)]),
            Some("Array<u8>"),
        )),
        assignment("xs", array(vec![integer(3), integer(4)])),
        &["xs"],
    );
}

#[test]
fn undeclared_and_rhs_failures_plus_reuse_have_exact_pre_i0_snapshot_parity() {
    assert_failure_and_reuse_parity(
        None,
        assignment(
            "missing",
            binary(BinaryOperator::Add, integer(91), integer(1)),
        ),
        Some(local("x", integer(1), None)),
        assignment("x", integer(5)),
        &["missing", "x"],
    );
    assert_failure_and_reuse_parity(
        Some(local("x", integer(1), None)),
        assignment(
            "x",
            binary(BinaryOperator::Add, integer(1), variable("missing_rhs")),
        ),
        None,
        assignment("x", integer(5)),
        &["x", "missing_rhs"],
    );
}

#[test]
fn pre_i0_reference_rejects_grouped_assignment_before_effects() {
    let mut builder = builder("assignment_parity_grouped_reject/0");
    let before = snapshot(&builder, Ok(ValueId(0)), &["x"]);

    let error =
        lower_pre_i0_assignment_reference(&mut builder, grouped_assignment("x", integer(9)))
            .unwrap_err();

    assert!(error.contains("requires Assignment"));
    assert_eq!(snapshot(&builder, Ok(ValueId(0)), &["x"]), before);
}
