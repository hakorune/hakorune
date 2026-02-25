use nyash_rust::{
    box_trait::{BoolBox, BoxBase, BoxCore, NyashBox, StringBox},
    runtime::host_handles as handles,
};
use std::{any::Any, sync::Arc};

#[derive(Debug, Clone)]
pub(crate) struct BorrowedHandleBox {
    pub(crate) inner: Arc<dyn NyashBox>,
    pub(crate) source_handle: i64,
    pub(crate) source_drop_epoch: u64,
    base: BoxBase,
}

impl BorrowedHandleBox {
    pub(crate) fn new(
        inner: Arc<dyn NyashBox>,
        source_handle: i64,
        source_drop_epoch: u64,
    ) -> Self {
        Self {
            inner,
            source_handle,
            source_drop_epoch,
            base: BoxBase::new(),
        }
    }
}

impl BoxCore for BorrowedHandleBox {
    fn box_id(&self) -> u64 {
        self.base.id
    }

    fn parent_type_id(&self) -> Option<std::any::TypeId> {
        self.base.parent_type_id
    }

    fn fmt_box(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.inner.fmt_box(f)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl NyashBox for BorrowedHandleBox {
    fn to_string_box(&self) -> StringBox {
        self.inner.to_string_box()
    }

    fn equals(&self, other: &dyn NyashBox) -> BoolBox {
        if let Some(other_alias) = other.as_any().downcast_ref::<BorrowedHandleBox>() {
            return self.inner.equals(other_alias.inner.as_ref());
        }
        self.inner.equals(other)
    }

    fn type_name(&self) -> &'static str {
        self.inner.type_name()
    }

    fn clone_box(&self) -> Box<dyn NyashBox> {
        Box::new(Self::new(
            self.inner.clone(),
            self.source_handle,
            self.source_drop_epoch,
        ))
    }

    fn share_box(&self) -> Box<dyn NyashBox> {
        self.clone_box()
    }

    fn is_identity(&self) -> bool {
        self.inner.is_identity()
    }

    fn borrowed_handle_source_fast(&self) -> Option<(i64, u64)> {
        if self.source_handle > 0 {
            Some((self.source_handle, self.source_drop_epoch))
        } else {
            None
        }
    }

    fn as_str_fast(&self) -> Option<&str> {
        self.inner.as_str_fast()
    }
}

pub(crate) fn maybe_borrow_string_handle(
    obj: Arc<dyn NyashBox>,
    source_handle: i64,
) -> Box<dyn NyashBox> {
    if obj.as_any().downcast_ref::<StringBox>().is_some() {
        return Box::new(BorrowedHandleBox::new(
            obj,
            source_handle,
            handles::drop_epoch(),
        ));
    }
    obj.clone_box()
}
