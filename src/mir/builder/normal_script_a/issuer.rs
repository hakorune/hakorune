//! Private Script A issuer and immediate C handoff.
//!
//! A consumes only the pre-effect, AST-free source products.  It performs the
//! one correspondence pass over resolver method rows, source coverage,
//! continuation, and lookup rows.  The capability never leaves this module:
//! it is moved directly into the named C disposition issuer.

use std::collections::{BTreeMap, BTreeSet};

use crate::mir::builder::normal_script_direct_static_recipe::validate_terminal_relation;
use crate::mir::builder::normal_script_root_demand_window::PreparedScriptRootAdmissionV1;
use crate::mir::builder::normal_script_semantic_lowering_input::
    CanonicalScriptACompleteZeroKindV1;
use crate::mir::builder::normal_script_semantic_source::ScriptSemanticSourcePreEffectPartsV1;
use crate::mir::callable_result_representation::VerifiedCallableResultRepresentationV1;
use crate::mir::resolved_semantics::{
    ResolvedMethodCallReceiverSourceV1, SourceExprSiteV1, VerifiedResolvedScriptV1,
    VerifiedSemanticOwnerProductV1,
};
use crate::mir::source_call_target::{
    VerifiedScriptCallCoverageDispositionV1, VerifiedScriptDirectStaticCallLookupRowV1,
    VerifiedScriptDirectStaticCallLookupV1,
};
use crate::parser::ParserInvocationWitnessV1;

use super::model::{
    CanonicalScriptADirectRowsV1,
    CanonicalScriptAIntegrityInvalidV1, CanonicalScriptAIssueV1,
    CanonicalScriptCDispositionV1, CanonicalScriptCTransportV1,
};
use super::required_argument_source::issue_required_argument_source;

#[derive(Debug)]
struct CanonicalScriptASourceCapabilityV1 {
    source_window: PreparedScriptRootAdmissionV1,
    invocation: ParserInvocationWitnessV1,
    parts: ScriptSemanticSourcePreEffectPartsV1,
    source_owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    observed_method_calls: usize,
    lookup_rows: BTreeMap<SourceExprSiteV1, VerifiedScriptDirectStaticCallLookupRowV1>,
    non_direct_rows: BTreeMap<
        SourceExprSiteV1,
        crate::mir::builder::normal_script_semantic_lowering_input::CanonicalScriptANonDirectRowV1,
    >,
    required_argument_rows: BTreeMap<
        SourceExprSiteV1,
        crate::mir::builder::normal_script_direct_static_join_handoff::
            ScriptDirectStaticRequiredArgumentProofDispositionV1,
    >,
}

pub(in crate::mir::builder) fn issue_into_c_transport(
    observation: super::super::PreEffectCompleteSourceObservationV1,
) -> Result<CanonicalScriptCTransportV1, CanonicalScriptAIssueV1> {
    let capability = CanonicalScriptASourceCapabilityIssuerV1::issue(observation)?;
    CanonicalScriptAObservationIssuerV1::consume(capability)
}

struct CanonicalScriptASourceCapabilityIssuerV1;

impl CanonicalScriptASourceCapabilityIssuerV1 {
    fn issue(
        observation: super::super::PreEffectCompleteSourceObservationV1,
    ) -> Result<CanonicalScriptASourceCapabilityV1, CanonicalScriptAIssueV1> {
        let (source_window, invocation, parts, lookup) = observation.into_a_parts();
        Self::issue_parts(source_window, invocation, parts, lookup)
    }

    fn issue_parts(
        source_window: PreparedScriptRootAdmissionV1,
        invocation: ParserInvocationWitnessV1,
        parts: ScriptSemanticSourcePreEffectPartsV1,
        lookup: VerifiedScriptDirectStaticCallLookupV1,
    ) -> Result<CanonicalScriptASourceCapabilityV1, CanonicalScriptAIssueV1> {
        if !source_window.is_from_invocation(&invocation)
            || !lookup.is_from_invocation(&invocation)
        {
            return Err(CanonicalScriptAIssueV1::IntegrityInvalid(
                CanonicalScriptAIntegrityInvalidV1::ForeignInvocation,
            ));
        }

        let product = script_product(&parts)?;
        let source_owner = product.core().data().owner;
        if parts.continuation().owner() != source_owner {
            return Err(CanonicalScriptAIssueV1::IntegrityInvalid(
                CanonicalScriptAIntegrityInvalidV1::RootOwnerMismatch,
            ));
        }

        let method_sites = product
            .method_calls()
            .map(|(site, _)| site.clone())
            .collect::<BTreeSet<_>>();
        let coverage = lookup.source_coverage();
        if !coverage.is_from_invocation(&invocation) {
            return Err(CanonicalScriptAIssueV1::IntegrityInvalid(
                CanonicalScriptAIntegrityInvalidV1::ForeignInvocation,
            ));
        }
        if coverage.len() != method_sites.len() {
            return Err(CanonicalScriptAIssueV1::IntegrityInvalid(
                CanonicalScriptAIntegrityInvalidV1::CoverageCardinalityMismatch,
            ));
        }

        let mut candidate_sites = BTreeSet::new();
        let mut non_direct_rows = BTreeMap::new();
        let mut required_argument_rows = BTreeMap::new();
        for (site, method) in product.method_calls() {
            let Some(coverage_row) = coverage.row(site) else {
                return Err(CanonicalScriptAIssueV1::Incomplete(
                    super::model::CanonicalScriptAIncompleteV1::MethodRowMissing(site.clone()),
                ));
            };
            validate_method_shape(site, method, coverage_row)?;
            let Some(continuation_row) = parts.continuation().row(site) else {
                return Err(CanonicalScriptAIssueV1::Incomplete(
                    super::model::CanonicalScriptAIncompleteV1::ContinuationRowMissing(
                        site.clone(),
                    ),
                ));
            };
            if continuation_row.owner() != source_owner
                || continuation_row.call_site() != site
            {
                return Err(CanonicalScriptAIssueV1::IntegrityInvalid(
                    CanonicalScriptAIntegrityInvalidV1::CallSiteMismatch(site.clone()),
                ));
            }

            match coverage_row.disposition() {
                VerifiedScriptCallCoverageDispositionV1::QualifiedUnboundOrdinary => {
                    if method.receiver() != ResolvedMethodCallReceiverSourceV1::QualifiedUnbound {
                        return Err(CanonicalScriptAIssueV1::IntegrityInvalid(
                            CanonicalScriptAIntegrityInvalidV1::ReceiverMismatch(site.clone()),
                        ));
                    }
                    let Some(lookup_row) = lookup.row(site) else {
                        return Err(CanonicalScriptAIssueV1::Incomplete(
                            super::model::CanonicalScriptAIncompleteV1::LookupRowMissing(
                                site.clone(),
                            ),
                        ));
                    };
                    validate_lookup_shape(site, method, lookup_row)?;
                    if !matches!(
                        lookup_row.representation(),
                        VerifiedCallableResultRepresentationV1::ExactI64
                    ) {
                        return Err(CanonicalScriptAIssueV1::Incomplete(
                            super::model::CanonicalScriptAIncompleteV1::ResultOutsideExactI64(
                                site.clone(),
                            ),
                        ));
                    }
                    validate_candidate_terminal(site, continuation_row)?;
                    let proof = issue_required_argument_source(product, lookup_row)?;
                    candidate_sites.insert(site.clone());
                    if required_argument_rows.insert(site.clone(), proof).is_some() {
                        return Err(CanonicalScriptAIssueV1::IntegrityInvalid(
                            CanonicalScriptAIntegrityInvalidV1::CallSiteMismatch(site.clone()),
                        ));
                    }
                }
                VerifiedScriptCallCoverageDispositionV1::NonDirect(reason) => {
                    if lookup.row(site).is_some() {
                        return Err(CanonicalScriptAIssueV1::IntegrityInvalid(
                            CanonicalScriptAIntegrityInvalidV1::LookupRowUnexpected(site.clone()),
                        ));
                    }
                    let row = crate::mir::builder::normal_script_semantic_lowering_input::
                        CanonicalScriptANonDirectRowV1::from_coverage(
                        site.clone(),
                        coverage_row.receiver_site().clone(),
                        coverage_row.argument_sites().to_vec().into_boxed_slice(),
                        coverage_row.result_site().clone(),
                        reason,
                    );
                    if non_direct_rows.insert(site.clone(), row).is_some() {
                        return Err(CanonicalScriptAIssueV1::IntegrityInvalid(
                            CanonicalScriptAIntegrityInvalidV1::CallSiteMismatch(site.clone()),
                        ));
                    }
                }
            }
        }

        if candidate_sites.len() + non_direct_rows.len() != method_sites.len() {
            return Err(CanonicalScriptAIssueV1::IntegrityInvalid(
                CanonicalScriptAIntegrityInvalidV1::CoverageCardinalityMismatch,
            ));
        }

        if coverage
            .rows()
            .is_some_and(|rows| rows.keys().any(|site| !method_sites.contains(site)))
        {
            return Err(CanonicalScriptAIssueV1::IntegrityInvalid(
                CanonicalScriptAIntegrityInvalidV1::CoverageCardinalityMismatch,
            ));
        }

        let lookup_rows = lookup.into_rows();
        if lookup_rows.len() != candidate_sites.len() {
            return Err(CanonicalScriptAIssueV1::IntegrityInvalid(
                CanonicalScriptAIntegrityInvalidV1::LookupCardinalityMismatch,
            ));
        }
        for site in &candidate_sites {
            if !lookup_rows.contains_key(site) {
                return Err(CanonicalScriptAIssueV1::Incomplete(
                    super::model::CanonicalScriptAIncompleteV1::LookupRowMissing(site.clone()),
                ));
            }
        }

        Ok(CanonicalScriptASourceCapabilityV1 {
            source_window,
            invocation,
            parts,
            source_owner,
            observed_method_calls: method_sites.len(),
            lookup_rows,
            non_direct_rows,
            required_argument_rows,
        })
    }
}

struct CanonicalScriptAObservationIssuerV1;

impl CanonicalScriptAObservationIssuerV1 {
    fn consume(
        capability: CanonicalScriptASourceCapabilityV1,
    ) -> Result<CanonicalScriptCTransportV1, CanonicalScriptAIssueV1> {
        CanonicalScriptCDispositionIssuerV1::consume(capability)
    }
}

struct CanonicalScriptCDispositionIssuerV1;

impl CanonicalScriptCDispositionIssuerV1 {
    fn consume(
        capability: CanonicalScriptASourceCapabilityV1,
    ) -> Result<CanonicalScriptCTransportV1, CanonicalScriptAIssueV1> {
        let CanonicalScriptASourceCapabilityV1 {
            source_window,
            invocation,
            parts,
            source_owner,
            observed_method_calls,
            lookup_rows,
            non_direct_rows,
            required_argument_rows,
        } = capability;
        let disposition = if lookup_rows.is_empty() {
            let kind = if source_window.window().entries().is_empty() {
                CanonicalScriptACompleteZeroKindV1::EmptyScript
            } else if observed_method_calls == 0 {
                CanonicalScriptACompleteZeroKindV1::NoMethodCalls
            } else {
                CanonicalScriptACompleteZeroKindV1::ObservedNonDirect
            };
            CanonicalScriptCDispositionV1::NonDirect(
                crate::mir::builder::normal_script_semantic_lowering_input::
                    CanonicalScriptCNoDirectClaimsV1::from_issued_c(
                    kind,
                    observed_method_calls,
                    non_direct_rows.into_values().collect::<Vec<_>>().into_boxed_slice(),
                ),
            )
        } else {
            CanonicalScriptCDispositionV1::DirectStatic(CanonicalScriptADirectRowsV1::new(
                source_owner,
                observed_method_calls,
                lookup_rows,
                non_direct_rows,
                required_argument_rows,
            ))
        };
        Ok(CanonicalScriptCTransportV1::new(
            source_window,
            invocation,
            parts,
            disposition,
        ))
    }
}

fn script_product(
    parts: &ScriptSemanticSourcePreEffectPartsV1,
) -> Result<&VerifiedResolvedScriptV1, CanonicalScriptAIssueV1> {
    let [root] = parts.forest().roots() else {
        return Err(CanonicalScriptAIssueV1::Incomplete(
            super::model::CanonicalScriptAIncompleteV1::ScriptRootMissing,
        ));
    };
    parts
        .forest()
        .semantic_owner(*root)
        .and_then(VerifiedSemanticOwnerProductV1::as_script)
        .ok_or(CanonicalScriptAIssueV1::Incomplete(
            super::model::CanonicalScriptAIncompleteV1::ScriptRootNotScript,
        ))
}

fn validate_method_shape(
    site: &SourceExprSiteV1,
    method: &crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1,
    coverage: &crate::mir::source_call_target::VerifiedScriptCallCoverageRowV1,
) -> Result<(), CanonicalScriptAIssueV1> {
    if coverage.site() != site {
        return Err(CanonicalScriptAIssueV1::IntegrityInvalid(
            CanonicalScriptAIntegrityInvalidV1::CallSiteMismatch(site.clone()),
        ));
    }
    if coverage.receiver_site() != method.receiver_site() {
        return Err(CanonicalScriptAIssueV1::IntegrityInvalid(
            CanonicalScriptAIntegrityInvalidV1::ReceiverMismatch(site.clone()),
        ));
    }
    if coverage.result_site() != method.result_site() {
        return Err(CanonicalScriptAIssueV1::IntegrityInvalid(
            CanonicalScriptAIntegrityInvalidV1::ResultMismatch(site.clone()),
        ));
    }
    if coverage.argument_sites().len() != method.arguments().len() {
        return Err(CanonicalScriptAIssueV1::IntegrityInvalid(
            CanonicalScriptAIntegrityInvalidV1::ArgumentMismatch(site.clone()),
        ));
    }
    for (ordinal, (argument, coverage_site)) in method
        .arguments()
        .iter()
        .zip(coverage.argument_sites())
        .enumerate()
    {
        if argument.ordinal() != ordinal as u32 || argument.site() != coverage_site {
            return Err(CanonicalScriptAIssueV1::IntegrityInvalid(
                CanonicalScriptAIntegrityInvalidV1::ArgumentMismatch(site.clone()),
            ));
        }
    }
    Ok(())
}

fn validate_lookup_shape(
    site: &SourceExprSiteV1,
    method: &crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1,
    lookup: &VerifiedScriptDirectStaticCallLookupRowV1,
) -> Result<(), CanonicalScriptAIssueV1> {
    if lookup.site() != site
        || lookup.receiver_site() != method.receiver_site()
        || lookup.argument_sites()
            != method
                .arguments()
                .iter()
                .map(|argument| argument.site().clone())
                .collect::<Vec<_>>()
                .as_slice()
    {
        return Err(CanonicalScriptAIssueV1::IntegrityInvalid(
            CanonicalScriptAIntegrityInvalidV1::ArgumentMismatch(site.clone()),
        ));
    }
    let target = lookup.target();
    if target.namespace() != crate::mir::builder::SameModuleCallableNamespaceV1::StaticBoxMethod
        || target.name() != method.selector()
        || target.arity() != method.arity()
    {
        return Err(CanonicalScriptAIssueV1::IntegrityInvalid(
            CanonicalScriptAIntegrityInvalidV1::TargetMismatch(site.clone()),
        ));
    }
    Ok(())
}

fn validate_candidate_terminal(
    site: &SourceExprSiteV1,
    continuation: &crate::mir::builder::normal_script_source_continuation::
        VerifiedScriptSourceContinuationRowV1,
) -> Result<(), CanonicalScriptAIssueV1> {
    let statement = match continuation.terminal() {
        crate::mir::builder::normal_script_source_continuation::ScriptSourceContinuationTerminalV1::Sequence(
            statement,
        )
        | crate::mir::builder::normal_script_source_continuation::ScriptSourceContinuationTerminalV1::Return(
            statement,
        ) => statement,
    };
    validate_terminal_relation(
        continuation.terminal(),
        statement,
        site,
        continuation.parent_relations(),
    )
    .map_err(|_| {
        CanonicalScriptAIssueV1::IntegrityInvalid(
            CanonicalScriptAIntegrityInvalidV1::TerminalMismatch(site.clone()),
        )
    })
}
