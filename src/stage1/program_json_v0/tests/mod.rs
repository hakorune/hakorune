use super::{
    emit_program_json_v0_for_current_stage1_build_box_mode,
    emit_program_json_v0_for_stage1_bridge_emit_program_json,
    emit_program_json_v0_for_stage1_build_box, emit_program_json_v0_for_strict_authority_source,
    routing, source_to_program_json_v0_relaxed, source_to_program_json_v0_strict,
    strict_authority_program_json_v0_source_rejection, STAGE1_PROGRAM_JSON_V0_FREEZE_TAG,
};
use std::collections::BTreeSet;
use std::sync::{Mutex, MutexGuard, OnceLock};

fn env_guard() -> &'static Mutex<()> {
    static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
    GUARD.get_or_init(|| Mutex::new(()))
}

struct FeatureOverrideGuard {
    prev: Option<String>,
    _lock: MutexGuard<'static, ()>,
}

impl FeatureOverrideGuard {
    fn new(features: Option<&str>) -> Self {
        let lock = match env_guard().lock() {
            Ok(lock) => lock,
            Err(poisoned) => poisoned.into_inner(),
        };
        let prev = std::env::var("NYASH_FEATURES").ok();
        match features {
            Some(v) => std::env::set_var("NYASH_FEATURES", v),
            None => std::env::remove_var("NYASH_FEATURES"),
        }
        Self { prev, _lock: lock }
    }
}

impl Drop for FeatureOverrideGuard {
    fn drop(&mut self) {
        match &self.prev {
            Some(v) => std::env::set_var("NYASH_FEATURES", v),
            None => std::env::remove_var("NYASH_FEATURES"),
        }
    }
}

fn with_features<R>(features: Option<&str>, f: impl FnOnce() -> R) -> R {
    let _guard = FeatureOverrideGuard::new(features);
    f()
}

mod basics_and_enums;
mod classification_contract;
mod routing_and_emission;
mod stage1_sources;
