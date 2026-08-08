//! Test-only parity receipt for the legacy scheduler route IDs.
//!
//! This mapping is deliberately outside the portable artifact. It proves the
//! migration inventory without giving the portable schema a route selector.

#![cfg(test)]

use super::producer_id::LoopRecipeProducerIdV1;
use super::route_id::LoopRouteId;

#[derive(Debug, PartialEq, Eq)]
struct LegacyRouteParityReceiptV1 {
    legacy_route: LoopRouteId,
    producer_id: Option<LoopRecipeProducerIdV1>,
    disposition: &'static str,
}

const RECEIPTS: &[LegacyRouteParityReceiptV1] = &[
    LegacyRouteParityReceiptV1 {
        legacy_route: LoopRouteId::AccumConstLoop,
        producer_id: Some(LoopRecipeProducerIdV1::DirectAccumV1),
        disposition: "portable_producer",
    },
    LegacyRouteParityReceiptV1 {
        legacy_route: LoopRouteId::LoopTrueBreakContinue,
        producer_id: Some(LoopRecipeProducerIdV1::LoopTrueBreakContinueV1),
        disposition: "portable_producer",
    },
    LegacyRouteParityReceiptV1 {
        legacy_route: LoopRouteId::NestedLoopMinimal,
        producer_id: Some(LoopRecipeProducerIdV1::NestedPredicateV1),
        disposition: "portable_producer",
    },
    LegacyRouteParityReceiptV1 {
        legacy_route: LoopRouteId::GenericLoopV0,
        producer_id: None,
        disposition: "legacy_only",
    },
    LegacyRouteParityReceiptV1 {
        legacy_route: LoopRouteId::GenericLoopV1,
        producer_id: None,
        disposition: "legacy_only",
    },
];

#[test]
fn legacy_route_parity_is_external_and_non_selecting() {
    assert_eq!(RECEIPTS.len(), 5);
    assert_eq!(
        RECEIPTS
            .iter()
            .filter(|receipt| receipt.disposition == "portable_producer")
            .count(),
        3
    );
    assert!(RECEIPTS.iter().any(|receipt| {
        receipt.legacy_route == LoopRouteId::GenericLoopV0 && receipt.producer_id.is_none()
    }));
    assert!(RECEIPTS.iter().any(|receipt| {
        receipt.legacy_route == LoopRouteId::GenericLoopV1 && receipt.producer_id.is_none()
    }));
}

#[test]
fn generic_g0_is_not_a_legacy_generic_route_alias() {
    assert!(!RECEIPTS
        .iter()
        .any(|receipt| { receipt.producer_id == Some(LoopRecipeProducerIdV1::GenericG0) }));
}

#[test]
fn producer_id_wire_keys_roundtrip_without_legacy_route_names() {
    for producer_id in [
        LoopRecipeProducerIdV1::DirectAccumV1,
        LoopRecipeProducerIdV1::LoopTrueBreakContinueV1,
        LoopRecipeProducerIdV1::NestedPredicateV1,
        LoopRecipeProducerIdV1::GenericG0,
        LoopRecipeProducerIdV1::CallableSingleLoopV1,
        LoopRecipeProducerIdV1::VariableAccumRecurrenceV1,
        LoopRecipeProducerIdV1::VariableAccumBreakV1,
    ] {
        let json = serde_json::to_string(&producer_id).expect("producer id encodes");
        let decoded: LoopRecipeProducerIdV1 =
            serde_json::from_str(&json).expect("producer id decodes");
        assert_eq!(decoded, producer_id);
        assert!(!json.contains("route"));
    }
}
