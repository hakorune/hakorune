/*!
 * Provider Lock (skeleton)
 *
 * Phase 15.5 受け口: 型→Provider のロック状態を保持するための最小スケルトン。
 * 既定では挙動を変えず、環境変数により警告/エラー化のみ可能にする。
 */

mod array;
mod console;
mod file;
mod map;
mod path;

use crate::config::env;
use crate::runtime::get_global_ring0;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;

pub use array::{get_arraybox_provider, new_arraybox_provider_instance, set_arraybox_provider};
pub use console::{
    get_consolebox_provider, new_consolebox_provider_instance, set_consolebox_provider,
};
pub use file::{
    get_filebox_caps, get_filebox_provider, new_filebox_provider_instance, set_filebox_provider,
};
pub use map::{get_mapbox_provider, new_mapbox_provider_instance, set_mapbox_provider};
pub use path::{
    get_pathbox_provider, get_pathbox_provider_instance, new_pathbox_provider_instance,
    set_pathbox_provider, PathService,
};

static LOCKED: AtomicBool = AtomicBool::new(false);
static WARN_ONCE: OnceLock<()> = OnceLock::new();

/// Return true when providers are locked
pub fn is_locked() -> bool {
    LOCKED.load(Ordering::Relaxed)
}

/// Lock providers (idempotent)
pub fn lock_providers() {
    LOCKED.store(true, Ordering::Relaxed);
}

/// Guard called before creating a new box instance.
/// Default: no-op. When NYASH_PROVIDER_LOCK_STRICT=1, returns Err if not locked.
/// When NYASH_PROVIDER_LOCK_WARN=1, prints a warning once.
pub fn guard_before_new_box(box_type: &str) -> Result<(), String> {
    if is_locked() {
        return Ok(());
    }
    let strict = env::env_bool("NYASH_PROVIDER_LOCK_STRICT");
    let warn = env::env_bool("NYASH_PROVIDER_LOCK_WARN");
    if strict {
        return Err(format!(
            "E_PROVIDER_NOT_LOCKED: attempted to create '{}' before Provider Lock",
            box_type
        ));
    }
    if warn {
        // Print once per process
        let _ = WARN_ONCE.get_or_init(|| {
            get_global_ring0().log.warn(
                "[provider-lock][warn] NewBox emitted before Provider Lock. Set NYASH_PROVIDER_LOCK_STRICT=1 to error.",
            );
        });
    }
    Ok(())
}
