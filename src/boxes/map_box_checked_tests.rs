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
