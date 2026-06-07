use crate::mir::function::FunctionMetadata;
use crate::mir::{BinaryOp, CompareOp};
use serde_json::json;

pub(super) fn insert_exact_numeric_metadata_json(
    obj: &mut serde_json::Map<String, serde_json::Value>,
    metadata: &FunctionMetadata,
) {
    obj.insert(
        "exact_numeric_binary_op_routes".to_string(),
        serde_json::Value::Array(build_exact_numeric_binary_op_routes_json(metadata)),
    );
    obj.insert(
        "exact_numeric_compare_routes".to_string(),
        serde_json::Value::Array(build_exact_numeric_compare_routes_json(metadata)),
    );
    obj.insert(
        "exact_numeric_shift_routes".to_string(),
        serde_json::Value::Array(build_exact_numeric_shift_routes_json(metadata)),
    );
}

fn build_exact_numeric_binary_op_routes_json(
    metadata: &FunctionMetadata,
) -> Vec<serde_json::Value> {
    metadata
        .exact_numeric_binary_op_route_facts
        .iter()
        .map(|route| {
            json!({
                "block": route.block.as_u32(),
                "instruction_index": route.instruction_index,
                "dst": route.dst.as_u32(),
                "operation": binary_op_route_symbol(route.op),
                "lhs": route.lhs.as_u32(),
                "rhs": route.rhs.as_u32(),
                "declared_type": route.declared_type_name,
            })
        })
        .collect()
}

fn build_exact_numeric_compare_routes_json(metadata: &FunctionMetadata) -> Vec<serde_json::Value> {
    metadata
        .exact_numeric_compare_route_facts
        .iter()
        .map(|route| {
            json!({
                "block": route.block.as_u32(),
                "instruction_index": route.instruction_index,
                "dst": route.dst.as_u32(),
                "operation": compare_route_symbol(route.op),
                "lhs": route.lhs.as_u32(),
                "rhs": route.rhs.as_u32(),
                "declared_type": route.declared_type_name,
            })
        })
        .collect()
}

fn build_exact_numeric_shift_routes_json(metadata: &FunctionMetadata) -> Vec<serde_json::Value> {
    metadata
        .exact_numeric_shift_route_facts
        .iter()
        .map(|route| {
            json!({
                "block": route.block.as_u32(),
                "instruction_index": route.instruction_index,
                "dst": route.dst.as_u32(),
                "operation": binary_op_route_symbol(route.op),
                "lhs": route.lhs.as_u32(),
                "rhs": route.rhs.as_u32(),
                "declared_type": route.declared_type_name,
            })
        })
        .collect()
}

fn binary_op_route_symbol(op: BinaryOp) -> &'static str {
    match op {
        BinaryOp::Add => "+",
        BinaryOp::Sub => "-",
        BinaryOp::Mul => "*",
        BinaryOp::Div => "/",
        BinaryOp::Mod => "%",
        BinaryOp::BitAnd | BinaryOp::And => "&",
        BinaryOp::BitOr | BinaryOp::Or => "|",
        BinaryOp::BitXor => "^",
        BinaryOp::Shl => "<<",
        BinaryOp::Shr => ">>",
    }
}

fn compare_route_symbol(op: CompareOp) -> &'static str {
    match op {
        CompareOp::Ge => ">=",
        CompareOp::Le => "<=",
        CompareOp::Gt => ">",
        CompareOp::Lt => "<",
        CompareOp::Eq => "==",
        CompareOp::Ne => "!=",
    }
}
