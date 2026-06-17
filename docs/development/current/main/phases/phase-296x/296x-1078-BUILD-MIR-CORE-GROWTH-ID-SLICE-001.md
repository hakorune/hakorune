Status: Done
Date: 2026-06-18
Scope: first mir_core growth implementation slice
Related:
  - docs/development/current/main/phases/phase-296x/296x-1077-BUILD-MIR-CORE-GROWTH-PREFLIGHT-001.md
  - crates/hakorune_mir_core/src/control_ids.rs

# BUILD-MIR-CORE-GROWTH-ID-SLICE-001

## Purpose

Move the first dependency-free MIR control-flow ID group into
`hakorune-mir-core` while preserving existing `src::mir::control_form` import
paths as a compatibility facade.

## Change

```text
new_core_module=crates/hakorune_mir_core/src/control_ids.rs
moved_types=LoopId,ExitEdgeId,ContinueEdgeId
compat_reexport=src/mir/control_form.rs
behavior_changed=0
```

The move is intentionally limited to newtypes. `LoopRegion`, `ExitEdge`,
`ContinueEdge`, and the analysis helpers stay in `src/mir/control_form.rs`
because they are control-form observation logic, not core identifier substrate.

## Contract

```text
output_contract=build-mir-core-growth-id-slice-v0

boxshape_only=1
boxcount_allowed=0
behavior_changed=0
control_form_import_compat_preserved=1
mir_core_growth_slice_done=1

summary=ok
```

## Next

```text
next_task=BUILD-MIR-PLANS-CRATE-PREFLIGHT-001
```
