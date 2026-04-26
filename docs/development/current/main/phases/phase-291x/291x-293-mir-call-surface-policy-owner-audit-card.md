---
Status: Landed
Date: 2026-04-26
Scope: Audit `MirCallSurfacePolicy` ownership before changing runtime/meta module exports or native surface-policy code.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-292-mir-call-need-policy-export-retirement-card.md
  - docs/development/current/main/design/stage2-hako-owner-vs-inc-thin-shim-ssot.md
  - lang/src/runtime/meta/README.md
  - lang/src/runtime/meta/hako_module.toml
  - lang/src/runtime/meta/mir_call_surface_policy_box.hako
  - lang/c-abi/shims/README.md
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_surface_policy.inc
  - src/runner/stage1_bridge/embedded_stage1_modules_snapshot.json
---

# 291x-293 MIR-Call Surface Policy Owner Audit Card

## Goal

Decide whether `lang/src/runtime/meta/mir_call_surface_policy_box.hako` is an
active owner path or stale transitional vocabulary before export cleanup.

This card is an owner audit. It does not change compiler behavior, module
exports, snapshots, or `.inc` lowering.

## Evidence

Repository search found no active `.hako` or Rust caller of:

```text
MirCallSurfacePolicy.accept_surface(...)
```

The current non-doc references are:

```text
lang/src/runtime/meta/hako_module.toml
src/runner/stage1_bridge/embedded_stage1_modules_snapshot.json
lang/src/runtime/meta/mir_call_surface_policy_box.hako
```

The active executable surface-policy path is native:

```text
lang/c-abi/shims/hako_llvmc_ffi_pure_compile.inc
lang/c-abi/shims/hako_llvmc_ffi_mir_call_surface_policy.inc
lang/c-abi/shims/hako_llvmc_ffi_mir_call_dispatch.inc
lang/c-abi/shims/hako_llvmc_ffi_mir_call_shell.inc
lang/c-abi/shims/hako_llvmc_ffi_string_concat_emit_routes.inc
```

The native path also has a wider string-extern surface vocabulary than the
`.hako` table:

```text
nyash.string.insert_hsi
nyash.string.substring_hii
nyash.string.substring_concat_hhii
nyash.string.substring_concat3_hhhii
nyash.string.substring_len_hii
```

This proves the `.hako` table is not the current executable authority.

## Decision

`MirCallSurfacePolicy` is a registered transitional reference table, not the
current executable surface-policy owner.

It may remain registered only as short-lived cleanup debt. New surface behavior
must not be added there unless the table is first wired as a generated
manifest-backed producer that feeds the native dispatcher.

## Next Implementation

Open `291x-294` as the implementation slice:

```text
MirCallSurfacePolicy export quarantine / retirement
```

Preferred order:

1. Remove the stale `MirCallSurfacePolicy` export if no caller is introduced.
2. Refresh `src/runner/stage1_bridge/embedded_stage1_modules_snapshot.json`.
3. Keep `lang/c-abi/shims/hako_llvmc_ffi_mir_call_surface_policy.inc`
   unchanged unless the implementation also introduces a generated manifest
   consumer.
4. Update shim/runtime-meta docs so surface policy is not described as a live
   `.hako` owner.
5. Run module-registry and current-state guards.

If removal exposes a real consumer, stop and convert the table to a generated
manifest-backed owner instead of preserving the by-name table.

## Boundaries

- Do not add new by-name surface rows to `mir_call_surface_policy_box.hako`.
- Do not reintroduce `.inc` method/box-name classifier rows.
- Do not delete or thin the native surface-policy consumer in this audit slice.
- Do not treat snapshot presence as proof of runtime execution.

## Acceptance

```bash
rg -n "MirCallSurfacePolicy|mir_call_surface_policy_box|accept_surface\\(" lang src tools crates apps --glob '!target/**' --glob '!*.md'
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
