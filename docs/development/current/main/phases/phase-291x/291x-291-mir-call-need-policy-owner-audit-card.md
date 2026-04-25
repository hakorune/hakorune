---
Status: Landed
Date: 2026-04-26
Scope: Audit `MirCallNeedPolicy` ownership before changing runtime/meta module exports or native need-policy code.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-290-mir-call-route-policy-export-retirement-card.md
  - docs/development/current/main/design/stage2-hako-owner-vs-inc-thin-shim-ssot.md
  - lang/src/runtime/meta/README.md
  - lang/src/runtime/meta/hako_module.toml
  - lang/src/runtime/meta/mir_call_need_policy_box.hako
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_need_policy.inc
  - src/runner/stage1_bridge/embedded_stage1_modules_snapshot.json
---

# 291x-291 MIR-Call Need Policy Owner Audit Card

## Goal

Decide whether `lang/src/runtime/meta/mir_call_need_policy_box.hako` is an
active owner path or stale transitional vocabulary before export cleanup.

This card is an owner audit. It does not change compiler behavior, module
exports, snapshots, or `.inc` lowering.

## Evidence

Repository search found no active `.hako` or Rust caller of:

```text
MirCallNeedPolicy.classify_need_flags(...)
```

The current non-doc references are:

```text
lang/src/runtime/meta/hako_module.toml
src/runner/stage1_bridge/embedded_stage1_modules_snapshot.json
lang/src/runtime/meta/mir_call_need_policy_box.hako
```

The active executable need-policy path is still native:

```text
lang/c-abi/shims/hako_llvmc_ffi_pure_compile.inc
lang/c-abi/shims/hako_llvmc_ffi_mir_call_need_policy.inc
```

That native path consumes route/CoreMethod metadata and applies declaration /
stable-object / publish / invalidation need flags during the `mir_call`
prepass. It must remain until a generated producer or typed LoweringPlan owns
those need flags.

## Decision

`MirCallNeedPolicy` is a registered transitional reference table, not the
current executable need-policy owner.

It may remain registered only as short-lived cleanup debt. New need behavior
must not be added there unless the table is first wired as a generated
manifest-backed producer that feeds the native prepass.

## Next Implementation

Open `291x-292` as the implementation slice:

```text
MirCallNeedPolicy export quarantine / retirement
```

Preferred order:

1. Remove the stale `MirCallNeedPolicy` export if no caller is introduced.
2. Refresh `src/runner/stage1_bridge/embedded_stage1_modules_snapshot.json`.
3. Keep `lang/c-abi/shims/hako_llvmc_ffi_mir_call_need_policy.inc` unchanged
   unless the implementation also introduces a generated manifest consumer.
4. Update shim/runtime-meta docs so route and need policy are not described as
   live `.hako` owners.
5. Run module-registry and current-state guards.

If removal exposes a real consumer, stop and convert the table to a generated
manifest-backed owner instead of preserving the by-name table.

## Boundaries

- Do not add new by-name need flags to `mir_call_need_policy_box.hako`.
- Do not reintroduce `.inc` method/box-name classifier rows.
- Do not delete or thin the native need-policy consumer in this audit slice.
- Do not merge this with `MirCallSurfacePolicy`; it needs a separate audit.
- Do not treat snapshot presence as proof of runtime execution.

## Acceptance

```bash
rg -n "MirCallNeedPolicy|mir_call_need_policy_box|classify_need_flags\\(" lang src tools crates apps --glob '!target/**' --glob '!*.md'
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
