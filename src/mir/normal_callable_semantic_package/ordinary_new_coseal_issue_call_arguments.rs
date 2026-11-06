//! Source argument request dispatch, shared by every admitted Call context.
use super::*;
use super::super::lexical_instance_call::{PreparedLexicalInstanceCallSourceTargetsV1, BorrowedResultCohortV1};
use crate::mir::resolved_semantics::home_new_prefix::{BorrowedCallActualRequestV1, BorrowedCallArgumentsV1};

#[path = "ordinary_new_received_handle_arguments.rs"]
mod received_handle;
#[path = "ordinary_new_coseal_issue_lexical_received_object.rs"]
mod received_object;

/// The same callback prepares observations or demands the already-staged row.
/// This neither re-observes locals nor reads a completed caller ledger.
pub(super) fn borrowed_call_arguments_callback_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    targets: &super::super::lexical_instance_call::PreparedLexicalInstanceCallSourceTargetsV1,
    dynamic_source_calls: &[crate::mir::source_call_target::VerifiedSourceBoundDynamicMemberCallV1],
    contracts: &[super::super::super::model::OwnedCallableParameterContractDeclarationV1],
    candidates: &[OrdinaryNewCandidate],
    receiver: Option<(BindingRefV1, &crate::parser::ParserOrdinaryBoxSourceRowV1)>,
    source: &Result<super::super::lexical_instance_call::PreparedBorrowedFormalIngressV1, String>,
    static_source_sites: &Result<std::collections::BTreeSet<OwnedExprSiteV1>, String>,
    pending: &mut super::super::lexical_instance_call::PendingBorrowedFormalActualsV1,
    results: &BorrowedResultCohortV1,
    nullable_class: &mut impl FnMut(BindingRefV1) -> Option<Box<str>>,
    site: &OwnedExprSiteV1,
    request: BorrowedCallActualRequestV1<'_>,
    classes: &super::super::result_class_claim::OrdinaryNewResultClassClaimsV1,
    receiver_rows: &BTreeMap<OwnedExprSiteV1, super::super::ReceiverCallClassObservationV1>,
    field_write_claims: &field_write_claim::OrdinaryNewFieldWriteClaimsV1,
    completed: Option<&crate::mir::normal_callable_semantic_package::result_contract::VerifiedCallableResultContractBuilderV1>,
) -> Result<Option<BorrowedCallArgumentsV1>, OrdinaryNewCoSealIssueV1> {
    if let BorrowedCallActualRequestV1::CallEntryHomes(input) = request {
        return super::super::lexical_instance_call::source_take_homes_v1(pending, site, candidates)
            .map(|taken| Some(BorrowedCallArgumentsV1::CallEntryHomes {
                preserves_reads: source.as_ref().is_ok_and(|source| source.call_preserves_read_loans_v1(input, batch, contracts, site)),
                taken,
            }))
            .map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress { site: site.clone(), issue });
    }
    // The constructor/current-entry Home adapters issue this source loan.
    // A generic source-only caller has no such context and grants nothing.
    if matches!(request, BorrowedCallActualRequestV1::StoredMapGetSource) {
        return Ok(None);
    }
    if matches!(request, BorrowedCallActualRequestV1::StoredMapSource) {
        return classes
            .map_stored_source_v1(site)
            .and_then(|source| {
                if source
                    .as_ref()
                    .is_some_and(|source| !source.receiver_v1().same_source(field_write_claims))
                {
                    return Err("callee-facts-identity");
                }
                Ok(source)
            })
            .map(|source| source.map(BorrowedCallArgumentsV1::StoredMapSource))
            .map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                site: site.clone(),
                issue: issue.to_owned(),
            });
    }
    if let BorrowedCallActualRequestV1::NamedArrayHolderInitializer(binding) = request {
        return source
            .as_ref()
            .map_err(Clone::clone)
            .and_then(|source| source.named_array_holder_source_v1(site, binding, true))
            .map(|loan| loan.map(BorrowedCallArgumentsV1::NamedArrayHolderSource))
            .map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                site: site.clone(),
                issue,
            });
    }
    if let BorrowedCallActualRequestV1::NamedArrayAppendStatement { binding, source: holder } = request {
        if binding.owner() != site.owner()
            || holder.binding() != binding
            || holder.append_candidate(site).is_none()
        {
            return Err(OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                site: site.clone(),
                issue: "[freeze:contract][array-holder/append-source-drift]".into(),
            });
        }
        return Ok(Some(BorrowedCallArgumentsV1::NamedArrayHolderSource(holder.clone())));
    }
    if let BorrowedCallActualRequestV1::ExpressionCoreSource { input, plan } = request {
        return source
            .as_ref()
            .map_err(Clone::clone)
            .and_then(|source| {
                super::super::lexical_instance_call::lend_expression_core_source_v1(
                    input, source, contracts, plan, site,
                )
            })
            .map(|loan| loan.map(BorrowedCallArgumentsV1::ExpressionCoreSource))
            .map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                site: site.clone(),
                issue,
            });
    }
    if let BorrowedCallActualRequestV1::DynamicInvocationSource { input, plan } = request {
        let original_owned = super::super::lexical_instance_call::original_owned_call_target_v1(
            source, targets, classes, site,
        ).map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
            site: site.clone(), issue,
        })?.is_some();
        let lexical_result_source = super::super::lexical_instance_call::
            lexical_scalar_result_source_owns_arguments_v1(targets, site)
            .map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                site: site.clone(),
                issue,
            })?;
        // The runtime has no provider for a user-box member, so an exact static
        // target at this site is the only executable meaning of the call.
        let static_target = super::super::lexical_instance_call::original_lexical_target_at_v1(targets, site)
            .map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                site: site.clone(), issue,
            })?.is_some();
        if original_owned || lexical_result_source || static_target {
            if crate::config::env::builder_debug_enabled() {
                eprintln!("[dynamic-source] {site:?} not dynamic: original_owned={original_owned} lexical_result_source={lexical_result_source} static_target={static_target} dynamic_candidates={}",
                    dynamic_source_calls.iter().filter(|target| target.owner() == input.owner() && target.call_site() == site.site()).count());
            }
            return Ok(None);
        }
        let mut candidates = dynamic_source_calls.iter().filter(|target| {
            target.owner() == input.owner() && target.call_site() == site.site()
        });
        let Some(target) = candidates.next() else {
            if crate::config::env::builder_debug_enabled() {
                eprintln!("[dynamic-source] {site:?} has no source-bound dynamic member call");
            }
            return Ok(None);
        };
        let issue = || OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
            site: site.clone(),
            issue: "dynamic-source-target-identity".to_owned(),
        };
        let Some(call) = input.function().method_call(site.site()) else {
            return Err(issue());
        };
        if candidates.next().is_some()
            || input.owner() != site.owner()
            || plan.owner() != input.owner()
            || !plan.contains_call_site_v1(site)
            || call.owner() != input.owner()
            || target.result_site() != site.site()
            || target.receiver_binding().owner() != input.owner()
            || target.dynamic_origin().owner() != input.owner()
            || call.receiver_site() != target.receiver_site()
            || call.arity() != target.dispatch().arity()
            || call.arguments().len() != target.arguments().len()
            || call.arguments().iter().zip(target.arguments()).any(|(actual, source)| {
                actual.ordinal() != source.ordinal() || actual.site() != source.site()
            })
        {
            return Err(issue());
        }
        let arguments = target
            .arguments()
            .iter()
            .map(|argument| crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1::Expression {
                ordinal: argument.ordinal(),
                site: argument.site().clone(),
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        return Ok(Some(BorrowedCallArgumentsV1::DynamicInvocationSource {
            loan: crate::mir::dynamic_invocation_contract::DynamicInvocationSourceLoanV1::from_admitted_target_v1(target),
            arguments,
        }));
    }
    if let BorrowedCallActualRequestV1::ArgumentExpressionSource(input, ordinal) = request {
        return source
            .as_ref()
            .map_err(Clone::clone)
            .and_then(|source| {
                super::super::lexical_instance_call::observed_argument_source_v1(
                    input, source, pending, contracts, site, ordinal,
                )
            })
            .map(|loan| loan.map(BorrowedCallArgumentsV1::ArgumentExpressionSource))
            .map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                site: site.clone(),
                issue,
            });
    }
    if let BorrowedCallActualRequestV1::ArgumentExpressionCarrierArrayAppendSource {
        input,
        ordinal,
        holder,
    } = request
    {
        return super::super::lexical_instance_call::observed_carrier_array_append_argument_source_v1(
            input,
            site,
            ordinal,
            holder,
            dynamic_source_calls,
            None,
        )
            .map(|loan| loan.map(BorrowedCallArgumentsV1::ArgumentExpressionSource))
            .map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                site: site.clone(),
                issue,
            });
    }
    if let BorrowedCallActualRequestV1::CompleteArgumentExpression { input, frame } = request {
        return source
            .as_ref()
            .map_err(Clone::clone)
            .and_then(|source| {
                super::super::lexical_instance_call::complete_argument_expression_v1(
                    input,
                    source,
                    pending,
                    contracts,
                    site,
                    dynamic_source_calls,
                    frame,
                )
            })
            .map(|frame| Some(BorrowedCallArgumentsV1::ArgumentExpressionFrame(frame)))
            .map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                site: site.clone(),
                issue,
            });
    }
    if let BorrowedCallActualRequestV1::ArgumentTextEntryRead {
        source: loan,
        read,
        binding,
        root,
    } = request
    {
        return source
            .as_ref()
            .map_err(Clone::clone)
            .and_then(|source| {
                super::super::lexical_instance_call::lend_text_entry_read_v1(
                    source, pending, contracts, site, loan, read, binding, root,
                )
            })
            .map(|row| Some(BorrowedCallArgumentsV1::TextEntryRead(row)))
            .map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                site: site.clone(),
                issue,
            });
    }
    if let BorrowedCallActualRequestV1::FieldArrayTextEntryRead {
        input,
        source: field_source,
        plan,
        read,
        binding,
        root,
    } = request
    {
        return source
            .as_ref()
            .map_err(Clone::clone)
            .and_then(|source| {
                super::super::lexical_instance_call::lend_field_array_text_entry_read_v1(
                    source,
                    contracts,
                    input,
                    site,
                    field_source,
                    plan,
                    read,
                    binding,
                    root,
                )
            })
            .map(|row| Some(BorrowedCallArgumentsV1::TextEntryRead(row)))
            .map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                site: site.clone(),
                issue,
            });
    }
    if let BorrowedCallActualRequestV1::FiniteEntryRead {
        input,
        plan,
        read,
        binding,
        root,
    } = request
    {
        return source
            .as_ref()
            .map_err(Clone::clone)
            .and_then(|source| {
                super::super::lexical_instance_call::lend_finite_entry_read_v1(
                    input, source, contracts, site, plan, read, binding, root,
                )
            })
            .map(|row| row.map(BorrowedCallArgumentsV1::FiniteEntryRead))
            .map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                site: site.clone(),
                issue,
            });
    }
    if let BorrowedCallActualRequestV1::DynamicEntryRead {
        input,
        plan,
        read,
        binding,
        root,
    } = request
    {
        return source
            .as_ref()
            .map_err(Clone::clone)
            .and_then(|source| {
                super::super::lexical_instance_call::lend_dynamic_entry_read_v1(
                    input, source, contracts, site, plan, read, binding, root,
                )
            })
            .map(|row| row.map(BorrowedCallArgumentsV1::DynamicEntryRead))
            .map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                site: site.clone(),
                issue,
            });
    }
    if matches!(
        request,
        BorrowedCallActualRequestV1::SourceUnitDiscardArguments
    ) {
        return super::super::lexical_instance_call::source_unit_discard_v1(
            targets, contracts, source, pending, site,
        )
        .map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
            site: site.clone(),
            issue,
        });
    }
    super::super::lexical_instance_call::recheck_pending_array_source_actuals_v1(
        source, pending, classes, targets, site,
    )
    .map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
        site: site.clone(),
        issue,
    })?;
    if let BorrowedCallActualRequestV1::SourceResultArguments { expression } = request {
        if let Some(text) =
            super::super::lexical_instance_call::source_ordinary_result_arguments_v1(
                source, pending, contracts, site, targets, selected, completed, expression,
            )
            .map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                site: site.clone(),
                issue,
            })?
        {
            return Ok(Some(text));
        }
        return super::super::lexical_instance_call::source_scalar_result_arguments_v1(
            targets, contracts, source, pending, results, site, classes,
        )
        .map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
            site: site.clone(),
            issue,
        });
    }
    if let BorrowedCallActualRequestV1::BorrowedNullableLocalArguments(destination) = request {
        return super::super::lexical_instance_call::borrowed_nullable_local_arguments_v1(
            targets,
            contracts,
            source,
            pending,
            classes,
            site,
            Some(destination),
        )
        .map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
            site: site.clone(),
            issue,
        });
    }
    if let BorrowedCallActualRequestV1::BorrowedNullableReturnArguments(statement) = request {
        let response = super::super::lexical_instance_call::borrowed_nullable_local_arguments_v1(
            targets, contracts, source, pending, classes, site, None,
        ).map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress { site: site.clone(), issue })?;
        if response.as_ref().is_some_and(|response| !matches!(response,
            BorrowedCallArgumentsV1::BorrowedNullableReturnSource { loan, .. }
                if loan.return_statement() == Some(statement))) {
            return Err(OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                site: site.clone(), issue: "borrowed-nullable/source-identity".into(),
            });
        }
        return Ok(response);
    }
    if let BorrowedCallActualRequestV1::MixedLocalArguments(destination) = request {
        return super::super::lexical_instance_call::mixed_local_arguments_v1(
            targets,
            contracts,
            source,
            pending,
            classes,
            site,
            destination,
        )
        .map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
            site: site.clone(),
            issue,
        });
    }
    if received_object::handles(&request) {
        return received_object::project(
            request,
            batch,
            selected,
            source,
            contracts,
            pending,
            targets,
            candidates,
            classes,
            field_write_claims,
            site,
        )
        .map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
            site: site.clone(),
            issue,
        });
    }
    if let BorrowedCallActualRequestV1::ReceivedHandleArguments(destination) = request {
        return received_handle::received_handle_arguments_v1(
            targets,
            candidates,
            classes,
            site,
            destination,
            &mut |target, loan| {
                super::super::lexical_instance_call::project_pending_object_arguments_v1(
                    source, contracts, pending, target, loan, None,
                )
            },
        )
        .map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
            site: site.clone(),
            issue,
        });
    }
    if let BorrowedCallActualRequestV1::ObjectArguments(loan, destination) = request {
        let demand = || -> Result<Option<BorrowedCallArgumentsV1>, String> {
            if site != loan.call()
                || classes.object_return_qualification(loan.value()).as_ref() != Some(loan)
            {
                return Err("[freeze:contract][borrowed-call/foreign-object-qualification]".into());
            }
            let mut matching = Vec::new();
            for row in targets.as_ref().map_err(Clone::clone)? {
                if let Some(target) = row.as_ref().map_err(Clone::clone)? {
                    if target.call_site() == site {
                        matching.push(target);
                    }
                }
            }
            let target = match matching.as_slice() {
                [] => return Ok(None),
                [target] => *target,
                _ => return Err("[freeze:contract][borrowed-call/object-target-not-unique]".into()),
            };
            if let Some(destination) = destination {
                if !super::super::lexical_instance_call::corroborate_received_object_receiver_v1(
                    target,
                    loan,
                    destination,
                    receiver.map(|(binding, _)| binding),
                    receiver_rows,
                )? {
                    return Ok(None);
                }
            }
            let arguments =
                super::super::lexical_instance_call::project_pending_object_arguments_v1(
                    source, contracts, pending, target, loan, None,
                )?;
            Ok(Some(BorrowedCallArgumentsV1::Object {
                qualification: loan.clone(),
                arguments,
            }))
        };
        return demand().map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
            site: site.clone(),
            issue,
        });
    }
    if let BorrowedCallActualRequestV1::Observe(input, actuals, current_receiver) = request {
        let read_receiver = super::super::lexical_instance_call::original_lexical_target_at_v1(targets, site)
            .map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress { site: site.clone(), issue })?
            .and_then(|target| target.declared_read_receiver_v1());
        if match (read_receiver, current_receiver) {
            (None, None) => false,
            (Some(source), Some(read)) => !source.matches_value_v1(read.read()),
            _ => true,
        } {
            return Err(OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                site: site.clone(), issue: super::super::lexical_instance_call::freeze("borrowed-actual/unsupported-or-unavailable"),
            });
        }
        if (read_receiver.is_some() || actuals.iter().any(|actual| matches!(actual.value,
            crate::mir::resolved_semantics::home_new_prefix::BorrowedCallActualValueV1::DeclaredRead(_))))
            && !source.as_ref().is_ok_and(|source| source.call_preserves_read_loans_v1(input, batch, contracts, site)) {
            return Err(OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                site: site.clone(), issue: super::super::lexical_instance_call::freeze("borrowed-actual/unsupported-or-unavailable"),
            });
        }
        let ordinary = super::super::lexical_instance_call::original_owned_call_target_v1(
            source, targets, classes, site,
        ).map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
            site: site.clone(), issue,
        })?.is_some();
        let lexical_result_source = super::super::lexical_instance_call::
            lexical_scalar_result_source_owns_arguments_v1(targets, site)
            .map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                site: site.clone(),
                issue,
            })?;
        // An exact static target is the only executable meaning (the runtime has
        // no user-box provider); the Dynamic candidate then lends nothing.
        let static_target = super::super::lexical_instance_call::original_lexical_target_at_v1(targets, site)
            .map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                site: site.clone(), issue,
            })?.is_some();
        let mut dynamic_targets = dynamic_source_calls.iter().filter(|target| {
            !ordinary
                && !lexical_result_source
                && !static_target
                && target.owner() == input.owner()
                && target.call_site() == site.site()
        });
        let first_dynamic = dynamic_targets.next();
        let prepared = match (first_dynamic, dynamic_targets.next()) {
            (Some(target), None) => super::super::lexical_instance_call::
                prepare_dynamic_invocation_argument_actuals_v1(
                    input,
                    target,
                    actuals,
                    pending.get(site),
                )
                .map(Some),
            (Some(_), Some(_)) => Err("dynamic-argument/duplicate-source-target".to_owned()),
            (None, Some(_)) => Err("dynamic-argument/duplicate-source-target".to_owned()),
            (None, None) => super::super::lexical_instance_call::prepare_borrowed_call_actuals_v1(
                source,
                contracts,
                site,
                actuals,
                candidates,
                receiver,
                nullable_class,
            )
            .and_then(|mut rows| {
                super::super::lexical_instance_call::bind_argument_sources_v1(
                    input,
                    source.as_ref().map_err(Clone::clone)?,
                    Some(contracts),
                    site,
                    pending.get(site),
                    rows.as_mut(),
                )?;
                if let Some(rows) = rows.as_mut() { rows.bind_declared_receiver_v1(current_receiver)?; }
                else if current_receiver.is_some() { return Err(super::super::lexical_instance_call::freeze("borrowed-instance/receiver-source-identity")); }
                Ok(rows)
            }),
        };
        super::super::lexical_instance_call::stage_observed_call_actuals_v1(
            pending, site, prepared,
        );
        return Ok(None);
    }
    if matches!(
        request,
        BorrowedCallActualRequestV1::QualifiedStaticSourceArguments(_)
    ) {
        let sites = static_source_sites.as_ref().map_err(|issue| {
            OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                site: site.clone(),
                issue: issue.clone(),
            }
        })?;
        // Original purpose permits source projection, never executable membership.
        let scalar_source = source
            .as_ref()
            .ok()
            .map(|prepared| {
                prepared
                    .has_scalar_qualified_source_call_v1(site)
                    .and_then(|scalar| {
                        Ok(scalar || prepared.has_finite_forward_source_call_v1(site)?)
                    })
            })
            .transpose()
            .map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                site: site.clone(),
                issue,
            })?
            .unwrap_or(false);
        if !sites.contains(site)
            && !pending.get(site).is_some_and(|row| {
                row.as_ref()
                    .is_ok_and(|row| row.has_qualified_static_argument_source_v1())
            })
            && !scalar_source
            && !source
                .as_ref()
                .is_ok_and(|prepared| prepared.is_static_terminal_site_v1(site))
        {
            return Ok(None);
        }
    }
    if source.is_err() {
        // A failed preparation staged Err even for unrelated observed calls.
        // Demand that error only through the prepared source target/declaration proof.
        let borrowed_target = targets.as_ref().ok().is_some_and(|rows| {
            rows.iter()
                .filter_map(|row| row.as_ref().ok().and_then(Option::as_ref))
                .any(|target| {
                    target.call_site() == site
                        && contracts.iter().any(|contract| {
                            contract.owner == target.callee_owner()
                                && contract.batch_slot == target.target_batch_slot()
                                && contract
                                    .parameters
                                    .iter()
                                    .any(|formal| formal.kind.is_ordinary_borrowed_handle())
                        })
                })
        });
        if !borrowed_target
            && !matches!(
                request,
                BorrowedCallActualRequestV1::QualifiedStaticSourceArguments(_)
                    | BorrowedCallActualRequestV1::CurrentOwnerStaticSourceArguments(_)
            )
        {
            return Ok(None);
        }
    }
    if matches!(request, BorrowedCallActualRequestV1::I64ResultArguments) {
        // The local initializer probes scalar arguments before object acquisition.
        // A checked original owning result belongs to that object protocol;
        // probing it must not demand executable inputs before Completion.
        if super::super::lexical_instance_call::original_owned_call_target_v1(
            source, targets, classes, site,
        ).map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
            site: site.clone(), issue,
        })?.is_some() {
            return Ok(None);
        }
        let proposed = targets.as_ref().is_ok_and(|rows| rows.iter()
            .filter_map(|row| row.as_ref().ok()?.as_ref())
            .any(|target| target.call_site() == site));
        if let Some(arguments) =
            super::super::lexical_instance_call::project_pending_i64_result_arguments_v1(
                source, pending, contracts, results, site, proposed,
            )
            .map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                site: site.clone(),
                issue,
            })?
        {
            return Ok(Some(BorrowedCallArgumentsV1::Scalar(arguments)));
        }
    }
    if matches!(request, BorrowedCallActualRequestV1::ScalarArguments) {
        if let Some(arguments) =
            super::super::lexical_instance_call::project_pending_instance_source_arguments_v1(
                source, pending, contracts, site,
            )
            .map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                site: site.clone(),
                issue,
            })?
        {
            return Ok(Some(BorrowedCallArgumentsV1::SourceInstance(arguments)));
        }
    }
    match request {
        BorrowedCallActualRequestV1::QualifiedStaticSourceArguments(claim) => {
            super::super::lexical_instance_call::project_pending_static_source_arguments_v1(
                source, pending, site, claim,
            )
            .and_then(|row| {
                row.map(BorrowedCallArgumentsV1::StaticSource)
                    .map(Some)
                    .ok_or_else(|| {
                        "[freeze:contract][borrowed-static/source-selection-identity]".to_owned()
                    })
            })
        }
        BorrowedCallActualRequestV1::CurrentOwnerStaticSourceArguments(claim) => {
            super::super::lexical_instance_call::project_pending_current_owner_static_source_arguments_v1(
                source, pending, site, claim,
            )
            .map(|row| row.map(BorrowedCallArgumentsV1::StaticSource))
        }
        BorrowedCallActualRequestV1::I64ResultArguments => {
            Ok(None)
        }
        BorrowedCallActualRequestV1::ScalarArguments => {
            super::super::lexical_instance_call::project_pending_borrowed_i64_arguments_v1(
                source, pending, results, site,
            )
            .map(|row| row.map(BorrowedCallArgumentsV1::Scalar))
        }
        BorrowedCallActualRequestV1::CallEntryHomes(_)
        | BorrowedCallActualRequestV1::StoredMapGetSource
        | BorrowedCallActualRequestV1::StoredMapSource
        | BorrowedCallActualRequestV1::NamedArrayHolderInitializer(_)
        | BorrowedCallActualRequestV1::NamedArrayAppendStatement { .. }
        | BorrowedCallActualRequestV1::ExpressionCoreSource { .. }
        | BorrowedCallActualRequestV1::DynamicInvocationSource { .. }
        | BorrowedCallActualRequestV1::ArgumentExpressionSource(..)
        | BorrowedCallActualRequestV1::ArgumentExpressionCarrierArrayAppendSource { .. }
        | BorrowedCallActualRequestV1::CompleteArgumentExpression { .. }
        | BorrowedCallActualRequestV1::ArgumentTextEntryRead { .. }
        | BorrowedCallActualRequestV1::FieldArrayTextEntryRead { .. }
        | BorrowedCallActualRequestV1::FiniteEntryRead { .. }
        | BorrowedCallActualRequestV1::DynamicEntryRead { .. }
        | BorrowedCallActualRequestV1::Observe(..)
        | BorrowedCallActualRequestV1::ReceivedObjectArguments(_, _)
        | BorrowedCallActualRequestV1::ReceivedFieldAliasObjectArguments { .. }
        | BorrowedCallActualRequestV1::ReceivedHandleArguments(_)
        | BorrowedCallActualRequestV1::BorrowedNullableLocalArguments(_)
        | BorrowedCallActualRequestV1::BorrowedNullableReturnArguments(_)
        | BorrowedCallActualRequestV1::MixedLocalArguments(_)
        | BorrowedCallActualRequestV1::SourceResultArguments { .. }
        | BorrowedCallActualRequestV1::SourceUnitDiscardArguments
        | BorrowedCallActualRequestV1::ObjectArguments(..) => {
            unreachable!("observation handled before demand")
        }
    }
    .map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
        site: site.clone(),
        issue,
    })
}
