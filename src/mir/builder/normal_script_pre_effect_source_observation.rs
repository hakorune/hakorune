//! AST-free Script source observation before target/Builder effects.
//!
//! The package owns the parser witness and AST.  This child is the only
//! pre-effect issuer that may move the resolver result across package install;
//! the later borrowed semantic wrapper is rebound from these owned facts and
//! never re-runs source observation.

use super::normal_script_resolution::{resolve_normal_script_source_v1, NormalScriptResolutionV1};
use super::normal_script_root_demand_window::PreparedScriptRootAdmissionV1;
use super::normal_script_semantic_source::{
    ScriptSemanticSourcePreEffectPartsV1, VerifiedScriptSemanticSourceV1,
};
use super::program_declaration_facts::PreparedNormalProgramDeclarationFactsV1;
use crate::mir::normal_callable_semantic_package::{
    InstalledNormalCallableSemanticPackageV1, VerifiedNormalCallableSemanticPackageV1,
};
use crate::mir::resolved_semantics::{FunctionSemanticResolverSessionV1, ScriptResolverDeferredV1};
use crate::mir::source_call_target::VerifiedScriptDirectStaticCallLookupV1;
use crate::parser::{
    ParserInvocationWitnessV1, ParserNormalProgramSourceLoanRejectV1,
    ParserNormalProgramSourceLoanV1,
};

#[derive(Debug)]
pub(super) struct PreEffectCompleteSourceObservationV1 {
    source_window: PreparedScriptRootAdmissionV1,
    invocation: ParserInvocationWitnessV1,
    parts: ScriptSemanticSourcePreEffectPartsV1,
    lookup: VerifiedScriptDirectStaticCallLookupV1,
    _seal: PreEffectCompleteSourceObservationSealV1,
}

#[derive(Debug)]
struct PreEffectCompleteSourceObservationSealV1;

#[derive(Debug)]
pub(super) struct PreEffectSourceObservationAfterWindowMoveV1 {
    invocation: ParserInvocationWitnessV1,
    parts: ScriptSemanticSourcePreEffectPartsV1,
    lookup: VerifiedScriptDirectStaticCallLookupV1,
    _seal: PreEffectCompleteSourceObservationSealV1,
}

#[derive(Debug)]
pub(super) enum NormalScriptPreEffectSourceObservationIssueV1 {
    SourceAuthorityUnavailable(Box<str>),
    ObservationDeferred(ScriptResolverDeferredV1),
    Incomplete(Box<str>),
    IntegrityInvalid(Box<str>),
}

#[derive(Debug)]
pub(super) enum PreEffectSourceBindIssueV1 {
    InvocationMismatch,
}

#[derive(Debug)]
pub(super) enum PreEffectSourceBindBoundaryIssueV1 {
    Loan(ParserNormalProgramSourceLoanRejectV1),
    Bind(PreEffectSourceBindIssueV1),
}

pub(super) struct BoundNormalScriptPreEffectSourceV1<'source> {
    source: VerifiedScriptSemanticSourceV1<'source>,
    lookup: VerifiedScriptDirectStaticCallLookupV1,
}

/// Sole pre-effect issuer for the AST-free Script source observation.
pub(super) struct NormalScriptPreEffectSourceObservationIssuerV1;

impl NormalScriptPreEffectSourceObservationIssuerV1 {
    pub(super) fn issue(
        package: &VerifiedNormalCallableSemanticPackageV1,
        source_window: PreparedScriptRootAdmissionV1,
        lookup: VerifiedScriptDirectStaticCallLookupV1,
        declaration_facts: &PreparedNormalProgramDeclarationFactsV1,
        resolver: &mut FunctionSemanticResolverSessionV1,
    ) -> Result<PreEffectCompleteSourceObservationV1, NormalScriptPreEffectSourceObservationIssueV1>
    {
        package
            .with_normal_program_source_loan(|loan| {
                if !source_window.is_from_invocation(loan.invocation_witness())
                    || !lookup.is_from_invocation(loan.invocation_witness())
                {
                    return Err(
                        NormalScriptPreEffectSourceObservationIssueV1::IntegrityInvalid(
                            "source window or lookup has a foreign parser invocation".into(),
                        ),
                    );
                }
                let result = resolve_normal_script_source_v1(
                    loan.program(),
                    Some(source_window.window()),
                    declaration_facts,
                    resolver,
                )
                .map_err(NormalScriptPreEffectSourceObservationIssueV1::IntegrityInvalid)?;
                let Some(result) = result else {
                    return Err(NormalScriptPreEffectSourceObservationIssueV1::Incomplete(
                        "Script admission did not produce a complete source result".into(),
                    ));
                };
                let source = match result {
                    NormalScriptResolutionV1::Complete(source) => source,
                    NormalScriptResolutionV1::Deferred(deferred) => {
                        return Err(
                            NormalScriptPreEffectSourceObservationIssueV1::ObservationDeferred(
                                deferred,
                            ),
                        )
                    }
                };
                let parts = source.into_pre_effect_parts().map_err(|error| {
                    NormalScriptPreEffectSourceObservationIssueV1::IntegrityInvalid(error.into())
                })?;
                Ok(PreEffectCompleteSourceObservationV1 {
                    source_window,
                    invocation: loan.invocation_witness().clone(),
                    parts,
                    lookup,
                    _seal: PreEffectCompleteSourceObservationSealV1,
                })
            })
            .map_err(|error| match error {
                ParserNormalProgramSourceLoanRejectV1::SourceAuthorityUnavailable(reason) => {
                    NormalScriptPreEffectSourceObservationIssueV1::SourceAuthorityUnavailable(
                        format!("{reason:?}").into(),
                    )
                }
                ParserNormalProgramSourceLoanRejectV1::Incomplete(reason) => {
                    NormalScriptPreEffectSourceObservationIssueV1::Incomplete(
                        format!("{reason:?}").into(),
                    )
                }
                ParserNormalProgramSourceLoanRejectV1::IntegrityInvalid(reason) => {
                    NormalScriptPreEffectSourceObservationIssueV1::IntegrityInvalid(
                        format!("{reason:?}").into(),
                    )
                }
            })?
    }
}

impl PreEffectCompleteSourceObservationV1 {
    pub(super) fn split_for_work_plan(
        self,
    ) -> (
        PreparedScriptRootAdmissionV1,
        PreEffectSourceObservationAfterWindowMoveV1,
    ) {
        (
            self.source_window,
            PreEffectSourceObservationAfterWindowMoveV1 {
                invocation: self.invocation,
                parts: self.parts,
                lookup: self.lookup,
                _seal: self._seal,
            },
        )
    }
}

impl PreEffectSourceObservationAfterWindowMoveV1 {
    pub(super) fn bind<'source>(
        self,
        loan: ParserNormalProgramSourceLoanV1<'source>,
    ) -> Result<BoundNormalScriptPreEffectSourceV1<'source>, PreEffectSourceBindIssueV1> {
        if !self.invocation.same_as(loan.invocation_witness()) {
            return Err(PreEffectSourceBindIssueV1::InvocationMismatch);
        }
        Ok(BoundNormalScriptPreEffectSourceV1 {
            source: VerifiedScriptSemanticSourceV1::from_pre_effect_parts(
                loan.program(),
                self.parts,
            ),
            lookup: self.lookup,
        })
    }

    pub(super) fn with_bound_source<R>(
        self,
        package: &InstalledNormalCallableSemanticPackageV1,
        callback: impl for<'source> FnOnce(
            VerifiedScriptSemanticSourceV1<'source>,
            VerifiedScriptDirectStaticCallLookupV1,
        ) -> R,
    ) -> Result<R, PreEffectSourceBindBoundaryIssueV1> {
        package
            .with_normal_program_source_loan(|loan| {
                let bound = self
                    .bind(loan)
                    .map_err(PreEffectSourceBindBoundaryIssueV1::Bind)?;
                let (source, lookup) = bound.into_parts();
                Ok(callback(source, lookup))
            })
            .map_err(PreEffectSourceBindBoundaryIssueV1::Loan)?
    }
}

impl<'source> BoundNormalScriptPreEffectSourceV1<'source> {
    pub(super) fn into_parts(
        self,
    ) -> (
        VerifiedScriptSemanticSourceV1<'source>,
        VerifiedScriptDirectStaticCallLookupV1,
    ) {
        (self.source, self.lookup)
    }
}

#[cfg(test)]
#[path = "normal_script_pre_effect_source_observation_tests.rs"]
mod tests;
