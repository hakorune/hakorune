use super::borrowed_handle::maybe_borrow_string_handle;
use nyash_rust::{
    box_trait::{IntegerBox, NyashBox, StringBox},
    runtime::host_handles as handles,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum CodecProfile {
    Generic,
    ArrayFastBorrowString,
}

pub(crate) fn int_arg_to_box(arg: i64) -> Box<dyn NyashBox> {
    Box::new(IntegerBox::new(arg))
}

pub(crate) fn any_arg_to_box_with_profile(arg: i64, profile: CodecProfile) -> Box<dyn NyashBox> {
    if arg > 0 {
        if let Some(obj) = handles::get(arg as u64) {
            if profile == CodecProfile::ArrayFastBorrowString
                && obj.as_any().downcast_ref::<StringBox>().is_some()
            {
                return maybe_borrow_string_handle(obj, arg);
            }
            return obj.clone_box();
        }
    }
    int_arg_to_box(arg)
}

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
