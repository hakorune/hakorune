//! Phase 110.5: FileBox/FileHandleBox shared error messages
//!
//! This module provides a single source of truth for all error messages
//! used across FileBox and FileHandleBox implementations.
//!
//! # Design Benefits
//!
//! - **SSOT**: All error messages defined in one place
//! - **i18n Ready**: Easy to internationalize later
//! - **Consistency**: Identical errors across FileBox and FileHandleBox
//! - **Maintainability**: Update once, apply everywhere

/// Provider not initialized error (FileBox/FileHandleBox common)
pub fn provider_not_initialized() -> String {
    "FileBox provider not initialized".to_string()
}

/// File I/O disabled in no-fs profile (FileHandleBox specific)
pub fn provider_disabled_in_nofs_profile() -> String {
    "File I/O disabled in no-fs profile. FileHandleBox is not available.".to_string()
}

/// FileHandleBox already open error
pub fn already_open() -> String {
    "FileHandleBox is already open. Call close() first.".to_string()
}

/// FileHandleBox not open error
pub fn not_open() -> String {
    "FileHandleBox is not open".to_string()
}

/// Unsupported mode error (with mode name)
pub fn unsupported_mode(mode: &str) -> String {
    format!("Unsupported mode: {}. Use 'r', 'w', or 'a'", mode)
}

/// Read not supported by provider
pub fn read_not_supported() -> String {
    "Read not supported by FileBox provider".to_string()
}

/// Write not supported by provider
pub fn write_not_supported() -> String {
    "Write not supported by FileBox provider".to_string()
}

/// No provider available
pub fn no_provider_available() -> String {
    "No provider available".to_string()
}

/// FileHandleBox opened in read mode (cannot write)
pub fn opened_in_read_mode() -> String {
    "FileHandleBox opened in read mode".to_string()
}

/// Read-only provider error message
pub fn write_not_supported_readonly() -> String {
    "Error: write not supported by provider (read-only)".to_string()
}
