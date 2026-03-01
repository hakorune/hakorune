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
