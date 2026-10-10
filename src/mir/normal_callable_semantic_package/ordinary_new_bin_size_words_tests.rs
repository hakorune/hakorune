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
                    && matches!(flow.exit_row(&exit), Some(Err(HomePrefixUnavailableV1::ReturnValueNotCovered(site))) if site == &exit);
                let stopped_in_prefix = first_uncovered == 10
                    && matches!(flow.exit_row(&exit), Some(Err(HomePrefixUnavailableV1::PrefixNotCovered(first)))
                        if first.node().segments() == [Segment::Body(10)]);
                assert!(reached_return || stopped_in_prefix, "{source}");
            }).unwrap();
    }
}
