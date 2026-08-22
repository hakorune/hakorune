/*! Neutral result-kind view derived from `CoreMethodContractBox`.
 *
 * The `.hako` contract remains the semantic owner. The checked-in generated
 * table is a read-only projection. Selected semantic call-relation consumers
 * may borrow a row, but they must not reissue its result/effect meaning.
 */

use super::core_method_op::{CoreMethodLoweringTier, CoreMethodOp};
pub(crate) use super::generated::core_method_contract_rows::{
    CoreMethodManifestBrandV2, CORE_METHOD_CONTRACT_ROWS_V2, CORE_METHOD_MANIFEST_BRAND_V2,
};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CoreMethodResultKindV1 {
    I64Value,
    BoolValue,
    StringValue,
    NoValue,
    Dynamic,
}

/// Typed effect projection from the `.hako` CoreMethodContractBox row.
/// This remains separate from the Dynamic invocation envelope's runtime
/// observation and suspension semantics.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CoreMethodEffectV1 {
    PureRead,
    MutatesSlot,
    MutatesShape,
}

#[allow(dead_code)]
impl CoreMethodEffectV1 {
    pub(crate) fn as_manifest_name(self) -> &'static str {
        match self {
            Self::PureRead => "pure_read",
            Self::MutatesSlot => "mutates_slot",
            Self::MutatesShape => "mutates_shape",
        }
    }
}

#[allow(dead_code)]
impl CoreMethodResultKindV1 {
    pub(crate) fn as_manifest_name(self) -> &'static str {
        match self {
            Self::I64Value => "I64Value",
            Self::BoolValue => "BoolValue",
            Self::StringValue => "StringValue",
            Self::NoValue => "NoValue",
            Self::Dynamic => "Dynamic",
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CoreMethodSemanticLawV2 {
    Unprojected,
    CodePointCount,
    CodePointHalfOpenClamped,
}

#[allow(dead_code)]
impl CoreMethodSemanticLawV2 {
    pub(crate) fn as_manifest_name(self) -> &'static str {
        match self {
            Self::Unprojected => "Unprojected",
            Self::CodePointCount => "CodePointCount",
            Self::CodePointHalfOpenClamped => "CodePointHalfOpenClamped",
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct CoreMethodContractRowV2 {
    pub(crate) receiver_box: &'static str,
    pub(crate) canonical: &'static str,
    pub(crate) aliases: &'static [&'static str],
    pub(crate) arities: &'static [u32],
    pub(crate) semantic_law: &'static [(u32, CoreMethodSemanticLawV2)],
    pub(crate) op: CoreMethodOp,
    pub(crate) result_kind: CoreMethodResultKindV1,
    pub(crate) effect: CoreMethodEffectV1,
    pub(crate) lowering_tier: CoreMethodLoweringTier,
}

/// Opaque generated-row reference branded by the `.hako` manifest projection.
///
/// This is the only row product accepted by the CoreMethod Home issuer. It
/// carries exact arity alongside the generated row so a union row such as
/// `StringSubstring/1|2` cannot cross that boundary unspecialized.
#[derive(Debug, Clone, Copy)]
pub(crate) struct CoreMethodManifestRowRefV2 {
    brand: CoreMethodManifestBrandV2,
    row: &'static CoreMethodContractRowV2,
    arity: u32,
}

impl CoreMethodManifestRowRefV2 {
    pub(crate) const fn brand(self) -> CoreMethodManifestBrandV2 {
        self.brand
    }

    pub(crate) const fn row(self) -> &'static CoreMethodContractRowV2 {
        self.row
    }

    pub(crate) const fn arity(self) -> u32 {
        self.arity
    }

    pub(crate) const fn lowering_tier(self) -> CoreMethodLoweringTier {
        self.row.lowering_tier
    }

    pub(crate) fn semantic_law_for_arity(self) -> Option<CoreMethodSemanticLawV2> {
        self.row
            .semantic_law
            .iter()
            .find_map(|(law_arity, law)| (*law_arity == self.arity).then_some(*law))
    }
}

/// Issue one exact generated manifest row by operation identity and arity.
///
/// This is a generated-table projection, not a spelling/selector lookup. A
/// duplicate `(op, arity)` row is rejected rather than selected; the receiver
/// is retained on the generated row for the downstream typed issuer to check.
pub(crate) fn issue_core_method_manifest_row_ref_v2(
    op: CoreMethodOp,
    arity: u32,
) -> Option<CoreMethodManifestRowRefV2> {
    let mut rows = CORE_METHOD_CONTRACT_ROWS_V2
        .iter()
        .filter(|row| row.op == op && row.arities.contains(&arity));
    let row = rows.next()?;
    rows.next().is_none().then_some(CoreMethodManifestRowRefV2 {
        brand: CORE_METHOD_MANIFEST_BRAND_V2,
        row,
        arity,
    })
}

#[cfg(test)]
pub(crate) fn issue_core_method_manifest_row_ref_for_test(
    op: CoreMethodOp,
    arity: u32,
    foreign_brand: bool,
) -> Option<CoreMethodManifestRowRefV2> {
    let mut row = issue_core_method_manifest_row_ref_v2(op, arity)?;
    if foreign_brand {
        row.brand = super::generated::core_method_contract_rows::
            CORE_METHOD_MANIFEST_FOREIGN_BRAND_FOR_TEST;
    }
    Some(row)
}

#[cfg(test)]
pub(crate) fn issue_core_method_manifest_test_row_ref(
    row: &'static CoreMethodContractRowV2,
    arity: u32,
    foreign_brand: bool,
) -> CoreMethodManifestRowRefV2 {
    CoreMethodManifestRowRefV2 {
        brand: if foreign_brand {
            super::generated::core_method_contract_rows::CORE_METHOD_MANIFEST_FOREIGN_BRAND_FOR_TEST
        } else {
            CORE_METHOD_MANIFEST_BRAND_V2
        },
        row,
        arity,
    }
}

#[allow(dead_code)]
impl CoreMethodContractRowV2 {
    fn matches(&self, receiver_box: &str, spelling: &str, arity: u32) -> bool {
        self.receiver_box == receiver_box
            && self.arities.contains(&arity)
            && (self.canonical == spelling || self.aliases.contains(&spelling))
    }
}

#[allow(dead_code)]
pub(crate) fn lookup_core_method_result_row_v2(
    receiver_box: &str,
    spelling: &str,
    arity: u32,
) -> Option<&'static CoreMethodContractRowV2> {
    CORE_METHOD_CONTRACT_ROWS_V2
        .iter()
        .find(|row| row.matches(receiver_box, spelling, arity))
}

/// Resolve a generated callable row by its already selected core operation.
///
/// This is intentionally not a selector lookup. The semantic caller owns the
/// source dispatch cross-check; this helper only projects the generated
/// CoreMethodContractBox row that was selected by operation identity.
#[allow(dead_code)]
pub(crate) fn lookup_core_method_result_row_by_op_v2(
    receiver_box: &str,
    op: CoreMethodOp,
    arity: u32,
) -> Option<&'static CoreMethodContractRowV2> {
    let mut rows = CORE_METHOD_CONTRACT_ROWS_V2.iter().filter(|row| {
        row.receiver_box == receiver_box && row.op == op && row.arities.contains(&arity)
    });
    let row = rows.next()?;
    rows.next().is_none().then_some(row)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_length_aliases_select_one_i64_row() {
        let length = lookup_core_method_result_row_v2("StringBox", "length", 0).unwrap();
        let len = lookup_core_method_result_row_v2("StringBox", "len", 0).unwrap();
        let size = lookup_core_method_result_row_v2("StringBox", "size", 0).unwrap();

        assert!(std::ptr::eq(length, len));
        assert!(std::ptr::eq(length, size));
        assert_eq!(length.canonical, "length");
        assert_eq!(length.op, CoreMethodOp::StringLen);
        assert_eq!(length.result_kind, CoreMethodResultKindV1::I64Value);
        let length_ref = issue_core_method_manifest_row_ref_v2(CoreMethodOp::StringLen, 0)
            .expect("generated StringLen/0 row");
        assert_eq!(
            length_ref.semantic_law_for_arity(),
            Some(CoreMethodSemanticLawV2::CodePointCount)
        );
    }

    #[test]
    fn lookup_requires_exact_receiver_spelling_and_arity() {
        assert!(lookup_core_method_result_row_v2("StringBox", "length", 1).is_none());
        assert!(lookup_core_method_result_row_v2("StringBox", "missing", 0).is_none());
        assert!(lookup_core_method_result_row_v2("UserStringBox", "length", 0).is_none());
    }

    #[test]
    fn equal_spellings_on_distinct_receivers_select_distinct_rows() {
        let array = lookup_core_method_result_row_v2("ArrayBox", "length", 0).unwrap();
        let map = lookup_core_method_result_row_v2("MapBox", "length", 0).unwrap();
        let string = lookup_core_method_result_row_v2("StringBox", "length", 0).unwrap();

        assert_eq!(array.op, CoreMethodOp::ArrayLen);
        assert_eq!(map.op, CoreMethodOp::MapLen);
        assert_eq!(string.op, CoreMethodOp::StringLen);
        assert!(!std::ptr::eq(array, map));
        assert!(!std::ptr::eq(map, string));
    }

    #[test]
    fn unresolved_value_shapes_remain_dynamic() {
        for (receiver, method, arity) in [
            ("ArrayBox", "get", 1),
            ("MapBox", "get", 1),
            ("MapBox", "set", 2),
            ("MapBox", "delete", 1),
            ("MapBox", "keys", 0),
        ] {
            let row = lookup_core_method_result_row_v2(receiver, method, arity).unwrap();
            assert_eq!(row.result_kind, CoreMethodResultKindV1::Dynamic);
        }
    }

    #[test]
    fn operation_projection_keeps_text_scan_result_kinds_in_generated_rows() {
        let substring =
            lookup_core_method_result_row_by_op_v2("StringBox", CoreMethodOp::StringSubstring, 2)
                .expect("generated substring row");
        let index_of =
            lookup_core_method_result_row_by_op_v2("StringBox", CoreMethodOp::StringIndexOf, 1)
                .expect("generated indexOf row");
        assert_eq!(substring.result_kind, CoreMethodResultKindV1::StringValue);
        assert_eq!(index_of.result_kind, CoreMethodResultKindV1::I64Value);
        assert_eq!(substring.effect, CoreMethodEffectV1::PureRead);
        assert_eq!(index_of.effect, CoreMethodEffectV1::PureRead);
        let substring_ref = issue_core_method_manifest_row_ref_v2(CoreMethodOp::StringSubstring, 1)
            .expect("generated StringSubstring/1 row");
        let substring_two_ref =
            issue_core_method_manifest_row_ref_v2(CoreMethodOp::StringSubstring, 2)
                .expect("generated StringSubstring/2 row");
        assert_eq!(
            substring_ref.semantic_law_for_arity(),
            Some(CoreMethodSemanticLawV2::Unprojected)
        );
        assert_eq!(
            substring_two_ref.semantic_law_for_arity(),
            Some(CoreMethodSemanticLawV2::CodePointHalfOpenClamped)
        );
    }

    #[test]
    fn string_equals_source_row_is_design_only_bool_contract() {
        let equals = lookup_core_method_result_row_v2("StringBox", "equals", 1)
            .expect("generated StringEquals/1 row");
        assert_eq!(equals.op, CoreMethodOp::StringEquals);
        assert_eq!(equals.result_kind, CoreMethodResultKindV1::BoolValue);
        assert_eq!(equals.effect, CoreMethodEffectV1::PureRead);
        assert_eq!(equals.lowering_tier, CoreMethodLoweringTier::DesignOnly);
        assert!(equals.aliases.is_empty());
    }

    #[test]
    fn json_and_static_rust_rows_have_normalized_parity() {
        let manifest = include_str!(
            "../../lang/src/runtime/meta/generated/core_method_contract_manifest.json"
        );
        let parsed: serde_json::Value = serde_json::from_str(manifest).expect("manifest json");
        assert_eq!(parsed["schema"], "core_method_contract_manifest/v2");
        let rows = parsed["rows"].as_array().expect("manifest rows");
        assert_eq!(rows.len(), CORE_METHOD_CONTRACT_ROWS_V2.len());

        for json_row in rows {
            let receiver = json_row["box"].as_str().expect("box");
            let canonical = json_row["canonical"].as_str().expect("canonical");
            let result_kind = json_row["result_kind"].as_str().expect("result_kind");
            let effect = json_row["effect"].as_str().expect("effect");
            let op = json_row["core_op"].as_str().expect("core_op");
            let arities = json_row["arity"]
                .as_str()
                .expect("arity")
                .split('|')
                .map(|part| part.parse::<u32>().expect("checked arity"));
            let aliases = json_row["aliases"].as_array().expect("aliases");

            for arity in arities {
                let row = lookup_core_method_result_row_v2(receiver, canonical, arity)
                    .expect("canonical generated row");
                assert_eq!(row.op.as_manifest_name(), op);
                assert_eq!(row.result_kind.as_manifest_name(), result_kind);
                assert_eq!(row.effect.as_manifest_name(), effect);
                let law = json_row["semantic_law"]
                    .get(arity.to_string())
                    .and_then(serde_json::Value::as_str)
                    .expect("exact semantic law");
                assert_eq!(
                    row.semantic_law
                        .iter()
                        .find(|(law_arity, _)| *law_arity == arity)
                        .expect("static semantic law")
                        .1
                        .as_manifest_name(),
                    law
                );
                for alias in aliases {
                    let alias = alias.as_str().expect("alias spelling");
                    let selected = lookup_core_method_result_row_v2(receiver, alias, arity)
                        .expect("alias generated row");
                    assert!(std::ptr::eq(row, selected));
                }
            }
        }
    }
}
