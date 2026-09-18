use super::*;
use std::sync::{Arc, Weak};

struct Probe {
    id: u32,
    map: Weak<CheckedMap>,
    events: Arc<Mutex<Vec<u32>>>,
    failure: Option<MapEndError>,
    ending: bool,
}
impl CanonicalMapResidence for Probe {
    fn end(self: Box<Self>) -> Result<(), MapEndError> {
        if let Some(map) = self.map.upgrade() {
            let result = map.observe_native(&MapKeyDomain::from_text("a"));
            assert_eq!(
                result.err(),
                Some(if self.ending {
                    CheckedMapError::InvalidState
                } else {
                    CheckedMapError::ProjectionUnavailable
                })
            );
            if self.ending {
                assert_eq!(map.acquire(), Err(CheckedMapError::InvalidState));
                assert_eq!(map.end(), Err(CheckedMapError::InvalidState));
                let rejected = map
                    .install(
                        MapKeyDomain::from_text("late"),
                        CheckedMapPayload::Residence(Box::new(Probe {
                            id: 999,
                            map: Weak::new(),
                            events: self.events.clone(),
                            failure: None,
                            ending: false,
                        })),
                    )
                    .err()
                    .expect("reentry must refuse install");
                assert_eq!(rejected.error, CheckedMapError::InvalidState);
                // No transfer occurred, so the callback still owns this attempt.
                drop(rejected.candidate);
            }
        }
        self.events.lock().unwrap().push(self.id);
        self.failure.map_or(Ok(()), Err)
    }
}
fn candidate(
    map: &Arc<CheckedMap>,
    events: &Arc<Mutex<Vec<u32>>>,
    id: u32,
    ending: bool,
    failure: Option<MapEndError>,
) -> CheckedMapPayload {
    CheckedMapPayload::Residence(Box::new(Probe {
        id,
        map: Arc::downgrade(map),
        events: events.clone(),
        failure,
        ending,
    }))
}
fn install(map: &CheckedMap, key: &str, value: CheckedMapPayload) -> DetachedMapEntry {
    match map.install(MapKeyDomain::from_text(key), value) {
        Ok(old) => old,
        Err(error) => panic!("unexpected install refusal: {:?}", error.error),
    }
}

#[test]
fn replacement_and_terminal_fault_preserve_order_and_refuse_reentry() {
    let map = Arc::new(CheckedMap::unissued());
    let events = Arc::new(Mutex::new(Vec::new()));
    map.require_disposable().unwrap();
    map.acquire().unwrap();
    assert_eq!(map.require_disposable(), Err(CheckedMapError::InvalidState));
    install(
        &map,
        "a",
        candidate(&map, &events, 1, false, Some(MapEndError::InvalidIdentity)),
    )
    .end()
    .unwrap();
    install(
        &map,
        "b",
        candidate(&map, &events, 2, true, Some(MapEndError::ProfileMismatch)),
    )
    .end()
    .unwrap();
    let old = install(
        &map,
        "a",
        candidate(
            &map,
            &events,
            3,
            true,
            Some(MapEndError::StorageUnavailable),
        ),
    );
    assert!(events.lock().unwrap().is_empty());
    assert_eq!(old.end(), Err(MapEndError::InvalidIdentity));
    assert_eq!(*events.lock().unwrap(), vec![1]);
    let report = map.end().unwrap();
    assert_eq!(*events.lock().unwrap(), vec![1, 3, 2]);
    assert_eq!(report.first, Some(MapEndError::StorageUnavailable));
    assert_eq!(report.suppressed[0], Some(MapEndError::ProfileMismatch));
    assert_eq!(report.suppressed_count, 1);
    assert_eq!(map.end(), Err(CheckedMapError::InvalidState));
    assert_eq!(map.acquire(), Err(CheckedMapError::InvalidState));
    map.require_disposable().unwrap();
}

#[test]
fn precommit_refusal_returns_the_same_candidate_and_keeps_live_entry() {
    let map = Arc::new(CheckedMap::unissued());
    let events = Arc::new(Mutex::new(Vec::new()));
    let value = candidate(&map, &events, 1, true, None);
    let address = residence_address(&value);
    let failed = map
        .install(MapKeyDomain::from_text("a"), value)
        .err()
        .unwrap();
    assert_eq!(residence_address(&failed.candidate), address);
    assert_eq!(failed.error, CheckedMapError::InvalidState);
    map.acquire().unwrap();
    install(&map, "a", failed.candidate).end().unwrap();
    map.state.lock().unwrap().next_order = u64::MAX;
    let failed = map
        .install(
            MapKeyDomain::from_text("a"),
            candidate(&map, &events, 2, true, None),
        )
        .err()
        .unwrap();
    assert_eq!(failed.error, CheckedMapError::OrderExhausted);
    assert!(events.lock().unwrap().is_empty());
    assert!(matches!(
        map.observe_native(&MapKeyDomain::from_text("a")),
        Err(CheckedMapError::ProjectionUnavailable)
    ));
    assert!(map
        .observe_native(&MapKeyDomain::from_text("missing"))
        .unwrap()
        .is_none());
    drop(failed.candidate);
    assert_eq!(map.end().unwrap().first, None);
    assert_eq!(*events.lock().unwrap(), vec![1]);
}

#[test]
fn poisoned_storage_refuses_install_but_still_ends_all_entries() {
    let map = Arc::new(CheckedMap::unissued());
    let events = Arc::new(Mutex::new(Vec::new()));
    map.acquire().unwrap();
    // No reentry observer here: poison is itself the source of refusal.
    install(
        &map,
        "a",
        CheckedMapPayload::Residence(Box::new(Probe {
            id: 1,
            map: Weak::new(),
            events: events.clone(),
            failure: None,
            ending: true,
        })),
    )
    .end()
    .unwrap();
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _state = map.state.lock().unwrap();
        panic!("real lock poison");
    }));
    let failed = map
        .install(
            MapKeyDomain::from_text("b"),
            candidate(&map, &events, 2, true, None),
        )
        .err()
        .unwrap();
    assert_eq!(failed.error, CheckedMapError::StorageUnavailable);
    drop(failed.candidate);
    assert_eq!(
        map.end().unwrap().first,
        Some(MapEndError::StorageUnavailable)
    );
    assert_eq!(*events.lock().unwrap(), vec![1]);
    map.require_disposable().unwrap();
}

#[test]
fn empty_map_and_bounded_failure_report_have_explicit_outcomes() {
    let map = CheckedMap::unissued();
    map.acquire().unwrap();
    assert_eq!(map.end().unwrap(), MapEndReport::default());
    let mut report = MapEndReport::default();
    for _ in 0..12 {
        report.record(MapEndError::InvalidIdentity);
    }
    assert_eq!(report.first, Some(MapEndError::InvalidIdentity));
    assert_eq!(report.suppressed, [Some(MapEndError::InvalidIdentity); 8]);
    assert_eq!(report.suppressed_count, 11); // total, not stored count
}

#[test]
fn checked_facade_cannot_be_cloned_or_published_as_nyash_box() {
    // Trait-inference ambiguity makes this test fail to compile if either
    // forbidden implementation is added, including an indirect Clone derive.
    macro_rules! not_impl {
        ($ty:ty, $bound:path) => {{
            trait Ambiguous<A> {
                fn check() {}
            }
            impl<T: ?Sized> Ambiguous<()> for T {}
            impl<T: ?Sized + $bound> Ambiguous<u8> for T {}
            let _ = <$ty as Ambiguous<_>>::check;
        }};
    }
    not_impl!(CheckedMap, Clone);
    not_impl!(CheckedMap, crate::box_trait::NyashBox);
    not_impl!(DetachedMapEntry, Clone);
}

fn residence_address(value: &CheckedMapPayload) -> *const () {
    match value {
        CheckedMapPayload::Residence(value) => {
            value.as_ref() as *const dyn CanonicalMapResidence as *const ()
        }
        _ => panic!("expected the same residence"),
    }
}

#[test]
fn inline_values_preserve_payload_on_rejection_and_detachment() {
    for value in [
        CheckedMapPayload::I64(i64::MIN),
        CheckedMapPayload::I64(i64::MAX),
        CheckedMapPayload::Bool(false),
        CheckedMapPayload::Bool(true),
    ] {
        let map = CheckedMap::unissued();
        let failed = map
            .install(MapKeyDomain::from_text("a"), value)
            .err()
            .unwrap();
        assert_eq!(failed.error, CheckedMapError::InvalidState);
        let expected = match &failed.candidate {
            CheckedMapPayload::I64(value) => (false, *value),
            CheckedMapPayload::Bool(value) => (true, i64::from(*value)),
            _ => panic!("value must not become a residence"),
        };
        map.acquire().unwrap();
        install(&map, "a", failed.candidate).end().unwrap();
        let old = install(&map, "a", CheckedMapPayload::I64(7));
        match old.0.as_ref().unwrap() {
            CheckedMapPayload::I64(value) => assert_eq!((false, *value), expected),
            CheckedMapPayload::Bool(value) => assert_eq!((true, i64::from(*value)), expected),
            _ => panic!("detached payload changed"),
        }
        old.end().unwrap();
        assert!(matches!(
            map.observe_native(&MapKeyDomain::from_text("a")),
            Err(CheckedMapError::ProjectionUnavailable)
        ));
        assert_eq!(map.end().unwrap(), MapEndReport::default());
        map.require_disposable().unwrap();
    }
}

#[test]
fn text_payload_owns_bytes_through_rejection_detachment_and_end() {
    // Text is an inline-owned payload: rejection returns the same bytes,
    // replacement detaches them unchanged, and end is a no-op with no
    // Home obligation — unlike a Residence or a shared/interned handle.
    let map = CheckedMap::unissued();
    let failed = map
        .install(
            MapKeyDomain::from_text("a"),
            CheckedMapPayload::Text("owned".into()),
        )
        .err()
        .unwrap();
    assert_eq!(failed.error, CheckedMapError::InvalidState);
    assert!(matches!(
        &failed.candidate,
        CheckedMapPayload::Text(text) if text.as_ref() == "owned"
    ));
    map.acquire().unwrap();
    install(&map, "a", failed.candidate).end().unwrap();
    let old = install(&map, "a", CheckedMapPayload::Text("next".into()));
    assert!(matches!(
        &old.0,
        Some(CheckedMapPayload::Text(text)) if text.as_ref() == "owned"
    ));
    old.end().unwrap();
    assert_eq!(map.end().unwrap(), MapEndReport::default());
    map.require_disposable().unwrap();
}

#[test]
fn empty_array_payload_is_an_owned_marker_with_trivial_end() {
    // EmptyArray carries empty-array meaning with no bytes, handle, or
    // Home obligation: rejection returns the marker, replacement detaches
    // it unchanged, and end is a no-op.
    let map = CheckedMap::unissued();
    let failed = map
        .install(MapKeyDomain::from_text("a"), CheckedMapPayload::EmptyArray)
        .err()
        .unwrap();
    assert_eq!(failed.error, CheckedMapError::InvalidState);
    assert!(matches!(failed.candidate, CheckedMapPayload::EmptyArray));
    map.acquire().unwrap();
    install(&map, "a", failed.candidate).end().unwrap();
    let old = install(&map, "a", CheckedMapPayload::EmptyArray);
    assert!(matches!(old.0, Some(CheckedMapPayload::EmptyArray)));
    old.end().unwrap();
    assert_eq!(map.end().unwrap(), MapEndReport::default());
    map.require_disposable().unwrap();
}

#[test]
fn borrowed_handle_payload_reads_non_scalar_and_ends_trivially() {
    // A borrowed-handle entry stores a non-consuming snapshot: the map
    // never owns the target, so install/detach/end carry no Home
    // obligation — and a scalar read reports NonScalar (the ABI's Fault
    // 104) rather than conflating handle bits with an I64.
    let map = CheckedMap::unissued();
    let failed = map
        .install(
            MapKeyDomain::from_text("a"),
            CheckedMapPayload::BorrowedHandle(0x5AFE),
        )
        .err()
        .unwrap();
    assert_eq!(failed.error, CheckedMapError::InvalidState);
    assert!(matches!(
        failed.candidate,
        CheckedMapPayload::BorrowedHandle(0x5AFE)
    ));
    map.acquire().unwrap();
    install(&map, "a", failed.candidate).end().unwrap();
    assert!(matches!(
        map.read_i64(&MapKeyDomain::from_text("a")),
        Ok(CheckedMapI64Read::NonScalar)
    ));
    let old = install(&map, "a", CheckedMapPayload::I64(7));
    assert!(matches!(
        old.0,
        Some(CheckedMapPayload::BorrowedHandle(0x5AFE))
    ));
    old.end().unwrap();
    assert_eq!(map.end().unwrap(), MapEndReport::default());
    map.require_disposable().unwrap();
}

#[test]
fn value_replacement_after_failed_residence_end_keeps_new_slot_and_other_homes() {
    let map = Arc::new(CheckedMap::unissued());
    let events = Arc::new(Mutex::new(Vec::new()));
    map.acquire().unwrap();
    install(&map, "a", CheckedMapPayload::Bool(true))
        .end()
        .unwrap();
    install(
        &map,
        "a",
        candidate(&map, &events, 1, false, Some(MapEndError::InvalidIdentity)),
    )
    .end()
    .unwrap();
    install(&map, "b", candidate(&map, &events, 2, true, None))
        .end()
        .unwrap();
    let old = install(&map, "a", CheckedMapPayload::I64(30));
    assert_eq!(old.end(), Err(MapEndError::InvalidIdentity));
    assert!(matches!(
        map.state
            .lock()
            .unwrap()
            .entries
            .get(&MapKeyDomain::from_text("a"))
            .unwrap()
            .payload,
        CheckedMapPayload::I64(30)
    ));
    assert_eq!(map.end().unwrap(), MapEndReport::default());
    assert_eq!(*events.lock().unwrap(), [1, 2]);
    map.require_disposable().unwrap();
}

fn text_child(value: &str) -> Arc<CheckedMap> {
    let map = Arc::new(CheckedMap::unissued());
    map.acquire().unwrap();
    install(&map, "name", CheckedMapPayload::Text(value.into()))
        .end()
        .unwrap();
    map
}

#[test]
fn owned_array_reads_nested_map_text_and_releases_duplicate_root_once() {
    let child = text_child("main");
    let mut builder = OwnedMapArrayResidence::builder(2).unwrap();
    builder.push_map(Arc::clone(&child)).unwrap();
    builder.push_map(Arc::clone(&child)).unwrap();
    let array = builder.finish();

    let map = CheckedMap::unissued();
    map.acquire().unwrap();
    install(&map, "functions", CheckedMapPayload::Array(Box::new(array)))
        .end()
        .unwrap();

    let view = map
        .read_array_map(&MapKeyDomain::from_text("functions"), 0)
        .unwrap();
    assert!(matches!(
        view.read_text(&MapKeyDomain::from_text("name")),
        Ok(CheckedMapTextRead::Value(value)) if value.as_ref() == "main"
    ));
    let second = map
        .read_array_map(&MapKeyDomain::from_text("functions"), 1)
        .unwrap();
    assert!(matches!(
        second.read_text(&MapKeyDomain::from_text("name")),
        Ok(CheckedMapTextRead::Value(value)) if value.as_ref() == "main"
    ));
    assert!(matches!(
        map.read_array_map(&MapKeyDomain::from_text("functions"), 2),
        Err(CheckedMapArrayReadError::Bounds)
    ));
    drop((view, second));
    assert_eq!(map.end().unwrap(), MapEndReport::default());
    child.require_disposable().unwrap();
    map.require_disposable().unwrap();
}

#[test]
fn array_builder_fault_releases_acquired_prefix_in_reverse_root_order() {
    let events = Arc::new(Mutex::new(Vec::new()));
    let first = Arc::new(CheckedMap::unissued());
    first.acquire().unwrap();
    install(&first, "name", candidate(&first, &events, 1, true, None))
        .end()
        .unwrap();
    let second = Arc::new(CheckedMap::unissued());
    second.acquire().unwrap();
    install(&second, "name", candidate(&second, &events, 2, true, None))
        .end()
        .unwrap();
    let invalid = Arc::new(CheckedMap::unissued());

    let mut builder = OwnedMapArrayResidence::builder(3).unwrap();
    builder.push_map(Arc::clone(&first)).unwrap();
    builder.push_map(Arc::clone(&second)).unwrap();
    assert_eq!(builder.push_map(invalid), Err(MapEndError::InvalidIdentity));
    assert_eq!(*events.lock().unwrap(), vec![2, 1]);
    first.require_disposable().unwrap();
    second.require_disposable().unwrap();
}

#[test]
fn array_candidate_is_returned_unchanged_when_map_install_refuses_it() {
    let child = text_child("main");
    let mut builder = OwnedMapArrayResidence::builder(1).unwrap();
    builder.push_map(Arc::clone(&child)).unwrap();
    let payload = CheckedMapPayload::Array(Box::new(builder.finish()));
    let address = match &payload {
        CheckedMapPayload::Array(value) => {
            value.as_ref() as *const dyn CanonicalMapArrayResidence as *const ()
        }
        _ => unreachable!(),
    };
    let map = CheckedMap::unissued();
    let failed = map
        .install(MapKeyDomain::from_text("functions"), payload)
        .err()
        .unwrap();
    assert_eq!(failed.error, CheckedMapError::InvalidState);
    match &failed.candidate {
        CheckedMapPayload::Array(value) => {
            assert_eq!(
                value.as_ref() as *const dyn CanonicalMapArrayResidence as *const (),
                address
            );
        }
        _ => panic!("array candidate changed shape"),
    }
    map.acquire().unwrap();
    install(&map, "functions", failed.candidate).end().unwrap();
    assert_eq!(map.end().unwrap(), MapEndReport::default());
    child.require_disposable().unwrap();
}

#[test]
fn array_index_and_text_lookup_reject_missing_kind_and_bounds() {
    let child = Arc::new(CheckedMap::unissued());
    child.acquire().unwrap();
    install(&child, "name", CheckedMapPayload::I64(7))
        .end()
        .unwrap();
    let mut builder = OwnedMapArrayResidence::builder(1).unwrap();
    builder.push_map(Arc::clone(&child)).unwrap();

    let map = CheckedMap::unissued();
    map.acquire().unwrap();
    install(
        &map,
        "functions",
        CheckedMapPayload::Array(Box::new(builder.finish())),
    )
    .end()
    .unwrap();
    install(&map, "scalar", CheckedMapPayload::I64(1))
        .end()
        .unwrap();
    assert!(matches!(
        map.read_array_map(&MapKeyDomain::from_text("missing"), 0),
        Err(CheckedMapArrayReadError::Missing)
    ));
    assert!(matches!(
        map.read_array_map(&MapKeyDomain::from_text("functions"), -1),
        Err(CheckedMapArrayReadError::Bounds)
    ));
    assert!(matches!(
        map.read_array_map(&MapKeyDomain::from_text("scalar"), 0),
        Err(CheckedMapArrayReadError::NonArray)
    ));
    assert!(matches!(
        map.read_array_map(&MapKeyDomain::from_text("functions"), 1),
        Err(CheckedMapArrayReadError::Bounds)
    ));
    let view = map
        .read_array_map(&MapKeyDomain::from_text("functions"), 0)
        .unwrap();
    assert!(matches!(
        view.read_text(&MapKeyDomain::from_text("missing")),
        Ok(CheckedMapTextRead::Missing)
    ));
    assert!(matches!(
        view.read_text(&MapKeyDomain::from_text("name")),
        Ok(CheckedMapTextRead::NonText)
    ));
    drop(view);
    assert_eq!(map.end().unwrap(), MapEndReport::default());
    child.require_disposable().unwrap();
}
