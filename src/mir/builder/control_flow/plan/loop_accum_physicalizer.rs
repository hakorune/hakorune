//! Caller-zero DirectAccum physicalizer.
//!
//! This is the first consumer of a verified Recipe/JoinSig pair. It uses the
//! existing canonical CFG, function-owned Binding SSA, and one PhiTxn; it
//! does not select routes or publish a candidate module.

pub(in crate::mir::builder) use super::loop_accum_binding_port::DirectAccumBindingPortV1;
#[cfg(test)]
pub(in crate::mir::builder) use super::loop_accum_binding_port::RawDirectAccumBindingPort;
#[cfg(test)]
use super::loop_accum_caller_zero::{abort_caller_zero, finish_caller_zero};
use super::loop_physical_input::{
    LoopPhysicalInputRejectV1, LoopPhysicalRoleV1, VerifiedLoopBindingProjectionV1,
    VerifiedLoopInputProjectionV1, VerifiedLoopPhysicalRolePlanV1,
};
#[cfg(test)]
use crate::mir::builder::emission::phi_lifecycle::PhiToken;
use crate::mir::builder::emission::{loop_operation, phi_lifecycle::PhiTxn};
use crate::mir::builder::resolved_lowering::canonical_cfg::{
    CanonicalCfgErrorV1, CanonicalCfgSessionV1,
};
#[cfg(test)]
use crate::mir::builder::ssa::binding::BindingSsaBuilderV1;
use crate::mir::builder::MirBuilder;
use crate::mir::loop_recipe_contract::{
    LoopBinaryI64OpV1, LoopBlockKeyV1, LoopCompareI64OpV1, LoopConditionV1, LoopJoinEdgeRoleV1,
    LoopOperationV1, LoopRecipeItemV1, VerifiedLoopPhysicalInputV1,
};
use crate::mir::resolved_semantics::BindingRefV1;
use crate::mir::{BasicBlockId, MirType, ValueId};
use std::collections::BTreeMap;

mod session;
use session::DirectAccumPhysicalizerV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum LoopResultDispositionV1 {
    Unit,
    Value(ValueId),
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) struct LoopPhysicalSuccessReceiptV1 {
    pub(in crate::mir::builder) final_values:
        Box<[(crate::mir::loop_recipe_contract::LoopBindingKeyV1, ValueId)]>,
    pub(in crate::mir::builder) result: LoopResultDispositionV1,
}

/// Production handoff.  The loop has been emitted, but `After` remains the
/// caller's open continuation block; no function return or final-value
/// snapshot is performed here.
#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) struct LoopPhysicalContinuationReceiptV1 {
    pub(in crate::mir::builder) continuation_block: BasicBlockId,
    pub(in crate::mir::builder) result: LoopResultDispositionV1,
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum LoopPhysicalizeErrorV1 {
    Input(LoopPhysicalInputRejectV1),
    MissingFunction,
    CurrentBlockMismatch {
        expected: BasicBlockId,
        actual: Option<BasicBlockId>,
    },
    PreheaderTerminated(BasicBlockId),
    ExistingPhysicalBlock(BasicBlockId),
    RecipeShape(&'static str),
    MissingValue(crate::mir::loop_recipe_contract::LoopValueKeyV1),
    MissingBinding(crate::mir::loop_recipe_contract::LoopBindingKeyV1),
    Operation(String),
    Cfg(CanonicalCfgErrorV1),
    Ssa(String),
    PhiAbort(String),
    #[cfg(test)]
    InjectedTestFailure(DirectAccumPhysicalizerTestFailurePointV1),
}

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum DirectAccumPhysicalizerTestFailurePointV1 {
    AfterHeaderCondition,
}

#[cfg(test)]
pub(in crate::mir::builder) fn physicalize_direct_accum_v1(
    builder: &mut MirBuilder,
    input: VerifiedLoopPhysicalInputV1,
    bindings: VerifiedLoopBindingProjectionV1,
    inputs: VerifiedLoopInputProjectionV1,
    roles: VerifiedLoopPhysicalRolePlanV1,
) -> Result<LoopPhysicalSuccessReceiptV1, LoopPhysicalizeErrorV1> {
    let owner = bindings.owner();
    let mut cfg = CanonicalCfgSessionV1::new();
    let mut ssa = BindingSsaBuilderV1::<PhiToken>::new(owner);
    let mut port = RawDirectAccumBindingPort { ssa: &mut ssa };
    let mut phis = Some(PhiTxn::begin("loop_direct_accum_physicalizer"));
    let mut session = DirectAccumPhysicalizerV1::preflight(
        builder,
        input,
        bindings,
        inputs,
        roles,
        &mut cfg,
        &mut port,
        phis.as_mut().expect("caller-zero PHI transaction"),
    )?;
    let result = session.emit();
    match result {
        Ok(receipt) => finish_caller_zero(builder, cfg, ssa, phis, receipt),
        Err(error) => abort_caller_zero(builder, &mut phis, error),
    }
}

#[cfg(test)]
pub(in crate::mir::builder) fn physicalize_direct_accum_v1_with_test_failure(
    builder: &mut MirBuilder,
    input: VerifiedLoopPhysicalInputV1,
    bindings: VerifiedLoopBindingProjectionV1,
    inputs: VerifiedLoopInputProjectionV1,
    roles: VerifiedLoopPhysicalRolePlanV1,
    failure: DirectAccumPhysicalizerTestFailurePointV1,
) -> Result<LoopPhysicalSuccessReceiptV1, LoopPhysicalizeErrorV1> {
    let owner = bindings.owner();
    let mut cfg = CanonicalCfgSessionV1::new();
    let mut ssa = BindingSsaBuilderV1::<PhiToken>::new(owner);
    let mut port = RawDirectAccumBindingPort { ssa: &mut ssa };
    let mut phis = Some(PhiTxn::begin("loop_direct_accum_physicalizer"));
    let mut session = DirectAccumPhysicalizerV1::preflight(
        builder,
        input,
        bindings,
        inputs,
        roles,
        &mut cfg,
        &mut port,
        phis.as_mut().expect("caller-zero PHI transaction"),
    )?;
    session.failure = Some(failure);
    let result = session.emit();
    match result {
        Ok(receipt) => finish_caller_zero(builder, cfg, ssa, phis, receipt),
        Err(error) => abort_caller_zero(builder, &mut phis, error),
    }
}

/// Borrowing production seam. The caller owns the function-wide CFG/SSA/PHI
/// services and decides when they are finished or committed.
#[cfg(test)]
pub(in crate::mir::builder) fn physicalize_direct_accum_v1_borrowing(
    builder: &mut MirBuilder,
    input: VerifiedLoopPhysicalInputV1,
    bindings: VerifiedLoopBindingProjectionV1,
    inputs: VerifiedLoopInputProjectionV1,
    roles: VerifiedLoopPhysicalRolePlanV1,
    cfg: &mut CanonicalCfgSessionV1,
    ssa: &mut BindingSsaBuilderV1<PhiToken>,
    phis: &mut Option<PhiTxn>,
) -> Result<LoopPhysicalSuccessReceiptV1, LoopPhysicalizeErrorV1> {
    let mut port = RawDirectAccumBindingPort { ssa };
    let transaction = phis.as_mut().expect("borrowed PHI transaction");
    let mut session = DirectAccumPhysicalizerV1::preflight(
        builder,
        input,
        bindings,
        inputs,
        roles,
        cfg,
        &mut port,
        transaction,
    )?;
    let result = session.emit();
    match result {
        Ok(receipt) => Ok(receipt),
        Err(error) => abort_caller_zero(builder, phis, error),
    }
}

/// Generic production entrypoint.  The caller supplies the existing
/// function-owned identity/SSA port; this helper never creates an owner.
pub(in crate::mir::builder) fn physicalize_direct_accum_v1_with_port<P>(
    builder: &mut MirBuilder,
    input: VerifiedLoopPhysicalInputV1,
    bindings: VerifiedLoopBindingProjectionV1,
    inputs: VerifiedLoopInputProjectionV1,
    roles: VerifiedLoopPhysicalRolePlanV1,
    cfg: &mut CanonicalCfgSessionV1,
    port: &mut P,
    phis: &mut PhiTxn,
) -> Result<LoopPhysicalContinuationReceiptV1, LoopPhysicalizeErrorV1>
where
    P: DirectAccumBindingPortV1,
{
    let mut session = DirectAccumPhysicalizerV1::preflight(
        builder, input, bindings, inputs, roles, cfg, port, phis,
    )?;
    session.emit_inline()
}
