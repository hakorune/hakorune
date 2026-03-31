use super::value_codec::{any_arg_to_box_with_profile, CodecProfile};
use std::cell::RefCell;
use nyash_rust::runtime::host_handles as handles;

#[allow(dead_code)]
struct ImmediateKeyCache {
    key: i64,
    text: String,
}

thread_local! {
    static IMMEDIATE_KEY_CACHE: RefCell<Option<ImmediateKeyCache>> = RefCell::new(None);
}

#[inline(always)]
pub(crate) fn map_key_string_from_i64(key_i64: i64) -> String {
    key_i64.to_string()
}

#[inline(always)]
pub(crate) fn map_key_string_from_any(key_any: i64) -> String {
    any_arg_to_box_with_profile(key_any, CodecProfile::ArrayFastBorrowString)
        .to_string_box()
        .value
}

#[allow(dead_code)]
#[inline(always)]
pub(crate) fn map_key_with_any_str_ref<T, F>(key_any: i64, f: F) -> T
where
    F: FnOnce(&str) -> T,
{
    if key_any > 0 {
        return handles::with_handle(key_any as u64, |obj| {
            let Some(obj) = obj else {
                let key_str = key_any.to_string();
                return f(&key_str);
            };
            if let Some(key_str) = obj.as_ref().as_str_fast() {
                return f(key_str);
            }
            let key_str = obj.as_ref().to_string_box().value;
            f(&key_str)
        });
    }
    IMMEDIATE_KEY_CACHE.with(|slot| {
        let text = {
            let mut cache = slot.borrow_mut();
            if let Some(entry) = cache.as_ref() {
                if entry.key == key_any {
                    return f(entry.text.as_str());
                }
            }
            let text = key_any.to_string();
            *cache = Some(ImmediateKeyCache {
                key: key_any,
                text,
            });
            cache
                .as_ref()
                .expect("immediate key cache just stored")
                .text
                .clone()
        };
        f(text.as_str())
    })
}
