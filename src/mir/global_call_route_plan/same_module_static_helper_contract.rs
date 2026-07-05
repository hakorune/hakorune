use std::collections::BTreeMap;

use crate::mir::generic_method_route_facts::GenericMethodReturnShape;
use crate::mir::generic_method_route_plan::GenericMethodRouteKind;
use crate::mir::{BasicBlockId, ConstValue, MirFunction, MirInstruction, MirType, ValueId};

use super::model::{GlobalCallProof, GlobalCallReturnContract};

pub(super) fn infer_same_module_static_helper_return_contract(
    function: &MirFunction,
    typed_plan_type_ids: &BTreeMap<String, u32>,
) -> Option<(GlobalCallProof, GlobalCallReturnContract)> {
    if same_module_static_helper_legacy_no_new_consumer(&function.signature.name) {
        return None;
    }
    let mut inferred = same_module_static_helper_return_type_contract(
        &function.signature.return_type,
        typed_plan_type_ids,
    );
    let mut copy_sources = BTreeMap::new();
    let mut result_contracts = BTreeMap::new();

    for route in &function.metadata.user_box_method_routes {
        if route.reason().is_none() {
            if let Some(value) = route.result_value() {
                if let Some(contract) =
                    same_module_static_helper_route_return_contract(route.return_shape())
                {
                    result_contracts.insert(value, contract);
                }
            }
        }
    }
    for route in &function.metadata.generic_method_routes {
        if let Some(value) = route.result_value() {
            if let Some(contract) = same_module_static_helper_generic_route_return_contract(
                route.return_shape(),
                route.route_kind(),
            ) {
                result_contracts.insert(value, contract);
            }
        }
    }

    for block in function.blocks.values() {
        for instruction in block.instructions.iter().chain(block.terminator.iter()) {
            match instruction {
                MirInstruction::Copy { dst, src } => {
                    copy_sources.insert(*dst, *src);
                }
                MirInstruction::Const { dst, value } => {
                    if let Some(contract) = same_module_static_helper_const_return_contract(value) {
                        result_contracts.insert(*dst, contract);
                    }
                }
                MirInstruction::NewBox { dst, box_type, .. } => {
                    if let Some(contract) =
                        same_module_static_helper_box_return_contract(box_type, typed_plan_type_ids)
                    {
                        result_contracts.insert(*dst, contract);
                    }
                }
                MirInstruction::VariantMake { dst, .. } => {
                    result_contracts.insert(*dst, GlobalCallReturnContract::ObjectHandle);
                }
                _ => {}
            }
        }
    }

    for _ in 0..32 {
        let mut changed = false;
        for block in function.blocks.values() {
            for instruction in block.instructions.iter().chain(block.terminator.iter()) {
                let MirInstruction::Phi { dst, inputs, .. } = instruction else {
                    continue;
                };
                if result_contracts.contains_key(dst) {
                    continue;
                }
                let Some(contract) = same_module_static_helper_phi_contract(
                    inputs,
                    typed_plan_type_ids,
                    &copy_sources,
                    &result_contracts,
                    &function.metadata.value_types,
                )?
                else {
                    continue;
                };
                result_contracts.insert(*dst, contract);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }

    for block in function.blocks.values() {
        for instruction in block.instructions.iter().chain(block.terminator.iter()) {
            let MirInstruction::Return { value } = instruction else {
                continue;
            };
            let contract = match value {
                Some(value) => same_module_static_helper_value_contract(
                    *value,
                    typed_plan_type_ids,
                    &copy_sources,
                    &result_contracts,
                    &function.metadata.value_types,
                ),
                None => Some(GlobalCallReturnContract::VoidSentinelI64Zero),
            };
            inferred = merge_same_module_static_helper_contract(inferred, contract)?;
        }
    }

    inferred.map(|contract| (same_module_static_helper_contract_proof(contract), contract))
}

pub(super) fn same_module_static_helper_contract_allowed(
    function: &MirFunction,
    contract: GlobalCallReturnContract,
    _typed_plan_type_ids: &BTreeMap<String, u32>,
) -> bool {
    match contract {
        GlobalCallReturnContract::ObjectHandle => {
            matches!(
                function.signature.return_type,
                MirType::Box(_) | MirType::Unknown
            )
        }
        GlobalCallReturnContract::MapHandle => {
            matches!(
                function.signature.return_type,
                MirType::Box(_) | MirType::Unknown
            )
        }
        GlobalCallReturnContract::MixedRuntimeI64OrHandle => {
            matches!(
                function.signature.return_type,
                MirType::Void | MirType::Unknown
            )
        }
        GlobalCallReturnContract::ScalarI64 => true,
        GlobalCallReturnContract::VoidSentinelI64Zero => {
            matches!(function.signature.return_type, MirType::Void)
        }
        _ => false,
    }
}

fn same_module_static_helper_phi_contract(
    inputs: &[(BasicBlockId, ValueId)],
    typed_plan_type_ids: &BTreeMap<String, u32>,
    copy_sources: &BTreeMap<ValueId, ValueId>,
    result_contracts: &BTreeMap<ValueId, GlobalCallReturnContract>,
    value_types: &BTreeMap<ValueId, MirType>,
) -> Option<Option<GlobalCallReturnContract>> {
    let mut out = None;
    for (_, input) in inputs {
        let input_contract = same_module_static_helper_value_contract(
            *input,
            typed_plan_type_ids,
            copy_sources,
            result_contracts,
            value_types,
        );
        if input_contract.is_none() {
            return Some(None);
        }
        out = merge_same_module_static_helper_contract(out, input_contract)?;
    }
    Some(out)
}

fn same_module_static_helper_return_type_contract(
    return_type: &MirType,
    typed_plan_type_ids: &BTreeMap<String, u32>,
) -> Option<GlobalCallReturnContract> {
    match return_type {
        MirType::Integer | MirType::Bool => Some(GlobalCallReturnContract::ScalarI64),
        MirType::Void => Some(GlobalCallReturnContract::VoidSentinelI64Zero),
        MirType::Box(name) => {
            same_module_static_helper_box_return_contract(name, typed_plan_type_ids)
        }
        _ => None,
    }
}

fn same_module_static_helper_box_return_contract(
    box_name: &str,
    typed_plan_type_ids: &BTreeMap<String, u32>,
) -> Option<GlobalCallReturnContract> {
    match box_name {
        "MapBox" => Some(GlobalCallReturnContract::MapHandle),
        "ArrayBox" | "DirectArrayI64" => Some(GlobalCallReturnContract::ObjectHandle),
        _ if typed_plan_type_ids.contains_key(box_name) => {
            Some(GlobalCallReturnContract::ObjectHandle)
        }
        _ => None,
    }
}

fn same_module_static_helper_legacy_no_new_consumer(symbol: &str) -> bool {
    matches!(
        symbol,
        "ProgramJsonV0ScannerBox.read_int_field_in_obj/3"
            | "ProgramJsonV0ScannerBox.read_string_field_last_in_obj/3"
    )
}

fn same_module_static_helper_route_return_contract(
    return_shape: Option<&str>,
) -> Option<GlobalCallReturnContract> {
    match return_shape {
        Some("scalar_i64") => Some(GlobalCallReturnContract::ScalarI64),
        Some("void_sentinel_i64_zero") => Some(GlobalCallReturnContract::VoidSentinelI64Zero),
        Some("mixed_runtime_i64_or_handle") => {
            Some(GlobalCallReturnContract::MixedRuntimeI64OrHandle)
        }
        Some("object_handle") => Some(GlobalCallReturnContract::ObjectHandle),
        Some("map_handle") => Some(GlobalCallReturnContract::MapHandle),
        _ => None,
    }
}

fn same_module_static_helper_generic_route_return_contract(
    return_shape: Option<GenericMethodReturnShape>,
    route_kind: GenericMethodRouteKind,
) -> Option<GlobalCallReturnContract> {
    match return_shape {
        Some(GenericMethodReturnShape::ScalarI64)
        | Some(GenericMethodReturnShape::ScalarI64OrMissingZero) => {
            Some(GlobalCallReturnContract::ScalarI64)
        }
        Some(GenericMethodReturnShape::MixedRuntimeI64OrHandle) => {
            Some(GlobalCallReturnContract::MixedRuntimeI64OrHandle)
        }
        None => match route_kind {
            GenericMethodRouteKind::MapLoadAny
            | GenericMethodRouteKind::MapLoadI64Any
            | GenericMethodRouteKind::RuntimeDataLoadAny
            | GenericMethodRouteKind::ArraySlotLoadAny => {
                Some(GlobalCallReturnContract::MixedRuntimeI64OrHandle)
            }
            _ => None,
        },
    }
}

fn same_module_static_helper_const_return_contract(
    value: &ConstValue,
) -> Option<GlobalCallReturnContract> {
    match value {
        ConstValue::Integer(_) | ConstValue::Bool(_) => Some(GlobalCallReturnContract::ScalarI64),
        ConstValue::Void => Some(GlobalCallReturnContract::VoidSentinelI64Zero),
        ConstValue::Null => Some(GlobalCallReturnContract::MixedRuntimeI64OrHandle),
        _ => None,
    }
}

fn same_module_static_helper_value_contract(
    value: ValueId,
    typed_plan_type_ids: &BTreeMap<String, u32>,
    copy_sources: &BTreeMap<ValueId, ValueId>,
    result_contracts: &BTreeMap<ValueId, GlobalCallReturnContract>,
    value_types: &BTreeMap<ValueId, MirType>,
) -> Option<GlobalCallReturnContract> {
    let mut current = value;
    for _ in 0..32 {
        if let Some(contract) = result_contracts.get(&current) {
            return Some(*contract);
        }
        if let Some(contract) = value_types
            .get(&current)
            .and_then(|ty| same_module_static_helper_return_type_contract(ty, typed_plan_type_ids))
        {
            return Some(contract);
        }
        let Some(next) = copy_sources.get(&current).copied() else {
            return None;
        };
        if next == current {
            return None;
        }
        current = next;
    }
    None
}

fn merge_same_module_static_helper_contract(
    current: Option<GlobalCallReturnContract>,
    next: Option<GlobalCallReturnContract>,
) -> Option<Option<GlobalCallReturnContract>> {
    match (current, next) {
        (None, Some(next)) => Some(Some(next)),
        (Some(current), Some(next)) if current == next => Some(Some(current)),
        (
            Some(GlobalCallReturnContract::MixedRuntimeI64OrHandle),
            Some(GlobalCallReturnContract::VoidSentinelI64Zero),
        )
        | (
            Some(GlobalCallReturnContract::MixedRuntimeI64OrHandle),
            Some(GlobalCallReturnContract::ScalarI64),
        )
        | (
            Some(GlobalCallReturnContract::MixedRuntimeI64OrHandle),
            Some(GlobalCallReturnContract::ObjectHandle),
        )
        | (
            Some(GlobalCallReturnContract::MixedRuntimeI64OrHandle),
            Some(GlobalCallReturnContract::MapHandle),
        )
        | (
            Some(GlobalCallReturnContract::VoidSentinelI64Zero),
            Some(GlobalCallReturnContract::MixedRuntimeI64OrHandle),
        ) => Some(Some(GlobalCallReturnContract::MixedRuntimeI64OrHandle)),
        (
            Some(GlobalCallReturnContract::ScalarI64),
            Some(GlobalCallReturnContract::MixedRuntimeI64OrHandle),
        )
        | (
            Some(GlobalCallReturnContract::ObjectHandle),
            Some(GlobalCallReturnContract::MixedRuntimeI64OrHandle),
        )
        | (
            Some(GlobalCallReturnContract::MapHandle),
            Some(GlobalCallReturnContract::MixedRuntimeI64OrHandle),
        ) => Some(Some(GlobalCallReturnContract::MixedRuntimeI64OrHandle)),
        (Some(GlobalCallReturnContract::ObjectHandle), None) => {
            Some(Some(GlobalCallReturnContract::ObjectHandle))
        }
        (Some(GlobalCallReturnContract::MapHandle), None) => {
            Some(Some(GlobalCallReturnContract::MapHandle))
        }
        (Some(_), None) | (None, None) => None,
        (Some(_), Some(_)) => None,
    }
}

fn same_module_static_helper_contract_proof(contract: GlobalCallReturnContract) -> GlobalCallProof {
    match contract {
        GlobalCallReturnContract::ScalarI64 => GlobalCallProof::SameModuleScalarI64,
        GlobalCallReturnContract::VoidSentinelI64Zero => GlobalCallProof::SameModuleVoidSentinel,
        GlobalCallReturnContract::MapHandle | GlobalCallReturnContract::ObjectHandle => {
            GlobalCallProof::SameModuleObjectHandle
        }
        GlobalCallReturnContract::MixedRuntimeI64OrHandle => {
            GlobalCallProof::SameModuleMixedRuntime
        }
        _ => GlobalCallProof::ContractMissing,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{BasicBlock, EffectMask, FunctionSignature};

    #[test]
    fn infers_object_handle_from_builtin_newbox_with_unknown_signature() {
        let entry = BasicBlockId::new(0);
        let signature = FunctionSignature {
            name: "ArrayFactory.make/0".to_string(),
            params: vec![],
            return_type: MirType::Unknown,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, entry);
        let mut block = BasicBlock::new(entry);
        block.instructions.push(MirInstruction::NewBox {
            dst: ValueId::new(1),
            box_type: "ArrayBox".to_string(),
            args: vec![],
        });
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(1)),
        });
        function.blocks.insert(entry, block);

        let (_proof, contract) =
            infer_same_module_static_helper_return_contract(&function, &BTreeMap::new())
                .expect("object handle contract");

        assert_eq!(contract, GlobalCallReturnContract::ObjectHandle);
        assert!(same_module_static_helper_contract_allowed(
            &function,
            contract,
            &BTreeMap::new()
        ));
    }
}
