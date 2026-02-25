use std::collections::HashMap;

use crate::backend::mir_interpreter::MirInterpreter;
use crate::mir::join_ir::{ConstValue, JoinFuncId, JoinInst, JoinModule, MirLikeInst, VarId};

use super::{JoinRuntimeError, JoinValue};

pub(super) fn execute_function(
    vm: &mut MirInterpreter,
    module: &JoinModule,
    mut current_func: JoinFuncId,
    mut current_args: Vec<JoinValue>,
) -> Result<JoinValue, JoinRuntimeError> {
    let verbose = crate::config::env::joinir_dev_enabled();

    'exec: loop {
        let func = module.functions.get(&current_func).ok_or_else(|| {
            JoinRuntimeError::new(format!("Function {:?} not found", current_func))
        })?;

        if func.params.len() != current_args.len() {
            return Err(JoinRuntimeError::new(format!(
                "Arity mismatch for {:?}: expected {}, got {}",
                func.id,
                func.params.len(),
                current_args.len()
            )));
        }

        let mut locals: HashMap<VarId, JoinValue> = HashMap::new();
        for (param, arg) in func.params.iter().zip(current_args.iter()) {
            locals.insert(*param, arg.clone());
        }

        let mut ip = 0usize;
        while ip < func.body.len() {
            match &func.body[ip] {
                JoinInst::Compute(inst) => {
                    eval_compute(vm, inst, &mut locals)?;
                    ip += 1;
                }
                JoinInst::Call {
                    func: target,
                    args,
                    k_next,
                    dst,
                } => {
                    if k_next.is_some() {
                        return Err(JoinRuntimeError::new(
                            "Join continuation (k_next) is not supported in the experimental runner",
                        ));
                    }
                    let resolved_args = materialize_args(args, &locals)?;
                    if let Some(dst_var) = dst {
                        let value = execute_function(vm, module, *target, resolved_args)?;
                        locals.insert(*dst_var, value);
                        ip += 1;
                    } else {
                        current_func = *target;
                        current_args = resolved_args;
                        continue 'exec;
                    }
                }
                JoinInst::Jump {
                    cont: _,
                    args,
                    cond,
                } => {
                    let should_jump = match cond {
                        Some(var) => as_bool(&read_var(&locals, *var)?)?,
                        None => true,
                    };
                    if should_jump {
                        let ret = if let Some(first) = args.first() {
                            read_var(&locals, *first)?
                        } else {
                            JoinValue::Unit
                        };
                        return Ok(ret);
                    }
                    ip += 1;
                }
                JoinInst::Ret { value } => {
                    let ret = match value {
                        Some(var) => read_var(&locals, *var)?,
                        None => JoinValue::Unit,
                    };
                    return Ok(ret);
                }
                // Phase 33: Select instruction execution
                JoinInst::Select {
                    dst,
                    cond,
                    then_val,
                    else_val,
                    type_hint: _, // Phase 63-3: 実行時は未使用
                } => {
                    // 1. Evaluate cond (Bool or Int)
                    let cond_value = read_var(&locals, *cond)?;
                    if verbose {
                        crate::runtime::get_global_ring0().log.debug(&format!(
                            "[joinir/runner/select] cond={:?}, cond_value={:?}",
                            cond, cond_value
                        ));
                    }
                    let cond_bool = match cond_value {
                        JoinValue::Bool(b) => b,
                        JoinValue::Int(i) => i != 0, // Int も許す（0=false, それ以外=true）
                        _ => {
                            return Err(JoinRuntimeError::new(format!(
                                "Select: cond must be Bool or Int, got {:?}",
                                cond_value
                            )))
                        }
                    };

                    // 2. Select then_val or else_val
                    let then_value = read_var(&locals, *then_val)?;
                    let else_value = read_var(&locals, *else_val)?;
                    if verbose {
                        crate::runtime::get_global_ring0().log.debug(&format!(
                            "[joinir/runner/select] cond_bool={}, then_val={:?}={:?}, else_val={:?}={:?}",
                            cond_bool, then_val, then_value, else_val, else_value
                        ));
                    }

                    let selected_id = if cond_bool { *then_val } else { *else_val };
                    let selected_value = read_var(&locals, selected_id)?;
                    if verbose {
                        crate::runtime::get_global_ring0().log.debug(&format!(
                            "[joinir/runner/select] selected_id={:?}, selected_value={:?}",
                            selected_id, selected_value
                        ));
                    }

                    // 3. Write to dst
                    locals.insert(*dst, selected_value);
                    ip += 1;
                }
                // Phase 33-6: IfMerge instruction execution (複数変数 PHI)
                JoinInst::IfMerge {
                    cond,
                    merges,
                    k_next,
                } => {
                    // Phase 33-6 最小実装: k_next は None のみサポート
                    if k_next.is_some() {
                        return Err(JoinRuntimeError::new(
                            "IfMerge: k_next continuation is not yet supported (Phase 33-6 minimal)",
                        ));
                    }

                    // 1. Evaluate cond (Bool or Int)
                    let cond_value = read_var(&locals, *cond)?;
                    let cond_bool = match cond_value {
                        JoinValue::Bool(b) => b,
                        JoinValue::Int(i) => i != 0,
                        _ => {
                            return Err(JoinRuntimeError::new(format!(
                                "IfMerge: cond must be Bool or Int, got {:?}",
                                cond_value
                            )))
                        }
                    };

                    // 2. 各 merge ペアについて、cond に応じて値を選択して代入
                    for merge in merges {
                        let selected_id = if cond_bool {
                            merge.then_val
                        } else {
                            merge.else_val
                        };
                        let selected_value = read_var(&locals, selected_id)?;
                        locals.insert(merge.dst, selected_value);
                    }

                    ip += 1;
                }
                // Phase 34-6: MethodCall instruction execution
                JoinInst::MethodCall { .. } => {
                    // Phase 34-6: MethodCall は JoinIR Runner では未対応
                    // JoinIR → MIR 変換経由で VM が実行する
                    return Err(JoinRuntimeError::new(
                        "MethodCall is not supported in JoinIR Runner (use JoinIR→MIR→VM bridge instead)",
                    ));
                }
                // Phase 56: ConditionalMethodCall instruction execution
                JoinInst::ConditionalMethodCall { .. } => {
                    // Phase 56: ConditionalMethodCall は JoinIR Runner では未対応
                    // JoinIR → MIR 変換経由で VM が実行する
                    return Err(JoinRuntimeError::new(
                        "ConditionalMethodCall is not supported in JoinIR Runner (use JoinIR→MIR→VM bridge instead)",
                    ));
                }
                // Phase 41-4: NestedIfMerge instruction execution
                JoinInst::NestedIfMerge { .. } => {
                    // Phase 41-4: NestedIfMerge は JoinIR Runner では未対応
                    // JoinIR → MIR 変換経由で VM が実行する
                    return Err(JoinRuntimeError::new(
                        "NestedIfMerge is not supported in JoinIR Runner (use JoinIR→MIR→VM bridge instead)",
                    ));
                }
                // Phase 51: FieldAccess instruction execution
                JoinInst::FieldAccess { .. } => {
                    // Phase 51: FieldAccess は JoinIR Runner では未対応
                    // JoinIR → MIR 変換経由で VM が実行する
                    return Err(JoinRuntimeError::new(
                        "FieldAccess is not supported in JoinIR Runner (use JoinIR→MIR→VM bridge instead)",
                    ));
                }
                // Phase 51: NewBox instruction execution
                JoinInst::NewBox { .. } => {
                    // Phase 51: NewBox は JoinIR Runner では未対応
                    // JoinIR → MIR 変換経由で VM が実行する
                    return Err(JoinRuntimeError::new(
                        "NewBox is not supported in JoinIR Runner (use JoinIR→MIR→VM bridge instead)",
                    ));
                }
            }
        }

        // fallthrough without explicit return
        return Ok(JoinValue::Unit);
    }
}

fn eval_compute(
    vm: &mut MirInterpreter,
    inst: &MirLikeInst,
    locals: &mut HashMap<VarId, JoinValue>,
) -> Result<(), JoinRuntimeError> {
    match inst {
        MirLikeInst::Const { dst, value } => {
            let v = match value {
                ConstValue::Integer(i) => JoinValue::Int(*i),
                ConstValue::Bool(b) => JoinValue::Bool(*b),
                ConstValue::String(s) => JoinValue::Str(s.clone()),
                ConstValue::Null => JoinValue::Unit,
            };
            locals.insert(*dst, v);
        }
        MirLikeInst::BinOp { dst, op, lhs, rhs } => {
            // Phase 27.8: ops box の eval_binop() を使用
            let l = read_var(locals, *lhs)?;
            let r = read_var(locals, *rhs)?;
            let v = crate::mir::join_ir_ops::eval_binop(*op, &l, &r)?;
            locals.insert(*dst, v);
        }
        MirLikeInst::Compare { dst, op, lhs, rhs } => {
            // Phase 27.8: ops box の eval_compare() を使用
            let l = read_var(locals, *lhs)?;
            let r = read_var(locals, *rhs)?;
            let v = crate::mir::join_ir_ops::eval_compare(*op, &l, &r)?;
            locals.insert(*dst, v);
        }
        // S-5.2-improved: BoxCall → VM execute_box_call ラッパー経由
        // - 制御フロー: JoinIR Runner が担当
        // - Box/Plugin 実装: Rust VM に完全委譲（VM 2号機を避ける）
        // - VM の完全な BoxCall 意味論を使用:
        //   * Void guards (Void.length() → 0)
        //   * PluginBox サポート (FileBox, NetBox)
        //   * InstanceBox policy checks
        //   * object_fields handling
        //   * Method re-routing (toString→str)
        MirLikeInst::BoxCall {
            dst,
            box_name: _, // box_name は VM が内部で判定するため不要
            method,
            args,
        } => {
            // First argument is the receiver (box instance)
            if args.is_empty() {
                return Err(JoinRuntimeError::new(
                    "BoxCall requires at least a receiver argument",
                ));
            }

            // Convert receiver to VMValue
            let receiver_jv = read_var(locals, args[0])?;
            let receiver_vm = receiver_jv.to_vm_value();

            // Convert remaining arguments to VMValue
            let method_args_vm: Vec<crate::backend::VMValue> = args[1..]
                .iter()
                .map(|&var_id| read_var(locals, var_id).map(|jv| jv.to_vm_value()))
                .collect::<Result<Vec<_>, _>>()?;

            // Invoke VM's execute_box_call for complete semantics
            let result_vm = vm
                .execute_box_call(receiver_vm, method, method_args_vm)
                .map_err(|e| JoinRuntimeError::new(format!("BoxCall failed: {}", e)))?;

            // Convert result back to JoinValue
            let result_jv = crate::mir::join_ir_ops::JoinValue::from_vm_value(&result_vm)?;

            // Store result if destination is specified
            if let Some(dst_var) = dst {
                locals.insert(*dst_var, result_jv);
            }
        }
        // Phase 56: UnaryOp
        MirLikeInst::UnaryOp { dst, op, operand } => {
            let operand_val = read_var(locals, *operand)?;
            let result = match op {
                crate::mir::join_ir::UnaryOp::Not => match operand_val {
                    JoinValue::Bool(b) => JoinValue::Bool(!b),
                    JoinValue::Int(i) => JoinValue::Bool(i == 0),
                    _ => {
                        return Err(JoinRuntimeError::new(format!(
                            "Cannot apply 'not' to {:?}",
                            operand_val
                        )))
                    }
                },
                crate::mir::join_ir::UnaryOp::Neg => match operand_val {
                    JoinValue::Int(i) => JoinValue::Int(-i),
                    _ => {
                        return Err(JoinRuntimeError::new(format!(
                            "Cannot apply '-' to {:?}",
                            operand_val
                        )))
                    }
                },
            };
            locals.insert(*dst, result);
        }
        // Phase 188: Print
        MirLikeInst::Print { value } => {
            let val = read_var(locals, *value)?;
            // Print to stdout (convert to string representation)
            let output = match val {
                JoinValue::Int(i) => i.to_string(),
                JoinValue::Bool(b) => b.to_string(),
                JoinValue::Str(s) => s,
                JoinValue::Unit => "null".to_string(),
                JoinValue::BoxRef(_) => "[BoxRef]".to_string(),
            };
            println!("{}", output);
        }
        // Phase 188-Impl-3: Select
        MirLikeInst::Select {
            dst,
            cond,
            then_val,
            else_val,
        } => {
            let cond_value = read_var(locals, *cond)?;
            let is_true = match cond_value {
                JoinValue::Bool(b) => b,
                JoinValue::Int(i) => i != 0,
                _ => {
                    return Err(JoinRuntimeError::new(format!(
                        "Select condition must be Bool or Int, got {:?}",
                        cond_value
                    )))
                }
            };
            let result = if is_true {
                read_var(locals, *then_val)?
            } else {
                read_var(locals, *else_val)?
            };
            locals.insert(*dst, result);
        }
    }
    Ok(())
}

fn read_var(locals: &HashMap<VarId, JoinValue>, var: VarId) -> Result<JoinValue, JoinRuntimeError> {
    locals
        .get(&var)
        .cloned()
        .ok_or_else(|| JoinRuntimeError::new(format!("Variable {:?} not bound", var)))
}

fn materialize_args(
    args: &[VarId],
    locals: &HashMap<VarId, JoinValue>,
) -> Result<Vec<JoinValue>, JoinRuntimeError> {
    args.iter().map(|v| read_var(locals, *v)).collect()
}

fn as_bool(value: &JoinValue) -> Result<bool, JoinRuntimeError> {
    match value {
        JoinValue::Bool(b) => Ok(*b),
        JoinValue::Int(i) => Ok(*i != 0),
        JoinValue::Unit => Ok(false),
        other => Err(JoinRuntimeError::new(format!(
            "Expected bool-compatible value, got {:?}",
            other
        ))),
    }
}
