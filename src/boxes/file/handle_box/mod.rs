//! Phase 110: FileHandleBox - Handle-based file I/O
//!
//! Provides multiple-access file I/O pattern:
//! - open(path, mode) → read/write (multiple times) → close()
//!
//! Complements FileBox (one-shot I/O) by allowing multiple reads/writes
//! to the same file handle.

use crate::box_trait::{BoolBox, BoxBase, StringBox};
use crate::boxes::file::provider::FileIo;
use std::sync::Arc;

mod core;
mod io;
mod metadata;
mod traits;
#[cfg(test)]
mod tests;

// ===== Phase 115: Helper macros for Nyash wrapper methods =====

macro_rules! ny_wrap_void {
    ($name:ident, $inner:ident, $display_name:expr, $($arg_name:ident: $arg_ty:ty),*) => {
        #[allow(unused_mut)]
        pub fn $name(&mut self, $($arg_name: $arg_ty),*) {
            self.$inner($($arg_name),*).unwrap_or_else(|e| panic!("FileHandleBox.{}() failed: {}", $display_name, e));
        }
    };
}

macro_rules! ny_wrap_string {
    ($name:ident, $inner:ident) => {
        pub fn $name(&self) -> StringBox {
            match self.$inner() {
                Ok(result) => StringBox::new(result),
                Err(e) => panic!(
                    "FileHandleBox.{}() failed: {}",
                    stringify!($name).trim_start_matches("ny_"),
                    e
                ),
            }
        }
    };
}

macro_rules! ny_wrap_bool {
    ($name:ident, $inner:ident) => {
        pub fn $name(&self) -> BoolBox {
            match self.$inner() {
                Ok(result) => BoolBox::new(result),
                Err(e) => panic!(
                    "FileHandleBox.{}() failed: {}",
                    stringify!($name).trim_start_matches("ny_"),
                    e
                ),
            }
        }
    };
}

macro_rules! ny_wrap_integer {
    ($name:ident, $inner:ident) => {
        pub fn $name(&self) -> crate::box_trait::IntegerBox {
            match self.$inner() {
                Ok(result) => crate::box_trait::IntegerBox::new(result as i64),
                Err(e) => panic!(
                    "FileHandleBox.{}() failed: {}",
                    stringify!($name).trim_start_matches("ny_"),
                    e
                ),
            }
        }
    };
}

/// Phase 110: FileHandleBox
///
/// Handle-based file I/O for multiple-access patterns.
///
/// # Lifecycle
///
/// 1. new() - Create handle (file not yet opened)
/// 2. open(path, mode) - Open file (stores FileIo instance)
/// 3. read_to_string() / write_all() - Multiple accesses allowed
/// 4. close() - Close file (resets FileIo)
///
/// # Design Principles
///
/// - **Fail-Fast**: Double open() returns Err
/// - **Independent instances**: Each FileHandleBox has its own FileIo
/// - **Profile-aware**: NoFs profile → open() returns Err
/// - **Ring0 reuse**: Uses Ring0FsFileIo internally
///
/// # Code Organization
///
/// Phase 115: モジュール化・箱化実装
/// - Nyash メソッド (ny_*) はマクロで統一化（重複削減）
/// - テストヘルパーは tests/common/file_box_helpers.rs に外出し
/// - NyashBox trait impl は最小化（ボイラープレート削減）
pub struct FileHandleBox {
    pub(super) base: BoxBase,
    /// Current file path (empty if not open)
    pub(super) path: String,
    /// Current file mode ("r" or "w", empty if not open)
    pub(super) mode: String,
    /// FileIo instance (None if not open)
    pub(super) io: Option<Arc<dyn FileIo>>,
}

impl FileHandleBox {
    // ===== Phase 113: Nyash-visible public API methods =====
    // Phase 115: Using macros for wrapper methods (defined at module level)

    ny_wrap_void!(ny_open, open, "open", path: &str, mode: &str);
    ny_wrap_string!(ny_read, read_to_string);
    ny_wrap_void!(ny_write, write_all, "write", text: &str);
    ny_wrap_void!(ny_close, close, "close",);
    ny_wrap_bool!(ny_exists, exists);
    ny_wrap_integer!(ny_size, size);
    ny_wrap_bool!(ny_is_file, is_file);
    ny_wrap_bool!(ny_is_dir, is_dir);
}
