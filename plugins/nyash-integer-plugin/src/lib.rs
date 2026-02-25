//! Nyash IntCellBox Plugin — TypeBox v2 (minimal)
//! Methods: birth(0), get(1), set(2), fini(u32::MAX)

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::{
    atomic::{AtomicU32, Ordering},
    Mutex,
};

// Error codes
const OK: i32 = 0;
const E_SHORT: i32 = -1;
const E_METHOD: i32 = -3;
const E_ARGS: i32 = -4;
const E_PLUGIN: i32 = -5;
const E_HANDLE: i32 = -8;

// Methods
const M_BIRTH: u32 = 0;
const M_GET: u32 = 1;
const M_SET: u32 = 2;
const M_FINI: u32 = u32::MAX;

struct IntInstance {
    value: i64,
}

static INST: Lazy<Mutex<HashMap<u32, IntInstance>>> = Lazy::new(|| Mutex::new(HashMap::new()));
static NEXT_ID: AtomicU32 = AtomicU32::new(1);

// ===== TypeBox FFI (resolve/invoke_id) =====
#[repr(C)]
pub struct NyashTypeBoxFfi {
    pub abi_tag: u32,        // 'TYBX'
    pub version: u16,        // 1
    pub struct_size: u16,    // sizeof(NyashTypeBoxFfi)
    pub name: *const c_char, // C string
    pub resolve: Option<extern "C" fn(*const c_char) -> u32>,
    pub invoke_id: Option<extern "C" fn(u32, u32, *const u8, usize, *mut u8, *mut usize) -> i32>,
    pub capabilities: u64,
}
unsafe impl Sync for NyashTypeBoxFfi {}

extern "C" fn intcell_resolve(name: *const c_char) -> u32 {
    if name.is_null() {
        return 0;
    }
    let s = unsafe { CStr::from_ptr(name) }.to_string_lossy();
    match s.as_ref() {
        "get" => M_GET,
        "set" => M_SET,
        _ => 0,
    }
}

extern "C" fn intcell_invoke_id(
    instance_id: u32,
    method_id: u32,
    args: *const u8,
    args_len: usize,
    result: *mut u8,
    result_len: *mut usize,
) -> i32 {
    match method_id {
        M_BIRTH => {
            // Birth contract (v2): return raw u32 instance_id (LE, 4 bytes).
            let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
            let init = read_arg_i64(args, args_len, 0).unwrap_or(0);
            if let Ok(mut m) = INST.lock() {
                m.insert(id, IntInstance { value: init });
                return write_u32(id, result, result_len);
            } else {
                return E_PLUGIN;
            }
        }
        M_FINI => {
            // Destroy IntCellBox instance
            if let Ok(mut m) = INST.lock() {
                m.remove(&instance_id);
                return OK;
            } else {
                return E_PLUGIN;
            }
        }
        M_GET => {
            if let Ok(m) = INST.lock() {
                if let Some(inst) = m.get(&instance_id) {
                    return write_tlv_i64(inst.value, result, result_len);
                } else {
                    return E_HANDLE;
                }
            } else {
                return E_PLUGIN;
            }
        }
        M_SET => {
            // Some MIR paths include receiver in args[0] for method calls.
            // Accept either args[0] or args[1] as the numeric payload.
            let v =
                match read_arg_i64(args, args_len, 0).or_else(|| read_arg_i64(args, args_len, 1)) {
                    Some(v) => v,
                    None => return E_ARGS,
                };
            if let Ok(mut m) = INST.lock() {
                if let Some(inst) = m.get_mut(&instance_id) {
                    inst.value = v;
                    return write_tlv_i64(inst.value, result, result_len);
                } else {
                    return E_HANDLE;
                }
            } else {
                return E_PLUGIN;
            }
        }
        _ => E_METHOD,
    }
}

#[no_mangle]
pub static nyash_typebox_IntCellBox: NyashTypeBoxFfi = NyashTypeBoxFfi {
    abi_tag: 0x54594258, // 'TYBX'
    version: 1,
    struct_size: std::mem::size_of::<NyashTypeBoxFfi>() as u16,
    name: b"IntCellBox\0".as_ptr() as *const c_char,
    resolve: Some(intcell_resolve),
    invoke_id: Some(intcell_invoke_id),
    capabilities: 0,
};

fn write_u32(v: u32, result: *mut u8, result_len: *mut usize) -> i32 {
    unsafe {
        if result_len.is_null() {
            return E_ARGS;
        }
        if result.is_null() || *result_len < 4 {
            *result_len = 4;
            return E_SHORT;
        }
        std::ptr::copy_nonoverlapping(v.to_le_bytes().as_ptr(), result, 4);
        *result_len = 4;
        OK
    }
}

fn write_tlv_result(payloads: &[(u8, &[u8])], result: *mut u8, result_len: *mut usize) -> i32 {
    if result_len.is_null() {
        return E_ARGS;
    }
    let mut buf: Vec<u8> =
        Vec::with_capacity(4 + payloads.iter().map(|(_, p)| 4 + p.len()).sum::<usize>());
    buf.extend_from_slice(&1u16.to_le_bytes());
    buf.extend_from_slice(&(payloads.len() as u16).to_le_bytes());
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
            return E_SHORT;
        }
        std::ptr::copy_nonoverlapping(buf.as_ptr(), result, needed);
        *result_len = needed;
    }
    OK
}

fn write_tlv_i64(v: i64, result: *mut u8, result_len: *mut usize) -> i32 {
    write_tlv_result(&[(3u8, &v.to_le_bytes())], result, result_len)
}

fn read_arg_i64(args: *const u8, args_len: usize, n: usize) -> Option<i64> {
    if args.is_null() || args_len < 4 {
        return None;
    }
    let buf = unsafe { std::slice::from_raw_parts(args, args_len) };
    let mut off = 4usize; // header
    for i in 0..=n {
        if buf.len() < off + 4 {
            return None;
        }
        let tag = buf[off];
        let size = u16::from_le_bytes([buf[off + 2], buf[off + 3]]) as usize;
        if buf.len() < off + 4 + size {
            return None;
        }
        if i == n {
            match (tag, size) {
                (3, 8) => {
                    let mut b = [0u8; 8];
                    b.copy_from_slice(&buf[off + 4..off + 12]);
                    return Some(i64::from_le_bytes(b));
                }
                (2, 4) => {
                    let mut b = [0u8; 4];
                    b.copy_from_slice(&buf[off + 4..off + 8]);
                    let v = i32::from_le_bytes(b) as i64;
                    return Some(v);
                }
                (6, _) => {
                    let bytes = &buf[off + 4..off + 4 + size];
                    if let Ok(s) = std::str::from_utf8(bytes) {
                        if let Ok(v) = s.trim().parse::<i64>() {
                            return Some(v);
                        }
                    }
                    return None;
                }
                _ => return None,
            }
        }
        off += 4 + size;
    }
    None
}
