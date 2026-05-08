# lang/src/runtime/substrate/intrin — Intrinsic Capability Facade

Responsibilities:
- Own the narrow `.hako` capability facade for substrate intrinsics.
- Keep machine-adjacent bit-count rows separate from optimizer metadata.
- Route live rows through explicit C ABI helpers.

Current live rows:
- `clz_i64(value)`
- `ctz_i64(value)`
- `popcnt_i64(value)`

Current contract:
- Inputs are current-lane non-negative `i64` values.
- Negative values fail fast at the `.hako` facade and return `-1`.
- `clz_i64(0)` and `ctz_i64(0)` return `64`.

Non-responsibilities:
- `@rune IntrinsicCandidate` registry activation.
- Backend optimization metadata export.
- `prefetch`, `assume`, or `unreachable`.
- Full `u64` runtime value semantics.
