use super::super::super::*;
use serde_json::{json, Value};

fn main_json(instructions: Vec<Value>) -> String {
    json!({
        "functions": [{
            "name": "main",
            "entry_block": 0,
            "blocks": [{
                "id": 0,
                "instructions": instructions
            }]
        }]
    })
    .to_string()
}

fn assert_subset_accepts(instructions: Vec<Value>) {
    let mir_json = main_json(instructions);
    let out = check_vm_hako_subset_json(&mir_json);
    assert_eq!(out, Ok(()));
}

fn assert_subset_rejects(instructions: Vec<Value>, reason: &str) {
    let mir_json = main_json(instructions);
    let out = check_vm_hako_subset_json(&mir_json);
    assert_eq!(out, Err(("main".to_string(), 0, reason.to_string())));
}

fn i64_const(dst: u64, value: i64) -> Value {
    json!({
        "op": "const",
        "dst": dst,
        "value": { "type": "i64", "value": value }
    })
}

fn bool_const(dst: u64, value: bool) -> Value {
    json!({
        "op": "const",
        "dst": dst,
        "value": { "type": "bool", "value": value }
    })
}

fn string_const(dst: u64, value: &str) -> Value {
    json!({
        "op": "const",
        "dst": dst,
        "value": {
            "type": { "kind": "handle", "box_type": "StringBox" },
            "value": value
        }
    })
}

fn newbox(dst: u64, box_type: &str) -> Value {
    json!({
        "op": "newbox",
        "dst": dst,
        "type": box_type
    })
}

fn externcall(func: &str, args: Vec<u64>, dst: Option<u64>) -> Value {
    json!({
        "op": "externcall",
        "func": func,
        "args": args,
        "dst": dst.map(Value::from).unwrap_or(Value::Null)
    })
}

fn boxcall(method: &str, box_reg: u64, args: Vec<u64>, dst: u64) -> Value {
    json!({
        "op": "boxcall",
        "method": method,
        "box": box_reg,
        "dst": dst,
        "args": args
    })
}

fn ret(value: u64) -> Value {
    json!({ "op": "ret", "value": value })
}

fn assert_boxcall_accepts(
    box_type: &str,
    method: &str,
    consts: &[(u64, i64)],
    args: Vec<u64>,
    dst: u64,
    ret_value: Option<u64>,
) {
    let mut instructions = vec![newbox(1, box_type)];
    instructions.extend(consts.iter().map(|(dst, value)| i64_const(*dst, *value)));
    instructions.push(boxcall(method, 1, args, dst));
    if let Some(value) = ret_value {
        instructions.push(ret(value));
    }
    assert_subset_accepts(instructions);
}

fn assert_externcall_accepts(
    func: &str,
    consts: &[(u64, i64)],
    args: Vec<u64>,
    dst: Option<u64>,
    ret_value: Option<u64>,
) {
    let mut instructions: Vec<Value> = consts
        .iter()
        .map(|(dst, value)| i64_const(*dst, *value))
        .collect();
    instructions.push(externcall(func, args, dst));
    if let Some(value) = ret_value {
        instructions.push(ret(value));
    }
    assert_subset_accepts(instructions);
}

#[test]
fn subset_rejects_legacy_debug_log_even_with_non_reg_values() {
    assert_subset_rejects(
        vec![json!({
            "op": "debug_log",
            "message": "bad-values",
            "values": ["x"]
        })],
        "debug_log",
    );
}

#[test]
fn subset_rejects_select_missing_then_val() {
    assert_subset_rejects(
        vec![
            bool_const(1, true),
            i64_const(2, 42),
            i64_const(3, 7),
            json!({
                "op": "select",
                "dst": 4,
                "cond": 1,
                "else_val": 3
            }),
        ],
        "select(missing-then-val)",
    );
}

#[test]
fn subset_accepts_externcall_env_get() {
    assert_subset_accepts(vec![
        string_const(1, "RVP_C05_ENV_KEY"),
        externcall("env.get/1", vec![1], Some(2)),
        ret(2),
    ]);
}

#[test]
fn subset_rejects_externcall_env_get_with_missing_arg() {
    assert_subset_rejects(
        vec![externcall("env.get/1", vec![], Some(2))],
        "externcall(env.get:args!=1)",
    );
}

#[test]
fn subset_rejects_externcall_env_get_route_symbol_legacy_shape() {
    assert_subset_rejects(
        vec![externcall("nyash.env.get/1", vec![1], Some(2))],
        "externcall(func:nyash.env.get/1)",
    );
}

#[test]
fn subset_accepts_externcall_env_mirbuilder_emit() {
    assert_subset_accepts(vec![
        string_const(1, "{\"type\":\"Program\",\"body\":[]}"),
        externcall("env.mirbuilder_emit/1", vec![1], Some(2)),
        ret(2),
    ]);
}

#[test]
fn subset_accepts_externcall_hako_last_error() {
    assert_externcall_accepts("hako_last_error/1", &[(1, 0)], vec![1], Some(2), Some(2));
}

#[test]
fn subset_accepts_boxcall_tlscore_last_error_text_h() {
    assert_boxcall_accepts("TlsCoreBox", "last_error_text_h", &[], vec![], 2, Some(2));
}

#[test]
fn subset_accepts_boxcall_tlscore_last_error_status_rows() {
    assert_subset_accepts(vec![
        newbox(1, "TlsCoreBox"),
        boxcall("last_error_is_ok_i64", 1, vec![], 2),
        boxcall("last_error_code_i64", 1, vec![], 3),
        ret(3),
    ]);
}

#[test]
fn subset_rejects_boxcall_tlscore_last_error_text_with_arg() {
    assert_subset_rejects(
        vec![
            newbox(1, "TlsCoreBox"),
            i64_const(2, 0),
            boxcall("last_error_text_h", 1, vec![2], 3),
        ],
        "boxcall(last_error_text_h:args!=0)",
    );
}

#[test]
fn subset_accepts_externcall_hako_barrier_touch_i64() {
    assert_externcall_accepts(
        "hako_barrier_touch_i64/1",
        &[(1, 0)],
        vec![1],
        None,
        Some(1),
    );
}

#[test]
fn subset_accepts_externcall_hako_osvm_reserve_bytes_i64() {
    assert_externcall_accepts(
        "hako_osvm_reserve_bytes_i64/1",
        &[(1, 4096)],
        vec![1],
        Some(2),
        Some(2),
    );
}

#[test]
fn subset_accepts_externcall_nyash_gc_barrier_write() {
    assert_externcall_accepts(
        "nyash.gc.barrier_write/1",
        &[(1, 0)],
        vec![1],
        Some(2),
        Some(2),
    );
}

#[test]
fn subset_accepts_boxcall_osvmcore_reserve_bytes_i64() {
    assert_boxcall_accepts(
        "OsVmCoreBox",
        "reserve_bytes_i64",
        &[(2, 4096)],
        vec![2],
        3,
        Some(3),
    );
}

#[test]
fn subset_accepts_boxcall_osvmcore_reserve_bytes_usize() {
    assert_boxcall_accepts(
        "OsVmCoreBox",
        "reserve_bytes_usize",
        &[(2, 4096)],
        vec![2],
        3,
        Some(3),
    );
}

#[test]
fn subset_accepts_boxcall_rawbufcore_alloc_bytes_i64() {
    assert_boxcall_accepts(
        "RawBufCoreBox",
        "alloc_bytes_i64",
        &[(2, 64)],
        vec![2],
        3,
        Some(3),
    );
}

#[test]
fn subset_accepts_boxcall_rawbufcore_alloc_bytes_usize() {
    assert_boxcall_accepts(
        "RawBufCoreBox",
        "alloc_bytes_usize",
        &[(2, 64)],
        vec![2],
        3,
        Some(3),
    );
}

#[test]
fn subset_accepts_boxcall_rawbufcore_realloc_bytes_i64() {
    assert_boxcall_accepts(
        "RawBufCoreBox",
        "realloc_bytes_i64",
        &[(2, 4096), (3, 128)],
        vec![2, 3],
        4,
        Some(4),
    );
}

#[test]
fn subset_accepts_boxcall_rawbufcore_realloc_bytes_usize() {
    assert_boxcall_accepts(
        "RawBufCoreBox",
        "realloc_bytes_usize",
        &[(2, 4096), (3, 128)],
        vec![2, 3],
        4,
        Some(4),
    );
}

#[test]
fn subset_accepts_boxcall_rawbufcore_free_bytes_i64() {
    assert_boxcall_accepts(
        "RawBufCoreBox",
        "free_bytes_i64",
        &[(2, 4096)],
        vec![2],
        3,
        Some(3),
    );
}

#[test]
fn subset_accepts_boxcall_osvmcore_commit_bytes_i64() {
    assert_boxcall_accepts(
        "OsVmCoreBox",
        "commit_bytes_i64",
        &[(2, 4096), (3, 8192)],
        vec![2, 3],
        4,
        None,
    );
}

#[test]
fn subset_accepts_boxcall_osvmcore_commit_bytes_usize() {
    assert_boxcall_accepts(
        "OsVmCoreBox",
        "commit_bytes_usize",
        &[(2, 4096), (3, 8192)],
        vec![2, 3],
        4,
        None,
    );
}

#[test]
fn subset_accepts_externcall_hako_osvm_commit_bytes_i64() {
    assert_externcall_accepts(
        "hako_osvm_commit_bytes_i64/2",
        &[(1, 4096), (2, 8192)],
        vec![1, 2],
        Some(3),
        None,
    );
}

#[test]
fn subset_accepts_boxcall_osvmcore_decommit_bytes_i64() {
    assert_boxcall_accepts(
        "OsVmCoreBox",
        "decommit_bytes_i64",
        &[(2, 4096), (3, 8192)],
        vec![2, 3],
        4,
        None,
    );
}

#[test]
fn subset_accepts_boxcall_osvmcore_decommit_bytes_usize() {
    assert_boxcall_accepts(
        "OsVmCoreBox",
        "decommit_bytes_usize",
        &[(2, 4096), (3, 8192)],
        vec![2, 3],
        4,
        None,
    );
}

#[test]
fn subset_accepts_externcall_hako_osvm_decommit_bytes_i64() {
    assert_externcall_accepts(
        "hako_osvm_decommit_bytes_i64/2",
        &[(1, 4096), (2, 8192)],
        vec![1, 2],
        Some(3),
        None,
    );
}

#[test]
fn subset_accepts_externcall_hako_osvm_unreserve_bytes_i64() {
    assert_externcall_accepts(
        "hako_osvm_unreserve_bytes_i64/2",
        &[(1, 4096), (2, 8192)],
        vec![1, 2],
        Some(3),
        None,
    );
}
