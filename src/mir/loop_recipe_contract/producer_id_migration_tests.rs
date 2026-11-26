//! Stable provenance IDs in the Loop Recipe wire.

use super::producer_id::LoopRecipeProducerIdV1;

#[test]
fn producer_id_wire_keys_roundtrip_without_legacy_route_names() {
    for producer_id in [
        LoopRecipeProducerIdV1::DirectAccumV1,
        LoopRecipeProducerIdV1::LoopTrueBreakContinueV1,
        LoopRecipeProducerIdV1::LoopCondBreakContinueV1,
        LoopRecipeProducerIdV1::NestedPredicateV1,
        LoopRecipeProducerIdV1::GenericG0,
        LoopRecipeProducerIdV1::CallableSingleLoopV1,
        LoopRecipeProducerIdV1::Main0ContinueV1,
        LoopRecipeProducerIdV1::Main0InBodyStepV1,
        LoopRecipeProducerIdV1::Main0DerivedPredicateV1,
        LoopRecipeProducerIdV1::ScanWithInitV2,
    ] {
        let json = serde_json::to_string(&producer_id).expect("producer id encodes");
        let decoded: LoopRecipeProducerIdV1 =
            serde_json::from_str(&json).expect("producer id decodes");
        assert_eq!(decoded, producer_id);
        assert!(!json.contains("route"));
    }
}
