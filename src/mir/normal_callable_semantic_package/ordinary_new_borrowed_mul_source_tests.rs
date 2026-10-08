//! Source meaning/refusal only. Runtime Mul acceptance is a separate obligation.
use super::super::tests::with_source_product;
use super::*;

#[test]
fn mul_source_preserves_original_order_alias_guard_and_complete_use_coverage() {
    for (body, admitted) in [
        ("if p <= 8 { return p * 2 } return 0", true),
        ("if p <= 8 { return 2 * p } return 0", true),
        ("if p <= 8 { return p * -2 } return 0", true),
        ("if p <= 8 { return p * p } return 0", true),
        ("local a = p if p <= 8 { return a * 2 } return 0", true),
        ("if p <= 8 { } else { return p * 2 } return 0", true),
        ("if p <= 8 { } local result = p * 2 return result", true),
        ("return p * 2", false),
        ("local result = p * 2 if p <= 8 { } return 0", false),
        ("if p == null { return p * 2 } return 0", false),
        ("if p <= 8 { return p * true } return 0", false),
        ("if p <= 8 { return p * null } return 0", false),
        ("if p <= 8 { return p + 2 } return 0", false),
        ("if p <= 8 { if true { return p * 2 } } return 0", false),
        ("if p <= 8 { } if true { return p * 2 } return 0", false),
    ] {
        with_source_product(
            body,
            |_| {},
            |input, product| {
                let product = product.unwrap();
                assert_eq!(product.draft.is_ok(), admitted, "{body}");
                let Ok(mut draft) = product.draft else { return };
                let rows: Vec<_> = draft
                    .uses
                    .iter()
                    .filter_map(|row| match &row.kind {
                        BorrowedFormalUseDraftKindV1::MulOperand {
                            binary,
                            source,
                            side,
                        } => Some((row.site.clone(), binary.clone(), Rc::clone(source), *side)),
                        _ => None,
                    })
                    .collect();
                assert!(!rows.is_empty(), "{body}");
                for (site, binary, source, side) in &rows {
                    assert_eq!(source.binary(), binary);
                    assert!(source.corroborates(input, &draft), "{body}");
                    assert!(draft.mul_operand_at(input, site).unwrap().is_some());
                    let original = input
                        .function()
                        .expression_source()
                        .binary(binary.site())
                        .unwrap();
                    assert_eq!(
                        source.view_at(*side).unwrap().operand().2.site(),
                        match side {
                            BorrowedMulSideV1::Left => original.lhs(),
                            BorrowedMulSideV1::Right => original.rhs(),
                        }
                    );
                    assert!(
                        rows.iter()
                            .all(|(_, _, other, _)| Rc::ptr_eq(source, other)),
                        "one product for original Mul"
                    );
                }
                let (site, _, source, _) = &rows[0];
                let target = draft.uses.iter_mut().find(|row| &row.site == site).unwrap();
                let BorrowedFormalUseDraftKindV1::MulOperand { side, .. } = &mut target.kind else {
                    unreachable!()
                };
                *side = match side {
                    BorrowedMulSideV1::Left => BorrowedMulSideV1::Right,
                    BorrowedMulSideV1::Right => BorrowedMulSideV1::Left,
                };
                assert!(
                    draft.mul_operand_at(input, site).is_err(),
                    "wrong side {body}"
                );
                assert!(!source.corroborates(input, &draft));
                // Restore exact side before independently removing Compare authority.
                let target = draft.uses.iter_mut().find(|row| &row.site == site).unwrap();
                let BorrowedFormalUseDraftKindV1::MulOperand { side, .. } = &mut target.kind else {
                    unreachable!()
                };
                *side = rows[0].3;
                draft.uses = draft
                    .uses
                    .into_vec()
                    .into_iter()
                    .filter(|row| {
                        !matches!(
                            row.kind,
                            BorrowedFormalUseDraftKindV1::CompareOperand { .. }
                        )
                    })
                    .collect::<Vec<_>>()
                    .into_boxed_slice();
                assert!(
                    draft.mul_operand_at(input, site).is_err(),
                    "missing Compare authority {body}"
                );
            },
        );
    }

    // Distinct formals must retain their own checked operand proof. The same
    // original comparison can establish both; checking only p cannot establish q.
    use crate::mir::builder::SelectedNormalCallableKeyV1;
    use crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog;
    use hakorune_mir_defs::CanonicalSameModuleCallableKeyV1;
    for (body, admitted) in [
        ("if p <= q { return p * q } return 0", true),
        ("if p <= 8 { } if q <= 8 { } return p * q", true),
        ("if p <= 8 { } return p * q", false),
    ] {
        let source = format!("static box Pair {{ probe(p, q) {{ {body} }} }} static box Main {{ main() {{ return 0 }} }}");
        let package = issue_with_brand_catalog(&source).unwrap();
        let key = SelectedNormalCallableKeyV1::Cataloged(
            CanonicalSameModuleCallableKeyV1::static_box_method("Pair", "probe", 2),
        );
        let slot = package.selected.batch_slot(&key).unwrap();
        let contract = package
            .parameter_contracts
            .iter()
            .find(|row| row.batch_slot == slot)
            .unwrap();
        package
            .batch()
            .with_lowering_input(slot, |input| {
                let product = draft_borrowed_formal_source_product_v1(
                    input,
                    contract,
                    &package.instance_constructors,
                    None,
                    None,
                )
                .unwrap();
                assert_eq!(product.draft.is_ok(), admitted, "{body}");
                let Ok(draft) = product.draft else { return };
                let row = draft
                    .uses
                    .iter()
                    .find(|row| matches!(row.kind, BorrowedFormalUseDraftKindV1::MulOperand { .. }))
                    .unwrap();
                let BorrowedFormalUseDraftKindV1::MulOperand { source, .. } = &row.kind else {
                    unreachable!()
                };
                assert!(source.corroborates(input, &draft));
                let left = source.view_at(BorrowedMulSideV1::Left).unwrap();
                let right = source.view_at(BorrowedMulSideV1::Right).unwrap();
                assert_ne!(left.operand().1, right.operand().1);
                assert!(!Rc::ptr_eq(left.guard(), right.guard()));
            })
            .unwrap();
    }
}
