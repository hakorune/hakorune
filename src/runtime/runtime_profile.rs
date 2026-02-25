//! Phase 109: RuntimeProfile enum
//!
//! Controls conditional core service initialization based on runtime profile.
//! Supports Default (selfhost/standard) and NoFs (minimal runtime without filesystem).
//!
//! # Profile Variants (Current and Future)
//!
//! ## Current Profiles
//!
//! - **Default**: Standard runtime (selfhost/default)
//!   - All core services enabled
//!   - FileBox provider required (Fail-Fast)
//!   - Full filesystem access via Ring0.FsApi
//!
//! - **NoFs**: Minimal runtime without FileSystem
//!   - Core services only (String/Integer/Bool/Array/Map/Console)
//!   - FileBox provider optional (NoFsFileIo stub)
//!   - Suitable for sandboxed/embedded environments
//!
//! ## Future Expansion Plans
//!
//! The following profiles are planned for future implementation:
//!
//! - **TestMock**: Test-only profile
//!   - All boxes return mocks for testing
//!   - Predictable behavior for unit tests
//!   - No side effects (no file I/O, no network)
//!
//! - **Sandbox**: Isolated filesystem
//!   - FileBox limited to designated sandbox directory
//!   - No external I/O (network disabled)
//!   - Memory and resource limits enforced
//!
//! - **ReadOnly**: Read-only mode
//!   - FileBox.read() enabled
//!   - FileBox.write() denied at capability level
//!   - Suitable for immutable environments
//!
//! - **Embedded**: Embedded profile
//!   - Memory limits enforced
//!   - Console output optional (may be disabled)
//!   - Reduced feature set for resource-constrained devices

/// Phase 109: RuntimeProfile
///
/// Controls availability of FileBox and other optional services.
///
/// - Default: selfhost/standard - most services enabled (FileBox required)
/// - NoFs: minimal runtime - FileBox disabled, core boxes only
///
/// Future expansion planned: TestMock, Sandbox, ReadOnly, Embedded
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeProfile {
    /// Standard runtime (selfhost/default)
    Default,
    /// Minimal runtime without FileSystem
    NoFs,
}

impl RuntimeProfile {
    /// Read RuntimeProfile from NYASH_RUNTIME_PROFILE environment variable
    ///
    /// # Recognized values
    ///
    /// - `"no-fs"` or `"nofs"` → NoFs
    /// - Any other value or missing → Default
    pub fn from_env() -> Self {
        match std::env::var("NYASH_RUNTIME_PROFILE").as_deref() {
            Ok("no-fs") | Ok("nofs") => RuntimeProfile::NoFs,
            _ => RuntimeProfile::Default,
        }
    }

    /// Get profile name for debugging
    pub fn name(&self) -> &'static str {
        match self {
            RuntimeProfile::Default => "Default",
            RuntimeProfile::NoFs => "NoFs",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    fn with_env_lock<F: FnOnce()>(f: F) {
        let _guard = ENV_LOCK.lock().expect("env lock poisoned");
        f();
    }

    #[test]
    fn test_runtime_profile_from_env_default() {
        with_env_lock(|| {
            // Without setting env, should return Default
            std::env::remove_var("NYASH_RUNTIME_PROFILE");
            assert_eq!(RuntimeProfile::from_env(), RuntimeProfile::Default);
        });
    }

    #[test]
    fn test_runtime_profile_from_env_nofs() {
        with_env_lock(|| {
            // Test "no-fs" variant
            std::env::set_var("NYASH_RUNTIME_PROFILE", "no-fs");
            assert_eq!(RuntimeProfile::from_env(), RuntimeProfile::NoFs);

            // Test "nofs" variant
            std::env::set_var("NYASH_RUNTIME_PROFILE", "nofs");
            assert_eq!(RuntimeProfile::from_env(), RuntimeProfile::NoFs);

            std::env::remove_var("NYASH_RUNTIME_PROFILE");
        });
    }

    #[test]
    fn test_runtime_profile_name() {
        assert_eq!(RuntimeProfile::Default.name(), "Default");
        assert_eq!(RuntimeProfile::NoFs.name(), "NoFs");
    }

    #[test]
    fn test_runtime_profile_from_env_unknown_defaults_to_default() {
        with_env_lock(|| {
            // Unknown profile env var → Default に fallback
            std::env::set_var("NYASH_RUNTIME_PROFILE", "unknown-profile");
            assert_eq!(RuntimeProfile::from_env(), RuntimeProfile::Default);
            std::env::remove_var("NYASH_RUNTIME_PROFILE");
        });
    }
}
