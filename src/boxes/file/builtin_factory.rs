//! Builtin FileBox Factory
//!
//! Provides ProviderFactory implementation for the builtin FileBox (core-ro).
//! This is auto-registered when feature "builtin-filebox" is enabled.

use crate::boxes::file::core_ro::CoreRoFileIo;
use crate::boxes::file::provider::FileIo;
use crate::runner::modes::common_util::provider_registry::{
    register_provider_factory, ProviderFactory,
};
use std::sync::Arc;

/// Builtin FileBox factory (static registration)
pub struct BuiltinFileBoxFactory;

impl ProviderFactory for BuiltinFileBoxFactory {
    fn box_name(&self) -> &str {
        "FileBox"
    }

    fn create_provider(&self) -> Arc<dyn FileIo> {
        Arc::new(CoreRoFileIo::new())
    }

    fn is_available(&self) -> bool {
        cfg!(feature = "builtin-filebox")
    }

    fn priority(&self) -> i32 {
        10 // Builtin priority (lower than dynamic plugins: 100)
    }
}

/// Auto-register builtin FileBox factory
///
/// Call this during VM initialization to register the builtin provider.
/// This is idempotent and safe to call multiple times.
pub fn register_builtin_filebox() {
    register_provider_factory(Arc::new(BuiltinFileBoxFactory));

    let quiet_pipe = crate::config::env::env_bool("NYASH_JSON_ONLY");
    if !quiet_pipe {
        crate::runtime::get_global_ring0()
            .log
            .info("[builtin-factory] FileBox: registered (priority=10)");
    }
}
