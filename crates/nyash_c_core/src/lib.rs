#[allow(non_camel_case_types)]
type c_int = i32;

extern "C" {
    fn ny_core_probe_invoke(target: *const u8, method: *const u8, argc: c_int) -> c_int;
    fn ny_core_map_set(type_id: i32, instance_id: u32, key: *const u8, val: *const u8) -> c_int;
    fn ny_core_array_push(type_id: i32, instance_id: u32, val: i64) -> c_int;
    fn ny_core_array_get(type_id: i32, instance_id: u32, idx: i64) -> c_int;
    fn ny_core_array_len(type_id: i32, instance_id: u32) -> c_int;
}

/// Safe wrapper for core probe invoke (design-stage)
pub fn core_probe_invoke(target: &str, method: &str, argc: i32) -> i32 {
    let t = std::ffi::CString::new(target).unwrap_or_else(|_| std::ffi::CString::new("?").unwrap());
    let m = std::ffi::CString::new(method).unwrap_or_else(|_| std::ffi::CString::new("?").unwrap());
    unsafe {
        ny_core_probe_invoke(
            t.as_ptr() as *const u8,
            m.as_ptr() as *const u8,
            argc as c_int,
        ) as i32
    }
}

/// MapBox.set stub (design-stage): returns 0 on success
pub fn core_map_set(type_id: i32, instance_id: u32, key: &str, val: &str) -> i32 {
    let k = std::ffi::CString::new(key).unwrap_or_else(|_| std::ffi::CString::new("").unwrap());
    let v = std::ffi::CString::new(val).unwrap_or_else(|_| std::ffi::CString::new("").unwrap());
    unsafe {
        ny_core_map_set(
            type_id as i32,
            instance_id as u32,
            k.as_ptr() as *const u8,
            v.as_ptr() as *const u8,
        ) as i32
    }
}

/// ArrayBox.push stub (design-stage): returns 0 on success
pub fn core_array_push(type_id: i32, instance_id: u32, val: i64) -> i32 {
    unsafe { ny_core_array_push(type_id as i32, instance_id as u32, val as i64) as i32 }
}

pub fn core_array_get(type_id: i32, instance_id: u32, idx: i64) -> i32 {
    unsafe { ny_core_array_get(type_id as i32, instance_id as u32, idx as i64) as i32 }
}

pub fn core_array_len(type_id: i32, instance_id: u32) -> i32 {
    unsafe { ny_core_array_len(type_id as i32, instance_id as u32) as i32 }
}
