use nyash_rust::abi::dynamic_call_slot_wire::DynamicV2CallOutV1;

#[test]
fn dynamic_v2_wire_projection_is_shared_without_kernel_duplicate() {
    assert_eq!(std::mem::size_of::<DynamicV2CallOutV1>(), 48);
    assert_eq!(std::mem::align_of::<DynamicV2CallOutV1>(), 8);
    assert_eq!(std::mem::offset_of!(DynamicV2CallOutV1, status), 0);
    assert_eq!(std::mem::offset_of!(DynamicV2CallOutV1, fault_code), 4);
    assert_eq!(std::mem::offset_of!(DynamicV2CallOutV1, result_tag), 8);
    assert_eq!(std::mem::offset_of!(DynamicV2CallOutV1, disposition), 12);
    assert_eq!(std::mem::offset_of!(DynamicV2CallOutV1, forwarded_input), 16);
    assert_eq!(std::mem::offset_of!(DynamicV2CallOutV1, reserved), 20);
    assert_eq!(std::mem::offset_of!(DynamicV2CallOutV1, value_payload), 24);
    assert_eq!(std::mem::offset_of!(DynamicV2CallOutV1, lease_token), 32);
    assert_eq!(std::mem::offset_of!(DynamicV2CallOutV1, continuation_token), 40);
}
