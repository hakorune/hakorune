use super::*;
use std::fs;

// Import test helpers from tests/common/file_box_helpers.rs
// Note: These helpers are defined in the tests crate, so we can't import them here
// Instead, we'll keep local helpers for now and document the external helpers
// TODO: Consider moving these tests to integration tests to use shared helpers

fn setup_test_file(path: &str, content: &str) {
    use std::io::Write;
    let mut file = fs::File::create(path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

fn cleanup_test_file(path: &str) {
    let _ = fs::remove_file(path);
}

/// Helper: Initialize FileBox provider for tests
fn init_test_provider() {
    use crate::providers::ring1::file::ring0_fs_fileio::Ring0FsFileIo;
    use crate::runtime::ring0::{default_ring0, init_global_ring0};
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
    let _ = crate::runtime::provider_lock::set_filebox_provider(provider);
}

#[test]
fn test_filehandlebox_basic_write_read() {
    init_test_provider();

    let tmp_path = "/tmp/phase110_test_write_read.txt";

    // Write to file
    let mut h = FileHandleBox::new();
    assert!(!h.is_open());

    h.open(tmp_path, "w").expect("open for write failed");
    assert!(h.is_open());

    h.write_all("hello world").expect("write failed");
    h.close().expect("close failed");
    assert!(!h.is_open());

    // Read from file (separate instance)
    let mut h2 = FileHandleBox::new();
    h2.open(tmp_path, "r").expect("open for read failed");
    let content = h2.read_to_string().expect("read failed");
    assert_eq!(content, "hello world");
    h2.close().expect("close failed");

    cleanup_test_file(tmp_path);
}

#[test]
fn test_filehandlebox_double_open_error() {
    init_test_provider();

    let tmp_path = "/tmp/phase110_test_double_open.txt";
    setup_test_file(tmp_path, "test");

    let mut h = FileHandleBox::new();
    h.open(tmp_path, "r").expect("first open");

    // Second open should fail
    let result = h.open(tmp_path, "r");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("already open"));

    h.close().expect("close");
    cleanup_test_file(tmp_path);
}

#[test]
fn test_filehandlebox_closed_access_error() {
    init_test_provider();

    let tmp_path = "/tmp/phase110_test_closed_access.txt";
    setup_test_file(tmp_path, "test");

    let mut h = FileHandleBox::new();
    h.open(tmp_path, "r").expect("open");
    h.close().expect("close");

    // Read after close should fail
    let result = h.read_to_string();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not open"));

    cleanup_test_file(tmp_path);
}

#[test]
fn test_filehandlebox_write_wrong_mode() {
    init_test_provider();

    let tmp_path = "/tmp/phase110_test_write_wrong_mode.txt";
    setup_test_file(tmp_path, "test");

    let mut h = FileHandleBox::new();
    h.open(tmp_path, "r").expect("open for read");

    // Write in read mode should fail
    let result = h.write_all("data");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("read-only"));

    h.close().expect("close");
    cleanup_test_file(tmp_path);
}

#[test]
fn test_filehandlebox_multiple_writes() {
    init_test_provider();

    // Use a dedicated path to avoid races with other write tests sharing the same file.
    let tmp_path = "/tmp/phase110_test_multiple_writes_truncate.txt";

    let mut h = FileHandleBox::new();
    h.open(tmp_path, "w").expect("open");

    // Multiple writes (note: current design is truncate mode)
    h.write_all("first write").expect("first write");
    // Second write will overwrite (truncate mode)
    h.write_all("second write").expect("second write");

    h.close().expect("close");

    // Verify final content
    let content = fs::read_to_string(tmp_path).unwrap();
    assert_eq!(content, "second write");

    cleanup_test_file(tmp_path);
}

#[test]
fn test_filehandlebox_unsupported_mode() {
    init_test_provider();

    let mut h = FileHandleBox::new();
    // Phase 111: "a" is now supported, test with "x" instead
    let result = h.open("/tmp/test.txt", "x");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Unsupported mode"));
}

#[test]
fn test_filehandlebox_independent_instances() {
    init_test_provider();

    let tmp_path1 = "/tmp/phase110_test_independent1.txt";
    let tmp_path2 = "/tmp/phase110_test_independent2.txt";
    setup_test_file(tmp_path1, "file1");
    setup_test_file(tmp_path2, "file2");

    // Two independent handles
    let mut h1 = FileHandleBox::new();
    let mut h2 = FileHandleBox::new();

    h1.open(tmp_path1, "r").expect("h1 open");
    h2.open(tmp_path2, "r").expect("h2 open");

    let content1 = h1.read_to_string().expect("h1 read");
    let content2 = h2.read_to_string().expect("h2 read");

    assert_eq!(content1, "file1");
    assert_eq!(content2, "file2");

    h1.close().expect("h1 close");
    h2.close().expect("h2 close");

    cleanup_test_file(tmp_path1);
    cleanup_test_file(tmp_path2);
}

// ==================== Priority 4: Integration Tests ====================

// Integration Test 1: 複数 FileHandleBox インスタンスが同一ファイルを読む
#[test]
fn test_multiple_filehandle_concurrent_read() {
    init_test_provider();

    let tmp_path = "/tmp/phase110_test_concurrent_read.txt";
    setup_test_file(tmp_path, "shared content");

    let mut h1 = FileHandleBox::new();
    let mut h2 = FileHandleBox::new();

    h1.open(tmp_path, "r").expect("h1 open");
    h2.open(tmp_path, "r").expect("h2 open");

    let content1 = h1.read_to_string().expect("h1 read");
    let content2 = h2.read_to_string().expect("h2 read");

    assert_eq!(content1, content2);
    assert_eq!(content1, "shared content");

    h1.close().expect("h1 close");
    h2.close().expect("h2 close");

    cleanup_test_file(tmp_path);
}

// Integration Test 2: 複数 FileHandleBox インスタンスが異なるファイルを操作
#[test]
fn test_multiple_filehandles_different_files() {
    init_test_provider();

    let tmp1 = "/tmp/phase110_test_handle1.txt";
    let tmp2 = "/tmp/phase110_test_handle2.txt";

    let mut h1 = FileHandleBox::new();
    let mut h2 = FileHandleBox::new();

    h1.open(tmp1, "w").expect("h1 open");
    h2.open(tmp2, "w").expect("h2 open");

    h1.write_all("file1 content").expect("h1 write");
    h2.write_all("file2 content").expect("h2 write");

    h1.close().expect("h1 close");
    h2.close().expect("h2 close");

    // Verify
    let content1 = fs::read_to_string(tmp1).expect("read tmp1");
    let content2 = fs::read_to_string(tmp2).expect("read tmp2");

    assert_eq!(content1, "file1 content");
    assert_eq!(content2, "file2 content");

    cleanup_test_file(tmp1);
    cleanup_test_file(tmp2);
}

// Integration Test 3: FileHandleBox sequential reads
#[test]
fn test_filehandle_sequential_reads() {
    init_test_provider();

    let tmp_path = "/tmp/phase110_test_sequential_reads.txt";
    setup_test_file(tmp_path, "test data");

    let mut h = FileHandleBox::new();
    h.open(tmp_path, "r").expect("open");

    // Read multiple times (each read returns the full content)
    let content1 = h.read_to_string().expect("read 1");
    let content2 = h.read_to_string().expect("read 2");
    let content3 = h.read_to_string().expect("read 3");

    assert_eq!(content1, "test data");
    assert_eq!(content2, "test data");
    assert_eq!(content3, "test data");

    h.close().expect("close");
    cleanup_test_file(tmp_path);
}

// Integration Test 4: FileHandleBox write multiple times (truncate behavior)
#[test]
fn test_filehandle_multiple_writes_truncate() {
    init_test_provider();

    let tmp_path = "/tmp/phase110_test_multiple_writes.txt";

    let mut h = FileHandleBox::new();
    h.open(tmp_path, "w").expect("open");

    h.write_all("first").expect("write 1");
    h.write_all("second").expect("write 2");
    h.write_all("third").expect("write 3");

    h.close().expect("close");

    // Verify final content (truncate mode)
    let content = fs::read_to_string(tmp_path).expect("read file");
    assert_eq!(content, "third");

    cleanup_test_file(tmp_path);
}

// ===== Phase 111: Append mode + metadata tests =====

#[test]
fn test_filehandlebox_append_mode() {
    init_test_provider();

    let path = "/tmp/phase111_append_test.txt";
    let _ = fs::remove_file(path); // cleanup

    // First write (truncate)
    let mut handle = FileHandleBox::new();
    handle.open(path, "w").unwrap();
    handle.write_all("hello\n").unwrap();
    handle.close().unwrap();

    // Append
    let mut handle = FileHandleBox::new();
    handle.open(path, "a").unwrap();
    handle.write_all("world\n").unwrap();
    handle.close().unwrap();

    // Verify
    let content = fs::read_to_string(path).unwrap();
    assert_eq!(content, "hello\nworld\n");

    let _ = fs::remove_file(path);
}

#[test]
fn test_filehandlebox_metadata_size() {
    init_test_provider();

    let path = "/tmp/phase111_metadata_test.txt";
    let _ = fs::remove_file(path);

    // Write test file
    let mut handle = FileHandleBox::new();
    handle.open(path, "w").unwrap();
    handle.write_all("hello").unwrap(); // 5 bytes
    handle.close().unwrap();

    // Check size
    let mut handle = FileHandleBox::new();
    handle.open(path, "r").unwrap();
    let size = handle.size().unwrap();
    assert_eq!(size, 5);
    handle.close().unwrap();

    let _ = fs::remove_file(path);
}

#[test]
fn test_filehandlebox_metadata_is_file() {
    init_test_provider();

    let path = "/tmp/phase111_file_test.txt";
    let _ = fs::remove_file(path);

    // Create file
    let mut handle = FileHandleBox::new();
    handle.open(path, "w").unwrap();
    handle.close().unwrap();

    // Check is_file
    let mut handle = FileHandleBox::new();
    handle.open(path, "r").unwrap();
    let is_file = handle.is_file().unwrap();
    assert!(is_file);

    let is_dir = handle.is_dir().unwrap();
    assert!(!is_dir);
    handle.close().unwrap();

    let _ = fs::remove_file(path);
}

#[test]
fn test_filehandlebox_write_readonly_error() {
    init_test_provider();

    let path = "/tmp/phase111_readonly_test.txt";
    let _ = fs::remove_file(path);

    // Create file
    fs::write(path, "content").unwrap();

    // Open in read mode
    let mut handle = FileHandleBox::new();
    handle.open(path, "r").unwrap();

    // Try to write → Error
    let result = handle.write_all("new");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("read-only"));

    handle.close().unwrap();
    let _ = fs::remove_file(path);
}

#[test]
#[ignore] // This test requires NYASH_RUNTIME_PROFILE=no-fs environment variable
fn test_filehandlebox_nofs_profile_error() {
    // Note: This test should be run with NYASH_RUNTIME_PROFILE=no-fs
    // For now, we test that open() fails with disabled message when provider is None

    // Cannot directly test NoFs profile in unit tests without setting env var
    // and reinitializing runtime. This test is marked as ignored and should be
    // run separately with proper profile configuration.

    // The test would look like:
    // let mut handle = FileHandleBox::new();
    // let result = handle.open("/tmp/test.txt", "w");
    // assert!(result.is_err());
    // assert!(result.unwrap_err().contains("disabled"));
}

// ===== Phase 113: Nyash-visible API tests =====

#[test]
fn test_phase113_ny_open_read_write_close() {
    init_test_provider();

    let path = "/tmp/phase113_ny_test.txt";
    let _ = fs::remove_file(path);

    let mut handle = FileHandleBox::new();

    // Test ny_open + ny_write + ny_close
    handle.ny_open(path, "w");
    handle.ny_write("test content\n");
    handle.ny_close();

    // Test ny_open + ny_read + ny_close
    handle.ny_open(path, "r");
    let content = handle.ny_read();
    assert_eq!(content.value, "test content\n");
    handle.ny_close();

    let _ = fs::remove_file(path);
}

#[test]
fn test_phase113_ny_append_mode() {
    init_test_provider();

    let path = "/tmp/phase113_ny_append.txt";
    let _ = fs::remove_file(path);

    let mut handle = FileHandleBox::new();

    // First write
    handle.ny_open(path, "w");
    handle.ny_write("first\n");
    handle.ny_close();

    // Append
    handle.ny_open(path, "a");
    handle.ny_write("second\n");
    handle.ny_close();

    // Read and verify
    handle.ny_open(path, "r");
    let content = handle.ny_read();
    assert_eq!(content.value, "first\nsecond\n");
    handle.ny_close();

    let _ = fs::remove_file(path);
}

#[test]
fn test_phase113_ny_metadata_methods() {
    init_test_provider();

    let path = "/tmp/phase113_ny_metadata.txt";
    let _ = fs::remove_file(path);

    let mut handle = FileHandleBox::new();

    // Create file
    handle.ny_open(path, "w");
    handle.ny_write("hello");
    handle.ny_close();

    // Test metadata methods
    handle.ny_open(path, "r");

    let exists = handle.ny_exists();
    assert!(exists.value);

    let size = handle.ny_size();
    assert_eq!(size.value, 5);

    let is_file = handle.ny_is_file();
    assert!(is_file.value);

    let is_dir = handle.ny_is_dir();
    assert!(!is_dir.value);

    handle.ny_close();

    let _ = fs::remove_file(path);
}

#[test]
#[should_panic(expected = "FileHandleBox.open() failed")]
fn test_phase113_ny_open_panic_on_error() {
    init_test_provider();

    let mut handle = FileHandleBox::new();

    // Double open should panic
    handle.ny_open("/tmp/test.txt", "w");
    handle.ny_open("/tmp/test.txt", "w"); // This should panic
}

#[test]
#[should_panic(expected = "FileHandleBox.read() failed")]
fn test_phase113_ny_read_panic_when_not_open() {
    init_test_provider();

    let handle = FileHandleBox::new();
    let _ = handle.ny_read(); // This should panic
}

#[test]
#[should_panic(expected = "FileHandleBox.write() failed")]
fn test_phase113_ny_write_panic_in_read_mode() {
    init_test_provider();

    let path = "/tmp/phase113_ny_write_panic.txt";
    fs::write(path, "content").unwrap();

    let mut handle = FileHandleBox::new();
    handle.ny_open(path, "r");
    handle.ny_write("data"); // This should panic (read-only mode)
}

// ===== Phase 114: metadata_internal() unification tests =====

#[test]
fn test_filehandlebox_metadata_internal_default() {
    init_test_provider();

    let path = "/tmp/phase114_metadata_internal.txt";
    let _ = fs::remove_file(path);

    // Create test file with known content
    let mut handle = FileHandleBox::new();
    handle.open(path, "w").unwrap();
    handle.write_all("12345").unwrap(); // 5 bytes
    handle.close().unwrap();

    // Test metadata_internal() via stat()
    handle.open(path, "r").unwrap();
    let stat = handle
        .metadata_internal()
        .expect("metadata_internal should succeed");
    assert!(stat.is_file);
    assert!(!stat.is_dir);
    assert_eq!(stat.size, 5);

    handle.close().unwrap();
    let _ = fs::remove_file(path);
}

#[test]
fn test_filehandlebox_metadata_internal_not_open() {
    init_test_provider();

    let handle = FileHandleBox::new();
    let result = handle.metadata_internal();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not open"));
}

#[test]
fn test_filehandlebox_ny_size_uses_stat() {
    init_test_provider();

    let path = "/tmp/phase114_ny_size_stat.txt";
    let _ = fs::remove_file(path);

    let mut handle = FileHandleBox::new();
    handle.ny_open(path, "w");
    handle.ny_write("test123"); // 7 bytes
    handle.ny_close();

    // Verify ny_size() uses stat() internally
    handle.ny_open(path, "r");
    let size = handle.ny_size();
    assert_eq!(size.value, 7);
    handle.ny_close();

    let _ = fs::remove_file(path);
}

#[test]
fn test_filehandlebox_exists_uses_fileio() {
    init_test_provider();

    let path = "/tmp/phase114_exists_fileio.txt";
    let _ = fs::remove_file(path);

    let mut handle = FileHandleBox::new();
    handle.ny_open(path, "w");
    handle.ny_close();

    // exists() should use FileIo::exists()
    handle.ny_open(path, "r");
    let exists = handle.ny_exists();
    assert!(exists.value);
    handle.ny_close();

    let _ = fs::remove_file(path);
}

#[test]
fn test_filehandlebox_is_file_is_dir_via_stat() {
    init_test_provider();

    let path = "/tmp/phase114_is_file_dir.txt";
    let _ = fs::remove_file(path);

    let mut handle = FileHandleBox::new();
    handle.ny_open(path, "w");
    handle.ny_close();

    // Test is_file/is_dir use metadata_internal() → stat()
    handle.ny_open(path, "r");

    let is_file = handle.is_file().expect("is_file should succeed");
    assert!(is_file);

    let is_dir = handle.is_dir().expect("is_dir should succeed");
    assert!(!is_dir);

    handle.ny_close();
    let _ = fs::remove_file(path);
}
