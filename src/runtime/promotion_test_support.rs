//! Test/perf-only input-wire issuer for caller-zero promotion evidence.
//!
//! This feature-gated adapter keeps generation capture in the host registry's
//! allocation transaction. It is absent from default and production builds.

use super::host_handles;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PromotionTestWireV1 {
    pub slot: u64,
    pub generation: u64,
}

pub fn issue_text_wire_v1(text: impl Into<String>) -> PromotionTestWireV1 {
    let (slot, identity) = host_handles::to_handle_text_with_lease_identity(text);
    PromotionTestWireV1 {
        slot,
        generation: identity.generation(),
    }
}

pub fn issue_non_text_wire_v1() -> PromotionTestWireV1 {
    let (slot, generation) = host_handles::issue_promotion_non_text_wire_v1();
    PromotionTestWireV1 { slot, generation }
}

pub fn drop_wire_v1(wire: PromotionTestWireV1) {
    host_handles::drop_handle(wire.slot);
}
