//! Feature-gated C adapter for offline pinned-corridor promotion tests.

use std::ffi::CStr;
use std::os::raw::c_char;

use nyash_rust::runtime::promotion_test_support::{
    drop_wire_v1, issue_non_text_wire_v1, issue_text_wire_v1, PromotionTestWireV1,
};

#[repr(C)]
pub struct HakoPromotionTestWireV1 {
    pub slot: u64,
    pub generation: u64,
}

impl From<PromotionTestWireV1> for HakoPromotionTestWireV1 {
    fn from(wire: PromotionTestWireV1) -> Self {
        Self {
            slot: wire.slot,
            generation: wire.generation,
        }
    }
}

#[export_name = "hako_promotion_test_issue_text_wire_v1"]
pub unsafe extern "C" fn issue_text_wire(text: *const c_char) -> HakoPromotionTestWireV1 {
    if text.is_null() {
        return HakoPromotionTestWireV1 {
            slot: 0,
            generation: 0,
        };
    }
    let text = CStr::from_ptr(text).to_string_lossy().into_owned();
    issue_text_wire_v1(text).into()
}

#[export_name = "hako_promotion_test_issue_non_text_wire_v1"]
pub extern "C" fn issue_non_text_wire() -> HakoPromotionTestWireV1 {
    issue_non_text_wire_v1().into()
}

#[export_name = "hako_promotion_test_drop_wire_v1"]
pub extern "C" fn drop_wire(wire: HakoPromotionTestWireV1) {
    drop_wire_v1(PromotionTestWireV1 {
        slot: wire.slot,
        generation: wire.generation,
    });
}
