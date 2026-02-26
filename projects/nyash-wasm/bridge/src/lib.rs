use wasm_bindgen::prelude::*;

const SUPPORTED_METHODS: [&str; 5] = ["log", "warn", "error", "info", "debug"];

#[wasm_bindgen]
pub struct NyashWasm {
    output_id: String,
}

#[wasm_bindgen]
impl NyashWasm {
    #[wasm_bindgen(constructor)]
    pub fn new() -> NyashWasm {
        console_error_panic_hook::set_once();
        NyashWasm {
            output_id: "output".to_string(),
        }
    }

    pub fn version(&self) -> String {
        "Nyash WASM bridge (WSM-G2-min1)".to_string()
    }

    pub fn eval(&self, code: &str) -> Result<String, JsValue> {
        let calls = collect_console_calls(code);
        if calls.is_empty() {
            return Err(JsValue::from_str(
                "unsupported: expected ConsoleBox log/warn/error/info/debug calls",
            ));
        }

        for call in calls {
            emit_console(&call.method, &call.message);
            append_output_line(&self.output_id, &call.method, &call.message);
        }
        Ok("ok".to_string())
    }
}

#[derive(Clone)]
struct ConsoleCall {
    index: usize,
    method: String,
    message: String,
}

fn collect_console_calls(code: &str) -> Vec<ConsoleCall> {
    let mut calls = Vec::new();
    for method in SUPPORTED_METHODS {
        let needle = format!("console.{}(", method);
        let mut start = 0usize;
        while let Some(rel_idx) = code[start..].find(&needle) {
            let absolute = start + rel_idx;
            let arg_start = absolute + needle.len();
            if let Some((msg, consumed)) = parse_first_string_arg(&code[arg_start..]) {
                calls.push(ConsoleCall {
                    index: absolute,
                    method: method.to_string(),
                    message: msg,
                });
                start = arg_start + consumed;
            } else {
                start = arg_start;
            }
        }
    }
    calls.sort_by_key(|call| call.index);
    calls
}

fn parse_first_string_arg(input: &str) -> Option<(String, usize)> {
    let bytes = input.as_bytes();
    if bytes.is_empty() {
        return None;
    }
    let quote = bytes[0];
    if quote != b'"' && quote != b'\'' {
        return None;
    }

    let mut i = 1usize;
    let mut escaped = false;
    let mut out = String::new();
    while i < bytes.len() {
        let b = bytes[i];
        if escaped {
            out.push(b as char);
            escaped = false;
            i += 1;
            continue;
        }
        if b == b'\\' {
            escaped = true;
            i += 1;
            continue;
        }
        if b == quote {
            return Some((out, i + 1));
        }
        out.push(b as char);
        i += 1;
    }
    None
}

fn emit_console(method: &str, message: &str) {
    let value = JsValue::from_str(message);
    match method {
        "log" => web_sys::console::log_1(&value),
        "warn" => web_sys::console::warn_1(&value),
        "error" => web_sys::console::error_1(&value),
        "info" => web_sys::console::info_1(&value),
        "debug" => web_sys::console::debug_1(&value),
        _ => web_sys::console::warn_1(&JsValue::from_str("unsupported console method")),
    }
}

fn append_output_line(output_id: &str, method: &str, message: &str) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };
    let Some(element) = document.get_element_by_id(output_id) else {
        return;
    };

    let mut next = element.text_content().unwrap_or_default();
    if !next.is_empty() && !next.ends_with('\n') {
        next.push('\n');
    }
    next.push_str(&format!("[{}] {}", method, message));
    element.set_text_content(Some(&next));
}
