use super::*;

thread_local! { static SHARED_ENDS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) }; }
thread_local! { static SHARED_NESTED: std::cell::Cell<(usize, i64)> = const { std::cell::Cell::new((0, 0)) }; }

unsafe extern "C" fn shared_test_end(frame: *mut std::ffi::c_void, payload: i64, site: u64) -> u32 {
    SHARED_ENDS.with(|n| n.set(n.get() + 1));
    let profile = match crate::exports::typed_object_store_backend::selected_backend() {
        crate::exports::typed_object_store_backend::TypedObjectStoreBackend::SafeMutex => 1,
        crate::exports::typed_object_store_backend::TypedObjectStoreBackend::SingleThreadExact => 2,
        _ => return 2,
    };
    unsafe { checked_object::home_release(frame, profile, site, payload, 10) }
}

unsafe extern "C" fn shared_test_faulting_end(frame: *mut std::ffi::c_void, payload: i64, site: u64) -> u32 {
    if unsafe { shared_test_end(frame, payload, site) } != 0 { return 2; }
    unsafe { &mut *frame.cast::<FaultFrame>() }
        .record(Diagnostic::new(205, site, [0, 0])).unwrap_or(Status::InvalidContract) as u32
}

unsafe extern "C" fn shared_test_reentrant_end(frame: *mut std::ffi::c_void, payload: i64, site: u64) -> u32 {
    let (descriptor, handle) = SHARED_NESTED.with(|nested| nested.replace((0, 0)));
    if descriptor != 0 && unsafe {
        checked_shared::release(frame, site, descriptor as *const checked_shared::Descriptor, handle)
    } != 0 { return 2; }
    unsafe { shared_test_end(frame, payload, site) }
}

#[test]
fn shared_handles_keep_distinct_obligations_and_end_payload_once() {
    use crate::exports::typed_object_store_backend::{selected_backend, TypedObjectStoreBackend};
    let profile = match selected_backend() {
        TypedObjectStoreBackend::SafeMutex => 1,
        TypedObjectStoreBackend::SingleThreadExact => 2,
        _ => return,
    };
    SHARED_ENDS.with(|n| n.set(0));
    let mut frame = FaultFrame::new();
    let frame = (&mut frame as *mut FaultFrame).cast();
    let descriptor = checked_shared::Descriptor { abi: 1, profile, type_id: 10, end: Some(shared_test_end) };
    let mut payload = 0;
    let (mut first, mut second, mut observed) = (0, 0, 0);
    unsafe {
        assert_eq!(checked_object::allocate(frame, profile, 1, 10, [1].as_ptr(), 1, &mut payload), 0);
        assert_eq!(checked_shared::new(frame, 2, &descriptor, payload, &mut first), 0);
        assert_eq!(checked_shared::clone_handle(frame, 3, &descriptor, first, &mut second), 0);
        assert_ne!(first, second);
        std::thread::scope(|scope| {
            scope.spawn(|| {
                let mut foreign_frame = FaultFrame::new();
                let foreign_frame = (&mut foreign_frame as *mut FaultFrame).cast();
                let mut foreign_read = 777;
                assert_eq!(checked_shared::borrow(foreign_frame, 3, &descriptor, first, &mut foreign_read), 2);
                assert_eq!(foreign_read, 777);
            }).join().unwrap();
        });
        assert_eq!(checked_shared::release(frame, 4, &descriptor, first), 0);
        assert_eq!(checked_shared::release(frame, 5, &descriptor, first), 2);
        assert_eq!(checked_shared::borrow(frame, 6, &descriptor, second, &mut observed), 0);
        assert_eq!(observed, payload);
        SHARED_ENDS.with(|n| assert_eq!(n.get(), 0));
        assert_eq!(checked_shared::release(frame, 7, &descriptor, second), 0);
        SHARED_ENDS.with(|n| assert_eq!(n.get(), 1));
        assert_eq!(checked_shared::release(frame, 8, &descriptor, second), 2);
        SHARED_ENDS.with(|n| assert_eq!(n.get(), 1));
    }
}

#[test]
fn shared_faulting_final_end_is_not_retried_and_other_cell_survives() {
    use crate::exports::typed_object_store_backend::{selected_backend, TypedObjectStoreBackend};
    let profile = match selected_backend() {
        TypedObjectStoreBackend::SafeMutex => 1,
        TypedObjectStoreBackend::SingleThreadExact => 2,
        _ => return,
    };
    SHARED_ENDS.with(|n| n.set(0));
    let mut frame = FaultFrame::new();
    let frame = (&mut frame as *mut FaultFrame).cast();
    let descriptor = checked_shared::Descriptor { abi: 1, profile, type_id: 10, end: Some(shared_test_faulting_end) };
    let (mut p1, mut p2, mut h1, mut h2, mut read) = (0, 0, 0, 0, 0);
    unsafe {
        assert_eq!(checked_object::allocate(frame, profile, 1, 10, [1].as_ptr(), 1, &mut p1), 0);
        assert_eq!(checked_object::allocate(frame, profile, 2, 10, [1].as_ptr(), 1, &mut p2), 0);
        assert_eq!(checked_shared::new(frame, 3, &descriptor, p1, &mut h1), 0);
        assert_eq!(checked_shared::new(frame, 4, &descriptor, p2, &mut h2), 0);
        assert_eq!(checked_shared::release(frame, 5, &descriptor, h1), 1);
        assert_eq!(checked_shared::release(frame, 6, &descriptor, h1), 2);
        assert_eq!(checked_shared::borrow(frame, 7, &descriptor, h2, &mut read), 0);
        assert_eq!(read, p2);
        assert_eq!(checked_shared::release(frame, 8, &descriptor, h2), 1);
        SHARED_ENDS.with(|n| assert_eq!(n.get(), 2));
    }
}

#[test]
fn shared_final_end_can_release_another_cell_without_borrowing_slab() {
    use crate::exports::typed_object_store_backend::{selected_backend, TypedObjectStoreBackend};
    let profile = match selected_backend() {
        TypedObjectStoreBackend::SafeMutex => 1,
        TypedObjectStoreBackend::SingleThreadExact => 2,
        _ => return,
    };
    SHARED_ENDS.with(|n| n.set(0));
    let mut frame = FaultFrame::new();
    let frame = (&mut frame as *mut FaultFrame).cast();
    let descriptor = checked_shared::Descriptor { abi: 1, profile, type_id: 10, end: Some(shared_test_reentrant_end) };
    let (mut p1, mut p2, mut h1, mut h2) = (0, 0, 0, 0);
    unsafe {
        assert_eq!(checked_object::allocate(frame, profile, 1, 10, [1].as_ptr(), 1, &mut p1), 0);
        assert_eq!(checked_object::allocate(frame, profile, 2, 10, [1].as_ptr(), 1, &mut p2), 0);
        assert_eq!(checked_shared::new(frame, 3, &descriptor, p1, &mut h1), 0);
        assert_eq!(checked_shared::new(frame, 4, &descriptor, p2, &mut h2), 0);
        SHARED_NESTED.with(|nested| nested.set((&descriptor as *const _ as usize, h2)));
        assert_eq!(checked_shared::release(frame, 5, &descriptor, h1), 0);
        assert_eq!(checked_shared::release(frame, 6, &descriptor, h2), 2);
        SHARED_ENDS.with(|n| assert_eq!(n.get(), 2));
    }
}

#[test]
fn exact_field_read_distinguishes_zero_and_preserves_output_on_contract_failure() {
    use super::checked_object as api;
    use crate::exports::typed_object_store_backend::{selected_backend, TypedObjectStoreBackend};
    let profile = match selected_backend() {
        TypedObjectStoreBackend::SafeMutex => 1,
        TypedObjectStoreBackend::SingleThreadExact => 2,
        _ => {
            let mut output = 777;
            assert_eq!(unsafe { api::field_get(1, -1, 10, 0, &mut output) }, 2);
            assert_eq!(output, 777);
            return;
        }
    };
    let mut frame = FaultFrame::new();
    let pointer = (&mut frame as *mut FaultFrame).cast();
    let mut handle = 0;
    unsafe {
        assert_eq!(
            api::allocate(pointer, profile, 1, 10, [1].as_ptr(), 1, &mut handle),
            0
        );
        for value in [0, i64::MIN, i64::MAX] {
            assert_eq!(api::field_set(pointer, profile, 2, handle, 10, 0, value), 0);
            let mut output = 777;
            assert_eq!(api::field_get(profile, handle, 10, 0, &mut output), 0);
            assert_eq!(output, value);
            output = 777;
            assert_eq!(api::field_get_fallible(pointer, profile, 20, handle, 10, 0, &mut output), 0);
            assert_eq!(output, value, "fallible Normal read includes exact zero and extrema");
        }
        for (p, h, ty, slot) in [
            (99, handle, 10, 0),
            (3 - profile, handle, 10, 0),
            (profile, handle, 11, 0),
            (profile, handle, 10, 1),
            (profile, 0, 10, 0),
            (profile, i64::MIN, 10, 0),
        ] {
            let mut output = 777;
            assert_eq!(api::field_get(p, h, ty, slot, &mut output), 2);
            assert_eq!(output, 777);
        }
        assert_eq!(
            api::field_get(profile, handle, 10, 0, std::ptr::null_mut()),
            2
        );
        let mut output = 777;
        assert_eq!(api::field_get(profile, handle, 10, 0, &mut output), 0);
        assert_eq!(output, i64::MAX, "failed reads leave storage unchanged");
        assert!(frame.diagnostics().unwrap().0.is_none());
        let mut failed_read_frame = FaultFrame::new();
        let failed_read_pointer = (&mut failed_read_frame as *mut FaultFrame).cast();
        for (h, ty, slot) in [(0, 10, 0), (handle, 11, 0), (handle, 10, 1)] {
            output = 777;
            assert_eq!(api::field_get_fallible(failed_read_pointer, profile, 41, h, ty, slot, &mut output), 1);
            assert_eq!(output, 777, "Fault cannot publish a value");
        }
        let (primary, suppressed, _) = failed_read_frame.diagnostics().unwrap();
        assert_eq!((primary.unwrap().reason, primary.unwrap().site), (101, 41));
        assert_eq!(suppressed.len(), 2);
        for p in [99, 3 - profile] {
            assert_eq!(api::field_get_fallible(pointer, p, 42, handle, 10, 0, &mut output), 2);
            assert_eq!(output, 777);
        }
        assert_eq!(api::field_get_fallible(std::ptr::null_mut(), profile, 42, handle, 10, 0, &mut output), 2);
        assert_eq!(api::field_get_fallible(pointer, profile, 42, handle, 10, 0, std::ptr::null_mut()), 2);
        assert!(frame.diagnostics().unwrap().0.is_none());
        assert_eq!(api::home_release(failed_read_pointer, profile, 43, handle, 10), 0);
        output = 777;
        assert_eq!(api::field_get_fallible(failed_read_pointer, profile, 44, handle, 10, 0, &mut output), 1);
        assert_eq!(output, 777);
        assert_eq!(failed_read_frame.diagnostics().unwrap().0.unwrap().site, 41,
            "remaining cleanup preserves the primary read Fault");
        let mut survivor = 0;
        assert_eq!(api::allocate(pointer, profile, 45, 10, [1].as_ptr(), 1, &mut survivor), 0);
        assert_eq!(api::home_release(failed_read_pointer, profile, 46, survivor, 10), 0,
            "a prior read Fault does not prevent remaining Home cleanup");
        // The original handle was released above under the failing frame.
        assert_eq!(api::field_get(profile, handle, 10, 0, &mut output), 2);
        output = i64::MAX;
        assert_eq!(api::field_get(profile, handle, 10, 0, &mut output), 2);
        assert_eq!(output, i64::MAX);
    }
}

// Test-thread-local accounting: no environment mutation, production allocator
// replacement or counts from concurrently running tests.
struct CountingAllocator;
thread_local! {
    static COUNTS: std::cell::Cell<Option<(usize, usize)>> = const { std::cell::Cell::new(None) };
    static FAIL_NEXT: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}
fn count(allocation: bool) {
    let _ = COUNTS.try_with(|counts| {
        if let Some((alloc, free)) = counts.get() {
            counts.set(Some((
                alloc.saturating_add(usize::from(allocation)),
                free.saturating_add(usize::from(!allocation)),
            )));
        }
    });
}
unsafe impl std::alloc::GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
        count(true);
        if FAIL_NEXT
            .try_with(|flag| flag.replace(false))
            .unwrap_or(false)
        {
            return std::ptr::null_mut();
        }
        unsafe { std::alloc::GlobalAlloc::alloc(&std::alloc::System, layout) }
    }
    unsafe fn alloc_zeroed(&self, layout: std::alloc::Layout) -> *mut u8 {
        count(true);
        if FAIL_NEXT
            .try_with(|flag| flag.replace(false))
            .unwrap_or(false)
        {
            return std::ptr::null_mut();
        }
        unsafe { std::alloc::GlobalAlloc::alloc_zeroed(&std::alloc::System, layout) }
    }
    unsafe fn realloc(&self, pointer: *mut u8, layout: std::alloc::Layout, size: usize) -> *mut u8 {
        count(true);
        if FAIL_NEXT
            .try_with(|flag| flag.replace(false))
            .unwrap_or(false)
        {
            return std::ptr::null_mut();
        }
        unsafe { std::alloc::GlobalAlloc::realloc(&std::alloc::System, pointer, layout, size) }
    }
    unsafe fn dealloc(&self, pointer: *mut u8, layout: std::alloc::Layout) {
        count(false);
        unsafe { std::alloc::GlobalAlloc::dealloc(&std::alloc::System, pointer, layout) }
    }
}
#[global_allocator]
static TEST_ALLOCATOR: CountingAllocator = CountingAllocator;

#[test]
fn recording_overflow_and_disposal_allocate_zero_and_release_each_payload_once() {
    let messages: [Diagnostic; 11] = std::array::from_fn(|index| {
        Diagnostic::new(1, index as u64, [0; 2])
            .with_message(vec![index as u8; 16].into_boxed_slice())
    });
    let mut frame = FaultFrame::new();
    COUNTS.with(|counts| counts.set(Some((0, 0))));
    for message in messages {
        assert!(frame.record(message).is_ok());
    }
    let after_record = COUNTS.with(|counts| counts.get().unwrap());
    assert_eq!(frame.dispose(), Status::Normal);
    let after_dispose = COUNTS.with(|counts| counts.replace(None).unwrap());
    assert_eq!(after_record, (0, 2)); // primary + eight retained; two omitted
    assert_eq!(after_dispose, (0, 11));
}

#[test]
fn frame_layout_matches_c_header_contract() {
    use std::mem::{align_of, offset_of, size_of};
    assert_eq!(offset_of!(Diagnostic, site), 8);
    assert_eq!(offset_of!(Diagnostic, details), 16);
    assert_eq!(offset_of!(Diagnostic, message), 32);
    assert_eq!(
        size_of::<Diagnostic>(),
        32 + size_of::<*mut u8>() + size_of::<usize>()
    );
    assert_eq!(offset_of!(FaultFrame, primary), 16);
    assert_eq!(
        offset_of!(FaultFrame, suppressed),
        16 + size_of::<Diagnostic>()
    );
    assert_eq!(size_of::<FaultFrame>(), 16 + 9 * size_of::<Diagnostic>());
    assert_eq!(align_of::<FaultFrame>(), align_of::<Diagnostic>());
}

#[test]
fn primary_and_order_survive_overflow() {
    let mut frame = FaultFrame::new();
    for site in 0..12 {
        assert!(matches!(
            frame.record(Diagnostic::new(1, site, [0; 2])),
            Ok(Status::Fault)
        ));
    }
    let (primary, suppressed, omitted) = frame.diagnostics().unwrap();
    assert_eq!(primary.unwrap().site, 0);
    assert_eq!(
        suppressed.iter().map(|d| d.site).collect::<Vec<_>>(),
        (1..9).collect::<Vec<_>>()
    );
    assert!(omitted);
    assert_eq!(Status::Normal as u32, 0); // per-operation success, not frame outcome
    assert_eq!(frame.diagnostics().unwrap().0.unwrap().site, 0);
}

#[test]
fn invalid_header_returns_message_ownership_unchanged() {
    let mut frame = FaultFrame::new();
    frame.abi_version = 99;
    let bytes = b"owned after caller exit".to_vec().into_boxed_slice();
    let pointer = bytes.as_ptr();
    let rejected = frame
        .record(Diagnostic::new(1, 7, [3, 4]).with_message(bytes))
        .err()
        .unwrap();
    assert_eq!(rejected.message().unwrap().as_ptr(), pointer);
    assert_eq!(frame.primary_present, 0);
    assert!(matches!(frame.diagnostics(), Err(Status::InvalidContract)));
}

#[test]
fn owned_and_absent_messages_have_distinct_lifetimes() {
    let mut frame = FaultFrame::new();
    assert!(Diagnostic::new(1, 0, [0; 2]).message().is_none());
    let message = Vec::from(&b"retained"[..]).into_boxed_slice();
    assert!(frame
        .record(Diagnostic::new(1, 0, [0; 2]).with_message(message))
        .is_ok());
    assert_eq!(
        frame.diagnostics().unwrap().0.unwrap().message(),
        Some(&b"retained"[..])
    );
    let empty_owned = Diagnostic::new(1, 1, [0; 2]).with_message(Vec::new().into_boxed_slice());
    assert_eq!(empty_owned.message(), Some(&b""[..]));
    // Frame/Diagnostic Drop is the sole byte disposal owner, also for an empty Box.
}

#[test]
fn c_entry_initializes_records_and_invalidates_once() {
    let mut storage = std::mem::MaybeUninit::<FaultFrame>::uninit();
    let pointer = storage.as_mut_ptr().cast();
    unsafe {
        assert_eq!(frame_init(pointer), Status::Normal as u32);
        assert_eq!(report_final(pointer), 0); // empty frame emits nothing
        assert_eq!(record_static(pointer, 1, 27, -1, 4), Status::Fault as u32);
        assert_eq!(
            (*storage.as_ptr()).diagnostics().unwrap().0.unwrap().site,
            27
        );
        assert_eq!(frame_dispose(pointer), Status::Normal as u32);
        assert_eq!(frame_dispose(pointer), Status::InvalidContract as u32);
        assert_eq!(report_final(pointer), -1);
        assert_eq!(
            record_static(pointer, 1, 28, 0, 0),
            Status::InvalidContract as u32
        );
        storage.assume_init_drop(); // only empty records remain; no second byte free
        assert_eq!(
            frame_init(std::ptr::null_mut()),
            Status::InvalidContract as u32
        );
    }
}

#[test]
fn report_borrows_primary_and_reports_omission_after_cleanup() {
    let mut frame = FaultFrame::new();
    for site in 0..10 {
        assert!(frame.record(Diagnostic::new(1, site, [0; 2])).is_ok());
    }
    let mut output = Vec::new();
    frame.report(&mut output).unwrap();
    let text = String::from_utf8(output).unwrap();
    assert!(text.starts_with("[fault:primary] reason=1 site=0"));
    assert_eq!(text.matches("[fault:suppressed]").count(), 8);
    assert!(text.ends_with("[fault] additional diagnostics omitted\n"));
    assert_eq!(frame.diagnostics().unwrap().0.unwrap().site, 0);
}

#[test]
fn checked_c_operations_preserve_primary_and_out_slot() {
    use super::checked_object as api;
    use crate::exports::typed_object_store_backend::{selected_backend, TypedObjectStoreBackend};
    let profile = match selected_backend() {
        TypedObjectStoreBackend::SafeMutex => 1,
        TypedObjectStoreBackend::SingleThreadExact => 2,
        _ => return, // unsupported physical profiles have their own rejection test
    };
    let mut frame = FaultFrame::new();
    let pointer = (&mut frame as *mut FaultFrame).cast();
    let mut handle = 777;
    unsafe {
        assert_eq!(
            api::allocate(pointer, 99, 1, 10, [1].as_ptr(), 1, &mut handle),
            2
        );
        assert_eq!(handle, 777);
        assert_eq!(
            api::allocate(pointer, profile, 1, 10, [2].as_ptr(), 1, &mut handle),
            2
        );
        assert_eq!(handle, 777);
        assert!(frame.diagnostics().unwrap().0.is_none());
        assert_eq!(
            api::allocate(pointer, profile, 1, 10, [1].as_ptr(), 1, &mut handle),
            0
        );
        assert!(handle < 0);
        let mut failed_result = 777;
        FAIL_NEXT.with(|flag| flag.set(true));
        let allocation_status =
            api::allocate(pointer, profile, 2, 10, [1].as_ptr(), 1, &mut failed_result);
        let flag_consumed = FAIL_NEXT.with(|flag| !flag.replace(false));
        assert!(flag_consumed);
        assert_eq!(allocation_status, 1);
        assert_eq!(failed_result, 777);
        assert_eq!(frame.diagnostics().unwrap().0.unwrap().site, 2);
        assert_eq!(api::field_set(pointer, profile, 2, handle, 10, 0, 42), 0);
        assert_eq!(api::field_set(pointer, profile, 3, handle, 11, 0, 99), 1);
        assert_eq!(frame.diagnostics().unwrap().0.unwrap().site, 2);
        assert_eq!(api::home_release(pointer, profile, 4, handle, 10), 0);
        assert_eq!(frame.diagnostics().unwrap().0.unwrap().site, 2);
        assert_eq!(api::reclaim(pointer, profile, 5, handle, 10), 1);
        assert_eq!(
            frame
                .diagnostics()
                .unwrap()
                .1
                .iter()
                .map(|d| d.site)
                .collect::<Vec<_>>(),
            vec![3, 5]
        );
        assert_eq!(frame_dispose(pointer), 0);
        handle = 777;
        assert_eq!(
            api::allocate(pointer, profile, 6, 10, [1].as_ptr(), 1, &mut handle),
            2
        );
        assert_eq!(handle, 777);
    }
    let mut frame = FaultFrame::new();
    let pointer = (&mut frame as *mut FaultFrame).cast();
    let mut parent = 777;
    let mut child = 777;
    let sentinel = api::ObjectResidenceV1 { value: 999, outcome: 99, reserved: 88 };
    unsafe {
        assert_eq!(api::allocate(pointer, profile, 80, 901, [1, 1, 1].as_ptr(), 3, &mut parent), 0);
        assert_eq!(api::allocate(pointer, profile, 81, 902, std::ptr::null(), 0, &mut child), 0);
        let mut out = sentinel;
        assert_eq!(api::residence_get(profile, parent, 901, 0, 902, &mut out), 2);
        assert_eq!(out, sentinel);
        assert_eq!(api::residence_set(pointer, profile, 90, parent, 901, 0, 902, child, 99), 2);
        assert_eq!(api::residence_set(pointer, profile, 90, parent, 901, 0, 902, child, 0), 2);
        assert!(frame.diagnostics().unwrap().0.is_none());
        // A valid pair with a wrong canonical child records Fault without
        // changing the slot. Later successful operations preserve that Fault.
        assert_eq!(api::residence_set(pointer, profile, 91, parent, 901, 0, 903, child, 1), 1);
        for (slot, value, outcome) in [(0, 0, 0), (1, child, 1), (2, child, 2)] {
            assert_eq!(api::residence_set(pointer, profile, 92, parent, 901, slot, 902, value, outcome), 0);
            assert_eq!(api::residence_get(profile, parent, 901, slot, 902, &mut out), 0);
            assert_eq!(out, api::ObjectResidenceV1 { value, outcome, reserved: 0 });
            for (bad_profile, bad_parent, bad_type, bad_slot, bad_child) in [
                (99, parent, 901, slot, 902), (profile, parent, 900, slot, 902),
                (profile, i64::MIN, 901, slot, 902), (profile, parent, 901, 9, 902),
            ] {
                out = sentinel;
                assert_eq!(api::residence_get(bad_profile, bad_parent, bad_type, bad_slot, bad_child, &mut out), 2);
                assert_eq!(out, sentinel);
            }
            assert_eq!(api::residence_get(profile, parent, 901, slot, 902, std::ptr::null_mut()), 2);
            assert_eq!(api::field_set(pointer, profile, 93, parent, 901, slot, 44), 1);
            assert_eq!(api::residence_get(profile, parent, 901, slot, 902, &mut out), 0);
            assert_eq!(out, api::ObjectResidenceV1 { value, outcome, reserved: 0 });
            assert_eq!(frame.diagnostics().unwrap().0.unwrap().site, 91);
        }
        assert_eq!(api::reclaim(pointer, profile, 94, parent, 901), 0);
        assert_eq!(api::reclaim(pointer, profile, 95, child, 902), 0);
        out = sentinel;
        assert_eq!(api::residence_get(profile, parent, 901, 0, 902, &mut out), 2);
        assert_eq!(out, sentinel);
        assert_eq!(frame_dispose(pointer), 0);
    }
}

#[test]
fn checked_string_pairs_preserve_bytes_independence_and_fault_cleanup() {
    use super::checked_string::{self as api, Operand, OwnedText};
    use nyash_rust::runtime::dynamic_v2_lease as lease;
    use std::num::NonZeroU64;
    unsafe extern "C" {
        #[link_name = "nyash.string.checked_add_v1"]
        fn checked_add(
            storage: *mut std::ffi::c_void,
            site: u64,
            left: *const Operand,
            right: *const Operand,
            out: *mut OwnedText,
        ) -> u32;
        #[link_name = "nyash.string.checked_end_v1"]
        fn checked_end(storage: *mut std::ffi::c_void, site: u64, handle: u64, token: u64) -> u32;
        #[link_name = "nyash.string.checked_length_v1"]
        fn checked_length(storage: *mut std::ffi::c_void, site: u64, value: *const Operand, out: *mut i64) -> u32;
        #[link_name = "nyash.string.checked_materialize_v1"]
        fn checked_materialize(storage: *mut std::ffi::c_void, site: u64, value: *const Operand, out: *mut OwnedText) -> u32;
        #[link_name = "nyash.string.checked_substring_v1"]
        fn checked_substring(storage: *mut std::ffi::c_void, site: u64, value: *const Operand,
            start: i64, end: i64, out: *mut OwnedText) -> u32;
        #[link_name = "nyash.string.checked_substring_operands_v1"]
        fn checked_substring_operands(storage: *mut std::ffi::c_void, site: u64, value: *const Operand,
            start: *const Operand, end: *const Operand, out: *mut OwnedText) -> u32;
        #[link_name = "nyash.string.checked_index_of_v1"]
        fn checked_index_of(storage: *mut std::ffi::c_void, site: u64, value: *const Operand,
            needle: *const Operand, out: *mut i64) -> u32;
    }
    let literal = |bytes: &[u8]| Operand {
        kind: 1,
        reserved: 0,
        bytes: bytes.as_ptr(),
        len: bytes.len(),
        handle: 0,
        token: 0,
        integer: 0,
    };
    let owned = |pair: OwnedText| Operand {
        kind: 2,
        reserved: 0,
        bytes: std::ptr::null(),
        len: 0,
        handle: pair.handle,
        token: pair.token,
        integer: 0,
    };
    let read = |pair: OwnedText| {
        lease::with_end_authorized_text_pair(
            pair.handle,
            NonZeroU64::new(pair.token).unwrap(),
            str::to_owned,
        )
    };
    let mut frame = FaultFrame::new();
    let pointer = (&mut frame as *mut FaultFrame).cast();
    let sentinel = OwnedText {
        handle: 777,
        token: 888,
    };
    let mut first = sentinel;
    let mut second = sentinel;
    unsafe {
        assert_eq!(
            checked_materialize(pointer, 1, &literal(b""), &mut first),
            0
        );
        assert_eq!(
            checked_materialize(pointer, 2, &owned(first), &mut second),
            0
        );
        assert_ne!(first.handle, second.handle);
        assert_ne!(first.token, second.token);
        assert_eq!(read(first), Ok(String::new()));
        assert_eq!(read(second), Ok(String::new()));
        assert_eq!(checked_end(pointer, 3, second.handle, first.token), 2);
        assert_eq!(checked_end(pointer, 3, 0, first.token), 2);
        assert_eq!(read(first), Ok(String::new()));
        assert_eq!(checked_end(pointer, 3, first.handle, first.token), 0);
        assert_eq!(checked_end(pointer, 3, first.handle, first.token), 2);
        assert_eq!(read(second), Ok(String::new()));
        assert_eq!(checked_end(pointer, 3, second.handle, second.token), 0);

        let mut result = sentinel;
        let integer = Operand {
            kind: 3,
            integer: i64::MIN,
            ..owned(OwnedText {
                handle: 0,
                token: 0,
            })
        };
        assert_eq!(
            checked_add(
                pointer,
                4,
                &literal("猫\0".as_bytes()),
                &integer,
                &mut result
            ),
            0
        );
        let expected = format!("猫\0{}", i64::MIN);
        assert_eq!(read(result), Ok(expected.clone()));
        let mut bytes = Vec::new();
        assert_eq!(
            api::print_to(&mut frame, 5, expected.as_bytes(), &mut bytes),
            0
        );
        assert_eq!(bytes, format!("{expected}\n").as_bytes());
        for (left, right) in [
            (integer, literal(b"wrong direction")),
            (literal(b"prefix"), literal(&[0xff])),
            (literal(b"prefix"), owned(first)),
            (
                literal(b"prefix"),
                Operand {
                    kind: 99,
                    ..integer
                },
            ),
        ] {
            let mut out = sentinel;
            assert_eq!(checked_add(pointer, 6, &left, &right, &mut out), 2);
            assert_eq!(out, sentinel);
        }
        assert!(frame.diagnostics().unwrap().0.is_none());
        let mut base = sentinel;
        assert_eq!(checked_add(pointer, 20, &literal("aé界z".as_bytes()), &literal(b""), &mut base), 0);
        for operand in [literal("aé界z".as_bytes()), owned(base)] {
            let mut count = 999;
            assert_eq!(checked_length(pointer, 21, &operand, &mut count), 0);
            assert_eq!(count, 4);
            for (start, end, expected) in [
                (1, 3, "é界"), (-5, 99, "aé界z"), (3, 1, ""),
                (i64::MIN, i64::MAX, "aé界z"), (2, 2, ""), (99, 100, ""),
            ] {
                let mut slice = sentinel;
                assert_eq!(checked_substring(pointer, 22, &operand, start, end, &mut slice), 0);
                assert_ne!(slice.handle, base.handle);
                assert_ne!(slice.token, base.token);
                assert_eq!(read(slice), Ok(expected.into()));
                assert_eq!(checked_end(pointer, 23, slice.handle, slice.token), 0);
            }
            for (needle, expected) in [("界", 2), ("", 0), ("absent", -1)] {
                let mut index = 999;
                assert_eq!(checked_index_of(pointer, 24, &operand, &literal(needle.as_bytes()), &mut index), 0);
                assert_eq!(index, expected);
            }
        }
        for bad in [integer, literal(&[0xff]), owned(first), Operand { reserved: 1, ..owned(base) },
            Operand { token: base.token + 1, ..owned(base) }] {
            let mut scalar = 999;
            let mut slice = sentinel;
            assert_eq!(checked_length(pointer, 25, &bad, &mut scalar), 2);
            assert_eq!(checked_index_of(pointer, 25, &bad, &literal(b""), &mut scalar), 2);
            assert_eq!(checked_index_of(pointer, 25, &owned(base), &bad, &mut scalar), 2);
            assert_eq!(checked_materialize(pointer, 25, &bad, &mut slice), 2);
            assert_eq!(scalar, 999);
            assert_eq!(slice, sentinel);
        }
        let mut full = sentinel;
        let mut empty = sentinel;
        assert_eq!(checked_substring(pointer, 26, &owned(base), 0, 99, &mut full), 0);
        assert_eq!(checked_substring(pointer, 26, &owned(base), 4, 0, &mut empty), 0);
        assert_ne!(full.handle, empty.handle);
        assert_ne!(full.token, empty.token);
        assert_eq!(checked_length(pointer, 27, &owned(base), std::ptr::null_mut()), 2);
        assert_eq!(checked_substring(pointer, 27, &owned(base), 0, 1, std::ptr::null_mut()), 2);
        assert_eq!(checked_index_of(pointer, 27, &owned(base), &literal(b""), std::ptr::null_mut()), 2);
        assert_eq!(checked_end(pointer, 28, base.handle, base.token), 0);
        assert_eq!(read(full), Ok("aé界z".into()));
        assert_eq!(read(empty), Ok(String::new()));
        assert_eq!(checked_end(pointer, 29, empty.handle, empty.token), 0);
        assert_eq!(checked_end(pointer, 29, full.handle, full.token), 0);
        let mut count = 999;
        assert_eq!(checked_length(pointer, 30, &literal(b"\0"), &mut count), 0);
        assert_eq!(count, 1);
        assert!(frame.diagnostics().unwrap().0.is_none());
        // Existing primary stays primary, IO fault becomes suppressed; every
        // successful acquisition remains End-able after the writer fails.
        assert!(matches!(
            frame.record(Diagnostic::new(999, 7, [0, 0])),
            Ok(Status::Fault)
        ));
        struct Failing;
        impl std::io::Write for Failing {
            fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
                Err(std::io::Error::other("sink failed"))
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }
        assert_eq!(
            api::print_to(&mut frame, 8, expected.as_bytes(), &mut Failing),
            1
        );
        assert_eq!(frame.diagnostics().unwrap().0.unwrap().reason, 999);
        assert_eq!(frame.diagnostics().unwrap().1[0].reason, 300);
        assert_eq!(checked_end(pointer, 9, result.handle, result.token), 0);
        assert!(read(result).is_err());
        assert_eq!(frame.diagnostics().unwrap().0.unwrap().reason, 999);

        let mut dynamic_frame = FaultFrame::new();
        let dynamic_pointer = (&mut dynamic_frame as *mut FaultFrame).cast();
        let dynamic_integer = |value| Operand {
            kind: 4, reserved: 1, integer: value,
            ..owned(OwnedText { handle: 0, token: 0 })
        };
        let mut dynamic_slice = sentinel;
        assert_eq!(checked_substring_operands(dynamic_pointer, 40, &literal("aé界z".as_bytes()),
            &dynamic_integer(1), &dynamic_integer(3), &mut dynamic_slice), 0);
        assert_eq!(read(dynamic_slice), Ok("é界".into()));
        assert_eq!(checked_end(dynamic_pointer, 41, dynamic_slice.handle, dynamic_slice.token), 0);

        let dynamic_bool = Operand { reserved: 2, integer: 1, ..dynamic_integer(0) };
        let mut rejected_slice = sentinel;
        assert_eq!(checked_substring_operands(dynamic_pointer, 42, &literal(b"text"),
            &dynamic_bool, &dynamic_integer(2), &mut rejected_slice), 1);
        assert_eq!(rejected_slice, sentinel);
        assert_eq!(dynamic_frame.diagnostics().unwrap().0.unwrap().reason, 302);
    }
}

#[test]
fn checked_dynamic_invocation_publishes_and_ends_one_opaque_carrier() {
    use super::checked_dynamic::{self as api, Operand};
    use super::checked_string::{self as text_api, Operand as TextOperand, OwnedText};
    use nyash_rust::{box_trait::StringBox, runtime::host_handles};
    use std::sync::Arc;

    let handle = host_handles::to_handle_arc(Arc::new(StringBox::new("猫x")));
    let receiver = Operand {
        kind: 1,
        tag: 3,
        payload: handle,
    };
    let selector = b"length";
    let mut frame = FaultFrame::new();
    let pointer = (&mut frame as *mut FaultFrame).cast();
    let mut carrier = 0x1234;
    let mut integer = -91;
    let text_carrier = |token| TextOperand {
        kind: 5, reserved: 0, bytes: std::ptr::null(), len: 0,
        handle: 0, token, integer: 0,
    };
    // Nullable Text publication owns one carrier; invalid pairs never publish.
    let mut nullable_frame = FaultFrame::new();
    let nullable_pointer = (&mut nullable_frame as *mut FaultFrame).cast();
    let mut nullable_carrier = 0x1234;
    let literal = TextOperand { kind: 1, reserved: 0, bytes: b"cat".as_ptr(), len: 3,
        handle: 0, token: 0, integer: 0 };
    unsafe {
        assert_eq!(api::publish_text_or_null(nullable_pointer, 37, 0, 0, &mut nullable_carrier), 0);
        assert_eq!(api::end(nullable_pointer, 38, nullable_carrier), 0);
        nullable_carrier = 0x1234;
        assert_eq!(api::publish_text_or_null(nullable_pointer, 39, 0, 1, &mut nullable_carrier), 2);
        assert_eq!(nullable_carrier, 0x1234);
        let mut pair = OwnedText { handle: 0, token: 0 };
        assert_eq!(text_api::materialize(nullable_pointer, 39, &literal, &mut pair), 0);
        assert_eq!(api::publish_text_or_null(nullable_pointer, 39, pair.handle, pair.token, &mut nullable_carrier), 0);
        let mut snapshot = OwnedText { handle: 0, token: 0 };
        assert_eq!(text_api::materialize(nullable_pointer, 39, &text_carrier(nullable_carrier), &mut snapshot), 0);
        assert_eq!(nyash_rust::runtime::dynamic_v2_lease::with_end_authorized_text_pair(
            snapshot.handle, std::num::NonZeroU64::new(snapshot.token).unwrap(), str::to_owned), Ok("cat".into()));
        assert_eq!(text_api::end(nullable_pointer, 39, snapshot.handle, snapshot.token), 0);
        assert_eq!(text_api::end(nullable_pointer, 39, pair.handle, pair.token), 2);
        assert_eq!(api::end(nullable_pointer, 39, nullable_carrier), 0);
    }
    unsafe {
        assert_eq!(
            api::invoke(
                pointer,
                41,
                &receiver,
                selector.as_ptr(),
                selector.len(),
                std::ptr::null(),
                0,
                &mut carrier
            ),
            0
        );
        let carrier_raw = carrier;
        assert_eq!(
            api::integer_payload(pointer, 42, carrier_raw, &mut integer),
            0
        );
        assert_eq!(integer, 2);
        let mut rejected_text = OwnedText { handle: 777, token: 888 };
        let sentinel = rejected_text;
        let mut snapshot_fault = FaultFrame::new();
        assert_eq!(text_api::materialize((&mut snapshot_fault as *mut FaultFrame).cast(),
            42, &text_carrier(carrier_raw), &mut rejected_text), 1);
        assert_eq!(rejected_text, sentinel, "wrong payload publishes no Text");
        assert_eq!(snapshot_fault.diagnostics().unwrap().0.unwrap().reason, 302);
        assert_eq!(api::end(pointer, 43, carrier_raw), 0);

        carrier = 0x5678;
        assert_eq!(
            api::invoke(
                pointer,
                44,
                &receiver,
                b"missing".as_ptr(),
                7,
                std::ptr::null(),
                0,
                &mut carrier
            ),
            1
        );
        assert_eq!(carrier, 0x5678, "Fault publishes no carrier");
        assert_eq!(api::end(pointer, 45, carrier_raw), 1);
    }
    let (primary, suppressed, _) = frame.diagnostics().unwrap();
    assert_eq!((primary.unwrap().reason, primary.unwrap().site), (400, 44));
    assert_eq!(
        primary.unwrap().details[0],
        2,
        "missing provider is retained as a terminal Fault"
    );
    assert_eq!(
        suppressed.len(),
        1,
        "a later cleanup Fault is suppressed behind the first"
    );
    assert_eq!(suppressed[0].site, 45);

    let substring = b"substring";
    let arguments = [
        Operand {
            kind: 2,
            tag: 0,
            payload: 0,
        },
        Operand {
            kind: 2,
            tag: 0,
            payload: 1,
        },
    ];
    let mut type_frame = FaultFrame::new();
    let type_pointer = (&mut type_frame as *mut FaultFrame).cast();
    let mut string_carrier = 0;
    let mut rejected_integer = -73;
    unsafe {
        assert_eq!(
            api::invoke(
                type_pointer,
                50,
                &receiver,
                substring.as_ptr(),
                substring.len(),
                arguments.as_ptr(),
                arguments.len(),
                &mut string_carrier
            ),
            0
        );
        let mut snapshot = OwnedText { handle: 0, token: 0 };
        assert_eq!(text_api::materialize(type_pointer, 50, &text_carrier(string_carrier), &mut snapshot), 0);
        assert_eq!(
            api::integer_payload(type_pointer, 51, string_carrier, &mut rejected_integer),
            1
        );
        assert_eq!(
            rejected_integer, -73,
            "Fault leaves the projection output untouched"
        );
        assert_eq!(api::end(type_pointer, 52, string_carrier), 0);
        let owned = TextOperand { kind: 2, reserved: 0, bytes: std::ptr::null(), len: 0,
            handle: snapshot.handle, token: snapshot.token, integer: 0 };
        let mut length = -1;
        assert_eq!(text_api::length(type_pointer, 53, &owned, &mut length), 0);
        assert_eq!(length, 1, "Text snapshot outlives the borrowed carrier");
        assert_eq!(text_api::end(type_pointer, 54, snapshot.handle, snapshot.token), 0);
    }
    let (primary, _, _) = type_frame.diagnostics().unwrap();
    let primary = primary.unwrap();
    assert_eq!((primary.reason, primary.site, primary.details[0]), (400, 51, 4));
    host_handles::drop_handle(handle);
}
