/// Phase 97 Refactoring: Structured Error Reporter Box for Plugin Loader
///
/// This module provides structured error reporting with clear context,
/// attempted paths, and actionable hints for plugin loading failures.
use crate::bid::BidError;
use std::path::PathBuf;

/// Structured plugin error information
#[derive(Debug, Clone)]
pub struct PluginErrorContext {
    pub kind: PluginErrorKind,
    #[allow(dead_code)]
    pub plugin_name: String,
    pub message: String,
    pub attempted_paths: Vec<String>,
    pub hint: Option<String>,
}

/// Plugin error kind classification
#[derive(Debug, Clone, PartialEq)]
pub enum PluginErrorKind {
    /// Plugin library file not found
    MissingLibrary,
    /// dlopen() failed
    LoadFailed,
    /// Plugin initialization failed
    #[allow(dead_code)]
    InitFailed,
    /// Version mismatch
    #[allow(dead_code)]
    VersionMismatch,
}

impl PluginErrorContext {
    /// Create error context for missing plugin
    pub fn missing_library(
        plugin_name: &str,
        configured_path: &str,
        attempted_paths: Vec<PathBuf>,
    ) -> Self {
        let paths_str: Vec<String> = attempted_paths
            .iter()
            .map(|p| p.display().to_string())
            .collect();

        Self {
            kind: PluginErrorKind::MissingLibrary,
            plugin_name: plugin_name.to_string(),
            message: format!(
                "Plugin '{}' not found at configured path: {}",
                plugin_name, configured_path
            ),
            attempted_paths: paths_str,
            hint: Some(
                "Check LD_LIBRARY_PATH or configure nyash.toml [libraries] section".to_string(),
            ),
        }
    }

    /// Create error context for load failure
    pub fn load_failed(plugin_name: &str, path: &str, error_msg: &str) -> Self {
        Self {
            kind: PluginErrorKind::LoadFailed,
            plugin_name: plugin_name.to_string(),
            message: format!(
                "Failed to load plugin '{}' from {}: {}",
                plugin_name, path, error_msg
            ),
            attempted_paths: vec![path.to_string()],
            hint: Some("Check plugin architecture (32/64-bit) and dependencies".to_string()),
        }
    }

    /// Create error context for init failure
    #[allow(dead_code)]
    pub fn init_failed(plugin_name: &str, error_msg: &str) -> Self {
        Self {
            kind: PluginErrorKind::InitFailed,
            plugin_name: plugin_name.to_string(),
            message: format!(
                "Plugin '{}' initialization failed: {}",
                plugin_name, error_msg
            ),
            attempted_paths: vec![],
            hint: Some("Check plugin logs for initialization errors".to_string()),
        }
    }

    /// Log structured error using global ring0 logger
    pub fn log_structured(&self) {
        use crate::runtime::get_global_ring0;

        let ring0 = get_global_ring0();

        match self.kind {
            PluginErrorKind::MissingLibrary => {
                ring0
                    .log
                    .error(&format!("[plugin/missing] {}", self.message));
                if !self.attempted_paths.is_empty() {
                    ring0.log.error(&format!(
                        "[plugin/missing] Attempted paths: {}",
                        self.attempted_paths.join(", ")
                    ));
                }
                if let Some(ref hint) = self.hint {
                    ring0.log.warn(&format!("[plugin/hint] {}", hint));
                }
            }
            PluginErrorKind::LoadFailed => {
                ring0.log.error(&format!("[plugin/init] {}", self.message));
                if let Some(ref hint) = self.hint {
                    ring0.log.warn(&format!("[plugin/hint] {}", hint));
                }
            }
            PluginErrorKind::InitFailed => {
                ring0.log.error(&format!("[plugin/init] {}", self.message));
                if let Some(ref hint) = self.hint {
                    ring0.log.warn(&format!("[plugin/hint] {}", hint));
                }
            }
            PluginErrorKind::VersionMismatch => {
                ring0
                    .log
                    .error(&format!("[plugin/version] {}", self.message));
                if let Some(ref hint) = self.hint {
                    ring0.log.warn(&format!("[plugin/hint] {}", hint));
                }
            }
        }
    }

    /// Convert to BidError
    pub fn to_bid_error(&self) -> BidError {
        match self.kind {
            PluginErrorKind::MissingLibrary => BidError::PluginError,
            PluginErrorKind::LoadFailed => BidError::PluginError,
            PluginErrorKind::InitFailed => BidError::PluginError,
            PluginErrorKind::VersionMismatch => BidError::VersionMismatch,
        }
    }
}

/// Helper function for logging and returning error
pub fn report_and_fail(ctx: PluginErrorContext) -> BidError {
    ctx.log_structured();
    ctx.to_bid_error()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_library_context() {
        let ctx = PluginErrorContext::missing_library(
            "test_plugin",
            "/usr/lib/test.so",
            vec![
                PathBuf::from("/usr/lib/test.so"),
                PathBuf::from("/usr/lib/libtest.so"),
            ],
        );

        assert_eq!(ctx.kind, PluginErrorKind::MissingLibrary);
        assert_eq!(ctx.plugin_name, "test_plugin");
        assert_eq!(ctx.attempted_paths.len(), 2);
        assert!(ctx.hint.is_some());
    }

    #[test]
    fn test_load_failed_context() {
        let ctx = PluginErrorContext::load_failed(
            "test_plugin",
            "/usr/lib/test.so",
            "undefined symbol: foo",
        );

        assert_eq!(ctx.kind, PluginErrorKind::LoadFailed);
        assert!(ctx.message.contains("undefined symbol"));
        assert_eq!(ctx.attempted_paths.len(), 1);
    }
}
