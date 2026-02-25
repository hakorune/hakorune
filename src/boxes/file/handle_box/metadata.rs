use super::FileHandleBox;

impl FileHandleBox {
    // ===== Phase 111/114: Metadata methods (internal Rust API) =====

    /// Phase 114: Internal helper using FileIo::stat()
    ///
    /// Unified metadata access through FileIo trait.
    pub(super) fn metadata_internal(&self) -> Result<crate::boxes::file::provider::FileStat, String> {
        let io = self
            .io
            .as_ref()
            .ok_or_else(|| "FileHandleBox is not open".to_string())?;

        io.stat().map_err(|e| format!("Metadata failed: {}", e))
    }

    /// Get file size in bytes
    ///
    /// # Errors
    ///
    /// - Not open: "FileHandleBox is not open"
    /// - Metadata failed: "Metadata failed: ERROR"
    pub fn size(&self) -> Result<u64, String> {
        self.metadata_internal().map(|meta| meta.size)
    }

    /// Check if file exists
    ///
    /// # Errors
    ///
    /// - Not open: "FileHandleBox is not open"
    ///
    /// # Note
    ///
    /// Uses FileIo::exists() for direct check.
    pub fn exists(&self) -> Result<bool, String> {
        let io = self
            .io
            .as_ref()
            .ok_or_else(|| "FileHandleBox is not open".to_string())?;

        Ok(io.exists())
    }

    /// Check if path is a file
    ///
    /// # Errors
    ///
    /// - Not open: "FileHandleBox is not open"
    /// - Metadata failed: "Metadata failed: ERROR"
    pub fn is_file(&self) -> Result<bool, String> {
        self.metadata_internal().map(|meta| meta.is_file)
    }

    /// Check if path is a directory
    ///
    /// # Errors
    ///
    /// - Not open: "FileHandleBox is not open"
    /// - Metadata failed: "Metadata failed: ERROR"
    pub fn is_dir(&self) -> Result<bool, String> {
        self.metadata_internal().map(|meta| meta.is_dir)
    }
}
