#![deny(unused_must_use)]

use libc::{c_uchar, size_t};

#[cfg(feature = "c-shim")]
extern "C" {
    fn ny_tlv_identity(in_ptr: *const c_uchar, len: size_t, out_ptr: *mut *mut c_uchar) -> size_t;
    fn ny_tlv_free(ptr: *mut c_uchar);
}

/// Round‑trip helper (identity): returns a freshly allocated copy of the input.
///
/// When built with `c-shim` feature, this calls the C implementation.
/// Otherwise, it falls back to a pure Rust copy (stub), preserving the public API.
pub fn tlv_roundtrip_identity(input: &[u8]) -> Vec<u8> {
    #[cfg(feature = "c-shim")]
    unsafe {
        let mut out_ptr: *mut c_uchar = std::ptr::null_mut();
        let sz = ny_tlv_identity(
            input.as_ptr(),
            input.len() as size_t,
            &mut out_ptr as *mut *mut c_uchar,
        );
        if sz == 0 || out_ptr.is_null() {
            return Vec::new();
        }
        let slice = std::slice::from_raw_parts(out_ptr, sz as usize);
        let v = slice.to_vec();
        ny_tlv_free(out_ptr);
        return v;
    }
    #[cfg(not(feature = "c-shim"))]
    {
        input.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn identity_roundtrip() {
        let src = b"hello tlv";
        let out = tlv_roundtrip_identity(src);
        assert_eq!(out, src);
    }
}
