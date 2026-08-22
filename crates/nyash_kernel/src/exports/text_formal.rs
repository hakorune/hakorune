//! Fixed C status projection for the caller-zero Text formal lane.
//!
//! This bridge does not classify source types, recapture generations, or
//! provide a fallback.  It forwards the published slot/generation pair to
//! the Rust-owned validator and returns its exhaustive status.

use nyash_rust::runtime::text_formal_abi::validate_text_formal_wire_v1;
use nyash_rust::runtime::text_formal_residence::{
    enter_text_formal_residence_c_v1, finish_text_formal_residence_or_abort_v1,
    TextFormalResidenceFrameHeaderV1,
};

#[export_name = "hako_text_formal_validate_v1"]
pub extern "C" fn hako_text_formal_validate_v1(slot: u64, generation: u64) -> u32 {
    validate_text_formal_wire_v1(slot, generation).as_u32()
}

/// Private caller-zero frame entry.  The frame is a backend-owned buffer;
/// runtime keeps only the opaque lease record carried in its header.  This
/// `extern "C"` boundary returns an exhaustive status and never unwinds.
#[export_name = "hako_text_formal_residence_enter_v1"]
pub unsafe extern "C" fn hako_text_formal_residence_enter_v1(
    pairs: *const nyash_rust::runtime::text_formal_abi::TextFormalBorrowV1,
    pair_count: u32,
    frame: *mut TextFormalResidenceFrameHeaderV1,
    frame_bytes: u32,
) -> u32 {
    enter_text_formal_residence_c_v1(pairs, pair_count, frame, frame_bytes)
}

/// Private caller-zero frame finish.  A successful call consumes the
/// residence token and clears it from the frame; every nonzero status
/// fail-stops inside the runtime without exposing caller CFG.  Successful
/// completion returns `void`; neither outcome unwinds through this boundary.
#[export_name = "hako_text_formal_residence_finish_or_abort_v1"]
pub unsafe extern "C" fn hako_text_formal_residence_finish_or_abort_v1(
    frame: *mut TextFormalResidenceFrameHeaderV1,
) {
    finish_text_formal_residence_or_abort_v1(frame)
}

#[cfg(test)]
mod tests {
    use super::hako_text_formal_validate_v1;

    #[test]
    fn c_status_rejects_zero_wire() {
        assert_eq!(hako_text_formal_validate_v1(0, 0), 1);
    }
}
