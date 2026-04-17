mod borrowed_handle;
mod decode;
mod encode;
mod string_classify;
mod string_materialize;
mod string_store;

#[cfg(test)]
pub(crate) use borrowed_handle::SourceLifetimeKeep;
pub(crate) use borrowed_handle::{
    try_retarget_borrowed_string_slot_take_unpublished_keep,
    try_retarget_borrowed_string_slot_take_verified_text_source, BorrowedHandleBox,
};
pub(crate) use decode::{
    any_arg_to_box, any_arg_to_box_with_profile, any_arg_to_index, decode_array_fast_value,
    int_arg_to_box, owned_string_from_handle, ArrayFastDecodedValue, CodecProfile,
};
#[cfg(test)]
pub(crate) use encode::box_to_runtime_i64;
pub(crate) use encode::{
    box_to_handle, runtime_i64_from_box_ref_caller, BorrowedAliasEncodeCaller,
};
pub(crate) use string_classify::{
    with_array_store_str_source, ArrayStoreStrSource, StringHandleSourceKind, StringLikeProof,
};
pub use string_materialize::KernelTextSlot;
pub(crate) use string_materialize::{
    freeze_owned_string_into_slot, issue_fresh_handle_from_arc, materialize_owned_string,
    materialize_owned_string_generic_fallback, publish_kernel_text_slot,
    with_kernel_text_slot_text, KernelTextSlotState,
};
#[cfg(test)]
pub(crate) use string_store::store_string_box_from_source;
#[cfg(test)]
pub(crate) use string_store::store_string_box_from_source_keep;
#[cfg(test)]
pub(crate) use string_store::store_string_box_from_source_keep_owned;
pub(crate) use string_store::{
    maybe_store_non_string_box_from_verified_source, store_string_box_from_kernel_text_slot,
    store_string_keep_from_kernel_text_slot,
    store_string_box_from_verified_text_source,
};

#[cfg(test)]
mod tests;
