//! Global environment configuration aggregator (管理棟)
//!
//! Consolidates NYASH_* environment variables across subsystems and
//! optionally applies overrides from `nyash.toml`.
//!
//! # Global Environment Configuration (管理棟)
//!
//! ## 環境変数の集約と直読み禁止
//!
//! **Phase 286A/B** で、全システムの環境変数を `src/config/env/` 以下のモジュールに集約しました。
//!
//! - **直読み禁止**: `std::env::var()` / `std::env::set_var()` の直接呼び出しは禁止です。
//! - **必ず `src/config/env/*` 経由でアクセス**: 各サブシステムのフラグモジュール (`macro_flags`, `box_factory_flags`, etc.) を使用してください。
//!
//! ## モジュール構成
//!
//! | モジュール | 担当環境変数 | 用途 |
//! | --- | --- | --- |
//! | `macro_flags` | `NYASH_MACRO_*` | Macro システム設定 |
//! | `box_factory_flags` | `NYASH_BOX_FACTORY_*`, `NYASH_DISABLE_PLUGINS` | Box Factory / プラグイン設定 |
//! | `helper_boundary_flags` | `NYASH_HOST_HANDLE_*`, `NYASH_STRING_SPAN_CACHE_*` | helper 境界 policy 設定 |
//! | `joinir_flags` | `NYASH_JOINIR_*` | JoinIR 設定 |
//! | `mir_flags` | `NYASH_MIR_*` | MIR 設定 |
//! | `vm_backend_flags` | `NYASH_VM_*` | VM / Backend 設定 |
//! | `parser_flags` | `NYASH_PARSER_*` | Parser 設定 |
//! | `tokenizer_flags` | `NYASH_TOK_*` | Tokenizer 設定 |
//! | `llvm_provider_flags` | `NYASH_LLVM_*`, `HAKO_LLVM_*` | LLVM provider 設定 |
//! | `paths` | `NYASH_ROOT` | パス解決ヒント |
//! | `using_flags` | `NYASH_USING_*` | Using / Namespace 設定 |
//! | `verification_flags` | `NYASH_VERIFY_*` | Verification 設定 |
//! | `selfhost_flags` | `NYASH_NY_COMPILER_*` | Selfhost compiler 設定 |
//! | `string_flags` | `NYASH_STR_*` | String / Unicode 設定 |
//!
//! ## 新規環境変数追加の手順
//!
//! 1. **モジュール選択**: 上記のモジュールから適切なものを選択。
//! 2. **関数定義**: 選択したモジュールに `fn env_var_name() -> type` 形式で関数を定義。
//! 3. **再export**: `src/config/env.rs` で `pub use module::*;` を確認（既に集約済み）。
//! 4. **ドキュメント追記**: `docs/reference/environment-variables.md` に必ず追記してください。
//!
//! ## 直読み禁止のチェック
//!
//! 置換漏れを確認するには、以下のコマンドを使用してください：
//!
//! ```bash
//! # std::env::var() の直接呼び出しを検索（src/config/env/ 以外は禁止）
//! rg -n "std::env::(var|set_var|remove_var)\(" src | rg -v "src/config/env/"
//!
//! # NYASH_* 環境変数の直読みを検索（src/config/env/ 以外は禁止）
//! rg -n "NYASH_(MACRO|BOX_FACTORY|DISABLE_PLUGINS)" src | rg -v "src/config/env/"
//! ```
//!
//! ## 再export ポリシー
//!
//! 全ての環境変数関数は `src/config/env.rs` で再exportされています：
//!
//! ```rust
//! // Backward-compatible re-exports (NO BREAKING CHANGES!)
//! pub use box_factory_flags::*;
//! pub use joinir_flags::*;
//! pub use macro_flags::*;
//! pub use mir_flags::*;
//! pub use parser_flags::*;
//! pub use selfhost_flags::*;
//! pub use using_flags::*;
//! pub use verification_flags::*;
//! pub use vm_backend_flags::*;
//! ```
//!
//! 使用時は `crate::config::env::function_name()` でアクセスしてください。
//!
//! ## 参照
//!
//! - SSOT ドキュメント: `docs/reference/environment-variables.md`
//! - AGENTS.md 5.3: 環境変数スパロー防止ポリシー
//!
//! # Modular Organization
//!
//! Environment flags are now organized into focused Box modules:
//! - `joinir_flags` - JoinIR-related flags (30+ functions)
//! - `mir_flags` - MIR-related flags (20+ functions)
//! - `vm_backend_flags` - VM/Backend flags (15+ functions)
//! - `parser_flags` - Parser flags (10+ functions)
//! - `using_flags` - Using/namespace flags (10+ functions)
//! - `verification_flags` - Verification flags (8+ functions)
//! - `selfhost_flags` - Selfhost compiler flags (10+ functions)
//! - `string_flags` - String/Unicode flags (1+ functions)
//! - `helper_boundary_flags` - helper-boundary policy flags
//!
//! All functions are re-exported at the top level for backward compatibility.

mod catalog;
pub mod dump;
// Phase 124: hako_check module removed (JoinIR-only consolidation)
// pub mod hako_check;
pub mod joinir_dev;
pub mod stage1;

// New modular organization
mod box_factory_flags;
mod builder_flags;
mod helper_boundary_flags;
mod joinir_flags;
mod llvm_provider_flags;
mod macro_flags;
mod mir_flags;
mod parser_flags;
mod paths;
mod runner_flags;
mod selfhost_flags;
mod string_flags;
mod tokenizer_flags;
mod using_flags;
mod verification_flags;
mod vm_backend_flags;

pub use catalog::{env_vars, AppliesTo, EnvVarMeta};
pub use dump::*;
// Phase 124: hako_check exports removed (JoinIR-only consolidation)
// pub use hako_check::*;
pub use joinir_dev::*;
pub use stage1::*;

// Backward-compatible re-exports (NO BREAKING CHANGES!)
pub use box_factory_flags::*;
pub use builder_flags::*;
pub use helper_boundary_flags::*;
pub use joinir_flags::*;
pub use llvm_provider_flags::*;
pub use macro_flags::*;
pub use mir_flags::*;
pub use parser_flags::*;
pub use paths::*;
pub use runner_flags::*;
pub use selfhost_flags::*;
pub use string_flags::*;
pub use tokenizer_flags::*;
pub use using_flags::*;
pub use verification_flags::*;
pub use vm_backend_flags::*;

use std::collections::BTreeMap;

#[derive(Debug, Clone, Default)]
pub struct NyashEnv {
    // ARCHIVED: JIT-related configuration moved to archive/jit-cranelift/ during Phase 15
    // pub jit: crate::jit::config::JitConfig,
    /// Arbitrary key-value overrides loaded from nyash.toml [env]
    pub overrides: BTreeMap<String, String>,
}

impl NyashEnv {
    pub fn from_env() -> Self {
        Self {
            // ARCHIVED: JIT config during Phase 15
            // jit: crate::jit::config::JitConfig::from_env(),
            overrides: BTreeMap::new(),
        }
    }
    /// Apply current struct values into process environment
    pub fn apply_env(&self) {
        // ARCHIVED: JIT config during Phase 15
        // self.jit.apply_env();
        for (k, v) in &self.overrides {
            std::env::set_var(k, v);
        }
    }
}

// Global current env config (thread-safe)
use once_cell::sync::OnceCell;
use std::collections::HashSet;
use std::sync::Mutex;

static GLOBAL_ENV: OnceCell<std::sync::RwLock<NyashEnv>> = OnceCell::new();
static WARNED_ALIASES: OnceCell<Mutex<HashSet<String>>> = OnceCell::new();

pub fn current() -> NyashEnv {
    if let Some(lock) = GLOBAL_ENV.get() {
        if let Ok(cfg) = lock.read() {
            return cfg.clone();
        }
    }
    NyashEnv::from_env()
}

pub fn set_current(cfg: NyashEnv) {
    if let Some(lock) = GLOBAL_ENV.get() {
        if let Ok(mut w) = lock.write() {
            *w = cfg;
            return;
        }
    }
    let _ = GLOBAL_ENV.set(std::sync::RwLock::new(cfg));
}

/// Load overrides from nyash.toml `[env]` table and apply them to process env.
///
/// Example:
/// [env]
/// NYASH_JIT_THRESHOLD = "1"
/// NYASH_CLI_VERBOSE = "1"
pub fn bootstrap_from_toml_env() {
    // Allow disabling nyash.toml env bootstrapping for isolated smokes/CI
    if std::env::var("NYASH_SKIP_TOML_ENV").ok().as_deref() == Some("1") {
        return;
    }
    // Prefer hakorune.toml, fallback to nyash.toml
    let alt = if std::path::Path::new("hakorune.toml").exists() {
        "hakorune.toml"
    } else {
        "nyash.toml"
    };
    let path = alt;
    let content = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => return,
    };
    let Ok(value) = toml::from_str::<toml::Value>(&content) else {
        return;
    };
    let Some(env_tbl) = value.get("env").and_then(|v| v.as_table()) else {
        return;
    };
    let mut overrides: BTreeMap<String, String> = BTreeMap::new();
    for (k, v) in env_tbl {
        if let Some(s) = v.as_str() {
            std::env::set_var(k, s);
            overrides.insert(k.clone(), s.to_string());
        } else if let Some(b) = v.as_bool() {
            let sv = if b { "1" } else { "0" };
            std::env::set_var(k, sv);
            overrides.insert(k.clone(), sv.to_string());
        } else if let Some(n) = v.as_integer() {
            let sv = n.to_string();
            std::env::set_var(k, &sv);
            overrides.insert(k.clone(), sv);
        }
    }
    // Merge into global
    let mut cur = current();
    cur.overrides.extend(overrides);
    set_current(cur);
}

/// Generic boolean env parser: accepts 1/true/on (case-insensitive) as true.
pub fn env_bool(key: &str) -> bool {
    match std::env::var(key).ok() {
        Some(v) => {
            let lv = v.to_ascii_lowercase();
            lv == "1" || lv == "true" || lv == "on"
        }
        None => false,
    }
}

/// Generic boolean env parser with default when unset.
pub fn env_bool_default(key: &str, default: bool) -> bool {
    match std::env::var(key).ok() {
        Some(v) => {
            let lv = v.to_ascii_lowercase();
            lv == "1" || lv == "true" || lv == "on"
        }
        None => default,
    }
}

pub(crate) fn env_flag(var: &str) -> Option<bool> {
    std::env::var(var).ok().map(|v| {
        let lv = v.to_ascii_lowercase();
        lv == "1" || lv == "true" || lv == "on"
    })
}

/// Generic env presence check: returns true when the key is set (any value).
pub fn env_present(key: &str) -> bool {
    std::env::var(key).is_ok()
}

/// Generic env string fetch.
pub fn env_string(key: &str) -> Option<String> {
    std::env::var(key).ok()
}

pub(crate) fn warn_alias_once(alias: &str, primary: &str) {
    let set = WARNED_ALIASES.get_or_init(|| Mutex::new(HashSet::new()));
    if let Ok(mut s) = set.lock() {
        if !s.contains(alias) {
            let ring0 = crate::runtime::ring0::get_global_ring0();
            ring0.log.warn(&format!(
                "[deprecate/env] '{}' is deprecated; use '{}'",
                alias, primary
            ));
            s.insert(alias.to_string());
        }
    }
}
