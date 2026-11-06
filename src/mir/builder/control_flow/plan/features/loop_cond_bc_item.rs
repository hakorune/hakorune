//! LoopCondBreakContinueItem variant lowering.

use crate::mir::builder::control_flow::plan::parts;
use crate::mir::builder::control_flow::plan::LoopPlanExpressionPortV1;
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::control_flow::recipes::loop_cond_break_continue::LoopCondBreakContinueItem;
use crate::mir::builder::MirBuilder;
use std::collections::BTreeMap;

use super::loop_cond_bc_util::{ lower_simple_effect_stmt_body_input,
};

/// Source-port preparation for the recipe's direct statement item.
///
/// Complex LoopCond variants keep their existing owner until their nested and
/// exit contexts are parameterized. Returning `None` here is deliberate: the
/// source caller must not fall through to the raw item lowerer.
pub(in crate::mir::builder) fn lower_loop_cond_item_input<'input, P>(
    port: &P,
    body: &P::BodyInput<'input>,
    item: &LoopCondBreakContinueItem,
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    error_prefix: &str,
) -> Result<Option<Vec<LoweredRecipe>>, String>
where
    P: LoopPlanExpressionPortV1 + 'input,
{
    let LoopCondBreakContinueItem::Stmt(stmt_ref) = item else {
        if let LoopCondBreakContinueItem::ExitLeaf { kind, stmt } = item {
            let statement = port
                .body_stmt(body, stmt.index())
                .map_err(|error| error.render())?;
            return parts::exit::lower_loop_cond_exit_source_input(
                port,
                builder,
                current_bindings,
                carrier_step_phis,
                break_phi_dsts,
                statement,
                *kind,
                error_prefix,
            )
            .map(Some);
        }
        return Ok(None);
    };
    lower_simple_effect_stmt_body_input(
        port,
        body,
        *stmt_ref,
        builder,
        current_bindings,
        carrier_phis,
        carrier_updates,
        error_prefix,
    )
}
