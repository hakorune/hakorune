use crate::bid::{BidError, BidResult};

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
    if crate::config::env::fail_fast() {
        if crate::config::env::dev_provider_trace() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[provider/trace] reject legacy file fallback box_type={} method={} reason=fail_fast",
                box_type, method_name
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

    #[test]
    fn compat_method_table_maps_known_entries() {
        assert_eq!(resolve_method_id_from_file("StringBox", "concat").unwrap(), 102);
        assert_eq!(resolve_method_id_from_file("StringBox", "upper").unwrap(), 103);
        assert_eq!(resolve_method_id_from_file("CounterBox", "inc").unwrap(), 102);
        assert_eq!(resolve_method_id_from_file("CounterBox", "get").unwrap(), 103);
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
}
