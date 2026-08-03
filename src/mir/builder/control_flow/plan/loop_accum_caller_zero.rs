//! Caller-zero finish/abort boundary for DirectAccum tests and fixtures.
//!
//! Production lowering never calls these helpers: its enclosing canonical
//! candidate owns the CFG/SSA/PHI finish and drops the candidate on failure.

use crate::mir::builder::control_flow::plan::loop_accum_physicalizer::{
    LoopPhysicalSuccessReceiptV1, LoopPhysicalizeErrorV1,
};
use crate::mir::builder::emission::phi_lifecycle::{PhiToken, PhiTxn};
use crate::mir::builder::resolved_lowering::canonical_cfg::CanonicalCfgSessionV1;
use crate::mir::builder::ssa::binding::BindingSsaBuilderV1;
use crate::mir::builder::MirBuilder;

pub(in crate::mir::builder) fn finish_caller_zero<T>(
    builder: &mut MirBuilder,
    cfg: CanonicalCfgSessionV1,
    ssa: BindingSsaBuilderV1<PhiToken>,
    mut phis: Option<PhiTxn>,
    receipt: T,
) -> Result<T, LoopPhysicalizeErrorV1> {
    if let Err(error) = ssa.finish() {
        return abort_caller_zero(
            builder,
            &mut phis,
            LoopPhysicalizeErrorV1::Ssa(format!("{error:?}")),
        );
    }
    let Some(function) = builder.function_state.current_function.as_ref() else {
        return abort_caller_zero(builder, &mut phis, LoopPhysicalizeErrorV1::MissingFunction);
    };
    if let Err(error) = cfg.finish(function) {
        return abort_caller_zero(builder, &mut phis, LoopPhysicalizeErrorV1::Cfg(error));
    }
    let txn = phis.take().expect("caller-zero PHI transaction");
    txn.commit(builder)
        .map_err(|error| LoopPhysicalizeErrorV1::PhiAbort(error.to_string()))?;
    Ok(receipt)
}

pub(in crate::mir::builder) fn abort_caller_zero<T>(
    builder: &mut MirBuilder,
    phis: &mut Option<PhiTxn>,
    error: LoopPhysicalizeErrorV1,
) -> Result<T, LoopPhysicalizeErrorV1> {
    let Some(txn) = phis.take() else {
        return Err(error);
    };
    let aborted = txn.abort_on_err(builder, format!("{error:?}"));
    Err(LoopPhysicalizeErrorV1::PhiAbort(aborted.to_string()))
}
