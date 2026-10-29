use crate::mir::{BasicBlockId, MirFunction, MirInstruction, MirModule};
use hakorune_mir_defs::{CanonicalFieldRefV1, CanonicalObjectIdV1};

type SourceRead = ((BasicBlockId, MirInstruction), CanonicalFieldRefV1);

pub(super) fn source_array_reads(
    module: Option<&MirModule>, function: &MirFunction,
    view: Option<&super::PublishedMirBackendView<'_>>,
    role: &super::PublishedLifecyclePhysicalFunctionRoleV1,
) -> Result<Vec<SourceRead>, String> {
    let Some(view) = view else { return Ok(Vec::new()) };
    let Some(source) = view.retained_root_source() else { return Ok(Vec::new()) };
    let module = module.ok_or_else(|| fault("field-read-module-missing"))?;
    if !std::ptr::eq(module, view.module()) { return Err(fault("field-read-view-membership")); }
    let Some(owner) = source_owner(view, role)? else { return Ok(Vec::new()) };
    source.finished_array_field_reads_v1(owner, module, function, view.validated_named_arrays()?)
}

pub(super) fn source_array_counts(
    module: Option<&MirModule>, function: &MirFunction,
    view: Option<&super::PublishedMirBackendView<'_>>,
) -> Result<Vec<(BasicBlockId, MirInstruction)>, String> {
    let Some(view) = view else { return Ok(Vec::new()) };
    let module = module.ok_or_else(|| fault("array-count/module-missing"))?;
    if !std::ptr::eq(module, view.module()) { return Err(fault("array-count/view-membership")); }
    let mut result = Vec::new();
    for row in view.validated_named_arrays()? {
        if let crate::mir::normal_callable_semantic_package::EmittedNamedArrayRequirementV1::Read(read) = row {
            if let Some(binding) = read.count_binding_v1(module, function)? { result.push(binding); }
        }
    }
    if let Some(source) = view.retained_root_source() {
        result.extend(source.finished_keys_count_reads_v1(module, function)?);
    }
    Ok(result)
}

/// Same source membership for checked received reads, independently of opcode.
pub(super) fn source_received_reads(
    module: Option<&MirModule>, function: &MirFunction,
    view: Option<&super::PublishedMirBackendView<'_>>,
    role: &super::PublishedLifecyclePhysicalFunctionRoleV1,
) -> Result<Vec<(BasicBlockId, MirInstruction)>, String> {
    let Some(view) = view else { return Ok(Vec::new()) };
    let Some(source) = view.retained_root_source() else { return Ok(Vec::new()) };
    let module = module.ok_or_else(|| fault("received-field-module-missing"))?;
    if !std::ptr::eq(module, view.module()) { return Err(fault("received-field-view-membership")); }
    let Some(owner) = source_owner(view, role)? else { return Ok(Vec::new()) };
    source.finished_received_field_reads_v1(owner, module, function)
}

fn source_owner(
    view: &super::PublishedMirBackendView<'_>, role: &super::PublishedLifecyclePhysicalFunctionRoleV1,
) -> Result<Option<crate::mir::resolved_semantics::FunctionOwnerIdV1>, String> {
    let Some(source) = view.retained_root_source() else { return Ok(None) };
    let owner = if let Some(key) = role.ordinary_target() {
        view.retained_handoff.and_then(|handoff| handoff.callables())
            .and_then(|callables| callables.owner_for_canonical_key(key))
            .ok_or_else(|| fault("field-read-owner-missing"))?
    } else if matches!(role, super::PublishedLifecyclePhysicalFunctionRoleV1::Root { .. }) {
        source.owner()
    } else { return Ok(None) };
    Ok(Some(owner))
}

/// Prepare an already-issued exact typed-object route for the physical row.
/// A missing route is not repaired here: the selected physical consumer
/// rejects the instruction.
pub(super) fn prepare_field_ref(
    module: Option<&MirModule>,
    function: &MirFunction,
    block: BasicBlockId,
    instruction_index: usize,
    instruction: &MirInstruction,
    source_reads: &[SourceRead],
) -> Result<Option<CanonicalFieldRefV1>, String> {
    match instruction {
        MirInstruction::FieldGet { .. } | MirInstruction::FieldSet { .. } => {}
        _ => return Ok(None),
    }
    let Some(module) = module else {
        return Ok(None);
    };
    match instruction {
        MirInstruction::FieldGet { .. } => {
            project_field_get(module, function, block, instruction_index, instruction, source_reads)
        }
        MirInstruction::FieldSet { .. } => {
            project_field_set(module, function, block, instruction_index, instruction)
        }
        _ => unreachable!("field instructions filtered above"),
    }
}

fn project_field_get(
    module: &MirModule,
    function: &MirFunction,
    block: BasicBlockId,
    instruction_index: usize,
    instruction: &MirInstruction,
    source_reads: &[SourceRead],
) -> Result<Option<CanonicalFieldRefV1>, String> {
    let MirInstruction::FieldGet { field, .. } = instruction else {
        return Ok(None);
    };
    let mut rows = function.metadata.route_decisions.iter().filter(|decision| {
        decision.source_plan_kind == "TypedObjectExactSlotRoute"
            && decision.semantic_op == "FieldGet"
            && decision.block == block
            && decision.instruction_index == instruction_index
    });
    let Some(decision) = rows.next() else {
        return Ok(None);
    };
    // Exact i64 reads are unchanged; a numeric-integer slot stays physical
    // bits only inside a function that carries a borrowed tagged formal, the
    // same corridor invoke.rs grants the checked-compare view.
    let i64_route = decision.selected_route == "hako.typed_object.slot_load_i64"
        && decision.selected_storage == Some("i64");
    let numeric_view_route = decision.selected_route == "hako.typed_object.slot_load_u64"
        && decision.selected_storage == Some("u64")
        && function.metadata.physical_param_carriers.as_deref().is_some_and(
            |carriers| {
                carriers.contains(
                    &crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1::BorrowedTaggedValue,
                )
            },
        );
    // Original field-resident aliases borrow exact source and finished reads.
    // The older direct `.set` corridor remains until its source-use handoff
    // replaces its distinct guarded receiver/operand obligations.
    let source_field = source_reads.iter().find_map(|(binding, field)|
        (binding.0 == block && binding.1 == *instruction).then_some(*field));
    let array_receiver_route = matches!(instruction,
            MirInstruction::FieldGet {
                declared_type: Some(crate::mir::MirType::Box(name)), ..
            } if name == "ArrayBox")
        && decision.selected_route == "hako.typed_object.slot_load_handle"
        && decision.selected_storage == Some("handle")
        && (source_field.is_some() || function.metadata.physical_param_carriers.as_deref().is_some_and(
            |carriers| carriers.contains(
                &crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1::BorrowedTaggedValue,
            ),
        ));
    if rows.next().is_some()
        || !(i64_route || numeric_view_route || array_receiver_route)
        || decision.field_id.as_deref() != Some(field.as_str())
    {
        return Err(format!("{} function={} block={block:?} index={instruction_index} instruction={instruction:?} route={} storage={:?} receiver={:?} slot={:?} field={:?} carriers={:?}",
            fault("field-get-route-drift"), function.signature.name,
            decision.selected_route, decision.selected_storage, decision.receiver_box_name,
            decision.selected_slot, decision.field_id, function.metadata.physical_param_carriers));
    }
    let Some(box_name) = decision.receiver_box_name.as_deref() else {
        return Err(fault("field-get-receiver-missing"));
    };
    let Some(slot) = decision.selected_slot else {
        return Err(fault("field-get-slot-missing"));
    };
    let object = module
        .metadata
        .canonical_object_membership
        .as_ref()
        .and_then(|membership| membership.get(box_name).copied())
        .ok_or_else(|| fault("field-get-object-missing"))?;
    let object = CanonicalObjectIdV1::from_declaration_index(object.declaration_index() as usize)
        .ok_or_else(|| fault("field-get-object-overflow"))?;
    let field = CanonicalFieldRefV1::from_declaration_ordinal(object, slot as usize)
        .ok_or_else(|| fault("field-get-slot-overflow"))?;
    if source_field.is_some_and(|source| source != field) {
        return Err(fault("field-get-source-route-drift"));
    }
    Ok(Some(field))
}

fn project_field_set(
    module: &MirModule,
    function: &MirFunction,
    block: BasicBlockId,
    instruction_index: usize,
    instruction: &MirInstruction,
) -> Result<Option<CanonicalFieldRefV1>, String> {
    let MirInstruction::FieldSet { field, .. } = instruction else {
        return Ok(None);
    };
    let mut rows = function.metadata.route_decisions.iter().filter(|decision| {
        decision.source_plan_kind == "TypedObjectExactSlotRoute"
            && decision.semantic_op == "FieldSet"
            && decision.block == block
            && decision.instruction_index == instruction_index
    });
    let Some(decision) = rows.next() else {
        return Ok(None);
    };
    // Exact i64 stores are unchanged; a numeric-integer slot stays physical
    // bits only inside a function that carries a borrowed tagged formal, the
    // same corridor the u64 load lane already uses.
    let i64_route = decision.selected_route == "hako.typed_object.slot_store_i64"
        && decision.selected_storage == Some("i64");
    let numeric_view_route = decision.selected_route == "hako.typed_object.slot_store_u64"
        && decision.selected_storage == Some("u64")
        && function.metadata.physical_param_carriers.as_deref().is_some_and(
            |carriers| {
                carriers.contains(
                    &crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1::BorrowedTaggedValue,
                )
            },
        );
    if rows.next().is_some()
        || !(i64_route || numeric_view_route)
        || decision.field_id.as_deref() != Some(field.as_str())
    {
        return Err(fault("field-set-route-drift"));
    }
    let Some(box_name) = decision.receiver_box_name.as_deref() else {
        return Err(fault("field-set-receiver-missing"));
    };
    let Some(slot) = decision.selected_slot else {
        return Err(fault("field-set-slot-missing"));
    };
    let object = module
        .metadata
        .canonical_object_membership
        .as_ref()
        .and_then(|membership| membership.get(box_name).copied())
        .ok_or_else(|| fault("field-set-object-missing"))?;
    let object = CanonicalObjectIdV1::from_declaration_index(object.declaration_index() as usize)
        .ok_or_else(|| fault("field-set-object-overflow"))?;
    CanonicalFieldRefV1::from_declaration_ordinal(object, slot as usize)
        .map(Some)
        .ok_or_else(|| fault("field-set-slot-overflow"))
}

fn fault(reason: &str) -> String {
    format!("[freeze:contract][published-lifecycle-program/{reason}]")
}
