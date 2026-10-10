use super::*;

#[test]
fn bin_size_words_root_mul_requires_ordered_integer_add_and_local() {
    use crate::mir::resolved_semantics::{
        home_new_prefix::HomePrefixUnavailableV1, SourceNodeSiteV1, SourcePathSegmentV1 as Segment,
        SourceStmtSiteV1,
    };
    let original = include_str!("../../../lang/src/hako_alloc/memory/size_class_box.hako");
    let words = "local words = (5 + top) * scale";
    let variants = [
        (original.to_string(), 11),
        (
            original.replacen(words, "local words = scale * (5 + top)", 1),
            10,
        ),
        (
            original.replacen(words, "local words = (true + top) * scale", 1),
            10,
        ),
        (
            original.replacen(words, "local words = (5 + top) * true", 1),
            10,
        ),
        (
            original.replacen(words, "local words = (5 + top + 1) * scale", 1),
            10,
        ),
        (
            original.replacen(words, "local words = (5 + top) / scale", 1),
            10,
        ),
    ];
    for (source, first_uncovered) in variants {
        let package = match brand_catalog_tests::issue_with_brand_catalog(&source) {
            Ok(package) => package,
            Err(error)
                if first_uncovered == 10
                    && format!("{error:?}").contains(
                        "[freeze:contract][ordinary-new/static-scalar/incoming-identity]",
                    ) =>
            {
                continue;
            }
            Err(error) => panic!("unexpected source refusal: {error:?}"),
        };
        let key = crate::mir::builder::CanonicalSameModuleCallableKeyV1::static_box_method(
            "SizeClassBox",
            "bin_size",
            1,
        );
        let slot = package
            .selected
            .batch_slot(&crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key))
            .unwrap();
        package.batch().with_lowering_input(slot, |input| {
                let exit = SourceStmtSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![Segment::Body(11)]));
                let flow = package.ordinary_new_claim_ledger.completion_for_owner(input.owner())
                    .unwrap().cleanup().root_flow().unwrap();
                let reached_return = first_uncovered == 11
                    && matches!(flow.exit_row(&exit), Some(Ok(_)));
                let stopped_in_prefix = first_uncovered == 10
                    && matches!(flow.exit_row(&exit), Some(Err(HomePrefixUnavailableV1::PrefixNotCovered(first)))
                        if first.node().segments() == [Segment::Body(10)]);
                assert!(reached_return || stopped_in_prefix, "{source}");
            }).unwrap();
    }
}

#[test]
fn size_to_bin_search_loop_keeps_distinct_early_and_tail_home_calls() {
    use crate::mir::resolved_semantics::OwnedExprSiteV1;
    let source = include_str!("../../../lang/src/hako_alloc/memory/size_class_box.hako");
    let package = brand_catalog_tests::issue_with_brand_catalog(source).unwrap();
    let key = crate::mir::builder::CanonicalSameModuleCallableKeyV1::static_box_method(
        "SizeClassBox",
        "size_to_bin",
        1,
    );
    let slot = package
        .selected
        .batch_slot(&crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key))
        .unwrap();
    package
        .batch()
        .with_lowering_input(slot, |input| {
            let ledger = &package.ordinary_new_claim_ledger;
            let other = crate::mir::builder::CanonicalSameModuleCallableKeyV1::static_box_method(
                "SizeClassBox",
                "bin_size",
                1,
            );
            let other_slot = package
                .selected
                .batch_slot(&crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(other))
                .unwrap();
            package
                .batch()
                .with_lowering_input(other_slot, |other_input| {
                    assert!(
                        crate::mir::builder::take_at_function_entry_v2(other_input, ledger)
                            .unwrap()
                            .is_none()
                    );
                })
                .unwrap();
            let completion = ledger.completion_for_owner(input.owner()).unwrap();
            let flow = completion.cleanup().root_flow().unwrap();
            assert!(flow.all_exits_ready(), "{flow:?}");
            assert_eq!(completion.explicit_sites().len(), 2);
            let product = ledger
                .take_preissued_static_i64_product_v2(input.owner())
                .unwrap()
                .unwrap();
            let semantic = product.semantic();
            let entry_loan = semantic.source_calls().0;
            let packet = ledger
                .take_static_loop_packet_source_v1(entry_loan.loop_site(), entry_loan.declaration())
                .expect("selected packet source")
                .expect("selected packet proof");
            assert!(std::rc::Rc::ptr_eq(
                packet.original_source(),
                entry_loan.original()
            ));
            let body_loan = semantic.source_calls().2;
            assert!(ledger
                .take_loop_static_body_scalar_source_v1(
                    body_loan.loop_site(),
                    body_loan.call_site(),
                )
                .expect("selected body scalar source")
                .is_ok());
            let entry = semantic.source_calls().0.call_site().clone();
            let header =
                OwnedExprSiteV1::new(input.owner(), semantic.roles().header_call_site.clone());
            let body = OwnedExprSiteV1::new(input.owner(), semantic.roles().body_call_site.clone());
            let tail = semantic.tail_call().call_site().clone();
            let early = flow
                .exit_row(&semantic.roles().return_site)
                .unwrap()
                .unwrap();
            let tail_exit = flow
                .exit_row(semantic.tail_call().return_site())
                .unwrap()
                .unwrap();
            assert_eq!(
                early
                    .covered_calls()
                    .iter()
                    .cloned()
                    .collect::<std::collections::BTreeSet<_>>(),
                [entry.clone(), header.clone(), body.clone()]
                    .into_iter()
                    .collect()
            );
            assert_eq!(
                tail_exit
                    .covered_calls()
                    .iter()
                    .cloned()
                    .collect::<std::collections::BTreeSet<_>>(),
                [entry, header, body, tail].into_iter().collect()
            );
        })
        .unwrap();
}

#[test]
fn size_to_bin_search_loop_rejects_changed_source_and_control() {
    let original = include_str!("../../../lang/src/hako_alloc/memory/size_class_box.hako");
    for (before, after) in [
        (
            "loop(bin <= me.max_regular_bin())",
            "loop(bin < me.max_regular_bin())",
        ),
        ("bin = bin + 1", "bin = bin + 2"),
        ("if n <= me.bin_size(bin)", "if bin <= me.bin_size(bin)"),
        (
            "bin = bin + 1",
            "if bin == 2 { return bin }\n      bin = bin + 1",
        ),
        ("bin = bin + 1", "break"),
        ("bin = bin + 1", "continue"),
        ("if n <= me.bin_size(bin)", "if n <= me.bin_size(true)"),
        ("if n <= me.bin_size(bin)", "if n <= bin"),
    ] {
        assert_eq!(original.matches(before).count(), 1);
        let source = original.replacen(before, after, 1);
        let package = brand_catalog_tests::issue_with_brand_catalog(&source).unwrap();
        let key = crate::mir::builder::CanonicalSameModuleCallableKeyV1::static_box_method(
            "SizeClassBox",
            "size_to_bin",
            1,
        );
        let slot = package
            .selected
            .batch_slot(&crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key))
            .unwrap();
        package
            .batch()
            .with_lowering_input(slot, |input| {
                let ledger = &package.ordinary_new_claim_ledger;
                let product = ledger
                    .take_preissued_static_i64_product_v2(input.owner())
                    .expect("selected source retains a refusal");
                assert!(
                    product.is_err(),
                    "{before}: altered source must not get a V2 product"
                );
                let flow = ledger
                    .completion_for_owner(input.owner())
                    .unwrap()
                    .cleanup()
                    .root_flow()
                    .unwrap();
                assert!(
                    !flow.all_exits_ready(),
                    "{before}: altered Loop admitted Home"
                );
            })
            .unwrap();
    }

    let before = "bin_size(bin) {\n    if bin <= 0";
    assert_eq!(original.matches(before).count(), 1);
    let source = original.replacen(
        before,
        "bin_size(bin) {\n    local scratch = []\n    if bin <= 0",
        1,
    );
    let package = brand_catalog_tests::issue_with_brand_catalog(&source).unwrap();
    let key = crate::mir::builder::CanonicalSameModuleCallableKeyV1::static_box_method(
        "SizeClassBox",
        "size_to_bin",
        1,
    );
    let slot = package
        .selected
        .batch_slot(&crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key))
        .unwrap();
    package
        .batch()
        .with_lowering_input(slot, |input| {
            let ledger = &package.ordinary_new_claim_ledger;
            let error = ledger
                .take_preissued_static_i64_product_v2(input.owner())
                .expect("selected source retains a refusal")
                .unwrap_err();
            assert!(error.contains("static-home-effect-unavailable"), "{error}");
            let flow = ledger
                .completion_for_owner(input.owner())
                .unwrap()
                .cleanup()
                .root_flow()
                .unwrap();
            assert!(!flow.all_exits_ready());
        })
        .unwrap();
}

#[test]
fn bin_size_return_mul_requires_i64_local_and_exact_static_call() {
    use crate::mir::resolved_semantics::{
        home_new_prefix::{HomePrefixUnavailableV1, TerminalRelationV1},
        SourceNodeSiteV1, SourcePathSegmentV1 as Segment, SourceStmtSiteV1,
    };
    let original = include_str!("../../../lang/src/hako_alloc/memory/size_class_box.hako");
    let returned = "return words * me.word_size()";
    let variants = [
        (original.to_string(), true),
        (
            original.replacen(returned, "return me.word_size() * words", 1),
            false,
        ),
        (
            original.replacen(returned, "return true * me.word_size()", 1),
            false,
        ),
        (original.replacen(returned, "return words * 2", 1), false),
    ];
    for (source, expected) in variants {
        let package = match brand_catalog_tests::issue_with_brand_catalog(&source) {
            Ok(package) => package,
            Err(error)
                if !expected
                    && format!("{error:?}").contains(
                        "[freeze:contract][ordinary-new/static-scalar/incoming-identity]",
                    ) =>
            {
                continue;
            }
            Err(error) => panic!("unexpected source refusal: {error:?}"),
        };
        let key = crate::mir::builder::CanonicalSameModuleCallableKeyV1::static_box_method(
            "SizeClassBox",
            "bin_size",
            1,
        );
        let slot = package
            .selected
            .batch_slot(&crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key))
            .unwrap();
        package.batch().with_lowering_input(slot, |input| {
            let exit = SourceStmtSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![Segment::Body(11)]));
            let ledger = &package.ordinary_new_claim_ledger;
            let flow = ledger.completion_for_owner(input.owner()).unwrap().cleanup().root_flow().unwrap();
            if expected {
                assert!(matches!(flow.exit_row(&exit), Some(Ok(_))), "{source}");
                assert!(matches!(ledger.terminal_relation_for_owner_at(input.owner(), &exit),
                    Some(TerminalRelationV1::I64Scalar(_))), "{source}");
            } else {
                assert!(matches!(flow.exit_row(&exit), Some(Err(HomePrefixUnavailableV1::ReturnValueNotCovered(site)))
                    if site == &exit), "{source}");
                assert!(ledger.terminal_relation_for_owner_at(input.owner(), &exit).is_none());
            }
        }).unwrap();
    }
}

#[test]
fn bin_size_signed_immediate_return_uses_exact_home_terminal_source() {
    use crate::mir::resolved_semantics::{
        home_new_prefix::{HomePrefixUnavailableV1, TerminalRelationV1},
        SourceNodeSiteV1, SourcePathSegmentV1 as Segment, SourceStmtSiteV1,
    };
    let original = include_str!("../../../lang/src/hako_alloc/memory/size_class_box.hako");
    for (source, expected) in [
        (original.to_string(), true),
        (original.replacen("return -1", "return -bin", 1), false),
    ] {
        let package = brand_catalog_tests::issue_with_brand_catalog(&source).unwrap();
        let key = crate::mir::builder::CanonicalSameModuleCallableKeyV1::static_box_method(
            "SizeClassBox",
            "bin_size",
            1,
        );
        let slot = package
            .selected
            .batch_slot(&crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key))
            .unwrap();
        package.batch().with_lowering_input(slot, |input| {
            let exit = SourceStmtSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
                Segment::Body(0), Segment::IfThen(0),
            ]));
            let ledger = &package.ordinary_new_claim_ledger;
            let flow = ledger.completion_for_owner(input.owner()).unwrap().cleanup().root_flow().unwrap();
            if expected {
                assert!(flow.all_exits_ready());
                assert!(matches!(flow.exit_row(&exit), Some(Ok(_))));
                assert!(matches!(ledger.terminal_relation_for_owner_at(input.owner(), &exit),
                    Some(TerminalRelationV1::IntegerLiteral(row)) if row.value() == -1));
            } else {
                assert!(matches!(flow.exit_row(&exit),
                    Some(Err(HomePrefixUnavailableV1::ReturnValueNotCovered(site))) if site == &exit));
                assert!(!flow.all_exits_ready());
                assert!(ledger.terminal_relation_for_owner_at(input.owner(), &exit).is_none());
            }
        }).unwrap();
        if expected {
            let good_size =
                crate::mir::builder::CanonicalSameModuleCallableKeyV1::static_box_method(
                    "SizeClassBox",
                    "good_size",
                    1,
                );
            let slot = package
                .selected
                .batch_slot(&crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(good_size))
                .unwrap();
            package
                .batch()
                .with_lowering_input(slot, |input| {
                    let exit = SourceStmtSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
                        Segment::Body(1),
                        Segment::IfThen(0),
                    ]));
                    let ledger = &package.ordinary_new_claim_ledger;
                    let flow = ledger
                        .completion_for_owner(input.owner())
                        .unwrap()
                        .cleanup()
                        .root_flow()
                        .unwrap();
                    assert!(flow.all_exits_ready());
                    assert!(
                        matches!(ledger.terminal_relation_for_owner_at(input.owner(), &exit),
                    Some(TerminalRelationV1::IntegerLiteral(row)) if row.value() == -1)
                    );
                })
                .unwrap();
        }
    }
}
