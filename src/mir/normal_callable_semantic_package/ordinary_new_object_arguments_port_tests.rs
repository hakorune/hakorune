use crate::mir::normal_callable_semantic_package::STORED_BORROWED_LOCAL_SOURCE;

use super::*;

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
        let result = borrowed_call_arguments_callback_v1(package.batch(), &package.selected,
            &Ok(Vec::new()),
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
            &ledger.receiver_call_observations, &ledger.field_write_claims,
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
        let result = borrowed_call_arguments_callback_v1(package.batch(), &package.selected,
            &Ok(Vec::new()),
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
            &ledger.receiver_call_observations, &ledger.field_write_claims,
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
        (false, true, false), (true, true, false), (true, false, false),
        (true, true, true), (true, false, true),
    ] {
    let mut text = if field_write {
        STORED_BORROWED_LOCAL_SOURCE.replace("maybe(p: Item)", "maybe(p: Item, n)")
            .replace("if p == null { return null } return p",
                "if p == null { return null } if n <= 0 { return null } p.value = n return p")
            .replace("me.child.maybe(p)", "me.child.maybe(p, 7)")
    } else { STORED_BORROWED_LOCAL_SOURCE.to_owned() };
    if field_index {
        text = text.replace("box Leaf { flag: i64", "box Leaf { slots: ArrayBox flag: i64")
            .replace("me.flag = 0", "me.flag = 0 me.slots = new ArrayBox()")
            .replace("p.value = n", "me.slots.set(p.value, n) p.value = n");
    }
    if !terminating_null_guard {
        text = text.replace("if p == null { return null }", "if p == null { local ignored = 0 }");
    }
    let prepared = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(&text);
    if !terminating_null_guard {
        if let Err(issue) = &prepared {
            assert!(format!("{issue:?}").contains("PrefixNotCovered"), "{issue:?}");
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
        target, &package.selected, &package.parameter_contracts, &package.physical_signature,
    );
    if !terminating_null_guard {
        let issue = completed.unwrap_err();
        assert!(issue.contains("PrefixNotCovered"), "{issue}");
        continue;
    }
    let completed = completed.unwrap().expect("same complete original borrowed-result inputs");
    assert_eq!(completed, observed.arguments());
    assert!(!observed.prior_homes().contains(&loan.destination()));
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
         None)
        .unwrap();
    let prepared = Ok(vec![Ok(Some(target.clone()))]);
    assert!(
        !package.batch.with_lowering_input(slot, |input| has_stored_terminal_v1(
            &prepared,
            &ledger.borrowed_i64_results,
            input,
        )).unwrap().unwrap(),
        "borrowed nullable demand must not enter the legacy I64 gate"
    );
    let mut actuals = ledger.borrowed_formal_actuals.clone();
    let mut request = |destination| {
        borrowed_call_arguments_callback_v1(package.batch(), &package.selected,
        &prepared, &package.parameter_contracts, &[], None,
        ledger.borrowed_formal_source.as_ref().unwrap(), &Ok(Default::default()),
        &mut actuals, &ledger.borrowed_i64_results, &mut |_| None,
        target.call_site(),
        crate::mir::resolved_semantics::home_new_prefix::BorrowedCallActualRequestV1::BorrowedNullableLocalArguments(destination),
        &ledger.callable_result_classes, &ledger.receiver_call_observations, &ledger.field_write_claims,
    )
    };
    let response = request(loan.destination())
        .unwrap()
        .expect("same source argument projection");
    assert!(
        matches!(response, crate::mir::resolved_semantics::home_new_prefix::BorrowedCallArgumentsV1::BorrowedNullableLocalSource { loan: retained, .. } if retained == *loan)
    );
    assert!(
        request(loan.root()).is_err(),
        "foreign destination cannot borrow the source loan"
    );
    assert!(ledger.take_receiver_object_packet_v1(call.owner(), call.site(), target.target()).unwrap().is_none());
    let row = ledger.take_lexical_instance_call(call.owner(), call.site()).unwrap().unwrap();
    let expected = crate::mir::instruction::InvokeCallResultKind::BorrowedNullableHandle { formal_ordinal: loan.callee_ordinal() };
    assert_eq!(row.result(), Some(expected));
    assert_eq!(ledger.corroborate_borrowed_nullable_call_v1(&row).unwrap(), expected);
    assert!(!ledger.uses_object_packet_protocol_v1(&row).unwrap());
    assert!(ledger.borrowed_call_actuals_v1(&row).unwrap().is_some());
    assert!(ledger.take_lexical_instance_call(call.owner(), call.site()).unwrap_err().contains("already-taken"));
    let ledger = std::rc::Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let issuance = ledger.original_instance_input_issuance.take().unwrap();
    assert!(ledger.corroborate_borrowed_nullable_call_v1(&row).unwrap_err().contains("input-issuance-missing"));
    ledger.original_instance_input_issuance = Some(issuance);
    ledger.corroborate_borrowed_nullable_call_v1(&row).unwrap();
    let row = row.with_result_for_test(Some(crate::mir::instruction::InvokeCallResultKind::NullableHandle));
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
                            assert_ne!(loan.destination(), loan.root());
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
                                .contains(&loan.destination()));
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
    let contract = package.parameter_contracts.iter().find(|row| row.parameters.len() == 4).unwrap();
    package.batch().with_lowering_input(contract.batch_slot, |input| {
        let body = input.source().root_body().unwrap();
        let statement = input.source().body_stmt(&body, 0).unwrap();
        let call_site = input.function().method_calls().find(|(_, call)| call.selector() == "sink")
            .map(|(site, _)| OwnedExprSiteV1::new(input.owner(), site.clone())).unwrap();
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
                    borrowed_call_arguments_callback_v1(package.batch(), &package.selected, &Ok(Vec::new()), &contracts,
                        &[], None, source, &Ok(Default::default()), &mut pending, &ledger.borrowed_i64_results,
                        &mut |_| None, site, request, &ledger.callable_result_classes, &ledger.receiver_call_observations, &ledger.field_write_claims)
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
                let source_loan = crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::lend_argument_expression_source_v1(ingress, &pending, &call_site, 0).unwrap().unwrap();
                assert_eq!(frame.source_loan(), &source_loan);
                // Existing Core actual tests retain the independent execution-pending checks.
            }
        }
    }).unwrap();
}

#[test]
fn initializer_text_entry_read_retains_all_incoming_and_original_alias_provenance() {
    use crate::mir::resolved_value_profile::expression_plan::{local_expression_source_plan_v1, ExpressionNodeSourceV1};
    use crate::mir::resolved_semantics::ResolvedLexicalRefV1;
    for (prefix, actual, sibling, accepted) in [
        ("", "\"field\"", "", true),
        ("local alias = key", "\"field\"", "", true),
        ("key = other", "\"field\"", "", false),
        ("", "null", "", true), ("", "true", "", false), ("", "7", "", true),
        ("", "\"field\"", "second() { return Values.caller(true, \"spare\") }", false),
        ("", "\"field\"", "relay(p) { return Values.caller(p, \"spare\") }", false),
    ] {
        let read_name = if prefix.contains("local alias") { "alias" } else { "key" };
        let text = format!(r#"static box Values {{
            caller(key, other) {{ {prefix} local needle = "x" + {read_name} return 0 }}
            seed() {{ return Values.caller({actual}, "spare") }} {sibling}
        }} static box Main {{ main() {{ return 0 }} }}"#);
        let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog_selection(
            &text, crate::mir::normal_callable_semantic_package::PackagePhysicalSelectionV1::ClosedAppExecutable,
        ).unwrap();
        let ledger = &package.ordinary_new_claim_ledger;
        let source = ledger.borrowed_formal_source.as_ref().unwrap();
        let contract = package.parameter_contracts.iter().find(|row| row.parameters.len() == 2).unwrap();
        package.batch().with_lowering_input(contract.batch_slot, |input| {
            let initializer = input.function().expression_source().initializers().last().unwrap();
            let original = input.forest().callable_source_ledger(input.owner()).unwrap();
            let plan = local_expression_source_plan_v1(&original, input.body_shape(), initializer).unwrap();
            let read = plan.nodes().iter().find_map(|node| match node.source() {
                ExpressionNodeSourceV1::Reference(ResolvedLexicalRefV1::Local(binding)) => Some((node.site(), *binding)), _ => None,
            }).unwrap();
            let site = OwnedExprSiteV1::new(input.owner(), initializer.initializer_site().unwrap().clone());
            let mut pending = ledger.borrowed_formal_actuals.clone();
            let before = pending.clone();
            let mut ask = |plan: &Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>, root| {
                borrowed_call_arguments_callback_v1(package.batch(), &package.selected, &Ok(Vec::new()),
                    &package.parameter_contracts, &[], None, source, &Ok(Default::default()), &mut pending,
                    &ledger.borrowed_i64_results, &mut |_| None, &site,
                    BorrowedCallActualRequestV1::InitializerTextEntryRead { input, plan, read: read.0, binding: read.1, root },
                    &ledger.callable_result_classes, &ledger.receiver_call_observations, &ledger.field_write_claims).unwrap()
            };
            let row = ask(&plan, contract.parameters[0].binding);
            assert_eq!(row.is_some(), accepted, "{prefix:?}/{actual}");
            if let Some(BorrowedCallArgumentsV1::InitializerTextEntryRead(loan)) = row {
                assert!(Rc::ptr_eq(loan.requirement().source_plan(), &plan));
                assert_eq!(loan.leaf_class(None).is_some(), actual != "null", "source Null still requires the real terminating branch");
                assert_eq!(loan.requirement().formal(), (0, contract.parameters[0].binding));
                assert_eq!(Some(BorrowedCallArgumentsV1::InitializerTextEntryRead(loan.clone())), ask(&plan, contract.parameters[0].binding));
                let reissued = local_expression_source_plan_v1(&original, input.body_shape(), initializer).unwrap();
                assert_ne!(Some(BorrowedCallArgumentsV1::InitializerTextEntryRead(loan)), ask(&reissued, contract.parameters[0].binding));
            }
            assert_eq!(ask(&plan, contract.parameters[1].binding).is_some(), prefix == "key = other", "only the original current entry can supply provenance");
            assert_eq!(before, pending, "source demand does not arm execution or stage actuals");
        }).unwrap();
    }
}

#[test]
fn text_initializer_frame_requires_original_text_signed_source_and_current_prefix() {
    for (annotation, actual, mutation, accepted) in [
        (": i64", "\"text\"", 0, true), (": usize", "\"text\"", 0, false),
        ("", "\"text\"", 0, true), (": i64", "true", 0, false),
        (": i64", "\"text\"", 1, false), (": i64", "\"text\"", 2, false),
        (": i64", "\"text\"", 3, false), (": i64", "\"text\"", 4, false),
    ] {
        let text = format!(r#"static box Values {{
            caller(line, count{annotation}) {{ local needle = "x" + line + count
                local alias = needle local next = alias + "!" return 0 }}
            seed() {{ return Values.caller({actual}, 7) }}
        }} static box Main {{ main() {{ return Values.seed() }} }}"#);
        let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog_selection(
            &text, crate::mir::normal_callable_semantic_package::PackagePhysicalSelectionV1::ClosedAppExecutable).unwrap();
        let ledger = &package.ordinary_new_claim_ledger;
        let source = ledger.borrowed_formal_source.as_ref().unwrap();
        let contract = package.parameter_contracts.iter().find(|row| row.parameters.len() == 2).unwrap();
        package.batch().with_lowering_input(contract.batch_slot, |input| {
            let parameters = contract.parameters.iter().map(|row| (row.ordinal, row.binding, row.kind.clone())).collect();
            let mut pending = ledger.borrowed_formal_actuals.clone();
            let mut batches = 0;
            let mut requests = 0;
            let (plans, homes) = crate::mir::resolved_semantics::home_new_prefix::observe_text_initializers_for_test(
                input, parameters, mutation != 1,
                match mutation { 2 => Some(contract.parameters[0].binding), 4 => Some(contract.parameters[1].binding), _ => None },
                &mut |fields, _| -> Result<_, OrdinaryNewCoSealIssueV1> { batches += 1; assert!(fields.is_empty()); Ok((mutation != 3).then(Vec::new)) },
                &mut |site, request| {
                    if matches!(request, BorrowedCallActualRequestV1::InitializerTextEntryRead { .. }) { requests += 1; }
                    borrowed_call_arguments_callback_v1(package.batch(), &package.selected, &Ok(Vec::new()), &package.parameter_contracts,
                        &[], None, source, &Ok(Default::default()), &mut pending, &ledger.borrowed_i64_results,
                        &mut |_| None, site, request, &ledger.callable_result_classes, &ledger.receiver_call_observations, &ledger.field_write_claims)
                }).unwrap();
            assert_eq!(plans.len(), if accepted { 2 } else { 0 }, "{annotation}/{actual}/{mutation}");
            assert_eq!(homes.len(), if accepted { 2 } else { 0 });
            assert_eq!(batches, if accepted { 2 } else { usize::from(mutation == 3) });
            if mutation == 1 { assert_eq!(requests, 0, "unknown prefix cannot qualify current state"); }
            if accepted {
                let first = plans[0].text_initializer().unwrap();
                let second = plans[1].text_initializer().unwrap();
                assert_eq!(first.entry_reads().len(), if annotation.is_empty() { 2 } else { 1 });
                assert_eq!(first.scalar_reads().len(), usize::from(!annotation.is_empty()));
                assert_eq!(first.evaluation().owned_result_nodes().len(), 2);
                assert_eq!(second.residence_reads().len(), 1);
                assert!(Rc::ptr_eq(second.residence_reads()[0].acquisition(), first));
                assert_eq!(second.residence_reads()[0].root(), first.binding());
                assert_ne!(second.residence_reads()[0].binding(), first.binding(), "alias is a read, not a second owner");
                assert_eq!(second.prior_homes(), [first.binding()]);
                crate::mir::resolved_semantics::home_new_prefix::assert_text_residence_identity_for_test(input, Rc::clone(first), second);
                // The actual production scan retains the same complete frame,
                // not just this test's independent pre-evaluation walk.
                let flow = ledger.completion_for_owner(input.owner()).unwrap().cleanup().root_flow().unwrap();
                let retained = flow.expression_plans().iter().find_map(|plan| plan.text_initializer()).unwrap();
                let selected = ledger.text_initializer_frame_v1(input.owner(), retained.source().root().statement().node()).unwrap().unwrap();
                assert!(Rc::ptr_eq(retained, &selected));
            }
        }).unwrap();
    }
}

#[test]
fn owned_text_initializer_refuses_execution_until_the_same_physical_consumer_is_complete() {
    let _ring0 = crate::runtime::ring0::ensure_global_ring0_initialized();
    for (initializer, selected) in [("\"hello\"", false), ("\"he\" + \"llo\"", true)] {
        let text = format!("static box Main {{ main() {{ local text = {initializer} return 0 }} }}");
        let parsed = crate::parser::NyashParser::parse_normal_callable_program_with_build_config(
            &text, crate::parser::ParserBuildConfig::default()).unwrap();
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) =
            crate::r#macro::transform_normal_callable_program_v1(parsed).unwrap() else { panic!("original source") };
        let result = crate::mir::MirCompiler::with_options(false).compile_normal(
            crate::mir::NormalCompileRequestV1::for_mir_mode_callable_source(source, None, Default::default()));
        if selected {
            let issue = result.unwrap_err().to_string();
            assert!(issue.contains("text-initializer/execution-pending"), "{issue}");
        } else {
            assert!(result.is_ok(), "existing non-owning literal initializer stays with its owner: {result:?}");
        }
    }
}

#[test]
fn finite_initializer_entry_uses_the_original_terminating_null_arm_and_both_add_envelopes() {
    for (guard, middle, accepted) in [
        ("if value == null { return 0 }", "", true),
        ("if null == value { return 0 }", "local alias = value", true),
        ("if value != null { } else { return 0 }", "", true),
        ("if value == null { local ignored = 0 }", "", false),
        ("if value == null { local ignored = \"\" + true return 0 }", "", false),
        ("if value != null { return 0 }", "", false),
        ("if value == null { return 0 }", "value = other", false),
    ] {
        let read = if middle.starts_with("local alias") { "alias" } else { "value" };
        let text = format!(r#"static box Values {{
            parse(value, other) {{ {guard} {middle} local out = "" + {read} return 0 }}
            textSeed() {{ return Values.parse("bytes", null) }}
            integerSeed() {{ return Values.parse(7, null) }}
            nullSeed() {{ return Values.parse(null, null) }}
        }} static box Main {{ main() {{
            local text = Values.textSeed() local integer = Values.integerSeed()
            local absent = Values.nullSeed() return 0
        }} }}"#);
        let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog_selection(
            &text, crate::mir::normal_callable_semantic_package::PackagePhysicalSelectionV1::ClosedAppExecutable).unwrap();
        let ledger = &package.ordinary_new_claim_ledger;
        let contract = package.parameter_contracts.iter().find(|row| row.parameters.len() == 2).unwrap();
        let frame = ledger.completion_for_owner(contract.owner)
            .and_then(|completion| completion.cleanup().root_flow())
            .and_then(|flow| flow.expression_plans().iter().find_map(|plan| plan.text_initializer()));
        assert_eq!(frame.is_some(), accepted, "{guard}/{middle}; role={:?}; completion={:?}",
            package.selected.role_for_batch_slot(contract.batch_slot),
            ledger.completion_for_owner(contract.owner).map(|completion| completion.cleanup().root_flow()));
        if let Some(frame) = frame {
            assert_eq!(frame.entry_reads().len(), 1);
            assert_eq!(frame.entry_refinements().len(), 1);
            assert!(frame.entry_reads()[0].leaf_class(None).is_none(), "original SUM still contains Null");
            let proof = frame.entry_refinements().values().next().unwrap();
            assert!(frame.entry_reads()[0].leaf_class(Some(proof)).is_some());
            let operation = frame.evaluation().operations().iter().find(|operation| operation.node() == 0).unwrap();
            assert_eq!(operation.envelopes().len(), 2);
            assert!(operation.single_envelope().is_none(), "physical writer must not select one alternative");
            assert!(operation.envelopes().iter().all(|envelope| envelope.normal_result() ==
                crate::mir::dynamic_operator_contract::DynamicOperatorNormalResultV1::OwnedStringValue));
            assert!(frame.owns_root());
            let selected = ledger.text_initializer_frame_v1(contract.owner, frame.source().root().statement().node()).unwrap().unwrap();
            assert!(Rc::ptr_eq(frame, &selected), "the real production selector retains this SAME complete frame");
        }
    }
}

#[test]
fn original_text_argument_product_keeps_static_residence_entry_and_core_distinct() {
    for (prefix, argument, incoming, expected, static_reads, residence_reads, entry_reads, owned) in [
        ("", "\"bytes\"", "\"seed\"", true, 0, 0, 0, false),
        ("local key = \"bytes\"", "key", "\"seed\"", true, 1, 0, 0, false),
        ("local key = \"bytes\" local alias = key", "alias", "\"seed\"", true, 1, 0, 0, false),
        ("local key = \"bytes\" key = 7", "key", "\"seed\"", false, 0, 0, 0, false),
        ("local key = \"bytes\" key = \"bytes\"", "key", "\"seed\"", false, 0, 0, 0, false),
        ("local key = \"a\" + \"b\"", "key", "\"seed\"", true, 0, 1, 0, false),
        ("local key = \"a\" + \"b\" local alias = key", "alias", "\"seed\"", true, 0, 1, 0, false),
        ("", "line", "\"seed\"", true, 0, 0, 1, false),
        ("local alias = line", "alias", "\"seed\"", true, 0, 0, 1, false),
        ("", "line", "7", false, 0, 0, 0, false),
        ("", "line", "null", false, 0, 0, 0, false),
        ("local key = \"bytes\"", "key.substring(0,1)", "\"seed\"", true, 1, 0, 0, true),
        ("local key = \"a\" + \"b\"", "key.substring(0,1)", "\"seed\"", true, 0, 1, 0, true),
        ("", "line.substring(0,1)", "\"seed\"", true, 0, 0, 1, true),
    ] {
        let text = format!("static box Values {{ sink(p) {{ return 0 }} caller(line) {{ {prefix} local n = Values.sink({argument}) return 0 }} seed() {{ return Values.caller({incoming}) }} }} static box Main {{ main() {{ return 0 }} }}");
        let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog_selection(
            &text, crate::mir::normal_callable_semantic_package::PackagePhysicalSelectionV1::ClosedAppExecutable).unwrap();
        let ledger = &package.ordinary_new_claim_ledger;
        let source = ledger.borrowed_formal_source.as_ref().unwrap();
        let declaration = package.batch().declarations().find(|row|
            package.parameter_contracts.iter().any(|contract| contract.owner == row.owner() && contract.parameters.len() == 1)
                && package.batch().with_lowering_input(row.batch_slot(), |input|
                    input.function().method_calls().any(|(_, call)| call.selector() == "sink")).unwrap()).unwrap();
        package.batch().with_lowering_input(declaration.batch_slot(), |input| {
            let contract = package.parameter_contracts.iter().find(|row| row.owner == input.owner()).unwrap();
            let parameters = contract.parameters.iter().map(|row| (row.ordinal, row.binding, row.kind.clone())).collect();
            let mut pending = ledger.borrowed_formal_actuals.clone();
            let (plans, _) = crate::mir::resolved_semantics::home_new_prefix::observe_text_initializers_for_test(
                input, parameters, true, None,
                &mut |fields, _| -> Result<_, OrdinaryNewCoSealIssueV1> { assert!(fields.is_empty()); Ok(Some(Vec::new())) },
                &mut |site, request| borrowed_call_arguments_callback_v1(package.batch(), &package.selected, &Ok(Vec::new()),
                    &package.parameter_contracts, &[], None, source, &Ok(Default::default()), &mut pending,
                    &ledger.borrowed_i64_results, &mut |_| None, site, request,
                    &ledger.callable_result_classes, &ledger.receiver_call_observations, &ledger.field_write_claims)).unwrap();
            let frames: Vec<_> = plans.iter().filter_map(|plan| plan.argument_frame()).collect();
            assert_eq!(frames.len(), usize::from(expected), "{prefix}/{argument}/{incoming}");
            if let Some(frame) = frames.first() {
                assert!(Rc::ptr_eq(frame.source(), frame.source_loan().plan()));
                assert_eq!(frame.static_reads().len(), static_reads);
                assert_eq!(frame.residence_reads().len(), residence_reads);
                assert_eq!(frame.text_reads().len(), entry_reads);
                assert_eq!(frame.evaluation().owned_result_nodes().contains(&0), owned);
                assert_eq!(frame.source_loan().resolver_source().is_some(), owned);
                if let Some(read) = frame.static_reads().first() { assert_eq!(read.utf8(), "bytes"); }
            }
        }).unwrap();
    }
}
