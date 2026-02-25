use crate::mir::basic_block::EdgeArgs;
use crate::mir::builder::control_flow::plan::CoreEffectPlan;
use crate::mir::builder::MirBuilder;
use crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout;
use crate::mir::{CompareOp, ConstValue, MirType, ValueId};

pub(in crate::mir::builder) fn empty_args() -> EdgeArgs {
    EdgeArgs {
        layout: JumpArgsLayout::CarriersOnly,
        values: vec![],
    }
}

pub(in crate::mir::builder) fn negate_bool_cond(
    builder: &mut MirBuilder,
    cond_id: ValueId,
) -> (ValueId, Vec<CoreEffectPlan>) {
    let false_id = builder.alloc_typed(MirType::Bool);
    let neg_id = builder.alloc_typed(MirType::Bool);
    let effects = vec![
        CoreEffectPlan::Const {
            dst: false_id,
            value: ConstValue::Bool(false),
        },
        CoreEffectPlan::Compare {
            dst: neg_id,
            lhs: cond_id,
            op: CompareOp::Eq,
            rhs: false_id,
        },
    ];
    (neg_id, effects)
}
