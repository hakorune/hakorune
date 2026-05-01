---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: document LoweringPlan JSON v0 before backend implementation.
Related:
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
  - docs/development/current/main/design/backend-recipe-route-profile-ssot.md
---

# P69 LoweringPlan JSON v0 SSOT

## Goal

Stop treating unsupported pure shapes as a backend matcher-growth problem.

The next structural step is to make `LoweringPlan` the backend-facing contract
and move ny-llvmc toward emit-only behavior.

## Decision

- Add `docs/development/current/main/design/lowering-plan-json-v0-ssot.md`.
- v0 plan entries are flat JSON under function `metadata.lowering_plan`.
- v0 starts from existing `generic_method_routes` and `CoreMethodOp` metadata.
- `ColdRuntime` is allowed only as an explicit plan entry naming an ABI symbol.
- `Unsupported` is a plan-builder diagnosis; backend must not emit it.

## Implementation Slice After This Card

1. MIR JSON emission derives `metadata.lowering_plan` from
   `generic_method_routes`.
2. ny-llvmc reads `lowering_plan` first for the same site.
3. legacy `generic_method_routes` readers remain fallback during migration.
4. Add one plan-only fixture proving `MapGet` can lower through plan metadata
   without relying on `generic_method_routes`.

## Non-goals

- do not remove existing route metadata in this card
- do not widen pure-first accepted shapes beyond the plan-backed slice
- do not promote `ColdRuntime` to perf proof

## Acceptance

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
