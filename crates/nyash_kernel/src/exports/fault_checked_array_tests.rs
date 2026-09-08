use super::*;
use crate::nyrt_handle_release_h;

// Exercise linked C ABI symbols rather than invoking private Rust functions.
mod wire {
    use std::ffi::c_void;
    extern "C" {
        #[link_name = "nyash.array.checked_new_v1"]
        pub fn new(frame: *mut c_void, site: u64, out: *mut i64) -> u32;
        #[link_name = "nyash.array.checked_claim_v1"]
        pub fn claim(frame: *mut c_void, site: u64, handle: i64, tag: u32) -> u32;
        #[link_name = "nyash.array.checked_append_i64_v1"]
        pub fn i64(frame: *mut c_void, site: u64, handle: i64, value: i64) -> u32;
        #[link_name = "nyash.array.checked_append_bool_v1"]
        pub fn bool(frame: *mut c_void, site: u64, handle: i64, value: u32) -> u32;
        #[link_name = "nyash.array.checked_append_f64_v1"]
        pub fn f64(frame: *mut c_void, site: u64, handle: i64, value: f64) -> u32;
    }
}

fn pointer(frame: &mut FaultFrame) -> *mut c_void {
    (frame as *mut FaultFrame).cast()
}

struct OwnedArray(i64);
impl OwnedArray {
    fn new(frame: &mut FaultFrame) -> Self {
        let mut handle = -91;
        assert_eq!(unsafe { wire::new(pointer(frame), 10, &mut handle) }, 0);
        assert!(handle > 0);
        Self(handle)
    }

    fn values(&self) -> Vec<String> {
        with_array_box_direct(self.0, |array| {
            (0..array.len())
                .map(|index| array.get_index_i64(index as i64).to_string_box().value)
                .collect()
        })
        .unwrap()
    }
}
impl Drop for OwnedArray {
    fn drop(&mut self) {
        nyrt_handle_release_h(self.0);
    }
}

fn primary(frame: &FaultFrame) -> (u32, u64, [i64; 2]) {
    let diagnostic = frame.diagnostics().unwrap().0.unwrap();
    (diagnostic.reason, diagnostic.site, diagnostic.details)
}

#[test]
fn checked_array_seven_wire_tags_adopt_and_preserve_exact_rejections() {
    let _guard = crate::test_support::handle_registry_test_lock();
    for (tag, element, low, high) in [
        (1, ExactArrayElementType::I8, -128, 127),
        (2, ExactArrayElementType::I16, -32768, 32767),
        (
            3,
            ExactArrayElementType::I32,
            i32::MIN as i64,
            i32::MAX as i64,
        ),
        (4, ExactArrayElementType::I64, i64::MIN, i64::MAX),
        (5, ExactArrayElementType::U8, 0, 255),
        (6, ExactArrayElementType::U16, 0, 65535),
        (7, ExactArrayElementType::U32, 0, u32::MAX as i64),
    ] {
        assert_eq!(element_spec(tag).unwrap().element, element);
        let mut frame = FaultFrame::new();
        let array = OwnedArray::new(&mut frame);
        let ptr = pointer(&mut frame);
        unsafe {
            assert_eq!(wire::i64(ptr, 20, array.0, low), 0);
            assert_eq!(wire::i64(ptr, 21, array.0, high), 0);
            assert_eq!(wire::claim(ptr, 22, array.0, tag), 0);
            assert_eq!(wire::claim(ptr, 23, array.0, tag), 0);
        }
        let before = array.values();
        for invalid in [low.checked_sub(1), high.checked_add(1)]
            .into_iter()
            .flatten()
        {
            let mut rejected = FaultFrame::new();
            assert_eq!(
                unsafe { wire::i64(pointer(&mut rejected), 30, array.0, invalid) },
                1
            );
            assert_eq!(
                primary(&rejected),
                (202, 30, [if low == 0 && invalid < 0 { 2 } else { 3 }, 0,])
            );
            assert_eq!(array.values(), before);
        }
        assert_eq!(unsafe { wire::bool(ptr, 31, array.0, 1) }, 1);
        assert_eq!(primary(&frame), (202, 31, [1, 0]));
        assert_eq!(unsafe { wire::f64(ptr, 32, array.0, 1.0) }, 1);
        assert_eq!(array.values(), before);
        let conflict_tag = if tag == 1 { 2 } else { 1 };
        let mut conflict = FaultFrame::new();
        assert_eq!(
            unsafe { wire::claim(pointer(&mut conflict), 40, array.0, conflict_tag) },
            1
        );
        assert_eq!(primary(&conflict), (200, 40, [i64::from(conflict_tag), 0]));
        assert_eq!(unsafe { wire::claim(ptr, 41, array.0, tag) }, 0);
        assert_eq!(unsafe { wire::i64(ptr, 42, array.0, low) }, 0);
        assert_eq!(
            primary(&frame),
            (202, 31, [1, 0]),
            "success retains first Fault"
        );
    }
}

#[test]
fn checked_array_payloads_keep_bool_f64_and_integer_distinct() {
    let _guard = crate::test_support::handle_registry_test_lock();
    let mut frame = FaultFrame::new();
    let array = OwnedArray::new(&mut frame);
    let ptr = pointer(&mut frame);
    // Positive integers remain values even when identical to an allocated handle.
    assert_eq!(unsafe { wire::i64(ptr, 1, array.0, array.0) }, 0);
    for value in [0, 1] {
        assert_eq!(unsafe { wire::bool(ptr, 2, array.0, value) }, 0);
    }
    let floats = [
        0.0,
        -0.0,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::from_bits(0x7ff8_0000_0000_0042),
    ];
    for value in floats {
        assert_eq!(unsafe { wire::f64(ptr, 3, array.0, value) }, 0);
    }
    let expected_integer = array.0;
    with_array_box_direct(array.0, |array| {
        assert_eq!(array.get_index_i64(0).as_i64_fast(), Some(expected_integer));
        assert_eq!(array.get_index_i64(1).as_bool_fast(), Some(false));
        assert_eq!(array.get_index_i64(2).as_bool_fast(), Some(true));
        for (index, expected) in floats.into_iter().enumerate() {
            assert_eq!(
                array
                    .get_index_i64(index as i64 + 3)
                    .as_f64_fast()
                    .unwrap()
                    .to_bits(),
                expected.to_bits()
            );
        }
    })
    .unwrap();
    assert!(frame.diagnostics().unwrap().0.is_none());
}

#[test]
fn checked_array_failed_adoption_does_not_install_a_contract() {
    let _guard = crate::test_support::handle_registry_test_lock();
    for (payload, tag, subtype) in [(0, 1, 1), (1, 1, 3), (2, 5, 2), (3, 1, 1)] {
        let mut frame = FaultFrame::new();
        let array = OwnedArray::new(&mut frame);
        let ptr = pointer(&mut frame);
        assert_eq!(unsafe { wire::i64(ptr, 1, array.0, 7) }, 0);
        let result = unsafe {
            match payload {
                0 => wire::bool(ptr, 2, array.0, 1),
                1 => wire::i64(ptr, 2, array.0, 128),
                2 => wire::i64(ptr, 2, array.0, -1),
                _ => wire::f64(ptr, 2, array.0, 1.0),
            }
        };
        assert_eq!(result, 0);
        let before = array.values();
        assert_eq!(unsafe { wire::claim(ptr, 3, array.0, tag) }, 1);
        assert_eq!(primary(&frame), (201, 3, [1, subtype]));
        assert_eq!(array.values(), before);
        // A rejected claim cannot leave a hidden installed integer restriction.
        assert_eq!(unsafe { wire::bool(ptr, 4, array.0, 0) }, 0);
    }
}

#[test]
fn checked_array_malformed_inputs_preserve_out_frame_and_array() {
    let _guard = crate::test_support::handle_registry_test_lock();
    let mut frame = FaultFrame::new();
    let array = OwnedArray::new(&mut frame);
    let ptr = pointer(&mut frame);
    let before = array.values();
    unsafe {
        let mut out = 777;
        assert_eq!(wire::new(std::ptr::null_mut(), 1, &mut out), 2);
        assert_eq!(out, 777);
        assert_eq!(wire::new(ptr, 1, std::ptr::null_mut()), 2);
        for tag in [0, 8, u32::MAX] {
            assert_eq!(wire::claim(ptr, 1, array.0, tag), 2);
        }
        for value in [2, u32::MAX] {
            assert_eq!(wire::bool(ptr, 1, array.0, value), 2);
        }
        let non_array = handles::to_handle_arc(Arc::new(nyash_rust::box_trait::IntegerBox::new(1)));
        for handle in [0, -1, i64::MAX, non_array as i64] {
            assert_eq!(wire::claim(ptr, 1, handle, 1), 2);
            assert_eq!(wire::i64(ptr, 1, handle, 1), 2);
            assert_eq!(wire::bool(ptr, 1, handle, 1), 2);
            assert_eq!(wire::f64(ptr, 1, handle, 1.0), 2);
        }
        nyrt_handle_release_h(non_array as i64);
        for malformed in 0..3 {
            let mut bad = FaultFrame::new();
            match malformed {
                0 => bad.abi_version = 0,
                1 => bad.primary_present = 2,
                _ => bad.suppressed_len = 9,
            }
            let bad_ptr = pointer(&mut bad);
            for invalid_ptr in [std::ptr::null_mut(), bad_ptr] {
                assert_eq!(wire::new(invalid_ptr, 1, &mut out), 2);
                assert_eq!(out, 777);
                assert_eq!(wire::claim(invalid_ptr, 1, array.0, 1), 2);
                assert_eq!(wire::i64(invalid_ptr, 1, array.0, 1), 2);
                assert_eq!(wire::bool(invalid_ptr, 1, array.0, 1), 2);
                assert_eq!(wire::f64(invalid_ptr, 1, array.0, 1.0), 2);
            }
        }
    }
    assert_eq!(array.values(), before);
    assert!(frame.diagnostics().unwrap().0.is_none());
}

#[test]
fn checked_array_first_suppressed_overflow_and_normal_are_independent() {
    let _guard = crate::test_support::handle_registry_test_lock();
    let mut frame = FaultFrame::new();
    let array = OwnedArray::new(&mut frame);
    let ptr = pointer(&mut frame);
    assert_eq!(unsafe { wire::claim(ptr, 1, array.0, 4) }, 0);
    for site in 20..31 {
        assert_eq!(unsafe { wire::bool(ptr, site, array.0, 1) }, 1);
    }
    let (first, suppressed, omitted) = frame.diagnostics().unwrap();
    assert_eq!(first.unwrap().site, 20);
    assert_eq!(
        suppressed.iter().map(|d| d.site).collect::<Vec<_>>(),
        (21..29).collect::<Vec<_>>()
    );
    assert!(omitted);
    assert!(array.values().is_empty());
    assert_eq!(unsafe { wire::i64(ptr, 40, array.0, 9) }, 0);
    let other = OwnedArray::new(&mut frame);
    assert!(other.values().is_empty());
    assert_eq!(primary(&frame), (202, 20, [1, 0]));
    assert_eq!(unsafe { wire::bool(ptr, 41, array.0, 9) }, 2);
    assert_eq!(frame.suppressed_len, 8);
    assert_eq!(frame.dispose(), Status::Normal);
}

#[test]
fn checked_array_release_retires_the_published_host_residence() {
    let _guard = crate::test_support::handle_registry_test_lock();
    let mut frame = FaultFrame::new();
    let array = OwnedArray::new(&mut frame);
    let handle = array.0;
    assert!(handles::get(handle as u64)
        .unwrap()
        .as_any()
        .is::<ArrayBox>());
    drop(array); // exactly one ordinary void release
    assert!(handles::get(handle as u64).is_none());
    assert_eq!(unsafe { wire::i64(pointer(&mut frame), 1, handle, 7) }, 2);
}

#[test]
fn checked_array_error_mapping_rejects_unknown_vocabulary_without_recording() {
    let mut frame = FaultFrame::new();
    assert_eq!(
        record(
            &mut frame,
            claim_diagnostic(
                TypedArrayRuntimeContractError::ExistingElementMismatch {
                    index: 0,
                    reason: "runtime-type-mismatch",
                },
                1,
                1
            )
        ),
        1
    );
    assert_eq!(primary(&frame), (201, 1, [0, 1]));
    let before = primary(&frame);
    for error in [
        ArrayPrimitiveWriteError::UnsupportedStorage,
        ArrayPrimitiveWriteError::InvalidIndex,
        ArrayPrimitiveWriteError::ElementContract { reason: "unknown" },
    ] {
        assert_eq!(append_result(&mut frame, 2, Some(Err(error))), 2);
    }
    assert_eq!(
        record(
            &mut frame,
            claim_diagnostic(
                TypedArrayRuntimeContractError::ExistingElementMismatch {
                    index: 0,
                    reason: "unknown",
                },
                3,
                1
            )
        ),
        2
    );
    if usize::BITS > 63 {
        assert_eq!(
            record(
                &mut frame,
                claim_diagnostic(
                    TypedArrayRuntimeContractError::ExistingElementMismatch {
                        index: usize::MAX,
                        reason: "runtime-type-mismatch",
                    },
                    4,
                    1
                )
            ),
            2
        );
    }
    assert_eq!(primary(&frame), before);
    assert_eq!(frame.suppressed_len, 0);
}

#[test]
fn checked_array_c_header_constants_and_signatures_compile() {
    let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let source = manifest.join("tests/checked_array_abi.c");
    let output = std::env::temp_dir().join(format!("nyrt-checked-array-{}.o", std::process::id()));
    let result = std::process::Command::new("cc")
        .args(["-std=c11", "-Wall", "-Werror", "-c"])
        .arg(source)
        .arg("-o")
        .arg(&output)
        .output()
        .expect("C ABI compiler");
    let _ = std::fs::remove_file(output);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
}
