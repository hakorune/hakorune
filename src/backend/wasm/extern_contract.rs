/// Canonical extern-call contract shared by WASM codegen and runtime imports.
/// Keep this list as single source of truth for supported extern call names.
pub(crate) const EXTERN_CALL_MAP: [(&str, &str); 18] = [
    ("env.console.log", "console_log"),
    ("env.console.warn", "console_warn"),
    ("env.console.error", "console_error"),
    ("env.console.info", "console_info"),
    ("env.console.debug", "console_debug"),
    ("env.canvas.fillRect", "canvas_fillRect"),
    ("env.canvas.fillText", "canvas_fillText"),
    ("env.canvas.clear", "canvas_clear"),
    ("env.canvas.strokeRect", "canvas_strokeRect"),
    ("env.canvas.beginPath", "canvas_beginPath"),
    ("env.canvas.arc", "canvas_arc"),
    ("env.canvas.fill", "canvas_fill"),
    ("env.canvas.stroke", "canvas_stroke"),
    ("env.canvas.setFillStyle", "canvas_setFillStyle"),
    ("env.canvas.setStrokeStyle", "canvas_setStrokeStyle"),
    ("env.canvas.setLineWidth", "canvas_setLineWidth"),
    ("env.canvas.fillCircle", "canvas_fillCircle"),
    ("env.canvas.drawLine", "canvas_drawLine"),
];
