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
        for prefix in ["console", "me.console"] {
            let needle = format!("{}.{}(", prefix, method);
            let mut start = 0usize;
            while let Some(rel_idx) = code[start..].find(&needle) {
                let absolute = start + rel_idx;
                if absolute > 0 {
                    let prev = code.as_bytes()[absolute - 1] as char;
                    if prev.is_ascii_alphanumeric() || prev == '_' || prev == '.' {
                        start = absolute + 1;
                        continue;
                    }
                }
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
    }
    calls.sort_by_key(|call| call.index);
    calls
}

fn parse_first_string_arg(input: &str) -> Option<(String, usize)> {
    let mut chars = input.char_indices();
    let (_, quote) = chars.next()?;
    if quote != '"' && quote != '\'' {
        return None;
    }

    let mut out = String::new();
    let mut escaped = false;
    for (idx, ch) in chars {
        if escaped {
            match ch {
                'n' => out.push('\n'),
                'r' => out.push('\r'),
                't' => out.push('\t'),
                '\\' => out.push('\\'),
                '"' => out.push('"'),
                '\'' => out.push('\''),
                other => out.push(other),
            }
            escaped = false;
            continue;
        }
        if ch == '\\' {
            escaped = true;
            continue;
        }
        if ch == quote {
            return Some((out, idx + ch.len_utf8()));
        }
        out.push(ch);
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

#[cfg(test)]
mod tests {
    use super::collect_console_calls;

    #[test]
    fn collect_console_calls_accepts_console_and_me_console_contract() {
        let code = r#"
            static box Main {
                main() {
                    local console
                    console = new ConsoleBox()
                    console.log("a")
                    me.console.warn("b")
                    return 0
                }
            }
        "#;
        let calls = collect_console_calls(code);
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0].method, "log");
        assert_eq!(calls[0].message, "a");
        assert_eq!(calls[1].method, "warn");
        assert_eq!(calls[1].message, "b");
    }
}
