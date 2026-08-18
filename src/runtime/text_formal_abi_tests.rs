use super::{
    issue_stable_text_formal_wire_v1, issue_text_formal_borrow_v1, TextFormalBorrowStatusV1,
};
use crate::box_trait::{IntegerBox, NyashBox, StringBox};
use crate::runtime::host_handles;
use std::sync::Arc;

fn with_policy<F: FnOnce()>(policy: &str, f: F) {
    let _guard = host_handles::test_host_handle_policy_lock()
        .lock()
        .expect("host handle policy lock");
    let previous = std::env::var("NYASH_HOST_HANDLE_ALLOC_POLICY").ok();
    std::env::set_var("NYASH_HOST_HANDLE_ALLOC_POLICY", policy);
    f();
    if let Some(value) = previous {
        std::env::set_var("NYASH_HOST_HANDLE_ALLOC_POLICY", value);
    } else {
        std::env::remove_var("NYASH_HOST_HANDLE_ALLOC_POLICY");
    }
}

#[test]
fn live_stringbox_lends_exact_text_and_consumes_once() {
    with_policy("lifo", || {
        let handle = host_handles::to_handle_text("hello");
        let borrow = issue_text_formal_borrow_v1(handle).expect("formal Text");
        assert_eq!(borrow.with_text(str::to_owned).expect("borrow"), "hello");
        host_handles::drop_handle(handle);
    });
}

#[test]
fn stable_stringbox_is_admitted_but_integer_is_not() {
    with_policy("lifo", || {
        let handle = host_handles::to_handle_arc(Arc::new(StringBox::new("boxed")));
        let borrow = issue_text_formal_borrow_v1(handle).expect("StringBox formal Text");
        assert_eq!(borrow.with_text(str::to_owned).expect("borrow"), "boxed");
        host_handles::drop_handle(handle);

        let integer =
            host_handles::to_handle_arc(Arc::new(IntegerBox::new(7)) as Arc<dyn NyashBox>);
        assert!(matches!(
            issue_text_formal_borrow_v1(integer),
            Err(TextFormalBorrowStatusV1::NonTextPayload)
        ));
        host_handles::drop_handle(integer);
    });
}

#[test]
fn zero_and_unknown_slots_fail_closed() {
    assert!(matches!(
        issue_text_formal_borrow_v1(0),
        Err(TextFormalBorrowStatusV1::ZeroOrOutOfRangeSlot)
    ));
    assert!(matches!(
        issue_text_formal_borrow_v1(u64::MAX),
        Err(TextFormalBorrowStatusV1::ZeroOrOutOfRangeSlot)
    ));
}

#[test]
fn dropped_and_reused_slot_rejects_old_generation() {
    with_policy("lifo", || {
        let first = host_handles::to_handle_text("first");
        let old = issue_text_formal_borrow_v1(first).expect("old formal");
        host_handles::drop_handle(first);
        let replacement = host_handles::to_handle_text("replacement");
        assert_eq!(
            old.validate(),
            Err(TextFormalBorrowStatusV1::GenerationMismatch)
        );
        host_handles::drop_handle(replacement);
    });
}

#[test]
fn published_wire_requires_nonzero_generation() {
    assert_eq!(std::mem::size_of::<super::TextFormalBorrowV1>(), 16);
    assert_eq!(std::mem::align_of::<super::TextFormalBorrowV1>(), 8);
    assert_eq!(
        super::validate_text_formal_wire_v1(1, 0),
        TextFormalBorrowStatusV1::ZeroOrOutOfRangeSlot
    );
}

#[test]
fn stable_wire_issuer_accepts_published_stable_text_pair() {
    with_policy("lifo", || {
        let handle = host_handles::to_handle_text("stable-wire");
        let (slot, generation) =
            host_handles::capture_text_formal_pair(handle).expect("published pair");
        let pair = issue_stable_text_formal_wire_v1(slot, generation).expect("stable wire");
        assert_eq!(pair.slot(), slot);
        assert_eq!(pair.generation(), generation);
        host_handles::drop_handle(handle);
    });
}

#[test]
fn stable_wire_issuer_rejects_stringbox_and_stale_pair() {
    with_policy("lifo", || {
        let boxed = host_handles::to_handle_arc(Arc::new(StringBox::new("boxed")));
        let (boxed_slot, boxed_generation) =
            host_handles::capture_text_formal_pair(boxed).expect("exact text pair");
        assert_eq!(
            issue_stable_text_formal_wire_v1(boxed_slot, boxed_generation),
            Err(TextFormalBorrowStatusV1::NonTextPayload)
        );
        host_handles::drop_handle(boxed);

        let old = host_handles::to_handle_text("old");
        let (slot, generation) = host_handles::capture_text_formal_pair(old).expect("old pair");
        host_handles::drop_handle(old);
        let replacement = host_handles::to_handle_text("replacement");
        assert_eq!(
            issue_stable_text_formal_wire_v1(slot, generation),
            Err(TextFormalBorrowStatusV1::GenerationMismatch)
        );
        host_handles::drop_handle(replacement);
    });
}
