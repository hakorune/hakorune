use super::super::helpers::{parse_binop, parse_compare, require_u64};
use super::mir_call::parse_v1_mir_call;
use crate::mir::{BasicBlock, BasicBlockId, FunctionSignature, MirInstruction, MirType, ValueId};
use crate::runner::mir_json::common as mirjson_common;
use serde_json::Value;

pub(super) fn apply_v1_instruction(
    inst: &Value,
    func_name: &str,
    block_ref: &mut BasicBlock,
    signature: &mut FunctionSignature,
    max_value_id: &mut u32,
) -> Result<(), String> {
    let op = inst.get("op").and_then(|o| o.as_str()).ok_or_else(|| {
        format!(
            "function '{}' block {} missing op field",
            func_name,
            block_ref.id.as_u32()
        )
    })?;

    match op {
        "const" => {
            let dst = inst.get("dst").and_then(|d| d.as_u64()).ok_or_else(|| {
                format!("const instruction missing dst in function '{}'", func_name)
            })? as u32;
            let value_obj = inst.get("value").ok_or_else(|| {
                format!(
                    "const instruction missing value in function '{}'",
                    func_name
                )
            })?;
            let const_val = mirjson_common::parse_const_value_generic(value_obj)?;
            block_ref.add_instruction(MirInstruction::Const {
                dst: ValueId::new(dst),
                value: const_val,
            });
            if dst >= *max_value_id {
                *max_value_id = dst + 1;
            }
        }
        "copy" => {
            let dst = inst.get("dst").and_then(|d| d.as_u64()).ok_or_else(|| {
                format!("copy instruction missing dst in function '{}'", func_name)
            })? as u32;
            let src = inst.get("src").and_then(|d| d.as_u64()).ok_or_else(|| {
                format!("copy instruction missing src in function '{}'", func_name)
            })? as u32;
            block_ref.add_instruction(MirInstruction::Copy {
                dst: ValueId::new(dst),
                src: ValueId::new(src),
            });
            *max_value_id = (*max_value_id).max(dst + 1).max(src + 1);
        }
        "newbox" => {
            let dst = require_u64(inst, "dst", "newbox dst")? as u32;
            let box_type = inst
                .get("type")
                .and_then(Value::as_str)
                .ok_or_else(|| format!("newbox missing type in function '{}'", func_name))?
                .to_string();
            let mut args: Vec<ValueId> = Vec::new();
            if let Some(arr) = inst.get("args").and_then(Value::as_array) {
                for arg in arr {
                    let id = arg.as_u64().ok_or_else(|| {
                        format!(
                            "newbox arg must be integer value id in function '{}'",
                            func_name
                        )
                    })? as u32;
                    args.push(ValueId::new(id));
                }
            }
            if let Some(arg_max) = args.iter().map(|v| v.as_u32()).max() {
                *max_value_id = (*max_value_id).max(arg_max + 1);
            }
            block_ref.add_instruction(MirInstruction::NewBox {
                dst: ValueId::new(dst),
                box_type,
                args,
            });
            *max_value_id = (*max_value_id).max(dst + 1);
        }
        "field_get" => {
            let dst = require_u64(inst, "dst", "field_get dst")? as u32;
            let base = require_u64(inst, "box", "field_get box")? as u32;
            let field = inst
                .get("field")
                .and_then(Value::as_str)
                .ok_or_else(|| format!("field_get missing field in function '{}'", func_name))?
                .to_string();
            block_ref.add_instruction(MirInstruction::FieldGet {
                dst: ValueId::new(dst),
                base: ValueId::new(base),
                field,
                declared_type: None,
            });
            *max_value_id = (*max_value_id).max(dst + 1).max(base + 1);
        }
        "binop" => {
            let dst = require_u64(inst, "dst", "binop dst")? as u32;
            let lhs = require_u64(inst, "lhs", "binop lhs")? as u32;
            let rhs = require_u64(inst, "rhs", "binop rhs")? as u32;
            let operation = inst
                .get("operation")
                .and_then(Value::as_str)
                .ok_or_else(|| format!("binop operation missing in function '{}'", func_name))?;
            let bop = parse_binop(operation)?;
            block_ref.add_instruction(MirInstruction::BinOp {
                dst: ValueId::new(dst),
                op: bop,
                lhs: ValueId::new(lhs),
                rhs: ValueId::new(rhs),
            });
            *max_value_id = (*max_value_id).max(dst + 1).max(lhs + 1).max(rhs + 1);
        }
        "compare" => {
            let dst = require_u64(inst, "dst", "compare dst")? as u32;
            let lhs = require_u64(inst, "lhs", "compare lhs")? as u32;
            let rhs = require_u64(inst, "rhs", "compare rhs")? as u32;
            let op_sym_opt = inst
                .get("operation")
                .and_then(Value::as_str)
                .map(|s| s.to_string());
            let op_sym = if let Some(sym) = op_sym_opt {
                sym
            } else if let Some(name) = inst.get("cmp").and_then(Value::as_str) {
                match name {
                    "Lt" => "<".to_string(),
                    "Le" => "<=".to_string(),
                    "Gt" => ">".to_string(),
                    "Ge" => ">=".to_string(),
                    "Eq" => "==".to_string(),
                    "Ne" => "!=".to_string(),
                    other => {
                        return Err(format!(
                            "unsupported compare cmp '{}' in Gate-C v1 bridge (function '{}')",
                            other, func_name
                        ));
                    }
                }
            } else {
                return Err(format!(
                    "compare operation missing in function '{}'",
                    func_name
                ));
            };
            let cop = parse_compare(&op_sym)?;
            block_ref.add_instruction(MirInstruction::Compare {
                dst: ValueId::new(dst),
                op: cop,
                lhs: ValueId::new(lhs),
                rhs: ValueId::new(rhs),
            });
            *max_value_id = (*max_value_id).max(dst + 1).max(lhs + 1).max(rhs + 1);
        }
        "branch" => {
            let cond = require_u64(inst, "cond", "branch cond")? as u32;
            let then_bb = require_u64(inst, "then", "branch then")? as u32;
            let else_bb = require_u64(inst, "else", "branch else")? as u32;
            block_ref.add_instruction(MirInstruction::Branch {
                condition: ValueId::new(cond),
                then_bb: BasicBlockId::new(then_bb),
                else_bb: BasicBlockId::new(else_bb),
                then_edge_args: None,
                else_edge_args: None,
            });
            *max_value_id = (*max_value_id).max(cond + 1);
        }
        "jump" => {
            let target = require_u64(inst, "target", "jump target")? as u32;
            block_ref.add_instruction(MirInstruction::Jump {
                target: BasicBlockId::new(target),
                edge_args: None,
            });
        }
        "phi" => {
            let dst = require_u64(inst, "dst", "phi dst")? as u32;
            let pairs = mirjson_common::parse_phi_incoming_generic(inst)
                .map_err(|e| format!("{} in function '{}'", e, func_name))?;
            block_ref.add_instruction(MirInstruction::Phi {
                dst: ValueId::new(dst),
                inputs: pairs,
                type_hint: None,
            });
            let mut phi_max = dst + 1;
            for (_pred, value) in block_ref
                .instructions
                .last()
                .and_then(|i| match i {
                    MirInstruction::Phi { inputs, .. } => Some(inputs.as_slice()),
                    _ => None,
                })
                .unwrap_or(&[])
            {
                phi_max = phi_max.max(value.as_u32() + 1);
            }
            *max_value_id = (*max_value_id).max(phi_max);
        }
        "ret" => {
            let value = inst
                .get("value")
                .and_then(|v| v.as_u64())
                .map(|v| ValueId::new(v as u32));
            block_ref.add_instruction(MirInstruction::Return { value });
            if let Some(val) = value {
                signature.return_type = MirType::Integer;
                *max_value_id = (*max_value_id).max(val.as_u32() + 1);
            } else {
                signature.return_type = MirType::Void;
            }
        }
        "mir_call" => {
            parse_v1_mir_call(inst, func_name, block_ref, max_value_id)?;
        }
        other => {
            return Err(format!(
                "unsupported instruction '{}' in function '{}' (Gate-C v1 bridge)",
                other, func_name
            ));
        }
    }

    Ok(())
}
