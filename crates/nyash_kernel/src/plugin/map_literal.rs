//! Explicit-kind Map literal runtime boundary; never infer kinds from raw bits.
//! Storage and live-handle clone policy remain in their existing owners.
use super::value_codec::map_value_from_live_object;
use nyash_rust::{
    box_trait::{BoolBox, IntegerBox, NyashBox, StringBox, VoidBox},
    boxes::{map_box::MapBox, FloatBox},
    runtime::host_handles as handles,
};

pub(crate) const NYRT_MAP_LITERAL_I64: u32 = 1;
pub(crate) const NYRT_MAP_LITERAL_BOOL: u32 = 2;
pub(crate) const NYRT_MAP_LITERAL_F64: u32 = 3;
pub(crate) const NYRT_MAP_LITERAL_VOID: u32 = 4;
pub(crate) const NYRT_MAP_LITERAL_HANDLE: u32 = 5;
pub(crate) const NYRT_MAP_LITERAL_OK: u32 = 0;
pub(crate) const NYRT_MAP_LITERAL_INVALID_CONTRACT: u32 = 2;

/// Borrow live map/key/value handles; invalid input never inserts a value.
#[export_name = "nyash.map.literal_store_v1"]
pub extern "C" fn nyash_map_literal_store_v1(map: i64, key: i64, kind: u32, payload: u64) -> u32 {
    match store(map, key, kind, payload) {
        Some(()) => NYRT_MAP_LITERAL_OK,
        None => NYRT_MAP_LITERAL_INVALID_CONTRACT,
    }
}

fn store(map: i64, key: i64, kind: u32, payload: u64) -> Option<()> {
    if map <= 0 || key <= 0 {
        return None;
    }
    // Arc keeps the receiver alive without holding a registry or Map lock.
    let receiver = handles::get(map as u64)?;
    let map = receiver.as_any().downcast_ref::<MapBox>()?;
    let key_object = handles::get(key as u64)?;
    let key = key_object
        .as_any()
        .downcast_ref::<StringBox>()?
        .value
        .clone();
    let value: Box<dyn NyashBox> = match kind {
        NYRT_MAP_LITERAL_I64 => Box::new(IntegerBox::new(payload as i64)),
        NYRT_MAP_LITERAL_BOOL if payload <= 1 => Box::new(BoolBox::new(payload != 0)),
        NYRT_MAP_LITERAL_F64 => Box::new(FloatBox::new(f64::from_bits(payload))),
        NYRT_MAP_LITERAL_VOID if payload == 0 => Box::new(VoidBox::new()),
        NYRT_MAP_LITERAL_HANDLE if payload > 0 && payload <= i64::MAX as u64 => {
            let object = handles::get(payload)?;
            map_value_from_live_object(&object, payload as i64)
        }
        _ => return None,
    };
    // Decode/clone completes before acquiring the Map write lock, including self-store.
    map.insert_key_str(key, value);
    Some(())
}

#[cfg(test)]
#[path = "map_literal_tests.rs"]
mod tests;
