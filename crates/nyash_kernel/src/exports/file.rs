// FileBox-related C ABI exports.

use nyash_rust::{boxes::file::FileBox, runtime::host_handles as handles};

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
