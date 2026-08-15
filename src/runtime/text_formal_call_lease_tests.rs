use super::*;
use crate::runtime::host_handles;
use crate::runtime::text_formal_abi::issue_text_formal_borrow_v1;

#[test]
fn facade_acquires_and_consumes_one_opaque_set() {
    let _guard = host_handles::test_host_handle_policy_lock()
        .lock()
        .expect("host handle lock");
    let first_handle = host_handles::to_handle_text("first");
    let second_handle = host_handles::to_handle_text("second");
    let formals = [
        issue_text_formal_borrow_v1(first_handle).expect("first formal"),
        issue_text_formal_borrow_v1(second_handle).expect("second formal"),
    ];

    let token = acquire_text_formal_call_leases_v1(&formals).expect("lease set");
    token.finish().expect("finish set");

    host_handles::drop_handle(first_handle);
    host_handles::drop_handle(second_handle);
}

#[test]
fn facade_rejects_empty_invocation_set() {
    let formals: [TextFormalBorrowV1; 0] = [];
    assert!(matches!(
        acquire_text_formal_call_leases_v1(&formals),
        Err(TextFormalLeaseAcquireRejectV1::EmptyLeaseSet)
    ));
}
