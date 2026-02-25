use crate::mir::join_ir::{
    BinOpKind, CompareOp, ConstValue, JoinFunction, JoinInst, MirLikeInst, UnaryOp,
};
use crate::mir::ValueId;

pub(crate) fn const_i64(func: &mut JoinFunction, dst: ValueId, value: i64) {
    func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst,
        value: ConstValue::Integer(value),
    }));
}

pub(crate) fn compare(func: &mut JoinFunction, dst: ValueId, op: CompareOp, lhs: ValueId, rhs: ValueId) {
    func.body.push(JoinInst::Compute(MirLikeInst::Compare {
        dst,
        op,
        lhs,
        rhs,
    }));
}

pub(crate) fn unary_not(func: &mut JoinFunction, dst: ValueId, operand: ValueId) {
    func.body.push(JoinInst::Compute(MirLikeInst::UnaryOp {
        dst,
        op: UnaryOp::Not,
        operand,
    }));
}

pub(crate) fn bin_add(func: &mut JoinFunction, dst: ValueId, lhs: ValueId, rhs: ValueId) {
    func.body.push(JoinInst::Compute(MirLikeInst::BinOp {
        dst,
        op: BinOpKind::Add,
        lhs,
        rhs,
    }));
}

pub(crate) fn select(
    func: &mut JoinFunction,
    dst: ValueId,
    cond: ValueId,
    then_val: ValueId,
    else_val: ValueId,
) {
    func.body.push(JoinInst::Compute(MirLikeInst::Select {
        dst,
        cond,
        then_val,
        else_val,
    }));
}
