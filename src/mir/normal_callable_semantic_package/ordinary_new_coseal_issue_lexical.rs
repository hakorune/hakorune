//! Claim-local `recv.m(...)` lexical call probes for the co-seal walk.
use super::super::lexical_instance_call::{
    PreparedBorrowedFormalIngressV1, PreparedLexicalInstanceCallSourceTargetsV1,
};
use super::*;
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::BorrowedResultCohortV1;
use crate::mir::resolved_semantics::home_new_prefix::{
    BorrowedCallActualRequestV1, BorrowedCallArgumentsV1,
};
use crate::mir::resolved_semantics::BindingKindV1;

#[path = "ordinary_new_received_handle_arguments.rs"]
mod received_handle;
#[path = "ordinary_new_coseal_issue_lexical_received_object.rs"]
mod received_object;

/// The shared claim-local receiver proof for a `recv.m(...)` local call:
/// the method-call inventory row at `site` carries a `Lexical(Local)`
/// receiver binding owned by this function, never rebound, initialized
/// exactly once by an ordinary-new claim candidate. On success this
/// returns the uniquely selected `InstanceBoxMethod` target, its batch
/// slot, and the callee owner — the members every lexical result lane
/// agrees on before it proves its own result class.
fn lexical_claim_local_target(
    selected: &VerifiedSelectedCallableBatchMapV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    candidates: &[OrdinaryNewCandidate],
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    site: &OwnedExprSiteV1,
) -> Option<(
    hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
    u32,
    crate::mir::resolved_semantics::FunctionOwnerIdV1,
    u32,
)> {
    use crate::mir::resolved_semantics::{
        ResolvedAssignmentTargetV1, ResolvedLexicalRefV1, ResolvedMethodCallReceiverSourceV1,
    };
    if site.owner() != input.owner() {
        return None;
    }
    let Some((observed_site, call)) = input
        .function()
        .method_calls()
        .find(|(call_site, _)| *call_site == site.site())
    else {
        return None;
    };
    if observed_site != site.site() {
        return None;
    }
    let ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Local(binding)) =
        call.receiver()
    else {
        return None;
    };
    if binding.owner() != input.owner()
        || !matches!(
            input.function().binding(binding).map(|row| row.kind()),
            Some(BindingKindV1::Local { .. })
        )
        || input.function().assignment_targets().any(|(_, target)| {
            matches!(
                target,
                ResolvedAssignmentTargetV1::BindingRebind(rebound) if *rebound == binding
            )
        })
    {
        return None;
    }
    let mut initializers = input
        .function()
        .expression_source()
        .initializers()
        .filter(|initializer| initializer.binding() == binding);
    if initializers.next().is_none() || initializers.next().is_some() {
        return None;
    }
    let mut claim_rows = candidates
        .iter()
        .filter(|candidate| candidate.destination == binding);
    let Some(candidate) = claim_rows.next() else {
        return None;
    };
    if claim_rows.next().is_some() {
        return None;
    }
    let Some((key, target_batch_slot)) =
        super::super::lexical_instance_call::unique_instance_target(
            selected,
            candidate.class.as_ref(),
            call.selector(),
            call.arity(),
        )
    else {
        return None;
    };
    let callee_owner = batch
        .declarations()
        .find(|declaration| declaration.batch_slot() == target_batch_slot)
        .map(|declaration| declaration.owner())?;
    Some((key, target_batch_slot, callee_owner, call.arity()))
}

/// Every formal of the resolved callee must be an exact `i64` scalar in
/// declared order — the sealed contract rows carry the callee's declared
/// parameter bindings, so both sides agree on the same ordinal map. The
/// unique contract row itself comes back so callers can reuse its proven
/// binding set without re-filtering.
fn all_formals_exact_i64<'a>(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    parameter_contracts: &'a [crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1],
    callee_owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    arity: u32,
) -> Option<&'a crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1>
{
    let mut contracts = parameter_contracts
        .iter()
        .filter(|row| row.owner == callee_owner);
    let contract = contracts.next()?;
    if contracts.next().is_some() || contract.parameters.len() != arity as usize {
        return None;
    }
    batch
        .with_lowering_input(contract.batch_slot, |callee_input| {
            (callee_input.owner() == contract.owner
                && contract
                    .parameters
                    .iter()
                    .enumerate()
                    .all(|(index, parameter)| {
                        parameter.ordinal as usize == index
                            && parameter.kind
                                == crate::mir::callable_parameter_contract::CallableParameterContractKindV1::ExactTrivial(
                                    crate::mir::exact_trivial_parameter_abi::ExactTrivialParameterAbiV1::I64,
                                )
                            && callee_input.function().declaration_binding(
                                &SourceBindingSiteV1::Parameter {
                                    index: parameter.ordinal,
                                },
                            ) == Some(parameter.binding)
                    }))
            .then_some(contract)
        })
        .unwrap_or(None)
}

/// A `recv.m(...)` local-call site whose receiver is a claim-local `new`
/// binding and whose uniquely selected callee returns `new` of one class.
///
/// This is the scan-side mirror of `issue_lexical_instance_call_dispositions`
/// bounded to claim-local receivers: `candidates` stands in for this
/// declaration's not-yet-committed claims, the callee's parameter contract
/// proves every formal is an exact i64, and the callee's body shape proves
/// the uniform construction result. Parameter receivers, nested
/// handle-result receivers, and rebound bindings prove nothing here — the
/// site simply stays on its existing path.
pub(super) fn lexical_handle_result_call(
    selected: &VerifiedSelectedCallableBatchMapV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    parameter_contracts: &[crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1],
    candidates: &[OrdinaryNewCandidate],
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    site: &OwnedExprSiteV1,
) -> bool {
    let Some((_key, _slot, callee_owner, arity)) =
        lexical_claim_local_target(selected, batch, candidates, input, site)
    else {
        return false;
    };
    all_formals_exact_i64(batch, parameter_contracts, callee_owner, arity).is_some()
        && crate::mir::normal_callable_semantic_package::direct_call_loan::lifecycle::construction_result_callee(
            batch,
            callee_owner,
        )
}

/// A `recv.m(...)` local-call site whose receiver is a claim-local `new`
/// binding and whose uniquely selected callee's every verified value-return
/// is a literal or an exact-`i64` formal — the sole scan-side proof that
/// the callee's co-sealed disposition mints `InvokeCallResultKind::I64`.
///
/// The scan cannot re-read terminal relations minted by the same cohort walk.
/// Only Integer literals and the callee's exact-I64 formals prove the source
/// representation here. A return annotation alone does not verify the literal
/// type: Float, Bool, String and Null must not enter this lane. Other result
/// shapes keep their existing owner. The emitter separately corroborates the
/// minted row's `result == Some(I64)`; disagreement freezes without fallback.
pub(super) fn lexical_i64_result_call(
    selected: &VerifiedSelectedCallableBatchMapV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    parameter_contracts: &[crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1],
    callable_result_classes: &super::super::result_class_claim::OrdinaryNewResultClassClaimsV1,
    candidates: &[OrdinaryNewCandidate],
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    site: &OwnedExprSiteV1,
) -> bool {
    let Some((key, target_batch_slot, callee_owner, arity)) =
        lexical_claim_local_target(selected, batch, candidates, input, site)
    else {
        return false;
    };
    let Some(contract) = all_formals_exact_i64(batch, parameter_contracts, callee_owner, arity)
    else {
        return false;
    };
    if callable_result_classes.contains_key(&key) {
        return false;
    }
    batch
        .with_lowering_input(target_batch_slot, |callee_input| {
            let Some(shape) = callee_input.body_shape() else {
                return false;
            };
            let Some(sites) = super::super::verified_value_return_sites(callee_input, shape) else {
                return false;
            };
            !sites.is_empty()
                && sites.iter().all(|site| {
                    let function = callee_input.function();
                    matches!(
                        function.expression_source().literal(site),
                        Some(crate::mir::resolved_semantics::ResolvedLiteralSourceV1::Integer(_))
                    ) || matches!(
                        function.variable_ref(site),
                        Some(
                            crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(
                                binding
                            )
                        ) if contract
                            .parameters
                            .iter()
                            .any(|parameter| parameter.binding == binding)
                    )
                })
        })
        .unwrap_or(false)
}

/// A `recv.m(...)` local-call site whose receiver is a claim-local `new`
/// binding and whose uniquely selected callee carries the sealed
/// `NullableObject` claim — every verified value-return is `return null`
/// or a `return new ..` construction. This is the sole scan-side proof
/// that the callee's co-sealed disposition mints
/// `InvokeCallResultKind::NullableHandle`: the claim names the result
/// class and the same sealed return sites re-prove its shape, so the
/// lane never infers nullability from MIR types or a backend role. The
/// received binding joins the owned-Home ledger for a checked release.
pub(super) fn lexical_nullable_result_call(
    selected: &VerifiedSelectedCallableBatchMapV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    callable_result_classes: &super::super::result_class_claim::OrdinaryNewResultClassClaimsV1,
    candidates: &[OrdinaryNewCandidate],
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    site: &OwnedExprSiteV1,
) -> bool {
    let Some((key, target_batch_slot, _callee_owner, _arity)) =
        lexical_claim_local_target(selected, batch, candidates, input, site)
    else {
        return false;
    };
    if !matches!(
        callable_result_classes.get(&key),
        Some(
            crate::mir::normal_callable_semantic_package::OrdinaryNewResultClassV1::NullableObject(
                _
            ),
        )
    ) {
        return false;
    }
    batch
        .with_lowering_input(target_batch_slot, |callee_input| {
            let Some(shape) = callee_input.body_shape() else {
                return false;
            };
            let Some(sites) = super::super::verified_value_return_sites(callee_input, shape) else {
                return false;
            };
            !sites.is_empty()
                && sites.iter().all(|site| {
                    let function = callee_input.function();
                    matches!(
                        function.expression_source().literal(site),
                        Some(crate::mir::resolved_semantics::ResolvedLiteralSourceV1::Null)
                    ) || function.expression_source().construction(site).is_some()
                })
        })
        .unwrap_or(false)
}

#[path = "ordinary_new_nullable_received_class.rs"]
mod nullable_received_class;
pub(in crate::mir::normal_callable_semantic_package) use nullable_received_class::nullable_received_result_class;

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
        if original_owned || lexical_result_source {
            if crate::config::env::builder_debug_enabled() {
                eprintln!("[dynamic-source] {site:?} not dynamic: original_owned={original_owned} lexical_result_source={lexical_result_source} dynamic_candidates={}",
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
            destination,
        )
        .map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
            site: site.clone(),
            issue,
        });
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
    if let BorrowedCallActualRequestV1::Observe(input, actuals) = request {
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
        let mut dynamic_targets = dynamic_source_calls.iter().filter(|target| {
            !ordinary
                && !lexical_result_source
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
        BorrowedCallActualRequestV1::StoredMapGetSource
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

#[cfg(test)]
#[path = "ordinary_new_borrowed_callback_tests.rs"]
mod borrowed_callback_tests;

/// Preserve the existing source preflight order and retained per-slot errors.
pub(super) fn prepare_source_preflight_v1(
    brands: Option<&crate::analysis::brand_program_declaration_catalog::VerifiedBrandProgramDeclarationCatalogV1>,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    omitted_static_callers: &BTreeMap<
        FunctionOwnerIdV1,
        crate::mir::builder::CanonicalSameModuleCallableKeyV1,
    >,
    instance_constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    parameter_contracts: &[super::super::super::model::OwnedCallableParameterContractDeclarationV1],
    static_call_claims: &super::super::super::qualified_static_call_claim::QualifiedStaticCallClaimIndexV1,
    app_main_batch_slot: Option<u32>,
    app_main: Option<&super::super::lexical_instance_call::BorrowedAppMainSourceLoanV1<'_>>,
    dynamic_slot: Option<u32>,
    entry_home_loans: &crate::mir::resolved_semantics::VerifiedInstanceEntryHomeCatalogV1,
    field_write_claims: &field_write_claim::OrdinaryNewFieldWriteClaimsV1,
    callable_result_classes: &result_class_claim::OrdinaryNewResultClassClaimsV1,
    residences: &field_write_claim::OwnedFieldResidencesV1,
    array_i64_fields: &BTreeMap<Box<str>, BTreeSet<hakorune_mir_defs::CanonicalFieldRefV1>>,
    array_field_write_candidates: &[array_i64_fields::ArrayFieldWriteCandidateV1],
    array_field_get_candidates: &[array_i64_fields::ArrayFieldGetCandidateV1],
    owned: &mut BTreeMap<hakorune_mir_defs::CanonicalObjectIdV1, Option<Box<[OwnedFieldChildV1]>>>,
) -> Result<
    (
        Box<[Box<str>]>,
        BTreeMap<u32, Result<Vec<OrdinaryNewCandidate>, OrdinaryNewCoSealIssueV1>>,
        PreparedLexicalInstanceCallSourceTargetsV1,
        Result<PreparedBorrowedFormalIngressV1, String>,
        Result<std::collections::BTreeSet<OwnedExprSiteV1>, String>,
        BorrowedResultCohortV1,
    ),
    OrdinaryNewCoSealIssueV1,
> {
    let names: Box<[Box<str>]> = batch
        .ordinary_box_coverage()
        .rows()
        .iter()
        .map(|row| row.name().to_owned().into_boxed_str())
        .collect();
    let local_candidates = source_claims::prepare_local_candidates_by_slot_v1(
        batch,
        selected,
        omitted_static_callers,
        instance_constructors,
        app_main_batch_slot,
        dynamic_slot,
    );
    let new_classes = local_candidates
        .values()
        .filter_map(|rows| rows.as_ref().ok())
        .flatten()
        .map(|candidate| (candidate.site.clone(), candidate.class.clone()))
        .collect();
    let (lexical_source_targets, borrowed_formal_source, static_source_sites, borrowed_i64_results) =
        super::super::lexical_instance_call::prepare_borrowed_profile_v1(
            brands,
            batch,
            selected,
            omitted_static_callers,
            instance_constructors,
            parameter_contracts,
            static_call_claims,
            app_main_batch_slot,
            app_main,
            dynamic_slot,
            entry_home_loans,
            &local_candidates,
            &new_classes,
            &names,
            field_write_claims,
            callable_result_classes,
            array_i64_fields,
            array_field_write_candidates,
            array_field_get_candidates,
            &mut |caller, input, call| {
                let Some(slot) = caller.declaration_slot() else {
                    return source_claims::stored_constructor_child_source_v1(caller, input, call);
                };
                source_claims::stored_child_source_v1(
                    batch,
                    instance_constructors,
                    selected,
                    entry_home_loans.for_batch_slot(slot),
                    slot,
                    input,
                    call,
                    residences,
                    callable_result_classes,
                    &local_candidates,
                )
            },
            &mut |site, receiver| {
                source_claims::stored_child_receiver_v1(
                    batch,
                    instance_constructors,
                    callable_result_classes,
                    site,
                    receiver,
                    field_write_claims,
                    residences,
                    owned,
                )
            },
        )?;
    Ok((
        names,
        local_candidates,
        lexical_source_targets,
        borrowed_formal_source,
        static_source_sites,
        borrowed_i64_results,
    ))
}

/// Stored scalar source rows demand sealed I64/Bool; execution stays separate.
pub(super) fn has_stored_terminal_v1(
    rows: &PreparedLexicalInstanceCallSourceTargetsV1,
    results: &BorrowedResultCohortV1,
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
) -> Result<bool, OrdinaryNewCoSealIssueV1> {
    let owner = input.owner();
    let Some(rows) = rows.as_ref().ok() else {
        return Ok(false);
    };
    let candidates = || {
        rows.iter()
            .filter_map(|row| row.as_ref().ok()?.as_ref())
            .filter(|row| {
                row.call_site().owner() == owner
                    && row.stored_receiver().is_some()
                    && row.uses_existing_borrowed_result_v1()
            })
    };
    let Some(first) = candidates().next() else {
        return Ok(false);
    };
    let shape = input
        .body_shape()
        .filter(|shape| shape.owner() == owner)
        .ok_or_else(|| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
            site: first.call_site().clone(),
            issue: "[freeze:contract][stored-child/original-return-shape]".into(),
        })?;
    // Source membership selects this obligation, never result-proof availability
    // or Completion success. Print/local operands keep their own consumers.
    let Some(row) = candidates().find(|row| {
        shape.statements().iter().any(|statement| {
            matches!(statement, crate::mir::resolved_semantics::BodyStatementShapeV1::Return {
            value: Some(value), .. } if value == row.call_site().site())
        })
    }) else {
        return Ok(false);
    };
    let demand = results
        .get(&owner)
        .ok_or_else(|| "[freeze:contract][stored-child/result-source-missing]".to_owned())
        .and_then(|proof| proof.as_ref().map_err(Clone::clone))
        .and_then(|proof| proof.require_source_scalar_v1());
    demand.map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
        site: row.call_site().clone(),
        issue,
    })?;
    Ok(true)
}

#[cfg(test)]
#[path = "ordinary_new_borrowed_static_dispatch_tests.rs"]
mod static_dispatch_tests;

pub(super) use super::super::receiver_call_observation::has_object_receiver_call_at_v1;

#[cfg(test)]
#[path = "ordinary_new_receiver_source_role_tests.rs"]
mod receiver_source_role_tests;

#[cfg(test)]
#[path = "ordinary_new_object_arguments_port_tests.rs"]
mod object_arguments_port_tests;
