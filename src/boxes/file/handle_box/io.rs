use super::FileHandleBox;
use crate::boxes::file::errors::*;
use crate::runtime::provider_lock;

impl FileHandleBox {
    /// Open a file
    ///
    /// # Arguments
    ///
    /// - path: File path to open
    /// - mode: "r" (read), "w" (write/truncate), or "a" (append)
    ///
    /// # Errors
    ///
    /// - Already open: "FileHandleBox is already open. Call close() first."
    /// - Unsupported mode: "Unsupported mode: X. Use 'r', 'w', or 'a'"
    /// - NoFs profile: "File I/O disabled in no-fs profile. FileHandleBox is not available."
    /// - File not found (mode="r"): "File not found: PATH"
    ///
    /// # Design Notes
    ///
    /// - Phase 111: Supports "r", "w", and "a" modes
    /// - Each FileHandleBox instance gets its own FileIo (independent)
    pub fn open(&mut self, path: &str, mode: &str) -> Result<(), String> {
        // Fail-Fast: Check for double open
        if self.io.is_some() {
            return Err(already_open());
        }

        // Validate mode (Phase 111: "a" added)
        if mode != "r" && mode != "w" && mode != "a" {
            return Err(unsupported_mode(mode));
        }

        // Get FileIo provider to check capabilities
        let provider =
            provider_lock::get_filebox_provider().ok_or_else(provider_not_initialized)?;

        // NoFs profile check (Fail-Fast)
        let caps = provider.caps();
        if !caps.read && !caps.write {
            return Err(provider_disabled_in_nofs_profile());
        }

        // Mode-specific capability check (using helper)
        caps.check_mode(mode)?;

        use crate::runtime::get_global_ring0;
        let ring0 = get_global_ring0();

        // For write/append mode, create the file if it doesn't exist
        // Ring0FsFileIo expects the file to exist for open(), so we create it first
        if mode == "w" || mode == "a" {
            use std::path::Path;
            let path_obj = Path::new(path);
            if !ring0.fs.exists(path_obj) {
                // Create empty file for write/append mode
                ring0
                    .fs
                    .write_all(path_obj, &[])
                    .map_err(|e| format!("Failed to create file: {}", e))?;
            }
        }

        // Allocate via provider_lock SSOT (single route for provider instantiation).
        let io = provider_lock::new_filebox_provider_instance(Some(mode))?;

        // Now open the file with the new instance
        io.open(path)
            .map_err(|e| format!("Failed to open file: {}", e))?;

        // Store state
        self.path = path.to_string();
        self.mode = mode.to_string();
        self.io = Some(io);

        Ok(())
    }

    /// Read file contents to string
    ///
    /// # Errors
    ///
    /// - Not open: "FileHandleBox is not open"
    /// - Read failed: "Read failed: ERROR"
    pub fn read_to_string(&self) -> Result<String, String> {
        self.io
            .as_ref()
            .ok_or_else(not_open)?
            .read()
            .map_err(|e| format!("Read failed: {}", e))
    }

    /// Write content to file
    ///
    /// # Errors
    ///
    /// - Not open: "FileHandleBox is not open"
    /// - Wrong mode: "FileHandleBox opened in read mode"
    /// - Write failed: "Write failed: ERROR"
    ///
    /// # Phase 111
    ///
    /// Supports both "w" (truncate) and "a" (append) modes.
    /// Mode "r" returns error.
    pub fn write_all(&self, content: &str) -> Result<(), String> {
        // Fail-Fast: Check mode (Phase 111: allow "w" and "a")
        if self.mode == "r" {
            return Err("FileHandleBox is opened in read-only mode".to_string());
        }

        self.io
            .as_ref()
            .ok_or_else(not_open)?
            .write(content)
            .map_err(|e| format!("Write failed: {}", e))
    }

    /// Close the file
    ///
    /// # Errors
    ///
    /// - Not open: "FileHandleBox is not open"
    ///
    /// # Post-condition
    ///
    /// After close(), is_open() returns false and read/write will fail.
    pub fn close(&mut self) -> Result<(), String> {
        if self.io.is_none() {
            return Err(not_open());
        }

        // Drop the FileIo instance
        self.io.take();
        self.path.clear();
        self.mode.clear();

        Ok(())
    }

    /// Check if file is currently open
    pub fn is_open(&self) -> bool {
        self.io.is_some()
    }
}
