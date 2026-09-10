use crate::mir::{BasicBlockId, MirFunction, MirInstruction, MirModule};
use hakorune_mir_defs::{CanonicalFieldRefV1, CanonicalObjectIdV1};

/// Prepare an already-issued exact typed-object route for the physical row.
/// A missing route is not repaired here: the selected physical consumer
/// rejects the instruction.
pub(super) fn prepare_field_ref(
    module: Option<&MirModule>,
    function: &MirFunction,
    block: BasicBlockId,
    instruction_index: usize,
    instruction: &MirInstruction,
) -> Result<Option<CanonicalFieldRefV1>, String> {
    let MirInstruction::FieldGet { .. } = instruction else {
        return Ok(None);
    };
    let Some(module) = module else {
        return Ok(None);
    };
    project_field_get(module, function, block, instruction_index, instruction)
}

fn project_field_get(
    module: &MirModule,
    function: &MirFunction,
    block: BasicBlockId,
    instruction_index: usize,
    instruction: &MirInstruction,
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
    if rows.next().is_some()
        || decision.selected_route != "hako.typed_object.slot_load_i64"
        || decision.selected_storage != Some("i64")
        || decision.field_id.as_deref() != Some(field.as_str())
    {
        return Err(fault("field-get-route-drift"));
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
    CanonicalFieldRefV1::from_declaration_ordinal(object, slot as usize)
        .map(Some)
        .ok_or_else(|| fault("field-get-slot-overflow"))
}

fn fault(reason: &str) -> String {
    format!("[freeze:contract][published-lifecycle-program/{reason}]")
}
