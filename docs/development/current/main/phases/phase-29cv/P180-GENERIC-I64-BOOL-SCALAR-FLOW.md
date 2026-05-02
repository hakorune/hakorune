---
Status: Accepted
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P180, generic i64 Bool scalar flow
Related:
  - docs/development/current/main/phases/phase-29cv/P179-GENERIC-STRING-FLOW-PHI-FACTS.md
  - src/mir/global_call_route_plan/generic_i64_body.rs
  - src/mir/global_call_route_plan/tests/generic_i64.rs
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P180: Generic I64 Bool Scalar Flow

## Problem

After P179, the source-execution probe exposed the next scalar corridor around
`JsonFragBox.get_int/2`.

The immediate child path used `StringHelpers.skip_ws/2`, which calls
`StringHelpers.is_space/1`. `is_space/1` is a Bool-return scalar helper, but the
generic i64 body classifier only accepted integer returns and treated nested
generic i64 global-call results as `I64`. That erased existing Bool metadata for
the call destination, so the branch condition in `skip_ws/2` failed classifier
refinement before the real JSON cursor blocker could be reached.

The C generic-function return emitter also needed to preserve the ABI contract:
Bool scalar return values are represented as `i1` in LLVM IR and must be
zero-extended to the generic scalar `i64` return ABI.

## Decision

Keep this as scalar-flow evidence, not a fallback:

- Accept `Bool` return types and Bool return values in the generic i64 body
  classifier.
- Seed typed scalar PHI destinations from `type_hint` when the destination and
  input evidence are still unknown.
- Preserve an existing Bool destination class for nested generic i64 global
  calls; otherwise keep the existing `I64` default.
- In the C generic-function emitter, resolve copy aliases for return values and
  zero-extend `i1` returns to `i64`.

This keeps `Bool` inside the generic scalar lane while preserving the existing
DirectAbi shape name and return-shape contract.

## Acceptance

```bash
cargo test -q refresh_module_global_call_routes_accepts_bool_return_generic_i64_body --lib
cargo test -q refresh_module_global_call_routes_preserves_bool_dst_from_generic_i64_global_call --lib
cargo test -q refresh_module_global_call_routes_accepts_typed_i64_phi_from_unknown_param --lib
cargo test -q generic_i64 --lib
cargo test -q global_call_routes --lib
cargo fmt --check
bash tools/build_hako_llvmc_ffi.sh
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p180_scalar_bool_stage1_cli_env.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The final `--emit-exe` command is an advance-to-next-blocker probe.

## Result

`StringHelpers.skip_ws/2` now routes its call to `StringHelpers.is_space/1` as a
direct generic i64 call:

```text
target_symbol=StringHelpers.is_space/1
target_return_type=i1
target_shape=generic_i64_body
return_shape=ScalarI64
```

`JsonFragBox.read_int_from/2` now reaches `StringHelpers.skip_ws/2` through
DirectAbi. The remaining child blocker is the recursive string-return cursor
shape:

```text
target_shape_blocker_symbol=JsonCursorBox._digits_from_rec/4
target_shape_blocker_reason=generic_string_global_target_shape_unknown
```

The top-level unsupported-shape report still surfaces through
`JsonFragBox.get_int/2`; treat the recursive cursor string body as the next
card.
