use crate::bid::{BidError, BidResult};

fn compat_method_fallback_enabled() -> bool {
    !crate::config::env::fail_fast() && crate::config::env::vm_compat_fallback_allowed()
}

pub(super) fn resolve_method_id_from_file(box_type: &str, method_name: &str) -> BidResult<u32> {
    match (box_type, method_name) {
        ("StringBox", "concat") => Ok(102),
        ("StringBox", "upper") => Ok(103),
        ("CounterBox", "inc") => Ok(102),
        ("CounterBox", "get") => Ok(103),
        _ => Err(BidError::InvalidMethod),
    }
}

pub(super) fn resolve_method_id_with_compat_policy(
    box_type: &str,
    method_name: &str,
) -> BidResult<u32> {
    if !compat_method_fallback_enabled() {
        if crate::config::env::dev_provider_trace() {
            let ring0 = crate::runtime::get_global_ring0();
            let reason = if crate::config::env::fail_fast() {
                "fail_fast"
            } else {
                "compat_disabled"
            };
            ring0.log.debug(&format!(
                "[provider/trace] reject legacy file fallback box_type={} method={} reason={}",
                box_type, method_name, reason
            ));
        }
        return Err(BidError::InvalidMethod);
    }

    if crate::config::env::dev_provider_trace() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!(
            "[provider/trace] compat legacy file fallback box_type={} method={}",
            box_type, method_name
        ));
    }
    resolve_method_id_from_file(box_type, method_name)
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

    #[test]
    fn compat_method_table_maps_known_entries() {
        assert_eq!(
            resolve_method_id_from_file("StringBox", "concat").unwrap(),
            102
        );
        assert_eq!(
            resolve_method_id_from_file("StringBox", "upper").unwrap(),
            103
        );
        assert_eq!(
            resolve_method_id_from_file("CounterBox", "inc").unwrap(),
            102
        );
        assert_eq!(
            resolve_method_id_from_file("CounterBox", "get").unwrap(),
            103
        );
    }

    #[test]
    fn compat_method_table_rejects_unknown_entries() {
        assert!(matches!(
            resolve_method_id_from_file("StringBox", "missing"),
            Err(BidError::InvalidMethod)
        ));
        assert!(matches!(
            resolve_method_id_from_file("UnknownBox", "concat"),
            Err(BidError::InvalidMethod)
        ));
    }

    #[test]
    fn compat_method_policy_respects_vm_fallback_flag() {
        with_env_vars(
            &[("NYASH_FAIL_FAST", "0"), ("NYASH_VM_USE_FALLBACK", "1")],
            || {
                let got =
                    resolve_method_id_with_compat_policy("StringBox", "concat").expect("compat");
                assert_eq!(got, 102);
            },
        );
        with_env_vars(
            &[("NYASH_FAIL_FAST", "0"), ("NYASH_VM_USE_FALLBACK", "0")],
            || {
                let got = resolve_method_id_with_compat_policy("StringBox", "concat");
                assert!(matches!(got, Err(BidError::InvalidMethod)));
            },
        );
    }
}
