/*!
 * String corridor facts for canonical MIR.
 *
 * This module keeps no-behavior-change semantic facts about current string
 * lowering carriers. It does not introduce a second MIR dialect; it annotates
 * existing MIR with inventory-friendly facts over canonical string ops.
 * Legacy/helper/runtime-name recovery is quarantined in
 * `string_corridor_compat`.
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

/// Birth / placement outcomes carried by the current string-lane SSOT.
///
/// Keep this local until a second real lifecycle/outcome consumer appears
/// outside the string lane. Barrier-cause vocabularies such as
/// `EscapeBarrier` and `SumObjectizationBarrier` answer a different question.
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
///
/// This remains string-local for now. `phase-166x` explicitly defers a generic
/// extraction until lifecycle/outcome consumers exist beyond this lane.
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

/// Explicit object -> text provenance contract owned by MIR/lowering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringCorridorBorrowContract {
    BorrowTextFromObject,
}

impl std::fmt::Display for StringCorridorBorrowContract {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BorrowTextFromObject => f.write_str("borrow_text_from_obj"),
        }
    }
}

/// Explicit text -> object publish reason owned by MIR/lowering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringPublishReason {
    StableObjectDemand,
    ExplicitApiReplay,
}

impl std::fmt::Display for StringPublishReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StableObjectDemand => f.write_str("stable_object_demand"),
            Self::ExplicitApiReplay => f.write_str("explicit_api_replay"),
        }
    }
}

/// Public representation policy selected by `publish.text`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringPublishReprPolicy {
    StableOwned,
    StableView,
}

impl std::fmt::Display for StringPublishReprPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StableOwned => f.write_str("stable_owned"),
            Self::StableView => f.write_str("stable_view"),
        }
    }
}

/// String-only provenance witness that makes a `stable_view` publication legal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringStableViewProvenance {
    AlreadyStable,
    ImmutableHostOwned,
    PinnedNoMutation,
}

impl std::fmt::Display for StringStableViewProvenance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyStable => f.write_str("already_stable"),
            Self::ImmutableHostOwned => f.write_str("immutable_host_owned"),
            Self::PinnedNoMutation => f.write_str("pinned_no_mutation"),
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
    pub borrow_contract: Option<StringCorridorBorrowContract>,
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
            borrow_contract: None,
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
            borrow_contract: Some(StringCorridorBorrowContract::BorrowTextFromObject),
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
            borrow_contract: None,
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
        let borrow_contract = self
            .borrow_contract
            .map(|v| v.to_string())
            .unwrap_or_else(|| "none".to_string());
        format!(
            "{} carrier={} role={} borrow={} outcome={} objectize={} publish={} materialize={}",
            self.op,
            self.carrier,
            self.role,
            borrow_contract,
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
            callee: Some(Callee::Global(name)),
            ..
        } => infer_from_canonical_global(name).map(|fact| (*dst, fact)),
        _ => super::string_corridor_compat::infer_compat_fact_from_instruction(inst),
    }
}

fn infer_from_canonical_global(name: &str) -> Option<StringCorridorFact> {
    match name {
        "str.len" | "__str.len" => Some(StringCorridorFact::str_len(
            StringCorridorCarrier::CanonicalIntrinsic,
        )),
        "str.slice" | "__str.slice" => Some(StringCorridorFact::str_slice(
            StringCorridorCarrier::CanonicalIntrinsic,
        )),
        "freeze.str" => Some(StringCorridorFact::freeze_str(
            StringCorridorCarrier::CanonicalIntrinsic,
        )),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirType};

    #[test]
    fn infer_canonical_global_length_fact() {
        let fact = infer_from_canonical_global("str.len").expect("length fact");
        assert_eq!(fact.op, StringCorridorOp::StrLen);
        assert_eq!(fact.role, StringCorridorRole::ScalarConsumer);
        assert_eq!(fact.carrier, StringCorridorCarrier::CanonicalIntrinsic);
        assert_eq!(fact.objectize, StringPlacementFact::None);
    }

    #[test]
    fn infer_canonical_global_slice_fact() {
        let fact = infer_from_canonical_global("str.slice").expect("slice fact");
        assert_eq!(fact.op, StringCorridorOp::StrSlice);
        assert_eq!(fact.role, StringCorridorRole::BorrowProducer);
        assert_eq!(fact.carrier, StringCorridorCarrier::CanonicalIntrinsic);
        assert_eq!(
            fact.borrow_contract,
            Some(StringCorridorBorrowContract::BorrowTextFromObject)
        );
    }

    #[test]
    fn infer_canonical_global_freeze_fact() {
        let fact = infer_from_canonical_global("freeze.str").expect("freeze fact");
        assert_eq!(fact.op, StringCorridorOp::FreezeStr);
        assert_eq!(fact.role, StringCorridorRole::BirthSink);
        assert_eq!(fact.carrier, StringCorridorCarrier::CanonicalIntrinsic);
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
