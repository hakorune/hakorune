//! Durable backend-capability witness for canonical direct static calls.
//!
//! The row is explicit metadata. Backend admission must not infer it by
//! scanning generic `MirInstruction::Call` instructions, source names, or
//! parameter/return contracts.

pub(crate) const CANONICAL_DIRECT_STATIC_CALL_CAPABILITY_V1: &str =
    "canonical_direct_static_call_v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CanonicalDirectStaticCallCapabilityV1 {
    schema_version: u8,
}

impl CanonicalDirectStaticCallCapabilityV1 {
    pub(crate) const fn v1() -> Self {
        Self { schema_version: 1 }
    }

    pub(crate) const fn schema_version(self) -> u8 {
        self.schema_version
    }
}
