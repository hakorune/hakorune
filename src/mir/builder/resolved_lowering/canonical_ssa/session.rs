//! One function-scoped owner bundle shared by canonical lowering profiles.

use crate::mir::builder::emission::phi_lifecycle::PhiTxn;
use crate::mir::builder::resolved_lowering::canonical_cfg::CanonicalCfgSessionV1;
use crate::mir::builder::resolved_lowering::draft_seal::ReadyFunctionDraftSealV1;
use crate::mir::builder::MirBuilder;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::located::SourceBodySiteV1;
use crate::mir::resolved_control_flow::if_control::{
    FunctionIfControlUseLedgerV1, VerifiedResolvedFunctionIfControlV1,
};
use crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1;
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, RegionId};
use crate::mir::BasicBlockId;

use super::super::completion_consumption::ResolvedFunctionCompletionConsumptionV1;
use super::super::semantic_stack::{ResolvedSemanticExpectedCountsV1, ResolvedSemanticStackV1};
use super::identity::ResolvedSsaIdentityStateV2;

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
    pub(in crate::mir::builder::resolved_lowering) if_control: FunctionIfControlUseLedgerV1,
    pub(in crate::mir::builder::resolved_lowering) completion:
        ResolvedFunctionCompletionConsumptionV1,
    pub(in crate::mir::builder::resolved_lowering) cfg: CanonicalCfgSessionV1,
    pub(in crate::mir::builder::resolved_lowering) phis: PhiTxn,
    pub(in crate::mir::builder::resolved_lowering) implicit_completion: bool,
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
        let implicit_completion = completion.is_implicit_void();
        let target_function = input.function().function_region();
        let completion = ResolvedFunctionCompletionConsumptionV1::new(input.owner(), completion)?;
        Ok(Self {
            owner: input.owner(),
            root_body: root_body.site().clone(),
            root_body_end,
            target_function,
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

    pub(in crate::mir::builder::resolved_lowering) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir::builder::resolved_lowering) fn finish_for_draft_seal(
        self,
        builder: &mut MirBuilder,
        profile_close: ReadyCanonicalProfileCloseV1,
    ) -> Result<ReadyFunctionDraftSealV1, CanonicalFunctionFinishErrorV1> {
        let (profile_owner, terminal_block) = profile_close.parts();
        if profile_owner != self.owner {
            return Err(CanonicalFunctionFinishErrorV1::ProfileOwnerMismatch);
        }
        if builder.function_state.current_block != Some(terminal_block) {
            return Err(CanonicalFunctionFinishErrorV1::TerminalBlockMismatch);
        }
        let Self {
            owner,
            root_body,
            root_body_end,
            target_function,
            identity,
            semantics,
            if_control,
            completion,
            cfg,
            phis,
            ..
        } = self;
        let function = builder
            .function_state
            .current_function
            .as_ref()
            .ok_or(CanonicalFunctionFinishErrorV1::FunctionMissing)?;
        cfg.finish(function)
            .map_err(|error| CanonicalFunctionFinishErrorV1::Cfg(error.to_string()))?;
        semantics
            .finish()
            .map_err(CanonicalFunctionFinishErrorV1::Semantic)?;
        if_control
            .finish()
            .map_err(|error| CanonicalFunctionFinishErrorV1::IfControl(format!("{error:?}")))?;
        identity
            .finish()
            .map_err(CanonicalFunctionFinishErrorV1::Identity)?;
        phis.commit(builder)
            .map_err(|error| CanonicalFunctionFinishErrorV1::Phi(error.to_string()))?;
        builder
            .function_state
            .resolved_binding_state
            .finish(owner)
            .map_err(CanonicalFunctionFinishErrorV1::Binding)?;
        let completion = completion
            .finish(&root_body, root_body_end, target_function)
            .map_err(CanonicalFunctionFinishErrorV1::Completion)?;
        if completion.returns_value() && completion.explicit_claims().is_empty() {
            return Err(CanonicalFunctionFinishErrorV1::ReturnOperandMissing);
        }
        Ok(ReadyFunctionDraftSealV1::from_v2_finish(
            completion,
            terminal_block,
        ))
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
