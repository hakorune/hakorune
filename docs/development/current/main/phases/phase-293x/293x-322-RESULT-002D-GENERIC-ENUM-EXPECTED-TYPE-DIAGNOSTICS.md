# 293x-322 RESULT-002D generic enum expected-type diagnostics

Status: Complete
Date: 2026-05-14

## Decision

Prelude `Option<T>` and `Result<T,E>` constructors require explicit local typed
context when the constructor itself cannot determine all generic parameters.

This keeps the canonical surface explicit and avoids adding inference:

```hako
local err: Result<i64, String> = Result::Err("bad")
local empty: Option<i64> = Option::None
```

The untyped forms fail-fast:

```hako
local err = Result::Err("bad")
local empty = Option::None
```

## Scope

- Add `[enum/expected-type][prelude]` diagnostics in Program(JSON v0) Stage1
  lowering for valid prelude Option/Result constructors used as untyped local
  initializers.
- Preserve existing payload arity and `Option::Some(null|void)` diagnostics by
  running expected-type checks only after constructor shape is valid.
- Track source enum names so same-program `enum Option<T>` / `enum Result<T,E>`
  declarations do not accidentally receive prelude-only diagnostics.

## Guard

- `tools/checks/k2_wide_result_option_expected_type_diagnostics_guard.sh`

## Validation

- `bash tools/checks/k2_wide_result_option_expected_type_diagnostics_guard.sh`
