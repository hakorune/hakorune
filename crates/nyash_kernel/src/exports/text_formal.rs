//! Fixed C status projection for the caller-zero Text formal lane.
//!
//! This bridge does not classify source types, recapture generations, or
//! provide a fallback.  It forwards the published slot/generation pair to
//! the Rust-owned validator and returns its exhaustive status.

use nyash_rust::runtime::text_formal_abi::validate_text_formal_wire_v1;

#[export_name = "hako_text_formal_validate_v1"]
pub extern "C" fn hako_text_formal_validate_v1(slot: u64, generation: u64) -> u32 {
    validate_text_formal_wire_v1(slot, generation).as_u32()
}

#[cfg(test)]
mod tests {
    use super::hako_text_formal_validate_v1;

    #[test]
    fn c_status_rejects_zero_wire() {
        assert_eq!(hako_text_formal_validate_v1(0, 0), 1);
    }
}
