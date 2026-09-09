use super::*;
use crate::exports::typed_object_store_backend::{
    new_checked_indexed, reclaim_checked_indexed, validate_checked_indexed_identity,
};
use std::mem::MaybeUninit;
struct Slot<T>(Box<MaybeUninit<T>>);
impl<T> Slot<T> {
    fn new() -> Self {
        Self(Box::new(MaybeUninit::uninit()))
    }
    fn ptr(&mut self) -> *mut c_void {
        self.0.as_mut_ptr().cast()
    }
}
fn child() -> i64 {
    new_checked_indexed(TypedObjectStoreBackend::SafeMutex, 919, &[]).unwrap()
}
fn live(h: i64) -> bool {
    validate_checked_indexed_identity(TypedObjectStoreBackend::SafeMutex, h, 919).is_ok()
}
unsafe fn prepare(frame: *mut c_void, key: *mut c_void, text: &[u8]) {
    assert_eq!(unsafe { key_init(key) }, 0);
    assert_eq!(
        unsafe { key_prepare(frame, 3, key, text.as_ptr(), text.len()) },
        0
    );
}
#[test]
fn opaque_install_end_and_disposal_use_real_indexed_objects() {
    let (mut f, mut m, mut k, mut o) = (
        Slot::<FaultFrame>::new(),
        Slot::<MapStorage>::new(),
        Slot::<KeyStorage>::new(),
        Slot::<OutcomeStorage>::new(),
    );
    unsafe {
        let (f, m, k, o) = (f.ptr(), m.ptr(), k.ptr(), o.ptr());
        assert_eq!(super::super::frame_init(f), 0);
        assert_eq!(map_init(m), 0);
        assert_eq!(allocate(f, 1, 1, m), 0);
        assert_eq!(map_dispose(m), 2);
        prepare(f, k, b"a\0b"); // preparation before actual child allocation
        let a = child();
        assert_eq!(outcome_init(o), 0);
        assert_eq!(install(f, 1, 4, m, k, a, 919, o), 0);
        assert_eq!(outcome_dispose(o), 2); // ReadyNoOld must also be consumed
        assert_eq!(outcome_end(f, 5, o), 0);
        assert_eq!(outcome_end(f, 5, o), 2);
        assert_eq!(outcome_dispose(o), 0);
        assert_eq!(key_dispose(k), 0);
        assert!(live(a));
        prepare(f, k, b"a\0b");
        let b = child();
        assert_eq!(outcome_init(o), 0);
        assert_eq!(install(f, 1, 6, m, k, b, 919, o), 0);
        assert_eq!(outcome_dispose(o), 2);
        assert!(live(a) && live(b));
        assert_eq!(outcome_end(f, 7, o), 0);
        assert!(!live(a) && live(b));
        assert_eq!(outcome_dispose(o), 0);
        assert_eq!(key_dispose(k), 0);
        assert_eq!(map_end(f, 8, m), 0);
        assert!(!live(b));
        assert_eq!(map_end(f, 8, m), 2);
        assert_eq!(map_dispose(m), 0);
        assert_eq!(map_dispose(m), 2);
        assert_eq!(super::super::frame_dispose(f), 0);
    }
}
#[test]
fn preflight_preserves_ready_but_attempt_fault_consumes_key() {
    let (mut f, mut m, mut k, mut o) = (
        Slot::<FaultFrame>::new(),
        Slot::<MapStorage>::new(),
        Slot::<KeyStorage>::new(),
        Slot::<OutcomeStorage>::new(),
    );
    unsafe {
        let (f, m, k, o) = (f.ptr(), m.ptr(), k.ptr(), o.ptr());
        assert_eq!(super::super::frame_init(f), 0);
        assert_eq!(map_init(m), 0);
        assert_eq!(allocate(f, 1, 1, m), 0);
        prepare(f, k, b"key");
        assert_eq!(outcome_init(o), 0);
        let h = child();
        assert_eq!(install(f, 2, 2, m, k, h, 919, o), 2);
        assert_eq!(install(f, 1, 2, m, k, h, 919, k), 2); // overlapping key/output
        assert!(matches!(
            *admit::<Mutex<KeyState>>(k, KEY_TAG)
                .unwrap()
                .lock()
                .unwrap(),
            KeyState::Ready(_)
        ));
        assert_eq!(install(f, 1, 2, m, k, h, 920, o), 1); // type mismatch is an attempt Fault
        assert!(live(h));
        assert!(matches!(
            *admit::<Mutex<KeyState>>(k, KEY_TAG)
                .unwrap()
                .lock()
                .unwrap(),
            KeyState::Consumed
        ));
        assert_eq!(outcome_end(f, 3, o), 2);
        assert_eq!((*f.cast::<FaultFrame>()).primary.reason, 101);
        assert_eq!(outcome_dispose(o), 0);
        assert_eq!(key_dispose(k), 0);
        assert_eq!(map_end(f, 4, m), 0);
        assert_eq!(map_dispose(m), 0);
        reclaim_checked_indexed(TypedObjectStoreBackend::SafeMutex, h, 919).unwrap();
        assert_eq!(super::super::frame_dispose(f), 0);
    }
}
#[test]
fn utf8_numeric_domains_and_child_fault_cancellation() {
    let (mut f, mut k) = (Slot::<FaultFrame>::new(), Slot::<KeyStorage>::new());
    unsafe {
        let (f, k) = (f.ptr(), k.ptr());
        assert_eq!(super::super::frame_init(f), 0);
        for text in [
            "",
            "0",
            "-0",
            "01",
            "+1",
            "-9223372036854775808",
            "a\0b",
            "猫",
        ] {
            prepare(f, k, text.as_bytes());
            let key = admit::<Mutex<KeyState>>(k, KEY_TAG)
                .unwrap()
                .lock()
                .unwrap();
            match &*key {
                KeyState::Ready(value) => assert_eq!(*value, MapKeyDomain::from_text(text)),
                _ => panic!(),
            }
            drop(key);
            assert_eq!(key_prepare(f, 2, k, text.as_ptr(), text.len()), 2);
            // Same native cancellation used by the child-Fault continuation.
            assert_eq!(key_dispose(k), 0);
        }
        assert_eq!(key_init(k), 0);
        assert_eq!(key_prepare(f, 3, k, b"\xff".as_ptr(), 1), 2);
        assert_eq!(key_prepare(f, 3, k, std::ptr::null(), 0), 0);
        assert_eq!(key_dispose(k), 0);
        assert_eq!(super::super::frame_dispose(f), 0);
    }
}
#[test]
fn end_report_preserves_prior_primary_and_marks_omitted_without_fake_facts() {
    let mut frame = FaultFrame::new();
    assert!(frame.record(Diagnostic::new(77, 1, [0; 2])).is_ok());
    let ptr = (&mut frame as *mut FaultFrame).cast();
    let result = MapEndReport {
        first: Some(MapEndError::InvalidIdentity),
        suppressed: [Some(MapEndError::StorageUnavailable); 8],
        suppressed_count: 20,
    };
    assert_eq!(unsafe { report(ptr, 8, result) }, 1);
    assert_eq!(frame.primary.reason, 77);
    assert_eq!(frame.suppressed_len, 8);
    assert_eq!(frame.omitted, 1);
    assert!(frame.valid());
}
#[test]
fn descriptor_contains_target_compiled_opaque_layouts() {
    let bytes = &super::super::RUNTIME_ABI_DESCRIPTOR_V2;
    assert_eq!(&bytes[..8], b"NYRTABI2");
    for (offset, size, align) in [
        (
            200,
            size_of::<MapStorage>(),
            std::mem::align_of::<MapStorage>(),
        ),
        (
            212,
            size_of::<KeyStorage>(),
            std::mem::align_of::<KeyStorage>(),
        ),
        (
            224,
            size_of::<OutcomeStorage>(),
            std::mem::align_of::<OutcomeStorage>(),
        ),
    ] {
        let word = |i| u32::from_le_bytes(bytes[i..i + 4].try_into().unwrap());
        assert_eq!(word(offset), size as u32);
        assert_eq!(word(offset + 4), align as u32);
        assert_eq!(word(offset + 8), 1);
    }
}

unsafe extern "C" {
    #[link_name = "nyash.map.checked_install_value_v1"]
    fn value_export(
        frame: *mut c_void,
        profile: u32,
        site: u64,
        map: *mut c_void,
        key: *mut c_void,
        kind: u32,
        payload: i64,
        outcome: *mut c_void,
    ) -> u32;
}

#[test]
fn value_abi_rejects_bad_bits_before_key_consumption_and_preserves_real_home() {
    let (mut f, mut m, mut k, mut o) = (
        Slot::<FaultFrame>::new(),
        Slot::<MapStorage>::new(),
        Slot::<KeyStorage>::new(),
        Slot::<OutcomeStorage>::new(),
    );
    unsafe {
        let (f, m, k, o) = (f.ptr(), m.ptr(), k.ptr(), o.ptr());
        assert_eq!(super::super::frame_init(f), 0);
        assert_eq!(map_init(m), 0);
        assert_eq!(allocate(f, 1, 1, m), 0);
        prepare(f, k, b"a");
        assert_eq!(outcome_init(o), 0);
        let a = child();
        assert_eq!(install(f, 1, 2, m, k, a, 919, o), 0);
        assert_eq!(outcome_end(f, 3, o), 0);
        assert_eq!(outcome_dispose(o), 0);
        assert_eq!(key_dispose(k), 0);
        prepare(f, k, b"a");
        assert_eq!(outcome_init(o), 0);
        for (kind, bits) in [(0, 0), (3, 0), (MAP_VALUE_BOOL, 2), (MAP_VALUE_BOOL, -1)] {
            assert_eq!(value_export(f, 1, 4, m, k, kind, bits, o), 2);
            assert!(live(a));
            assert!(matches!(
                *admit::<Mutex<KeyState>>(k, KEY_TAG)
                    .unwrap()
                    .lock()
                    .unwrap(),
                KeyState::Ready(_)
            ));
            assert!(matches!(
                *admit::<Mutex<OutcomeState>>(o, OUT_TAG)
                    .unwrap()
                    .lock()
                    .unwrap(),
                OutcomeState::Unissued
            ));
        }
        assert_eq!(value_export(f, 1, 4, m, k, MAP_VALUE_I64, 30, o), 0);
        assert_eq!(outcome_dispose(o), 2);
        assert!(live(a));
        // Invalidate only this physical identity to exercise old-end Fault.
        reclaim_checked_indexed(TypedObjectStoreBackend::SafeMutex, a, 919).unwrap();
        assert_eq!(outcome_end(f, 5, o), 1);
        assert_eq!(outcome_dispose(o), 0);
        assert_eq!(key_dispose(k), 0);
        assert_eq!(map_end(f, 6, m), 0); // installed Value needs no indexed reclaim
        assert_eq!(map_end(f, 6, m), 2);
        assert_eq!(map_dispose(m), 0);
        assert_eq!(super::super::frame_dispose(f), 0);
    }
}

#[test]
fn value_abi_mixed_replacement_never_treats_integer_bits_as_a_handle() {
    let (mut f, mut m, mut k, mut o) = (
        Slot::<FaultFrame>::new(),
        Slot::<MapStorage>::new(),
        Slot::<KeyStorage>::new(),
        Slot::<OutcomeStorage>::new(),
    );
    unsafe {
        let (f, m, k, o) = (f.ptr(), m.ptr(), k.ptr(), o.ptr());
        assert_eq!(super::super::frame_init(f), 0);
        assert_eq!(map_init(m), 0);
        assert_eq!(allocate(f, 1, 1, m), 0);
        let caller_owned = child();
        for (kind, value) in [
            (MAP_VALUE_I64, i64::MIN),
            (MAP_VALUE_I64, i64::MAX),
            (MAP_VALUE_BOOL, 0),
            (MAP_VALUE_BOOL, 1),
            (MAP_VALUE_I64, caller_owned),
        ] {
            prepare(f, k, b"a");
            assert_eq!(outcome_init(o), 0);
            assert_eq!(value_export(f, 1, 2, m, k, kind, value, o), 0);
            assert_eq!(outcome_end(f, 3, o), 0);
            assert_eq!(outcome_dispose(o), 0);
            assert_eq!(key_dispose(k), 0);
            assert!(live(caller_owned));
        }
        prepare(f, k, b"a");
        assert_eq!(outcome_init(o), 0);
        let transferred = child();
        assert_eq!(install(f, 1, 4, m, k, transferred, 919, o), 0);
        assert_eq!(outcome_end(f, 5, o), 0); // old integer bits are not a Home
        assert_eq!(outcome_dispose(o), 0);
        assert_eq!(key_dispose(k), 0);
        assert!(live(caller_owned) && live(transferred));
        assert!(matches!(
            admit::<CheckedMap>(m, MAP_TAG)
                .unwrap()
                .observe_native(&MapKeyDomain::from_text("a")),
            Err(CheckedMapError::ProjectionUnavailable)
        ));
        assert_eq!(map_end(f, 6, m), 0);
        assert!(!live(transferred) && live(caller_owned));
        assert_eq!(map_dispose(m), 0);
        reclaim_checked_indexed(TypedObjectStoreBackend::SafeMutex, caller_owned, 919).unwrap();
        assert_eq!(super::super::frame_dispose(f), 0);
    }
}
