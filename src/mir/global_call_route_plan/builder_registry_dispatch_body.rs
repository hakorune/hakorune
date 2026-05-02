use std::collections::BTreeMap;

use crate::mir::{Callee, CompareOp, ConstValue, MirFunction, MirInstruction, MirType};

use super::generic_string_abi::generic_pure_string_abi_type_is_handle_compatible;
use super::generic_string_reject::GenericPureStringReject;
use super::model::{GlobalCallTargetFacts, GlobalCallTargetShapeReason};
use super::string_return_profile::{
    generic_string_void_sentinel_return_candidate,
    generic_string_void_sentinel_return_global_blocker,
};

pub(super) fn builder_registry_dispatch_body_reject_reason(
    function: &MirFunction,
    targets: &BTreeMap<String, GlobalCallTargetFacts>,
) -> Option<GenericPureStringReject> {
    if let Some(reject) = generic_string_void_sentinel_return_global_blocker(function, targets) {
        return Some(reject);
    }
    if !generic_string_void_sentinel_return_candidate(function, targets) {
        return Some(GenericPureStringReject::new(
            GlobalCallTargetShapeReason::GenericStringReturnNotString,
        ));
    }
    None
}

pub(super) fn is_builder_registry_dispatch_body_candidate(function: &MirFunction) -> bool {
    if function.params.len() != function.signature.params.len() {
        return false;
    }
    if !builder_registry_return_type_candidate(&function.signature.return_type) {
        return false;
    }
    if function.signature.params.len() != 1
        || !function
            .signature
            .params
            .iter()
            .all(generic_pure_string_abi_type_is_handle_compatible)
    {
        return false;
    }

    let mut facts = BuilderRegistryDispatchFacts::default();
    for block in function.blocks.values() {
        for instruction in block.instructions.iter().chain(block.terminator.iter()) {
            facts.observe(instruction);
        }
    }
    facts.is_registry_dispatch_shape()
}

fn builder_registry_return_type_candidate(ty: &MirType) -> bool {
    matches!(
        ty,
        MirType::Integer | MirType::Void | MirType::Unknown | MirType::String
    ) || matches!(ty, MirType::Box(name) if name == "StringBox")
}

#[derive(Default)]
struct BuilderRegistryDispatchFacts {
    array_lengths: usize,
    array_gets: usize,
    array_pushes: usize,
    registry_name_consts: usize,
    string_compares: usize,
    try_lower_calls: usize,
    void_sentinel_const: bool,
    returns_value: bool,
}

impl BuilderRegistryDispatchFacts {
    fn observe(&mut self, instruction: &MirInstruction) {
        match instruction {
            MirInstruction::Const { value, .. } => match value {
                ConstValue::String(text) if text.contains('.') => self.registry_name_consts += 1,
                ConstValue::Null | ConstValue::Void => self.void_sentinel_const = true,
                _ => {}
            },
            MirInstruction::Compare { op, .. } if matches!(op, CompareOp::Eq | CompareOp::Ne) => {
                self.string_compares += 1;
            }
            MirInstruction::Call {
                callee:
                    Some(Callee::Method {
                        box_name, method, ..
                    }),
                ..
            } => match (box_name.as_str(), method.as_str()) {
                ("ArrayBox" | "RuntimeDataBox", "length") => self.array_lengths += 1,
                ("ArrayBox" | "RuntimeDataBox", "get") => self.array_gets += 1,
                ("ArrayBox" | "RuntimeDataBox", "push") => self.array_pushes += 1,
                _ => {}
            },
            MirInstruction::Call {
                callee: Some(Callee::Global(name)),
                args,
                ..
            } if args.len() == 1 && name.ends_with(".try_lower/1") => {
                self.try_lower_calls += 1;
            }
            MirInstruction::Return { value: Some(_), .. } => self.returns_value = true,
            _ => {}
        }
    }

    fn is_registry_dispatch_shape(&self) -> bool {
        self.returns_value
            && self.void_sentinel_const
            && self.array_lengths >= 1
            && self.array_gets >= 1
            && self.registry_name_consts >= 1
            && self.string_compares >= 1
            && self.try_lower_calls >= 1
    }
}
