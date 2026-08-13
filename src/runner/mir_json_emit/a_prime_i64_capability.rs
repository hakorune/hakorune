use crate::mir::function::FunctionMetadata;
use serde_json::json;

/// Emit the sealed post-session A-prime capability exactly once.
///
/// Absence is intentional for ordinary/legacy functions.  This encoder does
/// not infer rows from MIR instructions and does not synthesize an empty
/// capability.
pub(super) fn insert_a_prime_i64_physical_receipt_json(
    obj: &mut serde_json::Map<String, serde_json::Value>,
    metadata: &FunctionMetadata,
) {
    let Some(receipt) = metadata.a_prime_i64_physical_receipt() else {
        return;
    };
    debug_assert!(receipt.validate().is_ok());
    let value = json!({
        "schema_version": receipt.schema_version(),
        "backend_family": receipt.backend_family().as_str(),
        "formal_parameter_count": receipt.formal_parameter_count(),
        "fallback": false,
        "retry": false,
        "parameters": receipt.parameters().iter().map(|row| json!({
            "role": &row.role,
            "formal_parameter_index": row.formal_parameter_index,
            "value_id": row.value_id.as_u32(),
            "lane": row.lane.as_str(),
        })).collect::<Vec<_>>(),
        "call_edges": receipt.call_edges().iter().map(|row| json!({
            "site_id": row.site_id.0,
            "role": &row.role,
            "target_fingerprint": &row.target_fingerprint,
            "receiver_role": &row.receiver_role,
            "receiver_value_id": row.receiver_value_id.as_u32(),
            "receiver_lane": row.receiver_lane.as_str(),
            "arguments": row.arguments.iter().map(|arg| json!({
                "ordinal": arg.ordinal,
                "role": &arg.role,
                "value_id": arg.value_id.as_u32(),
                "lane": arg.lane.as_str(),
            })).collect::<Vec<_>>(),
            "result_value_id": row.result_value_id.as_u32(),
            "result_lane": row.result_lane.as_str(),
        })).collect::<Vec<_>>(),
        "returns": receipt.returns().iter().map(|row| json!({
            "site": &row.site,
            "block": row.block.as_u32(),
            "value_id": row.value_id.as_u32(),
            "lane": row.lane.as_str(),
        })).collect::<Vec<_>>(),
    });
    obj.insert("a_prime_i64_physical_receipt".to_string(), value);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::a_prime_i64_physical_receipt::{
        APrimeI64BackendFamilyV1, APrimeI64CallArgumentReceiptV1, APrimeI64CallEdgeReceiptV1,
        APrimeI64LaneV1, APrimeI64ParameterReceiptV1, APrimeI64PhysicalReceiptV1,
        APrimeI64ReturnReceiptV1,
    };
    use crate::mir::{BasicBlockId, ValueId};

    #[test]
    fn absent_receipt_does_not_create_metadata_key() {
        let metadata = FunctionMetadata::default();
        let mut obj = serde_json::Map::new();
        insert_a_prime_i64_physical_receipt_json(&mut obj, &metadata);
        assert!(!obj.contains_key("a_prime_i64_physical_receipt"));
    }

    #[test]
    fn sealed_receipt_emits_strict_schema() {
        let receipt = APrimeI64PhysicalReceiptV1::seal_for_test(
            APrimeI64BackendFamilyV1::Llvm,
            crate::mir::a_prime_i64_physical_receipt::A_PRIME_I64_FORMAL_PARAMETER_COUNT,
            vec![
                APrimeI64ParameterReceiptV1 {
                    role: "pos".into(),
                    formal_parameter_index: 1,
                    value_id: ValueId::new(2),
                    lane: APrimeI64LaneV1::ImmediateI64,
                },
                APrimeI64ParameterReceiptV1 {
                    role: "end".into(),
                    formal_parameter_index: 2,
                    value_id: ValueId::new(3),
                    lane: APrimeI64LaneV1::ImmediateI64,
                },
            ],
            vec![call("substring", 0), call("index_of", 1)],
            vec![
                APrimeI64ReturnReceiptV1 {
                    site: "inner".into(),
                    block: BasicBlockId::new(8),
                    value_id: ValueId::new(30),
                    lane: APrimeI64LaneV1::ImmediateI64,
                },
                APrimeI64ReturnReceiptV1 {
                    site: "outer".into(),
                    block: BasicBlockId::new(9),
                    value_id: ValueId::new(31),
                    lane: APrimeI64LaneV1::ImmediateI64,
                },
            ],
        )
        .expect("valid receipt");
        let mut metadata = FunctionMetadata::default();
        metadata
            .install_a_prime_i64_physical_receipt_for_test(receipt)
            .expect("receipt slot install");
        let mut obj = serde_json::Map::new();
        insert_a_prime_i64_physical_receipt_json(&mut obj, &metadata);
        let value = &obj["a_prime_i64_physical_receipt"];
        assert_eq!(value["schema_version"], 2);
        assert_eq!(value["backend_family"], "llvm");
        assert_eq!(value["formal_parameter_count"], 4);
        assert_eq!(value["parameters"].as_array().unwrap().len(), 2);
        assert_eq!(value["call_edges"].as_array().unwrap().len(), 2);
        assert_eq!(value["returns"].as_array().unwrap().len(), 2);
        assert_eq!(value["fallback"], false);
        assert_eq!(value["retry"], false);
    }

    fn call(role: &str, site_id: u32) -> APrimeI64CallEdgeReceiptV1 {
        APrimeI64CallEdgeReceiptV1 {
            site_id: crate::mir::checked_callout::CheckedCallOutSiteIdV1(site_id),
            role: role.into(),
            target_fingerprint: if role == "substring" {
                "substring/2".into()
            } else {
                "indexOf/1".into()
            },
            receiver_role: if role == "substring" {
                "src"
            } else {
                "pred_chars"
            }
            .into(),
            receiver_value_id: ValueId::new(if role == "substring" { 10 } else { 14 }),
            receiver_lane: APrimeI64LaneV1::OpaqueHandle,
            arguments: if role == "substring" {
                vec![
                    APrimeI64CallArgumentReceiptV1 {
                        ordinal: 0,
                        role: "start".into(),
                        value_id: ValueId::new(12),
                        lane: APrimeI64LaneV1::ImmediateI64,
                    },
                    APrimeI64CallArgumentReceiptV1 {
                        ordinal: 1,
                        role: "end".into(),
                        value_id: ValueId::new(13),
                        lane: APrimeI64LaneV1::ImmediateI64,
                    },
                ]
            } else {
                vec![APrimeI64CallArgumentReceiptV1 {
                    ordinal: 0,
                    role: "ch".into(),
                    value_id: ValueId::new(20),
                    lane: APrimeI64LaneV1::OpaqueHandle,
                }]
            },
            result_value_id: ValueId::new(if role == "substring" { 20 } else { 21 }),
            result_lane: if role == "substring" {
                APrimeI64LaneV1::OpaqueHandle
            } else {
                APrimeI64LaneV1::ImmediateI64
            },
        }
    }
}
