mod borrowed_handle;
mod decode;
mod encode;
mod string_classify;
mod string_materialize;
mod string_store;
mod text_carrier;

pub(crate) use borrowed_handle::test_support::{
    maybe_borrow_string_keep_with_epoch, try_retarget_borrowed_string_slot_take_keep,
};
#[cfg(test)]
pub(crate) use borrowed_handle::BorrowedHandleBox;
#[cfg(test)]
pub(crate) use borrowed_handle::SourceLifetimeKeep;
#[cfg(test)]
pub(crate) use decode::any_arg_to_box;
pub(crate) use decode::int_arg_to_box;
pub(crate) use decode::{
    any_arg_to_box_with_profile, any_arg_to_index, decode_array_fast_value,
    owned_string_from_handle, ArrayFastDecodedValue, CodecProfile,
};
#[cfg(test)]
pub(crate) use encode::runtime_i64_from_box_ref;
pub(crate) use encode::{box_to_handle, runtime_i64_from_box_ref_caller};
pub(crate) use encode::{
    runtime_i64_from_scalar_checked_box_ref_caller, BorrowedAliasEncodeCaller,
};
pub(crate) use string_classify::{
    with_array_store_str_source, ArrayStoreStrSource, StringHandleSourceKind, StringLikeProof,
};
pub use string_materialize::KernelTextSlot;
pub(crate) use string_materialize::{
    freeze_owned_bytes_with_site, freeze_owned_string_into_slot, issue_fresh_handle,
    materialize_owned_string, publish_kernel_text_slot, publish_owned_bytes_with_reason_and_site,
    with_kernel_text_slot_text, KernelTextSlotState, PublishReason, StringPublishSite,
};
#[cfg(test)]
pub(crate) use string_store::store_string_box_from_source;
pub(crate) use text_carrier::{OwnedText, TextRef};

#[cfg(test)]
mod tests;
