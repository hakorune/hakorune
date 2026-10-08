//! Exact source reach and correspondence, using the existing source-use fixture.
use super::super::tests::with_source_product;
use super::super::{BorrowedFormalUseDraftErrorV1, BorrowedFormalUseDraftKindV1};
use super::*;

#[test]
fn checked_actual_requires_same_sequence_normal_reach_and_unpoisoned_binding() {
    for (body, count, unsupported) in [
        (
            "if p <= 0 { return 0 } local out = me.sink(p) return 0",
            1,
            false,
        ),
        (
            "if p > 0 { } local a = p local out = me.sink(a) return 0",
            1,
            false,
        ),
        (
            "local a = p if a <= 0 { } local out = me.sink(p) return 0",
            1,
            false,
        ),
        ("if p <= true { } local out = me.sink(p) return 0", 0, true),
        ("if p <= null { } local out = me.sink(p) return 0", 0, true),
        (
            "if true { if p > 0 { } local out = me.sink(p) } return 0",
            1,
            false,
        ),
        (
            "if false { } else { if p <= 0 { return 0 } local out = me.sink(p) } return 0",
            1,
            false,
        ),
        // Reach is conditional on Normal completion; facts promise no positivity
        // or reachability, even when the later source call is unreachable.
        (
            "if p <= 0 { return 0 } else { return 0 } local out = me.sink(p) return 0",
            1,
            false,
        ),
        (
            "local bad = p * 2 if p > 0 { } local out = me.sink(p) return 0",
            1,
            true,
        ),
        (
            "if p > 0 { } local bad = p * 2 local out = me.sink(p) return 0",
            1,
            true,
        ),
        (
            "local out = me.sink(p) if p <= 0 { return 0 } return 0",
            0,
            false,
        ),
        (
            "if true { if p <= 0 { return 0 } } local out = me.sink(p) return 0",
            0,
            false,
        ),
        (
            "if true { if p > 0 { } } else { local out = me.sink(p) } return 0",
            0,
            false,
        ),
        (
            "loop(true) { if p <= 0 { break } } local out = me.sink(p) return 0",
            0,
            false,
        ),
        (
            "loop(true) { if p <= 0 { continue } break } local out = me.sink(p) return 0",
            0,
            false,
        ),
        (
            "if p == null { return 0 } local out = me.sink(p) return 0",
            0,
            false,
        ),
    ] {
        with_source_product(
            body,
            |_| {},
            |input, product| {
                let product = product.expect(body);
                assert_eq!(product.guarded_actuals.len(), count, "{body}");
                assert_eq!(
                    matches!(
                        product.draft,
                        Err(BorrowedFormalUseDraftErrorV1::UnsupportedUse(_))
                    ),
                    unsupported,
                    "{body}"
                );
                for ((call, ordinal), fact) in &product.guarded_actuals {
                    assert!(
                        fact.corroborates(input, call, *ordinal, fact.site.site()),
                        "{body}"
                    );
                }
            },
        );
    }
    for body in [
        "if p > 0 { } p = 1 local out = me.sink(p) return 0",
        "local a = p if p > 0 { } a = 1 local out = me.sink(p) return 0",
        "local a: i64 = p if p > 0 { } local out = me.sink(a) return 0",
        "local f = fn() { return p } if p > 0 { } local out = me.sink(p) return 0",
    ] {
        with_source_product(
            body,
            |_| {},
            |_, product| {
                assert!(
                    matches!(
                        product,
                        Err(BorrowedFormalUseDraftErrorV1::Rebound(_)
                            | BorrowedFormalUseDraftErrorV1::AnnotatedAlias(_)
                            | BorrowedFormalUseDraftErrorV1::Captured(_))
                    ),
                    "poison: {body}"
                );
            },
        );
    }
}

#[test]
fn checked_actual_borrows_same_compare_and_refuses_call_ordinal_site_or_guard_drift() {
    with_source_product(
        "if p > 0 { } local a = me.sink(p) local b = me.sink(p) return 0",
        |_| {},
        |input, product| {
            let mut product = product.unwrap();
            let draft = product.draft.unwrap();
            let compare = draft
                .uses
                .iter()
                .find_map(|row| {
                    if let BorrowedFormalUseDraftKindV1::CompareOperand { source, .. } = &row.kind {
                        Some(source)
                    } else {
                        None
                    }
                })
                .unwrap();
            let mut facts = product.guarded_actuals.values_mut();
            let first = facts.next().unwrap();
            let second = facts.next().unwrap();
            assert!(Rc::ptr_eq(compare, &first.guard.source));
            assert!(Rc::ptr_eq(&first.guard.source, &second.guard.source));
            assert!(!first.corroborates(input, &second.call, first.ordinal, first.site.site()));
            assert!(!first.corroborates(input, &first.call, 1, first.site.site()));
            assert!(!first.corroborates(input, &first.call, first.ordinal, second.site.site()));
            let original = first.site.clone();
            first.site = second.site.clone();
            assert!(!first.corroborates(input, &first.call, first.ordinal, original.site()));
            first.site = original;
            let guard = Rc::get_mut(&mut first.guard);
            assert!(
                guard.is_none(),
                "retained compare is shared, not reminted per use"
            );
            // Remove the sibling reference so guard identity can be deliberately
            // corrupted without exposing a production mutation API.
            let (_, mut fact) = product.guarded_actuals.pop_first().unwrap();
            product.guarded_actuals.clear();
            Rc::get_mut(&mut fact.guard).unwrap().statement = fact.call.site().node().clone();
            assert!(!fact.corroborates(input, &fact.call, fact.ordinal, fact.site.site()));
        },
    );
}
