mod borrowed_handle;
mod decode;
mod encode;
mod string_classify;
mod string_materialize;
mod string_store;
mod text_carrier;

#[cfg(test)]
pub(crate) use borrowed_handle::maybe_borrow_string_keep_with_epoch;
#[cfg(test)]
pub(crate) use borrowed_handle::SourceLifetimeKeep;
pub(crate) use borrowed_handle::{
    try_retarget_borrowed_string_slot_take_verified_text_source, BorrowedHandleBox,
};
pub(crate) use decode::{
    any_arg_to_box, any_arg_to_box_with_profile, any_arg_to_index, decode_array_fast_value,
    owned_string_from_handle, ArrayFastDecodedValue, CodecProfile,
};
#[cfg(test)]
pub(crate) use encode::box_to_runtime_i64;
#[cfg(test)]
pub(crate) use encode::runtime_i64_from_box_ref;
pub(crate) use encode::{
    box_to_handle_materializing_borrowed_string, runtime_i64_from_box_ref_caller,
    runtime_i64_from_scalar_checked_box_ref_caller, BorrowedAliasEncodeCaller,
};
pub(crate) use string_classify::{
    with_array_store_str_source, ArrayStoreStrSource, StringHandleSourceKind, StringLikeProof,
};
#[cfg(test)]
pub(crate) use string_materialize::objectize_kernel_text_slot_stable_box;
pub use string_materialize::KernelTextSlot;
pub(crate) use string_materialize::{
    freeze_owned_bytes_with_site, freeze_owned_string_into_slot, issue_fresh_handle_from_arc,
    materialize_owned_string, materialize_owned_string_explicit_api_boundary_for_site,
    materialize_owned_string_generic_fallback, materialize_owned_string_generic_fallback_for_site,
    materialize_owned_string_need_stable_object_boundary_for_site,
    publish_kernel_text_slot, publish_owned_bytes_generic_fallback_boundary_for_site,
    with_kernel_text_slot_text, KernelTextSlotState, StringPublishSite,
};
#[cfg(test)]
pub(crate) use string_store::store_string_box_from_source;
#[cfg(test)]
pub(crate) use string_store::store_string_box_from_source_keep;
#[cfg(test)]
pub(crate) use string_store::store_string_box_from_source_keep_owned;
pub(crate) use string_store::{
    maybe_store_non_string_box_from_verified_source, store_string_box_from_verified_text_source,
};
pub(crate) use text_carrier::{OwnedText, TextRef};

#[cfg(test)]
mod tests;
