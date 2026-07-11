---
Status: Completed
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv Stage0 body-shape inventory and uniform emitter boundary lock
Related:
  - docs/development/current/main/design/stage0-llvm-line-shape-inventory-ssot.md
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - src/mir/global_call_route_plan/model.rs
  - docs/development/current/main/CURRENT_STATE.toml
---

# P345A: Stage0 Shape Inventory Lock

## Problem

P207A fixed the direction: Stage0 must stay a small MIR-to-LLVM bootstrap line.
However, the shape guard was still spread across phase cards and comments.

`GlobalCallTargetShape` variants need an explicit status table so future
blocker work does not treat every body-specific capsule as permanent Stage0
knowledge.

## Boundary

Do not add or remove `GlobalCallTargetShape` variants.

Do not change route classification behavior.

Do not add C shim emitters.

This is a docs/code-pointer BoxShape lock only.

## Implementation

- add `stage0-llvm-line-shape-inventory-ssot.md`
- record the one-line Stage0 rule:
  `Stage0 does not know the selfhost compiler; Stage0 knows Canonical MIR and uniform ABI`
- classify every current `GlobalCallTargetShape` as fail-fast, permanent
  candidate, or temporary capsule
- require a removal path for temporary capsules
- lock `missing_multi_function_emitter` as a uniform MIR function emitter
  request, not permission for another body shape
- add a source-code pointer next to the enum

## Acceptance

```text
rg -n "stage0-llvm-line-shape-inventory-ssot" \
  docs/development/current/main/design src/mir/global_call_route_plan/model.rs
-> design index, bootstrap route SSOT, lowering plan SSOT, and enum pointer
```

```text
bash tools/checks/current_state_pointer_guard.sh
-> ok

git diff --check
-> ok
```
