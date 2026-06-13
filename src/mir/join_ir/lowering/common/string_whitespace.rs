//! String whitespace predicate builder shared by trim lowerers.
//!
//! This is instruction-sequence sharing only. It does not decide which route
//! accepts a source shape or which target lowerer owns execution.

use crate::mir::join_ir::{BinOpKind, CompareOp, ConstValue, JoinFunction, JoinInst, MirLikeInst};
use crate::mir::ValueId;

pub(crate) struct WhitespacePredicateIds {
    pub cmp_space: ValueId,
    pub cmp_tab: ValueId,
    pub cmp_newline: ValueId,
    pub cmp_cr: ValueId,
    pub const_space: ValueId,
    pub const_tab: ValueId,
    pub const_newline: ValueId,
    pub const_cr: ValueId,
    pub or1: ValueId,
    pub or2: ValueId,
    pub is_space: ValueId,
}

pub(crate) fn append_string_whitespace_predicate(
    func: &mut JoinFunction,
    ch: ValueId,
    ids: WhitespacePredicateIds,
) -> ValueId {
    func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: ids.const_space,
        value: ConstValue::String(" ".to_string()),
    }));
    func.body.push(JoinInst::Compute(MirLikeInst::Compare {
        dst: ids.cmp_space,
        lhs: ch,
        rhs: ids.const_space,
        op: CompareOp::Eq,
    }));

    func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: ids.const_tab,
        value: ConstValue::String("\\t".to_string()),
    }));
    func.body.push(JoinInst::Compute(MirLikeInst::Compare {
        dst: ids.cmp_tab,
        lhs: ch,
        rhs: ids.const_tab,
        op: CompareOp::Eq,
    }));

    func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: ids.const_newline,
        value: ConstValue::String("\\n".to_string()),
    }));
    func.body.push(JoinInst::Compute(MirLikeInst::Compare {
        dst: ids.cmp_newline,
        lhs: ch,
        rhs: ids.const_newline,
        op: CompareOp::Eq,
    }));

    func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: ids.const_cr,
        value: ConstValue::String("\\r".to_string()),
    }));
    func.body.push(JoinInst::Compute(MirLikeInst::Compare {
        dst: ids.cmp_cr,
        lhs: ch,
        rhs: ids.const_cr,
        op: CompareOp::Eq,
    }));

    func.body.push(JoinInst::Compute(MirLikeInst::BinOp {
        dst: ids.or1,
        lhs: ids.cmp_space,
        rhs: ids.cmp_tab,
        op: BinOpKind::Or,
    }));
    func.body.push(JoinInst::Compute(MirLikeInst::BinOp {
        dst: ids.or2,
        lhs: ids.or1,
        rhs: ids.cmp_newline,
        op: BinOpKind::Or,
    }));
    func.body.push(JoinInst::Compute(MirLikeInst::BinOp {
        dst: ids.is_space,
        lhs: ids.or2,
        rhs: ids.cmp_cr,
        op: BinOpKind::Or,
    }));

    ids.is_space
}
