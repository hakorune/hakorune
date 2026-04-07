use super::borrowed_handle::maybe_borrow_string_handle;
use nyash_rust::{
    box_trait::{BoolBox, IntegerBox, NyashBox, StringBox},
    runtime::host_handles as handles,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum CodecProfile {
    Generic,
    ArrayFastBorrowString,
    ArrayBorrowStringOnly,
}

// Internal-only carrier for array fast decode.
// Public ABI rows must use canonical value classes from docs, not this enum directly.
pub(crate) enum ArrayFastDecodedValue {
    ImmediateI64(i64),
    Boxed(Box<dyn NyashBox>),
}

pub(crate) fn int_arg_to_box(arg: i64) -> Box<dyn NyashBox> {
    Box::new(IntegerBox::new(arg))
}

#[inline(always)]
pub(crate) fn decode_array_fast_value(arg: i64) -> ArrayFastDecodedValue {
    // String/StringView handles become borrowed string aliases.
    // Other positive handles stay conservative and fall back to immediate-style handling.
    if arg <= 0 {
        return ArrayFastDecodedValue::ImmediateI64(arg);
    }
    handles::with_handle_caller(
        arg as u64,
        handles::PerfObserveObjectWithHandleCaller::DecodeArrayFast,
        |obj| {
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
            return handles::with_handle_caller(
                arg as u64,
                handles::PerfObserveObjectWithHandleCaller::DecodeAnyArg,
                |obj| {
                let Some(obj) = obj else {
                    return int_arg_to_box(arg);
                };
                if obj.as_any().downcast_ref::<StringBox>().is_some() {
                    return maybe_borrow_string_handle(obj.clone(), arg);
                }
                int_arg_to_box(arg)
            });
        }
        // Phase-29cc route lock: ArrayFastBorrowString keeps scalar-prefer behavior.
        let scalar_prefer = profile == CodecProfile::ArrayFastBorrowString;
        return handles::with_handle_caller(
            arg as u64,
            handles::PerfObserveObjectWithHandleCaller::DecodeAnyArg,
            |obj| {
            let Some(obj) = obj else {
                return int_arg_to_box(arg);
            };
            if profile == CodecProfile::ArrayFastBorrowString {
                if obj.as_any().downcast_ref::<StringBox>().is_some()
                    || obj
                        .as_any()
                        .downcast_ref::<crate::exports::string_view::StringViewBox>()
                        .is_some()
                {
                    return maybe_borrow_string_handle(obj.clone(), arg);
                }
                if let Some(bb) = obj.as_any().downcast_ref::<BoolBox>() {
                    return Box::new(BoolBox::new(bb.value));
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
pub(crate) fn any_arg_to_box(arg: i64) -> Box<dyn NyashBox> {
    any_arg_to_box_with_profile(arg, CodecProfile::Generic)
}

pub(crate) fn any_arg_to_index(arg: i64) -> Option<i64> {
    if arg <= 0 {
        return Some(arg);
    }
    handles::with_handle_caller(
        arg as u64,
        handles::PerfObserveObjectWithHandleCaller::DecodeAnyIndex,
        |obj| {
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

#[inline(always)]
pub(crate) fn owned_string_from_handle(handle: i64) -> Option<String> {
    if handle <= 0 {
        return None;
    }
    if let Some(text) = handles::with_str_handle(handle as u64, str::to_owned) {
        return Some(text);
    }
    handles::with_handle_caller(
        handle as u64,
        handles::PerfObserveObjectWithHandleCaller::Generic,
        |obj| {
            let obj = obj?;
            Some(obj.to_string_box().value)
        },
    )
}
