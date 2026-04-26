---
Status: Landed
Date: 2026-04-26
Scope: Retire the unused `MirCallSurfacePolicy` runtime/meta export after owner audit.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-293-mir-call-surface-policy-owner-audit-card.md
  - docs/development/current/main/design/stage2-hako-owner-vs-inc-thin-shim-ssot.md
  - lang/src/runtime/meta/README.md
  - lang/src/runtime/meta/hako_module.toml
  - lang/c-abi/shims/README.md
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_surface_policy.inc
  - src/runner/stage1_bridge/embedded_stage1_modules_snapshot.json
---

# 291x-294 MIR-Call Surface Policy Export Retirement Card

## Goal

Remove the stale `MirCallSurfacePolicy` `.hako` export after `291x-293`
showed that it is not an active executable owner path.

This is a BoxShape cleanup slice. It does not change native surface-policy
lowering, string extern routing, constructor/global emission, or `mir_call`
behavior.

## Implementation

Removed:

```text
lang/src/runtime/meta/mir_call_surface_policy_box.hako
lang/src/runtime/meta/hako_module.toml: MirCallSurfacePolicy export
```

Refreshed:

```text
src/runner/stage1_bridge/embedded_stage1_modules_snapshot.json
```

Kept intact:

```text
lang/c-abi/shims/hako_llvmc_ffi_mir_call_surface_policy.inc
```

The native surface-policy consumer remains the current executable owner until a
generated manifest or typed LoweringPlan owns constructor/global/string-extern
surfaces and feeds the dispatcher.

## Boundary

- Do not re-add a `.hako` by-name surface table.
- Do not treat this as native shim thinning; the native surface consumer is
  still required.
- Do not add any `.inc` method/box-name classifier rows.
- Do not use snapshot presence as an execution proof.

## Acceptance

```bash
bash tools/selfhost/refresh_stage1_module_env_snapshot.sh
bash tools/checks/module_registry_hygiene_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
cargo test -q embedded_snapshot_matches_registry_doc
git diff --check
tools/checks/dev_gate.sh quick
```

## Next

With route/need/surface `.hako` mirror tables retired, the next cleanup should
inventory remaining `runtime/meta` modules and confirm whether `CoreMethodContract`
and generated manifest are the only live compiler semantic contract tables.
