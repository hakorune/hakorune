---
Status: Active
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P140, BuildBox null-opts direct scan-source route
Related:
  - docs/development/current/main/phases/phase-29cv/P139-GLOBAL-CALL-TRANSITIVE-RETURN-BLOCKER.md
  - lang/src/compiler/build/build_box.hako
---

# P140: BuildBox Null Opts Direct Scan

## Problem

P139 moved the source-execution blocker to the leaf object boundary:

```text
target_shape_blocker_symbol=BuildBox._new_prepare_scan_src_result/1
target_shape_blocker_reason=generic_string_return_object_abi_not_handle_compatible
```

For the normal `BuildBox.emit_program_json_v0(source, null)` route, that
prepare-result `MapBox` is not semantically needed unless env-driven bundle
inputs are present.

## Decision

Add an explicit null-opts fast path:

```text
opts == null && bundle env not requested -> _emit_program_json_from_scan_src(src)
```

The existing prepare-result path remains the owner for non-null opts and
env-driven bundle inputs.

## Rules

Allowed:

- bypass the prepare-result `MapBox` only when `opts == null` and bundle env
  inputs are absent
- keep env-driven bundle inputs on the existing prepare path
- preserve `BuildBox.emit_program_json_v0` return semantics

Forbidden:

- deleting the prepare-result path
- ignoring bundle env inputs
- adding backend MapBox/object lowering as a shortcut

## Expected Evidence

The active source-execution route should advance past
`BuildBox._new_prepare_scan_src_result/1` and expose the next real
source-to-Program blocker.

## Acceptance

- `target/release/hakorune --emit-mir-json ... stage1_cli_env.hako` succeeds.
- The generated MIR route no longer reports
  `BuildBox._new_prepare_scan_src_result/1` as the top source-execution blocker
  when bundle env inputs are absent.
- `NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe ... stage1_cli_env.hako`
  advances to the next blocker.
