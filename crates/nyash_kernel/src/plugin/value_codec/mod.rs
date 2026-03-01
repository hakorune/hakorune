mod borrowed_handle;
mod decode;
mod encode;

pub(crate) use borrowed_handle::{
    try_retarget_borrowed_string_slot, try_retarget_borrowed_string_slot_with_source,
};
pub(crate) use decode::{
    any_arg_to_box, any_arg_to_box_with_profile, any_arg_to_index, decode_array_fast_value,
    int_arg_to_box, string_handle_or_immediate_box_from_obj, ArrayFastDecodedValue, CodecProfile,
};
pub(crate) use encode::{
    bool_box_to_i64, box_to_handle, box_to_runtime_i64, integer_box_to_i64, runtime_i64_from_box_ref,
};

#[cfg(test)]
mod tests;
