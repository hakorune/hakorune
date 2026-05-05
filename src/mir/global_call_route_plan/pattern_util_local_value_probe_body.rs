use crate::mir::{BinaryOp, Callee, ConstValue, MirFunction, MirInstruction, MirType};
use std::collections::BTreeMap;

use super::generic_string_abi::generic_pure_string_abi_type_is_handle_compatible;
use super::generic_string_surface::generic_pure_string_global_name_is_self;
use super::{
    lookup_global_call_target, GlobalCallProof, GlobalCallReturnContract, GlobalCallTargetFacts,
    GlobalCallTargetShape,
};

pub(super) fn is_pattern_util_local_value_probe_body_function(
    function: &MirFunction,
    targets: &BTreeMap<String, GlobalCallTargetFacts>,
) -> bool {
    if function.params.len() != function.signature.params.len() {
        return false;
    }
    if function.signature.params.len() != 3 {
        return false;
    }
    if !pattern_util_local_value_probe_return_type_is_abi_compatible(
        &function.signature.return_type,
    ) {
        return false;
    }
    if !function
        .signature
        .params
        .iter()
        .all(generic_pure_string_abi_type_is_handle_compatible)
    {
        return false;
    }

    let mut facts = PatternUtilLocalValueProbeFacts::default();
    for block in function.blocks.values() {
        for instruction in block.instructions.iter().chain(block.terminator.iter()) {
            facts.observe(&function.signature.name, instruction, targets);
        }
    }
    facts.is_local_value_probe_shape()
}

fn pattern_util_local_value_probe_return_type_is_abi_compatible(ty: &MirType) -> bool {
    matches!(
        ty,
        MirType::Integer | MirType::Bool | MirType::Unknown | MirType::Void
    )
}

#[derive(Default)]
struct PatternUtilLocalValueProbeFacts {
    local_marker: bool,
    name_marker: bool,
    value_marker: bool,
    int_marker: bool,
    bool_marker: bool,
    binary_marker: bool,
    compare_marker: bool,
    void_sentinel_const: bool,
    returns_value: bool,
    index_of_from_calls: usize,
    read_string_after_calls: usize,
    read_int_after_calls: usize,
    read_bool_after_calls: usize,
    self_probe_calls: usize,
    child_probe_calls: usize,
    scalar_coerce_calls: usize,
    arithmetic_ops: usize,
    compare_ops: usize,
}

impl PatternUtilLocalValueProbeFacts {
    fn observe(
        &mut self,
        current_function_name: &str,
        instruction: &MirInstruction,
        targets: &BTreeMap<String, GlobalCallTargetFacts>,
    ) {
        match instruction {
            MirInstruction::Const { value, .. } => self.observe_const(value),
            MirInstruction::BinOp { op, .. } => {
                if matches!(
                    op,
                    BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div
                ) {
                    self.arithmetic_ops += 1;
                }
            }
            MirInstruction::Compare { .. } => self.compare_ops += 1,
            MirInstruction::Call {
                callee: Some(Callee::Global(name)),
                args,
                ..
            } => self.observe_global_call(current_function_name, name, args.len(), targets),
            MirInstruction::Return { value: Some(_), .. } => self.returns_value = true,
            _ => {}
        }
    }

    fn observe_const(&mut self, value: &ConstValue) {
        match value {
            ConstValue::String(text) => {
                self.local_marker |= text.contains("\"type\":\"Local\"");
                self.name_marker |= text.contains("\"name\":\"") || text.contains("\"name\":");
                self.value_marker |= text.contains("\"value\":");
                self.int_marker |= text.contains("\"expr\":{\"type\":\"Int\"")
                    || text.contains("\"type\":\"Int\"");
                self.bool_marker |= text.contains("\"expr\":{\"type\":\"Bool\"")
                    || text.contains("\"type\":\"Bool\"");
                self.binary_marker |= text.contains("\"expr\":{\"type\":\"Binary\"")
                    || text.contains("\"type\":\"Binary\"");
                self.compare_marker |= text.contains("\"expr\":{\"type\":\"Compare\"")
                    || text.contains("\"type\":\"Compare\"");
            }
            ConstValue::Null | ConstValue::Void => self.void_sentinel_const = true,
            _ => {}
        }
    }

    fn observe_global_call(
        &mut self,
        current_function_name: &str,
        name: &str,
        arity: usize,
        targets: &BTreeMap<String, GlobalCallTargetFacts>,
    ) {
        let normalized_name = crate::mir::naming::normalize_static_global_name(name);
        match normalized_name.as_str() {
            "JsonFragBox.index_of_from/3" => self.index_of_from_calls += 1,
            "JsonFragBox.read_string_after/2" => self.read_string_after_calls += 1,
            "JsonFragBox.read_int_after/2" => self.read_int_after_calls += 1,
            "JsonFragBox.read_bool_after/2" => self.read_bool_after_calls += 1,
            _ => {}
        }

        if arity == 3 && generic_pure_string_global_name_is_self(name, current_function_name) {
            self.self_probe_calls += 1;
            return;
        }

        let Some(target) = lookup_global_call_target(name, targets) else {
            return;
        };
        if arity == 3 && target_is_pattern_util_local_value_probe(target) {
            self.child_probe_calls += 1;
        } else if arity == 1 && target.shape() == GlobalCallTargetShape::GenericI64Body {
            self.scalar_coerce_calls += 1;
        }
    }

    fn is_local_value_probe_shape(&self) -> bool {
        self.returns_value
            && self.void_sentinel_const
            && self.local_marker
            && self.name_marker
            && self.value_marker
            && self.index_of_from_calls >= 3
            && self.read_string_after_calls >= 1
            && self.read_int_after_calls >= 1
            && (self.is_int_value_probe_shape() || self.is_bool_value_probe_shape())
    }

    fn is_int_value_probe_shape(&self) -> bool {
        self.int_marker
            && self.binary_marker
            && self.self_probe_calls >= 1
            && self.scalar_coerce_calls >= 1
            && self.arithmetic_ops >= 1
    }

    fn is_bool_value_probe_shape(&self) -> bool {
        self.bool_marker
            && self.compare_marker
            && self.read_bool_after_calls >= 1
            && self.child_probe_calls >= 1
            && self.scalar_coerce_calls >= 1
            && self.compare_ops >= 1
    }
}

fn target_is_pattern_util_local_value_probe(target: &GlobalCallTargetFacts) -> bool {
    target.proof() == GlobalCallProof::PatternUtilLocalValueProbe
        && target.return_contract() == Some(GlobalCallReturnContract::MixedRuntimeI64OrHandle)
}
