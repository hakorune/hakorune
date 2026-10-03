//! Field-read batch issuer and stager for the ordinary-New source claims.
//!
//! Exact bodies moved verbatim from the `issue_source` parent
//! (FIELDSIZE-T0): the complete root-batch proof and the all-or-nothing
//! staging insertion. No predicate, evaluation order, error arm or
//! signature changed; this child issues no ABI and decides no admission.

use super::*;

/// Prove the complete root batch without modifying the persistent staging map.
/// A `nullable` request's class comes from `nullable_class` — the issuer's
/// sealed-call claim resolution — and enters `local_read_field` through the
/// same `alias_class` arm an earlier proven field read uses.
#[allow(clippy::too_many_arguments)]
pub(in crate::mir::normal_callable_semantic_package) fn prove_local_field_read_batch(
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    candidates: &[OrdinaryNewCandidate],
    coverage: &crate::parser::ParserOrdinaryBoxSourceCoverageV1,
    receiver_proof: Option<(BindingRefV1, &crate::parser::ParserOrdinaryBoxSourceRowV1)>,
    requests: &[crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadRequestV1],
    scalar_only: bool,
    nullable_class: &mut impl FnMut(BindingRefV1) -> Option<Box<str>>,
) -> Result<Option<Vec<(OwnedExprSiteV1, field_reads::LocalFieldRead)>>, OrdinaryNewCoSealIssueV1> {
    use crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1;
    let mut seen = BTreeSet::new();
    let mut rows = Vec::with_capacity(requests.len());
    for request in requests {
        if request.site.owner() != request.receiver.owner()
            || request.site.owner() != request.home.owner()
            || requests
                .first()
                .is_some_and(|first| first.site.owner() != request.site.owner())
        {
            return Err(OrdinaryNewCoSealIssueV1::FieldReadOwnerMismatch {
                site: request.site.clone(),
            });
        }
        if !seen.insert(request.site.clone()) {
            return Err(OrdinaryNewCoSealIssueV1::DuplicateSite {
                site: request.site.clone(),
            });
        }
        let alias_class = if request.nullable {
            match nullable_class(request.home) {
                Some(class) => Some(class),
                None => return Ok(None),
            }
        } else {
            request.alias_class.clone()
        };
        let Some((field, result)) = terminal_home::local_read_field(
            constructors,
            candidates,
            coverage,
            receiver_proof,
            &request.site,
            request.home,
            alias_class.as_deref(),
            &request.field,
        )?
        else {
            return Ok(None);
        };
        if scalar_only && result != LocalFieldReadResultV1::Scalar {
            return Ok(None);
        }
        rows.push((
            request.site.clone(),
            field_reads::LocalFieldRead {
                receiver_site: request.receiver_site.clone(),
                receiver: request.receiver,
                home: request.home,
                field,
                result,
                progress: field_reads::Progress::Pending,
            },
        ));
    }
    Ok(Some(rows))
}

/// Validate the entire batch against cached rows before inserting any new row.
pub(in crate::mir::normal_callable_semantic_package) fn stage_local_field_read_batch(
    staged: &mut BTreeMap<OwnedExprSiteV1, field_reads::LocalFieldRead>,
    rows: Vec<(OwnedExprSiteV1, field_reads::LocalFieldRead)>,
) -> Result<
    Vec<crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1>,
    OrdinaryNewCoSealIssueV1,
> {
    let mut seen = BTreeSet::new();
    for (site, row) in &rows {
        if !seen.insert(site.clone()) {
            return Err(OrdinaryNewCoSealIssueV1::DuplicateSite { site: site.clone() });
        }
        if let Some(old) = staged.get(site) {
            if old.receiver_site != row.receiver_site
                || old.receiver != row.receiver
                || old.home != row.home
                || old.field != row.field
                || old.result != row.result
            {
                return Err(OrdinaryNewCoSealIssueV1::DuplicateSite { site: site.clone() });
            }
        }
    }
    let results = rows.iter().map(|(_, row)| row.result.clone()).collect();
    for (site, row) in rows {
        staged.entry(site).or_insert(row);
    }
    Ok(results)
}
