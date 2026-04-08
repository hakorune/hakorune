/*!
 * String corridor facts for canonical MIR.
 *
 * This module keeps no-behavior-change semantic facts about current string
 * lowering carriers. It does not introduce a second MIR dialect; it annotates
 * existing MIR with inventory-friendly facts so later placement/effect passes
 * can make decisions without guessing from helper names.
 */

use super::{MirFunction, MirInstruction, MirModule, ValueId};
use crate::mir::definitions::call_unified::Callee;

/// Canonical string corridor ops we want to reason about in MIR.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringCorridorOp {
    StrSlice,
    StrLen,
    FreezeStr,
}

impl std::fmt::Display for StringCorridorOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StrSlice => f.write_str("str.slice"),
            Self::StrLen => f.write_str("str.len"),
            Self::FreezeStr => f.write_str("freeze.str"),
        }
    }
}

/// Role of the current value inside the string corridor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringCorridorRole {
    BorrowProducer,
    ScalarConsumer,
    BirthSink,
}

impl std::fmt::Display for StringCorridorRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BorrowProducer => f.write_str("borrow_producer"),
            Self::ScalarConsumer => f.write_str("scalar_consumer"),
            Self::BirthSink => f.write_str("birth_sink"),
        }
    }
}

/// Birth / placement outcomes carried by the current SSOT.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringOutcomeFact {
    ReturnHandle,
    BorrowView,
    FreezeOwned,
    FreshHandle,
    MaterializeOwned,
    StoreFromSource,
}

impl std::fmt::Display for StringOutcomeFact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReturnHandle => f.write_str("ReturnHandle"),
            Self::BorrowView => f.write_str("BorrowView"),
            Self::FreezeOwned => f.write_str("FreezeOwned"),
            Self::FreshHandle => f.write_str("FreshHandle"),
            Self::MaterializeOwned => f.write_str("MaterializeOwned"),
            Self::StoreFromSource => f.write_str("StoreFromSource"),
        }
    }
}

/// Boundary-placement fact for objectization / publication / materialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringPlacementFact {
    Unknown,
    None,
    Sink,
    Deferred,
}

impl std::fmt::Display for StringPlacementFact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => f.write_str("?"),
            Self::None => f.write_str("none"),
            Self::Sink => f.write_str("sink"),
            Self::Deferred => f.write_str("deferred"),
        }
    }
}

/// Current lowering carrier that produced the fact.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringCorridorCarrier {
    MethodCall,
    GlobalLoweredFunction,
    RuntimeExport,
    CanonicalIntrinsic,
}

impl std::fmt::Display for StringCorridorCarrier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MethodCall => f.write_str("method_call"),
            Self::GlobalLoweredFunction => f.write_str("global_lowered"),
            Self::RuntimeExport => f.write_str("runtime_export"),
            Self::CanonicalIntrinsic => f.write_str("canonical_intrinsic"),
        }
    }
}

/// No-behavior-change fact carrier for string corridor values in canonical MIR.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringCorridorFact {
    pub op: StringCorridorOp,
    pub role: StringCorridorRole,
    pub carrier: StringCorridorCarrier,
    pub outcome: Option<StringOutcomeFact>,
    pub objectize: StringPlacementFact,
    pub publish: StringPlacementFact,
    pub materialize: StringPlacementFact,
}

impl StringCorridorFact {
    pub fn str_len(carrier: StringCorridorCarrier) -> Self {
        Self {
            op: StringCorridorOp::StrLen,
            role: StringCorridorRole::ScalarConsumer,
            carrier,
            outcome: None,
            objectize: StringPlacementFact::None,
            publish: StringPlacementFact::None,
            materialize: StringPlacementFact::None,
        }
    }

    pub fn str_slice(carrier: StringCorridorCarrier) -> Self {
        Self {
            op: StringCorridorOp::StrSlice,
            role: StringCorridorRole::BorrowProducer,
            carrier,
            outcome: None,
            objectize: StringPlacementFact::Unknown,
            publish: StringPlacementFact::Unknown,
            materialize: StringPlacementFact::Unknown,
        }
    }

    pub fn freeze_str(carrier: StringCorridorCarrier) -> Self {
        Self {
            op: StringCorridorOp::FreezeStr,
            role: StringCorridorRole::BirthSink,
            carrier,
            outcome: Some(StringOutcomeFact::FreezeOwned),
            objectize: StringPlacementFact::Deferred,
            publish: StringPlacementFact::Deferred,
            materialize: StringPlacementFact::Sink,
        }
    }

    pub fn summary(&self) -> String {
        let outcome = self
            .outcome
            .map(|v| v.to_string())
            .unwrap_or_else(|| "?".to_string());
        format!(
            "{} carrier={} role={} outcome={} objectize={} publish={} materialize={}",
            self.op,
            self.carrier,
            self.role,
            outcome,
            self.objectize,
            self.publish,
            self.materialize
        )
    }
}

/// Refresh every function's string corridor facts from the current MIR instruction shape.
pub fn refresh_module_string_corridor_facts(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_string_corridor_facts(function);
    }
}

/// Refresh a single function's string corridor facts.
pub fn refresh_function_string_corridor_facts(function: &mut MirFunction) {
    function.metadata.string_corridor_facts.clear();

    for block in function.blocks.values() {
        for inst in block.instructions.iter().chain(block.terminator.iter()) {
            if let Some((dst, fact)) = infer_fact_from_instruction(inst) {
                function.metadata.string_corridor_facts.insert(dst, fact);
            }
        }
    }
}

fn infer_fact_from_instruction(inst: &MirInstruction) -> Option<(ValueId, StringCorridorFact)> {
    match inst {
        MirInstruction::Call {
            dst: Some(dst),
            callee: Some(Callee::Method {
                box_name, method, ..
            }),
            args,
            ..
        } => infer_from_method(box_name, method, args.len()).map(|fact| (*dst, fact)),
        MirInstruction::Call {
            dst: Some(dst),
            callee: Some(Callee::Global(name)),
            ..
        } => infer_from_global(name).map(|fact| (*dst, fact)),
        MirInstruction::Call {
            dst: Some(dst),
            callee: Some(Callee::Extern(name)),
            ..
        } => infer_from_runtime_export(name).map(|fact| (*dst, fact)),
        _ => None,
    }
}

fn infer_from_method(box_name: &str, method: &str, arity: usize) -> Option<StringCorridorFact> {
    if !is_stringish_box_name(box_name) {
        return None;
    }

    match (method, arity) {
        ("length", 0) | ("len", 0) => Some(StringCorridorFact::str_len(
            StringCorridorCarrier::MethodCall,
        )),
        ("substring", 2) | ("slice", 2) => Some(StringCorridorFact::str_slice(
            StringCorridorCarrier::MethodCall,
        )),
        _ => None,
    }
}

fn infer_from_global(name: &str) -> Option<StringCorridorFact> {
    if matches!(name, "str.len" | "__str.len") {
        return Some(StringCorridorFact::str_len(
            StringCorridorCarrier::CanonicalIntrinsic,
        ));
    }
    if matches!(name, "str.slice" | "__str.slice") {
        return Some(StringCorridorFact::str_slice(
            StringCorridorCarrier::CanonicalIntrinsic,
        ));
    }
    if name == "freeze.str" {
        return Some(StringCorridorFact::freeze_str(
            StringCorridorCarrier::CanonicalIntrinsic,
        ));
    }
    if let Some(fact) = infer_from_runtime_export(name) {
        return Some(fact);
    }

    let (box_name, rest) = name.split_once('.')?;
    if !is_stringish_box_name(box_name) {
        return None;
    }
    let (method, arity) = rest.split_once('/').unwrap_or((rest, ""));
    let arity = arity.parse::<usize>().ok()?;

    match (method, arity) {
        ("length", 0) | ("len", 0) => Some(StringCorridorFact::str_len(
            StringCorridorCarrier::GlobalLoweredFunction,
        )),
        ("substring", 2) | ("slice", 2) => Some(StringCorridorFact::str_slice(
            StringCorridorCarrier::GlobalLoweredFunction,
        )),
        _ => None,
    }
}

fn infer_from_runtime_export(name: &str) -> Option<StringCorridorFact> {
    match name {
        "nyash.string.substring_hii" => Some(StringCorridorFact::str_slice(
            StringCorridorCarrier::RuntimeExport,
        )),
        "nyash.string.substring_len_hii" => Some(StringCorridorFact::str_len(
            StringCorridorCarrier::RuntimeExport,
        )),
        "nyash.string.length_si" | "nyrt_string_length" | "nyrt.string.length" => Some(
            StringCorridorFact::str_len(StringCorridorCarrier::RuntimeExport),
        ),
        _ => None,
    }
}

fn is_stringish_box_name(box_name: &str) -> bool {
    matches!(box_name, "StringBox" | "String" | "__str")
        || box_name.ends_with("StringBox")
        || box_name.ends_with("String")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirType};

    #[test]
    fn infer_string_method_length_fact() {
        let fact = infer_from_method("StringBox", "length", 0).expect("length fact");
        assert_eq!(fact.op, StringCorridorOp::StrLen);
        assert_eq!(fact.role, StringCorridorRole::ScalarConsumer);
        assert_eq!(fact.carrier, StringCorridorCarrier::MethodCall);
        assert_eq!(fact.objectize, StringPlacementFact::None);
    }

    #[test]
    fn infer_runtime_export_substring_fact() {
        let fact = infer_from_runtime_export("nyash.string.substring_hii")
            .expect("substring runtime export fact");
        assert_eq!(fact.op, StringCorridorOp::StrSlice);
        assert_eq!(fact.carrier, StringCorridorCarrier::RuntimeExport);
        assert_eq!(fact.role, StringCorridorRole::BorrowProducer);
    }

    #[test]
    fn refresh_function_collects_string_call_facts() {
        let signature = FunctionSignature {
            name: "test_func".to_string(),
            params: vec![MirType::Box("StringBox".to_string())],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId::new(0));
        let block = function.get_block_mut(BasicBlockId::new(0)).expect("entry");
        block.add_instruction(MirInstruction::Call {
            dst: Some(ValueId::new(1)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "StringBox".to_string(),
                method: "length".to_string(),
                receiver: Some(ValueId::new(0)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![],
            effects: EffectMask::PURE,
        });
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(1)),
        });

        refresh_function_string_corridor_facts(&mut function);

        let fact = function
            .metadata
            .string_corridor_facts
            .get(&ValueId::new(1))
            .expect("fact");
        assert_eq!(fact.op, StringCorridorOp::StrLen);
        assert_eq!(fact.carrier, StringCorridorCarrier::MethodCall);
    }
}
