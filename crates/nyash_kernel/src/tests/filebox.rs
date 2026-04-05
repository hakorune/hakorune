use super::*;

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
    assert_eq!(
        nyash_file_open_hhh_export(filebox_handle, 0, mode_handle),
        0
    );
    assert_eq!(
        nyash_file_open_hhh_export(filebox_handle, path_handle, 0),
        0
    );
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
        let result = nyash_plugin_invoke_by_name_i64(
            filebox_handle,
            method.as_ptr(),
            2,
            path_handle,
            mode_handle,
        );
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
        assert_eq!(
            result, 0,
            "readBytes should no longer use builtin by_name keep"
        );

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
        assert_eq!(
            result, 0,
            "writeBytes should no longer use builtin by_name keep"
        );

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
        let result =
            nyash_plugin_invoke_by_name_i64(inst_handle, method.as_ptr(), 1, field_handle, 0);
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
        let value_handle =
            handles::to_handle_arc(Arc::new(nyash_rust::box_trait::IntegerBox::new(99))) as i64;
        let result = nyash_plugin_invoke_by_name_i64(
            inst_handle,
            method.as_ptr(),
            2,
            field_handle,
            value_handle,
        );
        assert_eq!(
            result, 0,
            "InstanceBox.setField should no longer use builtin by_name keep"
        );
    });
}
