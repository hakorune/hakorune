---
Status: Landed
Date: 2026-04-26
Scope: Audit `MirCallRoutePolicy` ownership before changing runtime/meta module exports or route-policy code.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-288-post-inc-zero-rebaseline-card.md
  - docs/development/current/main/design/stage2-hako-owner-vs-inc-thin-shim-ssot.md
  - lang/src/runtime/meta/README.md
  - lang/src/runtime/meta/hako_module.toml
  - lang/src/runtime/meta/mir_call_route_policy_box.hako
  - src/runner/stage1_bridge/embedded_stage1_modules_snapshot.json
---

# 291x-289 MIR-Call Route Policy Owner Audit Card

## Goal

Decide whether `lang/src/runtime/meta/mir_call_route_policy_box.hako` is an
active owner path or stale transitional vocabulary before code cleanup.

This card is an owner audit. It does not change compiler behavior, module
exports, snapshots, or `.inc` lowering.

## Evidence

Repository search found no active `.hako` or Rust caller of:

```text
MirCallRoutePolicy.classify_generic_method_route(...)
```

The current non-doc references are:

```text
lang/src/runtime/meta/hako_module.toml
src/runner/stage1_bridge/embedded_stage1_modules_snapshot.json
lang/src/runtime/meta/mir_call_route_policy_box.hako
```

The active native route selection still calls the C-side function with the same
name:

```text
lang/c-abi/shims/hako_llvmc_ffi_mir_call_dispatch.inc
lang/c-abi/shims/hako_llvmc_ffi_mir_call_prepass.inc
lang/c-abi/shims/hako_llvmc_ffi_mir_call_route_policy.inc
```

That C path is already metadata-first after the `291x-287` cleanup; the guarded
CoreMethodContract method/box classifier baseline remains:

```text
classifiers=0
rows=0
```

## Decision

`MirCallRoutePolicy` is a registered transitional reference table, not the
current executable route-policy owner.

It may remain registered only as short-lived cleanup debt. New route behavior
must not be added there unless the table is first wired to a generated
CoreMethod/manifest contract and made the actual producer for native metadata.

## Next Implementation

Open `291x-290` as the implementation slice:

```text
MirCallRoutePolicy export quarantine / retirement
```

Preferred order:

1. Remove the stale `MirCallRoutePolicy` export if no caller is introduced.
2. Refresh `src/runner/stage1_bridge/embedded_stage1_modules_snapshot.json`.
3. Keep `lang/c-abi/shims/hako_llvmc_ffi_mir_call_route_policy.inc` unchanged
   unless the implementation also introduces a generated manifest consumer.
4. Run module-registry and current-state guards.

If removal exposes a real consumer, stop and convert the table to a generated
manifest-backed owner instead of preserving the by-name table.

## Boundaries

- Do not add new by-name routes to `mir_call_route_policy_box.hako`.
- Do not reintroduce `.inc` method/box-name classifier rows.
- Do not merge this with `MirCallNeedPolicy` or `MirCallSurfacePolicy`; those
  need separate audits.
- Do not treat snapshot presence as proof of runtime execution.

## Acceptance

```bash
rg -n "MirCallRoutePolicy|mir_call_route_policy_box|classify_generic_method_route\\(" lang src tools crates apps --glob '!target/**' --glob '!*.md'
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
