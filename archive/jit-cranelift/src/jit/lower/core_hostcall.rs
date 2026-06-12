#![allow(unreachable_patterns, unused_variables)]
//! HostCall-related lowering helpers split from core.rs (no behavior change)
use super::builder::IRBuilder;
use crate::mir::{MirFunction, ValueId};
use std::collections::HashMap;

mod advanced;

pub fn lower_array_get(
    b: &mut dyn IRBuilder,
    param_index: &HashMap<ValueId, usize>,
    known_i64: &HashMap<ValueId, i64>,
    array: &ValueId,
    index: &ValueId,
) {
    if crate::jit::config::current().hostcall {
        let use_bridge = std::env::var("NYASH_JIT_HOST_BRIDGE").ok().as_deref() == Some("1");
        let idx = known_i64.get(index).copied().unwrap_or(0);
        if let Some(pidx) = param_index.get(array).copied() {
            b.emit_param_i64(pidx);
            b.emit_const_i64(idx);
            let sym = if use_bridge {
                crate::jit::r#extern::host_bridge::SYM_HOST_ARRAY_GET
            } else {
                crate::jit::r#extern::collections::SYM_ARRAY_GET_H
            };
            b.emit_host_call(sym, 2, true);
        } else {
            let arr_idx = -1;
            b.emit_const_i64(arr_idx);
            b.emit_const_i64(idx);
            let sym = if use_bridge {
                crate::jit::r#extern::host_bridge::SYM_HOST_ARRAY_GET
            } else {
                crate::jit::r#extern::collections::SYM_ARRAY_GET
            };
            b.emit_host_call(sym, 2, true);
        }
    }
}

pub fn lower_map_size_simple(
    b: &mut dyn IRBuilder,
    param_index: &HashMap<ValueId, usize>,
    recv: &ValueId,
    dst_is_some: bool,
) {
    let use_bridge = std::env::var("NYASH_JIT_HOST_BRIDGE").ok().as_deref() == Some("1");
    if let Some(pidx) = param_index.get(recv).copied() {
        b.emit_param_i64(pidx);
        let sym = if use_bridge {
            crate::jit::r#extern::host_bridge::SYM_HOST_MAP_SIZE
        } else {
            crate::jit::r#extern::collections::SYM_MAP_SIZE_H
        };
        b.emit_host_call(sym, 1, dst_is_some);
    }
}

pub fn lower_map_get_simple(
    b: &mut dyn IRBuilder,
    param_index: &HashMap<ValueId, usize>,
    known_i64: &HashMap<ValueId, i64>,
    recv: &ValueId,
    key: &ValueId,
    dst_is_some: bool,
) {
    let use_bridge = std::env::var("NYASH_JIT_HOST_BRIDGE").ok().as_deref() == Some("1");
    if let Some(pidx) = param_index.get(recv).copied() {
        b.emit_param_i64(pidx);
        if let Some(i) = known_i64.get(key).copied() {
            b.emit_const_i64(i);
        } else if let Some(kp) = param_index.get(key).copied() {
            b.emit_param_i64(kp);
        } else {
            b.emit_const_i64(0);
        }
        let sym = if use_bridge {
            crate::jit::r#extern::host_bridge::SYM_HOST_MAP_GET
        } else {
            crate::jit::r#extern::collections::SYM_MAP_GET_H
        };
        b.emit_host_call(sym, 2, dst_is_some);
    }
}

pub fn lower_map_has_simple(
    b: &mut dyn IRBuilder,
    param_index: &HashMap<ValueId, usize>,
    known_i64: &HashMap<ValueId, i64>,
    recv: &ValueId,
    key: &ValueId,
    dst_is_some: bool,
) {
    let use_bridge = std::env::var("NYASH_JIT_HOST_BRIDGE").ok().as_deref() == Some("1");
    if let Some(pidx) = param_index.get(recv).copied() {
        b.emit_param_i64(pidx);
        if let Some(i) = known_i64.get(key).copied() {
            b.emit_const_i64(i);
        } else if let Some(kp) = param_index.get(key).copied() {
            b.emit_param_i64(kp);
        } else {
            b.emit_const_i64(0);
        }
        let sym = if use_bridge {
            crate::jit::r#extern::host_bridge::SYM_HOST_MAP_HAS
        } else {
            crate::jit::r#extern::collections::SYM_MAP_HAS_H
        };
        b.emit_host_call(sym, 2, dst_is_some);
    }
}

pub fn lower_map_set_simple(
    b: &mut dyn IRBuilder,
    param_index: &HashMap<ValueId, usize>,
    known_i64: &HashMap<ValueId, i64>,
    recv: &ValueId,
    key: &ValueId,
    value: &ValueId,
) {
    let use_bridge = std::env::var("NYASH_JIT_HOST_BRIDGE").ok().as_deref() == Some("1");
    if let Some(pidx) = param_index.get(recv).copied() {
        b.emit_param_i64(pidx);
        if let Some(i) = known_i64.get(key).copied() {
            b.emit_const_i64(i);
        } else if let Some(kp) = param_index.get(key).copied() {
            b.emit_param_i64(kp);
        } else {
            b.emit_const_i64(0);
        }
        if let Some(i) = known_i64.get(value).copied() {
            b.emit_const_i64(i);
        } else if let Some(vp) = param_index.get(value).copied() {
            b.emit_param_i64(vp);
        } else {
            b.emit_const_i64(0);
        }
        let sym = if use_bridge {
            crate::jit::r#extern::host_bridge::SYM_HOST_MAP_SET
        } else {
            crate::jit::r#extern::collections::SYM_MAP_SET_H
        };
        b.emit_host_call(sym, 3, false);
    }
}
pub fn lower_array_set(
    b: &mut dyn IRBuilder,
    param_index: &HashMap<ValueId, usize>,
    known_i64: &HashMap<ValueId, i64>,
    array: &ValueId,
    index: &ValueId,
    value: &ValueId,
) {
    if crate::jit::config::current().hostcall {
        let use_bridge = std::env::var("NYASH_JIT_HOST_BRIDGE").ok().as_deref() == Some("1");
        let idx = known_i64.get(index).copied().unwrap_or(0);
        let val = known_i64.get(value).copied().unwrap_or(0);
        if let Some(pidx) = param_index.get(array).copied() {
            b.emit_param_i64(pidx);
            b.emit_const_i64(idx);
            b.emit_const_i64(val);
            let sym = if use_bridge {
                crate::jit::r#extern::host_bridge::SYM_HOST_ARRAY_SET
            } else {
                crate::jit::r#extern::collections::SYM_ARRAY_SET_H
            };
            b.emit_host_call(sym, 3, false);
        } else {
            let arr_idx = -1;
            b.emit_const_i64(arr_idx);
            b.emit_const_i64(idx);
            b.emit_const_i64(val);
            let sym = if use_bridge {
                crate::jit::r#extern::host_bridge::SYM_HOST_ARRAY_SET
            } else {
                crate::jit::r#extern::collections::SYM_ARRAY_SET
            };
            b.emit_host_call(sym, 3, false);
        }
    }
}
