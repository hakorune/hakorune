use crate::mir::join_ir::lowering::common::string_whitespace::{
    append_string_whitespace_predicate, WhitespacePredicateIds,
};
use crate::mir::join_ir::{
    BinOpKind, CompareOp, ConstValue, JoinContId, JoinFuncId, JoinFunction, JoinInst, MirLikeInst,
};
use crate::mir::ValueId;

/// Builds the `skip_leading` helper function for Generic Case A trim lowering.
///
/// Keep ValueIds and instruction order in sync with the historical inline
/// implementation in `generic_case_a::trim`.
pub(super) fn build(skip_leading_id: JoinFuncId) -> JoinFunction {
    let mut skip_func = JoinFunction::new(
        skip_leading_id,
        "skip_leading".to_string(),
        vec![ValueId(7000), ValueId(7001), ValueId(7002)],
    );
    let s_skip = ValueId(7000);
    let i_skip = ValueId(7001);
    let n_skip = ValueId(7002);
    let cmp_len = ValueId(7003);
    let const_1_skip = ValueId(7004);
    let i_plus_1_skip = ValueId(7005);
    let ch_skip = ValueId(7006);
    let cmp_space_skip = ValueId(7007);
    let cmp_tab_skip = ValueId(7008);
    let cmp_newline_skip = ValueId(7009);
    let cmp_cr_skip = ValueId(7010);
    let const_space_skip = ValueId(7011);
    let const_tab_skip = ValueId(7012);
    let const_newline_skip = ValueId(7013);
    let const_cr_skip = ValueId(7014);
    let or1_skip = ValueId(7015);
    let or2_skip = ValueId(7016);
    let is_space_skip = ValueId(7017);
    let bool_false_skip = ValueId(7018);
    let is_space_false_skip = ValueId(7019);

    skip_func.body.push(JoinInst::Compute(MirLikeInst::Compare {
        dst: cmp_len,
        lhs: i_skip,
        rhs: n_skip,
        op: CompareOp::Ge,
    }));
    skip_func.body.push(JoinInst::Jump {
        cont: JoinContId::new(2),
        args: vec![i_skip],
        cond: Some(cmp_len),
    });
    skip_func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: const_1_skip,
        value: ConstValue::Integer(1),
    }));
    skip_func.body.push(JoinInst::Compute(MirLikeInst::BinOp {
        dst: i_plus_1_skip,
        lhs: i_skip,
        rhs: const_1_skip,
        op: BinOpKind::Add,
    }));
    skip_func.body.push(JoinInst::Compute(MirLikeInst::BoxCall {
        dst: Some(ch_skip),
        box_name: "StringBox".to_string(),
        method: "substring".to_string(),
        args: vec![s_skip, i_skip, i_plus_1_skip],
    }));
    let is_space_skip = append_string_whitespace_predicate(
        &mut skip_func,
        ch_skip,
        WhitespacePredicateIds {
            cmp_space: cmp_space_skip,
            cmp_tab: cmp_tab_skip,
            cmp_newline: cmp_newline_skip,
            cmp_cr: cmp_cr_skip,
            const_space: const_space_skip,
            const_tab: const_tab_skip,
            const_newline: const_newline_skip,
            const_cr: const_cr_skip,
            or1: or1_skip,
            or2: or2_skip,
            is_space: is_space_skip,
        },
    );
    skip_func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: bool_false_skip,
        value: ConstValue::Bool(false),
    }));
    skip_func.body.push(JoinInst::Compute(MirLikeInst::Compare {
        dst: is_space_false_skip,
        lhs: is_space_skip,
        rhs: bool_false_skip,
        op: CompareOp::Eq,
    }));
    skip_func.body.push(JoinInst::Jump {
        cont: JoinContId::new(3),
        args: vec![i_skip],
        cond: Some(is_space_false_skip),
    });
    skip_func.body.push(JoinInst::Call {
        func: skip_leading_id,
        args: vec![s_skip, i_plus_1_skip, n_skip],
        k_next: None,
        dst: None,
    });

    skip_func
}
