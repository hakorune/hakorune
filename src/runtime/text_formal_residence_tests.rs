use super::*;
use crate::box_trait::{BoolBox, BoxBase, BoxCore, NyashBox, StringBox};
use crate::runtime::host_handles;
use crate::runtime::text_formal_abi::issue_text_formal_borrow_v1;
use std::any::Any;
use std::fmt;
use std::sync::Arc;

#[derive(Debug)]
struct SpoofedStringBox {
    value: String,
    base: BoxBase,
}

impl SpoofedStringBox {
    fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            base: BoxBase::new(),
        }
    }
}

impl BoxCore for SpoofedStringBox {
    fn box_id(&self) -> u64 {
        self.base.id
    }

    fn parent_type_id(&self) -> Option<std::any::TypeId> {
        self.base.parent_type_id
    }

    fn fmt_box(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.value)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl NyashBox for SpoofedStringBox {
    fn to_string_box(&self) -> StringBox {
        StringBox::new(self.value.clone())
    }

    fn equals(&self, other: &dyn NyashBox) -> BoolBox {
        BoolBox::new(
            other
                .as_any()
                .downcast_ref::<Self>()
                .is_some_and(|other| other.value == self.value),
        )
    }

    fn type_name(&self) -> &'static str {
        "StringBox"
    }

    fn clone_box(&self) -> Box<dyn NyashBox> {
        Box::new(Self::new(self.value.clone()))
    }

    fn share_box(&self) -> Box<dyn NyashBox> {
        self.clone_box()
    }

    fn as_str_fast(&self) -> Option<&str> {
        Some(self.value.as_str())
    }
}

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
fn residence_abi_layout_is_explicit_and_not_host_inferred() {
    let layout = residence_abi_layout_v1();
    assert_eq!(layout.revision(), "text-formal-residence-v1");
    assert_eq!(layout.frame_revision(), RESIDENCE_FRAME_REVISION_V1);
    assert_eq!(layout.header_size(), RESIDENCE_FRAME_HEADER_SIZE_V1);
    assert_eq!(layout.root_row_size(), RESIDENCE_ROOT_ROW_SIZE_V1);
    assert_eq!(layout.header_alignment(), 8);
    assert_eq!(layout.root_row_alignment(), 8);
    assert_eq!(layout.frame_size_for_roots(2), Some(64));
    assert_eq!(
        layout.frame_size_for_roots(layout.max_root_count()),
        Some(16_416)
    );
    assert_eq!(
        layout.frame_size_for_roots(layout.max_root_count() + 1),
        None
    );
    assert!(layout.frame_size_for_roots(u32::MAX).is_none());
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
    assert_eq!(residence.with_root(0, |root| root.byte_len()), Some(7));
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
fn residence_admits_concrete_stringbox_without_snapshot() {
    let _guard = test_lock();
    let handle = host_handles::to_handle_arc(Arc::new(StringBox::new("box")));
    let pair = published_pair(handle);

    let residence = acquire_text_formal_residence_v1(&[pair]).expect("concrete StringBox root");
    assert_eq!(residence.root_count(), 1);
    assert_eq!(residence.with_root(0, |root| root.byte_len()), Some(3));
    host_handles::drop_handle(handle);
    assert_eq!(residence.with_root(0, |root| root.byte_len()), Some(3));
    residence.finish().expect("finish concrete StringBox root");
}

#[test]
fn residence_rejects_stringbox_name_spoof_before_pinning() {
    let _guard = test_lock();
    let handle = host_handles::to_handle_arc(Arc::new(SpoofedStringBox::new("spoof")));
    let identity =
        host_handles::capture_text_lease_identity(handle).expect("fast text spoof identity");
    let pair = TextFormalWirePairV1::from_published_wire(identity.handle(), identity.generation());

    assert!(matches!(
        acquire_text_formal_residence_v1(&[pair]),
        Err(TextFormalResidenceAcquireRejectV1::Lease(
            TextFormalLeaseAcquireRejectV1::NonTextPayload { formal_index: 0 }
        ))
    ));
    host_handles::drop_handle(handle);
}

#[test]
fn c_frame_entry_projects_rows_and_finishes_once() {
    let _guard = test_lock();
    let subject = host_handles::to_handle_text("subject");
    let needle = host_handles::to_handle_text("needle");
    let pairs = [
        issue_text_formal_borrow_v1(subject).expect("published subject pair"),
        issue_text_formal_borrow_v1(needle).expect("published needle pair"),
    ];
    let mut storage = [0_u64; 16];
    let frame = storage
        .as_mut_ptr()
        .cast::<TextFormalResidenceFrameHeaderV1>();

    let status = unsafe {
        enter_text_formal_residence_c_v1(
            pairs.as_ptr(),
            pairs.len() as u32,
            frame,
            storage.len() as u32 * std::mem::size_of::<u64>() as u32,
        )
    };
    assert_eq!(status, TextFormalResidenceCStatusV1::Valid.as_u32());
    unsafe {
        assert_eq!((*frame).root_count, 2);
        assert_ne!((*frame).lease_token, 0);
        let rows = frame
            .cast::<u8>()
            .add(std::mem::size_of::<TextFormalResidenceFrameHeaderV1>())
            .cast::<TextFormalResidenceRootRowV1>();
        assert_eq!((*rows.add(0)).byte_len, 7);
        assert_eq!((*rows.add(1)).byte_len, 6);
    }

    let finish = unsafe { finish_text_formal_residence_c_v1(frame) };
    assert_eq!(finish, TextFormalResidenceCStatusV1::Valid.as_u32());
    unsafe { assert_eq!((*frame).lease_token, 0) };
    assert_eq!(
        unsafe { finish_text_formal_residence_c_v1(frame) },
        TextFormalResidenceCStatusV1::InvalidFrame.as_u32()
    );

    host_handles::drop_handle(subject);
    host_handles::drop_handle(needle);
}

#[test]
fn c_frame_entry_rejects_small_frame_before_pinning() {
    let _guard = test_lock();
    let handle = host_handles::to_handle_text("small");
    let pair = issue_text_formal_borrow_v1(handle).expect("published pair");
    let mut storage = [0_u64; 8];
    let frame = storage
        .as_mut_ptr()
        .cast::<TextFormalResidenceFrameHeaderV1>();

    let status = unsafe {
        enter_text_formal_residence_c_v1(
            &pair,
            1,
            frame,
            std::mem::size_of::<TextFormalResidenceFrameHeaderV1>() as u32,
        )
    };
    assert_eq!(status, TextFormalResidenceCStatusV1::FrameTooSmall.as_u32());

    host_handles::drop_handle(handle);
}

#[test]
fn c_frame_entry_rejects_root_limit_before_pinning() {
    let _guard = test_lock();
    let handle = host_handles::to_handle_text("root-limit");
    let pair = issue_text_formal_borrow_v1(handle).expect("published pair");
    let mut storage = [0_u64; 64];
    let frame = storage
        .as_mut_ptr()
        .cast::<TextFormalResidenceFrameHeaderV1>();
    let layout = residence_abi_layout_v1();

    let status = unsafe {
        enter_text_formal_residence_c_v1(
            &pair,
            layout.max_root_count() + 1,
            frame,
            storage.len() as u32 * std::mem::size_of::<u64>() as u32,
        )
    };
    assert_eq!(
        status,
        TextFormalResidenceCStatusV1::FrameSizeOverflow.as_u32()
    );
    host_handles::drop_handle(handle);
}

#[test]
fn c_frame_entry_rejects_pair_frame_overlap_without_mutation() {
    let _guard = test_lock();
    let handle = host_handles::to_handle_text("overlap");
    let pair = issue_text_formal_borrow_v1(handle).expect("published pair");
    let pair_ptr = &pair as *const TextFormalBorrowV1;
    let status = unsafe {
        enter_text_formal_residence_c_v1(
            pair_ptr,
            1,
            pair_ptr as *mut TextFormalResidenceFrameHeaderV1,
            64,
        )
    };
    assert_eq!(
        status,
        TextFormalResidenceCStatusV1::PairFrameOverlap.as_u32()
    );

    host_handles::drop_handle(handle);
}
