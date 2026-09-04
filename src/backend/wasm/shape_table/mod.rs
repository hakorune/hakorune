mod native;
#[cfg(test)]
mod tests;

#[allow(unused_imports)]
pub(crate) use native::{
    fold_i64_binop, match_main_return_i32_const, match_main_return_i32_const_binop,
    match_main_return_i32_const_via_copy, match_native_shape, NativeMatch, NativeShape,
};
