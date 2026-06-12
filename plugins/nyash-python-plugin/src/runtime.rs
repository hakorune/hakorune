use super::*;

pub(super) fn handle_py_runtime(
    method_id: u32,
    _instance_id: u32,
    _args: *const u8,
    _args_len: usize,
    result: *mut u8,
    result_len: *mut usize,
) -> i32 {
    unsafe {
        match method_id {
            PY_METHOD_BIRTH => {
                if result_len.is_null() {
                    return NYB_E_INVALID_ARGS;
                }
                if preflight(result, result_len, 4) {
                    return NYB_E_SHORT_BUFFER;
                }
                if ensure_cpython().is_err() {
                    return NYB_E_PLUGIN_ERROR;
                }
                let id = RT_COUNTER.fetch_add(1, Ordering::Relaxed);
                let mut inst = PyRuntimeInstance::default();
                if let Some(cpy) = &*ffi::CPY.lock().unwrap() {
                    let c_main = pytypes::cstring_from_str("__main__").expect("literal __main__");
                    let _gil = gil::GILGuard::acquire(cpy);
                    let module = (cpy.PyImport_AddModule)(c_main.as_ptr());
                    if !module.is_null() {
                        let dict = (cpy.PyModule_GetDict)(module);
                        if !dict.is_null() {
                            inst.globals = Some(dict);
                        }
                    }
                }
                if let Ok(mut map) = RUNTIMES.lock() {
                    map.insert(id, inst);
                } else {
                    return NYB_E_PLUGIN_ERROR;
                }
                let bytes = id.to_le_bytes();
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), result, 4);
                *result_len = 4;
                NYB_SUCCESS
            }
            PY_METHOD_FINI => {
                if let Ok(mut map) = RUNTIMES.lock() {
                    map.remove(&_instance_id);
                }
                NYB_SUCCESS
            }
            PY_METHOD_EVAL | PY_METHOD_EVAL_R => {
                if ensure_cpython().is_err() {
                    return NYB_E_PLUGIN_ERROR;
                }
                let argc = pytypes::count_tlv_args(_args, _args_len);
                let code = if argc == 0 {
                    std::env::var("NYASH_PY_EVAL_CODE").unwrap_or_else(|_| "".to_string())
                } else {
                    if let Some(s) = read_arg_string(_args, _args_len, 0) {
                        s
                    } else {
                        return NYB_E_INVALID_ARGS;
                    }
                };
                let c_code = match pytypes::cstring_from_str(&code) {
                    Ok(s) => s,
                    Err(_) => return NYB_E_INVALID_ARGS,
                };
                if let Some(cpy) = &*ffi::CPY.lock().unwrap() {
                    let _gil = gil::GILGuard::acquire(cpy);
                    let mut dict: *mut PyObject = std::ptr::null_mut();
                    if let Ok(map) = RUNTIMES.lock() {
                        if let Some(rt) = map.get(&_instance_id) {
                            if let Some(g) = rt.globals {
                                dict = g;
                            }
                        }
                    }
                    if dict.is_null() {
                        let c_main =
                            pytypes::cstring_from_str("__main__").expect("literal __main__");
                        let module = (cpy.PyImport_AddModule)(c_main.as_ptr());
                        if module.is_null() {
                            return NYB_E_PLUGIN_ERROR;
                        }
                        dict = (cpy.PyModule_GetDict)(module);
                    }
                    let obj = (cpy.PyRun_StringFlags)(
                        c_code.as_ptr(),
                        258,
                        dict,
                        dict,
                        std::ptr::null_mut(),
                    );
                    if obj.is_null() {
                        let msg = pytypes::take_py_error_string(cpy);
                        if method_id == PY_METHOD_EVAL_R {
                            return NYB_E_PLUGIN_ERROR;
                        }
                        if let Some(m) = msg {
                            return write_tlv_string(&m, result, result_len);
                        }
                        return NYB_E_PLUGIN_ERROR;
                    }
                    if (method_id == PY_METHOD_EVAL || method_id == PY_METHOD_EVAL_R)
                        && should_autodecode()
                    {
                        if let Some(decoded) = pytypes::autodecode(cpy, obj) {
                            if write_autodecode_result(&decoded, result, result_len) {
                                (cpy.Py_DecRef)(obj);
                                return NYB_SUCCESS;
                            }
                        }
                    }
                    let id = OBJ_COUNTER.fetch_add(1, Ordering::Relaxed);
                    if let Ok(mut map) = PYOBJS.lock() {
                        map.insert(id, PyObjectInstance::default());
                    } else {
                        (cpy.Py_DecRef)(obj);
                        return NYB_E_PLUGIN_ERROR;
                    }
                    let owned = PyOwned::from_new(obj).expect("non-null PyObject");
                    PY_HANDLES.lock().unwrap().insert(id, owned);
                    return write_tlv_handle(TYPE_ID_PY_OBJECT, id, result, result_len);
                }
                NYB_E_PLUGIN_ERROR
            }
            PY_METHOD_IMPORT | PY_METHOD_IMPORT_R => {
                if ensure_cpython().is_err() {
                    return NYB_E_PLUGIN_ERROR;
                }
                let Some(name) = read_arg_string(_args, _args_len, 0) else {
                    return NYB_E_INVALID_ARGS;
                };
                let c_name = match pytypes::cstring_from_str(&name) {
                    Ok(s) => s,
                    Err(_) => return NYB_E_INVALID_ARGS,
                };
                if let Some(cpy) = &*ffi::CPY.lock().unwrap() {
                    let _gil = gil::GILGuard::acquire(cpy);
                    let obj = (cpy.PyImport_ImportModule)(c_name.as_ptr());
                    if obj.is_null() {
                        let msg = pytypes::take_py_error_string(cpy);
                        if method_id == PY_METHOD_IMPORT_R {
                            return NYB_E_PLUGIN_ERROR;
                        }
                        if let Some(m) = msg {
                            return write_tlv_string(&m, result, result_len);
                        }
                        return NYB_E_PLUGIN_ERROR;
                    }
                    if let Ok(map) = RUNTIMES.lock() {
                        if let Some(rt) = map.get(&_instance_id) {
                            if let Some(gl) = rt.globals {
                                (cpy.PyDict_SetItemString)(gl, c_name.as_ptr(), obj);
                            }
                        }
                    }
                    let id = OBJ_COUNTER.fetch_add(1, Ordering::Relaxed);
                    if let Ok(mut map) = PYOBJS.lock() {
                        map.insert(id, PyObjectInstance::default());
                    } else {
                        (cpy.Py_DecRef)(obj);
                        return NYB_E_PLUGIN_ERROR;
                    }
                    let owned = PyOwned::from_new(obj).expect("non-null PyObject");
                    PY_HANDLES.lock().unwrap().insert(id, owned);
                    return write_tlv_handle(TYPE_ID_PY_OBJECT, id, result, result_len);
                }
                NYB_E_PLUGIN_ERROR
            }
            _ => NYB_E_INVALID_METHOD,
        }
    }
}
