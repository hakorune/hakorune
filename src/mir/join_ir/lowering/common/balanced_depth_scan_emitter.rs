//! Phase 107: Balanced depth-scan derived emission (recipe only)
//!
//! This recipe is produced by builder-side policy (`plan/policies/keep_plan/*`) and
//! consumed by JoinIR lowering to derive:
//! - `depth_delta`: per-iteration delta (-1/0/+1)
//! - `depth_next`: depth + depth_delta (for break checks)

use crate::mir::join_ir::lowering::condition_env::ConditionEnv;
use crate::mir::join_ir::lowering::loop_body_local_env::LoopBodyLocalEnv;
use crate::mir::join_ir::{BinOpKind, CompareOp, ConstValue, JoinInst, MirLikeInst};
use crate::mir::ValueId;

#[derive(Debug, Clone)]
pub struct BalancedDepthScanRecipe {
    pub depth_var: String,
    pub ch_var: String,
    pub open: String,
    pub close: String,
    pub depth_delta_name: String,
    pub depth_next_name: String,
}

pub struct BalancedDepthScanEmitter;

impl BalancedDepthScanEmitter {
    pub fn emit_derived(
        recipe: &BalancedDepthScanRecipe,
        body_local_env: &mut LoopBodyLocalEnv,
        condition_env: &ConditionEnv,
        alloc_value: &mut dyn FnMut() -> ValueId,
        instructions: &mut Vec<JoinInst>,
    ) -> Result<(), String> {
        let ch_id = body_local_env.get(&recipe.ch_var).ok_or_else(|| {
            format!(
                "[phase107/balanced-depth] ch '{}' not found in LoopBodyLocalEnv",
                recipe.ch_var
            )
        })?;
        let depth_id = condition_env.get(&recipe.depth_var).ok_or_else(|| {
            format!(
                "[phase107/balanced-depth] depth '{}' not found in ConditionEnv",
                recipe.depth_var
            )
        })?;

        let const_open = alloc_value();
        instructions.push(JoinInst::Compute(MirLikeInst::Const {
            dst: const_open,
            value: ConstValue::String(recipe.open.clone()),
        }));
        let const_close = alloc_value();
        instructions.push(JoinInst::Compute(MirLikeInst::Const {
            dst: const_close,
            value: ConstValue::String(recipe.close.clone()),
        }));

        let is_open = alloc_value();
        instructions.push(JoinInst::Compute(MirLikeInst::Compare {
            dst: is_open,
            op: CompareOp::Eq,
            lhs: ch_id,
            rhs: const_open,
        }));
        let is_close = alloc_value();
        instructions.push(JoinInst::Compute(MirLikeInst::Compare {
            dst: is_close,
            op: CompareOp::Eq,
            lhs: ch_id,
            rhs: const_close,
        }));

        let const_1 = alloc_value();
        instructions.push(JoinInst::Compute(MirLikeInst::Const {
            dst: const_1,
            value: ConstValue::Integer(1),
        }));
        let const_0 = alloc_value();
        instructions.push(JoinInst::Compute(MirLikeInst::Const {
            dst: const_0,
            value: ConstValue::Integer(0),
        }));
        let const_m1 = alloc_value();
        instructions.push(JoinInst::Compute(MirLikeInst::Const {
            dst: const_m1,
            value: ConstValue::Integer(-1),
        }));

        let delta_open = alloc_value();
        instructions.push(JoinInst::Compute(MirLikeInst::Select {
            dst: delta_open,
            cond: is_open,
            then_val: const_1,
            else_val: const_0,
        }));

        let delta_close = alloc_value();
        instructions.push(JoinInst::Compute(MirLikeInst::Select {
            dst: delta_close,
            cond: is_close,
            then_val: const_m1,
            else_val: const_0,
        }));

        let depth_delta = alloc_value();
        instructions.push(JoinInst::Compute(MirLikeInst::BinOp {
            dst: depth_delta,
            op: BinOpKind::Add,
            lhs: delta_open,
            rhs: delta_close,
        }));

        let depth_next = alloc_value();
        instructions.push(JoinInst::Compute(MirLikeInst::BinOp {
            dst: depth_next,
            op: BinOpKind::Add,
            lhs: depth_id,
            rhs: depth_delta,
        }));

        body_local_env.insert(recipe.depth_delta_name.clone(), depth_delta);
        body_local_env.insert(recipe.depth_next_name.clone(), depth_next);
        Ok(())
    }
}
