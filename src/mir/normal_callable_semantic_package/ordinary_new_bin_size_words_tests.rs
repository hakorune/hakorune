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
