use crate::observe;
use nyash_rust::{
    box_trait::{NyashBox, StringBox},
    runtime::host_handles as handles,
};
use std::sync::Arc;

mod backing;
mod box_impl;

pub(crate) use backing::SourceLifetimeKeep;
pub(crate) use box_impl::{
    runtime_i64_from_borrowed_alias, BorrowedAliasEncodeCaller, BorrowedHandleBox,
};

#[inline(always)]
pub(crate) fn maybe_borrow_string_handle(
    obj: Arc<dyn NyashBox>,
    source_handle: i64,
) -> Box<dyn NyashBox> {
    if obj.as_any().downcast_ref::<StringBox>().is_some() {
        return Box::new(BorrowedHandleBox::new(
            SourceLifetimeKeep::string_box(obj),
            source_handle,
            handles::drop_epoch(),
        ));
    }
    if obj
        .as_any()
        .downcast_ref::<crate::exports::string_view::StringViewBox>()
        .is_some()
    {
        observe::record_birth_backend_publish_reason_need_stable_object();
        observe::record_birth_backend_carrier_kind_stable_box();
        return Box::new(StringBox::new(
            SourceLifetimeKeep::string_view(obj)
                .backing
                .stable_box
                .as_ref()
                .to_string_box()
                .value,
        ));
    }
    obj.clone_box()
}

#[cfg(test)]
pub(crate) mod test_support;
