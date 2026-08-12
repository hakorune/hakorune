/*! Neutral result-kind view derived from `CoreMethodContractBox`.
 *
 * The `.hako` contract remains the semantic owner. The checked-in generated
 * table is a read-only projection. Selected semantic call-relation consumers
 * may borrow a row, but they must not reissue its result/effect meaning.
 */

use super::core_method_op::CoreMethodOp;
use super::generated::core_method_contract_rows::CORE_METHOD_CONTRACT_RESULT_ROWS_V1;

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
#[derive(Debug)]
pub(crate) struct CoreMethodContractResultRowV1 {
    pub(crate) receiver_box: &'static str,
    pub(crate) canonical: &'static str,
    pub(crate) aliases: &'static [&'static str],
    pub(crate) arities: &'static [u32],
    pub(crate) op: CoreMethodOp,
    pub(crate) result_kind: CoreMethodResultKindV1,
    pub(crate) effect: CoreMethodEffectV1,
}

#[allow(dead_code)]
impl CoreMethodContractResultRowV1 {
    fn matches(&self, receiver_box: &str, spelling: &str, arity: u32) -> bool {
        self.receiver_box == receiver_box
            && self.arities.contains(&arity)
            && (self.canonical == spelling || self.aliases.contains(&spelling))
    }
}

#[allow(dead_code)]
pub(crate) fn lookup_core_method_result_row_v1(
    receiver_box: &str,
    spelling: &str,
    arity: u32,
) -> Option<&'static CoreMethodContractResultRowV1> {
    CORE_METHOD_CONTRACT_RESULT_ROWS_V1
        .iter()
        .find(|row| row.matches(receiver_box, spelling, arity))
}

/// Resolve a generated callable row by its already selected core operation.
///
/// This is intentionally not a selector lookup. The semantic caller owns the
/// source dispatch cross-check; this helper only projects the generated
/// CoreMethodContractBox row that was selected by operation identity.
#[allow(dead_code)]
pub(crate) fn lookup_core_method_result_row_by_op_v1(
    receiver_box: &str,
    op: CoreMethodOp,
    arity: u32,
) -> Option<&'static CoreMethodContractResultRowV1> {
    let mut rows = CORE_METHOD_CONTRACT_RESULT_ROWS_V1.iter().filter(|row| {
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
        let length = lookup_core_method_result_row_v1("StringBox", "length", 0).unwrap();
        let len = lookup_core_method_result_row_v1("StringBox", "len", 0).unwrap();
        let size = lookup_core_method_result_row_v1("StringBox", "size", 0).unwrap();

        assert!(std::ptr::eq(length, len));
        assert!(std::ptr::eq(length, size));
        assert_eq!(length.canonical, "length");
        assert_eq!(length.op, CoreMethodOp::StringLen);
        assert_eq!(length.result_kind, CoreMethodResultKindV1::I64Value);
    }

    #[test]
    fn lookup_requires_exact_receiver_spelling_and_arity() {
        assert!(lookup_core_method_result_row_v1("StringBox", "length", 1).is_none());
        assert!(lookup_core_method_result_row_v1("StringBox", "missing", 0).is_none());
        assert!(lookup_core_method_result_row_v1("UserStringBox", "length", 0).is_none());
    }

    #[test]
    fn equal_spellings_on_distinct_receivers_select_distinct_rows() {
        let array = lookup_core_method_result_row_v1("ArrayBox", "length", 0).unwrap();
        let map = lookup_core_method_result_row_v1("MapBox", "length", 0).unwrap();
        let string = lookup_core_method_result_row_v1("StringBox", "length", 0).unwrap();

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
            let row = lookup_core_method_result_row_v1(receiver, method, arity).unwrap();
            assert_eq!(row.result_kind, CoreMethodResultKindV1::Dynamic);
        }
    }

    #[test]
    fn operation_projection_keeps_text_scan_result_kinds_in_generated_rows() {
        let substring =
            lookup_core_method_result_row_by_op_v1("StringBox", CoreMethodOp::StringSubstring, 2)
                .expect("generated substring row");
        let index_of =
            lookup_core_method_result_row_by_op_v1("StringBox", CoreMethodOp::StringIndexOf, 1)
                .expect("generated indexOf row");
        assert_eq!(substring.result_kind, CoreMethodResultKindV1::StringValue);
        assert_eq!(index_of.result_kind, CoreMethodResultKindV1::I64Value);
        assert_eq!(substring.effect, CoreMethodEffectV1::PureRead);
        assert_eq!(index_of.effect, CoreMethodEffectV1::PureRead);
    }

    #[test]
    fn json_and_static_rust_rows_have_normalized_parity() {
        let manifest = include_str!(
            "../../lang/src/runtime/meta/generated/core_method_contract_manifest.json"
        );
        let parsed: serde_json::Value = serde_json::from_str(manifest).expect("manifest json");
        assert_eq!(parsed["schema"], "core_method_contract_manifest/v1");
        let rows = parsed["rows"].as_array().expect("manifest rows");
        assert_eq!(rows.len(), CORE_METHOD_CONTRACT_RESULT_ROWS_V1.len());

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
                let row = lookup_core_method_result_row_v1(receiver, canonical, arity)
                    .expect("canonical generated row");
                assert_eq!(row.op.as_manifest_name(), op);
                assert_eq!(row.result_kind.as_manifest_name(), result_kind);
                assert_eq!(row.effect.as_manifest_name(), effect);
                for alias in aliases {
                    let alias = alias.as_str().expect("alias spelling");
                    let selected = lookup_core_method_result_row_v1(receiver, alias, arity)
                        .expect("alias generated row");
                    assert!(std::ptr::eq(row, selected));
                }
            }
        }
    }
}
