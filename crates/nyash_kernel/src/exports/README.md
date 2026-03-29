# exports module notes

- `string.rs` contains the C ABI entrypoints and sink glue for string operations.
- `string_debug.rs` contains opt-in debug logging and feature-flag helpers for string exports.
- `string_search.rs` contains substring search, pair dispatch, and compare helpers.
- `string_plan.rs` contains the transient text carrier (`TextPlan` / `TextPiece`) and plan constructors.
- `string_view.rs` contains `StringView` / `StringSpan`, borrowed substring placement, and span resolution.
- `string_span_cache.rs` contains TLS span-cache storage/promotion helpers.
