//! Nyash Python Plugin (Phase 15):
//! - ABI v1 compatible entry points + ABI v2 TypeBox exports
//! - Two Box types: PyRuntimeBox (TYPE_ID=40) and PyObjectBox (TYPE_ID=41)

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicU32, Ordering},
    Mutex,
};

// ===== Error Codes (aligned with other plugins) =====
const NYB_SUCCESS: i32 = 0;
const NYB_E_SHORT_BUFFER: i32 = -1;
const _NYB_E_INVALID_TYPE: i32 = -2;
const NYB_E_INVALID_METHOD: i32 = -3;
const NYB_E_INVALID_ARGS: i32 = -4;
const NYB_E_PLUGIN_ERROR: i32 = -5;
const NYB_E_INVALID_HANDLE: i32 = -8;

// ===== Type IDs (must match nyash.toml) =====
const _TYPE_ID_PY_RUNTIME: u32 = 40;
const TYPE_ID_PY_OBJECT: u32 = 41;

// ===== Method IDs (initial draft) =====
// PyRuntimeBox
const PY_METHOD_BIRTH: u32 = 0; // returns instance_id (u32 LE, no TLV)
const PY_METHOD_EVAL: u32 = 1; // args: string code -> returns Handle(PyObject)
const PY_METHOD_IMPORT: u32 = 2; // args: string name -> returns Handle(PyObject)
const PY_METHOD_FINI: u32 = u32::MAX; // destructor
                                      // Result-returning variants (R)
const PY_METHOD_EVAL_R: u32 = 11;
const PY_METHOD_IMPORT_R: u32 = 12;

// PyObjectBox
const PYO_METHOD_BIRTH: u32 = 0; // reserved (should not be used directly)
const PYO_METHOD_GETATTR: u32 = 1; // args: string name -> returns Handle(PyObject)
const PYO_METHOD_CALL: u32 = 2; // args: variadic TLV -> returns Handle(PyObject)
const PYO_METHOD_STR: u32 = 3; // returns String
const PYO_METHOD_CALL_KW: u32 = 5; // args: key:string, val:TLV, ... -> returns Handle(PyObject)
const PYO_METHOD_FINI: u32 = u32::MAX; // destructor
                                       // Result-returning variants (R)
const PYO_METHOD_GETATTR_R: u32 = 11;
const PYO_METHOD_CALL_R: u32 = 12;
const PYO_METHOD_CALL_KW_R: u32 = 15;

// ===== Minimal in-memory state for stubs =====
#[derive(Default)]
struct PyRuntimeInstance {
    globals: Option<*mut PyObject>,
}
// Safety: Access to CPython state is guarded by the GIL in all call sites
// and we only store raw pointers captured under the GIL. We never mutate
// from multiple threads without reacquiring the GIL. Therefore, mark as
// Send/Sync for storage inside global Lazy<Mutex<...>>.
unsafe impl Send for PyRuntimeInstance {}
unsafe impl Sync for PyRuntimeInstance {}

#[derive(Default)]
struct PyObjectInstance {}

static RUNTIMES: Lazy<Mutex<HashMap<u32, PyRuntimeInstance>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
static PYOBJS: Lazy<Mutex<HashMap<u32, PyObjectInstance>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

static RT_COUNTER: AtomicU32 = AtomicU32::new(1);
static OBJ_COUNTER: AtomicU32 = AtomicU32::new(1);

// ====== CPython FFI and GIL guard ======
mod ffi;
mod gil;
mod pytypes;
use ffi::{ensure_cpython, PyObject};
use pytypes::{DecodedValue, PyOwned};

// loader moved to ffi.rs

// legacy v1 abi/init removed

/* legacy v1 entry removed
#[no_mangle]
pub extern "C" fn nyash_plugin_invoke(
    type_id: u32,
    method_id: u32,
    instance_id: u32,
    args: *const u8,
    args_len: usize,
    result: *mut u8,
    result_len: *mut usize,
) -> i32 {
    match type_id {
        TYPE_ID_PY_RUNTIME => {
            runtime::handle_py_runtime(method_id, instance_id, args, args_len, result, result_len)
        }
        TYPE_ID_PY_OBJECT => {
            object::handle_py_object(method_id, instance_id, args, args_len, result, result_len)
        }
        _ => NYB_E_INVALID_TYPE,
    }
}
*/

mod object;
mod runtime;

extern "C" fn pyruntime_resolve(name: *const std::os::raw::c_char) -> u32 {
    if name.is_null() {
        return 0;
    }
    let Some(s) = (unsafe { pytypes::cstr_to_string(name) }) else {
        return 0;
    };
    match s.as_str() {
        "birth" => PY_METHOD_BIRTH,
        "eval" | "evalR" => {
            if s == "evalR" {
                PY_METHOD_EVAL_R
            } else {
                PY_METHOD_EVAL
            }
        }
        "import" | "importR" => {
            if s == "importR" {
                PY_METHOD_IMPORT_R
            } else {
                PY_METHOD_IMPORT
            }
        }
        "fini" => PY_METHOD_FINI,
        _ => 0,
    }
}

extern "C" fn pyruntime_invoke_id(
    instance_id: u32,
    method_id: u32,
    args: *const u8,
    args_len: usize,
    result: *mut u8,
    result_len: *mut usize,
) -> i32 {
    runtime::handle_py_runtime(method_id, instance_id, args, args_len, result, result_len)
}

extern "C" fn pyobject_resolve(name: *const std::os::raw::c_char) -> u32 {
    if name.is_null() {
        return 0;
    }
    let Some(s) = (unsafe { pytypes::cstr_to_string(name) }) else {
        return 0;
    };
    match s.as_str() {
        "getattr" | "getAttr" | "getattrR" | "getAttrR" => {
            if s.ends_with('R') {
                PYO_METHOD_GETATTR_R
            } else {
                PYO_METHOD_GETATTR
            }
        }
        "call" | "callR" => {
            if s.ends_with('R') {
                PYO_METHOD_CALL_R
            } else {
                PYO_METHOD_CALL
            }
        }
        "callKw" | "callKW" | "call_kw" | "callKwR" | "callKWR" => {
            if s.to_lowercase().ends_with('r') {
                PYO_METHOD_CALL_KW_R
            } else {
                PYO_METHOD_CALL_KW
            }
        }
        "str" | "toString" => PYO_METHOD_STR,
        "birth" => PYO_METHOD_BIRTH,
        "fini" => PYO_METHOD_FINI,
        _ => 0,
    }
}

extern "C" fn pyobject_invoke_id(
    instance_id: u32,
    method_id: u32,
    args: *const u8,
    args_len: usize,
    result: *mut u8,
    result_len: *mut usize,
) -> i32 {
    object::handle_py_object(method_id, instance_id, args, args_len, result, result_len)
}

#[no_mangle]
pub static nyash_typebox_PyRuntimeBox: NyashTypeBoxFfi = NyashTypeBoxFfi {
    abi_tag: 0x54594258,
    version: 1,
    struct_size: std::mem::size_of::<NyashTypeBoxFfi>() as u16,
    name: b"PyRuntimeBox\0".as_ptr() as *const std::os::raw::c_char,
    resolve: Some(pyruntime_resolve),
    invoke_id: Some(pyruntime_invoke_id),
    capabilities: 0,
};

#[no_mangle]
pub static nyash_typebox_PyObjectBox: NyashTypeBoxFfi = NyashTypeBoxFfi {
    abi_tag: 0x54594258,
    version: 1,
    struct_size: std::mem::size_of::<NyashTypeBoxFfi>() as u16,
    name: b"PyObjectBox\0".as_ptr() as *const std::os::raw::c_char,
    resolve: Some(pyobject_resolve),
    invoke_id: Some(pyobject_invoke_id),
    capabilities: 0,
};
fn preflight(result: *mut u8, result_len: *mut usize, needed: usize) -> bool {
    unsafe {
        if result_len.is_null() {
            return false;
        }
        if result.is_null() || *result_len < needed {
            *result_len = needed;
            return true;
        }
    }
    false
}

fn write_tlv_string(s: &str, result: *mut u8, result_len: *mut usize) -> i32 {
    let payload = s.as_bytes();
    write_tlv_result(&[(6u8, payload)], result, result_len)
}

fn write_tlv_handle(
    type_id: u32,
    instance_id: u32,
    result: *mut u8,
    result_len: *mut usize,
) -> i32 {
    let mut payload = [0u8; 8];
    payload[..4].copy_from_slice(&type_id.to_le_bytes());
    payload[4..].copy_from_slice(&instance_id.to_le_bytes());
    write_tlv_result(&[(8u8, &payload)], result, result_len)
}

/// Read nth TLV argument as String (tag 6)
fn read_arg_string(args: *const u8, args_len: usize, n: usize) -> Option<String> {
    if args.is_null() || args_len < 4 {
        return None;
    }
    let buf = unsafe { std::slice::from_raw_parts(args, args_len) };
    let mut off = 4usize; // skip header
    for i in 0..=n {
        if buf.len() < off + 4 {
            return None;
        }
        let tag = buf[off];
        let _rsv = buf[off + 1];
        let size = u16::from_le_bytes([buf[off + 2], buf[off + 3]]) as usize;
        if buf.len() < off + 4 + size {
            return None;
        }
        if i == n {
            if tag != 6 {
                return None;
            }
            let slice = &buf[off + 4..off + 4 + size];
            return std::str::from_utf8(slice).ok().map(|s| s.to_string());
        }
        off += 4 + size;
    }
    None
}

// Side-table for PyObject* storage (instance_id -> pointer)
static PY_HANDLES: Lazy<Mutex<HashMap<u32, PyOwned>>> = Lazy::new(|| Mutex::new(HashMap::new()));

// Base TLV writer used by helpers
fn write_tlv_result(payloads: &[(u8, &[u8])], result: *mut u8, result_len: *mut usize) -> i32 {
    if result_len.is_null() {
        return NYB_E_INVALID_ARGS;
    }
    let mut buf: Vec<u8> =
        Vec::with_capacity(4 + payloads.iter().map(|(_, p)| 4 + p.len()).sum::<usize>());
    buf.extend_from_slice(&1u16.to_le_bytes()); // version
    buf.extend_from_slice(&(payloads.len() as u16).to_le_bytes()); // argc
    for (tag, payload) in payloads {
        buf.push(*tag);
        buf.push(0);
        buf.extend_from_slice(&(payload.len() as u16).to_le_bytes());
        buf.extend_from_slice(payload);
    }
    unsafe {
        let needed = buf.len();
        if result.is_null() || *result_len < needed {
            *result_len = needed;
            return NYB_E_SHORT_BUFFER;
        }
        std::ptr::copy_nonoverlapping(buf.as_ptr(), result, needed);
        *result_len = needed;
    }
    NYB_SUCCESS
}

// ===== TypeBox ABI v2 (resolve/invoke_id) =====
#[repr(C)]
pub struct NyashTypeBoxFfi {
    pub abi_tag: u32,     // 'TYBX'
    pub version: u16,     // 1
    pub struct_size: u16, // sizeof(NyashTypeBoxFfi)
    pub name: *const std::os::raw::c_char,
    pub resolve: Option<extern "C" fn(*const std::os::raw::c_char) -> u32>,
    pub invoke_id: Option<extern "C" fn(u32, u32, *const u8, usize, *mut u8, *mut usize) -> i32>,
    pub capabilities: u64,
}
unsafe impl Sync for NyashTypeBoxFfi {}

fn should_autodecode() -> bool {
    std::env::var("NYASH_PY_AUTODECODE")
        .map(|v| v != "0")
        .unwrap_or(false)
}

fn autodecode_logging_enabled() -> bool {
    std::env::var("NYASH_PY_LOG")
        .map(|v| v != "0")
        .unwrap_or(false)
}

fn write_autodecode_result(
    decoded: &DecodedValue,
    result: *mut u8,
    result_len: *mut usize,
) -> bool {
    let rc = match decoded {
        DecodedValue::Float(value) => {
            if autodecode_logging_enabled() {
                eprintln!("[PyPlugin] autodecode: Float {}", value);
            }
            let payload = value.to_le_bytes();
            write_tlv_result(&[(5u8, payload.as_slice())], result, result_len)
        }
        DecodedValue::Int(value) => {
            if autodecode_logging_enabled() {
                eprintln!("[PyPlugin] autodecode: I64 {}", value);
            }
            let payload = value.to_le_bytes();
            write_tlv_result(&[(3u8, payload.as_slice())], result, result_len)
        }
        DecodedValue::Str(text) => {
            if autodecode_logging_enabled() {
                eprintln!(
                    "[PyPlugin] autodecode: String '{}', len={} ",
                    text,
                    text.len()
                );
            }
            write_tlv_result(&[(6u8, text.as_bytes())], result, result_len)
        }
        DecodedValue::Bytes(data) => {
            if autodecode_logging_enabled() {
                eprintln!("[PyPlugin] autodecode: Bytes {} bytes", data.len());
            }
            write_tlv_result(&[(7u8, data.as_slice())], result, result_len)
        }
    };
    rc == NYB_SUCCESS || rc == NYB_E_SHORT_BUFFER
}
