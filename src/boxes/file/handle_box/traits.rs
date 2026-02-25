use super::FileHandleBox;
use crate::box_trait::{BoolBox, BoxCore, NyashBox, StringBox};
use std::any::Any;

impl BoxCore for FileHandleBox {
    fn box_id(&self) -> u64 {
        self.base.id
    }

    fn parent_type_id(&self) -> Option<std::any::TypeId> {
        self.base.parent_type_id
    }

    fn fmt_box(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "FileHandleBox(path={}, mode={}, open={})",
            if self.path.is_empty() {
                "<none>"
            } else {
                &self.path
            },
            if self.mode.is_empty() {
                "<none>"
            } else {
                &self.mode
            },
            self.is_open()
        )
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl NyashBox for FileHandleBox {
    fn clone_box(&self) -> Box<dyn NyashBox> {
        Box::new(self.clone())
    }

    fn share_box(&self) -> Box<dyn NyashBox> {
        self.clone_box()
    }

    fn to_string_box(&self) -> StringBox {
        StringBox::new(format!(
            "FileHandleBox(path={}, mode={}, open={})",
            self.path,
            self.mode,
            self.is_open()
        ))
    }

    fn type_name(&self) -> &'static str {
        "FileHandleBox"
    }

    fn equals(&self, other: &dyn NyashBox) -> BoolBox {
        if let Some(other_handle) = other.as_any().downcast_ref::<FileHandleBox>() {
            // Equality: Same path and mode
            // Note: Two independent handles to same file are equal if path/mode match
            BoolBox::new(self.path == other_handle.path && self.mode == other_handle.mode)
        } else {
            BoolBox::new(false)
        }
    }
}

impl std::fmt::Display for FileHandleBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_box(f)
    }
}
