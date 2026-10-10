//! Claim-local `recv.m(...)` lexical call probes for the co-seal walk.
use super::super::lexical_instance_call::{
    BorrowedI64ResultSourceV1, PreparedBorrowedFormalIngressV1,
    PreparedLexicalInstanceCallSourceTargetsV1,
};
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::{
    BorrowedCallActualRequestV1, BorrowedCallArgumentsV1,
};
use crate::mir::resolved_semantics::BindingKindV1;

#[path = "ordinary_new_received_handle_arguments.rs"]
mod received_handle;

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

/// The proven nullable-result object class for one received binding: the
/// binding's own initializer site names the same claim-local call target
/// that minted its `Nullable` observation, and that callee's sealed
/// `NullableObject` claim is the sole class authority. Anything else —
/// an unclassifiable initializer, a non-selected target, an `Object` or
/// absent claim — answers `None`; the caller stays on its existing path.
pub(in crate::mir::normal_callable_semantic_package) fn nullable_received_result_class(
    selected: &VerifiedSelectedCallableBatchMapV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    callable_result_classes: &super::super::result_class_claim::OrdinaryNewResultClassClaimsV1,
    candidates: &[OrdinaryNewCandidate],
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    home: BindingRefV1,
) -> Option<Box<str>> {
    let site = input
        .function()
        .expression_source()
        .initializers()
        .find(|row| row.binding() == home)?
        .initializer_site()?
        .clone();
    let (key, ..) = lexical_claim_local_target(
        selected,
        batch,
        candidates,
        input,
        &crate::mir::resolved_semantics::OwnedExprSiteV1::new(input.owner(), site),
    )?;
    match callable_result_classes.get(&key) {
        Some(
            crate::mir::normal_callable_semantic_package::OrdinaryNewResultClassV1::NullableObject(
                class,
            ),
        ) => Some(class.clone()),
        _ => None,
    }
}

/// The same callback prepares observations or demands the already-staged row.
/// This neither re-observes locals nor reads a completed caller ledger.
pub(super) fn borrowed_call_arguments_callback_v1(
    targets: &super::super::lexical_instance_call::PreparedLexicalInstanceCallSourceTargetsV1,
    contracts: &[super::super::super::model::OwnedCallableParameterContractDeclarationV1],
    candidates: &[OrdinaryNewCandidate],
    receiver: Option<(BindingRefV1, &crate::parser::ParserOrdinaryBoxSourceRowV1)>,
    source: &Result<super::super::lexical_instance_call::PreparedBorrowedFormalIngressV1, String>,
    static_source_sites: &Result<std::collections::BTreeSet<OwnedExprSiteV1>, String>,
    pending: &mut super::super::lexical_instance_call::PendingBorrowedFormalActualsV1,
    results: &BTreeMap<
        FunctionOwnerIdV1,
        Result<super::super::lexical_instance_call::BorrowedI64ResultSourceV1, String>,
    >,
    nullable_class: &mut impl FnMut(BindingRefV1) -> Option<Box<str>>,
    site: &OwnedExprSiteV1,
    request: BorrowedCallActualRequestV1<'_>,
    classes: &super::super::result_class_claim::OrdinaryNewResultClassClaimsV1,
    receiver_rows: &BTreeMap<OwnedExprSiteV1, super::super::ReceiverCallClassObservationV1>,
) -> Result<Option<BorrowedCallArgumentsV1>, OrdinaryNewCoSealIssueV1> {
    if let BorrowedCallActualRequestV1::ReceivedObjectArguments(destination, result) = request {
        return super::super::lexical_instance_call::received_producer_arguments_v1(
            source,
            pending,
            targets,
            candidates,
            classes,
            site,
            destination,
            result,
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
                    source, pending, target, loan,
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
                    source, pending, target, loan,
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
    if let BorrowedCallActualRequestV1::Observe(actuals) = request {
        let prepared = super::super::lexical_instance_call::prepare_borrowed_call_actuals_v1(
            source,
            contracts,
            site,
            actuals,
            candidates,
            receiver,
            nullable_class,
        );
        super::super::lexical_instance_call::stage_borrowed_call_actuals_v1(
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
        if !sites.contains(site) {
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
            if let Some(arguments) =
                super::super::lexical_instance_call::project_pending_instance_source_arguments_v1(
                    source, pending, contracts, site,
                )
                .map_err(|issue| {
                    OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                        site: site.clone(),
                        issue,
                    }
                })?
            {
                return Ok(Some(BorrowedCallArgumentsV1::SourceInstance(arguments)));
            }
            super::super::lexical_instance_call::project_pending_i64_result_arguments_v1(
                source, pending, results, site,
            )
            .map(|row| row.map(BorrowedCallArgumentsV1::Scalar))
        }
        BorrowedCallActualRequestV1::ScalarArguments => {
            super::super::lexical_instance_call::project_pending_borrowed_i64_arguments_v1(
                source, pending, results, site,
            )
            .map(|row| row.map(BorrowedCallArgumentsV1::Scalar))
        }
        BorrowedCallActualRequestV1::Observe(_)
        | BorrowedCallActualRequestV1::ReceivedObjectArguments(_, _)
        | BorrowedCallActualRequestV1::ReceivedHandleArguments(_)
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
mod borrowed_callback_tests {
    use super::*;

    #[test]
    fn borrowed_source_error_demands_parameter_target_but_not_strict_sibling() {
        let source = "box ParamStore { birth() { } readData(p): i64 { return 0 } readStrict(q: i64): i64 { return q } }
            box ParamManifest { birth() { } materialize(store): i64 {
                local out = store.readData(0) local strict = store.readStrict(1) return 0
            } }
            static box Main { main() {
                local store = new ParamStore() local manifest = new ParamManifest()
                local out = manifest.materialize(store) return 0
            } }";
        let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(source).unwrap();
        let ledger = &package.ordinary_new_claim_ledger;
        // Final issuance consumes the preparation; its Ready rows retain the
        // identical source targets. Borrow those rows without another resolver.
        let slots = ledger.lexical_instance_calls.borrow();
        let targets: super::super::super::lexical_instance_call::PreparedLexicalInstanceCallSourceTargetsV1 = Ok(slots.values().filter_map(|slot| match slot {
            crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::LexicalInstanceCallDispositionSlotV1::Ready(row) => Some(Ok(Some(row.source_target().clone()))),
            _ => None,
        }).collect());
        for (name, borrowed) in [("readData", true), ("readStrict", false)] {
            let target = targets
                .as_ref()
                .unwrap()
                .iter()
                .filter_map(|row| row.as_ref().ok().and_then(Option::as_ref))
                .find(|target| target.target().name() == name)
                .unwrap();
            let contract = package
                .parameter_contracts
                .iter()
                .find(|row| row.owner == target.call_site().owner())
                .unwrap();
            package
                .batch()
                .with_lowering_input(contract.batch_slot, |input| {
                    assert!(matches!(
                        input
                            .function()
                            .binding(target.receiver_binding().unwrap())
                            .unwrap()
                            .kind(),
                        BindingKindV1::Parameter { .. }
                    ));
                })
                .unwrap();
            let mut pending = BTreeMap::new();
            let result = borrowed_call_arguments_callback_v1(
                &targets,
                &package.parameter_contracts,
                &[],
                None,
                &Err("source-sentinel".into()),
                &Ok(std::collections::BTreeSet::new()),
                &mut pending,
                &BTreeMap::new(),
                &mut |_| None,
                target.call_site(),
                BorrowedCallActualRequestV1::ScalarArguments,
                &ledger.callable_result_classes,
                &ledger.receiver_call_observations,
            );
            if borrowed {
                assert!(
                    matches!(result, Err(OrdinaryNewCoSealIssueV1::BorrowedFormalIngress { issue, .. }) if issue == "source-sentinel")
                );
            } else {
                assert!(result.unwrap().is_none());
            }
            assert!(
                pending.is_empty(),
                "demand does not stage a second inventory"
            );
        }
    }
}

/// Preserve the existing source preflight order and retained per-slot errors.
pub(super) fn prepare_source_preflight_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    omitted_static_callers: &BTreeSet<FunctionOwnerIdV1>,
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
    owned: &mut BTreeMap<hakorune_mir_defs::CanonicalObjectIdV1, Option<Box<[OwnedFieldChildV1]>>>,
) -> Result<
    (
        Box<[Box<str>]>,
        BTreeMap<u32, Result<Vec<OrdinaryNewCandidate>, OrdinaryNewCoSealIssueV1>>,
        PreparedLexicalInstanceCallSourceTargetsV1,
        Result<PreparedBorrowedFormalIngressV1, String>,
        Result<std::collections::BTreeSet<OwnedExprSiteV1>, String>,
        BTreeMap<FunctionOwnerIdV1, Result<BorrowedI64ResultSourceV1, String>>,
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
            &mut |slot, input, call| {
                source_claims::stored_child_source_v1(
                    batch,
                    selected,
                    entry_home_loans.for_batch_slot(slot),
                    slot,
                    input,
                    call,
                    residences,
                )
            },
            &mut |site, receiver| {
                source_claims::stored_child_receiver_v1(
                    batch,
                    instance_constructors,
                    site,
                    receiver,
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

/// An admitted legacy stored row demands sealed I64; Object uses its original terminal walk.
pub(super) fn has_stored_terminal_v1(
    rows: &PreparedLexicalInstanceCallSourceTargetsV1,
    results: &BTreeMap<FunctionOwnerIdV1, Result<BorrowedI64ResultSourceV1, String>>,
    owner: FunctionOwnerIdV1,
) -> Result<bool, OrdinaryNewCoSealIssueV1> {
    let Some(row) = rows.as_ref().ok().and_then(|rows| {
        rows.iter()
            .filter_map(|row| row.as_ref().ok()?.as_ref())
            .find(|row| {
                row.call_site().owner() == owner
                    && row.stored_receiver().is_some()
                    && !row.has_object_source_requirement()
            })
    }) else {
        return Ok(false);
    };
    let demand = results
        .get(&owner)
        .ok_or_else(|| "[freeze:contract][stored-child/result-source-missing]".to_owned())
        .and_then(|proof| proof.as_ref().map_err(Clone::clone))
        .and_then(|proof| proof.require_source_i64_v1());
    demand.map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
        site: row.call_site().clone(),
        issue,
    })?;
    Ok(true)
}

#[cfg(test)]
#[path = "ordinary_new_borrowed_static_dispatch_tests.rs"]
mod static_dispatch_tests;

/// Same entry-receiver observation, restricted to its original non-null role.
pub(super) fn has_object_receiver_call_at_v1(
    observations: &BTreeMap<
        OwnedExprSiteV1,
        super::super::receiver_call_observation::ReceiverCallClassObservationV1,
    >,
    site: &OwnedExprSiteV1,
) -> bool {
    matches!(
        observations.get(site).map(|row| row.class()),
        Some(super::super::result_class_claim::OrdinaryNewResultClassV1::Object(_))
    )
}

#[cfg(test)]
#[path = "ordinary_new_receiver_source_role_tests.rs"]
mod receiver_source_role_tests;

#[cfg(test)]
#[path = "ordinary_new_object_arguments_port_tests.rs"]
mod object_arguments_port_tests;
