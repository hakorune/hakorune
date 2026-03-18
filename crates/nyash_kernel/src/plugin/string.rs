// ---- String helpers for LLVM lowering ----
#[inline]
fn into_c_string_ptr(mut bytes: Vec<u8>) -> *mut i8 {
    bytes.push(0);
    let boxed = bytes.into_boxed_slice();
    let raw = Box::into_raw(boxed) as *mut u8;
    raw as *mut i8
}

#[inline]
fn string_to_c_string_ptr(s: String) -> *mut i8 {
    into_c_string_ptr(s.into_bytes())
}

#[inline]
fn c_string_bytes<'a>(ptr: *const i8) -> &'a [u8] {
    if ptr.is_null() {
        return &[];
    }
    unsafe { std::ffi::CStr::from_ptr(ptr).to_bytes() }
}

// Exported as: nyash_string_new(i8* ptr, i32 len) -> i8*
#[no_mangle]
pub extern "C" fn nyash_string_new(ptr: *const u8, len: i32) -> *mut i8 {
    use std::ptr;
    if ptr.is_null() || len < 0 {
        return std::ptr::null_mut();
    }
    let n = len as usize;
    // Allocate n+1 and null-terminate for C interop (puts, etc.)
    let mut buf = Vec::<u8>::with_capacity(n + 1);
    unsafe {
        ptr::copy_nonoverlapping(ptr, buf.as_mut_ptr(), n);
        buf.set_len(n);
    }
    into_c_string_ptr(buf)
}

// ---- String concat helpers for LLVM lowering ----
// Exported as: nyash.string.concat_ss(i8* a, i8* b) -> i8*
#[export_name = "nyash.string.concat_ss"]
pub extern "C" fn nyash_string_concat_ss(a: *const i8, b: *const i8) -> *mut i8 {
    let a_bytes = c_string_bytes(a);
    let b_bytes = c_string_bytes(b);
    let mut out = Vec::with_capacity(a_bytes.len() + b_bytes.len() + 1);
    out.extend_from_slice(a_bytes);
    out.extend_from_slice(b_bytes);
    into_c_string_ptr(out)
}

// Exported as: nyash.string.concat_si(i8* a, i64 b) -> i8*
// NOTE: Phase 131-15-P1 - Kept for diagnostic/compatibility purposes
// LLVM backend now uses concat_hh(handle, handle) for all mixed concatenations
#[export_name = "nyash.string.concat_si"]
pub extern "C" fn nyash_string_concat_si(a: *const i8, b: i64) -> *mut i8 {
    let mut s = String::new();
    unsafe {
        if !a.is_null() {
            if let Ok(sa) = std::ffi::CStr::from_ptr(a).to_str() {
                s.push_str(sa);
            }
        }
    }
    s.push_str(&b.to_string());
    string_to_c_string_ptr(s)
}

// Exported as: nyash.string.concat_is(i64 a, i8* b) -> i8*
// NOTE: Phase 131-15-P1 - Kept for diagnostic/compatibility purposes
// LLVM backend now uses concat_hh(handle, handle) for all mixed concatenations
#[export_name = "nyash.string.concat_is"]
pub extern "C" fn nyash_string_concat_is(a: i64, b: *const i8) -> *mut i8 {
    let mut s = a.to_string();
    unsafe {
        if !b.is_null() {
            if let Ok(sb) = std::ffi::CStr::from_ptr(b).to_str() {
                s.push_str(sb);
            }
        }
    }
    string_to_c_string_ptr(s)
}

// Exported as: nyash.string.substring_sii(i8* s, i64 start, i64 end) -> i8*
#[export_name = "nyash.string.substring_sii"]
pub extern "C" fn nyash_string_substring_sii(s: *const i8, start: i64, end: i64) -> *mut i8 {
    if s.is_null() {
        return std::ptr::null_mut();
    }
    let src = c_string_bytes(s);
    let n = src.len() as i64;
    let mut st = if start < 0 { 0 } else { start };
    let mut en = if end < 0 { 0 } else { end };
    if st > n {
        st = n;
    }
    if en > n {
        en = n;
    }
    if en < st {
        std::mem::swap(&mut st, &mut en);
    }
    let (st_u, en_u) = (st as usize, en as usize);
    into_c_string_ptr(src[st_u..en_u].to_vec())
}

// Exported as: nyash.string.lastIndexOf_ss(i8* s, i8* needle) -> i64
#[export_name = "nyash.string.lastIndexOf_ss"]
pub extern "C" fn nyash_string_lastindexof_ss(s: *const i8, needle: *const i8) -> i64 {
    use std::ffi::CStr;
    if s.is_null() || needle.is_null() {
        return -1;
    }
    let hs = unsafe { CStr::from_ptr(s) };
    let ns = unsafe { CStr::from_ptr(needle) };
    let h = match hs.to_str() {
        Ok(v) => v,
        Err(_) => return -1,
    };
    let n = match ns.to_str() {
        Ok(v) => v,
        Err(_) => return -1,
    };
    if n.is_empty() {
        return h.len() as i64;
    }
    if let Some(pos) = h.rfind(n) {
        pos as i64
    } else {
        -1
    }
}

// Exported as: nyash.string.length_si(i8* s, i64 mode) -> i64
// mode: 0 = byte length (UTF-8 bytes), 1 = char length (Unicode scalar count)
#[export_name = "nyash.string.length_si"]
pub extern "C" fn nyash_string_length_si(s: *const i8, mode: i64) -> i64 {
    use std::ffi::CStr;
    if s.is_null() {
        return 0;
    }
    let cs = unsafe { CStr::from_ptr(s) };
    // Safe UTF-8 conversion; on failure, fall back to byte length scan
    if let Ok(st) = cs.to_str() {
        if mode == 1 {
            // char count
            return st.chars().count() as i64;
        } else {
            // byte length
            return st.as_bytes().len() as i64;
        }
    }
    // Fallback: manual byte scan to NUL
    let mut len: i64 = 0;
    unsafe {
        let mut p = s;
        while *p != 0 {
            len += 1;
            p = p.add(1);
        }
    }
    len
}

// Exported as: nyash.string.to_i8p_h(i64 handle) -> i8*
#[export_name = "nyash.string.to_i8p_h"]
pub extern "C" fn nyash_string_to_i8p_h(handle: i64) -> *mut i8 {
    use nyash_rust::runtime::host_handles as handles;
    if handle <= 0 {
        // return "0" for consistency with existing fallback behavior
        return string_to_c_string_ptr(handle.to_string());
    }
    if let Some(obj) = handles::get(handle as u64) {
        let s = obj.to_string_box().value;
        string_to_c_string_ptr(s)
    } else {
        // not found -> print numeric handle string
        string_to_c_string_ptr(handle.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;

    fn to_string(ptr: *mut i8) -> String {
        assert!(!ptr.is_null());
        unsafe { CStr::from_ptr(ptr) }.to_string_lossy().into_owned()
    }

    #[test]
    fn concat_ss_keeps_ascii_contract() {
        let a = c"line-".as_ptr();
        let b = c"seed".as_ptr();
        let out = nyash_string_concat_ss(a, b);
        assert_eq!(to_string(out), "line-seed");
    }

    #[test]
    fn substring_sii_keeps_byte_slice_contract() {
        let src = c"line-seed-abcdef".as_ptr();
        let out = nyash_string_substring_sii(src, 5, 9);
        assert_eq!(to_string(out), "seed");
    }
}
