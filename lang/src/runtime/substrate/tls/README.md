# lang/src/runtime/substrate/tls — TLS Capability Staging

Responsibilities:
- Current home for the helper-shaped first `hako.tls` row.
- Future home for:
  - language-level `thread_local` / `TlsCell<T>` substrate vocabulary
  - thread/task-local slot lowering
  - cache-slot primitive
  - locality-facing substrate vocabulary

Rules:
- `tls` is capability substrate, not semantic owner.
- current live subset is intentionally helper-shaped, not final-form generic TLS.

Current live subset:
- `tls_core_box.hako`
  - `last_error_text_h()`
  - `last_error_is_ok_i64()`
  - `last_error_code_i64()`
  - truthful row over diagnostics TLS (`hako_last_error`)
  - status helpers are diagnostics-only; they do not expose generic TLS slots

Non-goals:
- No raw numeric TLS slot API in this wave.
- No thread/task-local slot API in this wave.
- No cache-slot primitive in this wave.
- No allocator policy here.
- No final platform TLS fallback here.
- No GC barrier logic here.
