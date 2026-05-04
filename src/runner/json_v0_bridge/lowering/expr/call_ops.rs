use super::super::super::ast::ExprV0;
use super::super::BridgeEnv;
use super::VarScope;
use crate::mir::{BasicBlockId, ConstValue, EffectMask, MirFunction, MirInstruction, ValueId};

pub(super) fn lower_call_expr<S: VarScope>(
    env: &BridgeEnv,
    f: &mut MirFunction,
    cur_bb: BasicBlockId,
    name: &str,
    args: &[ExprV0],
    vars: &mut S,
) -> Result<(ValueId, BasicBlockId), String> {
    if let Some((recv_alias, method)) = split_imported_alias_call(env, name) {
        if let Some(result) = lower_canonical_imported_static_extern_call(
            env, f, cur_bb, recv_alias, method, args, vars,
        )? {
            return Ok(result);
        }
        let recv = ExprV0::Var {
            name: recv_alias.to_string(),
        };
        return lower_method_expr(env, f, cur_bb, &recv, method, args, vars);
    }

    if name == "array.of" {
        let arr = f.next_value_id();
        if let Some(bb) = f.get_block_mut(cur_bb) {
            bb.add_instruction(MirInstruction::NewBox {
                dst: arr,
                box_type: "ArrayBox".into(),
                args: vec![],
            });
        }
        let mut cur = cur_bb;
        for expr in args {
            let (value, next_bb) = super::lower_expr_with_scope(env, f, cur, expr, vars)?;
            cur = next_bb;
            let tmp = f.next_value_id();
            if let Some(bb) = f.get_block_mut(cur) {
                bb.add_instruction(crate::mir::ssot::method_call::runtime_method_call(
                    Some(tmp),
                    arr,
                    "ArrayBox",
                    "push",
                    vec![value],
                    EffectMask::READ,
                    crate::mir::definitions::call_unified::TypeCertainty::Known,
                ));
            }
        }
        return Ok((arr, cur));
    }

    if name == "map.of" {
        let mapv = f.next_value_id();
        if let Some(bb) = f.get_block_mut(cur_bb) {
            bb.add_instruction(MirInstruction::NewBox {
                dst: mapv,
                box_type: "MapBox".into(),
                args: vec![],
            });
        }
        let mut cur = cur_bb;
        let mut iter = args.iter();
        while let Some(key_expr) = iter.next() {
            if let Some(value_expr) = iter.next() {
                let (key_value, after_key) =
                    super::lower_expr_with_scope(env, f, cur, key_expr, vars)?;
                cur = after_key;
                let (map_value, after_value) =
                    super::lower_expr_with_scope(env, f, cur, value_expr, vars)?;
                cur = after_value;
                let tmp = f.next_value_id();
                if let Some(bb) = f.get_block_mut(cur) {
                    bb.add_instruction(crate::mir::ssot::method_call::runtime_method_call(
                        Some(tmp),
                        mapv,
                        "MapBox",
                        "set",
                        vec![key_value, map_value],
                        EffectMask::READ,
                        crate::mir::definitions::call_unified::TypeCertainty::Known,
                    ));
                }
            } else {
                break;
            }
        }
        return Ok((mapv, cur));
    }

    let (arg_ids, cur) = super::lower_args_with_scope(env, f, cur_bb, args, vars)?;
    let fun_value = f.next_value_id();
    if let Some(bb) = f.get_block_mut(cur) {
        bb.add_instruction(MirInstruction::Const {
            dst: fun_value,
            value: ConstValue::String(name.to_string()),
        });
    }
    let dst = f.next_value_id();
    if let Some(bb) = f.get_block_mut(cur) {
        bb.add_instruction(MirInstruction::Call {
            dst: Some(dst),
            func: fun_value,
            callee: None,
            args: arg_ids,
            effects: EffectMask::READ,
        });
    }
    Ok((dst, cur))
}

fn split_imported_alias_call<'a>(env: &'a BridgeEnv, name: &'a str) -> Option<(&'a str, &'a str)> {
    let (recv_alias, method) = name.split_once('.')?;
    if recv_alias.is_empty() || method.is_empty() {
        return None;
    }
    if !env.imports.contains_key(recv_alias) {
        return None;
    }
    Some((recv_alias, method))
}

fn lower_canonical_imported_static_extern_call<S: VarScope>(
    env: &BridgeEnv,
    f: &mut MirFunction,
    cur_bb: BasicBlockId,
    recv_alias: &str,
    method: &str,
    args: &[ExprV0],
    vars: &mut S,
) -> Result<Option<(ValueId, BasicBlockId)>, String> {
    let Some((extern_module, extern_method)) =
        classify_stage1_static_extern_import_call(env, recv_alias, method, args)
    else {
        return Ok(None);
    };

    let (source, cur2) = super::lower_expr_with_scope(env, f, cur_bb, &args[0], vars)?;
    let dst = f.next_value_id();
    if let Some(bb) = f.get_block_mut(cur2) {
        bb.add_instruction(crate::mir::ssot::extern_call::extern_call(
            Some(dst),
            extern_module,
            extern_method,
            vec![source],
            EffectMask::READ,
        ));
    }
    Ok(Some((dst, cur2)))
}

fn classify_stage1_static_extern_import_call<'a>(
    env: &BridgeEnv,
    recv_alias: &str,
    method: &str,
    args: &[ExprV0],
) -> Option<(&'a str, &'a str)> {
    let Some(mapped) = env.imports.get(recv_alias) else {
        return None;
    };
    if matches!(args, [_, ExprV0::Null]) {
        if mapped == "lang.compiler.build.build_box" && method == "emit_program_json_v0" {
            return Some(("nyash.stage1", "emit_program_json_v0_h"));
        }
        if mapped == "lang.mir.builder.MirBuilderBox" && method == "emit_from_source_v0" {
            return Some(("nyash.stage1", "emit_mir_from_source_v0_h"));
        }
    }
    None
}

pub(super) fn lower_method_expr<S: VarScope>(
    env: &BridgeEnv,
    f: &mut MirFunction,
    cur_bb: BasicBlockId,
    recv: &ExprV0,
    method: &str,
    args: &[ExprV0],
    vars: &mut S,
) -> Result<(ValueId, BasicBlockId), String> {
    let recv_is_console_new = matches!(recv, ExprV0::New { class, .. } if class == "ConsoleBox");
    if recv_is_console_new && (method == "println" || method == "print" || method == "log") {
        let (arg_ids, cur2) = super::lower_args_with_scope(env, f, cur_bb, args, vars)?;
        let dst = f.next_value_id();
        if let Some(bb) = f.get_block_mut(cur2) {
            bb.add_instruction(crate::mir::ssot::extern_call::extern_call(
                Some(dst),
                "env.console",
                "log",
                arg_ids,
                EffectMask::READ,
            ));
        }
        return Ok((dst, cur2));
    }

    if matches!(recv, ExprV0::Var { name } if name == "env") && (method == "get" || method == "set")
    {
        let (arg_ids, cur2) = super::lower_args_with_scope(env, f, cur_bb, args, vars)?;
        let dst = f.next_value_id();
        let effects = if method == "set" {
            EffectMask::IO
        } else {
            EffectMask::READ
        };
        if let Some(bb) = f.get_block_mut(cur2) {
            bb.add_instruction(crate::mir::ssot::extern_call::extern_call(
                Some(dst),
                "env",
                method.to_string(),
                arg_ids,
                effects,
            ));
        }
        return Ok((dst, cur2));
    }

    if let ExprV0::Method {
        recv: inner_recv,
        method: inner_method,
        args: inner_args,
    } = recv
    {
        if matches!(&**inner_recv, ExprV0::Var { name } if name == "env")
            && inner_method == "codegen"
            && inner_args.is_empty()
        {
            let (arg_ids, cur2) = super::lower_args_with_scope(env, f, cur_bb, args, vars)?;
            let dst = f.next_value_id();
            if let Some(bb) = f.get_block_mut(cur2) {
                bb.add_instruction(crate::mir::ssot::extern_call::extern_call(
                    Some(dst),
                    "env.codegen",
                    method.to_string(),
                    arg_ids,
                    EffectMask::IO,
                ));
            }
            return Ok((dst, cur2));
        }
    }

    if let ExprV0::Method {
        recv: inner_recv,
        method: inner_method,
        args: inner_args,
    } = recv
    {
        if matches!(&**inner_recv, ExprV0::Var { name } if name == "env")
            && inner_method == "box_introspect"
            && inner_args.is_empty()
        {
            let (arg_ids, cur2) = super::lower_args_with_scope(env, f, cur_bb, args, vars)?;
            let dst = f.next_value_id();
            if let Some(bb) = f.get_block_mut(cur2) {
                bb.add_instruction(crate::mir::ssot::extern_call::extern_call(
                    Some(dst),
                    "env.box_introspect",
                    method.to_string(),
                    arg_ids,
                    EffectMask::READ,
                ));
            }
            return Ok((dst, cur2));
        }
    }

    if let ExprV0::Var { name: recv_name } = recv {
        if let Some(result) =
            lower_stageb_static_method_call(env, f, cur_bb, recv_name, method, args, vars)?
        {
            return Ok(result);
        }
    }

    let (recv_v, cur) = super::lower_expr_with_scope(env, f, cur_bb, recv, vars)?;
    let (arg_ids, cur2) = super::lower_args_with_scope(env, f, cur, args, vars)?;
    let dst = f.next_value_id();
    if let Some(bb) = f.get_block_mut(cur2) {
        bb.add_instruction(crate::mir::ssot::method_call::runtime_method_call(
            Some(dst),
            recv_v,
            "RuntimeDataBox",
            method,
            arg_ids,
            EffectMask::READ,
            crate::mir::definitions::call_unified::TypeCertainty::Union,
        ));
    }
    Ok((dst, cur2))
}

pub(super) fn lower_new_expr<S: VarScope>(
    env: &BridgeEnv,
    f: &mut MirFunction,
    cur_bb: BasicBlockId,
    class: &str,
    args: &[ExprV0],
    vars: &mut S,
) -> Result<(ValueId, BasicBlockId), String> {
    let (arg_ids, cur) = super::lower_args_with_scope(env, f, cur_bb, args, vars)?;
    let dst = f.next_value_id();
    if let Some(bb) = f.get_block_mut(cur) {
        bb.add_instruction(MirInstruction::NewBox {
            dst,
            box_type: class.to_string(),
            args: arg_ids,
        });
    }
    Ok((dst, cur))
}

fn stageb_method_candidates(env: &BridgeEnv, recv_name: &str) -> Vec<String> {
    let mut candidates: Vec<String> = vec![recv_name.to_string()];
    if recv_name == "me" && env.me_class != "me" {
        candidates.push(env.me_class.clone());
    }
    if let Some(mapped) = env.imports.get(recv_name) {
        if mapped != recv_name {
            candidates.push(mapped.clone());
        }
    }
    candidates
}

fn lower_stageb_static_call_for_box<S: VarScope>(
    env: &BridgeEnv,
    f: &mut MirFunction,
    cur_bb: BasicBlockId,
    box_name: &str,
    method: &str,
    args: &[ExprV0],
    scope: &mut S,
    force_lower: bool,
) -> Result<Option<(ValueId, BasicBlockId)>, String> {
    let qualified = format!("{}.{}{}", box_name, method, format!("/{}", args.len()));
    if !force_lower && !env.static_methods.contains_key(&qualified) {
        return Ok(None);
    }
    let (arg_ids, cur2) = super::lower_args_with_scope(env, f, cur_bb, args, scope)?;
    let fun_val = f.next_value_id();
    if let Some(bb) = f.get_block_mut(cur2) {
        bb.add_instruction(MirInstruction::Const {
            dst: fun_val,
            value: ConstValue::String(qualified),
        });
    }
    let dst = f.next_value_id();
    if let Some(bb) = f.get_block_mut(cur2) {
        bb.add_instruction(MirInstruction::Call {
            dst: Some(dst),
            func: fun_val,
            callee: None,
            args: arg_ids,
            effects: EffectMask::READ,
        });
    }
    Ok(Some((dst, cur2)))
}

fn lower_stageb_instance_call_for_box<S: VarScope>(
    env: &BridgeEnv,
    f: &mut MirFunction,
    cur_bb: BasicBlockId,
    box_name: &str,
    method: &str,
    args: &[ExprV0],
    scope: &mut S,
) -> Result<Option<(ValueId, BasicBlockId)>, String> {
    let qualified = format!("{}.{}{}", box_name, method, format!("/{}", args.len() + 1));
    if !env.static_methods.contains_key(&qualified) {
        return Ok(None);
    }

    let (mut arg_ids, cur2) = super::lower_args_with_scope(env, f, cur_bb, args, scope)?;
    let me_v = f.next_value_id();
    if let Some(bb) = f.get_block_mut(cur2) {
        bb.add_instruction(MirInstruction::Const {
            dst: me_v,
            value: ConstValue::String(box_name.to_string()),
        });
    }
    arg_ids.insert(0, me_v);

    let fun_val = f.next_value_id();
    if let Some(bb) = f.get_block_mut(cur2) {
        bb.add_instruction(MirInstruction::Const {
            dst: fun_val,
            value: ConstValue::String(qualified),
        });
    }
    let dst = f.next_value_id();
    if let Some(bb) = f.get_block_mut(cur2) {
        bb.add_instruction(MirInstruction::Call {
            dst: Some(dst),
            func: fun_val,
            callee: None,
            args: arg_ids,
            effects: EffectMask::READ,
        });
    }
    Ok(Some((dst, cur2)))
}

fn lower_stageb_static_method_call<S: VarScope>(
    env: &BridgeEnv,
    f: &mut MirFunction,
    cur_bb: BasicBlockId,
    recv_name: &str,
    method: &str,
    args: &[ExprV0],
    scope: &mut S,
) -> Result<Option<(ValueId, BasicBlockId)>, String> {
    let recv_is_import_alias = env.imports.contains_key(recv_name);
    let import_maps_to_self = env
        .imports
        .get(recv_name)
        .map(|mapped| mapped == recv_name)
        .unwrap_or(false);
    for box_name in stageb_method_candidates(env, recv_name) {
        // Program(JSON v0) often carries imported static calls only via `imports`,
        // while `defs` stays empty. In that shape, allow alias-qualified static calls
        // to lower directly instead of falling back to runtime String.method(...).
        let force_alias_static =
            recv_is_import_alias && import_maps_to_self && box_name == recv_name;
        if let Some(hit) = lower_stageb_static_call_for_box(
            env,
            f,
            cur_bb,
            &box_name,
            method,
            args,
            scope,
            force_alias_static,
        )? {
            return Ok(Some(hit));
        }
        if let Some(hit) =
            lower_stageb_instance_call_for_box(env, f, cur_bb, &box_name, method, args, scope)?
        {
            return Ok(Some(hit));
        }
    }
    Ok(None)
}
