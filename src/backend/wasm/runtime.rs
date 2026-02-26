/*!
 * WASM Runtime Imports - External function imports for WASM modules
 *
 * Phase 8.2 PoC1: Implements env.print import for debugging
 * Future: Additional runtime functions (file I/O, network, etc.)
 */

use super::WasmError;
use crate::backend::wasm::extern_contract::EXTERN_CALL_MAP;

/// Runtime import definitions for WASM modules
pub struct RuntimeImports {
    /// Available imports
    imports: Vec<ImportFunction>,
}

/// External function import definition
#[derive(Debug, Clone)]
pub struct ImportFunction {
    pub module: String,
    pub name: String,
    pub params: Vec<String>,
    pub result: Option<String>,
}

impl RuntimeImports {
    pub fn new() -> Self {
        let mut runtime = Self {
            imports: Vec::new(),
        };

        // Add standard runtime imports
        runtime.add_standard_imports();
        runtime
    }

    /// Add standard runtime function imports
    fn add_standard_imports(&mut self) {
        // env.print for debugging output
        self.push_env_i32_import("print", 1, None);
        // env.print_str for string debugging (ptr, len)
        self.push_env_i32_import("print_str", 2, None);

        // Phase 9.7: Box FFI/ABI imports per BID specifications

        // ExternCall imports shared with codegen contract.
        // Console imports are fixed (ptr,len). Canvas imports are map-driven by arity.
        for (_, import_name) in EXTERN_CALL_MAP {
            if matches!(
                import_name,
                "console_log"
                    | "console_warn"
                    | "console_error"
                    | "console_info"
                    | "console_debug"
            ) {
                self.push_env_i32_import(import_name, 2, None);
                continue;
            }
            if let Some(arity) = canvas_import_arity(import_name) {
                self.push_env_i32_import(import_name, arity, None);
            }
        }

        // Phase 9.77: BoxCall runtime functions

        // box_to_string - Convert any Box to string representation
        self.push_env_i32_import("box_to_string", 1, Some("i32"));

        // box_print - Print any Box to console
        self.push_env_i32_import("box_print", 1, None);

        // box_equals - Compare two Boxes for equality
        self.push_env_i32_import("box_equals", 2, Some("i32"));

        // box_clone - Clone a Box
        self.push_env_i32_import("box_clone", 1, Some("i32"));

        // Future: env.file_read, env.file_write for file I/O
        // Future: env.http_request for network access
    }

    fn push_env_i32_import(&mut self, name: &str, param_count: usize, result: Option<&str>) {
        self.imports.push(ImportFunction {
            module: "env".to_string(),
            name: name.to_string(),
            params: vec!["i32".to_string(); param_count],
            result: result.map(str::to_string),
        });
    }

    /// Get all import declarations in WAT format
    pub fn get_imports(&self) -> Vec<String> {
        self.imports
            .iter()
            .map(|import| {
                let params = if import.params.is_empty() {
                    String::new()
                } else {
                    format!("(param {})", import.params.join(" "))
                };

                let result = if let Some(ref result_type) = import.result {
                    format!("(result {})", result_type)
                } else {
                    String::new()
                };

                format!(
                    "(import \"{}\" \"{}\" (func ${} {} {}))",
                    import.module, import.name, import.name, params, result
                )
            })
            .collect()
    }

    /// Add custom import function
    pub fn add_import(
        &mut self,
        module: String,
        name: String,
        params: Vec<String>,
        result: Option<String>,
    ) {
        self.imports.push(ImportFunction {
            module,
            name,
            params,
            result,
        });
    }

    /// Check if an import is available
    pub fn has_import(&self, name: &str) -> bool {
        self.imports.iter().any(|import| import.name == name)
    }

    /// Get import function by name
    pub fn get_import(&self, name: &str) -> Option<&ImportFunction> {
        self.imports.iter().find(|import| import.name == name)
    }

    /// Generate JavaScript import object for browser execution
    pub fn get_js_import_object(&self) -> String {
        let mut js = String::new();
        js.push_str("const importObject = {\n");

        // Group by module
        let mut modules: std::collections::HashMap<String, Vec<&ImportFunction>> =
            std::collections::HashMap::new();
        for import in &self.imports {
            modules
                .entry(import.module.clone())
                .or_default()
                .push(import);
        }

        for (module_name, functions) in modules {
            js.push_str(&format!("  {}: {{\n", module_name));

            for function in functions {
                if let Some(binding) = js_binding_for_import(&function.name) {
                    js.push_str(&binding);
                } else {
                    js.push_str(&format!(
                        "    {}: () => {{ throw new Error('Not implemented: {}'); }},\n",
                        function.name, function.name
                    ));
                }
            }

            js.push_str("  },\n");
        }

        js.push_str("};\n");
        js
    }

    /// Generate Rust wasmtime import bindings
    pub fn get_wasmtime_imports(&self) -> Result<String, WasmError> {
        let mut rust_code = String::new();
        rust_code.push_str("// Wasmtime import bindings\n");
        rust_code.push_str("let mut imports = Vec::new();\n\n");

        for import in &self.imports {
            match import.name.as_str() {
                "print" => {
                    rust_code.push_str(
                        r#"
let print_func = wasmtime::Func::wrap(&mut store, |value: i32| {
    println!("{}", value);
});
imports.push(print_func.into());
"#,
                    );
                }
                _ => {
                    rust_code.push_str(&format!(
                        r#"
// TODO: Implement {} import
let {}_func = wasmtime::Func::wrap(&mut store, || {{
    panic!("Not implemented: {}")
}});
imports.push({}_func.into());
"#,
                        import.name, import.name, import.name, import.name
                    ));
                }
            }
        }

        Ok(rust_code)
    }
}

fn js_binding_for_import(name: &str) -> Option<String> {
    match name {
        "print" => Some("    print: (value) => console.log(value),\n".to_string()),
        "print_str" => Some(js_console_binding("print_str", "log")),
        "console_log" => Some(js_console_binding("console_log", "log")),
        "console_warn" => Some(js_console_binding("console_warn", "warn")),
        "console_error" => Some(js_console_binding("console_error", "error")),
        "console_info" => Some(js_console_binding("console_info", "info")),
        "console_debug" => Some(js_console_binding("console_debug", "debug")),
        "canvas_fillRect" => Some(js_canvas_fill_rect_binding()),
        "canvas_fillText" => Some(js_canvas_fill_text_binding()),
        "canvas_clear" => Some(js_canvas_clear_binding()),
        "canvas_strokeRect" => Some(js_canvas_stroke_rect_binding()),
        "canvas_beginPath" => Some(js_canvas_begin_path_binding()),
        "canvas_arc" => Some(js_canvas_arc_binding()),
        "canvas_fill" => Some(js_canvas_fill_binding()),
        "canvas_stroke" => Some(js_canvas_stroke_binding()),
        "canvas_setFillStyle" => Some(js_canvas_set_fill_style_binding()),
        "canvas_setStrokeStyle" => Some(js_canvas_set_stroke_style_binding()),
        "canvas_setLineWidth" => Some(js_canvas_set_line_width_binding()),
        _ => None,
    }
}

fn js_console_binding(import_name: &str, console_method: &str) -> String {
    format!(
        "    {import_name}: (ptr, len) => {{\n      const memory = instance.exports.memory;\n      const str = new TextDecoder().decode(new Uint8Array(memory.buffer, ptr, len));\n      console.{console_method}(str);\n    }},\n"
    )
}

fn js_canvas_ctx_binding(
    import_name: &str,
    params: &str,
    extra_setup: &str,
    ctx_call: &str,
) -> String {
    format!(
        "    {import_name}: ({params}) => {{\n      const memory = instance.exports.memory;\n      const canvasId = new TextDecoder().decode(new Uint8Array(memory.buffer, canvasIdPtr, canvasIdLen));\n      const canvas = document.getElementById(canvasId);\n      if (canvas) {{\n        const ctx = canvas.getContext('2d');\n{extra_setup}        {ctx_call};\n      }}\n    }},\n"
    )
}

fn js_canvas_fill_rect_binding() -> String {
    js_canvas_ctx_binding(
        "canvas_fillRect",
        "canvasIdPtr, canvasIdLen, x, y, w, h, colorPtr, colorLen",
        "        const color = new TextDecoder().decode(new Uint8Array(memory.buffer, colorPtr, colorLen));\n        ctx.fillStyle = color;\n",
        "ctx.fillRect(x, y, w, h)",
    )
}

fn js_canvas_fill_text_binding() -> String {
    js_canvas_ctx_binding(
        "canvas_fillText",
        "canvasIdPtr, canvasIdLen, textPtr, textLen, x, y, fontPtr, fontLen, colorPtr, colorLen",
        "        const text = new TextDecoder().decode(new Uint8Array(memory.buffer, textPtr, textLen));\n        const font = new TextDecoder().decode(new Uint8Array(memory.buffer, fontPtr, fontLen));\n        const color = new TextDecoder().decode(new Uint8Array(memory.buffer, colorPtr, colorLen));\n        ctx.font = font;\n        ctx.fillStyle = color;\n",
        "ctx.fillText(text, x, y)",
    )
}

fn js_canvas_clear_binding() -> String {
    js_canvas_ctx_binding(
        "canvas_clear",
        "canvasIdPtr, canvasIdLen",
        "",
        "ctx.clearRect(0, 0, canvas.width, canvas.height)",
    )
}

fn js_canvas_stroke_rect_binding() -> String {
    js_canvas_ctx_binding(
        "canvas_strokeRect",
        "canvasIdPtr, canvasIdLen, x, y, w, h, colorPtr, colorLen",
        "        const color = new TextDecoder().decode(new Uint8Array(memory.buffer, colorPtr, colorLen));\n        ctx.strokeStyle = color;\n",
        "ctx.strokeRect(x, y, w, h)",
    )
}

fn js_canvas_begin_path_binding() -> String {
    js_canvas_ctx_binding("canvas_beginPath", "canvasIdPtr, canvasIdLen", "", "ctx.beginPath()")
}

fn js_canvas_arc_binding() -> String {
    js_canvas_ctx_binding(
        "canvas_arc",
        "canvasIdPtr, canvasIdLen, x, y, radius, startAngle, endAngle",
        "",
        "ctx.arc(x, y, radius, startAngle, endAngle)",
    )
}

fn js_canvas_fill_binding() -> String {
    js_canvas_ctx_binding("canvas_fill", "canvasIdPtr, canvasIdLen", "", "ctx.fill()")
}

fn js_canvas_stroke_binding() -> String {
    js_canvas_ctx_binding("canvas_stroke", "canvasIdPtr, canvasIdLen", "", "ctx.stroke()")
}

fn js_canvas_set_fill_style_binding() -> String {
    js_canvas_ctx_binding(
        "canvas_setFillStyle",
        "canvasIdPtr, canvasIdLen, colorPtr, colorLen",
        "        const color = new TextDecoder().decode(new Uint8Array(memory.buffer, colorPtr, colorLen));\n",
        "ctx.fillStyle = color",
    )
}

fn js_canvas_set_stroke_style_binding() -> String {
    js_canvas_ctx_binding(
        "canvas_setStrokeStyle",
        "canvasIdPtr, canvasIdLen, colorPtr, colorLen",
        "        const color = new TextDecoder().decode(new Uint8Array(memory.buffer, colorPtr, colorLen));\n",
        "ctx.strokeStyle = color",
    )
}

fn js_canvas_set_line_width_binding() -> String {
    js_canvas_ctx_binding(
        "canvas_setLineWidth",
        "canvasIdPtr, canvasIdLen, width",
        "",
        "ctx.lineWidth = width",
    )
}

fn canvas_import_arity(import_name: &str) -> Option<usize> {
    match import_name {
        "canvas_fillRect" | "canvas_strokeRect" => Some(8),
        "canvas_fillText" => Some(10),
        "canvas_clear" | "canvas_beginPath" | "canvas_fill" | "canvas_stroke" => Some(2),
        "canvas_arc" => Some(7),
        "canvas_setFillStyle" => Some(4),
        "canvas_setStrokeStyle" => Some(4),
        "canvas_setLineWidth" => Some(3),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_imports_creation() {
        let runtime = RuntimeImports::new();
        assert!(!runtime.imports.is_empty());
        assert!(runtime.has_import("print"));
        assert!(runtime.has_import("console_warn"));
        assert!(runtime.has_import("console_error"));
        assert!(runtime.has_import("console_info"));
        assert!(runtime.has_import("console_debug"));
        assert!(runtime.has_import("canvas_clear"));
        assert!(runtime.has_import("canvas_strokeRect"));
        assert!(runtime.has_import("canvas_beginPath"));
        assert!(runtime.has_import("canvas_arc"));
        assert!(runtime.has_import("canvas_fill"));
        assert!(runtime.has_import("canvas_stroke"));
        assert!(runtime.has_import("canvas_setFillStyle"));
        assert!(runtime.has_import("canvas_setStrokeStyle"));
        assert!(runtime.has_import("canvas_setLineWidth"));
    }

    #[test]
    fn test_import_wat_generation() {
        let runtime = RuntimeImports::new();
        let imports = runtime.get_imports();

        assert!(!imports.is_empty());
        assert!(imports[0].contains("import"));
        assert!(imports[0].contains("env"));
        assert!(imports[0].contains("print"));
    }

    #[test]
    fn test_custom_import_addition() {
        let mut runtime = RuntimeImports::new();
        runtime.add_import(
            "custom".to_string(),
            "test_func".to_string(),
            vec!["i32".to_string(), "i32".to_string()],
            Some("i32".to_string()),
        );

        assert!(runtime.has_import("test_func"));
        let import = runtime.get_import("test_func").unwrap();
        assert_eq!(import.module, "custom");
        assert_eq!(import.params.len(), 2);
        assert!(import.result.is_some());
    }

    #[test]
    fn test_js_import_object_generation() {
        let runtime = RuntimeImports::new();
        let js = runtime.get_js_import_object();

        assert!(js.contains("importObject"));
        assert!(js.contains("env"));
        assert!(js.contains("print"));
        assert!(js.contains("console.log"));
        assert!(js.contains("console.warn"));
        assert!(js.contains("console.error"));
        assert!(js.contains("console.info"));
        assert!(js.contains("console.debug"));
        assert!(js.contains("canvas_clear"));
        assert!(js.contains("canvas_strokeRect"));
        assert!(js.contains("canvas_beginPath"));
        assert!(js.contains("canvas_arc"));
        assert!(js.contains("canvas_fill"));
        assert!(js.contains("canvas_stroke"));
        assert!(js.contains("canvas_setFillStyle"));
        assert!(js.contains("canvas_setStrokeStyle"));
        assert!(js.contains("canvas_setLineWidth"));
        assert!(js.contains("clearRect"));
        assert!(js.contains("strokeRect"));
        assert!(js.contains("beginPath"));
        assert!(js.contains("arc("));
        assert!(js.contains("ctx.fill"));
        assert!(js.contains("ctx.stroke"));
        assert!(js.contains("ctx.fillStyle"));
        assert!(js.contains("ctx.strokeStyle"));
        assert!(js.contains("ctx.lineWidth"));
    }

    #[test]
    fn runtime_imports_canvas_stroke_rect_js_binding() {
        let runtime = RuntimeImports::new();
        let js = runtime.get_js_import_object();
        assert!(js.contains("canvas_strokeRect"));
        assert!(js.contains("strokeStyle"));
        assert!(js.contains("strokeRect"));
    }

    #[test]
    fn runtime_imports_canvas_begin_path_js_binding() {
        let runtime = RuntimeImports::new();
        let js = runtime.get_js_import_object();
        assert!(js.contains("canvas_beginPath"));
        assert!(js.contains("beginPath"));
    }

    #[test]
    fn runtime_imports_canvas_arc_js_binding() {
        let runtime = RuntimeImports::new();
        let js = runtime.get_js_import_object();
        assert!(js.contains("canvas_arc"));
        assert!(js.contains("ctx.arc"));
    }

    #[test]
    fn runtime_imports_canvas_fill_js_binding() {
        let runtime = RuntimeImports::new();
        let js = runtime.get_js_import_object();
        assert!(js.contains("canvas_fill"));
        assert!(js.contains("ctx.fill"));
    }

    #[test]
    fn runtime_imports_canvas_stroke_js_binding() {
        let runtime = RuntimeImports::new();
        let js = runtime.get_js_import_object();
        assert!(js.contains("canvas_stroke"));
        assert!(js.contains("ctx.stroke"));
    }

    #[test]
    fn runtime_imports_canvas_set_fill_style_js_binding() {
        let runtime = RuntimeImports::new();
        let js = runtime.get_js_import_object();
        assert!(js.contains("canvas_setFillStyle"));
        assert!(js.contains("ctx.fillStyle"));
    }

    #[test]
    fn runtime_imports_canvas_set_stroke_style_js_binding() {
        let runtime = RuntimeImports::new();
        let js = runtime.get_js_import_object();
        assert!(js.contains("canvas_setStrokeStyle"));
        assert!(js.contains("ctx.strokeStyle"));
    }

    #[test]
    fn runtime_imports_canvas_set_line_width_js_binding() {
        let runtime = RuntimeImports::new();
        let js = runtime.get_js_import_object();
        assert!(js.contains("canvas_setLineWidth"));
        assert!(js.contains("ctx.lineWidth"));
    }

    #[test]
    fn test_wasmtime_imports_generation() {
        let runtime = RuntimeImports::new();
        let rust_code = runtime.get_wasmtime_imports().unwrap();

        assert!(rust_code.contains("wasmtime::Func::wrap"));
        assert!(rust_code.contains("print_func"));
        assert!(rust_code.contains("println!"));
    }
}
