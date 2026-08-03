//! One function-scoped owner bundle shared by canonical lowering profiles.

use crate::mir::builder::emission::phi_lifecycle::PhiTxn;
use crate::mir::builder::resolved_lowering::canonical_cfg::CanonicalCfgSessionV1;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_control_flow::if_control::{
    FunctionIfControlUseLedgerV1, VerifiedResolvedFunctionIfControlV1,
};
use crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1;

use super::super::completion_consumption::ResolvedFunctionCompletionConsumptionV1;
use super::super::semantic_stack::{ResolvedSemanticExpectedCountsV1, ResolvedSemanticStackV1};
use super::identity::ResolvedSsaIdentityStateV2;

/// The only mutable SSA/CFG/PHI owner for one canonical function session.
///
/// A profile may add source-specific admission receipts, but it must borrow
/// this bundle rather than create another reaching-value or PHI authority.
pub(in crate::mir::builder::resolved_lowering) struct CanonicalSsaFunctionSessionV2<'source> {
    pub(in crate::mir::builder::resolved_lowering) identity: ResolvedSsaIdentityStateV2<'source>,
    pub(in crate::mir::builder::resolved_lowering) semantics: ResolvedSemanticStackV1,
    pub(in crate::mir::builder::resolved_lowering) if_control: FunctionIfControlUseLedgerV1,
    pub(in crate::mir::builder::resolved_lowering) completion:
        ResolvedFunctionCompletionConsumptionV1,
    pub(in crate::mir::builder::resolved_lowering) cfg: CanonicalCfgSessionV1,
    pub(in crate::mir::builder::resolved_lowering) phis: PhiTxn,
    pub(in crate::mir::builder::resolved_lowering) implicit_completion: bool,
}

impl<'source> CanonicalSsaFunctionSessionV2<'source> {
    pub(in crate::mir::builder::resolved_lowering) fn new(
        input: ResolvedFunctionLoweringInputV1<'source>,
        if_control: VerifiedResolvedFunctionIfControlV1,
        completion: VerifiedFunctionCompletionV1,
        block_expr_count: usize,
    ) -> Result<Self, String> {
        let if_controls = if_control.row_count();
        let if_branches = if_controls + if_control.explicit_else_count();
        let semantics = ResolvedSemanticStackV1::new_with_expectations(
            input.function(),
            input.function().lowering_roots(),
            ResolvedSemanticExpectedCountsV1::new(block_expr_count, if_controls, if_branches),
        )?;
        let implicit_completion = completion.is_implicit_void();
        let completion = ResolvedFunctionCompletionConsumptionV1::new(input.owner(), completion)?;
        Ok(Self {
            identity: ResolvedSsaIdentityStateV2::new(input.function()),
            semantics,
            if_control: if_control.into_use_ledger(),
            completion,
            cfg: CanonicalCfgSessionV1::new(),
            phis: PhiTxn::begin("canonical_binding_ssa"),
            implicit_completion,
        })
    }

    pub(in crate::mir::builder::resolved_lowering) const fn completion_is_implicit(&self) -> bool {
        self.implicit_completion
    }
}
