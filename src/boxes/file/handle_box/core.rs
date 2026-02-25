use super::FileHandleBox;
use crate::box_trait::BoxBase;

impl std::fmt::Debug for FileHandleBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FileHandleBox")
            .field("path", &self.path)
            .field("mode", &self.mode)
            .field("is_open", &self.is_open())
            .finish()
    }
}

impl Clone for FileHandleBox {
    fn clone(&self) -> Self {
        // Clone creates a new independent handle (not open)
        // Design decision: Cloning an open handle creates a closed handle
        // Rationale: Prevents accidental file handle leaks
        FileHandleBox {
            base: BoxBase::new(),
            path: String::new(),
            mode: String::new(),
            io: None,
        }
    }
}

impl FileHandleBox {
    /// Create new FileHandleBox (file not yet opened)
    pub fn new() -> Self {
        FileHandleBox {
            base: BoxBase::new(),
            path: String::new(),
            mode: String::new(),
            io: None,
        }
    }
}
