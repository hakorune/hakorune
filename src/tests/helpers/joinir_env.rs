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
#[allow(dead_code)]
pub fn set_core_on() {
    std::env::set_var("NYASH_JOINIR_CORE", "1");
}

/// IfSelect/Dev 系のフラグをすべてクリアする。
pub fn clear_joinir_flags() {
    std::env::remove_var("NYASH_JOINIR_CORE");
    std::env::remove_var("HAKO_JOINIR_IF_SELECT");
    std::env::remove_var("HAKO_JOINIR_IF_SELECT_DRYRUN");
    std::env::remove_var("NYASH_JOINIR_EXPERIMENT");
}

/// Phase 72-A: NYASH_JOINIR_EXPERIMENT SSOT ヘルパー
/// 実験モードが有効かどうかを判定する
pub fn is_experiment_enabled() -> bool {
    crate::config::env::joinir_experiment_enabled()
}

/// Phase 72-B: HAKO_JOINIR_IF_SELECT SSOT ヘルパー
/// IfSelect/JoinIR If分岐選択モードをONにする
pub fn set_if_select_on() {
    std::env::set_var("HAKO_JOINIR_IF_SELECT", "1");
}

/// IfSelect/JoinIR If分岐選択モードをOFFにする
pub fn set_if_select_off() {
    std::env::remove_var("HAKO_JOINIR_IF_SELECT");
}
