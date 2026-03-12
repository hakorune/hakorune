use crate::mir::join_ir::lowering::debug_output_box::DebugOutputBox;
use crate::mir::join_ir::{BinOpKind, ConstValue, JoinFuncId, JoinInst, MirLikeInst};
use crate::mir::ValueId;

pub(crate) fn emit_tail_call(
    loop_step_id: JoinFuncId,
    i_param: ValueId,
    updated_carrier_values: &[ValueId],
    loop_var_next_override: Option<ValueId>,
    alloc_local_fn: &mut dyn FnMut() -> ValueId,
    tail_block: &mut Vec<JoinInst>,
    dev_log: &DebugOutputBox,
) -> ValueId {
    let i_next = if let Some(i_next) = loop_var_next_override {
        i_next
    } else {
        let const_1 = alloc_local_fn();
        tail_block.push(JoinInst::Compute(MirLikeInst::Const {
            dst: const_1,
            value: ConstValue::Integer(1),
        }));

        let i_next = alloc_local_fn();
        tail_block.push(JoinInst::Compute(MirLikeInst::BinOp {
            dst: i_next,
            op: BinOpKind::Add,
            lhs: i_param,
            rhs: const_1,
        }));
        i_next
    };

    let mut tail_call_args = vec![i_next];
    tail_call_args.extend(updated_carrier_values.iter().copied());

    dev_log.log_if_enabled(|| {
        format!(
            "tail call args count: {}, updated_carrier_values: {:?}",
            tail_call_args.len(),
            updated_carrier_values
        )
    });

    tail_block.push(JoinInst::Call {
        func: loop_step_id,
        args: tail_call_args,
        k_next: None,
        dst: None,
    });

    i_next
}
