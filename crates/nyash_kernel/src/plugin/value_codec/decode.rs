use super::borrowed_handle::{maybe_borrow_string_handle, maybe_borrow_string_handle_with_epoch};
use nyash_rust::{
    config::env::ArrayFastValueDecodePolicyMode,
    box_trait::{IntegerBox, NyashBox, StringBox},
    runtime::host_handles as handles,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum CodecProfile {
    Generic,
    ArrayFastBorrowString,
    ArrayBorrowStringOnly,
}

pub(crate) enum ArrayFastDecodedValue {
    ImmediateI64(i64),
    Boxed(Box<dyn NyashBox>),
}

pub(crate) fn int_arg_to_box(arg: i64) -> Box<dyn NyashBox> {
    Box::new(IntegerBox::new(arg))
}

#[inline(always)]
pub(crate) fn decode_array_fast_value(arg: i64) -> ArrayFastDecodedValue {
    if arg <= 0 {
        return ArrayFastDecodedValue::ImmediateI64(arg);
    }
    let mode = nyash_rust::config::env::array_fast_value_decode_policy_mode();
    if !matches!(mode, ArrayFastValueDecodePolicyMode::ScalarPrefer) {
        return ArrayFastDecodedValue::Boxed(any_arg_to_box_with_profile(
            arg,
            CodecProfile::ArrayFastBorrowString,
        ));
    }
    handles::with_handle(arg as u64, |obj| {
        let Some(obj) = obj else {
            return ArrayFastDecodedValue::ImmediateI64(arg);
        };
        if obj.as_any().downcast_ref::<StringBox>().is_some() {
            return ArrayFastDecodedValue::Boxed(maybe_borrow_string_handle(obj.clone(), arg));
        }
        if let Some(ib) = obj.as_any().downcast_ref::<IntegerBox>() {
            return ArrayFastDecodedValue::ImmediateI64(ib.value);
        }
        ArrayFastDecodedValue::ImmediateI64(arg)
    })
}

#[inline(always)]
pub(crate) fn any_arg_to_box_with_profile(arg: i64, profile: CodecProfile) -> Box<dyn NyashBox> {
    if arg > 0 {
        if profile == CodecProfile::ArrayBorrowStringOnly {
            return handles::with_handle(arg as u64, |obj| {
                let Some(obj) = obj else {
                    return int_arg_to_box(arg);
                };
                if obj.as_any().downcast_ref::<StringBox>().is_some() {
                    return maybe_borrow_string_handle(obj.clone(), arg);
                }
                int_arg_to_box(arg)
            });
        }
        let scalar_prefer = profile == CodecProfile::ArrayFastBorrowString
            && matches!(
                nyash_rust::config::env::array_fast_value_decode_policy_mode(),
                ArrayFastValueDecodePolicyMode::ScalarPrefer
            );
        return handles::with_handle(arg as u64, |obj| {
            let Some(obj) = obj else {
                return int_arg_to_box(arg);
            };
            if profile == CodecProfile::ArrayFastBorrowString {
                if obj.as_any().downcast_ref::<StringBox>().is_some() {
                    return maybe_borrow_string_handle(obj.clone(), arg);
                }
                if scalar_prefer {
                    if let Some(ib) = obj.as_any().downcast_ref::<IntegerBox>() {
                        return Box::new(IntegerBox::new(ib.value));
                    }
                    return int_arg_to_box(arg);
                }
            }
            obj.clone_box()
        });
    }
    int_arg_to_box(arg)
}

#[inline(always)]
pub(crate) fn string_handle_or_immediate_box_from_obj(
    obj: Option<&std::sync::Arc<dyn NyashBox>>,
    source_handle: i64,
    source_drop_epoch: u64,
) -> Box<dyn NyashBox> {
    if source_handle <= 0 {
        return int_arg_to_box(source_handle);
    }
    let Some(obj) = obj else {
        return int_arg_to_box(source_handle);
    };
    if obj.as_any().downcast_ref::<StringBox>().is_some() {
        return maybe_borrow_string_handle_with_epoch(obj.clone(), source_handle, source_drop_epoch);
    }
    int_arg_to_box(source_handle)
}

#[inline(always)]
pub(crate) fn any_arg_to_box(arg: i64) -> Box<dyn NyashBox> {
    any_arg_to_box_with_profile(arg, CodecProfile::Generic)
}

pub(crate) fn any_arg_to_index(arg: i64) -> Option<i64> {
    if arg <= 0 {
        return Some(arg);
    }
    handles::with_handle(arg as u64, |obj| {
        let Some(obj) = obj else {
            return Some(arg);
        };
        // Treat integer-like handle keys as boxed indices, but keep positive immediates
        // as raw indices when the handle is non-index-like.
        if let Some(ib) = obj.as_any().downcast_ref::<IntegerBox>() {
            return Some(ib.value);
        }
        if let Some(sb) = obj.as_any().downcast_ref::<StringBox>() {
            return sb.value.parse::<i64>().ok().or(Some(arg));
        }
        Some(arg)
    })
}
