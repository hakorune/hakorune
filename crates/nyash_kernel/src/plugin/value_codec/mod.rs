mod borrowed_handle;
mod decode;
mod encode;
mod string_store;

pub(crate) use borrowed_handle::try_retarget_borrowed_string_slot_with_source;
pub(crate) use decode::{
    any_arg_to_box, any_arg_to_box_with_profile, any_arg_to_index, decode_array_fast_value,
    int_arg_to_box, ArrayFastDecodedValue, CodecProfile,
};
pub(crate) use encode::{box_to_handle, box_to_runtime_i64, runtime_i64_from_box_ref};
pub(crate) use string_store::{
    is_string_handle_source, materialize_owned_string, store_string_box_from_source,
    store_string_box_from_string_source,
};

#[cfg(test)]
mod tests;
