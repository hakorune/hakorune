use super::*;

pub(super) fn handle_py_object(
    method_id: u32,
    instance_id: u32,
    _args: *const u8,
    _args_len: usize,
    result: *mut u8,
    result_len: *mut usize,
) -> i32 {
    match method_id {
        PYO_METHOD_BIRTH => NYB_E_INVALID_METHOD,
        PYO_METHOD_FINI => {
            PY_HANDLES.lock().unwrap().remove(&instance_id);
            if let Ok(mut map) = PYOBJS.lock() {
                map.remove(&instance_id);
                NYB_SUCCESS
            } else {
                NYB_E_PLUGIN_ERROR
            }
        }
        PYO_METHOD_GETATTR | PYO_METHOD_GETATTR_R => {
            if ensure_cpython().is_err() {
                return NYB_E_PLUGIN_ERROR;
            }
            let Some(name) = read_arg_string(_args, _args_len, 0) else {
                return NYB_E_INVALID_ARGS;
            };
            if let Some(cpy) = &*ffi::CPY.lock().unwrap() {
                let obj_ptr = {
                    let guard = PY_HANDLES.lock().unwrap();
                    let Some(handle) = guard.get(&instance_id) else {
                        return NYB_E_INVALID_HANDLE;
                    };
                    handle.as_ptr()
                };
                let c_name = match pytypes::cstring_from_str(&name) {
                    Ok(s) => s,
                    Err(_) => return NYB_E_INVALID_ARGS,
                };
                let _gil = gil::GILGuard::acquire(cpy);
                let attr = unsafe { (cpy.PyObject_GetAttrString)(obj_ptr, c_name.as_ptr()) };
                if attr.is_null() {
                    let msg = pytypes::take_py_error_string(cpy);
                    if method_id == PYO_METHOD_GETATTR_R {
                        return NYB_E_PLUGIN_ERROR;
                    }
                    if let Some(m) = msg {
                        return write_tlv_string(&m, result, result_len);
                    }
                    return NYB_E_PLUGIN_ERROR;
                }
                if should_autodecode() {
                    if let Some(decoded) = pytypes::autodecode(cpy, attr) {
                        if write_autodecode_result(&decoded, result, result_len) {
                            unsafe {
                                (cpy.Py_DecRef)(attr);
                            }
                            return NYB_SUCCESS;
                        }
                    }
                }
                let id = OBJ_COUNTER.fetch_add(1, Ordering::Relaxed);
                let owned = unsafe { PyOwned::from_new(attr).expect("non-null PyObject") };
                PY_HANDLES.lock().unwrap().insert(id, owned);
                return write_tlv_handle(TYPE_ID_PY_OBJECT, id, result, result_len);
            }
            NYB_E_PLUGIN_ERROR
        }
        PYO_METHOD_CALL | PYO_METHOD_CALL_R => {
            if ensure_cpython().is_err() {
                return NYB_E_PLUGIN_ERROR;
            }
            if let Some(cpy) = &*ffi::CPY.lock().unwrap() {
                let func_ptr = {
                    let guard = PY_HANDLES.lock().unwrap();
                    let Some(handle) = guard.get(&instance_id) else {
                        return NYB_E_INVALID_HANDLE;
                    };
                    handle.as_ptr()
                };
                let _gil = gil::GILGuard::acquire(cpy);
                let tuple = match pytypes::tuple_from_tlv(cpy, _args, _args_len) {
                    Ok(t) => t,
                    Err(_) => return NYB_E_INVALID_ARGS,
                };
                let ret = unsafe { (cpy.PyObject_CallObject)(func_ptr, tuple) };
                unsafe {
                    (cpy.Py_DecRef)(tuple);
                }
                if ret.is_null() {
                    let msg = pytypes::take_py_error_string(cpy);
                    if method_id == PYO_METHOD_CALL_R {
                        return NYB_E_PLUGIN_ERROR;
                    }
                    if let Some(m) = msg {
                        return write_tlv_string(&m, result, result_len);
                    }
                    return NYB_E_PLUGIN_ERROR;
                }
                if should_autodecode() {
                    if let Some(decoded) = pytypes::autodecode(cpy, ret) {
                        if write_autodecode_result(&decoded, result, result_len) {
                            unsafe {
                                (cpy.Py_DecRef)(ret);
                            }
                            return NYB_SUCCESS;
                        }
                    }
                }
                let id = OBJ_COUNTER.fetch_add(1, Ordering::Relaxed);
                let owned = unsafe { PyOwned::from_new(ret).expect("non-null PyObject") };
                PY_HANDLES.lock().unwrap().insert(id, owned);
                return write_tlv_handle(TYPE_ID_PY_OBJECT, id, result, result_len);
            }
            NYB_E_PLUGIN_ERROR
        }
        PYO_METHOD_CALL_KW | PYO_METHOD_CALL_KW_R => {
            if ensure_cpython().is_err() {
                return NYB_E_PLUGIN_ERROR;
            }
            if let Some(cpy) = &*ffi::CPY.lock().unwrap() {
                let func_ptr = {
                    let guard = PY_HANDLES.lock().unwrap();
                    let Some(handle) = guard.get(&instance_id) else {
                        return NYB_E_INVALID_HANDLE;
                    };
                    handle.as_ptr()
                };
                let _gil = gil::GILGuard::acquire(cpy);
                let args_tup = unsafe { (cpy.PyTuple_New)(0) };
                if args_tup.is_null() {
                    return NYB_E_PLUGIN_ERROR;
                }
                let kwargs = match pytypes::kwargs_from_tlv(cpy, _args, _args_len) {
                    Ok(d) => d,
                    Err(_) => {
                        unsafe {
                            (cpy.Py_DecRef)(args_tup);
                        }
                        return NYB_E_INVALID_ARGS;
                    }
                };
                let ret = unsafe { (cpy.PyObject_Call)(func_ptr, args_tup, kwargs) };
                unsafe {
                    (cpy.Py_DecRef)(kwargs);
                    (cpy.Py_DecRef)(args_tup);
                }
                if ret.is_null() {
                    let msg = pytypes::take_py_error_string(cpy);
                    if method_id == PYO_METHOD_CALL_KW_R {
                        return NYB_E_PLUGIN_ERROR;
                    }
                    if let Some(m) = msg {
                        return write_tlv_string(&m, result, result_len);
                    }
                    return NYB_E_PLUGIN_ERROR;
                }
                if (method_id == PYO_METHOD_CALL_KW || method_id == PYO_METHOD_CALL_KW_R)
                    && should_autodecode()
                {
                    if let Some(decoded) = pytypes::autodecode(cpy, ret) {
                        if write_autodecode_result(&decoded, result, result_len) {
                            unsafe {
                                (cpy.Py_DecRef)(ret);
                            }
                            return NYB_SUCCESS;
                        }
                    }
                }
                let id = OBJ_COUNTER.fetch_add(1, Ordering::Relaxed);
                let owned = unsafe { PyOwned::from_new(ret).expect("non-null PyObject") };
                PY_HANDLES.lock().unwrap().insert(id, owned);
                return write_tlv_handle(TYPE_ID_PY_OBJECT, id, result, result_len);
            }
            NYB_E_PLUGIN_ERROR
        }
        PYO_METHOD_STR => {
            if ensure_cpython().is_err() {
                return NYB_E_PLUGIN_ERROR;
            }
            if let Some(cpy) = &*ffi::CPY.lock().unwrap() {
                let obj_ptr = {
                    let guard = PY_HANDLES.lock().unwrap();
                    let Some(handle) = guard.get(&instance_id) else {
                        return NYB_E_INVALID_HANDLE;
                    };
                    handle.as_ptr()
                };
                let _gil = gil::GILGuard::acquire(cpy);
                let s_obj = unsafe { (cpy.PyObject_Str)(obj_ptr) };
                if s_obj.is_null() {
                    return NYB_E_PLUGIN_ERROR;
                }
                let rust_str = unsafe {
                    let cstr = (cpy.PyUnicode_AsUTF8)(s_obj);
                    match pytypes::cstr_to_string(cstr) {
                        Some(s) => s,
                        None => {
                            (cpy.Py_DecRef)(s_obj);
                            return NYB_E_PLUGIN_ERROR;
                        }
                    }
                };
                unsafe {
                    (cpy.Py_DecRef)(s_obj);
                }
                return write_tlv_string(&rust_str, result, result_len);
            }
            NYB_E_PLUGIN_ERROR
        }
        _ => NYB_E_INVALID_METHOD,
    }
}
