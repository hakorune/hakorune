use super::*;

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
            counts.set(Some((alloc.saturating_add(usize::from(allocation)),
                free.saturating_add(usize::from(!allocation)))));
        }
    });
}
unsafe impl std::alloc::GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
        count(true);
        if FAIL_NEXT.try_with(|flag| flag.replace(false)).unwrap_or(false) { return std::ptr::null_mut(); }
        unsafe { std::alloc::GlobalAlloc::alloc(&std::alloc::System, layout) }
    }
    unsafe fn alloc_zeroed(&self, layout: std::alloc::Layout) -> *mut u8 {
        count(true);
        if FAIL_NEXT.try_with(|flag| flag.replace(false)).unwrap_or(false) { return std::ptr::null_mut(); }
        unsafe { std::alloc::GlobalAlloc::alloc_zeroed(&std::alloc::System, layout) }
    }
    unsafe fn realloc(&self, pointer: *mut u8, layout: std::alloc::Layout, size: usize) -> *mut u8 {
        count(true);
        if FAIL_NEXT.try_with(|flag| flag.replace(false)).unwrap_or(false) { return std::ptr::null_mut(); }
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
    for message in messages { assert!(frame.record(message).is_ok()); }
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
    assert_eq!(size_of::<Diagnostic>(), 32 + size_of::<*mut u8>() + size_of::<usize>());
    assert_eq!(offset_of!(FaultFrame, primary), 16);
    assert_eq!(offset_of!(FaultFrame, suppressed), 16 + size_of::<Diagnostic>());
    assert_eq!(size_of::<FaultFrame>(), 16 + 9 * size_of::<Diagnostic>());
    assert_eq!(align_of::<FaultFrame>(), align_of::<Diagnostic>());
}

#[test]
fn primary_and_order_survive_overflow() {
    let mut frame = FaultFrame::new();
    for site in 0..12 {
        assert!(matches!(frame.record(Diagnostic::new(1, site, [0; 2])), Ok(Status::Fault)));
    }
    let (primary, suppressed, omitted) = frame.diagnostics().unwrap();
    assert_eq!(primary.unwrap().site, 0);
    assert_eq!(suppressed.iter().map(|d| d.site).collect::<Vec<_>>(), (1..9).collect::<Vec<_>>());
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
    let rejected = frame.record(Diagnostic::new(1, 7, [3, 4]).with_message(bytes)).err().unwrap();
    assert_eq!(rejected.message().unwrap().as_ptr(), pointer);
    assert_eq!(frame.primary_present, 0);
    assert!(matches!(frame.diagnostics(), Err(Status::InvalidContract)));
}

#[test]
fn owned_and_absent_messages_have_distinct_lifetimes() {
    let mut frame = FaultFrame::new();
    assert!(Diagnostic::new(1, 0, [0; 2]).message().is_none());
    let message = Vec::from(&b"retained"[..]).into_boxed_slice();
    assert!(frame.record(Diagnostic::new(1, 0, [0; 2]).with_message(message)).is_ok());
    assert_eq!(frame.diagnostics().unwrap().0.unwrap().message(), Some(&b"retained"[..]));
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
        assert_eq!((*storage.as_ptr()).diagnostics().unwrap().0.unwrap().site, 27);
        assert_eq!(frame_dispose(pointer), Status::Normal as u32);
        assert_eq!(frame_dispose(pointer), Status::InvalidContract as u32);
        assert_eq!(report_final(pointer), -1);
        assert_eq!(record_static(pointer, 1, 28, 0, 0), Status::InvalidContract as u32);
        storage.assume_init_drop(); // only empty records remain; no second byte free
        assert_eq!(frame_init(std::ptr::null_mut()), Status::InvalidContract as u32);
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
        assert_eq!(api::allocate(pointer, 99, 1, 10, [1].as_ptr(), 1, &mut handle), 2);
        assert_eq!(handle, 777);
        assert_eq!(api::allocate(pointer, profile, 1, 10, [2].as_ptr(), 1, &mut handle), 2);
        assert_eq!(handle, 777);
        assert!(frame.diagnostics().unwrap().0.is_none());
        assert_eq!(api::allocate(pointer, profile, 1, 10, [1].as_ptr(), 1, &mut handle), 0);
        assert!(handle < 0);
        let mut failed_result = 777;
        FAIL_NEXT.with(|flag| flag.set(true));
        let allocation_status = api::allocate(pointer, profile, 2, 10, [1].as_ptr(), 1, &mut failed_result);
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
        assert_eq!(frame.diagnostics().unwrap().1.iter().map(|d| d.site).collect::<Vec<_>>(), vec![3, 5]);
        assert_eq!(frame_dispose(pointer), 0);
        handle = 777;
        assert_eq!(api::allocate(pointer, profile, 6, 10, [1].as_ptr(), 1, &mut handle), 2);
        assert_eq!(handle, 777);
    }
}
