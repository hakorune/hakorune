use super::*;
use crate::exports::fault::{checked_object, FaultFrame};
use nyash_rust::runtime::dynamic_invocation::DynamicCarrierTokenV1;
use std::{cell::Cell, ptr::null};

const TYPE: i64 = 51;
const METHOD: &[u8] = b"run";
const SITE: u64 = 123;
const PARAMETERS: [u32; 1] = [FORMAL_I64];
const OUT: u64 = 0x1234;

thread_local! {
    static ACTIVE: Cell<*const DispatchProgramV1> = const { Cell::new(null()) };
    static ENTERED: Cell<u32> = const { Cell::new(0) };
    static PINNED_END_STATUS: Cell<u32> = const { Cell::new(u32::MAX) };
}

fn storage(frame: &mut FaultFrame) -> *mut c_void {
    (frame as *mut FaultFrame).cast()
}

fn receiver(handle: i64, tag: u32) -> dynamic::Operand {
    dynamic::Operand {
        kind: dynamic::TAGGED,
        tag,
        payload: handle as u64,
    }
}

fn argument(value: i64) -> dynamic::Operand {
    dynamic::Operand {
        kind: dynamic::INTEGER,
        tag: 0,
        payload: value as u64,
    }
}

unsafe fn run(
    frame: *mut c_void,
    program: *const DispatchProgramV1,
    receiver: &dynamic::Operand,
    arg: &dynamic::Operand,
    out: &mut u64,
) -> u32 {
    unsafe {
        invoke(
            frame,
            SITE,
            program,
            receiver,
            METHOD.as_ptr(),
            METHOD.len(),
            arg,
            1,
            out,
        )
    }
}

unsafe extern "C" fn method(
    frame: *mut c_void,
    handle: i64,
    args: *const dynamic::Operand,
    _count: usize,
    out: *mut i64,
) -> u32 {
    ENTERED.with(|count| count.set(count.get() + 1));
    let depth = unsafe { (*args).payload as i64 };
    if depth < 0 {
        return record(frame, SITE + 1, Failure::ArgumentTypeMismatch);
    }
    if depth == 0 {
        unsafe { out.write(7) };
        return Status::Normal as u32;
    }

    let mut result = OUT;
    let program = ACTIVE.with(Cell::get);
    let status = unsafe {
        run(
            frame,
            program,
            &receiver(handle, TYPED_STORE_BORROW),
            &argument(depth - 1),
            &mut result,
        )
    };
    if status != Status::Normal as u32 {
        return status;
    }
    let Ok(token) = DynamicCarrierTokenV1::from_raw_v1(result) else {
        return Status::InvalidContract as u32;
    };
    let value = token.integer_payload_v1();
    if token.end_v1().is_err() {
        return Status::InvalidContract as u32;
    }
    // Use a separate frame so this intentionally rejected End does not
    // inject a diagnostic into the successful invocation.
    let mut end_frame = FaultFrame::new();
    let status =
        unsafe { checked_object::home_release(storage(&mut end_frame), 1, SITE, handle, TYPE) };
    PINNED_END_STATUS.with(|slot| slot.set(status));
    let Ok(value) = value else {
        return Status::InvalidContract as u32;
    };
    unsafe { out.write(value + 1) };
    Status::Normal as u32
}

fn descriptor(types: &[DispatchTypeV1], rows: &[DispatchProviderV1]) -> DispatchProgramV1 {
    DispatchProgramV1 {
        version: 1,
        reserved: 0,
        types: types.as_ptr(),
        type_count: types.len(),
        rows: rows.as_ptr(),
        row_count: rows.len(),
    }
}

fn provider() -> DispatchProviderV1 {
    DispatchProviderV1 {
        type_id: TYPE,
        selector: METHOD.as_ptr(),
        selector_len: METHOD.len(),
        arity: 1,
        parameters: PARAMETERS.as_ptr(),
        thunk: Some(method),
    }
}

fn allocate(program: &DispatchProgramV1) -> i64 {
    let mut frame = FaultFrame::new();
    let mut handle = 0;
    assert_eq!(
        unsafe {
            checked_object::allocate_in_program(
                storage(&mut frame),
                1,
                SITE,
                program,
                TYPE,
                null(),
                0,
                &mut handle,
            )
        },
        Status::Normal as u32
    );
    assert!(handle < 0);
    handle
}

fn release(handle: i64) {
    let mut frame = FaultFrame::new();
    assert_eq!(
        unsafe { checked_object::home_release(storage(&mut frame), 1, SITE, handle, TYPE) },
        Status::Normal as u32
    );
}

#[test]
fn program_dispatch_reentrant_pin_closes_on_normal_and_fault() {
    let types = [DispatchTypeV1 {
        type_id: TYPE,
        layout: null(),
        layout_count: 0,
    }];
    let rows = [provider()];
    let program = descriptor(&types, &rows);
    let handle = allocate(&program);
    let receiver = receiver(handle, TYPED_STORE_BORROW);
    ACTIVE.with(|slot| slot.set(&program));
    ENTERED.with(|count| count.set(0));
    let mut frame = FaultFrame::new();
    let mut out = OUT;
    assert_eq!(
        unsafe {
            run(
                storage(&mut frame),
                &program,
                &receiver,
                &argument(2),
                &mut out,
            )
        },
        0
    );
    assert_eq!(ENTERED.with(Cell::get), 3);
    assert_eq!(PINNED_END_STATUS.with(Cell::get), Status::Fault as u32);
    let token = DynamicCarrierTokenV1::from_raw_v1(out).unwrap();
    assert_eq!(token.integer_payload_v1().unwrap(), 9);
    token.end_v1().unwrap();
    assert!(frame.diagnostics().unwrap().0.is_none());

    out = OUT;
    assert_eq!(
        unsafe {
            run(
                storage(&mut frame),
                &program,
                &receiver,
                &argument(-1),
                &mut out,
            )
        },
        1
    );
    assert_eq!(out, OUT);
    let (primary, suppressed, _) = frame.diagnostics().unwrap();
    let primary = primary.unwrap();
    assert_eq!(
        (primary.reason, primary.site, primary.details),
        (400, SITE + 1, [4, 0])
    );
    assert!(suppressed.is_empty());
    release(handle);
    ACTIVE.with(|slot| slot.set(null()));
}

#[test]
fn program_dispatch_never_enters_body_for_invalid_identity_or_selection() {
    let types = [DispatchTypeV1 {
        type_id: TYPE,
        layout: null(),
        layout_count: 0,
    }];
    let rows = [provider()];
    let duplicate_rows = [provider(), provider()];
    let own = descriptor(&types, &rows);
    let foreign = descriptor(&types, &rows);
    let duplicate = descriptor(&types, &duplicate_rows);
    let missing = descriptor(&types, &[]);
    let live = allocate(&own);
    let double = allocate(&duplicate);
    let absent = allocate(&missing);
    let reclaimed = allocate(&own);
    release(reclaimed);
    ENTERED.with(|count| count.set(0));
    let truth = dynamic::Operand {
        kind: dynamic::TAGGED,
        tag: 2,
        payload: 1,
    };
    for (program, handle, tag, arg, detail) in [
        (&foreign, live, TYPED_STORE_BORROW, argument(0), 1),
        (&own, reclaimed, TYPED_STORE_BORROW, argument(0), 1),
        (&duplicate, double, TYPED_STORE_BORROW, argument(0), 3),
        (&missing, absent, TYPED_STORE_BORROW, argument(0), 2),
        (&own, live, TYPED_STORE_BORROW, truth, 4),
    ] {
        let mut frame = FaultFrame::new();
        let mut out = OUT;
        assert_eq!(
            unsafe {
                run(
                    storage(&mut frame),
                    program,
                    &receiver(handle, tag),
                    &arg,
                    &mut out,
                )
            },
            1
        );
        assert_eq!(out, OUT);
        let (primary, suppressed, _) = frame.diagnostics().unwrap();
        let primary = primary.unwrap();
        assert_eq!(
            (primary.reason, primary.site, primary.details),
            (400, SITE, [detail, 0])
        );
        assert!(suppressed.is_empty());
    }
    let mut frame = FaultFrame::new();
    let mut out = OUT;
    assert_ne!(
        unsafe {
            run(
                storage(&mut frame),
                &own,
                &receiver(live, 3),
                &argument(0),
                &mut out,
            )
        },
        0
    );
    assert_eq!(out, OUT);
    assert_eq!(
        unsafe {
            run(
                storage(&mut frame),
                &own,
                &receiver(1, TYPED_STORE_BORROW),
                &argument(0),
                &mut out,
            )
        },
        1
    );
    assert_eq!(out, OUT);
    assert_eq!(ENTERED.with(Cell::get), 0);
    for handle in [live, double, absent] {
        release(handle);
    }
}

#[test]
fn typed_store_borrow_is_an_operator_type_error_at_the_comparison_site() {
    let types = [DispatchTypeV1 {
        type_id: TYPE,
        layout: null(),
        layout_count: 0,
    }];
    let program = descriptor(&types, &[]);
    let handle = allocate(&program);
    let mut frame = FaultFrame::new();
    let mut out = 42;
    assert_eq!(
        unsafe {
            super::super::checked_string::borrowed_integer_payload(
                storage(&mut frame),
                SITE,
                TYPED_STORE_BORROW,
                handle as u64,
                &mut out,
            )
        },
        Status::Fault as u32,
    );
    assert_eq!(out, 42);
    let (primary, suppressed, _) = frame.diagnostics().unwrap();
    let primary = primary.unwrap();
    assert_eq!(
        (primary.reason, primary.site, primary.details),
        (302, SITE, [0, 0])
    );
    assert!(suppressed.is_empty());
    release(handle);
}
