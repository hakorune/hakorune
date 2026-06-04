// ---- ExternCall helpers for LLVM lowering ----
// Exported as: nyash.console.log(i8* cstr) -> i64
use crate::c_string::c_string_text;

#[inline]
fn handle_to_text(handle: i64) -> Option<String> {
    nyash_rust::runtime::host_handles::get(handle as u64).map(|obj| obj.to_string_box().value)
}

#[inline]
fn emit_stderr_with_prefix(prefix: &str, handle: i64) {
    if let Some(text) = handle_to_text(handle) {
        eprintln!("{}{}", prefix, text);
    } else {
        eprintln!("{}{}", prefix, handle);
    }
}

#[inline]
fn emit_stderr_with_cstr(prefix: &str, ptr: *const i8) {
    if let Some(text) = c_string_text(ptr) {
        eprintln!("{}{}", prefix, text);
    }
}

macro_rules! emit_handle_logger {
    ($fn_name:ident, $export_name:literal, $prefix:literal) => {
        #[export_name = $export_name]
        pub extern "C" fn $fn_name(handle: i64) -> i64 {
            if handle <= 0 {
                return 0;
            }
            emit_stderr_with_prefix($prefix, handle);
            0
        }
    };
}

#[export_name = "nyash.console.log"]
pub extern "C" fn nyash_console_log_export(ptr: *const i8) -> i64 {
    if let Some(text) = c_string_text(ptr) {
        println!("{}", text);
    }
    0
}

// C alias for generators that emit bare `print(i8*)`.
#[no_mangle]
pub extern "C" fn print(ptr: *const i8) -> i64 {
    nyash_console_log_export(ptr)
}

// Exported as: nyash.console.log_handle(i64 handle) -> i64
#[export_name = "nyash.console.log_handle"]
pub extern "C" fn nyash_console_log_handle(handle: i64) -> i64 {
    if let Some(text) = handle_to_text(handle) {
        println!("{}", text);
    } else {
        // Fallback: handle is an unboxed integer.
        println!("{}", handle);
    }
    0
}

emit_handle_logger!(
    nyash_console_warn_handle,
    "nyash.console.warn_handle",
    "WARN: "
);
emit_handle_logger!(
    nyash_console_error_handle,
    "nyash.console.error_handle",
    "ERROR: "
);
emit_handle_logger!(
    nyash_debug_trace_handle,
    "nyash.debug.trace_handle",
    "TRACE: "
);

// Exported as: nyash.console.warn(i8* cstr) -> i64
#[export_name = "nyash.console.warn"]
pub extern "C" fn nyash_console_warn_export(ptr: *const i8) -> i64 {
    emit_stderr_with_cstr("[warn] ", ptr);
    0
}

// Exported as: nyash.console.error(i8* cstr) -> i64
#[export_name = "nyash.console.error"]
pub extern "C" fn nyash_console_error_export(ptr: *const i8) -> i64 {
    emit_stderr_with_cstr("[error] ", ptr);
    0
}

// Exported as: nyash.debug.trace(i8* cstr) -> i64
#[export_name = "nyash.debug.trace"]
pub extern "C" fn nyash_debug_trace_export(ptr: *const i8) -> i64 {
    emit_stderr_with_cstr("[trace] ", ptr);
    0
}

#[export_name = "nyash.console.readline"]
pub extern "C" fn nyash_console_readline_export() -> *mut i8 {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    while input.ends_with('\n') || input.ends_with('\r') {
        input.pop();
    }
    let mut bytes = input.into_bytes();
    bytes.push(0);
    let boxed = bytes.into_boxed_slice();
    let raw = Box::into_raw(boxed) as *mut u8;
    raw as *mut i8
}
