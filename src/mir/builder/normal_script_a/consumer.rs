//! Named C consumers for the private Script A disposition.
//!
//! This is the only place where the source-backed C disposition is projected
//! into the existing Script Facts/Recipe/Join lowering boundary.  The no-direct
//! arm does not manufacture empty downstream products.

use crate::mir::builder::normal_script_direct_static_join_handoff::{
    VerifiedScriptDirectStaticJoinHandoffV1,
    VerifiedScriptDirectStaticRequiredArgumentProofV1,
};
use crate::mir::builder::normal_script_direct_static_recipe::VerifiedScriptDirectStaticRecipeV1;
use crate::mir::builder::normal_script_direct_static_result_bundle::
    VerifiedScriptDirectStaticResultBundleV1;
use crate::mir::builder::normal_script_direct_static_result_publication_owner::
    VerifiedScriptDirectStaticResultPublicationOwnerV1;
use crate::mir::builder::normal_script_root_demand_window::PreparedScriptRootAdmissionV1;
use crate::mir::builder::normal_script_semantic_lowering_input::{
    ScriptDirectStaticClaimInputV1, VerifiedScriptDirectStaticClaimProductsV1,
    VerifiedScriptSemanticLoweringInputV1,
};
use crate::mir::builder::normal_script_semantic_source::VerifiedScriptSemanticSourceV1;
use crate::mir::normal_callable_semantic_package::InstalledNormalCallableSemanticPackageV1;
use crate::mir::resolved_semantics::VerifiedSemanticOwnerProductV1;
use crate::parser::{ParserNormalProgramSourceLoanRejectV1, ParserNormalProgramSourceLoanV1};

use super::model::{
    CanonicalScriptCDispositionV1, CanonicalScriptCPostWindowTransportV1,
};

#[derive(Debug)]
pub(in crate::mir::builder) enum CanonicalScriptCBindIssueV1 {
    Loan(ParserNormalProgramSourceLoanRejectV1),
    InvocationMismatch,
}

#[derive(Debug)]
pub(in crate::mir::builder) struct CanonicalScriptCBoundSourceV1<'source> {
    invocation: crate::parser::ParserInvocationWitnessV1,
    source: VerifiedScriptSemanticSourceV1<'source>,
    disposition: CanonicalScriptCDispositionV1,
}

#[derive(Debug)]
pub(in crate::mir::builder) struct CanonicalScriptCPreparedLoweringSourceV1<'source> {
    source: VerifiedScriptSemanticSourceV1<'source>,
    claim_input: ScriptDirectStaticClaimInputV1,
}

impl CanonicalScriptCPostWindowTransportV1 {
    pub(in crate::mir::builder) fn with_bound_source<R>(
        self,
        package: &InstalledNormalCallableSemanticPackageV1,
        callback: impl for<'source> FnOnce(CanonicalScriptCBoundSourceV1<'source>) -> R,
    ) -> Result<R, CanonicalScriptCBindIssueV1> {
        let (invocation, parts, disposition) = self.into_parts();
        package
            .with_normal_program_source_loan(|loan| {
                bind_source(invocation, parts, disposition, loan, callback)
            })
            .map_err(CanonicalScriptCBindIssueV1::Loan)?
    }
}

fn bind_source<'source, R>(
    invocation: crate::parser::ParserInvocationWitnessV1,
    parts: crate::mir::builder::normal_script_semantic_source::
        ScriptSemanticSourcePreEffectPartsV1,
    disposition: CanonicalScriptCDispositionV1,
    loan: ParserNormalProgramSourceLoanV1<'source>,
    callback: impl FnOnce(CanonicalScriptCBoundSourceV1<'source>) -> R,
) -> Result<R, CanonicalScriptCBindIssueV1> {
    if !invocation.same_as(loan.invocation_witness()) {
        return Err(CanonicalScriptCBindIssueV1::InvocationMismatch);
    }
    Ok(callback(CanonicalScriptCBoundSourceV1 {
        invocation,
        source: VerifiedScriptSemanticSourceV1::from_pre_effect_parts(loan.program(), parts),
        disposition,
    }))
}

impl<'source> CanonicalScriptCBoundSourceV1<'source> {
    /// The sole post-install C consumer.  It returns a lowering wrapper with
    /// a required claim disposition; no parallel optional products are made.
    pub(in crate::mir::builder) fn consume_into_lowering_source(
        self,
        admission: &PreparedScriptRootAdmissionV1,
    ) -> Result<CanonicalScriptCPreparedLoweringSourceV1<'source>, String> {
        let Self {
            invocation,
            source,
            disposition,
        } = self;
        if !admission.is_from_invocation(&invocation) {
            return Err("[freeze:contract][script-a-c/admission-invocation-mismatch]".to_owned());
        }
        let claim_input = match disposition {
            CanonicalScriptCDispositionV1::NonDirect(witness) => {
                ScriptDirectStaticClaimInputV1::CompleteNoDirectStaticClaims(witness)
            }
            CanonicalScriptCDispositionV1::DirectStatic(rows) => {
                let source_owner = source_owner(&source)?;
                let source_identity = source.source() as *const _ as usize;
                let (row_owner, lookup_rows, non_direct_rows, required_argument_rows) =
                    rows.into_parts();
                if row_owner != source_owner {
                    return Err(
                        "[freeze:contract][script-a-c/source-owner-rebind-mismatch]".to_owned(),
                    );
                }
                if non_direct_rows
                    .iter()
                    .any(|(site, row)| row.site() != site || lookup_rows.contains_key(site))
                {
                    return Err(
                        "[freeze:contract][script-a-c/non-direct-row-overlap]".to_owned(),
                    );
                }
                let bundle = VerifiedScriptDirectStaticResultBundleV1::from_canonical_a_rows(
                    source_owner,
                    source_identity,
                    lookup_rows,
                );
                let publication_owner =
                    VerifiedScriptDirectStaticResultPublicationOwnerV1::from_canonical_bundle(
                        source_owner,
                        source_identity,
                        &bundle,
                        source.continuation(),
                    )
                    .map_err(|error| {
                        format!("[freeze:contract][script-a-c/publication-owner] {error:?}")
                    })?;
                let recipe = VerifiedScriptDirectStaticRecipeV1::issue(
                    &publication_owner,
                    admission.window(),
                )
                .map_err(|error| {
                    format!("[freeze:contract][script-a-c/recipe] {error:?}")
                })?;
                let join_handoff = VerifiedScriptDirectStaticJoinHandoffV1::issue(
                    &recipe,
                    &publication_owner,
                )
                .map_err(|error| {
                    format!("[freeze:contract][script-a-c/join] {error:?}")
                })?;
                let required_argument_proof =
                    VerifiedScriptDirectStaticRequiredArgumentProofV1::from_canonical_source_rows(
                        source_owner,
                        source_identity,
                        &join_handoff,
                        required_argument_rows,
                    )
                    .map_err(|error| {
                        format!("[freeze:contract][script-a-c/required-arguments] {error:?}")
                    })?;
                ScriptDirectStaticClaimInputV1::DirectStaticClaims(
                    VerifiedScriptDirectStaticClaimProductsV1::from_canonical_c(
                        bundle,
                        publication_owner,
                        recipe,
                        join_handoff,
                        required_argument_proof,
                    ),
                )
            }
        };
        Ok(CanonicalScriptCPreparedLoweringSourceV1 {
            source,
            claim_input,
        })
    }
}

impl<'source> CanonicalScriptCPreparedLoweringSourceV1<'source> {
    pub(in crate::mir::builder) fn source(&self) -> &VerifiedScriptSemanticSourceV1<'source> {
        &self.source
    }

    pub(in crate::mir::builder) fn into_lowering_input(
        self,
    ) -> VerifiedScriptSemanticLoweringInputV1 {
        let Self {
            source,
            claim_input,
        } = self;
        let (projection, continuation) = source.into_lowering_parts();
        VerifiedScriptSemanticLoweringInputV1::new(projection, continuation, claim_input)
    }
}

fn source_owner(source: &VerifiedScriptSemanticSourceV1<'_>) -> Result<crate::mir::resolved_semantics::FunctionOwnerIdV1, String> {
    let [root] = source.forest().roots() else {
        return Err("[freeze:contract][script-a-c/source-root-cardinality]".to_owned());
    };
    source
        .forest()
        .semantic_owner(*root)
        .and_then(VerifiedSemanticOwnerProductV1::as_script)
        .map(|product| product.core().data().owner)
        .ok_or_else(|| "[freeze:contract][script-a-c/source-root-product]".to_owned())
}
