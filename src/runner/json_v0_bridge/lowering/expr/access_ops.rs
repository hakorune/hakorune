use super::super::BridgeEnv;
use super::VarScope;
use crate::mir::{BasicBlockId, MirFunction, ValueId};

pub(super) fn lower_var_expr<S: VarScope>(
    env: &BridgeEnv,
    f: &mut MirFunction,
    cur_bb: BasicBlockId,
    name: &str,
    vars: &mut S,
) -> Result<(ValueId, BasicBlockId), String> {
    match vars.resolve(env, f, cur_bb, name)? {
        Some(value_id) => Ok((value_id, cur_bb)),
        None => Err(format!("undefined variable: {}", name)),
    }
}
