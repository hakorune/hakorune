---
Status: Landed
Date: 2026-04-28
Scope: inventory legacy *_v0 plan families and classify thin wrappers versus live behavior owners before further cleanup
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-575-plan-compat-residue-inventory-card.md
  - src/mir/builder/control_flow/plan/facts/loop_builder.rs
  - src/mir/builder/control_flow/plan/loop_collect_using_entries_v0/pipeline.rs
  - src/mir/builder/control_flow/plan/loop_bundle_resolver_v0/pipeline.rs
  - src/mir/builder/control_flow/plan/loop_scan_phi_vars_v0/pipeline.rs
---

# 291x-585: Legacy V0 Family Boundary Inventory

## Goal

Inventory the remaining `*_v0` plan families and separate thin compatibility
wrappers from modules that still own real lowering behavior.

This is an inventory-only card. It does not move or rewrite live lowering code.

## Evidence

`facts/loop_builder.rs` shows the `*_v0` families are still part of the active
Facts extraction order, including the scope-aware ordering requirement around
`loop_scan_methods_block_v0` before `flatten_scope_boxes()`.

The inventory split the family into two groups:

| Group | Modules | Finding |
| --- | --- | --- |
| Thin wrappers | `loop_collect_using_entries_v0`, `loop_bundle_resolver_v0` | small lowering shims that validate a couple of bindings then delegate to `parts::entry::lower_loop_v0` |
| Live behavior owners | `loop_scan_v0`, `loop_scan_methods_v0`, `loop_scan_methods_block_v0`, `loop_scan_phi_vars_v0` | own real segment routing, nested-loop handling, or specialized phi/scan behavior |

One important seam remains in `loop_scan_phi_vars_v0`: it imports
`recipes::loop_scan_phi_vars_v0::LoopScanPhiSegment` directly, so recipe/data
ownership should be reviewed before any structural move there.

## Boundaries

- Do not merge or reorder the active facts extraction sites from this card.
- Do not prune the four live `loop_scan*` families blindly.
- Use the wrapper/live split as the queue for the next cleanup cards.

## Next Safe Queue

| Order | Size | Target | Reason |
| ---: | --- | --- | --- |
| 1 | S | `loop_collect_using_entries_v0` | thin wrapper over `parts::entry::lower_loop_v0` |
| 2 | S | `loop_bundle_resolver_v0` | same thin-wrapper shape with exit-allowed contract |
| 3 | L | `loop_scan_phi_vars_v0` recipe seam review | direct recipe import is the current awkward boundary |
| 4 | L | `loop_scan_v0` / `loop_scan_methods*_v0` consolidation planning | live segment-routing behavior still owns real lowering logic |

## Acceptance

- `CURRENT_STATE.toml` points at this inventory card.
- The wrapper/live split is recorded concretely enough to drive follow-up cards.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `git diff --check` passes.

## Result

- Fixed the `*_v0` family inventory in one place instead of treating every v0
  module as the same kind of cleanup target.
- Identified the two smallest wrapper-only slices for the next cards.
- Marked the recipe seam in `loop_scan_phi_vars_v0` as a hold point for future
  review instead of a blind prune candidate.

## Verification

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
