# exports module notes

- `string.rs` contains the C ABI entrypoints and sink glue for string operations.
- `string_debug.rs` contains opt-in debug logging for string exports.
- `string_route_policy.rs` contains string export route toggles such as compat slow-path allowance and substring view policy.
- `string_search.rs` contains substring search, pair dispatch, and compare helpers.
- `string_plan.rs` contains the transient text carrier (`TextPlan` / `TextPiece`) and plan constructors.
- `string_view.rs` contains `StringView` / `StringSpan`, borrowed substring placement, and span resolution.
- `string_span_cache.rs` contains TLS span-cache storage/promotion helpers.

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
- Do not replace a glob export with explicit symbols until that family has a
  wiring test or inventory note that pins the exported symbol set.
