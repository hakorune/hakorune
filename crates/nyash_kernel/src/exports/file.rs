// FileBox-related C ABI exports.

use crate::plugin::materialize_owned_string;
use nyash_rust::{
    box_trait::NyashBox, boxes::array::ArrayBox, boxes::file::FileBox,
    runtime::host_handles as handles,
};
use std::sync::Arc;

fn decode_handle_to_string_like(handle: i64) -> Option<String> {
    if handle <= 0 {
        return None;
    }
    let object = handles::get(handle as u64)?;
    if let Some(string_box) = object
        .as_any()
        .downcast_ref::<nyash_rust::box_trait::StringBox>()
    {
        return Some(string_box.value.clone());
    }
    Some(object.to_string_box().value)
}

fn array_handle_from_bytes(bytes: Vec<u8>) -> i64 {
    let arr = ArrayBox::new();
    for byte in bytes {
        arr.push(Box::new(nyash_rust::box_trait::IntegerBox::new(
            byte as i64,
        )));
    }
    let arc: Arc<dyn NyashBox> = Arc::new(arr);
    handles::to_handle_arc(arc) as i64
}

#[export_name = "nyash.file.open_hhh"]
pub extern "C" fn nyash_file_open_hhh_export(
    recv_handle: i64,
    path_handle: i64,
    mode_handle: i64,
) -> i64 {
    if recv_handle <= 0 {
        return 0;
    }
    let object = match handles::get(recv_handle as u64) {
        Some(object) => object,
        None => return 0,
    };
    let file_box = match object.as_any().downcast_ref::<FileBox>() {
        Some(file_box) => file_box,
        None => return 0,
    };
    let path = match decode_handle_to_string_like(path_handle) {
        Some(path) => path,
        None => return 0,
    };
    let mode = match decode_handle_to_string_like(mode_handle) {
        Some(mode) => mode,
        None => return 0,
    };
    if file_box.ny_open(&path, &mode).is_ok() {
        1
    } else {
        0
    }
}

#[export_name = "nyash.file.read_h"]
pub extern "C" fn nyash_file_read_h_export(recv_handle: i64) -> i64 {
    if recv_handle <= 0 {
        return 0;
    }
    let object = match handles::get(recv_handle as u64) {
        Some(object) => object,
        None => return 0,
    };
    let file_box = match object.as_any().downcast_ref::<FileBox>() {
        Some(file_box) => file_box,
        None => return 0,
    };
    match file_box.ny_read_to_string() {
        Ok(text) => materialize_owned_string(text),
        Err(_) => 0,
    }
}

#[export_name = "nyash.file.close_h"]
pub extern "C" fn nyash_file_close_h_export(recv_handle: i64) -> i64 {
    if recv_handle <= 0 {
        return 0;
    }
    let object = match handles::get(recv_handle as u64) {
        Some(object) => object,
        None => return 0,
    };
    let file_box = match object.as_any().downcast_ref::<FileBox>() {
        Some(file_box) => file_box,
        None => return 0,
    };
    let _ = file_box.ny_close();
    0
}

#[export_name = "nyash.file.read_bytes_h"]
pub extern "C" fn nyash_file_read_bytes_h_export(recv_handle: i64) -> i64 {
    if recv_handle <= 0 {
        return 0;
    }
    let object = match handles::get(recv_handle as u64) {
        Some(object) => object,
        None => return 0,
    };
    let file_box = match object.as_any().downcast_ref::<FileBox>() {
        Some(file_box) => file_box,
        None => return 0,
    };
    match file_box.ny_read_bytes() {
        Ok(bytes) => array_handle_from_bytes(bytes),
        Err(_) => 0,
    }
}
