//! Canonical Array mutation vocabulary owner.
//!
//! This module classifies and validates write shape. It does not activate
//! Typed `Array<T>`, infer identity from representation facts, or implement
//! Array storage semantics.

use std::collections::{BTreeMap, BTreeSet};

use crate::mir::function::{
    ArrayElementWriteWitness, ArrayStateTerm, ArrayStateTermId, ArrayStateTermKind,
};
use crate::mir::{
    ArrayElementWriteKind, ArrayWriteProducerKind, ArrayWriteSiteId, Callee, EffectMask,
    MirFunction, MirInstruction, MirModule, ValueId,
};
use hakorune_mir_defs::{CalleeBoxKind, TypeCertainty};

pub(crate) const UNCLASSIFIED_SURFACE_TAG: &str = "[mir/array_write/unclassified_surface]";
pub(crate) const INVALID_SHAPE_TAG: &str = "[mir/array_write/invalid_shape]";
pub(crate) const RESIDUAL_CALL_TAG: &str = "[mir/array_write/residual_call]";
pub(crate) const IDENTITY_MISSING_TAG: &str = "[mir/array_write/identity_missing]";
pub(crate) const IDENTITY_DRIFT_TAG: &str = "[mir/array_write/identity_drift]";
pub(crate) const REPRESENTATION_AS_IDENTITY_TAG: &str =
    "[mir/array_write/representation_as_identity]";
pub(crate) const PLANNER_BYPASS_TAG: &str = "[mir/array_write/planner_bypass]";
pub(crate) const COVERED_SITE_DRIFT_TAG: &str = "[mir/array_write/covered_site_drift]";
pub(crate) const OVERLAPPING_ROUTES_TAG: &str = "[mir/array_write/overlapping_selected_routes]";
pub(crate) const PROJECTION_DRIFT_TAG: &str = "[mir/array_write/projection_drift]";
pub(crate) const BACKEND_UNSUPPORTED_TAG: &str = "[mir/array_write/backend_unsupported]";
pub(crate) const RAW_RUNTIME_BYPASS_TAG: &str = "[mir/array_write/raw_runtime_bypass]";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ArrayElementWriteBackendMode {
    NativeV1,
    ValidatedLegacyCallProjectionV1,
    Unsupported,
}

pub(crate) fn backend_mode(backend: &str) -> ArrayElementWriteBackendMode {
    match backend {
        "vm" | "mir-interpreter" => ArrayElementWriteBackendMode::NativeV1,
        "ny-llvmc-exe" | "ny-llvmc-obj" | "llvmlite-obj" | "pyvm-harness" | "llvm-legacy-obj"
        | "llvm-mock-fallback" => ArrayElementWriteBackendMode::ValidatedLegacyCallProjectionV1,
        _ => ArrayElementWriteBackendMode::Unsupported,
    }
}

pub(crate) fn enforce_backend_supported(module: &MirModule, backend: &str) -> Result<(), String> {
    let has_writes = module.functions.values().any(|function| {
        function
            .blocks
            .values()
            .flat_map(|block| block.instructions.iter())
            .any(|instruction| matches!(instruction, MirInstruction::ArrayElementWrite { .. }))
    });
    if has_writes && backend_mode(backend) == ArrayElementWriteBackendMode::Unsupported {
        Err(format!("{} backend={backend}", BACKEND_UNSUPPORTED_TAG))
    } else {
        Ok(())
    }
}

pub(crate) fn instruction(
    site_id: ArrayWriteSiteId,
    dst: Option<ValueId>,
    kind: ArrayElementWriteKind,
    producer: ArrayWriteProducerKind,
    receiver: ValueId,
    index: Option<ValueId>,
    value: ValueId,
) -> Result<MirInstruction, String> {
    validate_shape(kind, index)?;
    Ok(MirInstruction::ArrayElementWrite {
        site_id,
        dst,
        kind,
        producer,
        receiver,
        index,
        value,
    })
}

pub(crate) fn validate_shape(
    kind: ArrayElementWriteKind,
    index: Option<ValueId>,
) -> Result<(), String> {
    let valid = match kind {
        ArrayElementWriteKind::LiteralAppend | ArrayElementWriteKind::Push => index.is_none(),
        ArrayElementWriteKind::Set | ArrayElementWriteKind::Insert => index.is_some(),
    };
    if valid {
        Ok(())
    } else {
        Err(format!("{} kind={}", INVALID_SHAPE_TAG, kind.as_str()))
    }
}

pub(crate) fn refresh_function_array_write_witnesses(
    function: &mut MirFunction,
) -> Result<(), String> {
    let (witnesses, terms) = rebuild(function)?;
    function.metadata.array_element_write_witnesses = witnesses;
    function.metadata.array_state_terms = terms;
    Ok(())
}

pub(crate) fn canonicalize_legacy_array_write_calls(
    function: &mut MirFunction,
) -> Result<(), String> {
    let mut next_site = function
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .filter_map(|instruction| match instruction {
            MirInstruction::ArrayElementWrite { site_id, .. } => Some(site_id.0 + 1),
            _ => None,
        })
        .max()
        .unwrap_or(0);
    for instruction in function
        .blocks
        .values_mut()
        .flat_map(|block| block.instructions.iter_mut())
    {
        let MirInstruction::Call {
            dst,
            callee:
                Some(Callee::Method {
                    box_name,
                    method,
                    receiver: Some(receiver),
                    ..
                }),
            args,
            ..
        } = instruction
        else {
            continue;
        };
        if box_name != "ArrayBox" {
            continue;
        }
        let Some(method_id) =
            crate::boxes::array::ArrayMethodId::from_name_and_arity(method, args.len())
        else {
            continue;
        };
        let (kind, index, value) = match method_id {
            crate::boxes::array::ArrayMethodId::Push => {
                (ArrayElementWriteKind::Push, None, args[0])
            }
            crate::boxes::array::ArrayMethodId::Set => {
                (ArrayElementWriteKind::Set, Some(args[0]), args[1])
            }
            crate::boxes::array::ArrayMethodId::Insert => {
                (ArrayElementWriteKind::Insert, Some(args[0]), args[1])
            }
            _ => continue,
        };
        *instruction = self::instruction(
            ArrayWriteSiteId::new(next_site),
            *dst,
            kind,
            ArrayWriteProducerKind::LegacyCanonicalized,
            *receiver,
            index,
            value,
        )?;
        next_site += 1;
    }
    Ok(())
}

pub(crate) fn validate_function_array_write_witnesses(
    function: &MirFunction,
) -> Result<(), String> {
    let (witnesses, terms) = rebuild(function)?;
    if witnesses != function.metadata.array_element_write_witnesses
        || terms != function.metadata.array_state_terms
    {
        return Err(format!(
            "{} function={} rebuilt_witnesses={} carried_witnesses={}",
            IDENTITY_DRIFT_TAG,
            function.signature.name,
            witnesses.len(),
            function.metadata.array_element_write_witnesses.len()
        ));
    }
    validate_planner_references(function)
}

fn validate_planner_references(function: &MirFunction) -> Result<(), String> {
    for route in &function.metadata.generic_method_routes {
        let Some(site_id) = route.array_write_site_id() else {
            continue;
        };
        let instruction = function
            .blocks
            .get(&route.block())
            .and_then(|block| block.instructions.get(route.instruction_index()));
        let Some(MirInstruction::ArrayElementWrite {
            site_id: actual,
            receiver,
            index,
            ..
        }) = instruction
        else {
            return Err(format!("{} site={}", COVERED_SITE_DRIFT_TAG, site_id.0));
        };
        if *actual != site_id || *receiver != route.receiver_value() || *index != route.key_value()
        {
            return Err(format!("{} site={}", COVERED_SITE_DRIFT_TAG, site_id.0));
        }
    }
    for route in &function.metadata.array_rmw_window_routes {
        validate_covered_site(
            function,
            route.block(),
            route.set_instruction_index(),
            route.array_write_site_id(),
        )?;
    }
    for route in &function.metadata.array_text_edit_routes {
        validate_covered_site(
            function,
            route.block(),
            route.set_instruction_index(),
            route.array_write_site_id(),
        )?;
    }
    Ok(())
}

fn validate_covered_site(
    function: &MirFunction,
    block: crate::mir::BasicBlockId,
    instruction_index: usize,
    expected: ArrayWriteSiteId,
) -> Result<(), String> {
    match function
        .blocks
        .get(&block)
        .and_then(|block| block.instructions.get(instruction_index))
    {
        Some(MirInstruction::ArrayElementWrite { site_id, .. }) if *site_id == expected => Ok(()),
        _ => Err(format!("{} site={}", COVERED_SITE_DRIFT_TAG, expected.0)),
    }
}

/// Owner-controlled compatibility projection for backends that have not yet
/// learned the V1 operation. Callers must run semantic refresh and backend
/// preflight before using the returned clone.
pub(crate) fn project_module_to_legacy_calls(module: &MirModule) -> Result<MirModule, String> {
    let mut projected = module.clone();
    for function in projected.functions.values_mut() {
        validate_function_array_write_witnesses(function)?;
        for instruction in function
            .blocks
            .values_mut()
            .flat_map(|block| block.instructions.iter_mut())
        {
            let MirInstruction::ArrayElementWrite {
                dst,
                kind,
                receiver,
                index,
                value,
                ..
            } = instruction
            else {
                continue;
            };
            let (method, args) = match kind {
                ArrayElementWriteKind::LiteralAppend | ArrayElementWriteKind::Push => {
                    ("push", vec![*value])
                }
                ArrayElementWriteKind::Set => (
                    "set",
                    vec![
                        index.ok_or_else(|| PROJECTION_DRIFT_TAG.to_string())?,
                        *value,
                    ],
                ),
                ArrayElementWriteKind::Insert => (
                    "insert",
                    vec![
                        index.ok_or_else(|| PROJECTION_DRIFT_TAG.to_string())?,
                        *value,
                    ],
                ),
            };
            *instruction = MirInstruction::Call {
                dst: *dst,
                func: ValueId::INVALID,
                callee: Some(Callee::Method {
                    box_name: "ArrayBox".to_string(),
                    method: method.to_string(),
                    receiver: Some(*receiver),
                    certainty: TypeCertainty::Known,
                    box_kind: CalleeBoxKind::RuntimeData,
                }),
                args,
                effects: EffectMask::WRITE,
            };
        }
    }
    Ok(projected)
}

fn rebuild(
    function: &MirFunction,
) -> Result<(Vec<ArrayElementWriteWitness>, Vec<ArrayStateTerm>), String> {
    reject_residual_calls(function)?;
    let definitions = value_definitions(function);
    let mut writes = function
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .filter_map(|instruction| match instruction {
            MirInstruction::ArrayElementWrite {
                site_id,
                kind,
                producer,
                receiver,
                index,
                value,
                ..
            } => Some((*site_id, *kind, *producer, *receiver, *index, *value)),
            _ => None,
        })
        .collect::<Vec<_>>();
    writes.sort_by_key(|write| write.0);

    let mut seen_sites = BTreeSet::new();
    let mut term_by_value = BTreeMap::new();
    let mut terms = Vec::new();
    let mut claim_values = function
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .filter_map(|instruction| match instruction {
            MirInstruction::ArrayStateContractClaim { array, .. } => Some(*array),
            _ => None,
        })
        .collect::<BTreeSet<_>>();
    claim_values.extend(function.metadata.typed_array_contract_sources.iter().filter_map(
        |source| match source.boundary_value {
            crate::mir::function::TypedArrayBoundaryValue::Value(value) => Some(value),
            crate::mir::function::TypedArrayBoundaryValue::FinalReturn => None,
        },
    ));
    for value in claim_values {
        let term_id = ArrayStateTermId(terms.len() as u32);
        terms.push(ArrayStateTerm {
            term_id,
            value,
            kind: classify_state_term(value, &definitions),
        });
        term_by_value.insert(value, term_id);
    }
    let mut witnesses = Vec::with_capacity(writes.len());
    for (site_id, kind, producer, receiver, index, value) in writes {
        if !seen_sites.insert(site_id) {
            return Err(format!(
                "{} duplicate_site={}",
                IDENTITY_DRIFT_TAG, site_id.0
            ));
        }
        validate_shape(kind, index)?;
        let state_term = if let Some(term_id) = term_by_value.get(&receiver) {
            *term_id
        } else {
            let term_id = ArrayStateTermId(terms.len() as u32);
            terms.push(ArrayStateTerm {
                term_id,
                value: receiver,
                kind: classify_state_term(receiver, &definitions),
            });
            term_by_value.insert(receiver, term_id);
            term_id
        };
        witnesses.push(ArrayElementWriteWitness {
            site_id,
            kind,
            producer,
            receiver,
            index,
            value,
            state_term,
        });
    }
    Ok((witnesses, terms))
}

fn value_definitions(function: &MirFunction) -> BTreeMap<ValueId, &MirInstruction> {
    function
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .filter_map(|instruction| instruction.dst_value().map(|dst| (dst, instruction)))
        .collect()
}

fn classify_state_term(
    value: ValueId,
    definitions: &BTreeMap<ValueId, &MirInstruction>,
) -> ArrayStateTermKind {
    match definitions.get(&value).copied() {
        Some(MirInstruction::NewBox { dst, box_type, .. }) if box_type == "ArrayBox" => {
            ArrayStateTermKind::Fresh {
                allocation_site: *dst,
            }
        }
        Some(MirInstruction::Copy { src, .. })
        | Some(MirInstruction::LocalContractWrite { src, .. }) => {
            ArrayStateTermKind::SameAs { source: *src }
        }
        Some(MirInstruction::Phi { inputs, .. }) => ArrayStateTermKind::Select {
            inputs: inputs.iter().map(|(_, value)| *value).collect(),
        },
        Some(MirInstruction::Select {
            then_val, else_val, ..
        }) => ArrayStateTermKind::Select {
            inputs: vec![*then_val, *else_val],
        },
        _ => ArrayStateTermKind::DynamicBoundary { value },
    }
}

fn reject_residual_calls(function: &MirFunction) -> Result<(), String> {
    for instruction in function
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
    {
        let MirInstruction::Call {
            callee: Some(Callee::Method {
                box_name, method, ..
            }),
            args,
            ..
        } = instruction
        else {
            continue;
        };
        if box_name == "ArrayBox"
            && crate::boxes::array::ArrayMethodId::from_name_and_arity(method, args.len())
                .is_some_and(|method| {
                    matches!(
                        method,
                        crate::boxes::array::ArrayMethodId::Push
                            | crate::boxes::array::ArrayMethodId::Set
                            | crate::boxes::array::ArrayMethodId::Insert
                    )
                })
        {
            return Err(format!("{} method={}", RESIDUAL_CALL_TAG, method));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{BasicBlockId, FunctionSignature, MirType};

    #[test]
    fn owner_enforces_kind_index_shape() {
        assert!(instruction(
            ArrayWriteSiteId::new(1),
            None,
            ArrayElementWriteKind::Push,
            ArrayWriteProducerKind::MethodCall,
            ValueId::new(0),
            None,
            ValueId::new(1),
        )
        .is_ok());
        let error = instruction(
            ArrayWriteSiteId::new(2),
            None,
            ArrayElementWriteKind::Set,
            ArrayWriteProducerKind::IndexAssignment,
            ValueId::new(0),
            None,
            ValueId::new(1),
        )
        .unwrap_err();
        assert!(error.contains(INVALID_SHAPE_TAG));
    }

    #[test]
    fn owner_projects_one_validated_write_to_one_legacy_call() {
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "main".to_string(),
                params: vec![],
                return_type: MirType::Void,
                effects: EffectMask::WRITE,
            },
            BasicBlockId::new(0),
        );
        function
            .get_block_mut(BasicBlockId::new(0))
            .unwrap()
            .add_instruction(
                instruction(
                    ArrayWriteSiteId::new(7),
                    None,
                    ArrayElementWriteKind::Set,
                    ArrayWriteProducerKind::IndexAssignment,
                    ValueId::new(1),
                    Some(ValueId::new(2)),
                    ValueId::new(3),
                )
                .unwrap(),
            );
        refresh_function_array_write_witnesses(&mut function).unwrap();
        let mut module = MirModule::new("projection".to_string());
        module.add_function(function);

        let projected = project_module_to_legacy_calls(&module).unwrap();
        let instruction =
            &projected.functions["main"].blocks[&BasicBlockId::new(0)].instructions[0];
        assert!(matches!(
            instruction,
            MirInstruction::Call {
                callee: Some(Callee::Method { method, .. }),
                args,
                ..
            } if method == "set" && args == &vec![ValueId::new(2), ValueId::new(3)]
        ));
        assert!(matches!(
            module.functions["main"].blocks[&BasicBlockId::new(0)].instructions[0],
            MirInstruction::ArrayElementWrite { .. }
        ));
    }
}
