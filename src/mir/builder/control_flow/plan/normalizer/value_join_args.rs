use crate::mir::basic_block::EdgeArgs;
use crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout;
use crate::mir::ValueId;

#[allow(dead_code)]
pub(super) fn expr_result_plus_carriers_args(
    expr_result: ValueId,
    carriers: Vec<ValueId>,
) -> EdgeArgs {
    let mut values = Vec::with_capacity(1 + carriers.len());
    values.push(expr_result);
    values.extend(carriers);
    EdgeArgs {
        layout: JumpArgsLayout::ExprResultPlusCarriers,
        values,
    }
}
