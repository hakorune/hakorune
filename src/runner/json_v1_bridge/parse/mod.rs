use crate::mir::{
    function::{FunctionSignature, MirFunction, MirModule},
    BasicBlock, BasicBlockId, EffectMask, MirType,
};
use serde_json::Value;

mod instruction;
mod mir_call;
#[cfg(test)]
mod tests;

fn infer_param_count_from_v1_func(func: &Value, func_name: &str) -> Result<usize, String> {
    if let Some(params) = func.get("params").and_then(Value::as_array) {
        for (idx, p) in params.iter().enumerate() {
            let pid = p
                .as_u64()
                .or_else(|| p.get("id").and_then(Value::as_u64))
                .ok_or_else(|| {
                    format!(
                        "function '{}' params[{}] must be integer id (or object with id)",
                        func_name, idx
                    )
                })?;
            if pid != idx as u64 {
                return Err(format!(
                    "[freeze:contract][json_v1_bridge/params] function '{}' params must be canonical [0..N-1]; got id {} at index {}",
                    func_name, pid, idx
                ));
            }
        }
        return Ok(params.len());
    }
    if let Some((_box_name, _method, arity)) = crate::mir::naming::decode_static_method(func_name) {
        return Ok(arity);
    }
    Ok(0)
}

/// Try to parse MIR JSON v1 schema into a MIR module.
/// Returns Ok(None) when the input is not v1 (schema_version missing).
/// Currently supports a minimal subset required for Gate-C parity tests:
/// - const (integer)
/// - copy
/// - ret
pub fn try_parse_v1_to_module(json: &str) -> Result<Option<MirModule>, String> {
    let value: Value = match serde_json::from_str(json) {
        Ok(v) => v,
        Err(e) => return Err(format!("invalid JSON: {}", e)),
    };

    let schema = match value.get("schema_version") {
        Some(Value::String(s)) => s.clone(),
        Some(other) => return Err(format!("expected schema_version string, found {}", other)),
        None => return Ok(None),
    };

    if !schema.starts_with('1') {
        return Err(format!(
            "unsupported schema_version '{}': expected 1.x",
            schema
        ));
    }

    let functions = value
        .get("functions")
        .and_then(|f| f.as_array())
        .ok_or_else(|| "v1 JSON missing functions array".to_string())?;

    let mut module = MirModule::new("ny_json_v1".to_string());

    for func in functions {
        let func_name = func
            .get("name")
            .and_then(|n| n.as_str())
            .unwrap_or("main")
            .to_string();

        let blocks = func
            .get("blocks")
            .and_then(|b| b.as_array())
            .ok_or_else(|| format!("function '{}' missing blocks array", func_name))?;

        if blocks.is_empty() {
            return Err(format!("function '{}' has no blocks", func_name));
        }

        let entry_id = blocks
            .get(0)
            .and_then(|b| b.get("id"))
            .and_then(|id| id.as_u64())
            .ok_or_else(|| format!("function '{}' entry block missing id", func_name))?;
        let entry_bb = BasicBlockId::new(entry_id as u32);

        let inferred_param_count = infer_param_count_from_v1_func(func, &func_name)?;
        let mut signature = FunctionSignature {
            name: func_name.clone(),
            params: vec![MirType::Unknown; inferred_param_count],
            return_type: MirType::Unknown,
            effects: EffectMask::PURE,
        };
        let mut mir_fn = MirFunction::new(signature.clone(), entry_bb);
        let mut max_value_id: u32 = inferred_param_count as u32;

        for block in blocks {
            let block_id = block
                .get("id")
                .and_then(|id| id.as_u64())
                .ok_or_else(|| format!("function '{}' block missing id", func_name))?
                as u32;
            let bb_id = BasicBlockId::new(block_id);
            if mir_fn.get_block(bb_id).is_none() {
                mir_fn.add_block(BasicBlock::new(bb_id));
            }
            let block_ref = mir_fn
                .get_block_mut(bb_id)
                .expect("block must exist after insertion");

            let instructions = block
                .get("instructions")
                .and_then(|insts| insts.as_array())
                .ok_or_else(|| {
                    format!(
                        "function '{}' block {} missing instructions array",
                        func_name, block_id
                    )
                })?;

            for inst in instructions {
                instruction::apply_v1_instruction(
                    inst,
                    &func_name,
                    block_ref,
                    &mut signature,
                    &mut max_value_id,
                )?;
            }
        }
        mir_fn.signature = signature;
        mir_fn.next_value_id = max_value_id.max(mir_fn.next_value_id);
        module.add_function(mir_fn);
    }

    Ok(Some(module))
}
