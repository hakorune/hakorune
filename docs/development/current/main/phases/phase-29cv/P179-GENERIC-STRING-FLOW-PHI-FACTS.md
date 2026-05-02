---
Status: Accepted
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P179, generic string body flow facts
Related:
  - docs/development/current/main/phases/phase-29cv/P178-GENERIC-STRING-SUBSTRING-SUFFIX.md
  - src/mir/global_call_route_plan/generic_string_body.rs
  - src/mir/global_call_route_plan/tests/runtime_methods.rs
---

# P179: Generic String Flow PHI Facts

## Problem

After P178, the source-execution probe advanced to:

```text
target_shape_blocker_symbol=JsonFragNormalizerBox._const_canonicalize/1
target_shape_blocker_reason=generic_string_unsupported_method_call
```

`_const_canonicalize/1` uses two flow facts that the generic string body
classifier did not preserve:

- scalar loop PHI values with `type_hint = i64` used as substring bounds
- `StringOrVoid` values refined by `value != null` before string methods such
  as `value.length()`

The backend method routes already had direct ABI plans for the string methods.
The missing piece was classifier evidence, not a backend fallback.

## Decision

Keep the acceptance inside the generic string body classifier:

- Seed scalar PHI destinations from `type_hint` only when all known inputs are
  scalar-compatible and no string/void/container evidence is present.
- Precompute null-guarded successor PHI values from `Eq` / `Ne` comparisons
  against `null` / `void`, following copy aliases for the compared value.
- Narrow only those successor PHI values from `StringOrVoid` to `String`.

This does not treat all `StringOrVoid` values as strings. The proof is tied to
the guarded CFG edge and the successor PHI, so unguarded null receivers still
fail fast.

## Acceptance

```bash
cargo test -q refresh_module_global_call_routes_accepts_runtime_data_string_substring_typed_phi_bound --lib
cargo test -q refresh_module_global_call_routes_accepts_string_or_void_null_guarded_length_receiver --lib
cargo test -q runtime_methods --lib
cargo test -q global_call_routes --lib
cargo fmt --check
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p179_flow_stage1_cli_env.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The final `--emit-exe` command is an advance-to-next-blocker probe.

## Result

`JsonFragNormalizerBox._const_canonicalize/1` no longer blocks on generic string
method-call flow evidence.

The probe advances to:

```text
target_shape_blocker_symbol=JsonFragBox.get_int/2
target_shape_blocker_reason=generic_string_return_abi_not_handle_compatible
```

Treat `JsonFragBox.get_int/2` return-shape acceptance as the next card.
