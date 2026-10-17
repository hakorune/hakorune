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
            let rows = rows.unwrap().unwrap();
            let BorrowedCallActualEvidencePhaseV1::SourceStatic(identity) = &rows.phase else { panic!("source phase only") };
            assert!(std::sync::Arc::ptr_eq(receipt.unwrap().source().source(), method));
            assert!(Rc::ptr_eq(receipt.unwrap().operands(), ingress.closed_core_source_v1(&site).unwrap()));
            assert!(Rc::ptr_eq(&identity.core_arguments[&0], receipt.unwrap()));
            assert_eq!(identity.integer_evidence.as_ref(), [false]);
            assert!(rows.require_executable_v1().is_err(), "unused formal cannot skip argument evaluation");
            let pending = BTreeMap::from([(original.call_site().clone(), Ok(rows.clone()))]);
            assert!(project_pending_static_source_arguments_for_route_v1(
                ingress, &pending, original.call_site(), original.required_i64_arguments(), false,
            ).unwrap_err().contains("core-expression-execution-pending"));
            for extra in [false, true] {
                let mut drift = rows.clone();
                let BorrowedCallActualEvidencePhaseV1::SourceStatic(identity) = &mut drift.phase else { unreachable!() };
                if extra { identity.core_arguments.insert(1, Rc::clone(receipt.unwrap())); }
                else { identity.core_arguments.clear(); }
                let pending = BTreeMap::from([(original.call_site().clone(), Ok(drift))]);
                // A missing map cannot recover via legacy transport; extras
                // cannot survive exact source/ordinal correspondence either.
                assert!(project_pending_static_source_arguments_for_route_v1(
                    ingress, &pending, original.call_site(), original.required_i64_arguments(), false,
                ).unwrap_err().contains("core-source-identity"));
            }
            let foreign = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog_selection(
                &text, crate::mir::normal_callable_semantic_package::PackagePhysicalSelectionV1::ClosedAppExecutable,
            ).unwrap();
            foreign.batch().with_lowering_input(slot, |substituted| {
                assert!(crate::mir::normal_callable_semantic_package::core_method_source::issue_ordinary_core_source_v1(
                    substituted, receipt.unwrap().operands(),
                ).unwrap_err().contains("source-identity"), "equal syntax cannot borrow another source loan");
            }).unwrap();
        }).unwrap();
    }
}

