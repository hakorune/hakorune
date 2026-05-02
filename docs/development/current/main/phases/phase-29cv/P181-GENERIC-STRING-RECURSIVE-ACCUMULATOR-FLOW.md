---
Status: Accepted
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P181, generic string recursive accumulator flow
Related:
  - docs/development/current/main/phases/phase-29cv/P180-GENERIC-I64-BOOL-SCALAR-FLOW.md
  - src/mir/global_call_route_plan/generic_string_body.rs
  - src/mir/global_call_route_plan/tests/runtime_methods.rs
---

# P181: Generic String Recursive Accumulator Flow

## Problem

After P180, the source-execution probe reached the JSON cursor digit scanner:

```text
target_shape_blocker_symbol=JsonCursorBox._digits_from_rec/4
target_shape_blocker_reason=generic_string_global_target_shape_unknown
```

`JsonCursorBox._digits_from_rec/4` is a self-recursive string accumulator body.
The classifier had three missing flow facts:

- a self-recursive global call depends on the current function's own shape, so
  using only the target table creates an Unknown cycle
- `ch >= "0"` / `ch <= "9"` compares a value with a string constant, but the
  unknown side was not inferred as a string
- `out + ch` proves the accumulator is string, but the proof had to travel
  backwards through a single-input PHI used by the recursive CFG lowering

These are classifier facts. The backend direct-call lowerer already has the
generic string global-call route once the shape is known.

## Decision

Keep the acceptance inside the generic pure string body classifier:

- Treat a global call to the current function symbol as the same generic string
  body shape during classification.
- Infer an unknown string-compare operand as `String` when the opposite operand
  is already `String`.
- Infer an unknown string-concat operand as `String` when the opposite operand
  is already `String`.
- Back-propagate an already-known PHI destination class to unknown PHI inputs
  when all known inputs agree with that destination class.

This does not add a by-name exception for `JsonCursorBox`; it is a structural
self-recursive string accumulator shape.

## Acceptance

```bash
cargo test -q refresh_module_global_call_routes_infers_unknown_lhs_from_string_compare --lib
cargo test -q refresh_module_global_call_routes_accepts_self_recursive_generic_pure_string_body --lib
cargo test -q runtime_methods --lib
cargo test -q global_call_routes --lib
cargo fmt --check
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p181_phi_recursive_stage1_cli_env.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The final `--emit-exe` command is an advance-to-next-blocker probe.

## Result

The JSON digit scanner chain now routes through DirectAbi:

```text
target_symbol=JsonCursorBox._digits_from_rec/4
target_shape=generic_pure_string_body
return_shape=string_handle
```

`JsonFragBox.get_int/2` now reaches `JsonFragBox.read_int_after/2` as a
`generic_string_or_void_sentinel_body` direct call. The probe advances to:

```text
target_shape_blocker_symbol=JsonFragNormalizerBox._normalize_instructions_array/1
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
```

Treat the normalizer void-sentinel const flow as the next card.
