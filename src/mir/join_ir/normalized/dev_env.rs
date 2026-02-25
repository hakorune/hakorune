#![cfg(feature = "normalized_dev")]

use once_cell::sync::Lazy;
use std::cell::Cell;
use std::sync::{Mutex, MutexGuard};

/// RAII guard for normalized_dev env toggling (NYASH_JOINIR_NORMALIZED_DEV_RUN).
/// ネストを許可し、最初の呼び出し時の状態だけを保存・復元する。
pub struct NormalizedDevEnvGuard {
    active: bool,
}

#[derive(Default)]
struct EnvState {
    stack: Vec<Option<String>>,
}

static NORMALIZED_ENV_STATE: Lazy<Mutex<EnvState>> = Lazy::new(|| Mutex::new(EnvState::default()));
static NORMALIZED_TEST_LOCK: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

thread_local! {
    // Per-thread depth counter for test_ctx() to allow re-entrant dev env toggling
    // without self-deadlocking on NORMALIZED_TEST_LOCK.
    static IN_NORMALIZED_TEST_CTX: Cell<u32> = Cell::new(0);
}

fn enter_test_ctx() {
    IN_NORMALIZED_TEST_CTX.with(|c| c.set(c.get().saturating_add(1)));
}

fn exit_test_ctx() {
    IN_NORMALIZED_TEST_CTX.with(|c| {
        let v = c.get();
        if v > 0 {
            c.set(v - 1);
        }
    });
}

fn in_test_ctx() -> bool {
    IN_NORMALIZED_TEST_CTX.with(|c| c.get() > 0)
}

impl NormalizedDevEnvGuard {
    pub fn new(enabled: bool) -> Self {
        let mut state = NORMALIZED_ENV_STATE
            .lock()
            .expect("normalized env mutex poisoned");

        // Save current value before overriding.
        let prev = crate::config::env::joinir_dev::joinir_normalized_dev_run_raw();
        state.stack.push(prev);

        if enabled {
            std::env::set_var("NYASH_JOINIR_NORMALIZED_DEV_RUN", "1");
        } else {
            std::env::remove_var("NYASH_JOINIR_NORMALIZED_DEV_RUN");
        }

        Self { active: true }
    }
}

impl Drop for NormalizedDevEnvGuard {
    fn drop(&mut self) {
        if !self.active {
            return;
        }
        let mut state = NORMALIZED_ENV_STATE
            .lock()
            .expect("normalized env mutex poisoned");
        if let Some(prev) = state.stack.pop() {
            if let Some(prev) = prev {
                std::env::set_var("NYASH_JOINIR_NORMALIZED_DEV_RUN", prev);
            } else {
                std::env::remove_var("NYASH_JOINIR_NORMALIZED_DEV_RUN");
            }
        }
    }
}

/// normalized_dev feature + env の ON/OFF をまとめた判定。
pub fn normalized_dev_enabled() -> bool {
    crate::config::env::normalized_dev_enabled()
}

/// normalized_dev かつ test/debug ログが有効なときだけ true。
pub fn normalized_dev_logs_enabled() -> bool {
    crate::config::env::normalized_dev_enabled() && crate::config::env::joinir_test_debug_enabled()
}

/// テスト用コンテキスト：env を ON にしつつロックで並列汚染を防ぐ。
pub struct NormalizedTestContext<'a> {
    _lock: MutexGuard<'a, ()>,
    _env_guard: NormalizedDevEnvGuard,
}

impl<'a> NormalizedTestContext<'a> {
    fn new(lock: MutexGuard<'a, ()>) -> Self {
        enter_test_ctx();
        let env_guard = NormalizedDevEnvGuard::new(true);
        NormalizedTestContext {
            _lock: lock,
            _env_guard: env_guard,
        }
    }
}

impl Drop for NormalizedTestContext<'_> {
    fn drop(&mut self) {
        exit_test_ctx();
    }
}

/// テストで使う共通ガード。
pub fn test_ctx() -> NormalizedTestContext<'static> {
    let lock = NORMALIZED_TEST_LOCK
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    NormalizedTestContext::new(lock)
}

/// 簡易ラッパー：クロージャを normalized_dev ON で実行。
pub fn with_dev_env<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    if in_test_ctx() {
        let _env_guard = NormalizedDevEnvGuard::new(true);
        f()
    } else {
        let _ctx = test_ctx();
        f()
    }
}

/// env が既に ON のときはそのまま、OFF のときだけ with_dev_env を噛ませる。
pub fn with_dev_env_if_unset<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    if normalized_dev_enabled() {
        f()
    } else if in_test_ctx() {
        let _env_guard = NormalizedDevEnvGuard::new(true);
        f()
    } else {
        with_dev_env(f)
    }
}
