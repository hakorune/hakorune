// Host bridge helpers for TypeBox invoke (v2)

// Library-level shim signature used across the runtime (compat convenience)
pub type InvokeFn = unsafe extern "C" fn(
    u32, /* type_id (for dispatch) */
    u32, /* method_id */
    u32, /* instance_id */
    *const u8,
    usize,
    *mut u8,
    *mut usize,
) -> i32;

// Native v2 per-Box signature
pub type BoxInvokeFn = extern "C" fn(
    u32, /* instance_id */
    u32, /* method_id */
    *const u8,
    usize,
    *mut u8,
    *mut usize,
) -> i32;

// Call library-level shim with a temporary output buffer
pub fn invoke_alloc(
    invoke: InvokeFn,
    type_id: u32,
    method_id: u32,
    instance_id: u32,
    tlv_args: &[u8],
) -> (i32, usize, Vec<u8>) {
    let mut out = vec![0u8; 1024];
    let mut out_len: usize = out.len();
    let code = unsafe {
        invoke(
            type_id,
            method_id,
            instance_id,
            tlv_args.as_ptr(),
            tlv_args.len(),
            out.as_mut_ptr(),
            &mut out_len,
        )
    };
    (code, out_len, out)
}

// Call per-Box invoke directly with a temporary output buffer
pub fn invoke_alloc_box(
    invoke: BoxInvokeFn,
    method_id: u32,
    instance_id: u32,
    tlv_args: &[u8],
) -> (i32, usize, Vec<u8>) {
    let mut out = vec![0u8; 1024];
    let mut out_len: usize = out.len();
    let code = invoke(
        instance_id,
        method_id,
        tlv_args.as_ptr(),
        tlv_args.len(),
        out.as_mut_ptr(),
        &mut out_len,
    );
    (code, out_len, out)
}

// Prefer per-Box invoke on mainline; shim fallback is compat-only.
pub fn invoke_alloc_with_route(
    invoke_box: Option<BoxInvokeFn>,
    invoke_shim: InvokeFn,
    allow_compat_shim: bool,
    type_id: u32,
    method_id: u32,
    instance_id: u32,
    tlv_args: &[u8],
) -> (i32, usize, Vec<u8>) {
    if let Some(invoke) = invoke_box {
        return invoke_alloc_box(invoke, method_id, instance_id, tlv_args);
    }
    if !allow_compat_shim {
        // Keep E_PLUGIN parity with nyash_plugin_invoke_v2_shim when no route exists.
        return (-5, 0, Vec::new());
    }
    super::compat_host_bridge::invoke_alloc_compat(
        invoke_shim,
        type_id,
        method_id,
        instance_id,
        tlv_args,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    fn with_env_vars<F: FnOnce()>(pairs: &[(&str, &str)], f: F) {
        let _guard = ENV_LOCK.lock().expect("env lock");
        let prev: Vec<(String, Option<String>)> = pairs
            .iter()
            .map(|(k, _)| ((*k).to_string(), std::env::var(k).ok()))
            .collect();
        for (k, v) in pairs {
            std::env::set_var(k, v);
        }
        f();
        for (k, prev_v) in prev {
            if let Some(v) = prev_v {
                std::env::set_var(&k, v);
            } else {
                std::env::remove_var(&k);
            }
        }
    }

    unsafe extern "C" fn compat_shim(
        _type_id: u32,
        _method_id: u32,
        _instance_id: u32,
        _args: *const u8,
        _args_len: usize,
        _result: *mut u8,
        _result_len: *mut usize,
    ) -> i32 {
        17
    }

    #[test]
    fn invoke_alloc_with_route_rejects_compat_shim_when_route_disallows_it() {
        with_env_vars(
            &[("NYASH_FAIL_FAST", "0"), ("NYASH_VM_USE_FALLBACK", "1")],
            || {
                let got = invoke_alloc_with_route(None, compat_shim, false, 42, 7, 1, &[]);
                assert_eq!(got.0, -5);
            },
        );
    }

    #[test]
    fn invoke_alloc_with_route_uses_compat_shim_when_explicitly_allowed() {
        with_env_vars(
            &[("NYASH_FAIL_FAST", "1"), ("NYASH_VM_USE_FALLBACK", "0")],
            || {
                let got = invoke_alloc_with_route(None, compat_shim, true, 42, 7, 1, &[]);
                assert_eq!(got.0, 17);
            },
        );
    }
}
