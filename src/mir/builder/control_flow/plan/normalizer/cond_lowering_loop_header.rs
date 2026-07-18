//! Loop-header condition lowering raw facade.
//!
//! Structural recursion and expression lowering live in the associated-input
//! core. This facade keeps the existing `CondBlockView` prelude behavior and
//! selects the raw expression port exactly once.

use super::cond_lowering_loop_header_port::lower_loop_header_cond_input;
use super::cond_lowering_prelude::lower_cond_prelude_stmts;
use crate::mir::builder::control_flow::cleanup::policies::cond_prelude_vocab::prelude_has_loop_like_stmt;
use crate::mir::builder::control_flow::edgecfg::api::BranchStub;
use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::{CoreEffectPlan, RawLoopPlanExpressionPortV1};
use crate::mir::builder::MirBuilder;
use crate::mir::{BasicBlockId, EdgeArgs, ValueId};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug)]
pub struct LoopHeaderCondResult {
    pub block_effects: BTreeMap<BasicBlockId, Vec<CoreEffectPlan>>,
    pub branches: Vec<BranchStub>,
    pub first_cond: ValueId,
}

impl LoopHeaderCondResult {
    pub fn preds_to(&self, target: BasicBlockId) -> BTreeSet<BasicBlockId> {
        let mut preds = BTreeSet::new();
        for branch in &self.branches {
            if branch.then_target == target || branch.else_target == target {
                preds.insert(branch.from);
            }
        }
        preds
    }
}

pub fn lower_loop_header_cond(
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, ValueId>,
    cond: &CondBlockView,
    current_bb: BasicBlockId,
    body_bb: BasicBlockId,
    after_bb: BasicBlockId,
    body_args: EdgeArgs,
    after_args: EdgeArgs,
    error_prefix: &str,
) -> Result<LoopHeaderCondResult, String> {
    if prelude_has_loop_like_stmt(&cond.prelude_stmts) {
        return Err(format!(
            "[freeze:contract][cond_prelude] {error_prefix}: loop-like stmt in loop-header prelude is unsupported in effect-only route"
        ));
    }

    let (bindings, prelude_effects) =
        lower_cond_prelude_stmts(builder, phi_bindings, &cond.prelude_stmts, error_prefix)?;
    let port = RawLoopPlanExpressionPortV1::new();
    let mut result = lower_loop_header_cond_input(
        builder,
        &bindings,
        &port,
        port.expr(&cond.tail_expr),
        current_bb,
        body_bb,
        after_bb,
        body_args,
        after_args,
        error_prefix,
    )?;

    if !prelude_effects.is_empty() {
        let entry = result.block_effects.entry(current_bb).or_default();
        let mut merged = prelude_effects;
        merged.append(entry);
        *entry = merged;
    }

    Ok(result)
}
