//! Exact source reach and correspondence, using the existing source-use fixture.
use super::super::tests::with_source_product;
use super::super::{BorrowedFormalUseDraftErrorV1, BorrowedFormalUseDraftKindV1};
use super::*;

#[test]
fn checked_operand_reach_lends_exact_normal_branch_and_retains_original_guard() {
    use crate::mir::resolved_semantics::ResolvedBinaryOperatorV1;

    for (body, admitted, branch) in [
        (
            "if p <= 8 { return p * me.word_size() } local out = me.sink(p) return 0",
            true,
            true,
        ),
        (
            "if p <= 8 { return 2 * p } local out = me.sink(p) return 0",
            true,
            true,
        ),
        (
            "if p <= 8 { } else { return p * 2 } local out = me.sink(p) return 0",
            true,
            true,
        ),
        (
            "local a = p if a <= 8 { return a * 2 } local out = me.sink(p) return 0",
            true,
            true,
        ),
        (
            "if p <= 8 { } local value = p * 2 local out = me.sink(p) return 0",
            true,
            false,
        ),
        (
            "local value = p * 2 if p <= 8 { } local out = me.sink(p) return 0",
            false,
            false,
        ),
        (
            "if true { local value = p * 2 } if p <= 8 { } local out = me.sink(p) return 0",
            false,
            false,
        ),
        (
            "if p <= 8 { if true { return p * 2 } } local out = me.sink(p) return 0",
            false,
            false,
        ),
        (
            "if p <= 8 { } if true { return p * 2 } local out = me.sink(p) return 0",
            false,
            false,
        ),
    ] {
        with_source_product(
            body,
            |_| {},
            |input, product| {
                let product = product.expect(body);
                // An instance call has no Static child loan in this fixture.
                // Literal cases now have an explicit Mul source; neither the
                // reach nor that source grants materialization/finishing.
                if let Ok(draft) = &product.draft {
                    assert!(admitted);
                    for row in &draft.uses {
                        if matches!(row.kind, BorrowedFormalUseDraftKindV1::MulOperand { .. }) {
                            assert!(draft.mul_operand_at(input, &row.site).unwrap().is_some());
                        }
                    }
                } else {
                    assert!(matches!(
                        product.draft,
                        Err(BorrowedFormalUseDraftErrorV1::UnsupportedUse(_))
                    ));
                }
                let actual = product.guarded_actuals.values().next().expect(body);
                let guard = &actual.guard;
                let multiply = input
                    .function()
                    .expression_source()
                    .binaries()
                    .find(|row| row.operator() == ResolvedBinaryOperatorV1::Multiply)
                    .expect(body);
                let (site, binding) = [multiply.lhs(), multiply.rhs()]
                    .into_iter()
                    .find_map(|site| match input.function().variable_ref(site) {
                        Some(ResolvedLexicalRefV1::Local(binding)) => Some((site, binding)),
                        _ => None,
                    })
                    .expect(body);
                let reach = guard.normal_path_for_operand(input, guard.formal, binding, site);
                assert_eq!(reach.is_some(), admitted, "{body}");
                if let Some(mut reach) = reach {
                    assert!(Rc::ptr_eq(reach.guard(), guard), "{body}");
                    assert!(reach.corroborates(input), "{body}");
                    assert_eq!(
                        matches!(reach.path, CheckedIntegerNormalPathV1::IfBranch { .. }),
                        branch,
                        "{body}"
                    );
                    assert!(
                        !guard.precedes(input, site) || !branch,
                        "old reach must stay unchanged: {body}"
                    );
                    if branch {
                        reach.path = CheckedIntegerNormalPathV1::FollowingStatement;
                        assert!(!reach.corroborates(input), "wrong retained path: {body}");
                    }
                }
                let comparison = input
                    .function()
                    .expression_source()
                    .binary(guard.binary.site())
                    .unwrap();
                let condition_site = [comparison.lhs(), comparison.rhs()]
                    .into_iter()
                    .find(|site| input.function().variable_ref(site).is_some())
                    .unwrap();
                assert!(
                    guard
                        .normal_path_for_operand(input, guard.formal, binding, condition_site)
                        .is_none(),
                    "the condition has not completed normally: {body}"
                );
            },
        );
    }
}

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
            false,
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
