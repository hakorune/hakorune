//! Ordinary Core result, operand and pending execution boundaries.
use super::*;

#[test]
fn ordinary_core_actual_keeps_catalog_result_separate_from_operand_and_execution_proofs() {
    for (bounds, closes) in [("0,1", true), ("true,1", false), ("0,null", false), ("0,\"end\"", false)] {
        let text = format!("static box Values {{ sink(p) {{ return 0 }} caller() {{ local receiver = \"text\" local n = Values.sink(receiver.substring({bounds})) return 0 }} }} static box Main {{ main() {{ return 0 }} }}");
        let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog_selection(
            &text, crate::mir::normal_callable_semantic_package::PackagePhysicalSelectionV1::ClosedAppExecutable,
        ).unwrap();
        let ingress = package.ordinary_new_claim_ledger.borrowed_formal_source.as_ref().unwrap().as_ref().unwrap();
        let original = ingress.source_incoming.source_rows_v1().filter_map(|row| match &row.source {
            crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(source) => Some(source), _ => None,
        }).find(|row| row.target().name() == "sink").unwrap();
        assert!(!ingress.source_incoming.static_observations().contains_key(original.call_site()), "omitted caller stays outside legacy transport");
        let p = original.parameters()[0].binding;
        use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::borrowed_formal_source::SourceValueAlternativeV1 as Value;
        assert_eq!(ingress.source_value_sum_v1(p).unwrap().alternatives(), &std::collections::BTreeSet::from([Value::Text]),
            "existing catalog result remains String on Normal despite unproved bounds: {bounds}");
        let owner = original.call_site().owner();
        let slot = package.batch().declarations().find(|row| row.owner() == owner).unwrap().batch_slot();
        package.batch().with_lowering_input(slot, |input| {
            let method = input.function().method_call_source_v1(&original.argument_sites()[0]).unwrap();
            let site = OwnedExprSiteV1::new(owner, method.site().clone());
            let receipt = ingress.ordinary_core_source_v1(&site);
            assert_eq!(receipt.is_some(), closes, "operand closure: {bounds}");
            let actual = BorrowedCallActualCandidateV1 { ordinal: 0, site: method.site().clone(),
                value: BorrowedCallActualValueV1::MethodCall(std::sync::Arc::clone(method)) };
            let rows = prepare_static_source_actuals_v1(ingress, &package.parameter_contracts, original.call_site(), &[actual]);
            if !closes { assert!(rows.unwrap().is_none(), "no original operand receipt can stage this input"); return; }
            let mut rows = rows.unwrap().unwrap();
            bind_argument_sources_v1(input, ingress, None, original.call_site(), None, Some(&mut rows)).unwrap();
            let BorrowedCallActualEvidencePhaseV1::SourceStatic(identity) = &rows.phase else { panic!("source phase only") };
            assert!(std::sync::Arc::ptr_eq(receipt.unwrap().source().source(), method));
            assert!(Rc::ptr_eq(receipt.unwrap().operands(), ingress.closed_core_source_v1(&site).unwrap()));
            assert!(Rc::ptr_eq(&identity.core_arguments[&0], receipt.unwrap()));
            assert_eq!(identity.integer_evidence.as_ref(), [false]);
            let plan = &identity.argument_sources[&0];
            assert!(matches!(plan.root(), crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument { call, ordinal: 0, statement }
                if std::sync::Arc::ptr_eq(call, original.method_source())
                    && input.function().source_site_inventory().containing_statement(original.call_site().site()) == Some(statement)));
            let mut repeated = prepare_static_source_actuals_v1(ingress, &package.parameter_contracts, original.call_site(), &identity.candidates).unwrap().unwrap();
            let previous = Ok(rows.clone());
            bind_argument_sources_v1(input, ingress, None, original.call_site(), Some(&previous), Some(&mut repeated)).unwrap();
            assert_eq!(repeated, rows, "re-observation reuses the original plan Rc");
            let failed = Err("original-source-refusal".to_owned());
            assert!(bind_argument_sources_v1(input, ingress, None, original.call_site(), Some(&failed), Some(&mut repeated))
                .unwrap_err().contains("repeated-walk-drift"), "a later Ok cannot replay an earlier refusal");
            let reissued = crate::mir::resolved_value_profile::expression_plan::call_argument_expression_source_plan_v1(input, original.method_source(), 0).unwrap();
            assert!(!Rc::ptr_eq(plan, &reissued));
            let mut changed = rows.clone();
            let BorrowedCallActualEvidencePhaseV1::SourceStatic(changed_identity) = &mut changed.phase else { unreachable!() };
            changed_identity.argument_sources.insert(0, reissued.into());
            let mut staged = BTreeMap::from([(original.call_site().clone(), Ok(rows.clone()))]);
            super::super::super::stage_borrowed_call_actuals_v1(&mut staged, original.call_site(), Ok(Some(changed)));
            assert!(staged[original.call_site()].as_ref().unwrap_err().contains("repeated-walk-drift"));
            assert!(rows.require_executable_v1().is_err(), "unused formal cannot skip argument evaluation");
            let pending = BTreeMap::from([(original.call_site().clone(), Ok(rows.clone()))]);
            assert!(project_pending_static_source_arguments_for_route_v1(
                ingress, &pending, original.call_site(), original.required_i64_arguments(), false,
            ).unwrap_err().contains("core-expression-execution-pending"));
            let loan = argument_source::lend_argument_expression_source_v1(ingress, &pending, &package.parameter_contracts, original.call_site(), 0).unwrap().unwrap();
            assert!(Rc::ptr_eq(loan.plan(), plan), "loan must retain the observed plan, not rebuild it");
            assert!(std::ptr::eq(loan.resolver_source().unwrap(), receipt.unwrap().source()), "loan must retain the ordinary Core receipt");
            assert_eq!(loan, argument_source::lend_argument_expression_source_v1(ingress, &pending, &package.parameter_contracts, original.call_site(), 0).unwrap().unwrap());
            assert!(argument_source::lend_argument_expression_source_v1(ingress, &pending, &package.parameter_contracts, original.call_site(), 1).unwrap_err().contains("core-source-identity"));
            let absent = BTreeMap::new();
            assert!(argument_source::lend_argument_expression_source_v1(ingress, &absent, &package.parameter_contracts, original.call_site(), 0).unwrap_err().contains("source-unobserved"));
            let refused = BTreeMap::from([(original.call_site().clone(), Err("retained-original-refusal".to_owned()))]);
            assert_eq!(argument_source::lend_argument_expression_source_v1(ingress, &refused, &package.parameter_contracts, original.call_site(), 0).unwrap_err(), "retained-original-refusal");

            for mutation in 0..7 {
                let mut drift = rows.clone();
                let BorrowedCallActualEvidencePhaseV1::SourceStatic(identity) = &mut drift.phase else { unreachable!() };
                match mutation {
                    0 => { identity.core_arguments.clear(); }
                    1 => { identity.core_arguments.insert(1, Rc::clone(receipt.unwrap())); }
                    2 => { identity.argument_sources.clear(); }
                    3 => { identity.argument_sources.insert(1, Rc::clone(plan).into()); }
                    4 => { identity.core_arguments.insert(0, crate::mir::normal_callable_semantic_package::core_method_source::issue_ordinary_core_source_v1(input, receipt.unwrap().operands()).unwrap()); }
                    5 => { identity.candidates[0].ordinal = 1; }
                    6 => { identity.integer_evidence[0] = true; }
                    _ => unreachable!(),
                }
                let pending = BTreeMap::from([(original.call_site().clone(), Ok(drift))]);
                assert!(argument_source::lend_argument_expression_source_v1(ingress, &pending, &package.parameter_contracts, original.call_site(), 0).is_err(), "no loan from corrupted full tuple: {mutation}");
                // A missing map cannot recover via legacy transport; extras
                // cannot survive exact source/ordinal correspondence either.
                assert!(project_pending_static_source_arguments_for_route_v1(
                    ingress, &pending, original.call_site(), original.required_i64_arguments(), false,
                ).unwrap_err().contains(if mutation == 5 { "source-projection-identity" } else { "core-source-identity" }));
            }
            let foreign = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog_selection(
                &text, crate::mir::normal_callable_semantic_package::PackagePhysicalSelectionV1::ClosedAppExecutable,
            ).unwrap();
            foreign.batch().with_lowering_input(slot, |substituted| {
                assert!(crate::mir::resolved_value_profile::expression_plan::call_argument_expression_source_plan_v1(substituted, original.method_source(), 0).is_err(), "foreign input cannot mint this original argument plan");
                assert!(crate::mir::normal_callable_semantic_package::core_method_source::issue_ordinary_core_source_v1(
                    substituted, receipt.unwrap().operands(),
                ).unwrap_err().contains("source-identity"), "equal syntax cannot borrow another source loan");
            }).unwrap();
        }).unwrap();
    }
    // Count borrows the same fixed provider through the original local alias;
    // neither a foreign container nor a replacement writer supplies that law.
    for (provider, extra, before, call, closes) in [
        ("ArrayBox", "", "", "length()", true), ("MapBox", "", "", "length()", false),
        ("ArrayBox", "", "local i = 0 loop(i < 1) { ids.push(i) i = i + 1 }", "length()", true),
        ("ArrayBox", "reset() { me.items = new ArrayBox() }", "", "length()", false),
        ("ArrayBox", "", "local i = 0", "get(i)", true),
        ("ArrayBox", "", "local i = true", "get(i)", false),
        ("ArrayBox", "reset() { me.items = new ArrayBox() }", "local i = 0", "get(i)", false),
    ] {
        let text = format!("box Holder {{ init {{ items }} birth() {{ me.items = new {provider}() }}
            count() {{ local ids = me.items {before} return ids.{call} }} {extra} }} static box Main {{ main() {{ return 0 }} }}");
        let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog_selection(
            &text, crate::mir::normal_callable_semantic_package::PackagePhysicalSelectionV1::ClosedAppExecutable,
        ).unwrap();
        let ingress = package.ordinary_new_claim_ledger.borrowed_formal_source.as_ref().unwrap().as_ref().unwrap();
        for declaration in package.batch().declarations() {
            package.batch().with_lowering_input(declaration.batch_slot(), |input| {
                let Some((site, _)) = input.function().method_calls().find(|(_, call)| matches!(call.selector(), "length" | "get")) else { return; };
                let site = OwnedExprSiteV1::new(input.owner(), site.clone());
                let source = ingress.ordinary_core_source_v1(&site);
                assert_eq!(source.is_some(), closes, "fixed Count source: {text}");
                if let Some(source) = source {
                    if call.starts_with("get") {
                        assert_eq!(source.source().target().schema(), crate::mir::resolved_semantics::CoreMethodHomeSchemaV1::ArrayDynamicRead);
                        assert_eq!(source.source().target().result(), crate::mir::resolved_semantics::CoreMethodHomeResultRelationV1::DynamicToCaller);
                        assert_eq!(source.source().target().expression_result_class_v1(), None);
                        assert!(source.matches_original_argument_v1(&site, source.source().source(), &CallableParameterContractKindV1::OpaqueHandle));
                        assert!(!source.matches_original_argument_v1(&site, source.source().source(), &CallableParameterContractKindV1::DeclaredObject("Holder".into())));
                    } else { assert!(source.source().target().checked_array_count_law_v1()); }
                    assert!(source.operands().receiver_core().is_none());
                    let count = source.operands().field_array_access_v1().unwrap();
                    assert!(count.matches_original_call_v1(input));
                    assert_eq!(count.call(), &site);
                    assert_eq!(count.receiver().site(), source.source().source().receiver_site());
                    assert!(crate::mir::normal_callable_semantic_package::core_method_source::issue_ordinary_core_source_v1(
                        input, source.operands()).is_ok());
                }
            }).unwrap();
        }
    }
    // Map presence closes only with BOTH the original stable field writer and
    // Text key; it never inherits the String receiver's class by selector.
    for (provider, extra_writer, key, closes) in [
        ("MapBox", "", "\"a\"", true),
        ("ArrayBox", "", "\"a\"", false),
        ("MapBox", "reset() { me.entries = new MapBox() }", "\"a\"", false),
        ("MapBox", "", "7", false),
    ] {
        let text = format!("box Holder {{ init {{ entries }} birth() {{ me.entries = new {provider}() }} \
            probe() {{ if me.entries.has({key}) {{ return 1 }} return 0 }} count() {{ return me.entries.keys().length() }} {extra_writer} }} \
            static box Main {{ main() {{ return 0 }} }}");
        let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog_selection(
            &text, crate::mir::normal_callable_semantic_package::PackagePhysicalSelectionV1::ClosedAppExecutable,
        ).unwrap();
        let ingress = package.ordinary_new_claim_ledger.borrowed_formal_source.as_ref().unwrap().as_ref().unwrap();
        for declaration in package.batch().declarations() {
            package.batch().with_lowering_input(declaration.batch_slot(), |input| {
                for (site, method) in input.function().method_calls().filter(|(_, method)| method.selector() == "has") {
                    let site = OwnedExprSiteV1::new(input.owner(), site.clone());
                    let closed = ingress.closed_core_source_v1(&site);
                    assert_eq!(closed.is_some(), closes, "receiver and key are joint source premises: {text}");
                    let receipt = ingress.ordinary_core_source_v1(&site);
                    assert_eq!(receipt.is_some(), closes);
                    if let Some(receipt) = receipt {
                        assert!(receipt.source().target().checked_map_presence_law_v1());
                        assert_eq!(receipt.source().target().expression_result_class_v1(),
                            Some(crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::Bool));
                        let source = closed.unwrap();
                        assert!(source.field_map().is_some());
                        assert!(Rc::ptr_eq(receipt.operands(), source));
                        assert!(std::sync::Arc::ptr_eq(source.method(), input.function().method_call_source_v1(method.site()).unwrap()));
                    }
                }
                if let Some((site, _)) = input.function().method_calls().find(|(_, method)| method.selector() == "keys") {
                    let keys_site = OwnedExprSiteV1::new(input.owner(), site.clone());
                    let count = input.function().method_calls().find(|(_, method)| method.selector() == "length").unwrap().0;
                    let count_site = OwnedExprSiteV1::new(input.owner(), count.clone());
                    let snapshot_closes = provider == "MapBox" && extra_writer.is_empty();
                    let keys = ingress.ordinary_core_source_v1(&keys_site);
                    let count = ingress.ordinary_core_source_v1(&count_site);
                    assert_eq!(keys.is_some(), snapshot_closes, "keys: {text}");
                    assert_eq!(count.is_some(), snapshot_closes, "count receiver: {text}");
                    if let (Some(keys), Some(count)) = (keys, count) {
                        assert!(keys.source().target().checked_text_keys_law_v1());
                        assert!(count.source().target().checked_array_count_law_v1());
                        assert!(Rc::ptr_eq(count.operands().receiver_core().unwrap(), keys.operands()));
                        package.ordinary_new_claim_ledger.borrowed_i64_results[&input.owner()]
                            .as_ref().unwrap().require_source_i64_v1().unwrap();
                    }
                }
            }).unwrap();
        }
    }

}


#[test]
fn static_argument_plans_cover_literal_local_and_forward_without_execution_grant() {
    for (prefix, argument, parameter) in [
        ("", "\"bytes\"", ""),
        ("local key = \"bytes\"", "key", ""),
        ("local alias = key", "alias", "key"),
    ] {
        let text = format!("static box Values {{ sink(p,q) {{ return 0 }} caller({parameter}) {{ {prefix} local n = Values.sink({argument},7) return 0 }} }} static box Main {{ main() {{ return 0 }} }}");
        let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog_selection(
            &text, crate::mir::normal_callable_semantic_package::PackagePhysicalSelectionV1::ClosedAppExecutable,
        ).unwrap();
        let ingress = package.ordinary_new_claim_ledger.borrowed_formal_source.as_ref().unwrap().as_ref().unwrap();
        let original = ingress.source_incoming.source_rows_v1().filter_map(|row| match &row.source {
            crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(source)
                if source.target().name() == "sink" => Some(source), _ => None,
        }).next().unwrap();
        let slot = package.batch().declarations().find(|row| row.owner() == original.call_site().owner()).unwrap().batch_slot();
        package.batch().with_lowering_input(slot, |input| {
            let candidates: Vec<_> = original.argument_sites().iter().enumerate().map(|(ordinal, site)| {
                let value = if let Some(fact) = ingress.static_arguments.get(&(original.call_site().clone(), ordinal as u32)) {
                    BorrowedCallActualValueV1::SelfRooted { binding: fact.binding(), root: fact.formal() }
                } else { crate::mir::resolved_semantics::home_new_prefix::borrowed_actual_source_atom_v1(input, site).unwrap() };
                BorrowedCallActualCandidateV1 { ordinal: ordinal as u32, site: site.clone(), value }
            }).collect();
            let mut rows = prepare_static_source_actuals_v1(ingress, &package.parameter_contracts, original.call_site(), &candidates).unwrap().unwrap();
            bind_argument_sources_v1(input, ingress, None, original.call_site(), None, Some(&mut rows)).unwrap();
            let BorrowedCallActualEvidencePhaseV1::SourceStatic(identity) = &rows.phase else { panic!("source only") };
            assert_eq!(identity.argument_sources.len(), 2, "every ordered actual has its original plan");
            assert!(identity.core_arguments.is_empty());
            let first = Rc::clone(&identity.argument_sources[&0]);
            let second = Rc::clone(&identity.argument_sources[&1]);
            argument_source::validate_static_source_actuals_v1(ingress, &rows, original.call_site(), original.required_i64_arguments()).unwrap();
            assert!(rows.require_executable_v1().is_err(), "source coverage cannot issue an executable actual");
            if candidates.iter().any(|row| matches!(row.value, BorrowedCallActualValueV1::String(_) | BorrowedCallActualValueV1::Binding(_))) {
                let pending = BTreeMap::from([(original.call_site().clone(), Ok(rows.clone()))]);
                assert!(project_pending_static_source_arguments_for_route_v1(ingress, &pending, original.call_site(), original.required_i64_arguments(), false)
                    .unwrap_err().contains("text-expression-execution-pending"), "even an unused formal cannot skip evaluation");
            }
            if matches!(candidates[0].value, BorrowedCallActualValueV1::String(_)) {
                let mut forged = candidates.clone();
                forged[0].value = BorrowedCallActualValueV1::String("forged".into());
                let mut changed = prepare_static_source_actuals_v1(ingress, &package.parameter_contracts, original.call_site(), &forged).unwrap().unwrap();
                assert!(bind_argument_sources_v1(input, ingress, None, original.call_site(), None, Some(&mut changed)).is_err(), "literal payload must be original");
            }
            let mut repeated = prepare_static_source_actuals_v1(ingress, &package.parameter_contracts, original.call_site(), &candidates).unwrap().unwrap();
            bind_argument_sources_v1(input, ingress, None, original.call_site(), Some(&Ok(rows.clone())), Some(&mut repeated)).unwrap();
            assert_eq!(rows, repeated, "re-observation retains every SAME plan Rc");
            for mutation in 0..4 {
                let mut changed = rows.clone();
                let BorrowedCallActualEvidencePhaseV1::SourceStatic(identity) = &mut changed.phase else { unreachable!() };
                match mutation {
                    0 => { identity.argument_sources.remove(&1); }
                    1 => { identity.argument_sources.insert(2, Rc::clone(&second).into()); }
                    2 => { identity.argument_sources.insert(0, Rc::clone(&second).into()); }
                    3 => { identity.argument_sources.insert(1, Rc::clone(&first).into()); }
                    _ => unreachable!(),
                }
                assert!(argument_source::validate_static_source_actuals_v1(ingress, &changed, original.call_site(), original.required_i64_arguments()).is_err(), "full ordered coverage: {mutation}");
            }
            let reissued = crate::mir::resolved_value_profile::expression_plan::call_argument_expression_source_plan_v1(input, original.method_source(), 0).unwrap();
            assert!(!Rc::ptr_eq(&first, &reissued));
            let mut changed = rows.clone();
            let BorrowedCallActualEvidencePhaseV1::SourceStatic(identity) = &mut changed.phase else { unreachable!() };
            identity.argument_sources.insert(0, reissued.into());
            let mut staged = BTreeMap::from([(original.call_site().clone(), Ok(rows.clone()))]);
            super::super::super::stage_borrowed_call_actuals_v1(&mut staged, original.call_site(), Ok(Some(changed)));
            assert!(staged[original.call_site()].as_ref().unwrap_err().contains("repeated-walk-drift"));
        }).unwrap();
    }
}
