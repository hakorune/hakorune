//! Whole-cohort scalar closure and the sibling-removal refusal.
use super::*;
use crate::mir::ValueId;

const SOURCE: &str = "static box Size { mul(p) { if p <= 8 { return p * 2 } return 0 } first(x: usize) { return me.mul(x) } second() { local y = 4 return me.mul(y) } } static box Main { main() { return 0 } }";

#[test]
fn current_owner_scalar_callers_finish_together_and_lend_the_mul_entry() {
    let qualified_only = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        "static box Size { mul(p) { if p <= 8 { return p * 2 } return 0 } } static box Layout { class_size() { return Size.mul(4) } } static box Main { main() { return 0 } }"
    ).unwrap();
    let ledger = &qualified_only.ordinary_new_claim_ledger;
    let ingress = ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    for call in ingress.source_incoming.exact_rows() {
        assert!(ingress
            .scalar_qualified_source_call_v1(&call.call)
            .unwrap()
            .is_none());
    }
    for qualified_siblings in [false, true] {
        let source = if qualified_siblings {
            format!("{SOURCE} static box Layout {{ class_size(id) {{ if id == 0 {{ return Size.mul(4) }} return Size.mul(8) }} }}")
        } else {
            SOURCE.to_owned()
        };
        let mut package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(&source).unwrap();
        let ledger = &package.ordinary_new_claim_ledger;
        let ingress = ledger
            .borrowed_formal_source
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap();
        let owner = package
            .parameter_contracts
            .iter()
            .find(|row| {
                package
                    .selected
                    .key_for_batch_slot(row.batch_slot)
                    .is_some_and(|key| {
                        matches!(key,
                SelectedNormalCallableKeyV1::Cataloged(key) if key.name() == "mul")
                    })
            })
            .unwrap()
            .owner;
        assert!(ingress.source_only_definitions.contains_key(&owner));
        let cohort = ledger
            .checked_completed_static_scalar_cohort_v1(ingress, owner)
            .unwrap()
            .unwrap_or_else(|| {
                panic!(
                    "qualified={qualified_siblings}; inputs={:?}",
                    ingress
                        .source_incoming
                        .exact_rows()
                        .filter(|row| row.callee == owner)
                        .map(|row| (
                            &row.call,
                            ledger.borrowed_formal_actuals.get(&row.call).map(|a| a
                                .as_ref()
                                .map(|a| (std::mem::discriminant(&a.phase), &a.opaque_actuals)))
                        ))
                        .collect::<Vec<_>>()
                )
            });
        assert_eq!(cohort.len(), if qualified_siblings { 4 } else { 2 });
        let mut qualified = Vec::new();
        for call in cohort.iter() {
            let super::super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(
                original,
            ) = &call.source
            else {
                panic!("original Static source");
            };
            let actual = ledger.borrowed_formal_actuals[&call.call].as_ref().unwrap();
            if original.is_qualified() {
                assert!(matches!(
                    actual.phase,
                    BorrowedCallActualEvidencePhaseV1::Executable
                ));
                assert!(std::ptr::eq(
                    ingress
                        .scalar_qualified_source_call_v1(&call.call)
                        .unwrap()
                        .unwrap(),
                    *call
                ));
                let projected = super::super::static_source::project_pending_static_source_arguments_for_route_v1(
                    ingress, &ledger.borrowed_formal_actuals, &call.call,
                    original.required_i64_arguments(), false,
                ).unwrap().unwrap();
                assert_eq!(projected, actual.ordered_arguments);
                qualified.push((call.call.clone(), actual.clone()));
                continue;
            }
            let (finished_source, binding, site) =
                original_scalar_source_v1(actual).unwrap().unwrap();
            assert!(Rc::ptr_eq(finished_source, original));
            let candidate = BorrowedCallActualCandidateV1 {
                ordinal: 0,
                site: site.clone(),
                value: BorrowedCallActualValueV1::Scalar(binding, SourceScalarKind::Integer),
            };
            let source_actual = super::super::static_source::prepare_static_source_actuals_v1(
                ingress,
                &package.parameter_contracts,
                &call.call,
                &[candidate],
            )
            .unwrap()
            .unwrap();
            let (before, original_binding, original_site) =
                original_scalar_source_v1(&source_actual).unwrap().unwrap();
            assert!(Rc::ptr_eq(before, finished_source));
            assert_eq!((original_binding, original_site), (binding, site));
            assert!(ledger
                .selected_static_local_source_v1(&call.call)
                .unwrap()
                .is_some());
        }
        let contract = package
            .parameter_contracts
            .iter()
            .find(|row| row.owner == owner)
            .unwrap();
        ledger
            .record_borrowed_ordinary_entry_values_v1(
                owner,
                Ok(Box::new([(0, contract.parameters[0].binding, ValueId(73))])),
            )
            .unwrap();
        let mul = ingress.source_only_definitions[&owner].uses.iter()
        .find_map(|row| match &row.kind {
            super::super::super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1::MulOperand { binary, .. } => Some(binary.clone()),
            _ => None,
        }).unwrap();
        assert!(ledger
            .prepare_borrowed_mul_source_v1(owner, &mul, Some(crate::mir::BinaryOp::Mul))
            .unwrap()
            .is_some());
        let (compare, literal, integer) = ingress.source_only_definitions[&owner].uses.iter()
        .find_map(|row| match &row.kind {
            super::super::super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1::CompareOperand { binary, source } => {
                let (literal, integer) = source.integer_literal()?;
                Some((binary.clone(), literal.clone(), integer))
            }
            _ => None,
        }).unwrap();
        assert!(ledger
            .prepare_borrowed_compare_source_v1(owner, &compare, Some(crate::mir::CompareOp::Le))
            .unwrap()
            .is_some());
        assert!(ledger
            .prepare_borrowed_compare_integer_literal_v1(owner, &literal, integer)
            .unwrap()
            .is_some());
        assert!(
            ledger
                .verify_borrowed_compare_reuse_v1(owner, std::iter::empty())
                .unwrap_err()
                .contains("reuse-coverage"),
            "completed source requires the original comparison append"
        );
        let other = package
            .parameter_contracts
            .iter()
            .find(|row| row.owner != owner)
            .unwrap()
            .owner;
        let foreign = OwnedExprSiteV1::new(other, compare.site().clone());
        assert!(ledger
            .prepare_borrowed_compare_source_v1(owner, &foreign, Some(crate::mir::CompareOp::Le))
            .unwrap_err()
            .contains("source-owner"));
        let removed = cohort[0].call.clone();
        drop(cohort);
        let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        // Finishing again neither replaces completed scalar receipts nor rewrites
        // the qualified literal protocol.
        let before = ledger.borrowed_formal_actuals.clone();
        ledger
            .finish_static_scalar_input_actuals_v1(
                &package.selected,
                &package.parameter_contracts,
                &package.physical_signature,
                &package.result_contracts,
            )
            .unwrap();
        assert_eq!(ledger.borrowed_formal_actuals, before);
        for (site, original) in &qualified {
            let saved = ledger.borrowed_formal_actuals.remove(site).unwrap();
            let ingress = ledger
                .borrowed_formal_source
                .as_ref()
                .unwrap()
                .as_ref()
                .unwrap();
            assert!(ledger
                .checked_completed_static_scalar_cohort_v1(ingress, owner)
                .unwrap()
                .is_none());
            ledger.borrowed_formal_actuals.insert(site.clone(), saved);
            let row = ledger
                .borrowed_formal_actuals
                .get_mut(site)
                .unwrap()
                .as_mut()
                .unwrap();
            row.opaque_actuals[0].source = BorrowedFormalActualSourceV1::Bool(true);
            let ingress = ledger
                .borrowed_formal_source
                .as_ref()
                .unwrap()
                .as_ref()
                .unwrap();
            assert!(ledger
                .checked_completed_static_scalar_cohort_v1(ingress, owner)
                .is_err());
            ledger
                .borrowed_formal_actuals
                .insert(site.clone(), Ok(original.clone()));
            let scalar_phase = before[&removed].as_ref().unwrap().phase.clone();
            ledger
                .borrowed_formal_actuals
                .get_mut(site)
                .unwrap()
                .as_mut()
                .unwrap()
                .phase = scalar_phase;
            let ingress = ledger
                .borrowed_formal_source
                .as_ref()
                .unwrap()
                .as_ref()
                .unwrap();
            assert!(ledger
                .checked_completed_static_scalar_cohort_v1(ingress, owner)
                .unwrap()
                .is_none());
            ledger
                .borrowed_formal_actuals
                .insert(site.clone(), Ok(original.clone()));
            let row = ledger
                .borrowed_formal_actuals
                .get_mut(site)
                .unwrap()
                .as_mut()
                .unwrap();
            row.opaque_actuals[0].formal =
                BindingRefV1::new(other, row.opaque_actuals[0].formal.binding());
            let ingress = ledger
                .borrowed_formal_source
                .as_ref()
                .unwrap()
                .as_ref()
                .unwrap();
            assert!(ledger
                .checked_completed_static_scalar_cohort_v1(ingress, owner)
                .is_err());
            ledger
                .borrowed_formal_actuals
                .insert(site.clone(), Ok(original.clone()));
        }

        let original_completion = ledger.completion_index[&owner].clone();
        ledger
            .completion_index
            .insert(owner, ledger.completion_index[&other].clone());
        let ingress = ledger
            .borrowed_formal_source
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap();
        assert!(ledger
            .checked_completed_static_scalar_cohort_v1(ingress, owner)
            .is_err());
        ledger.completion_index.insert(owner, original_completion);
        let saved = ledger.borrowed_formal_actuals.remove(&removed).unwrap();
        let ingress = ledger
            .borrowed_formal_source
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap();
        assert!(ledger
            .checked_completed_static_scalar_cohort_v1(ingress, owner)
            .unwrap()
            .is_none());
        assert!(ledger
            .prepare_borrowed_mul_source_v1(owner, &mul, Some(crate::mir::BinaryOp::Mul))
            .unwrap_err()
            .contains("source-only-entry"));
        assert!(ledger
            .prepare_borrowed_compare_source_v1(owner, &compare, Some(crate::mir::CompareOp::Le))
            .unwrap_err()
            .contains("source-only-entry"));
        assert!(ledger
            .prepare_borrowed_compare_integer_literal_v1(owner, &literal, integer)
            .unwrap_err()
            .contains("source-only-entry"));
        assert!(
            ledger
                .prepare_borrowed_compare_source_v1(owner, &mul, Some(crate::mir::CompareOp::Le))
                .unwrap()
                .is_none(),
            "unrelated source site stays unarmed even without its cohort"
        );
        assert!(ledger
            .prepare_borrowed_compare_integer_literal_v1(owner, &mul, integer)
            .unwrap()
            .is_none());
        ledger
            .borrowed_formal_actuals
            .insert(removed.clone(), saved);
        let row = ledger
            .borrowed_formal_actuals
            .get_mut(&removed)
            .unwrap()
            .as_mut()
            .unwrap();
        row.opaque_actuals[0].source = BorrowedFormalActualSourceV1::Bool(true);
        let ingress = ledger
            .borrowed_formal_source
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap();
        assert!(ledger
            .checked_completed_static_scalar_cohort_v1(ingress, owner)
            .unwrap_err()
            .contains("actual-identity"));
    }
}
