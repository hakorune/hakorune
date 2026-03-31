mod native;
mod p10;
#[cfg(test)]
mod tests;

#[allow(unused_imports)]
pub(crate) use native::{
    fold_i64_binop, match_main_return_i32_const, match_main_return_i32_const_binop,
    match_main_return_i32_const_via_copy, match_native_shape, NativeMatch, NativeShape,
};
#[allow(unused_imports)]
pub(crate) use p10::{
    detect_p10_fixed4_console_method_native_shape, detect_p10_loop_extern_call_candidate,
    detect_p10_min4_native_promotable_shape, detect_p10_min5_expansion_inventory_shape,
    detect_p10_min6_warn_native_promotable_shape, detect_p10_min7_info_native_promotable_shape,
    detect_p10_min8_error_native_promotable_shape, detect_p10_min9_debug_native_promotable_shape,
};
