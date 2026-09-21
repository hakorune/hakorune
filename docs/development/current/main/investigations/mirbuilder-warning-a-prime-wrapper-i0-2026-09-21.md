---
Status: closed__2026-09-21__WarningAPrimeWrapper__DeletedAndVerified
Task: MIRBUILDER-WARNING-A-PRIME-WRAPPER-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i116-2026-09-21.md
Implementation permission: true for the production-zero A-prime JSON wrapper
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I118
---

# Warning cleanup: obsolete A-prime JSON wrapper

## Six-line brief

```text
Decision: delete insert_a_prime_i64_physical_receipt_json and its wrapper-only
absence test; retain the value encoder used by the production metadata path.
Source authority + canonical issuer: a_prime_i64_capability.rs value encoder;
the selected receipt is already present before metadata emission.
Non-authority: warning suppression, cargo-fix, metadata schema changes, or a
new receipt issuer.
Fail-fast boundary: any production caller, metadata/JSON focused red, or
warning-count mismatch rejects the row.
Smallest next slice: remove the dead wrapper and its absence test, then run
the A-prime metadata/JSON focused gates.
Non-claims: no receipt schema change, selected-route change, fallback repair,
or LegacyCallV0 retirement.
```

## Census boundary

The bounded inventory is the wrapper function at
`src/runner/mir_json_emit/a_prime_i64_capability.rs:9-17` and its sole
wrapper-specific test `absent_receipt_does_not_create_metadata_key`. `rg`
found no production caller. The live production path in `metadata.rs` calls
`insert_a_prime_i64_physical_receipt_value_json` directly inside the selected
receipt branch. The sealed-receipt encoder tests and `io.rs` metadata tests
remain in scope as guards. Receipt types, key names, and selected metadata
emission are out of scope.

## Acceptance

```text
cargo fmt --all -- --check
cargo test --profile quick --lib runner::mir_json_emit::a_prime_i64_capability
cargo test --profile quick --lib runner::mir_json_emit::io
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
bash tools/checks/current_state_pointer_guard.sh
```

Expected warning refresh: lib **1,695 → 1,694** for the removed wrapper
group; lib-test remains **550**. No `#[allow]`, schema, receipt, or route
change is allowed.

## Closeout evidence

The production-zero wrapper and its wrapper-only absence test were deleted.
The selected metadata path still calls the value encoder directly, and the
sealed receipt JSON schema remains guarded by the surviving encoder test.
`FunctionMetadata` is now imported only inside the test module.

Focused and sequential validation:

```text
cargo test --profile quick --lib runner::mir_json_emit::a_prime_i64_capability
  -> 1/1 passed
cargo test --profile quick --lib runner::mir_json_emit::io
  -> 8/8 passed
cargo check --profile quick --lib -j4
  -> lib warnings 1,694
cargo test --profile quick --lib --no-run -j4
  -> lib-test warnings 550
cargo fmt --all -- --check                     -> pass
git diff --check                                -> pass
bash tools/checks/current_state_pointer_guard.sh -> pass
```

No receipt schema, selected route, fallback, or legacy edge changed. The
old-edge lane remains `NoSafeSlice`; the next action is a fresh warning
baseline refresh before selecting one bounded cohort.
