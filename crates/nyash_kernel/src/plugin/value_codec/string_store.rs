#[cfg(test)]
use super::borrowed_handle::maybe_borrow_string_handle;
#[cfg(test)]
use super::decode::int_arg_to_box;
#[cfg(test)]
use super::string_classify::classify_string_like_proof;
#[cfg(test)]
use nyash_rust::box_trait::NyashBox;
#[cfg(test)]
use std::sync::Arc;

#[inline(always)]
#[cfg(test)]
pub(crate) fn store_string_box_from_source(
    source_handle: i64,
    source_obj: Option<&Arc<dyn NyashBox>>,
    _source_drop_epoch: u64,
) -> Box<dyn NyashBox> {
    if source_handle <= 0 {
        return int_arg_to_box(source_handle);
    }
    let Some(obj) = source_obj else {
        return int_arg_to_box(source_handle);
    };
    if classify_string_like_proof(source_obj).is_some() {
        crate::observe::record_birth_placement_store_from_source();
        return maybe_borrow_string_handle(obj.clone(), source_handle);
    }
    int_arg_to_box(source_handle)
}
