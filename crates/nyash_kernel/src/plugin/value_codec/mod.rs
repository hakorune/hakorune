mod borrowed_handle;
mod decode;
mod encode;

pub(crate) use decode::{
    any_arg_to_box, any_arg_to_box_with_profile, any_arg_to_index, int_arg_to_box, CodecProfile,
};
pub(crate) use encode::{
    bool_box_to_i64, box_to_handle, box_to_runtime_i64, integer_box_to_i64, runtime_i64_from_box_ref,
};

#[cfg(test)]
mod tests;
