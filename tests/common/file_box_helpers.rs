//! Common test helpers for FileBox/FileHandleBox tests

use std::fs;
use std::path::Path;

/// Setup a test file with content
pub fn setup_test_file(path: &str, content: &str) {
    if let Some(parent) = Path::new(path).parent() {
        if !parent.as_os_str().is_empty() {
            let _ = fs::create_dir_all(parent);
        }
    }
    let _ = fs::write(path, content);
}

/// Cleanup a test file
pub fn cleanup_test_file(path: &str) {
    let _ = fs::remove_file(path);
}

/// Initialize test provider (Ring0 context)
pub fn init_test_provider() {
    use nyash_kernel::runtime::ring0::{default_ring0, init_global_ring0};
    use nyash_kernel::providers::ring1::file::ring0_fs_fileio::Ring0FsFileIo;
    use nyash_kernel::runtime::provider_lock;
    use std::panic;
    use std::sync::Arc;

    // Try to initialize Ring0 (ignore if already initialized)
    let _ = panic::catch_unwind(|| {
        let ring0 = default_ring0();
        init_global_ring0(ring0);
    });

    // Set provider if not already set (ignore errors from re-initialization)
    let ring0_arc = Arc::new(default_ring0());
    let provider = Arc::new(Ring0FsFileIo::new(ring0_arc));
    let _ = provider_lock::set_filebox_provider(provider);
}
