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
