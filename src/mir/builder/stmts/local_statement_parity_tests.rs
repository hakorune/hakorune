use crate::ast::{ASTNode, BinaryOperator, FieldDecl, LiteralValue, Span};
use crate::mir::builder::recursive_child_lowering::{
    drive_raw_legacy_expression_v1, with_legacy_expression_recursion_guard_v1,
    RawLegacyChildLoweringPortV1,
};
use crate::mir::builder::vars::lexical_scope::LexicalScopeGuard;
use crate::mir::exact_numeric_value_facts::{ExactNumericConstFact, ExactNumericValueFact};
use crate::mir::function::{
    ArrayElementWriteWitness, ArrayStateTerm, LocalIdentityEvidence, LocalSlotContract,
    RecordValueContract, TypedArrayContractSource, TypedArrayElementContract,
};
use crate::mir::region::function_slot_registry::FunctionSlotRegistry;
use crate::mir::region::RefSlotKind;
use crate::mir::value_kind::MirValueKind;
use crate::mir::{BasicBlockId, BindingId, MirBuilder, MirInstruction, MirType, ValueId};

use super::variable_stmt::{
    build_local_statement_from_values_with_types_and_preclaims,
    observe_preflighted_local_statement, preflight_exact_numeric_local_initializers,
};
use super::{drive_local_statement_v1, RawLegacyLocalInputV1};

#[derive(Debug, PartialEq)]
struct ScopeFrameSnapshotV1 {
    declared: Vec<String>,
    restore: Vec<(String, Option<ValueId>)>,
    restore_binding: Vec<(String, Option<BindingId>)>,
}

#[derive(Debug, PartialEq)]
struct RecordLocalSnapshotV1 {
    value: ValueId,
    record_name: String,
    fields: Vec<(String, Option<String>, ValueId)>,
}

#[derive(Debug, PartialEq)]
struct LocalParitySnapshotV1 {
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
    record_value_contracts: Vec<RecordValueContract>,
    array_element_write_witnesses: Vec<ArrayElementWriteWitness>,
    array_state_terms: Vec<ArrayStateTerm>,
    typed_array_contract_sources: Vec<TypedArrayContractSource>,
    typed_array_element_contracts: Vec<TypedArrayElementContract>,
    exact_numeric_const_facts: Vec<(ValueId, ExactNumericConstFact)>,
    exact_numeric_value_facts: Vec<(ValueId, ExactNumericValueFact)>,
    record_local_values: Vec<RecordLocalSnapshotV1>,
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

fn local(
    variables: &[&str],
    initial_values: Vec<Option<Box<ASTNode>>>,
    declared_type_names: Vec<Option<&str>>,
) -> ASTNode {
    ASTNode::Local {
        variables: variables.iter().map(|name| (*name).to_string()).collect(),
        initial_values,
        declared_type_names: declared_type_names
            .into_iter()
            .map(|name| name.map(str::to_string))
            .collect(),
        span: Span::unknown(),
    }
}

fn builder(name: &str, with_record: bool) -> MirBuilder {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test(name.to_string());
    builder.comp_ctx.current_slot_registry = Some(FunctionSlotRegistry::new());
    if with_record {
        builder.comp_ctx.register_record_decl(
            "Pair".to_string(),
            Vec::new(),
            &[FieldDecl {
                name: "value".to_string(),
                declared_type_name: None,
                is_weak: false,
                default_value: None,
            }],
        );
    }
    builder
}

fn lower_selected(builder: &mut MirBuilder, expression: ASTNode) -> Result<ValueId, String> {
    let span = expression.span();
    let node_kind = std::mem::discriminant(&expression);
    if !matches!(&expression, ASTNode::Local { .. }) {
        return Err("LCL0-I0 selected owner requires Local".to_string());
    }
    with_legacy_expression_recursion_guard_v1(builder, node_kind, move |builder| {
        builder.metadata_ctx.set_current_span(span);
        let input = RawLegacyLocalInputV1::new(expression);
        let mut port = RawLegacyChildLoweringPortV1;
        drive_local_statement_v1(builder, &mut port, input)
    })
}

fn lower_pre_i0_local_reference(
    builder: &mut MirBuilder,
    expression: ASTNode,
) -> Result<ValueId, String> {
    let span = expression.span();
    let node_kind = std::mem::discriminant(&expression);
    let ASTNode::Local {
        variables,
        initial_values,
        declared_type_names,
        ..
    } = expression
    else {
        return Err("LCL0-P0 reference requires Local".to_string());
    };
    with_legacy_expression_recursion_guard_v1(builder, node_kind, move |builder| {
        builder.metadata_ctx.set_current_span(span);
        build_pre_i0_local_reference(builder, variables, initial_values, declared_type_names)
    })
}

fn build_pre_i0_local_reference(
    builder: &mut MirBuilder,
    variables: Vec<String>,
    initial_values: Vec<Option<Box<ASTNode>>>,
    declared_type_names: Vec<Option<String>>,
) -> Result<ValueId, String> {
    preflight_exact_numeric_local_initializers(&variables, &initial_values, &declared_type_names)?;
    observe_preflighted_local_statement(&variables, &initial_values);

    let mut evaluated_values = Vec::with_capacity(variables.len());
    let mut preclaimed_arrays = Vec::with_capacity(variables.len());
    for (index, _) in variables.iter().enumerate() {
        let typed_spec = declared_type_names
            .get(index)
            .and_then(|value| value.as_deref())
            .map(crate::typed_array_contract_spec::parse_annotation)
            .transpose()?
            .flatten();
        let mut preclaimed = None;
        let value = match initial_values.get(index).and_then(|value| value.as_deref()) {
            Some(ASTNode::ArrayLiteral { elements, .. }) if typed_spec.is_some() => {
                let (value, contract_id) = builder.build_typed_array_literal(elements.to_vec())?;
                preclaimed = Some((contract_id, typed_spec.expect("guarded typed spec")));
                value
            }
            Some(ASTNode::New {
                class, arguments, ..
            }) if builder.is_record_constructor_class(class) => {
                builder.build_record_constructor_value(class.to_string(), arguments.to_vec())?
            }
            Some(initializer) => drive_raw_legacy_expression_v1(builder, initializer.clone())?,
            None => crate::mir::builder::emission::constant::emit_null(builder)?,
        };
        evaluated_values.push(value);
        preclaimed_arrays.push(preclaimed);
    }

    build_local_statement_from_values_with_types_and_preclaims(
        builder,
        variables,
        evaluated_values,
        declared_type_names,
        preclaimed_arrays,
    )
}

fn snapshot(
    builder: &MirBuilder,
    result: Result<ValueId, String>,
    observed_names: &[&str],
) -> LocalParitySnapshotV1 {
    let function = builder
        .function_state
        .current_function
        .as_ref()
        .expect("current LCL0-P0 function");
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

    let mut pin_slots = builder
        .function_state
        .pin_slot_names
        .iter()
        .map(|(value, name)| (*value, name.clone()))
        .collect::<Vec<_>>();
    pin_slots.sort_by_key(|(value, _)| *value);

    let mut record_local_values = builder
        .function_state
        .compilation
        .record_local_values
        .iter()
        .map(|(value, record)| RecordLocalSnapshotV1 {
            value: *value,
            record_name: record.record_name.clone(),
            fields: record
                .fields
                .iter()
                .map(|field| {
                    (
                        field.name.clone(),
                        field.declared_type_name.clone(),
                        field.value,
                    )
                })
                .collect(),
        })
        .collect::<Vec<_>>();
    record_local_values.sort_by_key(|record| record.value);

    let slot_registry = builder
        .comp_ctx
        .current_slot_registry
        .as_ref()
        .expect("LCL0-P0 slot registry")
        .iter_slots()
        .map(|slot| (slot.name.clone(), slot.ty.clone(), slot.ref_kind))
        .collect();

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

    LocalParitySnapshotV1 {
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
        record_value_contracts: function.metadata.record_value_contracts.clone(),
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
        record_local_values,
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

fn assert_parity(expression: ASTNode, observed_names: &[&str], with_record: bool) {
    let mut selected = builder("local_parity/0", with_record);
    let _selected_scope = LexicalScopeGuard::new(&mut selected);
    let selected_result = lower_selected(&mut selected, expression.clone());
    let selected_snapshot = snapshot(&selected, selected_result, observed_names);

    let mut reference = builder("local_parity/0", with_record);
    let _reference_scope = LexicalScopeGuard::new(&mut reference);
    let reference_result = lower_pre_i0_local_reference(&mut reference, expression);
    let reference_snapshot = snapshot(&reference, reference_result, observed_names);

    assert_eq!(selected_snapshot, reference_snapshot);
}

fn assert_failure_and_reuse_parity(
    failure: ASTNode,
    observed_names: &[&str],
    with_record: bool,
    seed: Option<ASTNode>,
) {
    let mut selected = builder("local_parity_failure/0", with_record);
    let _selected_scope = LexicalScopeGuard::new(&mut selected);
    let mut reference = builder("local_parity_failure/0", with_record);
    let _reference_scope = LexicalScopeGuard::new(&mut reference);

    if let Some(seed) = seed {
        let selected_seed = lower_selected(&mut selected, seed.clone());
        let reference_seed = lower_pre_i0_local_reference(&mut reference, seed);
        assert_eq!(
            snapshot(&selected, selected_seed, observed_names),
            snapshot(&reference, reference_seed, observed_names)
        );
    }

    let selected_result = lower_selected(&mut selected, failure.clone());
    let reference_result = lower_pre_i0_local_reference(&mut reference, failure);
    assert_eq!(
        snapshot(&selected, selected_result, observed_names),
        snapshot(&reference, reference_result, observed_names)
    );

    let recovery = local(&["recovered"], vec![Some(Box::new(integer(5)))], vec![None]);
    let selected_recovery = lower_selected(&mut selected, recovery.clone());
    let reference_recovery = lower_pre_i0_local_reference(&mut reference, recovery);
    assert_eq!(
        snapshot(&selected, selected_recovery, observed_names),
        snapshot(&reference, reference_recovery, observed_names)
    );
}

#[test]
fn ordinary_exact_numeric_and_null_locals_have_exact_pre_i0_snapshot_parity() {
    assert_parity(
        local(
            &["plain", "exact", "empty"],
            vec![Some(Box::new(integer(4))), Some(Box::new(integer(9))), None],
            vec![None, Some("i64"), None],
        ),
        &["plain", "exact", "empty"],
        false,
    );
}

#[test]
fn typed_array_local_has_exact_pre_i0_snapshot_parity() {
    assert_parity(
        local(
            &["xs"],
            vec![Some(Box::new(ASTNode::ArrayLiteral {
                elements: vec![integer(1), integer(2)],
                span: Span::unknown(),
            }))],
            vec![Some("Array<u8>")],
        ),
        &["xs"],
        false,
    );
}

#[test]
fn record_constructor_local_has_exact_pre_i0_snapshot_parity() {
    assert_parity(
        local(
            &["pair"],
            vec![Some(Box::new(ASTNode::New {
                class: "Pair".to_string(),
                arguments: vec![integer(7)],
                type_arguments: Vec::new(),
                field_initializers: Vec::new(),
                span: Span::unknown(),
            }))],
            vec![None],
        ),
        &["pair"],
        true,
    );
}

#[test]
fn binary_and_short_circuit_initializers_have_exact_pre_i0_snapshot_parity() {
    assert_parity(
        local(
            &["sum", "flag"],
            vec![
                Some(Box::new(binary(
                    BinaryOperator::Add,
                    integer(2),
                    integer(3),
                ))),
                Some(Box::new(binary(
                    BinaryOperator::And,
                    boolean(true),
                    boolean(false),
                ))),
            ],
            vec![None, None],
        ),
        &["sum", "flag"],
        false,
    );
}

#[test]
fn preflight_and_child_failures_plus_reuse_have_exact_pre_i0_snapshot_parity() {
    let failures = [
        local(
            &["x", "y"],
            vec![Some(Box::new(integer(1))), None],
            vec![None, Some("i64")],
        ),
        local(
            &["x", "y"],
            vec![
                Some(Box::new(variable("missing_left"))),
                Some(Box::new(integer(91))),
            ],
            vec![None, None],
        ),
        local(
            &["x", "y"],
            vec![
                Some(Box::new(integer(7))),
                Some(Box::new(variable("missing_right"))),
            ],
            vec![None, None],
        ),
    ];

    for failure in failures {
        assert_failure_and_reuse_parity(failure, &["x", "y", "recovered"], false, None);
    }
}

#[test]
fn specialized_and_completion_failures_plus_reuse_have_exact_pre_i0_snapshot_parity() {
    assert_failure_and_reuse_parity(
        local(
            &["x", "ys"],
            vec![Some(Box::new(integer(1))), Some(Box::new(integer(2)))],
            vec![None, Some("Array<String>")],
        ),
        &["x", "ys", "recovered"],
        false,
        None,
    );

    assert_failure_and_reuse_parity(
        local(
            &["xs"],
            vec![Some(Box::new(ASTNode::ArrayLiteral {
                elements: vec![variable("missing_array_element")],
                span: Span::unknown(),
            }))],
            vec![Some("Array<u8>")],
        ),
        &["xs", "recovered"],
        false,
        None,
    );

    assert_failure_and_reuse_parity(
        local(
            &["pair"],
            vec![Some(Box::new(ASTNode::New {
                class: "Pair".to_string(),
                arguments: Vec::new(),
                type_arguments: Vec::new(),
                field_initializers: Vec::new(),
                span: Span::unknown(),
            }))],
            vec![None],
        ),
        &["pair", "recovered"],
        true,
        None,
    );

    assert_failure_and_reuse_parity(
        local(&["duplicate"], vec![Some(Box::new(integer(2)))], vec![None]),
        &["duplicate", "recovered"],
        false,
        Some(local(
            &["duplicate"],
            vec![Some(Box::new(integer(1)))],
            vec![None],
        )),
    );
}
