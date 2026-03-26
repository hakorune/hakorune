use super::*;
use crate::test_support::{with_env_var, with_env_vars};
use nyash_rust::{
    box_trait::{NyashBox, StringBox},
    boxes::file::FileBox,
    instance_v2::InstanceBox,
    runtime::{host_handles as handles, plugin_loader_v2::make_plugin_box_v2},
    value::NyashValue,
};
use std::ffi::CString;
use std::sync::Arc;

fn decode_string_like_handle(handle: i64) -> Option<String> {
    if handle <= 0 {
        return None;
    }
    let object = handles::get(handle as u64)?;
    if let Some(string_box) = object.as_any().downcast_ref::<StringBox>() {
        return Some(string_box.value.clone());
    }
    Some(object.to_string_box().value)
}

fn ensure_test_ring0() {
    let _ = nyash_rust::runtime::ring0::ensure_global_ring0_initialized();
}

fn with_filebox_from_handle<R>(handle: i64, f: impl FnOnce(&FileBox) -> R) -> R {
    let object = handles::get(handle as u64).expect("FileBox handle must exist");
    let filebox = object
        .as_any()
        .downcast_ref::<FileBox>()
        .expect("handle must be a FileBox");
    f(filebox)
}

fn string_handle(value: &str) -> i64 {
    handles::to_handle_arc(Arc::new(StringBox::new(value.to_string()))) as i64
}

fn instancebox_handle_with_field(field_name: &str, value: NyashValue) -> i64 {
    let instance = InstanceBox::from_declaration(
        "Point".to_string(),
        vec![field_name.to_string()],
        Default::default(),
    );
    instance
        .set_field_ng(field_name.to_string(), value)
        .expect("seed instance field");
    handles::to_handle_arc(Arc::new(instance)) as i64
}

fn dispatch_stage1_module(receiver_name: &str, method: &str, source_text: &str) -> i64 {
    let receiver_handle =
        handles::to_handle_arc(Arc::new(StringBox::new(receiver_name.to_string()))) as i64;
    let source_handle =
        handles::to_handle_arc(Arc::new(StringBox::new(source_text.to_string()))) as i64;
    crate::plugin::try_module_string_dispatch(receiver_handle, method, 1, source_handle, 0)
        .expect("stage1 direct dispatch")
}

unsafe extern "C" fn fake_i32(
    _t: u32,
    _m: u32,
    _i: u32,
    _a: *const u8,
    _al: usize,
    res: *mut u8,
    len: *mut usize,
) -> i32 {
    let mut buf = Vec::new();
    buf.extend_from_slice(&1u16.to_le_bytes());
    buf.extend_from_slice(&1u16.to_le_bytes());
    buf.push(2);
    buf.push(0);
    buf.extend_from_slice(&4u16.to_le_bytes());
    buf.extend_from_slice(&123i32.to_le_bytes());
    if res.is_null() || len.is_null() || unsafe { *len } < buf.len() {
        unsafe {
            if !len.is_null() {
                *len = buf.len();
            }
        }
        return -1;
    }
    unsafe {
        std::ptr::copy_nonoverlapping(buf.as_ptr(), res, buf.len());
        *len = buf.len();
    }
    0
}

unsafe extern "C" fn fake_str(
    _t: u32,
    _m: u32,
    _i: u32,
    _a: *const u8,
    _al: usize,
    res: *mut u8,
    len: *mut usize,
) -> i32 {
    let s = b"hi";
    let mut buf = Vec::new();
    buf.extend_from_slice(&1u16.to_le_bytes());
    buf.extend_from_slice(&1u16.to_le_bytes());
    buf.push(7);
    buf.push(0);
    buf.extend_from_slice(&(s.len() as u16).to_le_bytes());
    buf.extend_from_slice(s);
    if res.is_null() || len.is_null() || unsafe { *len } < buf.len() {
        unsafe {
            if !len.is_null() {
                *len = buf.len();
            }
        }
        return -1;
    }
    unsafe {
        std::ptr::copy_nonoverlapping(buf.as_ptr(), res, buf.len());
        *len = buf.len();
    }
    0
}

#[test]
fn decode_i32_and_string_returns() {
    let pb = make_plugin_box_v2("Dummy".into(), 1, 1, fake_i32);
    let arc: Arc<dyn NyashBox> = Arc::new(pb);
    let handle = handles::to_handle_arc(arc) as i64;
    let val = nyash_plugin_invoke3_tagged_i64(1, 0, 0, handle, 0, 0, 0, 0, 0, 0, 0, 0);
    assert_eq!(val, 123);

    let pb = make_plugin_box_v2("Dummy".into(), 1, 2, fake_str);
    let arc: Arc<dyn NyashBox> = Arc::new(pb);
    let handle = handles::to_handle_arc(arc) as i64;
    let h = nyash_plugin_invoke3_tagged_i64(1, 0, 0, handle, 0, 0, 0, 0, 0, 0, 0, 0);
    assert!(h > 0);
    let obj = handles::get(h as u64).unwrap();
    let sb = obj.as_any().downcast_ref::<StringBox>().unwrap();
    assert_eq!(sb.value, "hi");
}

#[test]
fn env_box_new_i64x_creates_array_box() {
    let type_name = CString::new("ArrayBox").expect("CString");
    let handle = nyash_env_box_new_i64x(type_name.as_ptr(), 0, 0, 0, 0, 0);
    assert!(handle > 0, "expected ArrayBox handle");
    let object = handles::get(handle as u64).expect("handle must exist");
    assert_eq!(object.type_name(), "ArrayBox");
}

#[test]
fn env_box_new_i64x_creates_file_box() {
    ensure_test_ring0();
    let type_name = CString::new("FileBox").expect("CString");
    let handle = nyash_env_box_new_i64x(type_name.as_ptr(), 0, 0, 0, 0, 0);
    assert!(handle > 0, "expected FileBox handle");
    let object = handles::get(handle as u64).expect("handle must exist");
    assert_eq!(object.type_name(), "FileBox");
}

#[test]
fn filebox_direct_open_read_roundtrip() {
    ensure_test_ring0();

    let tmp_path = "/tmp/nyash_kernel_filebox_read_roundtrip.txt";
    std::fs::write(tmp_path, "kernel-filebox-read").expect("seed file");

    let type_name = CString::new("FileBox").expect("CString");
    let filebox_handle = nyash_env_box_new_i64x(type_name.as_ptr(), 0, 0, 0, 0, 0);
    assert!(filebox_handle > 0, "expected FileBox handle");

    with_filebox_from_handle(filebox_handle, |filebox| {
        filebox
            .ny_open(tmp_path, "r")
            .expect("direct open should succeed");
        assert_eq!(
            filebox
                .read_to_string()
                .expect("direct read should succeed"),
            "kernel-filebox-read"
        );
        filebox.ny_close().expect("direct close should succeed");
    });

    let _ = std::fs::remove_file(tmp_path);
}

#[test]
fn filebox_direct_open_write_roundtrip() {
    ensure_test_ring0();

    let tmp_path = "/tmp/nyash_kernel_filebox_write_roundtrip.txt";
    let _ = std::fs::remove_file(tmp_path);

    let type_name = CString::new("FileBox").expect("CString");
    let filebox_handle = nyash_env_box_new_i64x(type_name.as_ptr(), 0, 0, 0, 0, 0);
    assert!(filebox_handle > 0, "expected FileBox handle");

    with_filebox_from_handle(filebox_handle, |filebox| {
        filebox
            .ny_open(tmp_path, "w")
            .expect("direct open should succeed");
        let write_result =
            filebox.write(Box::new(StringBox::new("kernel-filebox-write".to_string())));
        let write_result = write_result.to_string_box().value;
        assert_eq!(write_result, "OK");
        filebox.ny_close().expect("direct close should succeed");
    });

    let written = std::fs::read_to_string(tmp_path).expect("written file");
    assert_eq!(written, "kernel-filebox-write");
    let _ = std::fs::remove_file(tmp_path);
}

#[test]
fn filebox_open_hhh_succeeds_with_explicit_read_mode() {
    ensure_test_ring0();

    let tmp_path = "/tmp/nyash_kernel_filebox_open_hhh_read.txt";
    std::fs::write(tmp_path, "kernel-filebox-open-hhh-read").expect("seed file");

    let type_name = CString::new("FileBox").expect("CString");
    let filebox_handle = nyash_env_box_new_i64x(type_name.as_ptr(), 0, 0, 0, 0, 0);
    assert!(filebox_handle > 0, "expected FileBox handle");

    let path_handle = string_handle(tmp_path);
    let mode_handle = string_handle("r");
    assert_eq!(
        nyash_file_open_hhh_export(filebox_handle, path_handle, mode_handle),
        1
    );

    with_filebox_from_handle(filebox_handle, |filebox| {
        assert_eq!(
            filebox
                .read_to_string()
                .expect("direct read should succeed"),
            "kernel-filebox-open-hhh-read"
        );
        filebox.ny_close().expect("direct close should succeed");
    });

    let _ = std::fs::remove_file(tmp_path);
}

#[test]
fn filebox_open_hhh_succeeds_with_explicit_write_mode() {
    ensure_test_ring0();

    let tmp_path = "/tmp/nyash_kernel_filebox_open_hhh_write.txt";
    let _ = std::fs::remove_file(tmp_path);

    let type_name = CString::new("FileBox").expect("CString");
    let filebox_handle = nyash_env_box_new_i64x(type_name.as_ptr(), 0, 0, 0, 0, 0);
    assert!(filebox_handle > 0, "expected FileBox handle");

    let path_handle = string_handle(tmp_path);
    let mode_handle = string_handle("w");
    assert_eq!(
        nyash_file_open_hhh_export(filebox_handle, path_handle, mode_handle),
        1
    );

    with_filebox_from_handle(filebox_handle, |filebox| {
        let write_result = filebox.write(Box::new(StringBox::new(
            "kernel-filebox-open-hhh-write".to_string(),
        )));
        assert_eq!(write_result.to_string_box().value, "OK");
        filebox.ny_close().expect("direct close should succeed");
    });

    let written = std::fs::read_to_string(tmp_path).expect("written file");
    assert_eq!(written, "kernel-filebox-open-hhh-write");
    let _ = std::fs::remove_file(tmp_path);
}

#[test]
fn filebox_open_hhh_returns_zero_for_invalid_receiver() {
    ensure_test_ring0();
    let path_handle = string_handle("/tmp/nyash_kernel_filebox_open_hhh_invalid_receiver.txt");
    let mode_handle = string_handle("r");
    assert_eq!(nyash_file_open_hhh_export(0, path_handle, mode_handle), 0);
}

#[test]
fn filebox_open_hhh_returns_zero_for_invalid_string_handles() {
    ensure_test_ring0();

    let type_name = CString::new("FileBox").expect("CString");
    let filebox_handle = nyash_env_box_new_i64x(type_name.as_ptr(), 0, 0, 0, 0, 0);
    assert!(filebox_handle > 0, "expected FileBox handle");

    let path_handle = string_handle("/tmp/nyash_kernel_filebox_open_hhh_invalid_mode.txt");
    let mode_handle = string_handle("r");
    assert_eq!(nyash_file_open_hhh_export(filebox_handle, 0, mode_handle), 0);
    assert_eq!(nyash_file_open_hhh_export(filebox_handle, path_handle, 0), 0);
}

#[test]
fn filebox_read_h_returns_string_handle_after_open() {
    ensure_test_ring0();

    let tmp_path = "/tmp/nyash_kernel_filebox_read_h_roundtrip.txt";
    std::fs::write(tmp_path, "kernel-filebox-read-h").expect("seed file");

    let type_name = CString::new("FileBox").expect("CString");
    let filebox_handle = nyash_env_box_new_i64x(type_name.as_ptr(), 0, 0, 0, 0, 0);
    assert!(filebox_handle > 0, "expected FileBox handle");

    let path_handle = string_handle(tmp_path);
    let mode_handle = string_handle("r");
    assert_eq!(
        nyash_file_open_hhh_export(filebox_handle, path_handle, mode_handle),
        1
    );

    let read_handle = nyash_file_read_h_export(filebox_handle);
    assert!(read_handle > 0, "read_h should return StringBox handle");
    assert_eq!(
        decode_string_like_handle(read_handle).as_deref(),
        Some("kernel-filebox-read-h")
    );

    with_filebox_from_handle(filebox_handle, |filebox| {
        filebox.ny_close().expect("direct close should succeed");
    });
    let _ = std::fs::remove_file(tmp_path);
}

#[test]
fn filebox_read_h_returns_zero_for_invalid_receiver() {
    ensure_test_ring0();
    assert_eq!(nyash_file_read_h_export(0), 0);
}

#[test]
fn filebox_read_h_returns_zero_when_not_opened() {
    ensure_test_ring0();

    let type_name = CString::new("FileBox").expect("CString");
    let filebox_handle = nyash_env_box_new_i64x(type_name.as_ptr(), 0, 0, 0, 0, 0);
    assert!(filebox_handle > 0, "expected FileBox handle");
    assert_eq!(nyash_file_read_h_export(filebox_handle), 0);
}

#[test]
fn filebox_close_h_closes_open_file() {
    ensure_test_ring0();

    let tmp_path = "/tmp/nyash_kernel_filebox_close_h_roundtrip.txt";
    std::fs::write(tmp_path, "kernel-filebox-close-h").expect("seed file");

    let type_name = CString::new("FileBox").expect("CString");
    let filebox_handle = nyash_env_box_new_i64x(type_name.as_ptr(), 0, 0, 0, 0, 0);
    assert!(filebox_handle > 0, "expected FileBox handle");

    let path_handle = string_handle(tmp_path);
    let mode_handle = string_handle("r");
    assert_eq!(
        nyash_file_open_hhh_export(filebox_handle, path_handle, mode_handle),
        1
    );
    assert_eq!(nyash_file_close_h_export(filebox_handle), 0);
    assert_eq!(nyash_file_read_h_export(filebox_handle), 0);

    let _ = std::fs::remove_file(tmp_path);
}

#[test]
fn filebox_close_h_returns_zero_for_invalid_receiver() {
    ensure_test_ring0();
    assert_eq!(nyash_file_close_h_export(0), 0);
}

#[test]
fn filebox_read_bytes_h_returns_array_handle_after_open() {
    ensure_test_ring0();

    let tmp_path = "/tmp/nyash_kernel_filebox_read_bytes_h_roundtrip.bin";
    std::fs::write(tmp_path, [65u8, 66u8, 67u8]).expect("seed bytes");

    let type_name = CString::new("FileBox").expect("CString");
    let filebox_handle = nyash_env_box_new_i64x(type_name.as_ptr(), 0, 0, 0, 0, 0);
    assert!(filebox_handle > 0, "expected FileBox handle");

    let path_handle = string_handle(tmp_path);
    let mode_handle = string_handle("r");
    assert_eq!(
        nyash_file_open_hhh_export(filebox_handle, path_handle, mode_handle),
        1
    );

    let out_handle = nyash_file_read_bytes_h_export(filebox_handle);
    assert!(out_handle > 0, "read_bytes_h should return ArrayBox handle");
    let object = handles::get(out_handle as u64).expect("array handle");
    let array = object
        .as_any()
        .downcast_ref::<nyash_rust::boxes::array::ArrayBox>()
        .expect("read_bytes_h result must be ArrayBox");
    assert_eq!(array.len(), 3);
    assert_eq!(array.get_index_i64(0).to_string_box().value, "65");
    assert_eq!(array.get_index_i64(1).to_string_box().value, "66");
    assert_eq!(array.get_index_i64(2).to_string_box().value, "67");

    with_filebox_from_handle(filebox_handle, |filebox| {
        let _ = filebox.ny_close();
    });
    let _ = std::fs::remove_file(tmp_path);
}

#[test]
fn filebox_read_bytes_h_returns_zero_for_invalid_receiver() {
    ensure_test_ring0();
    assert_eq!(nyash_file_read_bytes_h_export(0), 0);
}

#[test]
fn filebox_by_name_open_is_retired() {
    ensure_test_ring0();

    with_env_var("NYASH_VM_USE_FALLBACK", "1", || {
        let type_name = CString::new("FileBox").expect("CString");
        let filebox_handle = nyash_env_box_new_i64x(type_name.as_ptr(), 0, 0, 0, 0, 0);
        assert!(filebox_handle > 0, "expected FileBox handle");

        let method = CString::new("open").expect("CString");
        let path_handle = string_handle("/tmp/nyash_kernel_filebox_by_name_open_retired.txt");
        let mode_handle = string_handle("r");
        let result =
            nyash_plugin_invoke_by_name_i64(filebox_handle, method.as_ptr(), 2, path_handle, mode_handle);
        assert_eq!(result, 0, "open should no longer use builtin by_name keep");
    });
}

#[test]
fn filebox_by_name_read_bytes_is_retired() {
    ensure_test_ring0();

    with_env_var("NYASH_VM_USE_FALLBACK", "1", || {
        let tmp_path = "/tmp/nyash_kernel_filebox_by_name_read_bytes_retired.bin";
        std::fs::write(tmp_path, [65u8, 66u8]).expect("seed bytes");

        let type_name = CString::new("FileBox").expect("CString");
        let filebox_handle = nyash_env_box_new_i64x(type_name.as_ptr(), 0, 0, 0, 0, 0);
        assert!(filebox_handle > 0, "expected FileBox handle");

        let path_handle = string_handle(tmp_path);
        let mode_handle = string_handle("r");
        assert_eq!(
            nyash_file_open_hhh_export(filebox_handle, path_handle, mode_handle),
            1
        );

        let method = CString::new("readBytes").expect("CString");
        let result = nyash_plugin_invoke_by_name_i64(filebox_handle, method.as_ptr(), 0, 0, 0);
        assert_eq!(result, 0, "readBytes should no longer use builtin by_name keep");

        with_filebox_from_handle(filebox_handle, |filebox| {
            let _ = filebox.ny_close();
        });
        let _ = std::fs::remove_file(tmp_path);
    });
}

#[test]
fn filebox_by_name_write_is_retired() {
    ensure_test_ring0();

    with_env_var("NYASH_VM_USE_FALLBACK", "1", || {
        let tmp_path = "/tmp/nyash_kernel_filebox_by_name_write_retired.txt";
        let _ = std::fs::remove_file(tmp_path);

        let type_name = CString::new("FileBox").expect("CString");
        let filebox_handle = nyash_env_box_new_i64x(type_name.as_ptr(), 0, 0, 0, 0, 0);
        assert!(filebox_handle > 0, "expected FileBox handle");

        let path_handle = string_handle(tmp_path);
        let mode_handle = string_handle("w");
        assert_eq!(
            nyash_file_open_hhh_export(filebox_handle, path_handle, mode_handle),
            1
        );

        let method = CString::new("write").expect("CString");
        let data_handle = string_handle("kernel-filebox-by-name-write");
        let result =
            nyash_plugin_invoke_by_name_i64(filebox_handle, method.as_ptr(), 1, data_handle, 0);
        assert_eq!(result, 0, "write should no longer use builtin by_name keep");

        with_filebox_from_handle(filebox_handle, |filebox| {
            let _ = filebox.ny_close();
        });
        let _ = std::fs::remove_file(tmp_path);
    });
}

#[test]
fn filebox_by_name_close_is_retired() {
    ensure_test_ring0();

    with_env_var("NYASH_VM_USE_FALLBACK", "1", || {
        let tmp_path = "/tmp/nyash_kernel_filebox_by_name_close_retired.txt";
        std::fs::write(tmp_path, "kernel-filebox-by-name-close").expect("seed file");

        let type_name = CString::new("FileBox").expect("CString");
        let filebox_handle = nyash_env_box_new_i64x(type_name.as_ptr(), 0, 0, 0, 0, 0);
        assert!(filebox_handle > 0, "expected FileBox handle");

        let path_handle = string_handle(tmp_path);
        let mode_handle = string_handle("r");
        assert_eq!(
            nyash_file_open_hhh_export(filebox_handle, path_handle, mode_handle),
            1
        );

        let method = CString::new("close").expect("CString");
        let result = nyash_plugin_invoke_by_name_i64(filebox_handle, method.as_ptr(), 0, 0, 0);
        assert_eq!(result, 0, "close should no longer use builtin by_name keep");

        let _ = std::fs::remove_file(tmp_path);
    });
}

#[test]
fn filebox_by_name_read_is_retired() {
    ensure_test_ring0();

    with_env_var("NYASH_VM_USE_FALLBACK", "1", || {
        let tmp_path = "/tmp/nyash_kernel_filebox_by_name_read_retired.txt";
        std::fs::write(tmp_path, "kernel-filebox-by-name-read").expect("seed file");

        let type_name = CString::new("FileBox").expect("CString");
        let filebox_handle = nyash_env_box_new_i64x(type_name.as_ptr(), 0, 0, 0, 0, 0);
        assert!(filebox_handle > 0, "expected FileBox handle");

        let path_handle = string_handle(tmp_path);
        let mode_handle = string_handle("r");
        assert_eq!(
            nyash_file_open_hhh_export(filebox_handle, path_handle, mode_handle),
            1
        );

        let method = CString::new("read").expect("CString");
        let result = nyash_plugin_invoke_by_name_i64(filebox_handle, method.as_ptr(), 0, 0, 0);
        assert_eq!(result, 0, "read should no longer use builtin by_name keep");

        with_filebox_from_handle(filebox_handle, |filebox| {
            let _ = filebox.ny_close();
        });
        let _ = std::fs::remove_file(tmp_path);
    });
}

#[test]
fn filebox_by_name_write_bytes_is_retired() {
    ensure_test_ring0();

    with_env_var("NYASH_VM_USE_FALLBACK", "1", || {
        let tmp_path = "/tmp/nyash_kernel_filebox_write_bytes_retired.bin";
        let _ = std::fs::remove_file(tmp_path);

        let type_name = CString::new("FileBox").expect("CString");
        let filebox_handle = nyash_env_box_new_i64x(type_name.as_ptr(), 0, 0, 0, 0, 0);
        assert!(filebox_handle > 0, "expected FileBox handle");

        with_filebox_from_handle(filebox_handle, |filebox| {
            filebox
                .ny_open(tmp_path, "w")
                .expect("direct open should succeed");
        });

        let bytes = nyash_rust::boxes::array::ArrayBox::new();
        bytes.push(Box::new(StringBox::new("A".to_string())));
        let bytes_handle = handles::to_handle_arc(Arc::new(bytes)) as i64;

        let method = CString::new("writeBytes").expect("CString");
        let result =
            nyash_plugin_invoke_by_name_i64(filebox_handle, method.as_ptr(), 1, bytes_handle, 0);
        assert_eq!(result, 0, "writeBytes should no longer use builtin by_name keep");

        with_filebox_from_handle(filebox_handle, |filebox| {
            let _ = filebox.ny_close();
        });
        let _ = std::fs::remove_file(tmp_path);
    });
}

#[test]
fn instancebox_by_name_get_field_is_retired() {
    ensure_test_ring0();

    with_env_var("NYASH_VM_USE_FALLBACK", "1", || {
        let inst_handle = instancebox_handle_with_field("x", NyashValue::Integer(42));
        let method = CString::new("getField").expect("CString");
        let field_handle = string_handle("x");
        let result = nyash_plugin_invoke_by_name_i64(inst_handle, method.as_ptr(), 1, field_handle, 0);
        assert_eq!(
            result, 0,
            "InstanceBox.getField should no longer use builtin by_name keep"
        );
    });
}

#[test]
fn instancebox_by_name_set_field_is_retired() {
    ensure_test_ring0();

    with_env_var("NYASH_VM_USE_FALLBACK", "1", || {
        let inst_handle = instancebox_handle_with_field("x", NyashValue::Integer(1));
        let method = CString::new("setField").expect("CString");
        let field_handle = string_handle("x");
        let value_handle = handles::to_handle_arc(Arc::new(nyash_rust::box_trait::IntegerBox::new(99))) as i64;
        let result =
            nyash_plugin_invoke_by_name_i64(inst_handle, method.as_ptr(), 2, field_handle, value_handle);
        assert_eq!(
            result, 0,
            "InstanceBox.setField should no longer use builtin by_name keep"
        );
    });
}

#[test]
fn box_from_i8_string_const_reuses_handle() {
    let s = CString::new("phase21_5_fast").expect("CString");
    let h1 = nyash_box_from_i8_string_const(s.as_ptr());
    let h2 = nyash_box_from_i8_string_const(s.as_ptr());
    assert!(h1 > 0);
    assert_eq!(h1, h2, "const helper should intern and reuse handle");
    assert!(handles::get(h1 as u64).is_some());
}

#[test]
fn string_concat3_hhh_contract() {
    let a: Arc<dyn NyashBox> = Arc::new(StringBox::new("ha".to_string()));
    let b: Arc<dyn NyashBox> = Arc::new(StringBox::new("ko".to_string()));
    let c: Arc<dyn NyashBox> = Arc::new(StringBox::new("run".to_string()));
    let a_h = handles::to_handle_arc(a) as i64;
    let b_h = handles::to_handle_arc(b) as i64;
    let c_h = handles::to_handle_arc(c) as i64;

    let out_h = nyash_string_concat3_hhh_export(a_h, b_h, c_h);
    assert!(out_h > 0);
    let out_obj = handles::get(out_h as u64).expect("concat3 result handle");
    let out_str = out_obj
        .as_any()
        .downcast_ref::<StringBox>()
        .expect("concat3 result must be StringBox");
    assert_eq!(out_str.value, "hakorun");

    // Fallback contract: invalid handle is treated as empty string.
    let out_h2 = nyash_string_concat3_hhh_export(a_h, 0, c_h);
    assert!(out_h2 > 0);
    let out_obj2 = handles::get(out_h2 as u64).expect("concat3 fallback handle");
    let out_str2 = out_obj2
        .as_any()
        .downcast_ref::<StringBox>()
        .expect("concat3 fallback must be StringBox");
    assert_eq!(out_str2.value, "harun");
}

#[test]
fn string_compare_hh_contract_roundtrip() {
    let a: Arc<dyn NyashBox> = Arc::new(StringBox::new("abc".to_string()));
    let b: Arc<dyn NyashBox> = Arc::new(StringBox::new("abc".to_string()));
    let c: Arc<dyn NyashBox> = Arc::new(StringBox::new("abd".to_string()));
    let a_h = handles::to_handle_arc(a) as i64;
    let b_h = handles::to_handle_arc(b) as i64;
    let c_h = handles::to_handle_arc(c) as i64;

    assert_eq!(nyash_string_eq_hh_export(a_h, b_h), 1);
    assert_eq!(nyash_string_eq_hh_export(a_h, c_h), 0);
    assert_eq!(nyash_string_lt_hh_export(a_h, c_h), 1);
    assert_eq!(nyash_string_lt_hh_export(c_h, a_h), 0);
}

#[test]
fn string_indexof_lastindexof_invalid_needle_contract() {
    let hay: Arc<dyn NyashBox> = Arc::new(StringBox::new("abcabc".to_string()));
    let hay_h = handles::to_handle_arc(hay) as i64;

    // Invalid/zero handle is treated as empty needle by current contract.
    assert_eq!(nyash_string_indexof_hh_export(hay_h, 0), 0);
    assert_eq!(nyash_string_lastindexof_hh_export(hay_h, 0), 6);
}

#[test]
fn string_indexof_hh_cached_pair_route_roundtrip() {
    let hay: Arc<dyn NyashBox> = Arc::new(StringBox::new("abc".to_string()));
    let hay_h = handles::to_handle_arc(hay) as i64;
    let needle: Arc<dyn NyashBox> = Arc::new(StringBox::new("b".to_string()));
    let needle_h = handles::to_handle_arc(needle) as i64;

    // Repeated pair lookup must preserve semantics.
    assert_eq!(nyash_string_indexof_hh_export(hay_h, needle_h), 1);
    assert_eq!(nyash_string_indexof_hh_export(hay_h, needle_h), 1);
}

#[test]
fn string_len_h_invalid_handle_contract() {
    assert_eq!(nyash_string_len_h(0), 0);
    assert_eq!(nyash_string_len_h(-1), 0);
}

#[test]
fn string_exports_prefer_hako_forward_hook_when_registered() {
    extern "C" fn string_hook(op: i64, a0: i64, a1: i64, a2: i64) -> i64 {
        op * 1000 + a0 + a1 + a2
    }

    crate::hako_forward_bridge::with_test_reset(|| {
        assert_eq!(
            crate::hako_forward_bridge::register_string_dispatch(Some(string_hook)),
            1
        );
        assert_eq!(nyash_string_len_h(7), 1007);
        assert_eq!(nyash_string_concat_hh_export(3, 4), 3007);
    });
}

#[test]
fn string_exports_disable_rust_fallback_when_policy_is_off() {
    with_env_var("NYASH_VM_USE_FALLBACK", "0", || {
        crate::hako_forward_bridge::with_test_reset(|| {
            let src: Arc<dyn NyashBox> = Arc::new(StringBox::new("abc".to_string()));
            let src_h = handles::to_handle_arc(src) as i64;
            assert_eq!(
                nyash_string_len_h(src_h),
                crate::hako_forward_bridge::NYRT_E_HOOK_MISS
            );
            let concat_h = nyash_string_concat_hh_export(src_h, src_h);
            assert!(concat_h > 0);
            let concat_text =
                decode_string_like_handle(concat_h).expect("concat freeze handle string");
            assert!(concat_text.contains("[freeze:contract][hako_forward/hook_miss]"));
            assert!(concat_text.contains("route=string.concat_hh"));
        });
    });
}

#[test]
fn string_to_i8p_h_fallback_contract() {
    use std::ffi::CStr;

    let c0 = nyash_string_to_i8p_h(0);
    assert!(!c0.is_null());
    let s0 = unsafe { CStr::from_ptr(c0) }.to_str().expect("utf8");
    assert_eq!(s0, "0");

    let missing = 9_876_543_210_i64;
    let c_missing = nyash_string_to_i8p_h(missing);
    assert!(!c_missing.is_null());
    let s_missing = unsafe { CStr::from_ptr(c_missing) }.to_str().expect("utf8");
    assert_eq!(s_missing, missing.to_string());
}

#[test]
fn string_indexof_lastindexof_single_byte_contract() {
    let hay: Arc<dyn NyashBox> = Arc::new(StringBox::new("abba-bba".to_string()));
    let hay_h = handles::to_handle_arc(hay) as i64;
    let needle: Arc<dyn NyashBox> = Arc::new(StringBox::new("b".to_string()));
    let needle_h = handles::to_handle_arc(needle) as i64;

    assert_eq!(nyash_string_indexof_hh_export(hay_h, needle_h), 1);
    assert_eq!(nyash_string_lastindexof_hh_export(hay_h, needle_h), 6);
}

#[test]
fn string_indexof_lastindexof_multibyte_contract() {
    let hay: Arc<dyn NyashBox> = Arc::new(StringBox::new("hako-hako".to_string()));
    let hay_h = handles::to_handle_arc(hay) as i64;
    let needle: Arc<dyn NyashBox> = Arc::new(StringBox::new("ko".to_string()));
    let needle_h = handles::to_handle_arc(needle) as i64;

    assert_eq!(nyash_string_indexof_hh_export(hay_h, needle_h), 2);
    assert_eq!(nyash_string_lastindexof_hh_export(hay_h, needle_h), 7);
}

#[test]
fn substring_hii_short_slice_materializes_under_fast_contract() {
    use nyash_rust::boxes::array::ArrayBox;
    use std::ffi::CStr;

    with_env_var("NYASH_LLVM_FAST", "1", || {
        let source: Arc<dyn NyashBox> = Arc::new(StringBox::new("hakorune".to_string()));
        let source_handle = handles::to_handle_arc(source) as i64;
        let sub_handle = nyash_string_substring_hii_export(source_handle, 1, 5);
        assert!(sub_handle > 0, "substring handle");

        let sub_obj = handles::get(sub_handle as u64).expect("substring object");
        let sub_sb = sub_obj
            .as_any()
            .downcast_ref::<StringBox>()
            .expect("short fast slice should materialize to StringBox");
        assert_eq!(sub_sb.value, "akor");
        assert_eq!(nyash_string_len_h(sub_handle), 4);

        let needle: Arc<dyn NyashBox> = Arc::new(StringBox::new("ko".to_string()));
        let needle_handle = handles::to_handle_arc(needle) as i64;
        assert_eq!(nyash_string_indexof_hh_export(sub_handle, needle_handle), 1);

        let c_ptr = nyash_string_to_i8p_h(sub_handle);
        assert!(!c_ptr.is_null());
        let c_view = unsafe { CStr::from_ptr(c_ptr) }
            .to_str()
            .expect("substring utf8");
        assert_eq!(c_view, "akor");

        // Persistent container boundary still stores owned StringBox.
        let array: Arc<dyn NyashBox> = Arc::new(ArrayBox::new());
        let array_handle = handles::to_handle_arc(array) as i64;
        assert_eq!(nyash_runtime_data_push_hh(array_handle, sub_handle), 1);
        let stored_handle = nyash_runtime_data_get_hh(array_handle, 0);
        assert!(stored_handle > 0);
        let stored_obj = handles::get(stored_handle as u64).expect("stored object");
        let stored_sb = stored_obj
            .as_any()
            .downcast_ref::<StringBox>()
            .expect("stored value should materialize to StringBox");
        assert_eq!(stored_sb.value, "akor");
    });
}

#[test]
fn substring_hii_short_nested_slice_materializes_under_fast_contract() {
    with_env_var("NYASH_LLVM_FAST", "1", || {
        let source: Arc<dyn NyashBox> = Arc::new(StringBox::new("hakorune".to_string()));
        let source_handle = handles::to_handle_arc(source) as i64;
        let view_handle = nyash_string_substring_hii_export(source_handle, 1, 5);
        assert!(view_handle > 0, "view handle");

        let nested_handle = nyash_string_substring_hii_export(view_handle, 0, 2);
        assert!(nested_handle > 0, "nested substring handle");

        let nested_obj = handles::get(nested_handle as u64).expect("nested substring object");
        let nested_sb = nested_obj
            .as_any()
            .downcast_ref::<StringBox>()
            .expect("short nested slice should materialize to StringBox");
        assert_eq!(nested_sb.value, "ak");
        assert_eq!(nyash_string_len_h(nested_handle), 2);
        let c_ptr = nyash_string_to_i8p_h(nested_handle);
        assert!(!c_ptr.is_null());
        let c_view = unsafe { std::ffi::CStr::from_ptr(c_ptr) }
            .to_str()
            .expect("nested substring utf8");
        assert_eq!(c_view, "ak");
    });
}

#[test]
fn substring_hii_mid_slice_keeps_stringview_contract() {
    with_env_var("NYASH_LLVM_FAST", "1", || {
        let source: Arc<dyn NyashBox> = Arc::new(StringBox::new("line-seed-abcdefxx".to_string()));
        let source_handle = handles::to_handle_arc(source) as i64;

        let view_handle = nyash_string_substring_hii_export(source_handle, 1, 17);
        assert!(view_handle > 0, "mid substring handle");

        let view_obj = handles::get(view_handle as u64).expect("mid substring object");
        assert_eq!(view_obj.type_name(), "StringViewBox");
        assert_eq!(nyash_string_len_h(view_handle), 16);
    });
}

#[test]
fn substring_hii_fast_off_keeps_stringbox_contract() {
    with_env_var("NYASH_LLVM_FAST", "0", || {
        let source: Arc<dyn NyashBox> = Arc::new(StringBox::new("hakorune".to_string()));
        let source_handle = handles::to_handle_arc(source) as i64;
        let sub_handle = nyash_string_substring_hii_export(source_handle, 1, 5);
        assert!(sub_handle > 0, "substring handle");
        let sub_obj = handles::get(sub_handle as u64).expect("substring object");
        let sub_sb = sub_obj
            .as_any()
            .downcast_ref::<StringBox>()
            .expect("fast off should keep StringBox result");
        assert_eq!(sub_sb.value, "akor");
    });
}

#[test]
fn invoke_by_name_accepts_stage1_using_resolver_module_receiver() {
    let result_handle = dispatch_stage1_module(
        "lang.compiler.entry.using_resolver_box",
        "resolve_for_source",
        "static box Main { main() { return 0 } }",
    );
    assert!(result_handle > 0, "expected StringBox handle");

    let result_object = handles::get(result_handle as u64).expect("result handle");
    let result_string = result_object
        .as_any()
        .downcast_ref::<StringBox>()
        .expect("StringBox result");
    assert_eq!(result_string.value, "");
}

#[test]
fn invoke_by_name_accepts_stage1_mir_builder_source_route_for_stage1_cli_env() {
    let result_handle = dispatch_stage1_module(
        "lang.mir.builder.MirBuilderBox",
        "emit_from_source_v0",
        include_str!("../../../lang/src/runner/stage1_cli_env.hako"),
    );
    assert!(result_handle > 0, "expected MIR JSON StringBox handle");

    let mir_json = decode_string_like_handle(result_handle).expect("mir json string");
    assert!(
        mir_json.starts_with('{'),
        "expected MIR JSON payload, got: {}",
        mir_json
    );
    assert!(mir_json.contains("\"functions\""));
    let mir_value: serde_json::Value = serde_json::from_str(&mir_json).expect("valid mir json");
    let user_box_decls = mir_value["user_box_decls"]
        .as_array()
        .expect("user_box_decls array");
    let box_names = user_box_decls
        .iter()
        .filter_map(|decl| decl["name"].as_str())
        .collect::<std::collections::BTreeSet<_>>();
    assert!(
        box_names.contains("Stage1InputContractBox"),
        "source authority route should expose Stage1InputContractBox user_box_decl"
    );
    assert!(
        box_names.contains("Stage1SourceMirAuthorityBox"),
        "source authority route should expose Stage1SourceMirAuthorityBox user_box_decl"
    );
    assert!(
        box_names.contains("Stage1ProgramJsonCompatBox"),
        "source authority route should preserve explicit compat box decls for same-file closure"
    );
}

#[test]
fn invoke_by_name_export_accepts_stage1_mir_builder_source_route_for_stage1_cli_env() {
    with_env_vars(
        &[
            ("HAKO_MIR_BUILDER_INTERNAL", "1"),
            ("NYASH_VM_USE_FALLBACK", "1"),
        ],
        || {
            let recv_handle = handles::to_handle_arc(Arc::new(StringBox::new(
                "lang.mir.builder.MirBuilderBox".to_string(),
            ))) as i64;
            let method = CString::new("emit_from_source_v0").expect("CString");
            let source_handle = handles::to_handle_arc(Arc::new(StringBox::new(
                include_str!("../../../lang/src/runner/stage1_cli_env.hako").to_string(),
            ))) as i64;

            let result_handle =
                nyash_plugin_invoke_by_name_i64(recv_handle, method.as_ptr(), 1, source_handle, 0);
            assert!(result_handle > 0, "expected MIR JSON StringBox handle");

            let mir_json = decode_string_like_handle(result_handle).expect("mir json string");
            assert!(
                mir_json.starts_with('{'),
                "expected MIR JSON payload, got: {}",
                mir_json
            );
            assert!(mir_json.contains("\"functions\""));
            let mir_value: serde_json::Value =
                serde_json::from_str(&mir_json).expect("valid mir json");
            let user_box_decls = mir_value["user_box_decls"]
                .as_array()
                .expect("user_box_decls array");
            let box_names = user_box_decls
                .iter()
                .filter_map(|decl| decl["name"].as_str())
                .collect::<std::collections::BTreeSet<_>>();
            assert!(
                box_names.contains("Stage1InputContractBox"),
                "source authority route should expose Stage1InputContractBox user_box_decl"
            );
            assert!(
                box_names.contains("Stage1SourceMirAuthorityBox"),
                "source authority route should expose Stage1SourceMirAuthorityBox user_box_decl"
            );
            assert!(
                box_names.contains("Stage1ProgramJsonCompatBox"),
                "source authority route should preserve explicit compat box decls for same-file closure"
            );
        },
    );
}

#[test]
fn invoke_by_name_accepts_stage1_mir_builder_source_route_for_hello_simple_llvm() {
    ensure_test_ring0();
    let result_handle = dispatch_stage1_module(
        "lang.mir.builder.MirBuilderBox",
        "emit_from_source_v0",
        include_str!("../../../apps/tests/hello_simple_llvm.hako"),
    );
    assert!(result_handle > 0, "expected MIR JSON StringBox handle");

    let mir_json = decode_string_like_handle(result_handle).expect("mir json string");
    assert!(
        mir_json.starts_with('{'),
        "expected MIR JSON payload, got: {}",
        mir_json
    );
    assert!(mir_json.contains("\"functions\""));
}

#[test]
fn invoke_by_name_stage1_mir_builder_source_route_accepts_decode_escapes_nested_loop_fixture() {
    ensure_test_ring0();
    let result_handle = dispatch_stage1_module(
        "lang.mir.builder.MirBuilderBox",
        "emit_from_source_v0",
        include_str!(
            "../../../apps/tests/phase29bq_selfhost_blocker_decode_escapes_if_idx12_min.hako"
        ),
    );
    assert!(result_handle > 0, "expected MIR JSON StringBox handle");

    let mir_json = decode_string_like_handle(result_handle).expect("mir json string");
    assert!(
        mir_json.starts_with('{'),
        "expected MIR JSON payload, got: {mir_json}"
    );
    assert!(mir_json.contains("\"functions\""));
}

#[test]
fn invoke_by_name_stage1_using_resolver_route_is_stubbed_empty_in_kernel_dispatch() {
    ensure_test_ring0();
    let result_handle = dispatch_stage1_module(
        "lang.compiler.entry.using_resolver_box",
        "resolve_for_source",
        include_str!("../../../lang/src/runner/stage1_cli_env.hako"),
    );
    assert!(result_handle > 0, "expected stub StringBox handle");

    let prefix = decode_string_like_handle(result_handle).expect("prefix text");
    assert_eq!(
        prefix, "",
        "kernel direct module dispatch intentionally stubs resolve_for_source"
    );
}

#[test]
fn future_spawn_instance_disable_rust_fallback_when_policy_is_off() {
    with_env_var("NYASH_VM_USE_FALLBACK", "0", || {
        crate::hako_forward_bridge::with_test_reset(|| {
            let result_handle = nyash_future_spawn_instance3_i64(1, 2, 3, 4);
            assert!(result_handle > 0);
            let result_text =
                decode_string_like_handle(result_handle).expect("hook-miss freeze string");
            assert!(result_text.contains("[freeze:contract][hako_forward/hook_miss]"));
            assert!(result_text.contains("route=future.spawn_instance3"));
        });
    });
}

#[test]
fn runtime_data_dispatch_array_push_get_index_zero() {
    use nyash_rust::boxes::array::ArrayBox;

    let array: Arc<dyn NyashBox> = Arc::new(ArrayBox::new());
    let array_handle = handles::to_handle_arc(array) as i64;
    let value: Arc<dyn NyashBox> = Arc::new(StringBox::new("abc".to_string()));
    let value_handle = handles::to_handle_arc(value) as i64;

    let new_len = nyash_runtime_data_push_hh(array_handle, value_handle);
    assert_eq!(new_len, 1);

    let got_handle = nyash_runtime_data_get_hh(array_handle, 0);
    assert!(got_handle > 0, "array get should return a valid handle");
    let got_obj = handles::get(got_handle as u64).expect("array get handle");
    let got_str = got_obj
        .as_any()
        .downcast_ref::<StringBox>()
        .expect("array get value must be StringBox");
    assert_eq!(got_str.value, "abc");
    assert_eq!(nyash_runtime_data_has_hh(array_handle, 0), 1);
}

#[test]
fn runtime_data_dispatch_array_negative_index_contract() {
    use nyash_rust::boxes::array::ArrayBox;

    let array: Arc<dyn NyashBox> = Arc::new(ArrayBox::new());
    let array_handle = handles::to_handle_arc(array) as i64;

    assert_eq!(nyash_runtime_data_push_hh(array_handle, -11), 1);
    assert_eq!(nyash_runtime_data_get_hh(array_handle, 0), -11);

    // Legacy contract: negative index is immediate 0 (no handle allocation / no mutation).
    assert_eq!(nyash_runtime_data_get_hh(array_handle, -1), 0);
    assert_eq!(nyash_runtime_data_has_hh(array_handle, -1), 0);
    assert_eq!(nyash_runtime_data_set_hhh(array_handle, -1, 99), 0);

    assert_eq!(nyash_runtime_data_get_hh(array_handle, 0), -11);
    assert_eq!(nyash_runtime_data_has_hh(array_handle, 1), 0);
}

#[test]
fn runtime_data_dispatch_array_set_index_contract() {
    use nyash_rust::boxes::array::ArrayBox;

    let array: Arc<dyn NyashBox> = Arc::new(ArrayBox::new());
    let array_handle = handles::to_handle_arc(array) as i64;

    assert_eq!(nyash_array_length_h(array_handle), 0);

    // idx == len appends and reports success.
    assert_eq!(nyash_runtime_data_set_hhh(array_handle, 0, -10), 1);
    assert_eq!(nyash_array_length_h(array_handle), 1);
    assert_eq!(nyash_runtime_data_get_hh(array_handle, 0), -10);

    // idx < len overwrites and reports success.
    assert_eq!(nyash_runtime_data_set_hhh(array_handle, 0, -11), 1);
    assert_eq!(nyash_array_length_h(array_handle), 1);
    assert_eq!(nyash_runtime_data_get_hh(array_handle, 0), -11);

    // has_hh contract: idx in-range => 1, idx == len => 0.
    assert_eq!(nyash_runtime_data_has_hh(array_handle, 0), 1);
    assert_eq!(nyash_runtime_data_has_hh(array_handle, 1), 0);

    // idx > len rejects write and keeps length unchanged.
    assert_eq!(nyash_runtime_data_set_hhh(array_handle, 2, -99), 0);
    assert_eq!(nyash_array_length_h(array_handle), 1);
    assert_eq!(nyash_runtime_data_get_hh(array_handle, 0), -11);
    assert_eq!(nyash_runtime_data_has_hh(array_handle, 1), 0);
}

#[test]
fn runtime_data_dispatch_array_positive_immediate_index_contract() {
    use nyash_rust::{box_trait::IntegerBox, boxes::array::ArrayBox};

    let array: Arc<dyn NyashBox> = Arc::new(ArrayBox::new());
    let array_handle = handles::to_handle_arc(array) as i64;

    assert_eq!(nyash_runtime_data_push_hh(array_handle, -10), 1);
    assert_eq!(nyash_runtime_data_push_hh(array_handle, -20), 2);

    // Positive immediate indices must not be blocked by unrelated live handles.
    assert_eq!(nyash_runtime_data_get_hh(array_handle, 1), -20);
    assert_eq!(nyash_runtime_data_has_hh(array_handle, 1), 1);

    // Integer-handle key remains supported.
    let key_one: Arc<dyn NyashBox> = Arc::new(IntegerBox::new(1));
    let key_one_handle = handles::to_handle_arc(key_one) as i64;
    assert_eq!(nyash_runtime_data_get_hh(array_handle, key_one_handle), -20);
    assert_eq!(nyash_runtime_data_has_hh(array_handle, key_one_handle), 1);
    assert_eq!(
        nyash_runtime_data_set_hhh(array_handle, key_one_handle, -30),
        1
    );
    assert_eq!(nyash_runtime_data_get_hh(array_handle, 1), -30);
}

#[test]
fn array_runtime_data_route_hh_contract_roundtrip() {
    use nyash_rust::boxes::array::ArrayBox;

    let array: Arc<dyn NyashBox> = Arc::new(ArrayBox::new());
    let array_handle = handles::to_handle_arc(array) as i64;
    let key_zero = 0i64;

    let v1: Arc<dyn NyashBox> = Arc::new(StringBox::new("route-a".to_string()));
    let v1_h = handles::to_handle_arc(v1) as i64;

    assert_eq!(nyash_array_push_hh_alias(array_handle, v1_h), 1);
    assert_eq!(nyash_array_has_hh_alias(array_handle, key_zero), 1);

    let got_h = nyash_array_get_hh_alias(array_handle, key_zero);
    assert!(got_h > 0);
    let got_obj = handles::get(got_h as u64).expect("array.get_hh result handle");
    let got_str = got_obj
        .as_any()
        .downcast_ref::<StringBox>()
        .expect("array.get_hh result should be StringBox");
    assert_eq!(got_str.value, "route-a");

    let v2: Arc<dyn NyashBox> = Arc::new(StringBox::new("route-b".to_string()));
    let v2_h = handles::to_handle_arc(v2) as i64;
    assert_eq!(nyash_array_set_hhh_alias(array_handle, key_zero, v2_h), 1);

    let got2_h = nyash_array_get_hh_alias(array_handle, key_zero);
    assert!(got2_h > 0);
    let got2_obj = handles::get(got2_h as u64).expect("array.get_hh result handle 2");
    let got2_str = got2_obj
        .as_any()
        .downcast_ref::<StringBox>()
        .expect("array.get_hh result should be StringBox");
    assert_eq!(got2_str.value, "route-b");

    assert_eq!(nyash_array_get_hh_alias(array_handle, -1), 0);
    assert_eq!(nyash_array_has_hh_alias(array_handle, -1), 0);
    assert_eq!(nyash_array_set_hhh_alias(array_handle, -1, v1_h), 0);
}

#[test]
fn array_runtime_data_route_hi_contract_roundtrip() {
    use nyash_rust::boxes::array::ArrayBox;

    let array: Arc<dyn NyashBox> = Arc::new(ArrayBox::new());
    let array_handle = handles::to_handle_arc(array) as i64;
    let key_zero = 0i64;

    let v1: Arc<dyn NyashBox> = Arc::new(StringBox::new("route-hi-a".to_string()));
    let v1_h = handles::to_handle_arc(v1) as i64;

    assert_eq!(nyash_array_push_hh_alias(array_handle, v1_h), 1);
    assert_eq!(nyash_array_has_hi_alias(array_handle, key_zero), 1);

    let got_h = nyash_array_get_hi_alias(array_handle, key_zero);
    assert!(got_h > 0);
    let got_obj = handles::get(got_h as u64).expect("array.get_hi result handle");
    let got_str = got_obj
        .as_any()
        .downcast_ref::<StringBox>()
        .expect("array.get_hi result should be StringBox");
    assert_eq!(got_str.value, "route-hi-a");

    let v2: Arc<dyn NyashBox> = Arc::new(StringBox::new("route-hi-b".to_string()));
    let v2_h = handles::to_handle_arc(v2) as i64;
    assert_eq!(nyash_array_set_hih_alias(array_handle, key_zero, v2_h), 1);

    let got2_h = nyash_array_get_hi_alias(array_handle, key_zero);
    assert!(got2_h > 0);
    let got2_obj = handles::get(got2_h as u64).expect("array.get_hi result handle 2");
    let got2_str = got2_obj
        .as_any()
        .downcast_ref::<StringBox>()
        .expect("array.get_hi result should be StringBox");
    assert_eq!(got2_str.value, "route-hi-b");

    assert_eq!(nyash_array_get_hi_alias(array_handle, -1), 0);
    assert_eq!(nyash_array_has_hi_alias(array_handle, -1), 0);
    assert_eq!(nyash_array_set_hih_alias(array_handle, -1, v1_h), 0);
}

#[test]
fn array_runtime_data_route_hii_contract_roundtrip() {
    use nyash_rust::boxes::array::ArrayBox;

    let array: Arc<dyn NyashBox> = Arc::new(ArrayBox::new());
    let array_handle = handles::to_handle_arc(array) as i64;

    assert_eq!(nyash_array_set_hii_alias(array_handle, 0, 41), 1);
    assert_eq!(nyash_array_get_hi_alias(array_handle, 0), 41);
    assert_eq!(nyash_array_set_hii_alias(array_handle, 0, 42), 1);
    assert_eq!(nyash_array_get_hi_alias(array_handle, 0), 42);
    assert_eq!(nyash_array_set_hii_alias(array_handle, -1, 7), 0);
}

#[test]
fn array_get_hi_bool_returns_i64_contract() {
    use nyash_rust::{box_trait::BoolBox, boxes::array::ArrayBox};

    let array: Arc<dyn NyashBox> = Arc::new(ArrayBox::new());
    let array_handle = handles::to_handle_arc(array) as i64;
    let bool_obj: Arc<dyn NyashBox> = Arc::new(BoolBox::new(true));
    let bool_h = handles::to_handle_arc(bool_obj) as i64;

    assert_eq!(nyash_array_push_hh_alias(array_handle, bool_h), 1);
    assert_eq!(nyash_array_get_hi_alias(array_handle, 0), 1);
}

#[test]
fn array_set_h_legacy_return_code_contract() {
    use nyash_rust::boxes::array::ArrayBox;

    let array: Arc<dyn NyashBox> = Arc::new(ArrayBox::new());
    let array_handle = handles::to_handle_arc(array) as i64;

    // Legacy ABI: set_h always reports completion via return=0.
    assert_eq!(nyash_array_set_h(array_handle, 0, 41), 0);
    assert_eq!(nyash_array_get_h(array_handle, 0), 41);
    assert_eq!(nyash_array_length_h(array_handle), 1);

    // Gap write is ignored but still returns 0.
    assert_eq!(nyash_array_set_h(array_handle, 2, 99), 0);
    assert_eq!(nyash_array_length_h(array_handle), 1);

    // Exact-end write appends.
    assert_eq!(nyash_array_set_h(array_handle, 1, 42), 0);
    assert_eq!(nyash_array_get_h(array_handle, 1), 42);
    assert_eq!(nyash_array_length_h(array_handle), 2);
}

#[test]
fn runtime_v0_slice_invalid_handle_contract() {
    // V0 slice contract: invalid/negative handles are immediate zero path.
    assert_eq!(nyash_string_len_h(0), 0);
    assert_eq!(nyash_array_get_hi_alias(0, 0), 0);
    assert_eq!(nyash_array_get_hi_alias(-1, 0), 0);
    assert_eq!(nyash_array_set_hii_alias(0, 0, 1), 0);
    assert_eq!(nyash_array_set_hii_alias(-1, 0, 1), 0);
}

#[test]
fn runtime_data_dispatch_map_set_get_has() {
    use nyash_rust::{box_trait::IntegerBox, boxes::map_box::MapBox};

    let map: Arc<dyn NyashBox> = Arc::new(MapBox::new());
    let map_handle = handles::to_handle_arc(map) as i64;
    let key: Arc<dyn NyashBox> = Arc::new(StringBox::new("k".to_string()));
    let key_handle = handles::to_handle_arc(key) as i64;
    let expected = 42;
    let value: Arc<dyn NyashBox> = Arc::new(IntegerBox::new(expected));
    let value_handle = handles::to_handle_arc(value) as i64;

    assert_eq!(
        nyash_runtime_data_set_hhh(map_handle, key_handle, value_handle),
        1
    );
    assert_eq!(nyash_runtime_data_has_hh(map_handle, key_handle), 1);

    let value = nyash_runtime_data_get_hh(map_handle, key_handle);
    if value == expected {
        return;
    }
    assert!(value > 0, "map get should return raw int or valid handle");
    let value_obj = handles::get(value as u64).expect("map get handle");
    let value_int = value_obj
        .as_any()
        .downcast_ref::<nyash_rust::box_trait::IntegerBox>()
        .expect("map value handle must wrap IntegerBox");
    assert_eq!(value_int.value, expected);
}

#[test]
fn runtime_data_dispatch_map_push_missing_key_contract() {
    use nyash_rust::{box_trait::IntegerBox, boxes::map_box::MapBox};

    let map: Arc<dyn NyashBox> = Arc::new(MapBox::new());
    let map_handle = handles::to_handle_arc(map) as i64;

    assert_eq!(nyash_runtime_data_push_hh(map_handle, -1), 0);

    let missing_key: Arc<dyn NyashBox> = Arc::new(StringBox::new("missing".to_string()));
    let missing_key_handle = handles::to_handle_arc(missing_key) as i64;
    assert_eq!(nyash_runtime_data_has_hh(map_handle, missing_key_handle), 0);
    assert_eq!(nyash_runtime_data_get_hh(map_handle, missing_key_handle), 0);

    let present_key: Arc<dyn NyashBox> = Arc::new(StringBox::new("present".to_string()));
    let present_key_handle = handles::to_handle_arc(present_key) as i64;
    let expected = 314;
    let value: Arc<dyn NyashBox> = Arc::new(IntegerBox::new(expected));
    let value_handle = handles::to_handle_arc(value) as i64;

    assert_eq!(
        nyash_runtime_data_set_hhh(map_handle, present_key_handle, value_handle),
        1
    );
    assert_eq!(nyash_runtime_data_has_hh(map_handle, present_key_handle), 1);

    let got = nyash_runtime_data_get_hh(map_handle, present_key_handle);
    if got == expected {
        return;
    }
    assert!(got > 0, "map get should return raw int or valid handle");
    let got_obj = handles::get(got as u64).expect("map get handle");
    let got_int = got_obj
        .as_any()
        .downcast_ref::<IntegerBox>()
        .expect("map get handle must wrap IntegerBox");
    assert_eq!(got_int.value, expected);
}

#[test]
fn map_set_h_legacy_completion_code_and_mutation_roundtrip() {
    use nyash_rust::boxes::map_box::MapBox;

    let map: Arc<dyn NyashBox> = Arc::new(MapBox::new());
    let map_handle = handles::to_handle_arc(map) as i64;
    let key = -70001;

    let value: Arc<dyn NyashBox> = Arc::new(StringBox::new("legacy-set-h".to_string()));
    let value_handle = handles::to_handle_arc(value) as i64;

    assert_eq!(nyash_map_set_h(map_handle, key, value_handle), 0);
    assert_eq!(nyash_map_has_hh(map_handle, key), 1);

    let got_handle = nyash_map_get_hh(map_handle, key);
    assert!(got_handle > 0, "map get_hh must return a value handle");
    let got_obj = handles::get(got_handle as u64).expect("map get_hh handle");
    let got_value = got_obj
        .as_any()
        .downcast_ref::<StringBox>()
        .expect("map value must be StringBox");
    assert_eq!(got_value.value, "legacy-set-h");
}

#[test]
fn map_set_hh_legacy_completion_code_and_mutation_roundtrip() {
    use nyash_rust::boxes::map_box::MapBox;

    let map: Arc<dyn NyashBox> = Arc::new(MapBox::new());
    let map_handle = handles::to_handle_arc(map) as i64;

    let key: Arc<dyn NyashBox> = Arc::new(StringBox::new("legacy-key-hh".to_string()));
    let key_handle = handles::to_handle_arc(key) as i64;
    let value: Arc<dyn NyashBox> = Arc::new(StringBox::new("legacy-value-hh".to_string()));
    let value_handle = handles::to_handle_arc(value) as i64;

    assert_eq!(nyash_map_set_hh(map_handle, key_handle, value_handle), 0);
    assert_eq!(nyash_map_has_hh(map_handle, key_handle), 1);

    let got_handle = nyash_map_get_hh(map_handle, key_handle);
    assert!(got_handle > 0, "map get_hh must return a value handle");
    let got_obj = handles::get(got_handle as u64).expect("map get_hh handle");
    let got_value = got_obj
        .as_any()
        .downcast_ref::<StringBox>()
        .expect("map value must be StringBox");
    assert_eq!(got_value.value, "legacy-value-hh");
}

#[test]
fn map_invalid_handle_fail_safe_contract() {
    assert_eq!(nyash_map_size_h(0), 0);
    assert_eq!(nyash_map_get_h(0, 1), 0);
    assert_eq!(nyash_map_get_hh(0, 1), 0);
    assert_eq!(nyash_map_has_h(0, 1), 0);
    assert_eq!(nyash_map_has_hh(0, 1), 0);
    assert_eq!(nyash_map_set_h(0, 1, 2), 0);
    assert_eq!(nyash_map_set_hh(0, 1, 2), 0);

    assert_eq!(nyash_map_size_h(-1), 0);
    assert_eq!(nyash_map_get_h(-1, 1), 0);
    assert_eq!(nyash_map_get_hh(-1, 1), 0);
    assert_eq!(nyash_map_has_h(-1, 1), 0);
    assert_eq!(nyash_map_has_hh(-1, 1), 0);
    assert_eq!(nyash_map_set_h(-1, 1, 2), 0);
    assert_eq!(nyash_map_set_hh(-1, 1, 2), 0);
}

#[test]
fn intarray_invalid_and_bounds_contract() {
    assert_eq!(nyash_intarray_len_h(0), 0);
    assert_eq!(nyash_intarray_get_hi(0, 0), 0);
    assert_eq!(nyash_intarray_set_hii(0, 0, 1), 1);

    let h = nyash_intarray_new_h(2);
    assert!(h > 0);
    assert_eq!(nyash_intarray_len_h(h), 2);
    assert_eq!(nyash_intarray_set_hii(h, 0, 10), 0);
    assert_eq!(nyash_intarray_set_hii(h, 1, 20), 0);
    assert_eq!(nyash_intarray_get_hi(h, 0), 10);
    assert_eq!(nyash_intarray_get_hi(h, 1), 20);

    assert_eq!(nyash_intarray_set_hii(h, -1, 99), 1);
    assert_eq!(nyash_intarray_set_hii(h, 2, 99), 1);
    assert_eq!(nyash_intarray_get_hi(h, -1), 0);
    assert_eq!(nyash_intarray_get_hi(h, 2), 0);
}

#[test]
fn console_null_and_invalid_handle_contract() {
    assert_eq!(nyash_console_log_export(std::ptr::null()), 0);
    assert_eq!(nyash_console_warn_export(std::ptr::null()), 0);
    assert_eq!(nyash_console_error_export(std::ptr::null()), 0);
    assert_eq!(nyash_debug_trace_export(std::ptr::null()), 0);

    assert_eq!(nyash_console_log_handle(0), 0);
    assert_eq!(nyash_console_warn_handle(0), 0);
    assert_eq!(nyash_console_error_handle(0), 0);
    assert_eq!(nyash_debug_trace_handle(0), 0);
}

#[test]
fn runtime_data_dispatch_invalid_receiver_returns_zero() {
    let receiver: Arc<dyn NyashBox> = Arc::new(StringBox::new("not-array-or-map".to_string()));
    let receiver_handle = handles::to_handle_arc(receiver) as i64;
    let key: Arc<dyn NyashBox> = Arc::new(StringBox::new("k".to_string()));
    let key_handle = handles::to_handle_arc(key) as i64;
    let value: Arc<dyn NyashBox> = Arc::new(StringBox::new("v".to_string()));
    let value_handle = handles::to_handle_arc(value) as i64;

    assert_eq!(nyash_runtime_data_get_hh(receiver_handle, key_handle), 0);
    assert_eq!(
        nyash_runtime_data_set_hhh(receiver_handle, key_handle, value_handle),
        0
    );
    assert_eq!(nyash_runtime_data_has_hh(receiver_handle, key_handle), 0);
    assert_eq!(nyash_runtime_data_push_hh(receiver_handle, value_handle), 0);
}

#[test]
fn handle_lifecycle_retain_h_zero_is_noop() {
    assert_eq!(nyrt_handle_retain_h(0), 0);
}

#[test]
fn handle_lifecycle_retain_release_h_roundtrip() {
    let object: Arc<dyn NyashBox> = Arc::new(StringBox::new("phase29y".to_string()));
    let handle = handles::to_handle_arc(object) as i64;
    assert!(handle > 0);

    let retained = nyrt_handle_retain_h(handle);
    assert!(retained > 0);
    assert!(
        handles::get(retained as u64).is_some(),
        "retained handle must be present"
    );
    assert!(
        handles::get(handle as u64).is_some(),
        "original handle must remain present"
    );

    nyrt_handle_release_h(retained);
    assert!(
        handles::get(retained as u64).is_none(),
        "released handle must be dropped"
    );
    assert!(
        handles::get(handle as u64).is_some(),
        "release_h must not drop unrelated handles"
    );
}

#[test]
fn handle_lifecycle_legacy_release_alias_drops_handle() {
    let object: Arc<dyn NyashBox> = Arc::new(StringBox::new("legacy".to_string()));
    let handle = handles::to_handle_arc(object) as i64;
    assert!(handles::get(handle as u64).is_some());

    ny_release_strong(handle);
    assert!(handles::get(handle as u64).is_none());
}

#[test]
fn handle_abi_borrowed_owned_conformance() {
    // SSOT: args borrowed / return owned
    // - borrowed arg remains valid during callee execution
    // - return value must be independently releasable by caller
    let object: Arc<dyn NyashBox> = Arc::new(StringBox::new("borrowed-owned".to_string()));
    let borrowed_handle = handles::to_handle_arc(object) as i64;
    assert!(borrowed_handle > 0);
    assert!(handles::get(borrowed_handle as u64).is_some());

    // Simulate "callee returns borrowed arg as owned" by retain on escape.
    let returned_owned_handle = nyrt_handle_retain_h(borrowed_handle);
    assert!(returned_owned_handle > 0);
    assert_ne!(
        borrowed_handle, returned_owned_handle,
        "retain_h must allocate an independent caller-owned handle"
    );
    assert!(handles::get(returned_owned_handle as u64).is_some());

    // Caller may release borrowed argument path (e.g., overwrite old binding)
    // while returned-owned value must stay alive.
    nyrt_handle_release_h(borrowed_handle);
    assert!(
        handles::get(borrowed_handle as u64).is_none(),
        "borrowed handle should be released"
    );
    assert!(
        handles::get(returned_owned_handle as u64).is_some(),
        "owned return must survive borrowed release"
    );

    // Caller is responsible for releasing owned return.
    nyrt_handle_release_h(returned_owned_handle);
    assert!(
        handles::get(returned_owned_handle as u64).is_none(),
        "owned return handle should be released by caller"
    );
}

#[test]
fn handle_abi_borrowed_owned_multi_escape_conformance() {
    // Matrix case: returned-owned handle can be re-borrowed/re-escaped and released independently.
    let object: Arc<dyn NyashBox> = Arc::new(StringBox::new("borrowed-owned-chain".to_string()));
    let borrowed_handle = handles::to_handle_arc(object) as i64;
    assert!(borrowed_handle > 0);

    let owned_handle_1 = nyrt_handle_retain_h(borrowed_handle);
    let owned_handle_2 = nyrt_handle_retain_h(owned_handle_1);
    assert!(owned_handle_1 > 0);
    assert!(owned_handle_2 > 0);
    assert_ne!(borrowed_handle, owned_handle_1);
    assert_ne!(owned_handle_1, owned_handle_2);

    nyrt_handle_release_h(borrowed_handle);
    assert!(handles::get(borrowed_handle as u64).is_none());
    assert!(handles::get(owned_handle_1 as u64).is_some());
    assert!(handles::get(owned_handle_2 as u64).is_some());

    nyrt_handle_release_h(owned_handle_1);
    assert!(handles::get(owned_handle_1 as u64).is_none());
    assert!(handles::get(owned_handle_2 as u64).is_some());

    nyrt_handle_release_h(owned_handle_2);
    assert!(handles::get(owned_handle_2 as u64).is_none());
}

#[test]
fn handle_abi_borrowed_owned_invalid_handles_are_noop() {
    // Matrix case: invalid handle inputs never produce owned escapes and must not drop unrelated handles.
    let anchor: Arc<dyn NyashBox> = Arc::new(StringBox::new("borrowed-owned-anchor".to_string()));
    let anchor_handle = handles::to_handle_arc(anchor) as i64;
    assert!(anchor_handle > 0);
    assert!(handles::get(anchor_handle as u64).is_some());

    assert_eq!(nyrt_handle_retain_h(-1), 0, "negative handle must be no-op");
    assert_eq!(nyrt_handle_retain_h(0), 0, "zero handle must be no-op");
    assert_eq!(
        nyrt_handle_retain_h(i64::MAX),
        0,
        "unknown handle must not escape"
    );

    nyrt_handle_release_h(-1);
    nyrt_handle_release_h(0);
    nyrt_handle_release_h(i64::MAX);

    assert!(
        handles::get(anchor_handle as u64).is_some(),
        "invalid release path must not drop unrelated handles"
    );

    nyrt_handle_release_h(anchor_handle);
    assert!(handles::get(anchor_handle as u64).is_none());
}
