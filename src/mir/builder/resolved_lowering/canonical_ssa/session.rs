//! One function-scoped owner bundle shared by canonical lowering profiles.

#[path = "session/control_borrow.rs"]
mod control_borrow;

use crate::mir::builder::MirBuilder;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::located::SourceBodySiteV1;
use crate::mir::resolved_control_flow::if_control::{
    FunctionIfControlUseErrorV1, FunctionIfControlUseLedgerV1,
    VerifiedResolvedFunctionIfControlV1,
};
use crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1;
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, RegionId};
use crate::mir::{BasicBlockId, MirType, ValueId};

use super::super::completion_consumption::ResolvedFunctionCompletionConsumptionV1;
use super::super::semantic_stack::{ResolvedSemanticExpectedCountsV1, ResolvedSemanticStackV1};
use super::identity::ResolvedSsaIdentityStateV2;

#[path = "session/draft_seal_close.rs"]
mod draft_seal_close;
#[path = "session/generic_g0_entry_adoption.rs"]
mod generic_g0_entry_adoption;

pub(in crate::mir::builder::resolved_lowering) enum CanonicalIfControlConsumptionV1 {
    Resolved(FunctionIfControlUseLedgerV1),
    // The Dynamic profile's complete operation/control ledger is consumed by
    // the selected physical session before the common terminal is opened.
    // Until that terminal exists, this is intentionally a unit disposition:
    // it must not retain an owner token that finish() cannot validate.
    DynamicProfileOwned,
}

impl CanonicalIfControlConsumptionV1 {
    fn finish(self) -> Result<(), FunctionIfControlUseErrorV1> {
        match self {
            Self::Resolved(ledger) => ledger.finish(),
            Self::DynamicProfileOwned => Ok(()),
        }
    }
}

/// The only mutable SSA/CFG/PHI owner for one canonical function session.
///
/// A profile may add source-specific admission receipts, but it must borrow
/// this bundle rather than create another reaching-value or PHI authority.
pub(in crate::mir::builder::resolved_lowering) struct CanonicalSsaFunctionSessionV2<'source> {
    owner: FunctionOwnerIdV1,
    root_body: SourceBodySiteV1,
    root_body_end: u32,
    target_function: RegionId,
    pub(in crate::mir::builder::resolved_lowering) identity: ResolvedSsaIdentityStateV2<'source>,
    pub(in crate::mir::builder::resolved_lowering) semantics: ResolvedSemanticStackV1,
    pub(in crate::mir::builder::resolved_lowering) if_control: CanonicalIfControlConsumptionV1,
    pub(in crate::mir::builder::resolved_lowering) completion:
        ResolvedFunctionCompletionConsumptionV1,
    control: std::cell::OnceCell<super::SharedFunctionControlV1>,

    pub(in crate::mir::builder::resolved_lowering) implicit_completion: bool,
    generic_entry_adopted: bool,
}

/// One-shot evidence that a profile-specific ledger has closed before the
/// common function terminal consumes the session.
#[derive(Debug)]
pub(in crate::mir::builder::resolved_lowering) struct ReadyCanonicalProfileCloseV1 {
    owner: FunctionOwnerIdV1,
    terminal_block: BasicBlockId,
}

impl ReadyCanonicalProfileCloseV1 {
    fn from_closed_profile(owner: FunctionOwnerIdV1, terminal_block: BasicBlockId) -> Self {
        Self {
            owner,
            terminal_block,
        }
    }

    fn parts(self) -> (FunctionOwnerIdV1, BasicBlockId) {
        (self.owner, self.terminal_block)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder::resolved_lowering) enum CanonicalFunctionFinishErrorV1 {
    ProfileOwnerMismatch,
    TerminalBlockMismatch,
    FunctionMissing,
    Cfg(String),
    Semantic(String),
    IfControl(String),
    Identity(String),
    Phi(String),
    CheckedCallOut(String),
    Binding(String),
    Completion(String),
    ReturnOperandMissing,
}

impl std::fmt::Display for CanonicalFunctionFinishErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ProfileOwnerMismatch => {
                formatter.write_str("[freeze:contract][canonical_finish/profile_owner_mismatch]")
            }
            Self::TerminalBlockMismatch => {
                formatter.write_str("[freeze:contract][canonical_finish/terminal_block_mismatch]")
            }
            Self::FunctionMissing => {
                formatter.write_str("[freeze:contract][canonical_finish/function_missing]")
            }
            Self::Cfg(error) => {
                write!(formatter, "[freeze:contract][canonical_finish/cfg] {error}")
            }
            Self::Semantic(error) => write!(
                formatter,
                "[freeze:contract][canonical_finish/semantic] {error}"
            ),
            Self::IfControl(error) => write!(
                formatter,
                "[freeze:contract][canonical_finish/if_control] {error}"
            ),
            Self::Identity(error) => write!(
                formatter,
                "[freeze:contract][canonical_finish/identity] {error}"
            ),
            Self::Phi(error) => {
                write!(formatter, "[freeze:contract][canonical_finish/phi] {error}")
            }
            Self::CheckedCallOut(error) => write!(
                formatter,
                "[freeze:contract][canonical_finish/checked_callout] {error}"
            ),
            Self::Binding(error) => write!(
                formatter,
                "[freeze:contract][canonical_finish/binding] {error}"
            ),
            Self::Completion(error) => write!(
                formatter,
                "[freeze:contract][canonical_finish/completion] {error}"
            ),
            Self::ReturnOperandMissing => {
                formatter.write_str("[freeze:contract][canonical_finish/return_operand_missing]")
            }
        }
    }
}

pub(in crate::mir::builder::resolved_lowering) fn finish_profile_close(
    owner: FunctionOwnerIdV1,
    terminal_block: BasicBlockId,
    close: impl FnOnce() -> Result<(), String>,
) -> Result<ReadyCanonicalProfileCloseV1, String> {
    close()?;
    Ok(ReadyCanonicalProfileCloseV1::from_closed_profile(
        owner,
        terminal_block,
    ))
}

impl<'source> CanonicalSsaFunctionSessionV2<'source> {
    pub(in crate::mir::builder::resolved_lowering) fn new(
        input: ResolvedFunctionLoweringInputV1<'source>,
        if_control: VerifiedResolvedFunctionIfControlV1,
        completion: VerifiedFunctionCompletionV1,
        block_expr_count: usize,
    ) -> Result<Self, String> {
        let implicit_completion = completion.is_implicit_void();
        let completion = ResolvedFunctionCompletionConsumptionV1::new(input.owner(), completion)?;
        Self::from_consumption(
            input,
            if_control,
            completion,
            block_expr_count,
            implicit_completion,
        )
    }

    /// Generic-only session opener.  It consumes the resolver-issued outer-If
    /// and borrows Completion while projecting the typed BlockExpr count; no
    /// S6C envelope or common-V2 admission is involved.
    pub(in crate::mir::builder::resolved_lowering) fn new_generic(
        input: ResolvedFunctionLoweringInputV1<'source>,
        if_control: VerifiedResolvedFunctionIfControlV1,
        expectation: &crate::mir::resolved_semantics::VerifiedResolvedBlockExpressionExpectationV1,
        completion: &VerifiedFunctionCompletionV1,
    ) -> Result<Self, String> {
        if expectation.owner() != input.owner()
            || expectation.function_origin() != input.function().function_origin()
            || expectation.body_root() != input.function().root_profile().body_root()
            || if_control.owner() != input.owner()
            || completion.owner() != input.owner()
            || completion.target_function() != input.function().function_region()
        {
            return Err("[freeze:contract][generic_session/source_cohort_mismatch]".to_owned());
        }
        let block_expr_count = usize::try_from(expectation.pair_count())
            .map_err(|_| "[freeze:contract][generic_session/block_expr_overflow]".to_owned())?;
        let implicit_completion = completion.is_implicit_void();
        let completion =
            ResolvedFunctionCompletionConsumptionV1::new_borrowed(input.owner(), completion)?;
        Self::from_consumption(
            input,
            if_control,
            completion,
            block_expr_count,
            implicit_completion,
        )
    }

    fn from_consumption(
        input: ResolvedFunctionLoweringInputV1<'source>,
        if_control: VerifiedResolvedFunctionIfControlV1,
        completion: ResolvedFunctionCompletionConsumptionV1,
        block_expr_count: usize,
        implicit_completion: bool,
    ) -> Result<Self, String> {
        let if_controls = if_control.row_count();
        let if_branches = if_controls + if_control.explicit_else_count();
        let semantics = ResolvedSemanticStackV1::new_with_expectations(
            input.function(),
            input.function().lowering_roots(),
            ResolvedSemanticExpectedCountsV1::new(block_expr_count, if_controls, if_branches),
        )?;
        let root_body = input
            .source()
            .root_body()
            .map_err(|error| error.to_string())?;
        let root_body_end = u32::try_from(root_body.statements().len()).map_err(|_| {
            "[freeze:contract][canonical_completion/body_length_overflow]".to_string()
        })?;
        let target_function = input.function().function_region();
        Ok(Self {
            owner: input.owner(),
            root_body: root_body.site().clone(),
            root_body_end,
            target_function,
            identity: ResolvedSsaIdentityStateV2::new(input.function()),
            semantics,
            if_control: CanonicalIfControlConsumptionV1::Resolved(if_control.into_use_ledger()),
            completion,
            control: std::cell::OnceCell::new(),

            implicit_completion,
            generic_entry_adopted: false,
        })
    }

    pub(in crate::mir::builder::resolved_lowering) const fn completion_is_implicit(&self) -> bool {
        self.implicit_completion
    }

    pub(in crate::mir::builder::resolved_lowering) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir::builder::resolved_lowering) fn entry_block(
        &self,
        builder: &MirBuilder,
    ) -> Result<BasicBlockId, String> {
        let entry = builder
            .function_state
            .current_function
            .as_ref()
            .map(|function| function.entry_block)
            .ok_or_else(|| "canonical physical target requires current function".to_owned())?;
        self.bind_control(builder)?;
        Ok(entry)
    }

    /// Commit one physical value type through the canonical SSA owner.  The
    /// selected emitter may project an already-verified operation shape, but
    /// it may not write a competing type fact directly into the Builder.
    pub(in crate::mir::builder::resolved_lowering) fn publish_physical_value_type(
        &mut self,
        builder: &mut MirBuilder,
        value: ValueId,
        expected: MirType,
    ) -> Result<(), String> {
        self.bind_control(builder)?;
        match builder.function_state.type_ctx.get_type(value) {
            None | Some(MirType::Unknown) => {
                builder
                    .function_state
                    .type_ctx
                    .value_types
                    .insert(value, expected);
                Ok(())
            }
            Some(actual) if *actual == expected => Ok(()),
            Some(actual) => Err(format!(
                "[freeze:contract][canonical_physical_value/type_drift] value={value:?} expected={expected:?} actual={actual:?}"
            )),
        }
    }

    pub(in crate::mir::builder) fn adopt_generic_g0_entry_lanes(
        &mut self,
        builder: &mut MirBuilder,
        descriptors: &[crate::mir::compiler::generic_g0_physical_function_entry_input::
            GenericG0PhysicalParameterDescriptorV1],
    ) -> Result<(), String> {
        generic_g0_entry_adoption::adopt(self, builder, descriptors)?;
        self.bind_control(builder)
    }
}

#[cfg(test)]
mod tests {
    use super::finish_profile_close;
    use crate::mir::resolved_semantics::FunctionOwnerIssuerV1;
    use crate::mir::BasicBlockId;

    #[test]
    fn profile_close_receipt_is_minted_only_after_profile_success() {
        let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().expect("owner issuer");
        let owner = issuer.issue().expect("owner");
        let receipt = finish_profile_close(owner, BasicBlockId::new(7), || Ok(()))
            .expect("profile close receipt");
        assert_eq!(receipt.parts(), (owner, BasicBlockId::new(7)));
    }

    #[test]
    fn profile_close_failure_does_not_mint_a_receipt() {
        let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().expect("owner issuer");
        let owner = issuer.issue().expect("owner");
        let error = finish_profile_close(owner, BasicBlockId::new(7), || {
            Err("profile ledger remained open".to_string())
        })
        .expect_err("profile close must fail");
        assert_eq!(error, "profile ledger remained open");
    }
}
