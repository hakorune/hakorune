use super::super::builder::IRBuilder;
use super::LowerCore;
use crate::mir::{MirFunction, ValueId};

pub(super) fn lower_string_box_method(
    lower: &mut LowerCore,
    func: &MirFunction,
    b: &mut dyn IRBuilder,
    array: &ValueId,
    method: &str,
    args: &Vec<ValueId>,
    dst: Option<ValueId>,
) -> Result<bool, String> {
    if std::env::var("NYASH_USE_PLUGIN_BUILTINS").ok().as_deref() == Some("1")
        && matches!(method, "length" | "is_empty" | "charCodeAt")
    {
        if method == "length" {
            if let Some(pidx) = lower.param_index.get(array).copied() {
                lower.emit_len_with_fallback_param(b, pidx);
                if let Some(d) = dst {
                    let slot = *lower.local_index.entry(d).or_insert_with(|| {
                        let id = lower.next_local;
                        lower.next_local += 1;
                        id
                    });
                    b.store_local_i64(slot);
                }
                return Ok(true);
            }
            if let Some(slot) = lower.local_index.get(array).copied() {
                lower.emit_len_with_fallback_local_handle(b, slot);
                if let Some(d) = dst {
                    let slot = *lower.local_index.entry(d).or_insert_with(|| {
                        let id = lower.next_local;
                        lower.next_local += 1;
                        id
                    });
                    b.store_local_i64(slot);
                }
                return Ok(true);
            }
            let mut lit: Option<String> = None;
            for (_bid, bb) in func.blocks.iter() {
                for ins in bb.instructions.iter() {
                    if let crate::mir::MirInstruction::NewBox {
                        dst,
                        box_type,
                        args,
                    } = ins
                    {
                        if dst == array && box_type == "StringBox" && args.len() == 1 {
                            if let Some(src) = args.get(0) {
                                if let Some(s) = lower.known_str.get(src).cloned() {
                                    lit = Some(s);
                                    break;
                                }
                            }
                        }
                    }
                }
                if lit.is_some() {
                    break;
                }
            }
            if let Some(s) = lit {
                let n = s.len() as i64;
                b.emit_const_i64(n);
                if let Some(d) = dst {
                    lower.known_i64.insert(d, n);
                    let slot = *lower.local_index.entry(d).or_insert_with(|| {
                        let id = lower.next_local;
                        lower.next_local += 1;
                        id
                    });
                    b.store_local_i64(slot);
                }
                return Ok(true);
            }
            lower.push_value_if_known_or_param(b, array);
            b.emit_host_call(crate::jit::r#extern::handles::SYM_HANDLE_OF, 1, true);
            b.emit_host_call(crate::jit::r#extern::collections::SYM_ANY_LEN_H, 1, true);
            if let Some(d) = dst {
                let slot = *lower.local_index.entry(d).or_insert_with(|| {
                    let id = lower.next_local;
                    lower.next_local += 1;
                    id
                });
                b.store_local_i64(slot);
            }
            return Ok(true);
        }
        if let Some(pidx) = lower.param_index.get(array).copied() {
            b.emit_param_i64(pidx);
            b.emit_host_call(crate::jit::r#extern::handles::SYM_HANDLE_OF, 1, true);
        } else if let Some(slot) = lower.local_index.get(array).copied() {
            b.load_local_i64(slot);
        } else {
            lower.push_value_if_known_or_param(b, array);
            b.emit_host_call(crate::jit::r#extern::handles::SYM_HANDLE_OF, 1, true);
        }
        let mut argc = 1usize;
        if method == "charCodeAt" {
            if let Some(v) = args.get(0) {
                lower.push_value_if_known_or_param(b, v);
            } else {
                b.emit_const_i64(0);
            }
            argc = 2;
        }
        if method == "is_empty" {
            b.hint_ret_bool(true);
        }
        let decision = crate::jit::policy::invoke::decide_box_method(
            "StringBox",
            method,
            argc,
            dst.is_some(),
        );
        match decision {
            crate::jit::policy::invoke::InvokeDecision::HostCall { symbol, .. } => {
                crate::jit::observe::lower_hostcall(
                    &symbol,
                    argc,
                    &if argc == 1 {
                        ["Handle"][..].to_vec()
                    } else {
                        ["Handle", "I64"][..].to_vec()
                    },
                    "allow",
                    "mapped_symbol",
                );
                b.emit_host_call(&symbol, argc, dst.is_some());
                return Ok(true);
            }
            crate::jit::policy::invoke::InvokeDecision::PluginInvoke {
                type_id,
                method_id,
                box_type,
                ..
            } => {
                b.emit_plugin_invoke(type_id, method_id, argc, dst.is_some());
                crate::jit::observe::lower_plugin_invoke(
                    &box_type,
                    method,
                    type_id,
                    method_id,
                    argc,
                );
                return Ok(true);
            }
            _ => {}
        }
    }

    match method {
        "getField" | "setField" => {
            if std::env::var("NYASH_JIT_HOST_BRIDGE").ok().as_deref() == Some("1") {
                if let Some(v) = args.get(0) {
                    let _ = v;
                }
                lower.push_value_if_known_or_param(b, array);
                if let Some(name_id) = args.get(0) {
                    if let Some(s) = lower.known_str.get(name_id).cloned() {
                        b.emit_string_handle_from_literal(&s);
                    } else {
                        b.emit_const_i64(0);
                    }
                } else {
                    b.emit_const_i64(0);
                }
                let argc = if method == "setField" {
                    if let Some(val_id) = args.get(1) {
                        if let Some(s) = lower.known_str.get(val_id).cloned() {
                            b.emit_string_handle_from_literal(&s);
                        } else {
                            lower.push_value_if_known_or_param(b, val_id);
                        }
                    } else {
                        b.emit_const_i64(0);
                    }
                    3
                } else {
                    2
                };
                let sym = crate::jit::r#extern::host_bridge::SYM_HOST_INSTANCE_FIELD3;
                if method == "getField" {
                    b.emit_const_i64(-1);
                }
                b.emit_host_call_fixed3(sym, dst.is_some());
                return Ok(true);
            }
        }
        "len" => {
            let trace = std::env::var("NYASH_JIT_TRACE_LOWER_LEN").ok().as_deref() == Some("1");
            let mut lit_len: Option<i64> = None;
            for (_bbid, bb) in func.blocks.iter() {
                for ins in bb.instructions.iter() {
                    if let crate::mir::MirInstruction::Const { dst, value } = ins {
                        if dst == array {
                            if let crate::mir::ConstValue::String(s) = value {
                                lit_len = Some(s.len() as i64);
                            }
                            break;
                        }
                    }
                }
                if lit_len.is_some() {
                    break;
                }
            }
            if let Some(n) = lit_len {
                if trace {
                    eprintln!(
                        "[LOWER] StringBox.len: literal length={} (dst?={})",
                        n,
                        dst.is_some()
                    );
                }
                b.emit_const_i64(n);
                if let Some(d) = dst {
                    let slot = *lower.local_index.entry(d).or_insert_with(|| {
                        let id = lower.next_local;
                        lower.next_local += 1;
                        id
                    });
                    b.store_local_i64(slot);
                }
                return Ok(true);
            }
            if std::env::var("NYASH_JIT_HOST_BRIDGE").ok().as_deref() == Some("1") {
                if lower
                    .box_type_map
                    .get(array)
                    .map(|s| s == "StringBox")
                    .unwrap_or(false)
                {
                    if std::env::var("NYASH_JIT_TRACE_BRIDGE").ok().as_deref() == Some("1") {
                        eprintln!("[LOWER]string.len via host-bridge");
                    }
                    if trace {
                        eprintln!(
                            "[LOWER] StringBox.len via host-bridge (dst?={})",
                            dst.is_some()
                        );
                    }
                    lower.push_value_if_known_or_param(b, array);
                    b.emit_host_call(
                        crate::jit::r#extern::host_bridge::SYM_HOST_STRING_LEN,
                        1,
                        dst.is_some(),
                    );
                    if let Some(d) = dst {
                        let slot = *lower.local_index.entry(d).or_insert_with(|| {
                            let id = lower.next_local;
                            lower.next_local += 1;
                            id
                        });
                        b.store_local_i64(slot);
                    }
                    return Ok(true);
                }
            }
            if lower
                .box_type_map
                .get(array)
                .map(|s| s == "StringBox")
                .unwrap_or(false)
            {
                if let Some(s) = lower.string_box_literal.get(array).cloned() {
                    let n = s.len() as i64;
                    b.emit_const_i64(n);
                    if let Some(d) = dst {
                        let slot = *lower.local_index.entry(d).or_insert_with(|| {
                            let id = lower.next_local;
                            lower.next_local += 1;
                            id
                        });
                        b.store_local_i64(slot);
                        lower.known_i64.insert(d, n);
                    }
                    return Ok(true);
                }
                let mut lit: Option<String> = None;
                for (_bid, bb) in func.blocks.iter() {
                    for ins in bb.instructions.iter() {
                        if let crate::mir::MirInstruction::NewBox {
                            dst,
                            box_type,
                            args,
                        } = ins
                        {
                            if dst == array && box_type == "StringBox" && args.len() == 1 {
                                if let Some(src) = args.get(0) {
                                    if let Some(s) = lower.known_str.get(src).cloned() {
                                        lit = Some(s);
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    if lit.is_some() {
                        break;
                    }
                }
                if let Some(s) = lit {
                    if trace {
                        eprintln!(
                            "[LOWER] StringBox.len reconstructed literal '{}' (dst?={})",
                            s,
                            dst.is_some()
                        );
                    }
                    let n = s.len() as i64;
                    b.emit_const_i64(n);
                    if let Some(d) = dst {
                        let dslot = *lower.local_index.entry(d).or_insert_with(|| {
                            let id = lower.next_local;
                            lower.next_local += 1;
                            id
                        });
                        b.store_local_i64(dslot);
                        lower.known_i64.insert(d, n);
                    }
                    return Ok(true);
                }
                if let Some(pidx) = lower.param_index.get(array).copied() {
                    if trace {
                        eprintln!(
                            "[LOWER] StringBox.len param p{} (dst?={})",
                            pidx,
                            dst.is_some()
                        );
                    }
                    lower.emit_len_with_fallback_param(b, pidx);
                    if let Some(d) = dst {
                        let slot = *lower.local_index.entry(d).or_insert_with(|| {
                            let id = lower.next_local;
                            lower.next_local += 1;
                            id
                        });
                        b.store_local_i64(slot);
                    }
                    return Ok(true);
                }
                if let Some(slot) = lower.local_index.get(array).copied() {
                    if trace {
                        eprintln!(
                            "[LOWER] StringBox.len local slot#{} (dst?={})",
                            slot,
                            dst.is_some()
                        );
                    }
                    lower.emit_len_with_fallback_local_handle(b, slot);
                    if let Some(d) = dst {
                        let dslot = *lower.local_index.entry(d).or_insert_with(|| {
                            let id = lower.next_local;
                            lower.next_local += 1;
                            id
                        });
                        b.store_local_i64(dslot);
                    }
                    return Ok(true);
                }
                if trace {
                    eprintln!(
                        "[LOWER] StringBox.len last-resort handle.of + fallback (dst?={})",
                        dst.is_some()
                    );
                }
                lower.push_value_if_known_or_param(b, array);
                b.emit_host_call(crate::jit::r#extern::handles::SYM_HANDLE_OF, 1, true);
                let t_recv = {
                    let id = lower.next_local;
                    lower.next_local += 1;
                    id
                };
                b.store_local_i64(t_recv);
                lower.emit_len_with_fallback_local_handle(b, t_recv);
                if let Some(d) = dst {
                    let dslot = *lower.local_index.entry(d).or_insert_with(|| {
                        let id = lower.next_local;
                        lower.next_local += 1;
                        id
                    });
                    b.store_local_i64(dslot);
                }
                return Ok(true);
            }
            if trace {
                eprintln!(
                    "[LOWER] StringBox.len not handled (box_type={:?})",
                    lower.box_type_map.get(array)
                );
            }
            return Ok(false);
        }
        "length" => {
            let trace = std::env::var("NYASH_JIT_TRACE_LOWER_LEN").ok().as_deref() == Some("1");
            if lower
                .box_type_map
                .get(array)
                .map(|s| s == "StringBox")
                .unwrap_or(false)
            {
                let mut lit: Option<String> = None;
                for (_bid, bb) in func.blocks.iter() {
                    for ins in bb.instructions.iter() {
                        if let crate::mir::MirInstruction::NewBox {
                            dst,
                            box_type,
                            args,
                        } = ins
                        {
                            if dst == array && box_type == "StringBox" && args.len() == 1 {
                                if let Some(src) = args.get(0) {
                                    if let Some(s) = lower.known_str.get(src).cloned() {
                                        lit = Some(s);
                                        break;
                                    }
                                    for (_b2, bb2) in func.blocks.iter() {
                                        for ins2 in bb2.instructions.iter() {
                                            if let crate::mir::MirInstruction::Const {
                                                dst: cdst,
                                                value,
                                            } = ins2
                                            {
                                                if cdst == src {
                                                    if let crate::mir::ConstValue::String(sv) =
                                                        value
                                                    {
                                                        lit = Some(sv.clone());
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                        if lit.is_some() {
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if lit.is_some() {
                        break;
                    }
                }
                if let Some(s) = lit {
                    let n = s.len() as i64;
                    b.emit_const_i64(n);
                    if let Some(d) = dst {
                        let slot = *lower.local_index.entry(d).or_insert_with(|| {
                            let id = lower.next_local;
                            lower.next_local += 1;
                            id
                        });
                        b.store_local_i64(slot);
                        lower.known_i64.insert(d, n);
                    }
                    return Ok(true);
                }
                let handled = lower_string_box_method(lower, func, b, array, "len", args, dst)?;
                if handled {
                    return Ok(true);
                }
                if trace {
                    eprintln!(
                        "[LOWER] StringBox.length fallback any.length_h on handle.of (dst?={})",
                        dst.is_some()
                    );
                }
                lower.push_value_if_known_or_param(b, array);
                b.emit_host_call(crate::jit::r#extern::handles::SYM_HANDLE_OF, 1, true);
                b.emit_host_call(
                    crate::jit::r#extern::collections::SYM_ANY_LEN_H,
                    1,
                    dst.is_some(),
                );
                if let Some(d) = dst {
                    let slot = *lower.local_index.entry(d).or_insert_with(|| {
                        let id = lower.next_local;
                        lower.next_local += 1;
                        id
                    });
                    b.store_local_i64(slot);
                }
                return Ok(true);
            }
            return Ok(false);
        }
        "len" | "length" => {
            match lower.box_type_map.get(array).map(|s| s.as_str()) {
                Some("StringBox") => {
                    if std::env::var("NYASH_JIT_DISABLE_LEN_CONST").ok().as_deref() != Some("1")
                        && lower.string_box_literal.get(array).is_some()
                    {
                        let s = lower.string_box_literal.get(array).cloned().unwrap();
                        let n = s.len() as i64;
                        b.emit_const_i64(n);
                        if let Some(d) = dst {
                            let slot = *lower.local_index.entry(d).or_insert_with(|| {
                                let id = lower.next_local;
                                lower.next_local += 1;
                                id
                            });
                            b.store_local_i64(slot);
                            lower.known_i64.insert(d, n);
                        }
                        return Ok(true);
                    }
                    if let Some(pidx) = lower.param_index.get(array).copied() {
                        lower.emit_len_with_fallback_param(b, pidx);
                        if let Some(d) = dst {
                            let slot = *lower.local_index.entry(d).or_insert_with(|| {
                                let id = lower.next_local;
                                lower.next_local += 1;
                                id
                            });
                            b.store_local_i64(slot);
                        }
                        return Ok(true);
                    }
                    if let Some(slot) = lower.local_index.get(array).copied() {
                        lower.emit_len_with_fallback_local_handle(b, slot);
                        if let Some(d) = dst {
                            let slot = *lower.local_index.entry(d).or_insert_with(|| {
                                let id = lower.next_local;
                                lower.next_local += 1;
                                id
                            });
                            b.store_local_i64(slot);
                        }
                        return Ok(true);
                    }
                    let mut lit: Option<String> = None;
                    for (_bid, bb) in func.blocks.iter() {
                        for ins in bb.instructions.iter() {
                            if let crate::mir::MirInstruction::NewBox {
                                dst,
                                box_type,
                                args,
                            } = ins
                            {
                                if dst == array && box_type == "StringBox" && args.len() == 1 {
                                    if let Some(src) = args.get(0) {
                                        if let Some(s) = lower.known_str.get(src).cloned() {
                                            lit = Some(s);
                                            break;
                                        }
                                        for (_b2, bb2) in func.blocks.iter() {
                                            for ins2 in bb2.instructions.iter() {
                                                if let crate::mir::MirInstruction::Const {
                                                    dst: cdst,
                                                    value,
                                                } = ins2
                                                {
                                                    if cdst == src {
                                                        if let crate::mir::ConstValue::String(
                                                            sv,
                                                        ) = value
                                                        {
                                                            lit = Some(sv.clone());
                                                            break;
                                                        }
                                                    }
                                                }
                                            }
                                            if lit.is_some() {
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        if lit.is_some() {
                            break;
                        }
                    }
                    if let Some(s) = lit {
                        let n = s.len() as i64;
                        b.emit_const_i64(n);
                        if let Some(d) = dst {
                            let slot = *lower.local_index.entry(d).or_insert_with(|| {
                                let id = lower.next_local;
                                lower.next_local += 1;
                                id
                            });
                            b.store_local_i64(slot);
                            lower.known_i64.insert(d, n);
                        }
                        return Ok(true);
                    }
                    lower.push_value_if_known_or_param(b, array);
                    b.emit_host_call(crate::jit::r#extern::handles::SYM_HANDLE_OF, 1, true);
                    let slot = {
                        let id = lower.next_local;
                        lower.next_local += 1;
                        id
                    };
                    b.store_local_i64(slot);
                    lower.emit_len_with_fallback_local_handle(b, slot);
                    if let Some(d) = dst {
                        let dslot = *lower.local_index.entry(d).or_insert_with(|| {
                            let id = lower.next_local;
                            lower.next_local += 1;
                            id
                        });
                        b.store_local_i64(dslot);
                    }
                    return Ok(true);
                }
                Some("ArrayBox") => {}
                _ => {
                    lower.push_value_if_known_or_param(b, array);
                    b.emit_host_call(crate::jit::r#extern::handles::SYM_HANDLE_OF, 1, true);
                    b.emit_host_call(crate::jit::r#extern::collections::SYM_ANY_LEN_H, 1, true);
                    if let Some(d) = dst {
                        let slot = *lower.local_index.entry(d).or_insert_with(|| {
                            let id = lower.next_local;
                            lower.next_local += 1;
                            id
                        });
                        b.store_local_i64(slot);
                    }
                    return Ok(true);
                }
            }
            if let Ok(ph) = crate::runtime::plugin_loader_unified::get_global_plugin_host().read() {
                if let Ok(h) = ph.resolve_method("ArrayBox", "length") {
                    if let Some(pidx) = lower.param_index.get(array).copied() {
                        b.emit_param_i64(pidx);
                    } else {
                        b.emit_const_i64(-1);
                    }
                    b.emit_plugin_invoke(h.type_id, h.method_id, 1, dst.is_some());
                    return Ok(true);
                }
            }
            if let Some(pidx) = lower.param_index.get(array).copied() {
                crate::jit::observe::lower_hostcall(
                    crate::jit::r#extern::collections::SYM_ANY_LEN_H,
                    1,
                    &["Handle"],
                    "allow",
                    "mapped_symbol",
                );
                b.emit_param_i64(pidx);
                b.emit_host_call(
                    crate::jit::r#extern::collections::SYM_ANY_LEN_H,
                    1,
                    dst.is_some(),
                );
            } else {
                crate::jit::observe::lower_hostcall(
                    crate::jit::r#extern::collections::SYM_ARRAY_LEN,
                    1,
                    &["I64"],
                    "fallback",
                    "receiver_not_param",
                );
                b.emit_const_i64(-1);
                b.emit_host_call(
                    crate::jit::r#extern::collections::SYM_ARRAY_LEN,
                    1,
                    dst.is_some(),
                );
            }
            return Ok(true);
        }
        _ => {}
    }

    Ok(false)
}
