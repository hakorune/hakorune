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
    let Some(receipt) = metadata.a_prime_i64_physical_receipt.as_ref() else {
        return;
    };
    debug_assert!(receipt.validate().is_ok());
    let value = json!({
        "schema_version": receipt.schema_version(),
        "backend_family": receipt.backend_family().as_str(),
        "fallback": false,
        "retry": false,
        "parameters": receipt.parameters().iter().map(|row| json!({
            "role": &row.role,
            "formal_parameter_index": row.formal_parameter_index,
            "value_id": row.value_id.as_u32(),
            "lane": row.lane.as_str(),
        })).collect::<Vec<_>>(),
        "call_edges": receipt.call_edges().iter().map(|row| json!({
            "role": &row.role,
            "block": row.block.as_u32(),
            "instruction_index": row.instruction_index,
            "target_fingerprint": &row.target_fingerprint,
            "arguments": row.arguments.iter().map(|arg| json!({
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
        let receipt = APrimeI64PhysicalReceiptV1::seal(
            APrimeI64BackendFamilyV1::Llvm,
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
            vec![call("substring", 4), call("index_of", 5)],
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
        metadata.a_prime_i64_physical_receipt = Some(receipt);
        let mut obj = serde_json::Map::new();
        insert_a_prime_i64_physical_receipt_json(&mut obj, &metadata);
        let value = &obj["a_prime_i64_physical_receipt"];
        assert_eq!(value["schema_version"], 1);
        assert_eq!(value["backend_family"], "llvm");
        assert_eq!(value["parameters"].as_array().unwrap().len(), 2);
        assert_eq!(value["call_edges"].as_array().unwrap().len(), 2);
        assert_eq!(value["returns"].as_array().unwrap().len(), 2);
        assert_eq!(value["fallback"], false);
        assert_eq!(value["retry"], false);
    }

    fn call(role: &str, instruction_index: usize) -> APrimeI64CallEdgeReceiptV1 {
        APrimeI64CallEdgeReceiptV1 {
            role: role.into(),
            block: BasicBlockId::new(3),
            instruction_index,
            target_fingerprint: format!("{role}/2"),
            arguments: vec![APrimeI64CallArgumentReceiptV1 {
                value_id: ValueId::new(20),
                lane: APrimeI64LaneV1::OpaqueHandle,
            }],
            result_value_id: ValueId::new(21),
            result_lane: APrimeI64LaneV1::OpaqueHandle,
        }
    }
}
