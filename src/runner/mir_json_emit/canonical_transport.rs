//! Lossless data projection of the selected fallible Static-I64 document.
//! Neither this projection nor its reader publishes source authority.
use crate::mir::{MirFunction, MirInstruction, MirModule, MirType};
use hakorune_mir_defs::{CanonicalSameModuleCallableKeyV1, SameModuleCallableNamespaceV1};
use serde_json::{json, Value};

pub(super) fn selected(module: &MirModule) -> bool {
    module.functions.values().any(|function| {
        function.blocks.values().any(|block| {
            block
                .instructions
                .iter()
                .chain(block.terminator.iter())
                .any(|inst| {
                    matches!(
                        inst,
                        MirInstruction::Invoke { .. }
                            | MirInstruction::InvokeNormalResult { .. }
                            | MirInstruction::ReturnFault { .. }
                    )
                })
        })
    })
}

pub(super) fn key(key: &CanonicalSameModuleCallableKeyV1) -> Value {
    json!({"namespace": match key.namespace() {
        SameModuleCallableNamespaceV1::FreeFunction => "free_function",
        SameModuleCallableNamespaceV1::StaticBoxMethod => "static_box_method",
        SameModuleCallableNamespaceV1::InstanceBoxMethod => "instance_box_method",
        SameModuleCallableNamespaceV1::BirthConstructor => "birth_constructor",
    }, "owner": key.owner(), "name": key.name(), "arity": key.arity()})
}

pub(super) fn module(module: &MirModule) -> Value {
    json!({"version": 1, "module_name": module.name,
        "callable_definitions": module.canonical_callable_definitions.iter().map(|(id, symbol)| {
            json!({"key": key(id), "symbol": symbol})
        }).collect::<Vec<_>>()})
}

pub(super) fn validate_source(function: &MirFunction) -> Result<(), String> {
    // Generic MIR JSON has a narrow call transport. Reject lifecycle invoke
    // operations at that boundary before unrelated physical cleanup/type
    // rows obscure the designed refusal.
    if function.blocks.values().any(|block| {
        block.instructions.iter().chain(block.terminator.iter()).any(|instruction| {
            matches!(instruction, MirInstruction::Invoke { operation, .. }
                if !matches!(operation, crate::mir::instruction::InvokeOperation::Call {
                    result: crate::mir::instruction::InvokeCallResultKind::I64, ..
                }))
        })
    }) {
        return Err("unsupported terminator Invoke".into());
    }
    for block in function.blocks.values() {
        for instruction in block.instructions.iter().chain(block.terminator.iter()) {
            match instruction {
                MirInstruction::Const {
                    value:
                        crate::mir::ConstValue::Integer(_)
                        | crate::mir::ConstValue::Bool(_)
                        | crate::mir::ConstValue::Void,
                    ..
                }
                | MirInstruction::Call(_)
                | MirInstruction::Copy { .. }
                | MirInstruction::Compare { .. }
                | MirInstruction::Return { .. }
                | MirInstruction::ReturnFault { .. }
                | MirInstruction::Branch { .. }
                | MirInstruction::Jump { .. }
                | MirInstruction::Invoke { .. }
                | MirInstruction::InvokeNormalResult { .. }
                | MirInstruction::FaultFrameEnter { .. } => {}
                MirInstruction::BinOp {
                    op:
                        crate::mir::BinaryOp::Add
                        | crate::mir::BinaryOp::Sub
                        | crate::mir::BinaryOp::Mul
                        | crate::mir::BinaryOp::Div,
                    ..
                } => {}
                MirInstruction::Phi { type_hint, .. } => {
                    if let Some(hint) = type_hint {
                        ty(hint)?;
                    }
                }
                _ => return Err(
                    "[freeze:contract][mir-json/canonical-transport/instruction-outside-profile]"
                        .into(),
                ),
            }
        }
    }
    for value in function.metadata.value_types.values() {
        ty(value)?;
    }
    Ok(())
}

pub(super) fn ty(value: &MirType) -> Result<Value, String> {
    Ok(match value {
        MirType::Integer => json!("i64"),
        MirType::Bool => json!("i1"),
        MirType::Void => json!("void"),
        MirType::Unknown => json!("unknown"),
        _ => {
            return Err(
                "[freeze:contract][mir-json/canonical-transport/type-outside-profile]".into(),
            )
        }
    })
}

fn span(value: crate::ast::Span) -> Value {
    json!({"start": value.start, "end": value.end, "line": value.line, "column": value.column})
}

pub(super) fn function(function: &MirFunction) -> Result<Value, String> {
    let params = function
        .signature
        .params
        .iter()
        .map(ty)
        .collect::<Result<Vec<_>, _>>()?;
    let locals = function
        .locals
        .iter()
        .map(ty)
        .collect::<Result<Vec<_>, _>>()?;
    let mut blocks = function.blocks.values().collect::<Vec<_>>();
    blocks.sort_by_key(|block| block.id);
    let mut spans = Vec::new();
    for block in blocks {
        let mut non_phi = false;
        for inst in &block.instructions {
            if matches!(inst, MirInstruction::Phi { .. }) {
                if non_phi {
                    return Err(
                        "[freeze:contract][mir-json/canonical-transport/nonleading-phi]".into(),
                    );
                }
            } else {
                non_phi = true;
            }
        }
        if block.instructions.len() != block.instruction_spans.len() {
            return Err("[freeze:contract][mir-json/canonical-transport/span-alignment]".into());
        }
        spans.push(json!({"block": block.id.as_u32(),
            "instructions": block.instruction_spans.iter().copied().map(span).collect::<Vec<_>>(),
            "terminator": block.terminator_span.map(span),
            "return_env": block.return_env.as_ref().map(|values| values.iter().map(|v| v.as_u32()).collect::<Vec<_>>()),
            "return_env_layout": block.return_env_layout.map(layout)}));
    }
    Ok(
        json!({"signature": {"name": function.signature.name, "params": params,
            "return_type": ty(&function.signature.return_type)?, "effects_bits": function.signature.effects.bits()},
        "entry_block": function.entry_block.as_u32(), "next_value_id": function.next_value_id,
        "locals": locals, "physical_param_carriers": function.metadata.physical_param_carrier_transport_tags_v1(),
        "source_file": function.metadata.source_file, "line_number": function.metadata.line_number,
        "declared_params": function.metadata.declared_param_decls.iter().map(|param| json!({
            "name": param.name, "declared_type_name": param.declared_type_name,
            "implicit_receiver": param.implicit_receiver})).collect::<Vec<_>>(), "block_spans": spans}),
    )
}

// These are original neutral MIR options, not reconstructed PHI arguments.
pub(super) fn layout(value: crate::mir::edge_args::JumpArgsLayout) -> &'static str {
    match value {
        crate::mir::edge_args::JumpArgsLayout::CarriersOnly => "carriers_only",
        crate::mir::edge_args::JumpArgsLayout::ExprResultPlusCarriers => {
            "expr_result_plus_carriers"
        }
    }
}
pub(super) fn edge(value: &Option<crate::mir::edge_args::EdgeArgs>) -> Value {
    value.as_ref().map(|args| json!({"layout":layout(args.layout),"values":args.values.iter().map(|v|v.as_u32()).collect::<Vec<_>>()})).unwrap_or(Value::Null)
}
