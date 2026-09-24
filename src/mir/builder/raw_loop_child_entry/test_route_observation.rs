use std::cell::Cell;

thread_local! {
    static ROUTES: Cell<(usize, usize)> = const { Cell::new((0, 0)) };
    static FAIL_AFTER_VARIABLE_ACCUM_CONSUME: Cell<bool> = const { Cell::new(false) };
}

pub(in crate::mir::builder) fn reset() {
    ROUTES.with(|routes| routes.set((0, 0)));
    FAIL_AFTER_VARIABLE_ACCUM_CONSUME.with(|fail| fail.set(false));
}

pub(in crate::mir::builder) fn counts() -> (usize, usize) {
    ROUTES.with(Cell::get)
}

pub(in crate::mir::builder) fn record_variable_accum() {
    ROUTES.with(|routes| {
        let (variable_accum, non_callable_route) = routes.get();
        routes.set((variable_accum + 1, non_callable_route));
    });
}

pub(in crate::mir::builder) fn record_non_callable_route() {
    ROUTES.with(|routes| {
        let (variable_accum, non_callable_route) = routes.get();
        routes.set((variable_accum, non_callable_route + 1));
    });
}

pub(in crate::mir::builder) fn inject_failure_after_variable_accum_consume_once() {
    FAIL_AFTER_VARIABLE_ACCUM_CONSUME.with(|fail| fail.set(true));
}

pub(in crate::mir::builder) fn take_failure_after_variable_accum_consume() -> bool {
    FAIL_AFTER_VARIABLE_ACCUM_CONSUME.with(|fail| fail.replace(false))
}
