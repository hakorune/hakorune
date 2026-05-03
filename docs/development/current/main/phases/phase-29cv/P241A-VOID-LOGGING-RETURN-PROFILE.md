---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P241a, generic string return profile for void-logging child calls
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P240A-LOWER-LOAD-STORE-LOCAL-ROUTE-RETIRE.md
  - src/mir/global_call_route_plan/string_return_profile.rs
---

# P241a: Void Logging Return Profile

## Problem

P240a advances the source-exe probe to:

```text
target_shape_blocker_symbol=MirBuilderBox._emit_internal_program_json/3
target_shape_blocker_reason=generic_string_return_abi_not_handle_compatible
backend_reason=missing_multi_function_emitter
```

`MirBuilderBox._emit_internal_program_json/3` is a route-sequencing helper:

```text
loop-force -> registry -> fallback -> unsupported-tail
```

It returns promoted MIR JSON on matched routes, and returns the unsupported
tail result on fail-fast. The unsupported tail is already a
`GenericStringVoidLoggingBody`, whose backend return shape is
`void_sentinel_i64_zero`.

The return-profile analyzer currently treats `GenericStringVoidLoggingBody` as
`Other`, so a parent that returns either a string handle or a void-logging child
result cannot become a `GenericStringOrVoidSentinelBody`.

## Decision

Do not add a new body shape and do not add a C emitter.

Fix the value-level fact:

```text
GenericStringVoidLoggingBody return-profile class = Void
```

This keeps Stage0 small. The classifier learns that an already-proven
void-logging child produces the void sentinel for string-or-void return
analysis; it does not learn any new `.hako` compiler semantics.

## Non-Goals

- no new `GlobalCallTargetShape`
- no generic route sequencing shape
- no new ny-llvmc body emitter
- no source workaround in `MirBuilderBox.hako`
- no broad MapBox/Object semantics

## Acceptance

```bash
cargo test -q global_call_route_plan::tests::void_logging --lib
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p241a_void_logging_return_profile.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected source-exe result: the frontier should move past
`MirBuilderBox._emit_internal_program_json/3`; a later blocker may remain.

## Result

Observed probe:

```text
target_shape=generic_string_or_void_sentinel_body
target_symbol=MirBuilderBox._emit_internal_program_json/3
return_shape=string_handle_or_null
tier=DirectAbi
```

The next source-exe frontier is:

```text
target_shape_blocker_symbol=BuilderProgramJsonInputContractBox._program_json_header_present/1
target_shape_blocker_reason=generic_string_unsupported_method_call
backend_reason=missing_multi_function_emitter
```
