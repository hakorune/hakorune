//! FileBox 📁 - ファイルI/O（PathBox/DirBoxとセット）
// Nyashの箱システムによるファイル入出力を提供します。
// 参考: 既存Boxの設計思想

// SSOT: FileBox は「FileIo provider を常に経由する」（provider_lock に一元化）。
// provider の有無・必須/optional の判定は provider_lock/CoreBoxId の責務で、
// FileBox 実装内では生の環境変数や静的状態を見ない設計。

// SSOT provider design (ring‑0/1) — modules are currently placeholders
pub mod box_shim; // Thin delegating shim
pub mod builtin_factory;
pub mod core_ro; // Core read‑only provider
pub mod errors; // Phase 110.5: Shared error messages
pub mod handle_box; // Phase 110: FileHandleBox (handle-based multiple-access I/O)
pub mod provider; // trait FileIo / FileCaps / FileError // Builtin FileBox ProviderFactory

// Re-export FileHandleBox for easier access
pub use handle_box::FileHandleBox;

use crate::box_trait::{BoolBox, BoxBase, BoxCore, IntegerBox, NyashBox, StringBox};
use crate::boxes::array::ArrayBox;
use crate::boxes::buffer::BufferBox;
use crate::runtime::provider_lock;
use std::any::Any;
use std::sync::Arc;

use self::errors::*;
use self::provider::FileIo;

pub struct FileBox {
    provider: Option<Arc<dyn FileIo>>,
    path: String,
    base: BoxBase,
}

impl std::fmt::Debug for FileBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FileBox")
            .field("path", &self.path)
            .field("provider", &"<FileIo>")
            .finish()
    }
}

impl Clone for FileBox {
    fn clone(&self) -> Self {
        // Clone by copying provider reference and path
        FileBox {
            provider: self.provider.clone(),
            path: self.path.clone(),
            base: BoxBase::new(),
        }
    }
}

impl FileBox {
    /// Create new FileBox (Fail-Fast if provider not initialized)
    ///
    /// Phase 109: This method panics if FileBox provider is not initialized.
    /// Use `try_new()` for graceful error handling.
    pub fn new() -> Self {
        Self::try_new().expect(&provider_not_initialized())
    }

    /// Try to create new FileBox (Result-based)
    ///
    /// Phase 109: Returns Err if FileBox provider is not initialized.
    /// This is the recommended API for graceful error handling.
    pub fn try_new() -> Result<Self, String> {
        let provider = provider_lock::get_filebox_provider()
            .ok_or_else(provider_not_initialized)?
            .clone();

        Ok(FileBox {
            provider: Some(provider),
            path: String::new(),
            base: BoxBase::new(),
        })
    }

    /// Create FileBox with explicit provider (for builtin fallback)
    pub fn with_provider(provider: Arc<dyn FileIo>) -> Self {
        FileBox {
            provider: Some(provider),
            path: String::new(),
            base: BoxBase::new(),
        }
    }

    pub fn open(path: &str) -> Result<Self, String> {
        // Allocate via provider_lock SSOT (single route for provider instantiation).
        let provider = provider_lock::new_filebox_provider_instance(Some("w"))?;

        provider
            .open(path)
            .map_err(|e| format!("Failed to open: {}", e))?;

        Ok(FileBox {
            provider: Some(provider),
            path: path.to_string(),
            base: BoxBase::new(),
        })
    }

    pub fn read_to_string(&self) -> Result<String, String> {
        if let Some(ref provider) = self.provider {
            provider.read().map_err(|e| format!("Read failed: {}", e))
        } else {
            Err(no_provider_available())
        }
    }

    pub fn write_all(&self, buf: &[u8]) -> Result<(), String> {
        if let Some(ref provider) = self.provider {
            let caps = provider.caps();
            if !caps.write {
                return Err(write_not_supported());
            }
            // Phase 108: UTF-8 conversion (text-oriented design)
            let text = String::from_utf8_lossy(buf).to_string();
            provider
                .write(&text)
                .map_err(|e| format!("Write failed: {:?}", e))
        } else {
            Err(no_provider_available())
        }
    }

    pub fn read_bytes(&self) -> Result<Vec<u8>, String> {
        if let Some(ref provider) = self.provider {
            provider
                .read_bytes()
                .map_err(|e| format!("Read bytes failed: {}", e))
        } else {
            Err(no_provider_available())
        }
    }

    pub fn write_bytes(&self, data: &[u8]) -> Result<(), String> {
        if let Some(ref provider) = self.provider {
            let caps = provider.caps();
            if !caps.write {
                return Err(write_not_supported());
            }
            provider
                .write_bytes(data)
                .map_err(|e| format!("Write bytes failed: {}", e))
        } else {
            Err(no_provider_available())
        }
    }

    /// Nyash VM helper: open file into this FileBox's provider state.
    ///
    /// This is intentionally small and host-driven: it enables `.hako` tooling
    /// (Stage-B helpers, mirbuilder pins) to read a file without requiring a plugin FileBox.
    pub fn ny_open(&self, path: &str, mode: &str) -> Result<(), String> {
        let provider = self.provider.as_ref().ok_or_else(no_provider_available)?;
        provider.caps().check_mode(mode)?;
        if let Some(ring0_provider) = provider
            .as_any()
            .downcast_ref::<crate::providers::ring1::file::ring0_fs_fileio::Ring0FsFileIo>(
        ) {
            ring0_provider.set_mode(mode.to_string());
            if mode == "w" || mode == "a" {
                let ring0 = crate::runtime::get_global_ring0();
                let path_obj = std::path::Path::new(path);
                if !ring0.fs.exists(path_obj) {
                    ring0
                        .fs
                        .write_all(path_obj, &[])
                        .map_err(|e| format!("Failed to create file: {}", e))?;
                }
            }
        }
        provider.open(path).map_err(|e| format!("{}", e))?;
        Ok(())
    }

    /// Nyash VM helper: read from the currently opened file.
    pub fn ny_read_to_string(&self) -> Result<String, String> {
        self.read_to_string()
    }

    /// Nyash VM helper: read file bytes from the currently opened file.
    pub fn ny_read_bytes(&self) -> Result<Vec<u8>, String> {
        self.read_bytes()
    }

    /// Nyash VM helper: write file bytes to the currently opened file.
    pub fn ny_write_bytes(&self, data: &[u8]) -> Result<(), String> {
        self.write_bytes(data)
    }

    /// Nyash VM helper: close the currently opened file.
    pub fn ny_close(&self) -> Result<(), String> {
        let provider = self.provider.as_ref().ok_or_else(no_provider_available)?;
        provider.close().map_err(|e| format!("{}", e))?;
        Ok(())
    }

    /// ファイルの内容を読み取る
    pub fn read(&self) -> Box<dyn NyashBox> {
        match self.read_to_string() {
            Ok(content) => Box::new(StringBox::new(&content)),
            Err(e) => Box::new(StringBox::new(&format!("Error reading file: {}", e))),
        }
    }

    /// バイト列を読み取る（ArrayBox<Integer 0..255>）
    pub fn readBytes(&self) -> Box<dyn NyashBox> {
        match self.read_bytes() {
            Ok(content) => {
                let arr = ArrayBox::new();
                for byte in content {
                    arr.push(Box::new(IntegerBox::new(byte as i64)));
                }
                Box::new(arr)
            }
            Err(e) => Box::new(StringBox::new(format!("Error reading bytes: {}", e))),
        }
    }

    /// ファイルに内容を書き込む
    pub fn write(&self, content: Box<dyn NyashBox>) -> Box<dyn NyashBox> {
        if let Some(ref provider) = self.provider {
            let caps = provider.caps();
            if !caps.write {
                return Box::new(StringBox::new(write_not_supported_readonly()));
            }
            // Phase 108: Convert content to text
            let text = if let Some(str_box) = content.as_any().downcast_ref::<StringBox>() {
                str_box.to_string_box().value
            } else {
                content.to_string_box().value
            };

            match provider.write(&text) {
                Ok(()) => Box::new(StringBox::new("OK".to_string())),
                Err(e) => Box::new(StringBox::new(format!("Error: {:?}", e))),
            }
        } else {
            Box::new(StringBox::new(format!(
                "Error: {}",
                no_provider_available()
            )))
        }
    }

    /// バイト列を書き込む（ArrayBox<Integer 0..255> または BufferBox）
    pub fn writeBytes(&self, content: Box<dyn NyashBox>) -> Box<dyn NyashBox> {
        let bytes = if let Some(buffer_box) = content.as_any().downcast_ref::<BufferBox>() {
            Ok(buffer_box.to_vec())
        } else if let Some(array_box) = content.as_any().downcast_ref::<ArrayBox>() {
            arraybox_to_bytes(array_box)
        } else {
            Err("writeBytes() requires ArrayBox<Integer> or BufferBox".to_string())
        };

        match bytes.and_then(|b| self.write_bytes(&b)) {
            Ok(()) => Box::new(StringBox::new("OK".to_string())),
            Err(e) => Box::new(StringBox::new(format!("Error: {}", e))),
        }
    }

    /// ファイルが存在するかチェック
    pub fn exists(&self) -> Box<dyn NyashBox> {
        if let Some(ref provider) = self.provider {
            Box::new(BoolBox::new(provider.exists()))
        } else {
            Box::new(BoolBox::new(false))
        }
    }
}

impl BoxCore for FileBox {
    fn box_id(&self) -> u64 {
        self.base.id
    }

    fn parent_type_id(&self) -> Option<std::any::TypeId> {
        self.base.parent_type_id
    }

    fn fmt_box(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "FileBox({})", self.path)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl NyashBox for FileBox {
    fn clone_box(&self) -> Box<dyn NyashBox> {
        // Clone by copying provider and path reference
        Box::new(self.clone())
    }

    /// 仮実装: clone_boxと同じ（後で修正）
    fn share_box(&self) -> Box<dyn NyashBox> {
        self.clone_box()
    }

    fn to_string_box(&self) -> StringBox {
        StringBox::new(format!("FileBox({})", self.path))
    }

    fn type_name(&self) -> &'static str {
        "FileBox"
    }

    fn equals(&self, other: &dyn NyashBox) -> BoolBox {
        if let Some(other_file) = other.as_any().downcast_ref::<FileBox>() {
            BoolBox::new(self.path == other_file.path)
        } else {
            BoolBox::new(false)
        }
    }
}

impl std::fmt::Display for FileBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_box(f)
    }
}

fn arraybox_to_bytes(array_box: &ArrayBox) -> Result<Vec<u8>, String> {
    array_box.with_items_read(|items| {
        let mut out = Vec::with_capacity(items.len());
        for item in items.iter() {
            let Some(int_box) = item.as_any().downcast_ref::<IntegerBox>() else {
                return Err("ArrayBox must contain only IntegerBox values".to_string());
            };
            if !(0..=255).contains(&int_box.value) {
                return Err(format!(
                    "ArrayBox byte value out of range (0..255): {}",
                    int_box.value
                ));
            }
            out.push(int_box.value as u8);
        }
        Ok(out)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::providers::ring1::file::ring0_fs_fileio::Ring0FsFileIo;
    use crate::runtime::ring0::{default_ring0, GLOBAL_RING0};
    use std::fs;
    use std::io::Write;

    fn setup_test_file(path: &str, content: &str) {
        let mut file = fs::File::create(path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }

    fn cleanup_test_file(path: &str) {
        let _ = fs::remove_file(path);
    }

    /// Helper: Initialize FileBox provider for tests
    fn init_test_provider() {
        use crate::runtime::ring0::get_global_ring0;

        // Use GLOBAL_RING0 as the single source of truth; avoid double-init panics when
        // other tests already set up Ring0.
        GLOBAL_RING0.get_or_init(|| Arc::new(default_ring0()));

        // Get the initialized Ring0 context
        let ring0_arc = get_global_ring0();
        let provider = Arc::new(Ring0FsFileIo::new(ring0_arc));
        let _ = provider_lock::set_filebox_provider(provider);
    }

    // Test 1: FileBox::new() - provider initialized
    #[test]
    fn test_filebox_new_success() {
        init_test_provider();

        let fb = FileBox::new();
        assert_eq!(fb.type_name(), "FileBox");
        assert!(fb.provider.is_some());
    }

    // Test 2: FileBox::try_new() - provider not initialized
    #[test]
    fn test_filebox_try_new_with_provider() {
        init_test_provider();

        let result = FileBox::try_new();
        assert!(result.is_ok());
        let fb = result.unwrap();
        assert_eq!(fb.type_name(), "FileBox");
    }

    // Test 3: FileBox::open() - file open
    #[test]
    fn test_filebox_open_success() {
        init_test_provider();

        let tmp_path = "/tmp/phase110_test_filebox_open.txt";
        setup_test_file(tmp_path, "test content");

        let result = FileBox::open(tmp_path);
        assert!(result.is_ok());

        let fb = result.unwrap();
        assert_eq!(fb.path, tmp_path);

        cleanup_test_file(tmp_path);
    }

    // Test 4: FileBox::read() - read file contents
    #[test]
    fn test_filebox_read_success() {
        init_test_provider();

        let tmp_path = "/tmp/phase110_test_filebox_read.txt";
        setup_test_file(tmp_path, "test content");

        let fb = FileBox::open(tmp_path).expect("open failed");
        let content_box = fb.read();
        let content = content_box.to_string_box().value;

        assert_eq!(content, "test content");

        cleanup_test_file(tmp_path);
    }

    // Test 5: FileBox::write() - write to file
    #[test]
    fn test_filebox_write_success() {
        init_test_provider();

        let tmp_path = "/tmp/phase110_test_filebox_write.txt";
        setup_test_file(tmp_path, "initial content");

        let fb = FileBox::open(tmp_path).expect("open failed");
        let content = Box::new(StringBox::new("new content"));
        let result_box = fb.write(content);
        let result_str = result_box.to_string_box().value;

        assert!(result_str.contains("OK") || result_str == "OK");

        // Verify file was written
        let written = fs::read_to_string(tmp_path).unwrap();
        assert_eq!(written, "new content");

        cleanup_test_file(tmp_path);
    }

    // Test 6: FileBox::exists() - check file existence
    #[test]
    fn test_filebox_exists() {
        init_test_provider();

        let tmp_path = "/tmp/phase110_test_filebox_exists.txt";
        setup_test_file(tmp_path, "test");

        let fb = FileBox::open(tmp_path).expect("open failed");
        let exists_box = fb.exists();
        let exists = exists_box.as_any().downcast_ref::<BoolBox>().unwrap().value;

        assert!(exists);

        cleanup_test_file(tmp_path);

        // Test non-existent file
        let fb2 = FileBox::new();
        let not_exists_box = fb2.exists();
        let not_exists = not_exists_box
            .as_any()
            .downcast_ref::<BoolBox>()
            .unwrap()
            .value;

        assert!(!not_exists);
    }

    // Test 7: FileBox clone/share
    #[test]
    fn test_filebox_clone_and_share() {
        init_test_provider();

        let fb = FileBox::new();
        let cloned = fb.clone_box();
        assert_eq!(cloned.type_name(), "FileBox");

        let shared = fb.share_box();
        assert_eq!(shared.type_name(), "FileBox");
    }

    // Test 8: FileBox equals
    #[test]
    fn test_filebox_equals() {
        init_test_provider();

        let tmp_path = "/tmp/phase110_test_filebox_equals.txt";
        setup_test_file(tmp_path, "test");

        let fb1 = FileBox::open(tmp_path).expect("open fb1");
        let fb2 = FileBox::open(tmp_path).expect("open fb2");

        let equals_box = fb1.equals(&fb2 as &dyn NyashBox);
        assert!(equals_box.value);

        cleanup_test_file(tmp_path);
    }

    #[test]
    fn test_filebox_read_write_bytes_roundtrip() {
        init_test_provider();

        let tmp_path = "/tmp/phase110_test_filebox_bytes.bin";
        setup_test_file(tmp_path, "seed");

        let fb = FileBox::open(tmp_path).expect("open failed");
        let bytes = vec![0, 1, 2, 3, 254, 255];
        fb.write_bytes(&bytes).expect("write bytes failed");

        let roundtrip = fb.read_bytes().expect("read bytes failed");
        assert_eq!(roundtrip, bytes);

        cleanup_test_file(tmp_path);
    }

    #[test]
    fn test_filebox_ny_open_write_mode_supports_missing_output() {
        init_test_provider();

        let tmp_path = "/tmp/phase110_test_filebox_ny_open_write.txt";
        cleanup_test_file(tmp_path);

        let fb = FileBox::new();
        fb.ny_open(tmp_path, "w").expect("ny_open write failed");
        let result_box = fb.write(Box::new(StringBox::new("written by ny_open")));
        assert_eq!(result_box.to_string_box().value, "OK");
        fb.ny_close().expect("ny_close failed");

        let written = fs::read_to_string(tmp_path).expect("written file");
        assert_eq!(written, "written by ny_open");

        cleanup_test_file(tmp_path);
    }

    #[test]
    fn test_arraybox_to_bytes_rejects_out_of_range() {
        let arr = ArrayBox::new();
        arr.push(Box::new(IntegerBox::new(256)));

        let err = arraybox_to_bytes(&arr).expect_err("must reject out-of-range values");
        assert!(err.contains("out of range"));
    }
}
