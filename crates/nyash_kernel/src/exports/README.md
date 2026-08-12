# exports module notes

- `string.rs` contains the C ABI entrypoints and sink glue for string operations.
- `string_debug.rs` contains opt-in debug logging for string exports.
- `string_route_policy.rs` contains string export route toggles such as compat slow-path allowance and substring view policy.
- `string_search.rs` contains substring search, pair dispatch, and compare helpers.
- `string_plan.rs` contains the transient text carrier (`TextPlan` / `TextPiece`) and plan constructors.
- `string_view.rs` contains `StringView` / `StringSpan`, borrowed substring placement, and span resolution.
- `string_span_cache.rs` contains TLS span-cache storage/promotion helpers.
- `dynamic_v2_text_scan.rs` contains the work-branch-only strict CodePoint
  `hako.text.scan@1` entries. It uses the shared CallOut wire and the root
  runtime's one-shot lease owner; it must not call the generic String surface,
  VM, or LLVM dispatch until the complete I0 activation is cut over.

## Re-export Inventory

`mod.rs` currently keeps glob re-exports for crate-root ABI compatibility.
Treat them as public symbol-family exports, not as ownership boundaries.

- ABI families currently re-exported from `exports/mod.rs`: `any`, `atomic`,
  `birth`, `box_helpers`, `cmp`, `env`, `file`, `instance`, `mem`, `osvm`,
  `primitive`, `runtime`, `stage1`, `string`, `tls`, `typed_object`,
  `user_box`, `worker`.
- Internal string support modules such as `string_route_policy`,
  `string_search`, `string_plan`, `string_view`, and `string_span_cache`
  stay module imports only; they do not define crate-root ABI ownership.
- `dynamic_v2_text_scan` is also kept as an internal module import. Its two
  `export_name` symbols are a strict AOT checkpoint, not a selected production
  caller or a second provider registry.
- Do not replace a glob export with explicit symbols until that family has a
  wiring test or inventory note that pins the exported symbol set.
