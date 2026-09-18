use super::*;
use crate::exports::typed_object_store_backend::{
    new_checked_indexed, reclaim_checked_indexed, validate_checked_indexed_identity,
};
use nyash_rust::boxes::map_box::checked::{CheckedMap, CheckedMapPayload, OwnedMapArrayResidence};
use std::mem::MaybeUninit;
use std::sync::Arc;
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

unsafe fn live_map(ptr: *mut c_void) -> bool {
    match admit::<CheckedMap>(ptr, MAP_TAG) {
        Ok(map) => map.require_live().is_ok(),
        Err(_) => false,
    }
}

unsafe fn install_text_entry(
    frame: *mut c_void,
    map: *mut c_void,
    key: *mut c_void,
    out: *mut c_void,
) {
    assert_eq!(map_init(map), 0);
    assert_eq!(allocate(frame, 1, 10, map), 0);
    prepare(frame, key, b"name");
    assert_eq!(outcome_init(out), 0);
    assert_eq!(
        install_text(frame, 1, 11, map, key, b"main".as_ptr(), 4, out),
        0
    );
    assert_eq!(outcome_end(frame, 12, out), 0);
    assert_eq!(outcome_dispose(out), 0);
    assert_eq!(key_dispose(key), 0);
}

#[test]
fn borrowed_array_install_keeps_duplicate_map_local_roots_caller_owned() {
    let (mut f, mut parent, mut key, mut out, mut child, mut child2, mut child_key, mut child_out) = (
        Slot::<FaultFrame>::new(),
        Slot::<MapStorage>::new(),
        Slot::<KeyStorage>::new(),
        Slot::<OutcomeStorage>::new(),
        Slot::<MapStorage>::new(),
        Slot::<MapStorage>::new(),
        Slot::<KeyStorage>::new(),
        Slot::<OutcomeStorage>::new(),
    );
    unsafe {
        let (f, parent, key, out, child, child2, child_key, child_out) = (
            f.ptr(),
            parent.ptr(),
            key.ptr(),
            out.ptr(),
            child.ptr(),
            child2.ptr(),
            child_key.ptr(),
            child_out.ptr(),
        );
        assert_eq!(super::super::frame_init(f), 0);
        install_text_entry(f, child, child_key, child_out);
        install_text_entry(f, child2, child_key, child_out);
        assert_eq!(map_init(parent), 0);
        assert_eq!(allocate(f, 1, 20, parent), 0);
        prepare(f, key, b"functions");
        assert_eq!(outcome_init(out), 0);
        let elements = [child, child];
        assert_eq!(
            install_borrowed_array(
                f,
                1,
                21,
                parent,
                key,
                elements.as_ptr(),
                elements.len(),
                out,
            ),
            0
        );
        assert_eq!(outcome_end(f, 22, out), 0);
        assert_eq!(outcome_dispose(out), 0);
        assert_eq!(key_dispose(key), 0);
        assert!(live_map(parent));
        assert_eq!(map_end(f, 23, parent), 0);
        assert!(live_map(child));
        assert!(live_map(child2));
        assert_eq!(map_dispose(parent), 0);
        assert_eq!(map_end(f, 24, child), 0);
        assert_eq!(map_end(f, 25, child2), 0);
        assert_eq!(map_dispose(child), 0);
        assert_eq!(map_dispose(child2), 0);
        assert_eq!(super::super::frame_dispose(f), 0);
    }
}

#[test]
fn borrowed_array_install_rejects_foreign_element_without_consuming_child() {
    let (mut f, mut parent, mut key, mut out, mut child, mut child_key, mut child_out) = (
        Slot::<FaultFrame>::new(),
        Slot::<MapStorage>::new(),
        Slot::<KeyStorage>::new(),
        Slot::<OutcomeStorage>::new(),
        Slot::<MapStorage>::new(),
        Slot::<KeyStorage>::new(),
        Slot::<OutcomeStorage>::new(),
    );
    unsafe {
        let (f, parent, key, out, child, child_key, child_out) = (
            f.ptr(),
            parent.ptr(),
            key.ptr(),
            out.ptr(),
            child.ptr(),
            child_key.ptr(),
            child_out.ptr(),
        );
        assert_eq!(super::super::frame_init(f), 0);
        install_text_entry(f, child, child_key, child_out);
        assert_eq!(map_init(parent), 0);
        assert_eq!(allocate(f, 1, 30, parent), 0);
        prepare(f, key, b"functions");
        assert_eq!(outcome_init(out), 0);
        let elements = [child, std::ptr::null_mut()];
        assert_eq!(
            install_borrowed_array(
                f,
                1,
                31,
                parent,
                key,
                elements.as_ptr(),
                elements.len(),
                out,
            ),
            1
        );
        assert_eq!(outcome_dispose(out), 0);
        assert_eq!(key_dispose(key), 0);
        assert!(live_map(child));
        assert_eq!(map_end(f, 32, parent), 0);
        assert_eq!(map_dispose(parent), 0);
        assert_eq!(map_end(f, 33, child), 0);
        assert_eq!(map_dispose(child), 0);
        assert_eq!(super::super::frame_dispose(f), 0);
    }
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
fn storage_move_transfers_live_lease_and_leaves_source_disposable() {
    let (mut f, mut m, mut k, mut o, mut d, mut d2) = (
        Slot::<FaultFrame>::new(),
        Slot::<MapStorage>::new(),
        Slot::<KeyStorage>::new(),
        Slot::<OutcomeStorage>::new(),
        Slot::<MapStorage>::new(),
        Slot::<MapStorage>::new(),
    );
    unsafe {
        let (f, m, k, o, d, d2) = (f.ptr(), m.ptr(), k.ptr(), o.ptr(), d.ptr(), d2.ptr());
        assert_eq!(super::super::frame_init(f), 0);
        assert_eq!(map_init(m), 0);
        assert_eq!(allocate(f, 1, 1, m), 0);
        prepare(f, k, b"a");
        assert_eq!(outcome_init(o), 0);
        let h = child();
        assert_eq!(install(f, 1, 2, m, k, h, 919, o), 0);
        assert_eq!(outcome_end(f, 3, o), 0);
        assert_eq!(outcome_dispose(o), 0);
        assert_eq!(key_dispose(k), 0);
        assert_eq!(map_move(std::ptr::null_mut(), m), 2);
        assert_eq!(map_move(m, m), 2);
        assert_eq!(map_move(d, m), 0);
        // The moved-from placement is unissued: not live, not movable again.
        assert_eq!(map_move(d2, m), 2);
        assert!(live(h));
        // The moved lease still owns its transferred entry; End releases it.
        assert_eq!(map_end(f, 4, d), 0);
        assert!(!live(h));
        assert_eq!(map_dispose(d), 0);
        assert_eq!(map_dispose(m), 0);
        assert_eq!(map_dispose(d2), 2);
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
        for (kind, bits) in [(0, 0), (4, 0), (MAP_VALUE_BOOL, 2), (MAP_VALUE_BOOL, -1)] {
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

unsafe extern "C" {
    #[link_name = "nyash.map.checked_install_text_v1"]
    fn text_export(
        frame: *mut c_void,
        profile: u32,
        site: u64,
        map: *mut c_void,
        key: *mut c_void,
        bytes: *const u8,
        len: usize,
        outcome: *mut c_void,
    ) -> u32;
}

#[test]
fn text_abi_owns_validated_bytes_and_rejects_invalid_utf8_before_key_consumption() {
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
        // Invalid UTF-8 is a contract reject: the key is not consumed and
        // the outcome stays Unissued.
        assert_eq!(text_export(f, 1, 4, m, k, b"\xff".as_ptr(), 1, o), 2);
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
        // Valid bytes install: outcome publishes, the key is consumed.
        let text = "猫";
        assert_eq!(text_export(f, 1, 4, m, k, text.as_ptr(), text.len(), o), 0);
        assert_eq!(outcome_dispose(o), 2); // Ready outcome must be consumed
        assert_eq!(outcome_end(f, 5, o), 0); // ReadyNoOld
        assert_eq!(outcome_dispose(o), 0);
        // Replacement detaches the owned Text; ending it is a no-op.
        prepare(f, k, b"a");
        assert_eq!(outcome_init(o), 0);
        assert_eq!(text_export(f, 1, 6, m, k, b"other".as_ptr(), 5, o), 0);
        assert_eq!(outcome_end(f, 7, o), 0);
        assert_eq!(outcome_dispose(o), 0);
        assert_eq!(key_dispose(k), 0);
        // An empty payload is valid text (null pointer, zero length).
        prepare(f, k, b"b");
        assert_eq!(outcome_init(o), 0);
        assert_eq!(text_export(f, 1, 8, m, k, std::ptr::null(), 0, o), 0);
        assert_eq!(outcome_end(f, 9, o), 0);
        assert_eq!(outcome_dispose(o), 0);
        assert_eq!(key_dispose(k), 0);
        assert_eq!(map_end(f, 10, m), 0);
        assert_eq!(map_dispose(m), 0);
        assert_eq!(super::super::frame_dispose(f), 0);
    }
}

unsafe extern "C" {
    #[link_name = "nyash.map.checked_install_empty_array_v1"]
    fn empty_array_export(
        frame: *mut c_void,
        profile: u32,
        site: u64,
        map: *mut c_void,
        key: *mut c_void,
        outcome: *mut c_void,
    ) -> u32;
}

#[test]
fn empty_array_abi_installs_an_owned_marker_through_the_shared_protocol() {
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
        // The marker shares the preflight/commit/outcome protocol.
        assert_eq!(empty_array_export(f, 1, 4, m, k, o), 0);
        assert_eq!(outcome_dispose(o), 2); // Ready outcome must be consumed
        assert_eq!(outcome_end(f, 5, o), 0); // ReadyNoOld
        assert_eq!(outcome_dispose(o), 0);
        assert_eq!(key_dispose(k), 0);
        // Replacement detaches the marker; ending it is a no-op.
        prepare(f, k, b"a");
        assert_eq!(outcome_init(o), 0);
        let h = child();
        assert_eq!(install(f, 1, 6, m, k, h, 919, o), 0);
        assert_eq!(outcome_end(f, 7, o), 0);
        assert_eq!(outcome_dispose(o), 0);
        assert_eq!(key_dispose(k), 0);
        assert!(live(h));
        assert_eq!(map_end(f, 8, m), 0);
        assert!(!live(h));
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

unsafe extern "C" {
    #[link_name = "nyash.map.checked_get_i64_v1"]
    fn get_i64_export(
        frame: *mut c_void,
        site: u64,
        map: *mut c_void,
        bytes: *const u8,
        len: usize,
        out: *mut i64,
    ) -> u32;
}

#[test]
fn value_abi_borrowed_handle_never_claims_or_reads_back_the_target() {
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
        // Kind 3 installs the caller's bits as a non-owning snapshot.
        prepare(f, k, b"a");
        assert_eq!(outcome_init(o), 0);
        assert_eq!(
            value_export(f, 1, 2, m, k, MAP_VALUE_BORROWED_HANDLE, caller_owned, o),
            0
        );
        assert_eq!(outcome_end(f, 3, o), 0);
        assert_eq!(outcome_dispose(o), 0);
        assert_eq!(key_dispose(k), 0);
        assert!(live(caller_owned));
        // A scalar read must never expose the handle bits as an integer.
        let mut out = MaybeUninit::<i64>::new(-1);
        assert_eq!(
            get_i64_export(f, 4, m, b"a".as_ptr(), 1, out.as_mut_ptr()),
            1
        );
        assert_eq!((*f.cast::<FaultFrame>()).primary.reason, 104);
        assert_eq!(out.assume_init(), -1);
        // Replacing the entry ends the borrowed payload — a no-op that
        // leaves the caller's object live.
        prepare(f, k, b"a");
        assert_eq!(outcome_init(o), 0);
        assert_eq!(value_export(f, 1, 5, m, k, MAP_VALUE_I64, 7, o), 0);
        assert_eq!(outcome_end(f, 6, o), 0);
        assert_eq!(outcome_dispose(o), 0);
        assert_eq!(key_dispose(k), 0);
        assert!(live(caller_owned));
        // Map end releases nothing for a borrowed snapshot either.
        prepare(f, k, b"a");
        assert_eq!(outcome_init(o), 0);
        assert_eq!(
            value_export(f, 1, 7, m, k, MAP_VALUE_BORROWED_HANDLE, caller_owned, o),
            0
        );
        assert_eq!(outcome_end(f, 8, o), 0);
        assert_eq!(outcome_dispose(o), 0);
        assert_eq!(key_dispose(k), 0);
        assert_eq!(map_end(f, 9, m), 0);
        assert!(live(caller_owned));
        assert_eq!(map_dispose(m), 0);
        reclaim_checked_indexed(TypedObjectStoreBackend::SafeMutex, caller_owned, 919).unwrap();
        assert_eq!(super::super::frame_dispose(f), 0);
    }
}

fn text_child(value: CheckedMapPayload) -> Arc<CheckedMap> {
    let child = Arc::new(CheckedMap::unissued());
    child.acquire().unwrap();
    child
        .install(MapKeyDomain::from_text("name"), value)
        .map_err(|failure| failure.error)
        .unwrap()
        .end()
        .unwrap();
    child
}

unsafe fn install_array_parent(map: *mut c_void, child: Arc<CheckedMap>) {
    let mut residence = OwnedMapArrayResidence::builder(1).unwrap();
    residence.push_map(child).unwrap();
    let payload = CheckedMapPayload::Array(Box::new(residence.finish()));
    admit::<CheckedMap>(map, MAP_TAG)
        .unwrap()
        .install(MapKeyDomain::from_text("functions"), payload)
        .map_err(|failure| failure.error)
        .unwrap()
        .end()
        .unwrap();
}

#[test]
fn array_index_map_then_get_text_borrows_child_bytes_and_consumes_view() {
    let (mut f, mut m, mut view, mut text) = (
        Slot::<FaultFrame>::new(),
        Slot::<MapStorage>::new(),
        Slot::<MapViewStorage>::new(),
        Slot::<TextViewStorage>::new(),
    );
    unsafe {
        let (f, m, view, text) = (f.ptr(), m.ptr(), view.ptr(), text.ptr());
        assert_eq!(super::super::frame_init(f), 0);
        assert_eq!(map_init(m), 0);
        assert_eq!(allocate(f, 1, 1, m), 0);
        let child = text_child(CheckedMapPayload::Text("needle".into()));
        install_array_parent(m, child);
        assert_eq!(
            array_index_map(f, 2, m, b"functions".as_ptr(), 9, 0, view),
            0
        );
        assert_eq!(get_text(f, 3, view, b"name".as_ptr(), 4, text), 0);
        let result = admit::<TextView>(text, TEXT_VIEW_TAG).unwrap();
        let bytes = std::slice::from_raw_parts(result.bytes, result.len);
        assert_eq!(bytes, b"needle");
        assert_eq!(
            admit::<MapView>(view, VIEW_TAG).unwrap().state,
            VIEW_CONSUMED
        );
        assert_eq!(get_text(f, 4, view, b"name".as_ptr(), 4, text), 2);
        assert_eq!(map_end(f, 5, m), 0);
        assert_eq!(map_dispose(m), 0);
        assert_eq!(super::super::frame_dispose(f), 0);
    }
}

#[test]
fn array_and_text_view_faults_are_named_and_parent_remains_endable() {
    let (mut f, mut m, mut view) = (
        Slot::<FaultFrame>::new(),
        Slot::<MapStorage>::new(),
        Slot::<MapViewStorage>::new(),
    );
    unsafe {
        let (f, m, view) = (f.ptr(), m.ptr(), view.ptr());
        assert_eq!(super::super::frame_init(f), 0);
        assert_eq!(map_init(m), 0);
        assert_eq!(allocate(f, 1, 1, m), 0);
        let child = text_child(CheckedMapPayload::I64(7));
        install_array_parent(m, child);
        assert_eq!(
            array_index_map(f, 2, m, b"functions".as_ptr(), 9, 1, view),
            1
        );
        assert_eq!(
            (*f.cast::<FaultFrame>()).primary.reason,
            MAP_ARRAY_BOUNDS_REASON
        );
        assert_eq!(map_end(f, 3, m), 0);
        assert_eq!(map_dispose(m), 0);
        assert_eq!(super::super::frame_dispose(f), 0);
    }
}
