//! Same classifier retains original checked comparison operands and domains.
use super::super::BorrowedFormalUseDraftKindV1;
use super::draft;
use crate::mir::resolved_semantics::ResolvedBinaryOperatorV1;

#[test]
fn checked_compare_source_keeps_ordered_literal_child_and_alias() {
    for (body, literal_on_left) in [
        ("if p > 5 { return 1 } return 0", false),
        ("if 5 > p { return 1 } return 0", true),
        ("local q = p if q > 5 { return 1 } return 0", false),
    ] {
        let draft = draft(body).expect("existing admitted Greater");
        let row = draft
            .uses
            .iter()
            .find(|row| {
                matches!(
                    row.kind,
                    BorrowedFormalUseDraftKindV1::CompareOperand { .. }
                )
            })
            .unwrap();
        let BorrowedFormalUseDraftKindV1::CompareOperand { binary, source } = &row.kind else {
            unreachable!()
        };
        assert_eq!(source.operator, ResolvedBinaryOperatorV1::Greater);
        assert_ne!(source.left, source.right);
        assert_eq!(source.left.owner(), binary.owner());
        assert_eq!(source.right.owner(), binary.owner());
        let (literal_site, literal_value) = source.integer_literal.as_ref().unwrap();
        assert_eq!(*literal_value, 5);
        let (expected_literal, expected_formal) = if literal_on_left {
            (&source.left, &source.right)
        } else {
            (&source.right, &source.left)
        };
        assert_eq!(literal_site, expected_literal);
        assert_eq!(&row.site, expected_formal);
        assert_ne!(literal_site, &row.site);
    }
}

#[test]
fn checked_compare_source_distinguishes_same_value_at_different_sites() {
    let draft = draft("if p > 0 { return 1 } if p > 0 { return 2 } return 0")
        .expect("two existing Greater uses");
    let sources: Vec<_> = draft
        .uses
        .iter()
        .filter_map(|row| match &row.kind {
            BorrowedFormalUseDraftKindV1::CompareOperand { binary, source } => {
                Some((binary, source))
            }
            _ => None,
        })
        .collect();
    assert_eq!(sources.len(), 2);
    assert_ne!(sources[0].0, sources[1].0);
    let (first_site, first_value) = sources[0].1.integer_literal.as_ref().unwrap();
    let (second_site, second_value) = sources[1].1.integer_literal.as_ref().unwrap();
    assert_eq!((*first_value, *second_value), (0, 0));
    assert_ne!(first_site, second_site);
}

#[test]
fn checked_compare_source_borrowed_sibling_has_no_literal_receipt() {
    let draft =
        draft("local q = p if p > q { return 1 } return 0").expect("existing borrowed siblings");
    let sources: Vec<_> = draft
        .uses
        .iter()
        .filter_map(|row| match &row.kind {
            BorrowedFormalUseDraftKindV1::CompareOperand { binary, source } => {
                Some((row, binary, source))
            }
            _ => None,
        })
        .collect();
    assert_eq!(sources.len(), 2);
    assert_eq!(sources[0].1, sources[1].1);
    assert_eq!(sources[0].2, sources[1].2);
    assert_ne!(sources[0].0.site, sources[1].0.site);
    for (_, _, source) in sources {
        assert!(source.integer_literal.is_none());
    }
}

#[test]
fn checked_compare_source_retention_does_not_expand_acceptance() {
    for body in [
        "if p <= true { return 1 } return 0",
        "if p <= null { return 1 } return 0",
        "local result = p <= 0 return 0",
        "if p < 0 { return 1 } return 0",
        "if p >= 0 { return 1 } return 0",
        "if p > true { return 1 } return 0",
        "if p > null { return 1 } return 0",
        "local result = p > 0 return 0",
    ] {
        assert!(draft(body).is_err(), "unchanged rejection: {body}");
    }
}

#[test]
fn checked_compare_source_less_equal_retains_exact_integer_domain_and_order() {
    use crate::mir::dynamic_operator_contract::DynamicOperatorFamilyV1;
    for body in [
        "if p <= 5 { return 1 } return 0",
        "if 5 <= p { return 1 } return 0",
        "local a = p if a <= 5 { return 1 } return 0",
        "local a = p if p <= a { return 1 } return 0",
    ] {
        let draft = draft(body).expect("original LessEqual source");
        let mut count = 0;
        for row in &draft.uses {
            if let BorrowedFormalUseDraftKindV1::CompareOperand { binary, source } = &row.kind {
                assert_eq!(source.operator, ResolvedBinaryOperatorV1::LessEqual);
                assert_eq!(
                    source.envelope.domain().family(),
                    DynamicOperatorFamilyV1::LessEqual
                );
                assert_ne!(source.left, source.right);
                assert_eq!(source.left.owner(), binary.owner());
                assert_eq!(source.right.owner(), binary.owner());
                assert!(row.site == source.left || row.site == source.right);
                count += 1;
            }
        }
        assert!(count >= 1, "{body}");
    }
}

#[test]
fn checked_integer_return_retains_same_compare_and_exact_alias_source() {
    for body in [
        "if p <= 0 { return 0 } return p",
        "local q = p if q > 0 { return 0 } return q",
        "local q = p if p <= 0 { return 0 } return q",
    ] {
        let draft = draft(body).expect("checked Integer Return");
        let row = draft
            .uses
            .iter()
            .find(|row| matches!(row.kind, BorrowedFormalUseDraftKindV1::IntegerReturn { .. }))
            .unwrap();
        let BorrowedFormalUseDraftKindV1::IntegerReturn { guard, exit } = &row.kind else {
            unreachable!()
        };
        assert_eq!(
            exit.node().segments().first(),
            row.site.site().node().segments().first()
        );
        assert_eq!(draft.origins.get(&row.binding), Some(&row.formal));
        let compares: Vec<_> = draft
            .uses
            .iter()
            .filter_map(|compare| match &compare.kind {
                BorrowedFormalUseDraftKindV1::CompareOperand { binary, source } => {
                    Some((compare.formal, binary, source))
                }
                _ => None,
            })
            .collect();
        assert!(compares
            .iter()
            .any(|(formal, binary, source)| guard.matches_compare(*formal, binary, source)));
        for (formal, binary, source) in compares {
            // Equal text/domain reconstructed into a new Rc must never substitute the original.
            let reissued = std::rc::Rc::new(super::super::BorrowedCompareSourceV1 {
                operator: source.operator,
                left: source.left.clone(),
                right: source.right.clone(),
                integer_literal: source.integer_literal.clone(),
                integer_call: source.integer_call.clone(),
                envelope: source.envelope,
            });
            assert!(!guard.matches_compare(formal, binary, &reissued));
        }
    }
}

#[test]
fn checked_integer_return_rejects_bypass_inner_arm_and_rebound_source() {
    for body in [
        "return p",
        "if p <= 0 { return p } return 0",
        "if true { if p <= 0 { return 0 } } return p",
        "if p <= 0 { return 0 } p = 7 return p",
    ] {
        assert!(draft(body).is_err(), "no checked Return loan: {body}");
    }
}

#[test]
fn current_owner_integer_call_child_keeps_exact_loan_and_refusal_boundaries() {
    use super::super::{draft_borrowed_formal_source_product_v1, StaticOperandContextV1};
    use crate::mir::builder::SelectedNormalCallableKeyV1;
    use crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog;
    use crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1;
    use hakorune_mir_defs::CanonicalSameModuleCallableKeyV1;
    let program = |body: &str, limit: &str| {
        format!(
        "static box Gate {{ probe(p) {{ {body} }} {limit} }} static box Main {{ main() {{ return 0 }} }}"
    )
    };
    let caller = CanonicalSameModuleCallableKeyV1::static_box_method("Gate", "probe", 1);
    for (body, limit, accepted) in [
        (
            "if p > me.limit() { return 1 } return 0",
            "limit() { return 8 }",
            true,
        ),
        (
            "if me.limit() > p { return 1 } return 0",
            "limit() { return 8 }",
            true,
        ),
        (
            "local q = p if q <= me.limit() { return 1 } return 0",
            "limit() { return 8 }",
            true,
        ),
        (
            "if p <= 8 { return p * me.limit() } return 0",
            "limit() { return 8 }",
            true,
        ),
        (
            "if p <= 8 { return me.limit() * p } return 0",
            "limit() { return 8 }",
            true,
        ),
        (
            "local q = p if q <= 8 { return q * me.limit() } return 0",
            "limit() { return 8 }",
            true,
        ),
        (
            "if p <= 8 { return p * me.limit() } return 0",
            "limit() { return true }",
            false,
        ),
        (
            "if p <= 8 { return p * me.limit(7) } return 0",
            "limit(q: i64) { return q }",
            false,
        ),
        (
            "if p > me.limit() { return 1 } return 0",
            "limit() { return true }",
            false,
        ),
        (
            "if p > me.limit(7) { return 1 } return 0",
            "limit(q: i64) { return q }",
            false,
        ),
    ] {
        let text = program(body, limit);
        let package = issue_with_brand_catalog(&text).unwrap();
        let foreign = issue_with_brand_catalog(&text).unwrap();
        let slot = package
            .selected
            .batch_slot(&SelectedNormalCallableKeyV1::Cataloged(caller.clone()))
            .unwrap();
        let contract = package
            .parameter_contracts
            .iter()
            .find(|row| row.batch_slot == slot)
            .unwrap();
        let context = StaticOperandContextV1 {
            index: &package.source_static_claims_for_test,
            selected: &package.selected,
            contracts: &package.parameter_contracts,
            caller: &caller,
        };
        package.batch().with_lowering_input(slot, |input| {
            let product = draft_borrowed_formal_source_product_v1(
                input, contract, &package.instance_constructors, None, Some(&context),
            ).unwrap();
            if !accepted {
                assert!(product.draft.is_err(), "{body}: {limit}");
                return;
            }
            let draft = product.draft.unwrap();
            let call_site = input.function().method_calls().find(|(_, row)| row.selector() == "limit").unwrap().0;
            let owned = crate::mir::resolved_semantics::OwnedExprSiteV1::new(input.owner(), call_site.clone());
            let original = draft.static_operand_call_source_at(input, &owned).unwrap().unwrap();
            for row in &draft.uses {
                if let BorrowedFormalUseDraftKindV1::MulOperand { source, .. } = &row.kind {
                    assert!(draft.mul_operand_at(input, &row.site).unwrap().is_some());
                    assert!(source.integer_call_sources().any(|child| std::rc::Rc::ptr_eq(child, original)));
                }
            }
            assert_eq!(original.caller(), &caller);
            assert_eq!(original.target(), &CanonicalSameModuleCallableKeyV1::static_box_method("Gate", "limit", 0));
            assert!(original.argument_sites().is_empty());
            assert!(original.parameters().is_empty());
            assert!(original.required_i64_arguments().is_empty());
            assert!(original.require_qualified().is_err());
            // The production inventory must reuse the SAME source from its own
            // original draft; this transient draft intentionally has its own Rc.
            let ingress = package.ordinary_new_claim_ledger.borrowed_formal_source.as_ref().unwrap().as_ref().unwrap();
            let production_draft = ingress.source_definition_for(input.owner()).unwrap_or_else(|| panic!("original caller source missing: {body}"));
            let retained = production_draft.static_operand_call_source_at(input, &owned).unwrap().unwrap();
            let observed = ingress.source_incoming.exact_rows().find_map(|row| match &row.source {
                crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(source)
                    if source.call_site() == &owned => Some(source),
                _ => None,
            }).unwrap();
            assert!(std::rc::Rc::ptr_eq(retained, observed));
            assert!(draft_borrowed_formal_source_product_v1(
                input, contract, &package.instance_constructors, None, None,
            ).unwrap().draft.is_err(), "no index source, no call operand proof");
            for bad_context in [
                StaticOperandContextV1 { index: &foreign.source_static_claims_for_test, ..context },
                StaticOperandContextV1 { selected: &foreign.selected, ..context },
            ] {
                assert!(draft_borrowed_formal_source_product_v1(
                    input, contract, &package.instance_constructors, None, Some(&bad_context),
                ).is_err(), "foreign source cohort");
            }
            let duplicate: Vec<_> = package.parameter_contracts.iter().chain(package.parameter_contracts.iter()).map(|row| {
                OwnedCallableParameterContractDeclarationV1 {
                    owner: row.owner, batch_slot: row.batch_slot, mode: row.mode,
                    parameters: row.parameters.iter().map(|formal| crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractV1 {
                        ordinal: formal.ordinal, binding: formal.binding, kind: formal.kind.clone(),
                    }).collect(),
                }
            }).collect();
            let bad_context = StaticOperandContextV1 { contracts: &duplicate, ..context };
            assert!(draft_borrowed_formal_source_product_v1(
                input, contract, &package.instance_constructors, None, Some(&bad_context),
            ).is_err(), "duplicate caller/target contracts");
        }).unwrap();
    }
}
