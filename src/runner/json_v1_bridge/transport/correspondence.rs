//! Cross-surface consistency of data. This never proves source authorization.
use super::{shape::*, Definitions};
use crate::mir::{MirFunction, MirType, ValueId};
use serde_json::Value;

pub(super) fn validate(
    value: &Value,
    function: &MirFunction,
    definitions: &Definitions<'_>,
) -> Result<(), String> {
    // Direct producer kinds are explicit wire data, not a source-type solver.
    let producer_types = function
        .blocks
        .values()
        .flat_map(|block| &block.instructions)
        .filter_map(|inst| {
            use crate::mir::{ConstValue, MirInstruction as I};
            let ty = match inst {
                I::Const {
                    value: ConstValue::Integer(_),
                    ..
                }
                | I::InvokeNormalResult { .. } => MirType::Integer,
                I::Const {
                    value: ConstValue::Bool(_),
                    ..
                }
                | I::Compare { .. }
                | I::Call(_) => MirType::Bool,
                I::Const {
                    value: ConstValue::Void,
                    ..
                } => MirType::Void,
                _ => return None,
            };
            inst.dst_value().map(|dst| (dst, ty))
        })
        .collect::<std::collections::BTreeMap<_, _>>();
    let metadata = &value["metadata"];
    let contract = &metadata["canonical_transport_v1"];
    if contract["declared_params"] != metadata["declared_param_decls"] {
        return Err(error("declared-parameter-drift"));
    }
    for (param, expected) in function.params.iter().zip(&function.signature.params) {
        if let Some(actual) = function.metadata.value_types.get(param) {
            if *actual != MirType::Unknown && *expected != MirType::Unknown && actual != expected {
                return Err(error("parameter-type-drift"));
            }
        }
    }
    for block in array(value, "blocks")? {
        for row in array(block, "instructions")? {
            let producer_type = match row["op"].as_str() {
                Some("const") => Some(ty(&row["value"]["type"])?),
                Some("compare") => Some(MirType::Bool),
                Some("invoke_normal_result") => Some(MirType::Integer),
                Some("canonical_call") => Some(MirType::Bool),
                _ => None,
            };
            if let Some(expected) = producer_type {
                if let Some(actual) = function.metadata.value_types.get(&ValueId::new(id(
                    if row["op"] == "canonical_call" {
                        &row["call"]
                    } else {
                        row
                    },
                    "dst",
                )?)) {
                    if *actual != MirType::Unknown && *actual != expected {
                        return Err(error("producer-type-drift"));
                    }
                }
            }
            if let Some(hint) = row.get("dst_type") {
                let dst = ValueId::new(id(row, "dst")?);
                if function.metadata.value_types.get(&dst) != Some(&ty(hint)?) {
                    return Err(error("destination-type-drift"));
                }
            }
            let call = match row["op"].as_str() {
                Some("invoke") => &row["operation"]["call"],
                Some("canonical_call") => &row["call"],
                _ => continue,
            };
            let target = definitions
                .get(&key(&call["target"])?)
                .ok_or_else(|| error("foreign-invoke-target"))?;
            let target_contract = &target["metadata"]["canonical_transport_v1"];
            let types = array(&target_contract["signature"], "params")?;
            let args = array(call, "args")?;
            if args.len() != types.len() {
                return Err(error("invoke-signature-arity"));
            }
            let carriers = target_contract["physical_param_carriers"].as_array();
            if carriers.is_some_and(|rows| rows.len() != types.len()) {
                return Err(error("carrier-arity"));
            }
            for (ordinal, (arg, expected)) in args.iter().zip(types).enumerate() {
                let value = ValueId::new(id_value(arg)?);
                let Some(actual) = producer_types
                    .get(&value)
                    .or_else(|| function.metadata.value_types.get(&value))
                else {
                    // No fact is supplied; no typed use is certified.
                    continue;
                };
                let expected = ty(expected)?;
                let carrier = carriers
                    .and_then(|rows| rows.get(ordinal))
                    .and_then(Value::as_str);
                if carrier == Some("borrowed_tagged_value") {
                    if !matches!(actual, MirType::Integer | MirType::Bool | MirType::Unknown) {
                        return Err(error("invoke-argument-carrier-drift"));
                    }
                } else if matches!(carrier, Some("existing_callable_i64" | "u64_bits_on_i64"))
                    && !matches!(actual, MirType::Integer | MirType::Unknown)
                {
                    return Err(error("invoke-argument-carrier-drift"));
                } else if *actual != MirType::Unknown
                    && expected != MirType::Unknown
                    && *actual != expected
                {
                    return Err(error("invoke-argument-carrier-drift"));
                }
            }
        }
    }
    Ok(())
}

// Return environment is block metadata, not an ordinary Return operand. Reuse
// the same CFG dominator/definition utilities without fabricating MIR uses.
pub(super) fn validate_return_env(contract: &Value, function: &MirFunction) -> Result<(), String> {
    use crate::mir::verification::utils;
    let definitions = utils::compute_def_blocks(function);
    let dominators = utils::compute_dominators(function);
    let frames = function
        .blocks
        .values()
        .flat_map(|block| &block.instructions)
        .filter_map(|inst| {
            if let crate::mir::MirInstruction::FaultFrameEnter { dst, .. } = inst {
                Some(*dst)
            } else {
                None
            }
        })
        .collect::<std::collections::BTreeSet<_>>();
    for row in array(contract, "block_spans")? {
        let block = crate::mir::BasicBlockId::new(id(row, "block")?);
        if !row["return_env_layout"].is_null() {
            layout(&row["return_env_layout"])?;
        }
        if row["return_env"].is_null() {
            continue;
        }
        for value in array(row, "return_env")? {
            let value = ValueId::new(id_value(value)?);
            let defining_block = definitions
                .get(&value)
                .ok_or_else(|| error("return-env-definition"))?;
            if frames.contains(&value)
                || value.as_u32() >= function.next_value_id
                || !dominators.dominates(*defining_block, block)
            {
                return Err(error("return-env-scope"));
            }
            if function.blocks[&block]
                .terminator
                .as_ref()
                .and_then(|inst| inst.dst_value())
                == Some(value)
            {
                return Err(error("return-env-before-definition"));
            }
        }
    }
    Ok(())
}
