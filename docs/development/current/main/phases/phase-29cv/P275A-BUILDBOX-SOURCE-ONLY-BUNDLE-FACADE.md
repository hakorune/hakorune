---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P275a, BuildBox source-only boundary and bundle facade split
Related:
  - docs/development/current/main/phases/phase-29cv/P140-BUILDBOX-NULL-OPTS-DIRECT-SCAN.md
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-291x/291x-283-buildbox-bundle-input-collector-split-card.md
  - lang/src/compiler/build/build_box.hako
  - lang/src/compiler/build/build_bundle_facade_box.hako
  - lang/src/compiler/entry/stageb_compile_adapter_box.hako
---

# P275a: BuildBox Source-Only Bundle Facade

## Problem

After P274a, the active source-execution route advances to:

```text
target_shape_blocker_symbol=BuildBundleInputBox.collect/1
target_shape_blocker_reason=generic_string_return_object_abi_not_handle_compatible
```

`BuildBundleInputBox.collect/1` returns a `MapBox` bundle context. Lowering that
object return through generic string/i64 Stage0 would widen Stage0 object
semantics and violate the P207a size guard.

The source-execution lane only needs the source-only call:

```text
BuildBox.emit_program_json_v0(source, null)
```

Bundle opts/env are Stage-B VM features and should not be pulled into the
source-execution Stage0 function set.

## Decision

Split the owner boundary:

```text
BuildBox
  source-only Program(JSON v0) authority
  src + null opts + no bundle env -> Program(JSON)
  opts/env bundle inputs -> explicit tagged unsupported for this entry

BuildBundleFacadeBox
  bundle-aware Stage-B adapter facade
  collects opts/env bundle context
  resolves merged scan_src
  delegates merged source back to BuildBox source-only authority
```

`StageBCompileAdapterBox` uses `BuildBundleFacadeBox` so existing Stage-B bundle
VM smokes keep their behavior. The Stage0 source-execution source no longer
imports the bundle collector/resolver through `BuildBox`.

## Rules

Allowed:

- move bundle input collection out of `BuildBox`
- keep bundle collector/resolver live behind a Stage-B facade
- make `BuildBox.emit_program_json_v0(...)` fail-fast for non-source-only calls

Forbidden:

- adding generic MapBox/object return ABI lowering
- accepting `BuildBundleInputBox.collect/1` in generic string/i64 classifiers
- adding a new `GlobalCallTargetShape` or C shim emitter for bundle collection
- silently ignoring bundle opts/env inputs

## Acceptance

- `BuildBox.emit_program_json_v0/2` no longer reports
  `BuildBundleInputBox.collect/1` as its source-execution blocker.
- The active source-execution probe advances to the next blocker without adding
  Stage0 object-return ABI support.
- Stage-B bundle VM smokes continue to pass through `BuildBundleFacadeBox`.

## Result

Accepted.

`BuildBox.emit_program_json_v0/2` now stays on the source-only DirectAbi route:

```text
BuildBox.emit_program_json_v0/2              generic_pure_string_body             string_handle          DirectAbi
MirBuilderProgramJsonBuildBox.emit_program_json_v0/3
                                             generic_string_or_void_sentinel_body  string_handle_or_null  DirectAbi
```

The source-execution probe advanced to:

```text
target_shape_blocker_symbol=Stage1MirResultValidationBox._debug_print_mir_state/2
target_shape_blocker_reason=generic_string_return_abi_not_handle_compatible
backend_reason=missing_multi_function_emitter
```

Stage-B bundle VM smokes passed through the new facade:

```bash
bash tools/smokes/v2/profiles/integration/stageb/stageb_bundle_duplicate_fail_vm.sh
SMOKES_ENABLE_STAGEB=1 bash tools/smokes/v2/profiles/integration/stageb/stageb_bundle_require_ok_vm.sh
SMOKES_ENABLE_STAGEB=1 bash tools/smokes/v2/profiles/integration/stageb/stageb_bundle_mix_emit_vm.sh
```
