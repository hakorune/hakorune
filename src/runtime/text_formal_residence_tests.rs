use super::*;
use crate::box_trait::StringBox;
use crate::runtime::host_handles;
use std::sync::Arc;

fn test_lock() -> std::sync::MutexGuard<'static, ()> {
    host_handles::test_host_handle_policy_lock()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn published_pair(handle: u64) -> TextFormalWirePairV1 {
    let (slot, generation) =
        host_handles::capture_text_formal_pair(handle).expect("stable Text pair");
    TextFormalWirePairV1::from_published_wire(slot, generation)
}

#[test]
fn residence_projects_occurrence_ordered_roots_and_finishes() {
    let _guard = test_lock();
    let subject = host_handles::to_handle_text("subject");
    let needle = host_handles::to_handle_text("needle");
    let pairs = [published_pair(subject), published_pair(needle)];

    let residence = acquire_text_formal_residence_v1(&pairs).expect("residence");
    assert_eq!(residence.frame_revision(), RESIDENCE_FRAME_REVISION_V1);
    assert_eq!(residence.root_count(), 2);
    assert_eq!(residence.frame_size(), 64);
    let first_root = residence
        .with_root(0, |root| (root.as_ptr(), root.byte_len()))
        .expect("first root");
    assert!(!first_root.0.is_null());
    assert_eq!(first_root.1, 7);
    assert_eq!(residence.with_root(1, |root| root.byte_len()), Some(6));
    residence.finish().expect("finish residence");

    host_handles::drop_handle(subject);
    host_handles::drop_handle(needle);
}

#[test]
fn residence_keeps_same_pair_as_two_root_occurrences() {
    let _guard = test_lock();
    let handle = host_handles::to_handle_text("alias");
    let pair = published_pair(handle);
    let residence = acquire_text_formal_residence_v1(&[pair, pair]).expect("residence");

    assert_eq!(residence.root_count(), 2);
    assert_eq!(residence.with_root(0, |root| root.byte_len()), Some(5));
    assert_eq!(residence.with_root(1, |root| root.byte_len()), Some(5));
    residence.finish().expect("finish residence");
    host_handles::drop_handle(handle);
}

#[test]
fn residence_rejects_stale_pair_without_pinning() {
    let _guard = test_lock();
    let handle = host_handles::to_handle_text("stale");
    let (slot, generation) =
        host_handles::capture_text_formal_pair(handle).expect("stable Text pair");
    host_handles::drop_handle(handle);
    let stale = TextFormalWirePairV1::from_published_wire(slot, generation);

    assert!(matches!(
        acquire_text_formal_residence_v1(&[stale]),
        Err(TextFormalResidenceAcquireRejectV1::Lease(
            TextFormalLeaseAcquireRejectV1::MissingSlot { formal_index: 0 }
                | TextFormalLeaseAcquireRejectV1::GenerationMismatch { formal_index: 0 }
        ))
    ));
}

#[test]
fn residence_is_stable_text_only() {
    let _guard = test_lock();
    let handle = host_handles::to_handle_arc(Arc::new(StringBox::new("box")));
    let pair = published_pair(handle);

    assert!(matches!(
        acquire_text_formal_residence_v1(&[pair]),
        Err(TextFormalResidenceAcquireRejectV1::Lease(
            TextFormalLeaseAcquireRejectV1::NonTextPayload { formal_index: 0 }
        ))
    ));
    host_handles::drop_handle(handle);
}
