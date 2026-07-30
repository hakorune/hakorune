//! JoinIR テスト用の軽量 ENV ヘルパー
//!
//! Core/Dev のフラグを明示的にセット／クリアすることで、テスト間の競合を避ける。
//!
//! Note: JoinIR Core は常時 ON。`NYASH_JOINIR_CORE` は deprecated なので、セットは互換目的だけ。

#[cfg(test)]
use std::sync::Mutex;

#[cfg(test)]
static JOINIR_ENV_LOCK: Mutex<()> = Mutex::new(());

#[cfg(test)]
pub fn with_joinir_env_lock<F: FnOnce()>(f: F) {
    let _guard = JOINIR_ENV_LOCK.lock().expect("joinir env lock poisoned");
    f();
}

/// Core ON (joinir_core_enabled = true) にする。
#[allow(dead_code)] // ASTCLEAN-008: compat helper for tests that still toggle deprecated core env explicitly.
pub fn set_core_on() {
    std::env::set_var("NYASH_JOINIR_CORE", "1");
}

/// JoinIR test flagsをすべてクリアする。
pub fn clear_joinir_flags() {
    std::env::remove_var("NYASH_JOINIR_CORE");
    std::env::remove_var("NYASH_JOINIR_EXPERIMENT");
}

/// Phase 72-A: NYASH_JOINIR_EXPERIMENT SSOT ヘルパー
/// 実験モードが有効かどうかを判定する
pub fn is_experiment_enabled() -> bool {
    crate::config::env::joinir_experiment_enabled()
}
