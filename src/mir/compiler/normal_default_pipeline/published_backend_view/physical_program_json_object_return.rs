//! Serialize the SAME checked Return projection retained by the physical role.
//! Coordinates corroborate complete retained instructions; no origin inference.
use super::*;
use crate::mir::normal_callable_semantic_package::FinalizedObjectReturnOriginV1;
use crate::mir::{BasicBlockId, MirInstruction};

type Binding = (BasicBlockId, MirInstruction);

fn coordinate(
    function: &PublishedLifecyclePhysicalFunctionV1<'_>,
    binding: &Binding,
) -> Result<Value, String> {
    let mut matches = function
        .blocks()
        .iter()
        .filter(|block| block.id() == binding.0)
        .flat_map(|block| {
            block
                .instructions()
                .iter()
                .copied()
                .chain(std::iter::once(block.terminator()))
        })
        .filter(|row| row.instruction() == &binding.1);
    let row = matches
        .next()
        .ok_or_else(|| fault("object-return/json-binding-missing"))?;
    if matches.next().is_some() {
        return Err(fault("object-return/json-binding-duplicate"));
    }
    Ok(json!({ "block": binding.0.0, "index": row.index() }))
}

pub(in crate::mir::compiler::normal_default_pipeline::published_backend_view) fn encode_object_return(
    function: &PublishedLifecyclePhysicalFunctionV1<'_>,
    at: (BasicBlockId, u32),
    instruction: &MirInstruction,
) -> Result<Option<Value>, String> {
    let Some(rows) = function.role().object_return_outcomes() else {
        return Ok(None);
    };
    let mut matching = rows
        .iter()
        .filter(|row| row.returned().0 == at.0 && &row.returned().1 == instruction);
    let row = matching
        .next()
        .ok_or_else(|| fault("object-return/json-source-missing"))?;
    if matching.next().is_some() {
        return Err(fault("object-return/json-source-duplicate"));
    }
    if coordinate(function, row.returned())? != json!({ "block": at.0.0, "index": at.1 }) {
        return Err(fault("object-return/json-return-coordinate"));
    }
    let mut wire = match row.origin() {
        FinalizedObjectReturnOriginV1::Null { producer } => {
            json!({ "kind": "null", "producer": coordinate(function, producer)? })
        }
        FinalizedObjectReturnOriginV1::Fresh {
            producer,
            allocation,
        } => json!({ "kind": "fresh", "producer": coordinate(function, producer)?,
                "invoke": coordinate(function, allocation)? }),
        FinalizedObjectReturnOriginV1::OwnedCall { producer, invoke } => {
            json!({ "kind": "owned_call", "producer": coordinate(function, producer)?,
                "invoke": coordinate(function, invoke)? })
        }
        FinalizedObjectReturnOriginV1::BorrowedCall {
            producer,
            invoke,
            formal_ordinal,
        } => json!({ "kind": "borrowed_call", "producer": coordinate(function, producer)?,
                "invoke": coordinate(function, invoke)?, "formal_ordinal": formal_ordinal }),
        FinalizedObjectReturnOriginV1::MapSlot(source) => {
            let crate::mir::instruction::BorrowedResultSupportV1::ReceiverFieldSlot { field, key_ordinal, object } = source.support()
                else { return Err(fault("object-return/json-source-missing")); };
            let mut encoded = json!({ "kind": "borrowed_map_slot", "producer": coordinate(function, source.producer())?,
                "invoke": coordinate(function, source.invoke())?,
                "field": { "object_id": field.object().declaration_index(), "field_ordinal": field.declaration_ordinal() },
                "key_ordinal": key_ordinal, "object_id": object.declaration_index() });
            if let Some((normal, end)) = source.installed_bindings() {
                encoded["installed"] = json!({"normal":coordinate(function, normal)?, "end":coordinate(function, end)?});
            }
            if let Some(rows) = source.present_bindings() {
                encoded["present"] = json!({"invoke":coordinate(function, &rows[0])?,
                    "normal":coordinate(function, &rows[1])?,
                    "copies":rows[2..rows.len() - 1].iter().map(|row| coordinate(function, row))
                        .collect::<Result<Vec<_>, _>>()?,
                    "branch":coordinate(function, &rows[rows.len() - 1])?});
            }
            encoded
        }
        FinalizedObjectReturnOriginV1::BorrowedFormal { formal_ordinal, .. } => {
            json!({ "kind": "borrowed_formal", "formal_ordinal": formal_ordinal })
        }
    };
    wire["copies"] = Value::Array(
        row.transport_bindings()
            .iter()
            .map(|binding| coordinate(function, binding))
            .collect::<Result<Vec<_>, _>>()?,
    );
    if row.has_field_supports() {
        wire["field_supports"] = Value::Array(row.field_supports().iter().map(|support| {
            let (field, child, constructor_ordinal, outer_ordinal) = support.field_dependency()?;
            Ok(json!({
                "field": { "object_id": field.object().declaration_index(),
                    "field_ordinal": field.declaration_ordinal(),
                    "child_object_id": child.declaration_index(),
                    "constructor_formal_ordinal": constructor_ordinal },
                "formal_ordinal": outer_ordinal,
                "entry": support.entry().0,
                "copies": support.copies().iter().map(|binding| coordinate(function, binding))
                    .collect::<Result<Vec<_>, String>>()?,
                "call": { "invoke": coordinate(function, &support.call()[0])?,
                    "producer": coordinate(function, &support.call()[1])?,
                    "formal_ordinal": support.callee_ordinal() },
                "construction": support.construction().map(|binding| coordinate(function, binding)).transpose()?,
            }))
        }).collect::<Result<Vec<_>, String>>()?);
    }
    Ok(Some(wire))
}
