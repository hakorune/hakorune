//! Project the same pending/executable source arguments without issuing new facts.
use super::*;
use super::static_source::{original_static_source_v1, validate_static_source_actuals_v1, validate_qualified_argument_sources_v1};
use crate::mir::resolved_semantics::home_new_prefix::{QualifiedStaticCallClaimV1, StaticScalarCallClaimV1};
use crate::mir::resolved_semantics::ResolvedMethodCallReceiverSourceV1;
use std::rc::Rc;

pub(in crate::mir::normal_callable_semantic_package) fn project_pending_static_source_arguments_v1(
    source: &Result<PreparedBorrowedFormalIngressV1, String>,
    pending: &PendingBorrowedFormalActualsV1,
    site: &OwnedExprSiteV1,
    claim: &QualifiedStaticCallClaimV1,
) -> Result<Option<Box<[LocalCallArgumentV1]>>, String> {
    project_pending_static_source_arguments_for_route_v1(
        source.as_ref().map_err(Clone::clone)?,
        pending,
        site,
        claim.required_i64_arguments(),
        false,
    )
}

pub(in crate::mir::normal_callable_semantic_package) fn project_pending_current_owner_static_source_arguments_v1(
    source: &Result<PreparedBorrowedFormalIngressV1, String>,
    pending: &PendingBorrowedFormalActualsV1,
    site: &OwnedExprSiteV1,
    claim: &StaticScalarCallClaimV1,
) -> Result<Option<Box<[LocalCallArgumentV1]>>, String> {
    let required = claim
        .current_owner_source_required_i64_arguments()
        .ok_or_else(|| freeze("borrowed-static/current-owner-claim-required"))?;
    let prepared = source.as_ref().map_err(Clone::clone)?;
    let original = prepared
        .source_incoming
        .static_observations()
        .get(site)
        .ok_or_else(|| freeze("borrowed-static/source-observation-missing"))?
        .as_ref()
        .map_err(Clone::clone)?;
    if !claim.corroborates_source(
        site,
        ResolvedMethodCallReceiverSourceV1::CurrentOwner,
        original.argument_sites().len() as u32,
    ) {
        return Err(freeze("borrowed-static/current-owner-claim-identity"));
    }
    if !pending.contains_key(site) {
        return Ok(None);
    }
    project_pending_static_source_arguments_for_route_v1(prepared, pending, site, required, true)
}

/// Shared argument proof for one already-issued Qualified row. Membership is
/// lent separately by the unchanged transport or the whole original cohort.
pub(super) fn checked_executable_qualified_arguments_v1<'a>(
    prepared: &PreparedBorrowedFormalIngressV1,
    call: &super::super::borrowed_formal_uses::BorrowedIncomingCallDraftV1,
    rows: &'a PreparedBorrowedCallActualsV1,
    required_i64_arguments: &[u32],
) -> Result<&'a [LocalCallArgumentV1], String> {
    if !matches!(rows.phase, BorrowedCallActualEvidencePhaseV1::Executable) {
        return Err(freeze("borrowed-static/executable-phase-required"));
    }
    let site = &call.call;
    let super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original) =
        &call.source
    else {
        return Err(freeze("borrowed-static/executable-source-kind"));
    };
    original.require_qualified_scalar_source_v1()?;
    let retained = prepared
        .source_incoming
        .static_observations()
        .get(site)
        .ok_or_else(|| freeze("borrowed-static/source-observation-missing"))?
        .as_ref()
        .map_err(Clone::clone)?;
    if !Rc::ptr_eq(original, retained)
        || original.call_site() != site
        || original.callee_owner() != call.callee
        || required_i64_arguments != original.required_i64_arguments()
    {
        return Err(freeze("borrowed-static/executable-source-identity"));
    }
    let arguments = rows.ordered_arguments_for_v1(call)?;
    for ordinal in required_i64_arguments {
        let integer = match arguments.get(*ordinal as usize) {
            Some(LocalCallArgumentV1::Integer(_) | LocalCallArgumentV1::Scalar(_)) => true,
            Some(LocalCallArgumentV1::BorrowedActual { ordinal, .. }) => rows
                .opaque_actuals
                .iter()
                .find(|actual| actual.ordinal == *ordinal)
                .is_some_and(|actual| match &actual.source {
                    BorrowedFormalActualSourceV1::Integer(_) => true,
                    BorrowedFormalActualSourceV1::Scalar {
                        kind: SourceScalarKind::Integer,
                        ..
                    } => true,
                    BorrowedFormalActualSourceV1::Forwarded { formal, .. } => {
                        prepared.candidate_integer_agreement(*formal)
                    }
                    _ => false,
                }),
            _ => false,
        };
        if !integer {
            return Err(freeze("borrowed-static/required-integer-source-unproved"));
        }
    }
    Ok(arguments)
}

pub(in crate::mir::normal_callable_semantic_package) fn project_pending_static_source_arguments_for_route_v1(
    prepared: &PreparedBorrowedFormalIngressV1,
    pending: &PendingBorrowedFormalActualsV1,
    site: &OwnedExprSiteV1,
    required_i64_arguments: &[u32],
    current_owner: bool,
) -> Result<Option<Box<[LocalCallArgumentV1]>>, String> {
    let source_qualified = if current_owner {
        None
    } else {
        prepared.scalar_qualified_source_call_v1(site)?
    };
    let has_source_fact = prepared
        .static_arguments
        .keys()
        .any(|(call, _)| call == site);
    let has_final_static = prepared.incoming.iter().any(|row| {
        &row.call == site
            && matches!(
                row.source,
                super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(ref original) if original.qualified_scalar_result_v1().is_some()
            )
    });
    let has_expression_source = pending.get(site).is_some_and(|rows| matches!(rows,
        Ok(rows) if matches!(&rows.phase, BorrowedCallActualEvidencePhaseV1::SourceStatic(identity)
            if prepared.has_signed_static_expression_input_v1(identity.source.callee_owner())
                || !identity.core_arguments.is_empty()
                || rows.has_qualified_static_argument_source_v1()
                || identity.candidates.iter().any(|actual| matches!(&actual.value, BorrowedCallActualValueV1::MethodCall(_) | BorrowedCallActualValueV1::String(_) | BorrowedCallActualValueV1::Binding(_))))));
    let retained = original_static_source_v1(prepared, site)?;
    let has_finite_forward_source = prepared.has_finite_forward_source_call_v1(site)?;
    let has_tagged_source = prepared.has_tagged_static_source_call_v1(site)?;
    let has_current_owner_source = retained.is_some_and(|row| row.is_current_owner_i64_source_v1());
    let has_static_terminal_source =
        !current_owner && retained.is_some_and(|row| prepared.is_static_terminal_source_v1(row));
    if !has_source_fact
        && !has_tagged_source
        && !has_final_static
        && !has_finite_forward_source
        && !has_static_terminal_source
        && source_qualified.is_none()
        && !has_expression_source
        && !(current_owner && has_current_owner_source)
    {
        return Ok(None);
    }
    if retained.is_none_or(|row| {
        if current_owner {
            !row.is_current_owner_i64_source_v1()
        } else {
            row.qualified_scalar_result_v1().is_none()
        }
    }) {
        return Err(freeze("borrowed-static/source-route-identity"));
    }
    let rows = pending
        .get(site)
        .ok_or_else(|| freeze("borrowed-static/source-unobserved"))?
        .as_ref()
        .map_err(Clone::clone)?;
    if matches!(rows.phase, BorrowedCallActualEvidencePhaseV1::Executable) {
        if current_owner {
            return Err(freeze(
                "borrowed-static/current-owner-source-phase-required",
            ));
        }
        let mut incoming = prepared.incoming.iter().filter(|row| &row.call == site);
        let call = incoming
            .next()
            .or(source_qualified)
            .ok_or_else(|| freeze("borrowed-static/executable-incoming-missing"))?;
        if incoming.next().is_some() {
            return Err(freeze("borrowed-static/executable-source-identity"));
        }
        return checked_executable_qualified_arguments_v1(
            prepared,
            call,
            rows,
            required_i64_arguments,
        )
        .map(|arguments| Some(arguments.into()));
    }
    if !has_source_fact
        && !has_tagged_source
        && !current_owner
        && !has_static_terminal_source
        && source_qualified.is_none()
        && !has_expression_source
        && !has_finite_forward_source
    {
        return Err(freeze("borrowed-static/source-fact-required"));
    }
    if let Some(finish) = rows.forwarded_static_finish_v1()
        .or_else(|| rows.numeric_static_finish_v1().map(|(finish, _)| finish))
    {
        use super::static_input_finish::StaticInputEvidenceV1;
        let forwarded = matches!(finish.evidence, StaticInputEvidenceV1::Forwarded);
        if !current_owner {
            if forwarded {
                return Err(freeze("borrowed-static/executable-current-owner-required"));
            }
            if !has_static_terminal_source && source_qualified.is_none() {
                return Err(freeze("borrowed-static/scalar-source-required"));
            }
        }
        let original =
            retained.ok_or_else(|| freeze("borrowed-static/source-observation-missing"))?;
        let call = if forwarded {
            if !finish.corroborates_forwarded_v1(original)
                || required_i64_arguments != original.required_i64_arguments()
            {
                return Err(freeze("borrowed-static/executable-source-drift"));
            }
            let cohort = prepared.static_incoming_cohort_v1(original)?;
            if !super::static_selected_actual::retains_forwarded_static_cohort_v1(prepared, original, &cohort) {
                return Err(freeze("borrowed-static/executable-whole-cohort"));
            }
            if prepared.target_static.contains_key(&original.callee_owner()) {
                for sibling in &cohort {
                    let ready = pending
                        .get(sibling.call_site())
                        .ok_or_else(|| freeze("borrowed-static/executable-sibling-missing"))?
                        .as_ref()
                        .map_err(Clone::clone)?;
                    let Some(proof) = ready.forwarded_static_finish_v1() else {
                        return Err(freeze("borrowed-static/executable-sibling-unfinished"));
                    };
                    if !proof.corroborates_forwarded_v1(sibling) {
                        return Err(freeze("borrowed-static/executable-sibling-drift"));
                    }
                }
            }
            super::static_selected_actual::forwarded_static_incoming_row_v1(
                prepared,
                original,
                "borrowed-static/executable-incoming-missing",
                "borrowed-static/executable-incoming-duplicate",
            )?
        } else {
            if required_i64_arguments != original.required_i64_arguments()
                || !(required_i64_arguments == [0] || required_i64_arguments.is_empty())
            {
                return Err(freeze("borrowed-static/scalar-required-integer-drift"));
            }
            let cohort = super::static_numeric_evidence::checked_ready_scalar_rows_v1(
                prepared,
                pending,
                original,
                &finish.completion,
            )?
            .ok_or_else(|| freeze("borrowed-static/scalar-sibling-unfinished"))?;
            cohort
                .iter()
                .copied()
                .find(|row| row.call == *site)
                .ok_or_else(|| freeze("borrowed-static/scalar-incoming-missing"))?
        };
        return rows.ordered_arguments_for_v1(call).map(|args| Some(args.into()));
    }
    let identity = validate_static_source_actuals_v1(
        prepared,
        rows,
        site,
        required_i64_arguments,
    )?;
    // Current qualified frames lend passive source arguments during Home scanning.
    // SourceStatic still owns no executable payload or physical permission.
    validate_qualified_argument_sources_v1(prepared, pending, site, identity)?;
    if required_i64_arguments.iter().any(|ordinal| {
        if identity.integer_evidence.get(*ordinal as usize) == Some(&true)
            || identity.argument_sources.get(ordinal).is_some_and(|slot| {
                matches!(slot,
                ArgumentExpressionV1::Qualified(frame) if frame.is_i64())
            })
        {
            return false;
        }
        let index = *ordinal as usize;
        let checked_forward = current_owner
            && matches!(
                identity.candidates.get(index).map(|row| &row.value),
                Some(BorrowedCallActualValueV1::SelfRooted { .. })
            )
            && identity
                .source
                .parameters()
                .get(index)
                .is_some_and(|formal| {
                    formal.kind.is_ordinary_borrowed_handle()
                        && prepared.checked_static_input(formal.binding)
                });
        !checked_forward
    }) {
        if current_owner {
            return Ok(None);
        }
        return Err(freeze("borrowed-static/required-integer-source-unproved"));
    }
    Ok(Some(rows.ordered_arguments.clone()))
}
