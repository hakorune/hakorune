mod borrowed_handle;
mod decode;
mod encode;
mod string_classify;
mod string_store;

pub(crate) use borrowed_handle::{
    try_retarget_borrowed_string_slot_take_keep, BorrowedHandleBox, SourceLifetimeKeep,
};
pub(crate) use decode::{
    any_arg_to_box, any_arg_to_box_with_profile, any_arg_to_index, decode_array_fast_value,
    int_arg_to_box, owned_string_from_handle, ArrayFastDecodedValue, CodecProfile,
};
pub(crate) use encode::{
    box_to_handle, box_to_runtime_i64, runtime_i64_from_box_ref,
    runtime_i64_from_box_ref_caller, BorrowedAliasEncodeCaller,
};
pub(crate) use string_classify::{
    with_array_store_str_source, ArrayStoreStrSource, StringHandleSourceKind,
};
pub(crate) use string_store::{
    materialize_owned_string, maybe_store_non_string_box_from_verified_source,
    store_string_box_from_source, store_string_box_from_source_keep,
};

#[cfg(test)]
mod tests;
