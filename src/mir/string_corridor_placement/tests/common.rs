pub(super) use super::super::{build_value_def_map, infer_candidates};
pub(super) use crate::ast::Span;
pub(super) use crate::mir::string_corridor::{
    StringCorridorCarrier, StringCorridorFact, StringPublishReason, StringPublishReprPolicy,
};
pub(super) use crate::mir::string_corridor_placement::{
    refresh_function_string_corridor_candidates, StringCorridorCandidateKind,
    StringCorridorCandidateProof, StringCorridorCandidateState, StringCorridorPublicationContract,
};
pub(super) use crate::mir::{
    BasicBlock, BasicBlockId, BinaryOp, Callee, ConstValue, EffectMask, FunctionSignature,
    MirFunction, MirInstruction, MirType, ValueId,
};

pub(super) fn method_call(
    dst: ValueId,
    receiver: ValueId,
    box_name: &str,
    method: &str,
    args: Vec<ValueId>,
) -> MirInstruction {
    MirInstruction::Call {
        dst: Some(dst),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: box_name.to_string(),
            method: method.to_string(),
            receiver: Some(receiver),
            certainty: crate::mir::definitions::call_unified::TypeCertainty::Known,
            box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
        }),
        args,
        effects: EffectMask::PURE,
    }
}

pub(super) fn make_signature(return_type: MirType) -> FunctionSignature {
    FunctionSignature {
        name: "main".to_string(),
        params: vec![MirType::Box("StringBox".to_string())],
        return_type,
        effects: EffectMask::PURE,
    }
}

pub(super) fn make_function(return_type: MirType) -> MirFunction {
    MirFunction::new(make_signature(return_type), BasicBlockId(0))
}

pub(super) fn entry_block(function: &mut MirFunction) -> &mut BasicBlock {
    function.blocks.get_mut(&BasicBlockId(0)).expect("entry")
}

pub(super) fn push_unknown_span(block: &mut BasicBlock, inst: MirInstruction) {
    block.instructions.push(inst);
    block.instruction_spans.push(Span::unknown());
}

pub(super) fn push_const(block: &mut BasicBlock, dst: u32, value: ConstValue) {
    push_unknown_span(
        block,
        MirInstruction::Const {
            dst: ValueId(dst),
            value,
        },
    );
}

pub(super) fn push_binop(block: &mut BasicBlock, dst: u32, op: BinaryOp, lhs: u32, rhs: u32) {
    push_unknown_span(
        block,
        MirInstruction::BinOp {
            dst: ValueId(dst),
            op,
            lhs: ValueId(lhs),
            rhs: ValueId(rhs),
        },
    );
}
