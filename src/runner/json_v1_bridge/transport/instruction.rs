//! Finite instruction parsing; the executable v1 reader stays unchanged.
use super::{shape::*, Definitions};
use crate::mir::instruction::{FaultFrameMode, InvokeCallResultKind, InvokeOperation};
use crate::mir::{BasicBlockId, ConstValue, MirInstruction as I, ValueId};
use serde_json::Value;

pub(super) fn parse(row: &Value, definitions: &Definitions<'_>) -> Result<I, String> {
    let op = string(row, "op")?;
    let dst = || id(row, "dst").map(ValueId::new);
    let operand = |key| id(row, key).map(ValueId::new);
    Ok(match op {
        "const" => {
            fields(row, &["op", "dst", "value"], &[])?;
            fields(&row["value"], &["type", "value"], &[])?;
            let raw = &row["value"]["value"];
            let value = match row["value"]["type"].as_str() {
                Some("i64") => {
                    ConstValue::Integer(raw.as_i64().ok_or_else(|| error("integer-constant"))?)
                }
                Some("i1") => {
                    ConstValue::Bool(raw.as_bool().ok_or_else(|| error("boolean-constant"))?)
                }
                Some("void") if raw == &Value::from(0) => ConstValue::Void,
                _ => return Err(error("constant-outside-profile")),
            };
            I::Const { dst: dst()?, value }
        }
        "copy" => {
            fields(row, &["op", "dst", "src"], &[])?;
            I::Copy {
                dst: dst()?,
                src: operand("src")?,
            }
        }
        "binop" | "compare" => {
            fields(
                row,
                &["op", "operation", "lhs", "rhs", "dst"],
                &["dst_type", "cmp_kind"],
            )?;
            if let Some(hint) = row.get("dst_type") {
                ty(hint)?;
            }
            if row.get("cmp_kind").is_some() {
                return Err(error("compare-kind-outside-profile"));
            }
            let operation = string(row, "operation")?;
            let lhs = operand("lhs")?;
            let rhs = operand("rhs")?;
            if op == "binop" {
                if !matches!(operation, "+" | "-" | "*" | "/") {
                    return Err(error("binary-outside-profile"));
                }
                I::BinOp {
                    dst: dst()?,
                    op: super::super::helpers::parse_binop(operation)?,
                    lhs,
                    rhs,
                }
            } else {
                I::Compare {
                    dst: dst()?,
                    op: super::super::helpers::parse_compare(operation)?,
                    lhs,
                    rhs,
                }
            }
        }
        "phi" => {
            fields(row, &["op", "dst", "incoming", "type_hint"], &["dst_type"])?;
            if let Some(hint) = row.get("dst_type") {
                ty(hint)?;
            }
            let mut inputs = Vec::new();
            for pair in array(row, "incoming")? {
                let pair = pair
                    .as_array()
                    .filter(|pair| pair.len() == 2)
                    .ok_or_else(|| error("phi-pair"))?;
                inputs.push((
                    BasicBlockId::new(id_value(&pair[1])?),
                    ValueId::new(id_value(&pair[0])?),
                ));
            }
            I::Phi {
                dst: dst()?,
                inputs,
                type_hint: if row["type_hint"].is_null() {
                    None
                } else {
                    Some(ty(&row["type_hint"])?)
                },
            }
        }
        "fault_frame_enter" => {
            fields(row, &["op", "dst", "mode"], &[])?;
            let mode = match string(row, "mode")? {
                "root_owned" => FaultFrameMode::RootOwned,
                "borrowed" => FaultFrameMode::Borrowed,
                _ => return Err(error("frame-mode")),
            };
            I::FaultFrameEnter { dst: dst()?, mode }
        }
        "invoke_normal_result" => {
            fields(row, &["op", "invoke_block", "dst"], &[])?;
            I::InvokeNormalResult {
                invoke_block: BasicBlockId::new(id(row, "invoke_block")?),
                dst: dst()?,
            }
        }
        "invoke" => {
            fields(
                row,
                &["op", "operation", "fault_frame", "normal", "fault"],
                &[],
            )?;
            fields(&row["operation"], &["kind", "result", "call"], &[])?;
            if row["operation"]["kind"] != "call" || row["operation"]["result"] != "i64" {
                return Err(error("invoke-outside-profile"));
            }
            let call = parse_call(&row["operation"]["call"], definitions, "i64")?;
            if call.dst.is_some() {
                return Err(error("embedded-result"));
            }
            I::Invoke {
                operation: InvokeOperation::Call {
                    call,
                    result: InvokeCallResultKind::I64,
                },
                fault_frame: operand("fault_frame")?,
                normal_landing: BasicBlockId::new(id(row, "normal")?),
                fault_landing: BasicBlockId::new(id(row, "fault")?),
            }
        }
        "canonical_call" => {
            fields(row, &["op", "call"], &[])?;
            let call = parse_call(&row["call"], definitions, "i1")?;
            if call.dst.is_none() {
                return Err(error("missing-call-result"));
            }
            I::Call(call)
        }
        "return_fault" => {
            fields(row, &["op", "fault_frame"], &[])?;
            I::ReturnFault {
                fault_frame: operand("fault_frame")?,
            }
        }
        "ret" => {
            fields(row, &["op", "value"], &[])?;
            I::Return {
                value: if row["value"].is_null() {
                    None
                } else {
                    Some(operand("value")?)
                },
            }
        }
        "jump" => {
            fields(row, &["op", "target", "edge_args"], &[])?;
            I::Jump {
                target: BasicBlockId::new(id(row, "target")?),
                edge_args: edge(&row["edge_args"])?,
            }
        }
        "branch" => {
            fields(
                row,
                &[
                    "op",
                    "cond",
                    "then",
                    "else",
                    "then_edge_args",
                    "else_edge_args",
                ],
                &[],
            )?;
            I::Branch {
                condition: operand("cond")?,
                then_bb: BasicBlockId::new(id(row, "then")?),
                else_bb: BasicBlockId::new(id(row, "else")?),
                then_edge_args: edge(&row["then_edge_args"])?,
                else_edge_args: edge(&row["else_edge_args"])?,
            }
        }
        _ => return Err(error("instruction-outside-profile")),
    })
}

fn parse_call(
    value: &Value,
    definitions: &Definitions<'_>,
    result: &str,
) -> Result<crate::mir::definitions::MirCall, String> {
    fields(
        value,
        &["dst", "target", "args", "flags", "effects_bits"],
        &[],
    )?;
    let key = key(&value["target"])?;
    if key.0 != "static_box_method" {
        return Err(error("invoke-target-outside-profile"));
    }
    let function = definitions
        .get(&key)
        .ok_or_else(|| error("foreign-invoke-target"))?;
    if function["metadata"]["canonical_transport_v1"]["signature"]["return_type"] != result {
        return Err(error("invoke-result-type"));
    }
    let args = array(value, "args")?
        .iter()
        .map(|v| id_value(v).map(ValueId::new))
        .collect::<Result<Vec<_>, _>>()?;
    if args.len() != key.3 as usize {
        return Err(error("invoke-arity"));
    }
    let target = hakorune_mir_defs::CanonicalGlobalTargetV1::new_static_box_method(
        key.1.into(),
        key.2.into(),
        key.3,
    )
    .map_err(|_| error("canonical-key"))?;
    let dst = if value["dst"].is_null() {
        None
    } else {
        Some(ValueId::new(id(value, "dst")?))
    };
    let mut call = crate::mir::definitions::MirCall::global(dst, target, args);
    let flags = &value["flags"];
    fields(
        flags,
        &["tail_call", "no_return", "can_inline", "is_constructor"],
        &[],
    )?;
    call.flags.tail_call = boolean(flags, "tail_call")?;
    call.flags.no_return = boolean(flags, "no_return")?;
    call.flags.can_inline = boolean(flags, "can_inline")?;
    call.flags.is_constructor = boolean(flags, "is_constructor")?;
    call.effects = effects(value)?;
    Ok(call)
}
