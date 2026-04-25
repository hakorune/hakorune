---
Status: Landed
Date: 2026-04-26
Scope: Retire the unused `MirCallRoutePolicy` runtime/meta export after the owner audit.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-289-mir-call-route-policy-owner-audit-card.md
  - docs/development/current/main/design/stage2-hako-owner-vs-inc-thin-shim-ssot.md
  - lang/src/runtime/meta/README.md
  - lang/src/runtime/meta/hako_module.toml
  - src/runner/stage1_bridge/embedded_stage1_modules_snapshot.json
---

# 291x-290 MIR-Call Route Policy Export Retirement Card

## Goal

Remove the registered-but-unused `MirCallRoutePolicy` export so runtime/meta no
longer advertises a by-name route-policy owner that is not on the executable
compiler path.

## Decision

`MirCallRoutePolicy` is retired from runtime/meta exports.

The executable route path remains:

```text
MIR generic_method route metadata
  -> hako_llvmc_ffi_mir_call_route_policy.inc
  -> hako_llvmc_ffi_mir_call_dispatch.inc / prepass
```

This card intentionally does not change native route selection or lowering.

## Cleanup

- Removed `MirCallRoutePolicy` from `lang/src/runtime/meta/hako_module.toml`.
- Deleted the stale `lang/src/runtime/meta/mir_call_route_policy_box.hako`
  table instead of leaving an unexported policy with no caller.
- Refreshed `src/runner/stage1_bridge/embedded_stage1_modules_snapshot.json`
  so the embedded module list matches the workspace registry.
- Updated runtime/meta and stage2 owner-vs-shim docs to state that route policy
  is not `.hako`-exported today.

## Boundary

- Do not reintroduce a `.hako` by-name route table unless it is generated from
  CoreMethod/manifest metadata and wired as the real producer.
- Do not edit `hako_llvmc_ffi_mir_call_route_policy.inc` in this card.
- Keep CoreMethodContract `.inc` classifier baseline at zero.
- Audit `MirCallNeedPolicy` and `MirCallSurfacePolicy` separately; do not
  delete them by analogy.

## Acceptance

```bash
bash tools/selfhost/refresh_stage1_module_env_snapshot.sh
bash tools/checks/module_registry_hygiene_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
cargo test -q embedded_snapshot_matches_registry_doc
git diff --check
```
