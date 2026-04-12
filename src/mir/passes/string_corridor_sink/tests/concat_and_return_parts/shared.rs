use super::super::*;

pub(super) fn method_call(
    dst: ValueId,
    receiver: ValueId,
    box_name: &str,
    method: &str,
    args: Vec<ValueId>,
    ty: MirType,
) -> MirInstruction {
    let _ = ty;
    MirInstruction::Call {
        dst: Some(dst),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: box_name.to_string(),
            method: method.to_string(),
            receiver: Some(receiver),
            certainty: TypeCertainty::Known,
            box_kind: CalleeBoxKind::RuntimeData,
        }),
        args,
        effects: EffectMask::PURE,
    }
}

pub(super) fn extern_call(dst: ValueId, name: &str, args: Vec<ValueId>) -> MirInstruction {
    MirInstruction::Call {
        dst: Some(dst),
        func: ValueId::INVALID,
        callee: Some(Callee::Extern(name.to_string())),
        args,
        effects: EffectMask::PURE,
    }
}
