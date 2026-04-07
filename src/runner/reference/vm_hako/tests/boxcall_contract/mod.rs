use std::sync::{Mutex, OnceLock};

fn env_guard() -> &'static Mutex<()> {
    static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
    GUARD.get_or_init(|| Mutex::new(()))
}

fn with_joinir_strict_without_planner_required<F: FnOnce()>(f: F) {
    let _lock = env_guard().lock().unwrap_or_else(|e| e.into_inner());
    let prev_strict = std::env::var("HAKO_JOINIR_STRICT").ok();
    let prev_planner_required = std::env::var("HAKO_JOINIR_PLANNER_REQUIRED").ok();
    let prev_debug = std::env::var("NYASH_JOINIR_DEV").ok();

    std::env::set_var("HAKO_JOINIR_STRICT", "1");
    std::env::remove_var("HAKO_JOINIR_PLANNER_REQUIRED");
    std::env::remove_var("NYASH_JOINIR_DEV");

    f();

    match prev_strict {
        Some(v) => std::env::set_var("HAKO_JOINIR_STRICT", v),
        None => std::env::remove_var("HAKO_JOINIR_STRICT"),
    }
    match prev_planner_required {
        Some(v) => std::env::set_var("HAKO_JOINIR_PLANNER_REQUIRED", v),
        None => std::env::remove_var("HAKO_JOINIR_PLANNER_REQUIRED"),
    }
    match prev_debug {
        Some(v) => std::env::set_var("NYASH_JOINIR_DEV", v),
        None => std::env::remove_var("NYASH_JOINIR_DEV"),
    }
}

mod compile;
mod subset;
