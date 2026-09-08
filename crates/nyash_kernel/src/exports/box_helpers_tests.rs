use super::*;

#[test]
fn length_aware_literal_preserves_nul_utf8_and_empty_text() {
    for text in ["", "prefix\0猫\0suffix"] {
        let handle =
            unsafe { nyash_box_from_i8_string_const_len_v1(text.as_ptr(), text.len() as u64) };
        assert!(handle > 0);
        let object = handles::get(handle as u64).unwrap();
        assert_eq!(
            object.as_any().downcast_ref::<StringBox>().unwrap().value,
            text
        );
        assert_eq!(handle, string_literal_handle_from_text(text));
    }
}

#[test]
fn length_aware_literal_rejects_invalid_utf8_null_and_oversized_length() {
    let bad = [0xff];
    assert_eq!(
        unsafe { nyash_box_from_i8_string_const_len_v1(bad.as_ptr(), 1) },
        0
    );
    assert_eq!(
        unsafe { nyash_box_from_i8_string_const_len_v1(std::ptr::null(), 0) },
        0
    );
    assert_eq!(
        unsafe { nyash_box_from_i8_string_const_len_v1(bad.as_ptr(), u64::MAX) },
        0
    );
}
