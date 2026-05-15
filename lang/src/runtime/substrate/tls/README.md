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
  - `cache_slot_get_i64(slot)`
  - `cache_slot_set_i64(slot, value)`
  - truthful row over diagnostics TLS (`hako_last_error`)
  - status helpers are diagnostics-only; they do not expose generic TLS slots
  - cache-slot helpers are allocator-substrate rows; they do not expose generic
    TLS cells or source-level worker-local semantics

Non-goals:
- No generic raw numeric TLS slot API in this wave.
- No thread/task-local slot API in this wave.
- No allocator policy here.
- No final platform TLS fallback here.
- No GC barrier logic here.
