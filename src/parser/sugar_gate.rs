use std::cell::Cell;

thread_local! {
    static SUGAR_ON: Cell<bool> = Cell::new(false);
}

pub fn is_enabled_env() -> bool {
    if crate::parser::env::force_sugar_enabled() {
        return true;
    }
    match crate::parser::env::syntax_sugar_level_raw() {
        Some(v) => {
            let v = v.to_ascii_lowercase();
            // Accept legacy toggles and new explicit off
            v == "basic" || v == "full" || v == "on" || v == "1" || v == "true"
        }
        None => true, // default ON
    }
}

pub fn is_enabled() -> bool {
    SUGAR_ON.with(|c| c.get()) || is_enabled_env()
}

pub fn with_enabled<T>(f: impl FnOnce() -> T) -> T {
    SUGAR_ON.with(|c| {
        let prev = c.get();
        c.set(true);
        let r = f();
        c.set(prev);
        r
    })
}
