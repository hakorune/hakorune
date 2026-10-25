//! Field-read batch issuer and stager for the ordinary-New source claims.
//!
//! Declaration-only callers keep their established Scalar/Alias contract.
//! Whole-expression requests additionally retain the SAME original value loan;
//! the all-or-nothing stager cannot silently drop or substitute that loan.

use super::*;
use crate::mir::normal_callable_semantic_package::{core_method_source, LocalFieldReadSourceV1};

/// Source class agreement is independent of executable formal transport.
/// Preserve all original ingress and class vetoes for the requesting owner.
pub(in crate::mir::normal_callable_semantic_package) fn source_formal_class_v1(
    source: &Result<
        super::super::super::lexical_instance_call::PreparedBorrowedFormalIngressV1,
        String,
    >,
    formal: BindingRefV1,
) -> Result<Option<Box<str>>, String> {
    source
        .as_ref()
        .map_err(Clone::clone)?
        .source_object_view_v1(formal)
        .map(|view| view.map(|view| view.class().into()))
}

/// Prove the complete root batch without modifying the persistent staging map.
/// A `nullable` request's class comes from `nullable_class` — the issuer's
/// sealed-call claim resolution — and a `formal` request's class comes from
/// `formal_class` — original borrowed-formal source class agreement. Both enter
/// `local_read_field` through the same `alias_class` arm an earlier proven
/// field read uses; neither mints a release or ownership transfer.
#[allow(clippy::too_many_arguments)]
pub(in crate::mir::normal_callable_semantic_package) fn prove_local_field_read_batch(
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    candidates: &[OrdinaryNewCandidate],
    coverage: &crate::parser::ParserOrdinaryBoxSourceCoverageV1,
    receiver_proof: Option<(BindingRefV1, &crate::parser::ParserOrdinaryBoxSourceRowV1)>,
    entry_home: Option<&crate::mir::resolved_semantics::VerifiedInstanceEntryHomeLoanV1>,
    source_core_calls: Option<&core_method_source::SourceCoreMethodCallRowsV1>,
    requests: &[crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadRequestV1],
    demand: crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadDemandV1,
    results: &BorrowedResultCohortV1,
    ingress: &Result<
        super::super::super::lexical_instance_call::PreparedBorrowedFormalIngressV1,
        String,
    >,
    nullable_class: &mut impl FnMut(BindingRefV1) -> Option<Box<str>>,
    formal_class: &mut impl FnMut(BindingRefV1) -> Result<Option<Box<str>>, String>,
    received_context: Option<field_reads::ReceivedFieldContextV1<'_>>,
) -> Result<Option<Vec<(OwnedExprSiteV1, field_reads::LocalFieldRead)>>, OrdinaryNewCoSealIssueV1> {
    use crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadDemandV1;
    use crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1;
    let mut seen = BTreeSet::new();
    let mut rows: Vec<(OwnedExprSiteV1, field_reads::LocalFieldRead)> =
        Vec::with_capacity(requests.len());
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
        let parent = match request.path.as_ref().and_then(|path| path.parent()) {
            Some(site) => {
                let Some(parent) = rows
                    .iter()
                    .find(|(actual, _)| *actual == site)
                    .and_then(|(_, row)| row.source.statement_path())
                    .cloned()
                else {
                    return Ok(None);
                };
                Some(parent)
            }
            None => None,
        };
        if let Some(path) = &request.path {
            if request.nullable
                || request.formal
                || request.owned_ready.is_some()
                || !matches!(&demand, LocalFieldReadDemandV1::SourceExpression { plan }
                    if Rc::ptr_eq(plan, path.plan()) && request.node_in_plan(plan) == Some(path.node()))
            {
                return Ok(None);
            }
        }
        let stored_get = matches!(request.received, Some(crate::mir::resolved_semantics::home_new_prefix::ReceivedFieldReadRequestV1::StoredMapGet { .. }));
        if request.received.is_some()
            && !stored_get
            && !matches!(&demand, LocalFieldReadDemandV1::SourceExpression { plan }
            if matches!(plan.root(), crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::Statement {
                role: crate::mir::resolved_value_profile::expression_root::StatementExpressionRoleV1::PrintValue, .. }))
        {
            return Ok(None);
        }
        let received = if request.received.is_some() && !stored_get {
            let context = received_context.ok_or_else(|| {
                OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                    site: request.site.clone(),
                    issue: "received-field/context-missing".into(),
                }
            })?;
            Some(
                field_reads::ReceivedFieldReadSourceV1::issue(
                    context,
                    ingress.as_ref().map_err(Clone::clone).map_err(|issue| {
                        OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                            site: request.site.clone(),
                            issue,
                        }
                    })?,
                    request,
                )
                .map_err(|issue| {
                    OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                        site: request.site.clone(),
                        issue,
                    }
                })?,
            )
        } else {
            None
        };
        let alias_class = if let Some(site) = request.path.as_ref().and_then(|path| path.parent()) {
            let Some((_, row)) = rows.iter().find(|(actual, _)| *actual == site) else { return Ok(None); };
            let LocalFieldReadResultV1::Alias(class) = row.source.result() else { return Ok(None); };
            Some(class.clone())
        } else if let Some(crate::mir::resolved_semantics::home_new_prefix::ReceivedFieldReadRequestV1::StoredMapGet { frame, .. }) = &request.received {
            Some(frame.source().storage().payload_class_v1().into())
        } else if let Some(source) = &received { Some(source.class().into()) } else if request.nullable {
            match nullable_class(request.home) {
                Some(class) => Some(class),
                None => return Ok(None),
            }
        } else if request.formal {
            match formal_class(request.home).map_err(|issue| {
                OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                    site: request.site.clone(),
                    issue,
                }
            })? {
                Some(class) => Some(class),
                None => return Ok(None),
            }
        } else {
            request.alias_class.clone()
        };
        let object_obligation = matches!(&demand, LocalFieldReadDemandV1::SourceExpression { plan }
            if request.is_nullable_object_comparison_v1(plan));
        let object_source = if object_obligation {
            match (received_context, receiver_proof) {
                (Some((batch, _, classes, _, _)), Some((binding, parent)))
                    if binding == request.home && binding == request.receiver
                        && !request.nullable && !request.formal && alias_class.is_none() =>
                {
                    super::field_source::dynamic_stored_child_field_v1(batch, constructors, classes,
                        parent.name(), &request.field)
                        .map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress { site: request.site.clone(), issue })?
                        .map(|loan| LocalFieldReadSourceV1::from_declaration(loan.field(),
                            LocalFieldReadResultV1::BorrowedObject(Rc::new(loan)), None))
                },
                _ => None,
            }
        } else { None };
        // A syntactic handle obligation cannot fall back to an Integer view.
        if object_obligation && object_source.is_none() { return Ok(None); }
        let source = if object_source.is_some() { object_source } else if request.owned_ready.is_some() {
            let Some((field, declared)) = terminal_home::local_read_field_declaration(
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
            // Retain the original absent declaration. Only the exact successful
            // writer can lend Scalar here; ordinary untyped reads stay closed.
            let ready = field_reads::OwnedReadyFieldReadSourceV1::issue(
                candidates,
                request,
                field,
                declared.as_deref(),
            )
            .map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                site: request.site.clone(),
                issue,
            })?;
            let descriptor = LocalFieldReadSourceV1::from_declaration(
                field,
                LocalFieldReadResultV1::Scalar,
                declared.map(Into::into),
            );
            Some(
                LocalFieldReadSourceV1::with_owned_ready(&descriptor, ready).map_err(|issue| {
                    OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                        site: request.site.clone(),
                        issue,
                    }
                })?,
            )
        } else {
            terminal_home::local_read_field(
                constructors,
                candidates,
                coverage,
                receiver_proof,
                &request.site,
                request.home,
                alias_class.as_deref(),
                &request.field,
            )?
        };
        let source = match source {
            Some(source) => source,
            None => {
                // An absent annotation stays Dynamic. Only the SAME complete
                // all-writer value proof can lend the Integer view of this read.
                let numeric = if matches!(demand, LocalFieldReadDemandV1::SourceExpression { .. }) {
                    match terminal_home::local_read_field_declaration(constructors, candidates, coverage,
                        receiver_proof, &request.site, request.home, alias_class.as_deref(), &request.field)? {
                        Some((field, None)) if ingress.as_ref().is_ok_and(|ingress|
                            super::super::super::lexical_instance_call::ClosedNumericFieldSourceLoanV1::from_declaration(
                                results, ingress, field, None, &request.field).is_ok()) =>
                            Some(LocalFieldReadSourceV1::from_declaration(field, LocalFieldReadResultV1::Scalar, None)),
                        _ => None,
                    }
                } else {
                    None
                };
                if let Some(source) = numeric {
                    source
                } else {
                    let Some(source) = borrow_array_field_source(
                        constructors,
                        candidates,
                        coverage,
                        receiver_proof,
                        request,
                        alias_class.as_deref(),
                        source_core_calls,
                    )?
                    else {
                        return Ok(None);
                    };
                    source
                }
            }
        };
        let scalar_only = match &demand {
            LocalFieldReadDemandV1::Declared { scalar_only } => *scalar_only,
            LocalFieldReadDemandV1::SourceExpression { .. } => !request
                .path
                .as_ref()
                .is_some_and(|path| path.is_intermediate()),
        };
        if scalar_only && *source.result() != LocalFieldReadResultV1::Scalar
            && !(object_obligation && matches!(source.result(), LocalFieldReadResultV1::BorrowedObject(_))) {
            return Ok(None);
        }
        let mut source = match received.as_ref().filter(|_| request.path.is_none()) {
            Some(received) => LocalFieldReadSourceV1::with_received(&source, received.clone())
                .map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                    site: request.site.clone(),
                    issue,
                })?,
            None => source,
        };
        if request.path.is_some() {
            let root = if parent.is_none() {
                Some(
                    terminal_home::ExpressionFieldRootSourceV1::issue(
                        constructors,
                        candidates,
                        coverage,
                        receiver_proof,
                        entry_home,
                        received.as_ref(),
                        request,
                        source.field(),
                    )
                    .map_err(|issue| {
                        OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                            site: request.site.clone(),
                            issue,
                        }
                    })?,
                )
            } else {
                None
            };
            let returned = if request
                .path
                .as_ref()
                .is_some_and(|path| path.is_intermediate())
                && received.is_some()
            {
                let (batch, selected, classes, targets, fields) =
                    received_context.expect("received context checked");
                let loan = classes.returned_field_source_v1(batch, selected, constructors, &request.site,
                    &mut |site| super::super::super::lexical_instance_call::original_lexical_target_at_v1(targets, site)
                        .map_err(|_| "expression-path/returned-target"))
                    .map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress { site: request.site.clone(), issue: issue.into() })?;
                if let Some(loan) = &loan {
                    classes.recheck_returned_field_graph_v1(loan, fields,
                        &mut |site| super::super::super::lexical_instance_call::original_lexical_target_at_v1(targets, site)
                            .map_err(|_| "expression-path/returned-target"))
                        .map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress { site: request.site.clone(), issue: issue.into() })?;
                }
                loan
            } else {
                None
            };
            let path = field_reads::StatementFieldPathSourceV1::issue(
                request,
                Rc::clone(&source),
                parent,
                root,
                returned,
                results,
                ingress.as_ref().map_err(|issue| {
                    OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                        site: request.site.clone(),
                        issue: issue.clone(),
                    }
                })?,
            )
            .map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                site: request.site.clone(),
                issue,
            })?;
            source = LocalFieldReadSourceV1::with_statement_path(&source, path);
        }
        let numeric_source = match &demand {
            LocalFieldReadDemandV1::Declared { .. } => None,
            LocalFieldReadDemandV1::SourceExpression { .. }
                if request.path.is_some() || request.owned_ready.is_some()
                    || matches!(source.result(), LocalFieldReadResultV1::BorrowedObject(_)) =>
            {
                None
            }
            LocalFieldReadDemandV1::SourceExpression { plan } => Some(
                super::super::super::lexical_instance_call::ExpressionFieldSourceLoanV1::from_read(
                    results,
                    ingress.as_ref().map_err(|issue| {
                        OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                            site: request.site.clone(),
                            issue: issue.clone(),
                        }
                    })?,
                    Rc::clone(plan),
                    request,
                    &source,
                )
                .map_err(|issue| {
                    OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                        site: request.site.clone(),
                        issue,
                    }
                })?,
            ),
        };
        rows.push((
            request.site.clone(),
            field_reads::LocalFieldRead {
                local: None,
                receiver_site: request.receiver_site.clone(),
                receiver: request.receiver,
                home: request.home,
                source,
                numeric_source,
                progress: field_reads::LocalReadProgress::Pending,
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
                || old.source != row.source
                || match (&old.numeric_source, &row.numeric_source) {
                    (None, None) => false,
                    (Some(old), Some(new)) => !old.same_source(new),
                    _ => true,
                }
            {
                return Err(OrdinaryNewCoSealIssueV1::DuplicateSite { site: site.clone() });
            }
        }
    }
    let results = rows
        .iter()
        .map(|(_, row)| row.source.result().clone())
        .collect();
    for (site, row) in rows {
        staged.entry(site).or_insert(row);
    }
    Ok(results)
}

/// Lend an already-issued source residence; a declared Array name grants nothing.
fn borrow_array_field_source(
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    candidates: &[OrdinaryNewCandidate],
    coverage: &crate::parser::ParserOrdinaryBoxSourceCoverageV1,
    receiver_proof: Option<(BindingRefV1, &crate::parser::ParserOrdinaryBoxSourceRowV1)>,
    request: &crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadRequestV1,
    alias_class: Option<&str>,
    source_core_calls: Option<&core_method_source::SourceCoreMethodCallRowsV1>,
) -> Result<Option<Rc<LocalFieldReadSourceV1>>, OrdinaryNewCoSealIssueV1> {
    use crate::mir::resolved_semantics::home_new_prefix::{
        BorrowedArrayFieldSourceV1, LocalFieldReadResultV1,
    };
    let Some(calls) = source_core_calls else {
        return Ok(None);
    };
    let mut original = None;
    for row in calls.values() {
        let Some(crate::mir::resolved_semantics::NamedArrayRequirementV1::FieldResidence(source)) =
            row.contract().named_array_requirement()
        else {
            continue;
        };
        if source.owner() != request.site.owner() || source.field_site() != request.site.site() {
            continue;
        }
        if original.as_ref().is_some_and(
            |previous: &Rc<
                crate::mir::resolved_semantics::NamedArrayFieldResidenceRequirementV1,
            >| {
                previous.binding() != source.binding()
                    || previous.field() != source.field()
                    || previous.provider_caller() != source.provider_caller()
                    || previous.provider_site() != source.provider_site()
            },
        ) {
            return Err(OrdinaryNewCoSealIssueV1::FieldReadOwnerMismatch {
                site: request.site.clone(),
            });
        }
        if original.is_none() {
            original = Some(source.clone());
        }
    }
    let Some(original) = original else {
        return Ok(None);
    };
    let Some((field, declared)) = terminal_home::local_read_field_declaration(
        constructors,
        candidates,
        coverage,
        receiver_proof,
        &request.site,
        request.home,
        alias_class,
        &request.field,
    )?
    else {
        return Ok(None);
    };
    if field != original.field()
        || !constructors
            .is_array_field(field)
            .map_err(|_| OrdinaryNewCoSealIssueV1::BatchLoan)?
    {
        return Ok(None);
    }
    Ok(Some(LocalFieldReadSourceV1::from_declaration(
        field,
        LocalFieldReadResultV1::BorrowedArray(BorrowedArrayFieldSourceV1 {
            requirement: original,
        }),
        declared.map(String::into_boxed_str),
    )))
}
