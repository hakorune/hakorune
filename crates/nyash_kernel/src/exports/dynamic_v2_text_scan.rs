//! Strict CodePoint TextScan entries for the selected AOT lane.
//!
//! This module is intentionally separate from the generic String exports:
//! there is no environment mode, compatibility forwarding, parse fallback,
//! selector lookup, or Rust-VM route here.

use std::num::NonZeroU64;

use nyash_rust::abi::dynamic_call_slot_wire::{
    DynamicV2CallDispositionV1, DynamicV2CallFaultCodeV1, DynamicV2CallOutV1,
    DynamicV2CallStatusV1, DynamicV2WireTagV1, DYNAMIC_V2_FORWARDED_NONE_V1,
};
use nyash_rust::abi::text_scan_aot_export_facts::{
    TEXT_SCAN_CALL_INVALID_OUTPUT_V1, TEXT_SCAN_CALL_OK_V1,
};
use nyash_rust::boxes::string_ops::{self, StringIndexMode};
use nyash_rust::runtime::{dynamic_v2_lease, host_handles};

fn fault(out: *mut DynamicV2CallOutV1, code: DynamicV2CallFaultCodeV1) -> u32 {
    if out.is_null() {
        return TEXT_SCAN_CALL_INVALID_OUTPUT_V1;
    }
    // SAFETY: the null check above establishes a writable caller-provided
    // out-parameter for the fixed C ABI.
    unsafe {
        out.write(DynamicV2CallOutV1 {
            status: DynamicV2CallStatusV1::Fault as u32,
            fault_code: code as u32,
            result_tag: DynamicV2WireTagV1::Invalid as u32,
            disposition: DynamicV2CallDispositionV1::None as u32,
            forwarded_input: DYNAMIC_V2_FORWARDED_NONE_V1,
            reserved: 0,
            value_payload: 0,
            lease_token: 0,
            continuation_token: 0,
        });
    }
    TEXT_SCAN_CALL_OK_V1
}

fn normal_host_handle(
    out: *mut DynamicV2CallOutV1,
    handle: u64,
    lease: NonZeroU64,
) -> u32 {
    if out.is_null() {
        return TEXT_SCAN_CALL_INVALID_OUTPUT_V1;
    }
    // SAFETY: the null check above establishes a writable caller-provided
    // out-parameter for the fixed C ABI.
    unsafe {
        out.write(DynamicV2CallOutV1 {
            status: DynamicV2CallStatusV1::Normal as u32,
            fault_code: 0,
            result_tag: DynamicV2WireTagV1::HostHandle as u32,
            disposition: DynamicV2CallDispositionV1::EndAuthorized as u32,
            forwarded_input: DYNAMIC_V2_FORWARDED_NONE_V1,
            reserved: 0,
            value_payload: handle,
            lease_token: lease.get(),
            continuation_token: 0,
        });
    }
    TEXT_SCAN_CALL_OK_V1
}

fn normal_i64(out: *mut DynamicV2CallOutV1, value: i64) -> u32 {
    if out.is_null() {
        return TEXT_SCAN_CALL_INVALID_OUTPUT_V1;
    }
    // SAFETY: the null check above establishes a writable caller-provided
    // out-parameter for the fixed C ABI.
    unsafe {
        out.write(DynamicV2CallOutV1 {
            status: DynamicV2CallStatusV1::Normal as u32,
            fault_code: 0,
            result_tag: DynamicV2WireTagV1::ImmediateI64 as u32,
            disposition: DynamicV2CallDispositionV1::None as u32,
            forwarded_input: DYNAMIC_V2_FORWARDED_NONE_V1,
            reserved: 0,
            value_payload: value as u64,
            lease_token: 0,
            continuation_token: 0,
        });
    }
    TEXT_SCAN_CALL_OK_V1
}

/// `substring(receiver_u64, start_i64, end_i64, out)` with fixed CodePoint
/// and clamped range semantics.
#[export_name = "hako.text.scan.substring.v1"]
pub extern "C" fn hako_text_scan_substring_v1(
    receiver: u64,
    start: i64,
    end: i64,
    out: *mut DynamicV2CallOutV1,
) -> u32 {
    if out.is_null() {
        return TEXT_SCAN_CALL_INVALID_OUTPUT_V1;
    }
    let Some(text) = host_handles::with_str_handle_ready(receiver, str::to_owned) else {
        return fault(out, DynamicV2CallFaultCodeV1::InvalidReceiver);
    };
    let result = string_ops::substring(&text, start, Some(end), StringIndexMode::CodePoint);
    let Ok(published) = dynamic_v2_lease::publish_end_authorized_text(result) else {
        return fault(out, DynamicV2CallFaultCodeV1::Runtime);
    };
    normal_host_handle(out, published.handle(), published.token())
}

/// `indexOf(receiver_u64, needle_u64, out)` with fixed CodePoint semantics.
#[export_name = "hako.text.scan.index_of.v1"]
pub extern "C" fn hako_text_scan_index_of_v1(
    receiver: u64,
    needle: u64,
    out: *mut DynamicV2CallOutV1,
) -> u32 {
    if out.is_null() {
        return TEXT_SCAN_CALL_INVALID_OUTPUT_V1;
    }
    let Some(value) = host_handles::with_str_pair(receiver, needle, |haystack, needle| {
        string_ops::index_of(haystack, needle, None, StringIndexMode::CodePoint)
    }) else {
        return fault(out, DynamicV2CallFaultCodeV1::InvalidReceiver);
    };
    normal_i64(out, value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nyash_rust::runtime::host_handles;

    fn empty_out() -> DynamicV2CallOutV1 {
        DynamicV2CallOutV1 {
            status: 0,
            fault_code: 0,
            result_tag: 0,
            disposition: 0,
            forwarded_input: DYNAMIC_V2_FORWARDED_NONE_V1,
            reserved: 0,
            value_payload: 0,
            lease_token: 0,
            continuation_token: 0,
        }
    }

    #[test]
    fn substring_is_codepoint_and_end_authorized() {
        let source = host_handles::to_handle_text("aé界z");
        let mut out = empty_out();
        assert_eq!(hako_text_scan_substring_v1(source, 1, 3, &mut out), 0);
        assert_eq!(out.validate_for_synchronous_emitter(), Ok(()));
        assert_eq!(out.result_tag, DynamicV2WireTagV1::HostHandle as u32);
        assert_eq!(host_handles::with_str_handle(out.value_payload, str::to_owned), Some("é界".to_string()));
        let token = NonZeroU64::new(out.lease_token).expect("I6 lease");
        assert_eq!(dynamic_v2_lease::consume_end_authorized(token), Ok(()));
        host_handles::drop_handle(source);
    }

    #[test]
    fn index_of_is_codepoint_i64_and_zero_is_normal() {
        let source = host_handles::to_handle_text("aé界");
        let needle = host_handles::to_handle_text("é");
        let mut out = empty_out();
        assert_eq!(hako_text_scan_index_of_v1(source, needle, &mut out), 0);
        assert_eq!(out.validate_for_synchronous_emitter(), Ok(()));
        assert_eq!(out.result_tag, DynamicV2WireTagV1::ImmediateI64 as u32);
        assert_eq!(out.value_payload as i64, 1);
        assert_eq!(out.lease_token, 0);
        host_handles::drop_handle(source);
        host_handles::drop_handle(needle);
    }

    #[test]
    fn invalid_and_null_inputs_are_terminal() {
        let mut out = empty_out();
        assert_eq!(hako_text_scan_substring_v1(0, 0, 1, &mut out), 0);
        assert_eq!(out.status, DynamicV2CallStatusV1::Fault as u32);
        assert_eq!(out.fault_code, DynamicV2CallFaultCodeV1::InvalidReceiver as u32);
        assert_eq!(out.validate_for_synchronous_emitter(), Ok(()));
        assert_eq!(hako_text_scan_index_of_v1(0, 0, std::ptr::null_mut()), 1);
    }

    #[test]
    fn substring_clamps_ranges_and_index_of_not_found_is_minus_one() {
        let source = host_handles::to_handle_text("é界");
        let needle = host_handles::to_handle_text("x");
        let mut out = empty_out();
        assert_eq!(hako_text_scan_substring_v1(source, -5, 99, &mut out), 0);
        let result = out.value_payload;
        let token = NonZeroU64::new(out.lease_token).expect("lease");
        assert_eq!(host_handles::with_str_handle(result, str::to_owned), Some("é界".to_string()));
        assert_eq!(dynamic_v2_lease::consume_end_authorized(token), Ok(()));
        assert_eq!(hako_text_scan_index_of_v1(source, needle, &mut out), 0);
        assert_eq!(out.value_payload as i64, -1);
        assert_eq!(out.lease_token, 0);
        host_handles::drop_handle(source);
        host_handles::drop_handle(needle);
    }
}
