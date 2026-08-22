use crate::exports::dynamic_v2_text_scan::{
    hako_text_scan_index_of_v1, hako_text_scan_substring_v1,
};
use nyash_rust::abi::dynamic_call_slot_wire::{
    DynamicV2CallDispositionV1, DynamicV2CallFaultCodeV1, DynamicV2CallOutV1,
    DynamicV2CallStatusV1, DynamicV2WireTagV1, DYNAMIC_V2_FORWARDED_NONE_V1,
};
use nyash_rust::runtime::host_handles;

fn empty_out() -> DynamicV2CallOutV1 {
    DynamicV2CallOutV1 {
        status: 0,
        fault_code: 0,
        result_tag: 0,
        disposition: DynamicV2CallDispositionV1::None as u32,
        forwarded_input: DYNAMIC_V2_FORWARDED_NONE_V1,
        reserved: 0,
        value_payload: 0,
        lease_token: 0,
        continuation_token: 0,
    }
}

#[test]
fn strict_text_scan_exports_are_linked_and_typed() {
    let source = host_handles::to_handle_text("aé界");
    let needle = host_handles::to_handle_text("界");
    let mut out = empty_out();
    assert_eq!(hako_text_scan_index_of_v1(source, needle, &mut out), 0);
    assert_eq!(out.status, DynamicV2CallStatusV1::Normal as u32);
    assert_eq!(out.result_tag, DynamicV2WireTagV1::ImmediateI64 as u32);
    assert_eq!(out.value_payload as i64, 2);
    assert_eq!(out.lease_token, 0);
    assert_eq!(out.validate_for_synchronous_emitter(), Ok(()));
    assert_eq!(hako_text_scan_substring_v1(source, 1, 3, &mut out), 0);
    assert_eq!(out.result_tag, DynamicV2WireTagV1::HostHandle as u32);
    assert_eq!(
        out.disposition,
        DynamicV2CallDispositionV1::EndAuthorized as u32
    );
    let result = out.value_payload;
    assert_eq!(
        host_handles::with_str_handle(result, str::to_owned),
        Some("é界".to_string())
    );
    let token = std::num::NonZeroU64::new(out.lease_token).expect("lease");
    assert_eq!(
        nyash_rust::runtime::dynamic_v2_lease::consume_end_authorized(token),
        Ok(())
    );
    host_handles::drop_handle(source);
    host_handles::drop_handle(needle);
}

#[test]
fn malformed_receiver_is_fault_not_generic_fallback() {
    let mut out = empty_out();
    assert_eq!(hako_text_scan_substring_v1(0, 0, 1, &mut out), 0);
    assert_eq!(out.status, DynamicV2CallStatusV1::Fault as u32);
    assert_eq!(
        out.fault_code,
        DynamicV2CallFaultCodeV1::InvalidReceiver as u32
    );
    assert_eq!(out.result_tag, DynamicV2WireTagV1::Invalid as u32);
    assert_eq!(out.lease_token, 0);
}
