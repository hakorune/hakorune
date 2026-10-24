use crate::mir::normal_callable_semantic_package::STORED_BORROWED_LOCAL_SOURCE;

use super::*;
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::PendingBorrowedFormalActualsV1;
use crate::mir::resolved_semantics::home_new_prefix::{
    BorrowedCallActualCandidateV1, BorrowedCallActualValueV1, LocalCallArgumentV1,
};

fn package() -> crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1
{
    crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        "box Token {} box Maker { make(size: i64) { return new Token() } relay(size: i64) { return me.make(size) } } static box Main { main() { return 0 } }"
    ).unwrap()
}
fn loan(
    package: &crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1,
) -> crate::mir::normal_callable_semantic_package::ObjectReturnCallQualificationV1 {
    let claims = &package.ordinary_new_claim_ledger.callable_result_classes;
    let key = crate::mir::builder::CanonicalSameModuleCallableKeyV1::instance_box_method(
        "Maker", "relay", 1,
    );
    claims
        .object_return_qualification(claims.outcomes(&key).unwrap()[0].site())
        .unwrap()
}

#[test]
fn object_argument_request_rejects_foreign_facts_before_missing_target() {
    let package = package();
    let foreign = self::package();
    let own = loan(&package);
    let other = loan(&foreign);
    let ledger = &package.ordinary_new_claim_ledger;
    let mut pending = ledger.borrowed_formal_actuals.clone();
    let original = pending.clone();
    for (qualification, accepted) in [(&own, true), (&other, false)] {
        let result = borrowed_call_arguments_callback_v1(
            package.batch(),
            &package.selected,
            &Ok(Vec::new()), &[],
            &package.parameter_contracts,
            &[],
            None,
            ledger.borrowed_formal_source.as_ref().unwrap(),
            &Ok(Default::default()),
            &mut pending,
            &ledger.borrowed_i64_results,
            &mut |_| None,
            qualification.call(),
            BorrowedCallActualRequestV1::ObjectArguments(qualification, None),
            &ledger.callable_result_classes,
            &ledger.receiver_call_observations,
            &ledger.field_write_claims,
            None,
        );
        if accepted {
            assert!(result.unwrap().is_none());
        } else {
            assert!(format!("{:?}", result.unwrap_err()).contains("foreign-object-qualification"));
        }
        assert_eq!(
            pending, original,
            "a demand never stages a second authority"
        );
    }
}

#[test]
fn qualified_source_input_generic_demands_stay_passive() {
    use super::super::super::lexical_instance_call::project_pending_instance_source_arguments_v1;
    let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        "box Token {} box Maker {
            make(size) { if size <= 0 { return null } return new Token() }
            relay(size) { if size <= 0 { return null } local out = me.make(size) return out }
            qualified() { return me.make(7) }
        } static box Main { main() { return 0 } }"
    ).expect("original qualified source input stays nonexecuting");
    let ledger = &package.ordinary_new_claim_ledger;
    let key = crate::mir::builder::CanonicalSameModuleCallableKeyV1::instance_box_method(
        "Maker", "relay", 1,
    );
    let qualification = ledger
        .callable_result_classes
        .outcomes(&key)
        .unwrap()
        .iter()
        .find_map(|row| {
            ledger
                .callable_result_classes
                .object_return_qualification(row.site())
        })
        .expect("original qualified call");
    let source = ledger.borrowed_formal_source.as_ref().unwrap();
    let mut pending = ledger.borrowed_formal_actuals.clone();
    let arguments = project_pending_instance_source_arguments_v1(
        source,
        &pending,
        &package.parameter_contracts,
        qualification.call(),
    )
    .unwrap()
    .expect("source-only input projection");
    let original = pending.clone();
    for request in [
        BorrowedCallActualRequestV1::ScalarArguments,
        BorrowedCallActualRequestV1::I64ResultArguments,
    ] {
        let result = borrowed_call_arguments_callback_v1(
            package.batch(),
            &package.selected,
            &Ok(Vec::new()), &[],
            &package.parameter_contracts,
            &[],
            None,
            source,
            &Ok(Default::default()),
            &mut pending,
            &ledger.borrowed_i64_results,
            &mut |_| None,
            qualification.call(),
            request,
            &ledger.callable_result_classes,
            &ledger.receiver_call_observations,
            &ledger.field_write_claims,
            None,
        )
        .unwrap();
        assert!(
            matches!(result, Some(BorrowedCallArgumentsV1::SourceInstance(rows))
            if rows == arguments)
        );
        assert_eq!(pending, original, "passive demand retains the staged phase");
    }
}

#[test]
fn stored_nullable_local_source_retains_original_borrow_without_object_promotion() {
    for (field_write, terminating_null_guard, field_index) in [
        (false, true, false),
        (true, true, false),
        (true, false, false),
        (true, true, true),
        (true, false, true),
    ] {
        let mut text =
            if field_write {
                STORED_BORROWED_LOCAL_SOURCE.replace("maybe(p: Item)", "maybe(p: Item, n)")
            .replace("if p == null { return null } return p",
                "if p == null { return null } if n <= 0 { return null } p.value = n return p")
            .replace("me.child.maybe(p)", "me.child.maybe(p, 7)")
            } else {
                STORED_BORROWED_LOCAL_SOURCE.to_owned()
            };
        if field_index {
            text = text
                .replace(
                    "box Leaf { flag: i64",
                    "box Leaf { slots: ArrayBox flag: i64",
                )
                .replace("me.flag = 0", "me.flag = 0 me.slots = new ArrayBox()")
                .replace("p.value = n", "me.slots.set(p.value, n) p.value = n");
        }
        if !terminating_null_guard {
            text = text.replace(
                "if p == null { return null }",
                "if p == null { local ignored = 0 }",
            );
        }
        let prepared = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(&text);
        if !terminating_null_guard {
            if let Err(issue) = &prepared {
                assert!(
                    format!("{issue:?}").contains("PrefixNotCovered"),
                    "{issue:?}"
                );
                continue;
            }
        }
        let mut package = prepared.expect("original stored borrowed LOCAL source");
        let ledger = &package.ordinary_new_claim_ledger;
        let source = ledger
            .borrowed_formal_source
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap();
        let key = CanonicalSameModuleCallableKeyV1::instance_box_method("Parent", "run", 1);
        let slot = package
            .selected
            .batch_slot(&SelectedNormalCallableKeyV1::Cataloged(key))
            .unwrap();
        let call = package
            .batch
            .with_lowering_input(slot, |input| {
                let initializer = input
                    .function()
                    .expression_source()
                    .initializers()
                    .next()
                    .unwrap();
                OwnedExprSiteV1::new(
                    input.owner(),
                    initializer.initializer_site().unwrap().clone(),
                )
            })
            .unwrap();
        let target = source
            .object_source_target_at_v1(&call)
            .unwrap()
            .expect("original production source retention");
        let loan = target.borrowed_nullable_local_source().unwrap();
        assert!(ledger.is_lifecycle_local_call_site_v1(&call).unwrap());
        let observed = ledger
            .local_call_for_owner(call.owner(), call.site())
            .expect("original production Home walk records the borrowed local");
        assert_eq!(
        observed.result(),
        crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::BorrowedNullable
    );
        assert_eq!(observed.borrowed_nullable(), Some(loan));
        let completed = ledger.checked_completed_original_instance_target_arguments_v1(
            target,
            &package.selected,
            &package.parameter_contracts,
            &package.physical_signature,
        );
        if !terminating_null_guard {
            let issue = completed.unwrap_err();
            assert!(issue.contains("PrefixNotCovered"), "{issue}");
            continue;
        }
        let completed = completed
            .unwrap()
            .expect("same complete original borrowed-result inputs");
        assert_eq!(completed, observed.arguments());
        assert!(!observed.prior_homes().contains(
            &crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1::Stable(
                loan.destination()
            )
        ));
        assert!(target.stored_receiver().is_some());
        assert!(!target.has_object_source_requirement());
        assert!(target.object_return_sources().is_none());
        assert!(target.object_producer_dependencies().is_none());
        assert_eq!(loan.call(), target.call_site());
        assert_eq!(loan.key(), target.target());
        ledger
            .callable_result_classes
            .recheck_borrowed_nullable_local_source_v1(
                &package.batch,
                &package.selected,
                &ledger.field_write_claims,
                &package.parameter_contracts,
                loan,
                None,
            )
            .unwrap();
        let prepared = Ok(vec![Ok(Some(target.clone()))]);
        assert!(
            !package
                .batch
                .with_lowering_input(slot, |input| has_stored_terminal_v1(
                    &prepared,
                    &ledger.borrowed_i64_results,
                    input,
                ))
                .unwrap()
                .unwrap(),
            "borrowed nullable demand must not enter the legacy I64 gate"
        );
        let mut actuals = ledger.borrowed_formal_actuals.clone();
        let mut request = |destination| {
            borrowed_call_arguments_callback_v1(package.batch(), &package.selected,
        &prepared, &[], &package.parameter_contracts, &[], None,
        ledger.borrowed_formal_source.as_ref().unwrap(), &Ok(Default::default()),
        &mut actuals, &ledger.borrowed_i64_results, &mut |_| None,
        target.call_site(),
        crate::mir::resolved_semantics::home_new_prefix::BorrowedCallActualRequestV1::BorrowedNullableLocalArguments(destination),
        &ledger.callable_result_classes, &ledger.receiver_call_observations, &ledger.field_write_claims, None,
    )
        };
        let response = request(loan.destination())
            .unwrap()
            .expect("same source argument projection");
        assert!(
            matches!(response, crate::mir::resolved_semantics::home_new_prefix::BorrowedCallArgumentsV1::BorrowedNullableLocalSource { loan: retained, .. } if retained == *loan)
        );
        assert!(
            request(loan.root().unwrap()).is_err(),
            "foreign destination cannot borrow the source loan"
        );
        assert!(ledger
            .take_receiver_object_packet_v1(call.owner(), call.site(), target.target())
            .unwrap()
            .is_none());
        let row = ledger
            .take_lexical_instance_call(call.owner(), call.site())
            .unwrap()
            .unwrap();
        let expected = crate::mir::instruction::InvokeCallResultKind::borrowed_nullable_formal(
            loan.callee_ordinal().unwrap(),
        );
        assert_eq!(row.result(), Some(expected));
        assert_eq!(
            ledger.corroborate_borrowed_nullable_call_v1(&row).unwrap(),
            expected
        );
        assert!(!ledger.uses_object_packet_protocol_v1(&row).unwrap());
        assert!(ledger.borrowed_call_actuals_v1(&row).unwrap().is_some());
        assert!(ledger
            .take_lexical_instance_call(call.owner(), call.site())
            .unwrap_err()
            .contains("already-taken"));
        let ledger = std::rc::Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        let issuance = ledger.original_instance_input_issuance.take().unwrap();
        assert!(ledger
            .corroborate_borrowed_nullable_call_v1(&row)
            .unwrap_err()
            .contains("input-issuance-missing"));
        ledger.original_instance_input_issuance = Some(issuance);
        ledger.corroborate_borrowed_nullable_call_v1(&row).unwrap();
        let row = row.with_result_for_test(Some(
            crate::mir::instruction::InvokeCallResultKind::NullableHandle,
        ));
        assert!(ledger.corroborate_borrowed_nullable_call_v1(&row).is_err());
    }
}

#[test]
fn stored_nullable_source_branch_retains_exact_return_operand_and_root() {
    use crate::mir::resolved_semantics::home_new_prefix::{
        TerminalRelationV1, TerminalReturnedSourceV1,
    };
    for body in [
        "if same != null { return same } return null",
        "if same == null { return null } else { return same }",
    ] {
        let text = STORED_BORROWED_LOCAL_SOURCE.replace(
            "local same = me.child.maybe(p) return 0",
            &format!("local same = me.child.maybe(p) {body}"),
        );
        let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(&text)
            .expect("original borrowed nullable source branch");
        let ledger = &package.ordinary_new_claim_ledger;
        let key = CanonicalSameModuleCallableKeyV1::instance_box_method("Parent", "run", 1);
        let slot = package
            .selected
            .batch_slot(&SelectedNormalCallableKeyV1::Cataloged(key))
            .unwrap();
        package
            .batch
            .with_lowering_input(slot, |input| {
                let completion = ledger.completion_for_owner(input.owner()).unwrap();
                let mut borrowed = 0;
                for exit in completion.explicit_sites() {
                    let relation = ledger
                        .terminal_relation_for_owner_at(input.owner(), exit)
                        .expect("original child terminal retained");
                    if let TerminalRelationV1::Value(value) = relation {
                        if let TerminalReturnedSourceV1::BorrowedNullableLocal(loan) =
                            value.returned()
                        {
                            borrowed += 1;
                            let observed = ledger
                                .local_call_for_owner(input.owner(), loan.call().site())
                                .unwrap();
                            assert_eq!(observed.borrowed_nullable(), Some(loan.as_ref()));
                            assert_ne!(Some(loan.destination()), loan.root());
                            assert_eq!(
                                input.function().variable_ref(value.value_site()),
                                Some(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(
                                    loan.destination()
                                ))
                            );
                            let projection = ledger.normal_exit_projection_v1(input.owner(), exit)
                                .expect("checked borrowed source projection").expect("borrowed Normal homes");
                            let original = completion.cleanup().root_flow().unwrap()
                                .exit_row(exit).unwrap().unwrap();
                            assert_eq!(projection.homes(), original.homes());
                            assert_eq!(projection.homes().as_ptr(), original.homes().as_ptr());
                            assert_eq!(projection.fault_homes(), original.homes());
                            assert_eq!(ledger.checked_borrowed_nullable_return_source_v1(input.owner(), exit)
                                .unwrap(), Some(loan.as_ref()));
                            assert!(!completion
                                .cleanup()
                                .root_flow()
                                .unwrap()
                                .exit_row(exit)
                                .unwrap()
                                .unwrap()
                                .homes()
                                .contains(&crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1::Stable(loan.destination())));
                        }
                    }
                }
                assert_eq!(borrowed, 1);
            })
            .unwrap();
    }
}

#[test]
fn ordinary_core_argument_frame_requires_the_real_covered_prefix() {
    let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog_selection(
        r#"static box Values { sink(p) { return 0 } caller(line, spare, start: i64, end: i64) { return Values.sink(line.substring(start, end)) } seed() { return Values.caller("text", "other", 0, 1) } } static box Main { main() { return 0 } }"#,
        crate::mir::normal_callable_semantic_package::PackagePhysicalSelectionV1::ClosedAppExecutable,
    ).unwrap();
    let ledger = &package.ordinary_new_claim_ledger;
    let source = ledger.borrowed_formal_source.as_ref().unwrap();
    let ingress = source.as_ref().unwrap();
    let contract = package
        .parameter_contracts
        .iter()
        .find(|row| row.parameters.len() == 4)
        .unwrap();
    package.batch().with_lowering_input(contract.batch_slot, |input| {
        let body = input.source().root_body().unwrap();
        let statement = input.source().body_stmt(&body, 0).unwrap();
        let call_site = input.function().method_calls().find(|(_, call)| call.selector() == "sink")
            .map(|(site, _)| OwnedExprSiteV1::new(input.owner(), site.clone())).unwrap();
        let caller = CanonicalSameModuleCallableKeyV1::static_box_method("Values", "caller", contract.parameters.len() as u32);
        let (claim, _) = package.source_static_claims_for_test.claim_target(&caller, call_site.site()).unwrap();
        let project = |pending: &PendingBorrowedFormalActualsV1|
            crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::project_pending_static_source_arguments_v1(
                source, pending, &call_site, claim);
        for mutation in 0..7 {
            let mut contracts = crate::mir::normal_callable_semantic_package::model::parameter_catalog_for_drift_test(&package.parameter_contracts);
            if mutation == 6 {
                contracts.iter_mut().find(|row| row.owner == input.owner()).unwrap().parameters[0].kind =
                    crate::mir::callable_parameter_contract::CallableParameterContractKindV1::DeclaredObject("Token".into());
            }
            let parameters = contract.parameters.iter().map(|row| (row.ordinal, row.binding, row.kind.clone())).collect();
            let mut pending = BTreeMap::new();
            let mut batches = 0;
            let mut source_requests = 0;
            let result = crate::mir::resolved_semantics::home_new_prefix::observe_argument_expression_for_test(
                input, &statement, parameters, mutation != 1,
                match mutation { 2 => Some(contract.parameters[0].binding), 3 => Some(contract.parameters[2].binding), _ => None },
                &mut |requests, _| -> Result<_, OrdinaryNewCoSealIssueV1> {
                    batches += 1;
                    assert!(requests.is_empty());
                    Ok((mutation != 4).then(Vec::new))
                },
                &mut |site, request| {
                    if matches!(request, BorrowedCallActualRequestV1::ArgumentExpressionSource(..)) { source_requests += 1; }
                    let request = if let BorrowedCallActualRequestV1::ArgumentTextEntryRead { source, read, binding, root } = request {
                        BorrowedCallActualRequestV1::ArgumentTextEntryRead { source, read, binding,
                            root: if mutation == 5 { contract.parameters[1].binding } else { root } }
                    } else { request };
                    borrowed_call_arguments_callback_v1(package.batch(), &package.selected, &Ok(Vec::new()), &[], &contracts,
                        &[], None, source, &Ok(Default::default()), &mut pending, &ledger.borrowed_i64_results,
                        &mut |_| None, site, request, &ledger.callable_result_classes, &ledger.receiver_call_observations, &ledger.field_write_claims, None)
                },
            );
            if mutation == 6 {
                assert!(format!("{:?}", result.unwrap_err()).contains("borrowed-text-entry/source-identity"));
                assert_eq!(batches, 0);
                continue;
            }
            let frames = result.unwrap();
            assert_eq!(frames.len(), usize::from(mutation == 0), "complete current frame only: {mutation}");
            assert_eq!(batches, usize::from(mutation == 0 || mutation == 4), "no partial field commit: {mutation}");
            if mutation == 1 { assert_eq!(source_requests, 0, "uncovered staging cannot request a current frame"); }
            if let Some(plan) = frames.first() {
                let frame = plan.argument_frame().unwrap();
                assert!(Rc::ptr_eq(plan.source(), frame.source_loan().plan()));
                assert_eq!(frame.references().len(), 2);
                assert_eq!(frame.text_reads().len(), 1);
                assert_eq!(frame.text_reads()[0].formal(), (0, contract.parameters[0].binding));
                assert_eq!(frame.evaluation().owned_result_nodes(), [0]);
                assert_eq!(frame.evaluation().ordered_nodes().last(), Some(&0));
                assert!(frame.evaluation().ordered_nodes().iter().all(|node| !frame.transfers_node(*node)), "Core receiver/bounds remain in source postorder");
                let source_loan = crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::lend_argument_expression_source_v1(ingress, &pending, &package.parameter_contracts, &call_site, 0).unwrap().unwrap();
                assert_eq!(frame.source_loan(), &source_loan);
                let expected = vec![LocalCallArgumentV1::BorrowedActual { ordinal: 0,
                    site: frame.source().nodes()[0].site().clone() }].into_boxed_slice();
                assert_eq!(project(&pending).unwrap(), Some(expected.clone()), "qualified source projection stays passive");
                let projected = borrowed_call_arguments_callback_v1(package.batch(), &package.selected, &Ok(Vec::new()), &[], &contracts,
                    &[], None, source, &Ok(Default::default()), &mut pending, &ledger.borrowed_i64_results,
                    &mut |_| None, &call_site, BorrowedCallActualRequestV1::QualifiedStaticSourceArguments(claim),
                    &ledger.callable_result_classes, &ledger.receiver_call_observations, &ledger.field_write_claims, None).unwrap().unwrap();
                let BorrowedCallArgumentsV1::StaticSource(projected) = projected else { panic!("canonical source projection") };
                assert_eq!(projected, expected, "same current qualified source survives callback dispatch without legacy site membership");
                let reissued = Rc::new(frame.as_ref().clone());
                assert!(!Rc::ptr_eq(frame, &reissued));
                let retained = borrowed_call_arguments_callback_v1(package.batch(), &package.selected, &Ok(Vec::new()), &[], &contracts,
                    &[], None, source, &Ok(Default::default()), &mut pending, &ledger.borrowed_i64_results,
                    &mut |_| None, &call_site, BorrowedCallActualRequestV1::CompleteArgumentExpression { input, frame: &reissued },
                    &ledger.callable_result_classes, &ledger.receiver_call_observations, &ledger.field_write_claims, None).unwrap().unwrap();
                let BorrowedCallArgumentsV1::ArgumentExpressionFrame(retained) = retained else { panic!("same pending frame") };
                assert!(Rc::ptr_eq(frame, &retained), "pending actual and Home retain the SAME canonical frame");
                let actuals = [BorrowedCallActualCandidateV1 { ordinal: 0,
                    site: frame.source().nodes()[0].site().clone(),
                    value: BorrowedCallActualValueV1::MethodCall(source_loan.resolver_source().unwrap().source().clone()) }];
                borrowed_call_arguments_callback_v1(package.batch(), &package.selected, &Ok(Vec::new()), &[], &contracts,
                    &[], None, source, &Ok(Default::default()), &mut pending, &ledger.borrowed_i64_results,
                    &mut |_| None, &call_site, BorrowedCallActualRequestV1::Observe(input, &actuals),
                    &ledger.callable_result_classes, &ledger.receiver_call_observations, &ledger.field_write_claims, None).unwrap();
                assert!(project(&pending).unwrap_err().contains("core-expression-execution-pending"),
                    "Observe invalidates old qualification before a new Home walk");
                let reset = borrowed_call_arguments_callback_v1(package.batch(), &package.selected, &Ok(Vec::new()), &[], &contracts,
                    &[], None, source, &Ok(Default::default()), &mut pending, &ledger.borrowed_i64_results,
                    &mut |_| None, &call_site, BorrowedCallActualRequestV1::QualifiedStaticSourceArguments(claim),
                    &ledger.callable_result_classes, &ledger.receiver_call_observations, &ledger.field_write_claims, None).unwrap();
                assert!(reset.is_none(), "unqualified source retains existing callback selection boundary");
                let parameters = contract.parameters.iter().map(|row| (row.ordinal, row.binding, row.kind.clone())).collect();
                let repeated = crate::mir::resolved_semantics::home_new_prefix::observe_argument_expression_for_test(
                    input, &statement, parameters, true, None,
                    &mut |requests, _| -> Result<_, OrdinaryNewCoSealIssueV1> { assert!(requests.is_empty()); Ok(Some(Vec::new())) },
                    &mut |site, request| borrowed_call_arguments_callback_v1(package.batch(), &package.selected, &Ok(Vec::new()), &[], &contracts,
                        &[], None, source, &Ok(Default::default()), &mut pending, &ledger.borrowed_i64_results,
                        &mut |_| None, site, request, &ledger.callable_result_classes, &ledger.receiver_call_observations, &ledger.field_write_claims, None)).unwrap();
                assert!(!Rc::ptr_eq(frame, repeated[0].argument_frame().unwrap()), "explicit Observe requires a fresh current frame");
                assert!(Rc::ptr_eq(frame.source(), repeated[0].argument_frame().unwrap().source()));
                assert_eq!(project(&pending).unwrap(), Some(expected), "new current frame closes passive projection again");
                // Existing Core actual tests retain the independent execution-pending checks.
            }
        }
    }).unwrap();
}

// Initializer source/read/entry contracts share this existing port fixture.
#[path = "ordinary_new_initializer_expression_tests.rs"]
mod initializer_expression_tests;
