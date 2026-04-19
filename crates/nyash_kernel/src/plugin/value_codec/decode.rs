use super::borrowed_handle::maybe_borrow_string_handle;
use crate::plugin::value_demand::{
    DemandSet, CODEC_ARRAY_BORROW_STRING_ONLY, CODEC_ARRAY_FAST_BORROW_STRING, CODEC_GENERIC,
    CODEC_MAP_KEY_BORROW_STRING, CODEC_MAP_VALUE_BORROW_STRING,
};
use nyash_rust::{
    box_trait::{BoolBox, IntegerBox, NyashBox, StringBox},
    boxes::FloatBox,
    runtime::host_handles as handles,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum CodecProfile {
    Generic,
    ArrayFastBorrowString,
    ArrayBorrowStringOnly,
    MapKeyBorrowString,
    MapValueBorrowString,
}

impl CodecProfile {
    #[inline(always)]
    pub(crate) const fn demand(self) -> DemandSet {
        match self {
            Self::Generic => CODEC_GENERIC,
            Self::ArrayFastBorrowString => CODEC_ARRAY_FAST_BORROW_STRING,
            Self::ArrayBorrowStringOnly => CODEC_ARRAY_BORROW_STRING_ONLY,
            Self::MapKeyBorrowString => CODEC_MAP_KEY_BORROW_STRING,
            Self::MapValueBorrowString => CODEC_MAP_VALUE_BORROW_STRING,
        }
    }

    #[inline(always)]
    fn keeps_string_alias_and_prefers_scalar(self) -> bool {
        matches!(self, Self::ArrayFastBorrowString | Self::MapKeyBorrowString)
    }
}

// Internal-only carrier for array fast decode.
// Public ABI rows must use canonical value classes from docs, not this enum directly.
pub(crate) enum ArrayFastDecodedValue {
    ImmediateI64(i64),
    ImmediateBool(bool),
    ImmediateF64(f64),
    Boxed(Box<dyn NyashBox>),
}

pub(crate) fn int_arg_to_box(arg: i64) -> Box<dyn NyashBox> {
    Box::new(IntegerBox::new(arg))
}

#[inline(always)]
pub(crate) fn decode_array_fast_value(arg: i64) -> ArrayFastDecodedValue {
    let _demand = CodecProfile::ArrayFastBorrowString.demand();
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
            if let Some(bb) = obj.as_any().downcast_ref::<BoolBox>() {
                return ArrayFastDecodedValue::ImmediateBool(bb.value);
            }
            if let Some(fb) = obj.as_any().downcast_ref::<FloatBox>() {
                return ArrayFastDecodedValue::ImmediateF64(fb.value);
            }
            ArrayFastDecodedValue::ImmediateI64(arg)
        },
    )
}

#[inline(always)]
pub(crate) fn any_arg_to_box_with_profile(arg: i64, profile: CodecProfile) -> Box<dyn NyashBox> {
    let _demand = profile.demand();
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
                },
            );
        }
        if profile == CodecProfile::MapValueBorrowString {
            return handles::with_handle_caller(
                arg as u64,
                handles::PerfObserveObjectWithHandleCaller::DecodeAnyArg,
                |obj| {
                    let Some(obj) = obj else {
                        return int_arg_to_box(arg);
                    };
                    if obj.as_any().downcast_ref::<StringBox>().is_some()
                        || obj
                            .as_any()
                            .downcast_ref::<crate::exports::string_view::StringViewBox>()
                            .is_some()
                    {
                        return maybe_borrow_string_handle(obj.clone(), arg);
                    }
                    obj.clone_box()
                },
            );
        }
        // Phase-29cc route lock: map keys intentionally share the scalar-prefer
        // string-alias contract, but keep their own profile name at the call site.
        let scalar_prefer = profile.keeps_string_alias_and_prefers_scalar();
        return handles::with_handle_caller(
            arg as u64,
            handles::PerfObserveObjectWithHandleCaller::DecodeAnyArg,
            |obj| {
                let Some(obj) = obj else {
                    return int_arg_to_box(arg);
                };
                if scalar_prefer {
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
                    if let Some(fb) = obj.as_any().downcast_ref::<FloatBox>() {
                        return Box::new(FloatBox::new(fb.value));
                    }
                    if let Some(ib) = obj.as_any().downcast_ref::<IntegerBox>() {
                        return Box::new(IntegerBox::new(ib.value));
                    }
                    return int_arg_to_box(arg);
                }
                obj.clone_box()
            },
        );
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
        },
    )
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
