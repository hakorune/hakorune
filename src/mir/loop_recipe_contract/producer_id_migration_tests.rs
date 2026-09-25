//! Test-only parity receipt for the legacy scheduler route IDs.
//!
//! This mapping is deliberately outside the portable artifact. It proves the
//! migration inventory without giving the portable schema a route selector.
//! The M8 all19 closeout extends it to every canonical route: each row is
//! either backed by a landed portable cohort or an explicit `legacy_only`
//! typed pre-effect decline.

#![cfg(test)]

use super::producer_id::LoopRecipeProducerIdV1;
use super::route_id::LoopRouteId;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct LegacyRouteParityReceiptV1 {
    pub(crate) legacy_route: LoopRouteId,
    pub(crate) producer_id: Option<LoopRecipeProducerIdV1>,
    pub(crate) disposition: &'static str,
}

pub(crate) const RECEIPTS: &[LegacyRouteParityReceiptV1] = &[
    LegacyRouteParityReceiptV1 {
        legacy_route: LoopRouteId::LoopBreakRecipe,
        producer_id: Some(LoopRecipeProducerIdV1::VariableAccumBreakV1),
        disposition: "portable_producer",
    },
    LegacyRouteParityReceiptV1 {
        legacy_route: LoopRouteId::IfPhiJoin,
        producer_id: None,
        disposition: "legacy_only",
    },
    LegacyRouteParityReceiptV1 {
        legacy_route: LoopRouteId::LoopContinueOnly,
        producer_id: None,
        disposition: "legacy_only",
    },
    LegacyRouteParityReceiptV1 {
        legacy_route: LoopRouteId::LoopTrueEarlyExit,
        producer_id: None,
        disposition: "legacy_only",
    },
    LegacyRouteParityReceiptV1 {
        legacy_route: LoopRouteId::LoopSimpleWhile,
        producer_id: Some(LoopRecipeProducerIdV1::VariableAccumRecurrenceV1),
        disposition: "portable_producer",
    },
    LegacyRouteParityReceiptV1 {
        legacy_route: LoopRouteId::LoopCharMap,
        producer_id: None,
        disposition: "legacy_only",
    },
    LegacyRouteParityReceiptV1 {
        legacy_route: LoopRouteId::LoopArrayJoin,
        producer_id: None,
        disposition: "legacy_only",
    },
    LegacyRouteParityReceiptV1 {
        legacy_route: LoopRouteId::ScanWithInit,
        producer_id: Some(LoopRecipeProducerIdV1::ScanWithInitV2),
        disposition: "portable_v2_producer",
    },
    LegacyRouteParityReceiptV1 {
        legacy_route: LoopRouteId::SplitScan,
        producer_id: None,
        disposition: "legacy_only",
    },
    LegacyRouteParityReceiptV1 {
        legacy_route: LoopRouteId::BoolPredicateScan,
        producer_id: None,
        disposition: "legacy_only",
    },
    LegacyRouteParityReceiptV1 {
        legacy_route: LoopRouteId::AccumConstLoop,
        producer_id: Some(LoopRecipeProducerIdV1::DirectAccumV1),
        disposition: "portable_producer",
    },
    LegacyRouteParityReceiptV1 {
        legacy_route: LoopRouteId::NestedLoopMinimal,
        producer_id: Some(LoopRecipeProducerIdV1::NestedPredicateV1),
        disposition: "portable_producer",
    },
    LegacyRouteParityReceiptV1 {
        legacy_route: LoopRouteId::LoopTrueBreakContinue,
        producer_id: Some(LoopRecipeProducerIdV1::LoopTrueBreakContinueV1),
        disposition: "portable_producer",
    },
    LegacyRouteParityReceiptV1 {
        legacy_route: LoopRouteId::LoopCondBreakContinue,
        producer_id: Some(LoopRecipeProducerIdV1::LoopCondBreakContinueV1),
        disposition: "portable_producer",
    },
    LegacyRouteParityReceiptV1 {
        legacy_route: LoopRouteId::LoopCondContinueOnly,
        producer_id: None,
        disposition: "legacy_only",
    },
    LegacyRouteParityReceiptV1 {
        legacy_route: LoopRouteId::LoopCondContinueWithReturn,
        producer_id: None,
        disposition: "legacy_only",
    },
    LegacyRouteParityReceiptV1 {
        legacy_route: LoopRouteId::LoopCondReturnInBody,
        producer_id: None,
        disposition: "legacy_only",
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
    assert_eq!(RECEIPTS.len(), 19);
    assert_eq!(
        RECEIPTS
            .iter()
            .filter(|receipt| receipt.disposition == "portable_producer")
            .count(),
        6
    );
    assert_eq!(
        RECEIPTS
            .iter()
            .filter(|receipt| receipt.disposition == "portable_v2_producer")
            .count(),
        1
    );
    assert_eq!(
        RECEIPTS
            .iter()
            .filter(|receipt| receipt.disposition == "legacy_only")
            .count(),
        12
    );
    assert!(RECEIPTS.iter().any(|receipt| {
        receipt.legacy_route == LoopRouteId::GenericLoopV0 && receipt.producer_id.is_none()
    }));
    assert!(RECEIPTS.iter().any(|receipt| {
        receipt.legacy_route == LoopRouteId::GenericLoopV1 && receipt.producer_id.is_none()
    }));
}

#[test]
fn every_canonical_route_is_classified() {
    use crate::mir::loop_route_policy::CANONICAL_LOOP_ROUTE_ORDER_V1;
    for route in CANONICAL_LOOP_ROUTE_ORDER_V1 {
        assert_eq!(
            RECEIPTS
                .iter()
                .filter(|receipt| receipt.legacy_route == route)
                .count(),
            1,
            "route {route:?} must appear exactly once"
        );
    }
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
        LoopRecipeProducerIdV1::LoopCondBreakContinueV1,
        LoopRecipeProducerIdV1::NestedPredicateV1,
        LoopRecipeProducerIdV1::GenericG0,
        LoopRecipeProducerIdV1::CallableSingleLoopV1,
        LoopRecipeProducerIdV1::VariableAccumRecurrenceV1,
        LoopRecipeProducerIdV1::VariableAccumBreakV1,
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
