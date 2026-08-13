//! Neutral C ABI projection for the DynamicV2 one-shot End boundary.
//!
//! This adapter owns no lease state.  It converts the fixed-width C input and
//! status into the existing Rust `dynamic_v2_lease` owner.  Boundary lowering
//! owns the later policy that a non-zero status is a backend contract failure.

use std::num::NonZeroU64;

const CONSUME_OK: u32 = 0;
const CONSUME_INVALID_TOKEN: u32 = 1;
const CONSUME_UNKNOWN_OR_ALREADY_CONSUMED: u32 = 2;
const CONSUME_STALE_HANDLE_IDENTITY: u32 = 3;

#[export_name = "nyrt_dynamic_v2_lease_consume_end_authorized_v1"]
pub extern "C" fn nyrt_dynamic_v2_lease_consume_end_authorized_v1(lease_token: u64) -> u32 {
    let Some(token) = NonZeroU64::new(lease_token) else {
        return CONSUME_INVALID_TOKEN;
    };

    match nyash_rust::runtime::dynamic_v2_lease::consume_end_authorized(token) {
        Ok(()) => CONSUME_OK,
        Err(
            nyash_rust::runtime::dynamic_v2_lease::LeaseConsumeRejectV1::UnknownOrAlreadyConsumed,
        ) => CONSUME_UNKNOWN_OR_ALREADY_CONSUMED,
        Err(nyash_rust::runtime::dynamic_v2_lease::LeaseConsumeRejectV1::StaleHandleIdentity) => {
            CONSUME_STALE_HANDLE_IDENTITY
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nyash_rust::runtime::dynamic_v2_lease;

    #[test]
    fn zero_token_is_rejected_before_runtime_owner() {
        assert_eq!(
            nyrt_dynamic_v2_lease_consume_end_authorized_v1(0),
            CONSUME_INVALID_TOKEN
        );
    }

    #[test]
    fn valid_token_is_consumed_once_and_duplicate_is_rejected() {
        let published = dynamic_v2_lease::publish_end_authorized_text("ffi-end");
        let token = published.expect("lease publication").token().get();
        assert_eq!(
            nyrt_dynamic_v2_lease_consume_end_authorized_v1(token),
            CONSUME_OK
        );
        assert_eq!(
            nyrt_dynamic_v2_lease_consume_end_authorized_v1(token),
            CONSUME_UNKNOWN_OR_ALREADY_CONSUMED
        );
    }

    #[test]
    fn foreign_token_maps_to_unknown_status() {
        assert_eq!(
            nyrt_dynamic_v2_lease_consume_end_authorized_v1(u64::MAX),
            CONSUME_UNKNOWN_OR_ALREADY_CONSUMED
        );
    }

    #[test]
    fn status_projection_matches_rust_reject_variants() {
        assert_eq!(CONSUME_OK, 0);
        assert_eq!(CONSUME_INVALID_TOKEN, 1);
        assert_eq!(CONSUME_UNKNOWN_OR_ALREADY_CONSUMED, 2);
        assert_eq!(CONSUME_STALE_HANDLE_IDENTITY, 3);
    }
}
