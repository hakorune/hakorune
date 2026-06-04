#[inline]
pub(crate) fn c_string_text<'a>(ptr: *const i8) -> Option<&'a str> {
    if ptr.is_null() {
        return None;
    }
    let c = unsafe { std::ffi::CStr::from_ptr(ptr) };
    c.to_str().ok()
}

#[inline]
pub(crate) fn c_string_bytes<'a>(ptr: *const i8) -> &'a [u8] {
    if ptr.is_null() {
        return &[];
    }
    unsafe { std::ffi::CStr::from_ptr(ptr).to_bytes() }
}

#[cfg(test)]
pub(crate) fn cstring(text: &str) -> std::ffi::CString {
    std::ffi::CString::new(text).expect("CString")
}
