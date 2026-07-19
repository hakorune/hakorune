//! Borrowed source-proof gate for exact-i64 activation selection.
//!
//! This module decides only whether one observed source site may become a
//! selected activation row. It owns neither the final activation row nor any
//! lowered ValueId/type fact.

use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, SameModuleCallableNamespaceV1,
    VerifiedSameModuleCallableDeclarationCatalogV1,
};
use crate::mir::resolved_semantics::SourceExprSiteV1;
use crate::mir::source_call_target::{
    VerifiedSourceStaticCallTargetCatalogV1, VerifiedSourceStaticCallTargetV1,
};

use super::{
    CallableResultActivationErrorV1, VerifiedCallableResultCallSiteV1,
    VerifiedCallableResultDispositionV1, VerifiedSameModuleCallableResultCatalogV1,
};

/// Construction-only explanation for an ordinary raw activation row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CallableResultActivationUnselectedReasonV1 {
    NoStaticSourceTarget,
    TargetResultUnavailable,
    RequiredArgumentSourceProofUnavailable,
}

/// Borrowed source gate result. It is intentionally discarded after the
/// owning activation row has copied only the canonical selected disposition.
#[derive(Debug)]
pub(crate) enum CallableResultActivationSourceDecisionV1<'result, 'target> {
    Selected(VerifiedExactI64ActivationSourceSiteV1<'result, 'target>),
    Unselected(CallableResultActivationUnselectedReasonV1),
}

/// Co-sealed source facts for one selected exact-i64 activation site.
#[derive(Debug)]
pub(crate) struct VerifiedExactI64ActivationSourceSiteV1<'result, 'target> {
    source_target: &'target VerifiedSourceStaticCallTargetV1,
    call_result: &'result VerifiedCallableResultCallSiteV1<'target>,
}

impl<'result, 'target> VerifiedExactI64ActivationSourceSiteV1<'result, 'target> {
    pub(crate) fn target(&self) -> &CanonicalSameModuleCallableKeyV1 {
        self.source_target.target()
    }

    pub(crate) fn required_i64_arguments(&self) -> &[u32] {
        self.call_result
            .same_module_static_evidence()
            .expect("selected source gate seals same-module-static evidence")
            .1
    }
}

/// Classifies one source site without constructing an owned activation row.
pub(crate) fn classify_activation_source_site_v1<'result, 'target, 'catalog>(
    declarations: &VerifiedSameModuleCallableDeclarationCatalogV1,
    caller: &CanonicalSameModuleCallableKeyV1,
    site: &SourceExprSiteV1,
    targets: &'target VerifiedSourceStaticCallTargetCatalogV1<'catalog>,
    results: &'result VerifiedSameModuleCallableResultCatalogV1<'target, 'catalog>,
) -> Result<
    CallableResultActivationSourceDecisionV1<'result, 'target>,
    CallableResultActivationErrorV1,
> {
    if !results.is_branded_by(declarations, targets) {
        return Err(CallableResultActivationErrorV1::BorrowedResultCatalogBrandMismatch);
    }

    let Some(source_target) = targets.target(caller, site) else {
        return Ok(CallableResultActivationSourceDecisionV1::Unselected(
            CallableResultActivationUnselectedReasonV1::NoStaticSourceTarget,
        ));
    };
    let target = source_target.target();
    if target.namespace() != SameModuleCallableNamespaceV1::StaticBoxMethod {
        return Err(
            CallableResultActivationErrorV1::SelectedTargetMustBeStatic {
                caller: caller.clone(),
                site: site.clone(),
                target: target.clone(),
            },
        );
    }

    let Some(VerifiedCallableResultDispositionV1::ExactI64 {
        required_i64_arguments,
    }) = results.disposition(target)
    else {
        return Ok(CallableResultActivationSourceDecisionV1::Unselected(
            CallableResultActivationUnselectedReasonV1::TargetResultUnavailable,
        ));
    };

    validate_required_arguments(target, site, required_i64_arguments)?;

    let Some(call_result) = results.call_result(caller, site) else {
        return Ok(CallableResultActivationSourceDecisionV1::Unselected(
            CallableResultActivationUnselectedReasonV1::RequiredArgumentSourceProofUnavailable,
        ));
    };

    let Some((evidence_target, evidence_required_i64_arguments)) =
        call_result.same_module_static_evidence()
    else {
        return Err(
            CallableResultActivationErrorV1::StaticSourceTargetEvidenceMismatch {
                caller: caller.clone(),
                site: site.clone(),
                target: target.clone(),
            },
        );
    };

    if !std::ptr::eq(source_target, evidence_target) {
        return Err(
            CallableResultActivationErrorV1::StaticSourceTargetEvidenceMismatch {
                caller: caller.clone(),
                site: site.clone(),
                target: target.clone(),
            },
        );
    }
    if evidence_required_i64_arguments != required_i64_arguments.as_ref() {
        return Err(
            CallableResultActivationErrorV1::CalleeRequiredArgumentOrdinalMismatch {
                caller: caller.clone(),
                site: site.clone(),
                target: target.clone(),
                target_required_i64_arguments: required_i64_arguments.clone(),
                evidence_required_i64_arguments: evidence_required_i64_arguments.into(),
            },
        );
    }

    Ok(CallableResultActivationSourceDecisionV1::Selected(
        VerifiedExactI64ActivationSourceSiteV1 {
            source_target,
            call_result,
        },
    ))
}

fn validate_required_arguments(
    target: &CanonicalSameModuleCallableKeyV1,
    site: &SourceExprSiteV1,
    required: &[u32],
) -> Result<(), CallableResultActivationErrorV1> {
    if let Some(ordinal) = required
        .iter()
        .copied()
        .find(|ordinal| *ordinal >= target.arity())
    {
        return Err(
            CallableResultActivationErrorV1::RequiredCallArgumentOutOfRange {
                target: target.clone(),
                site: site.clone(),
                ordinal,
                arity: target.arity(),
            },
        );
    }
    Ok(())
}
