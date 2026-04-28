---
Status: Landed
Date: 2026-04-28
Scope: close broad plan/facts owner migration planning after small compatibility shelves were pruned
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-575-plan-compat-residue-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-577-unused-plan-facts-wrapper-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-578-small-plan-facts-owner-path-migration-card.md
  - src/mir/builder/control_flow/facts/mod.rs
  - src/mir/builder/control_flow/plan/facts/mod.rs
---

# 291x-620: Broad Plan Facts Migration Boundary

## Goal

Close the original `291x-586` broad `plan/facts` owner migration planning item
without starting a bulk move.

This is an inventory/decision card. It does not move fact modules, change
accepted shapes, or alter routing/lowering behavior.

## Evidence

The small compatibility shelves from the `291x-575` queue are already gone:

- zero-use `plan/facts` wrappers: `291x-577`
- small facts owner-path migrations: `291x-578`
- later residue wrappers and shelves: `291x-601` through `291x-611`

The remaining `plan::facts` surface is not a single compatibility shelf.
Current usage is still broad and live:

| Surface | Current references |
| --- | ---: |
| `control_flow::plan::facts` | 367 refs across 93 Rust files |
| `control_flow::facts` | 618 refs across 176 Rust files |

The module headers now describe two distinct owners:

- `control_flow::facts`: top-level owner surface for facts-owned modules.
- `plan::facts`: plan-side facts owner for structural loop facts, scan shapes,
  skeleton facts, exit-block recipes, reject diagnostics, and recipe-facing
  fact types.

## Decision

Do not perform a broad `plan/facts` bulk migration in the current compat-prune
queue.

The remaining work is no longer "delete a wrapper"; it is ownership design.
Any future move must be a new BoxShape lane with a family-sized SSOT and a
small acceptance boundary.

## Future Queue

If this reopens, split it by owner family:

| Order | Family | Required first step |
| ---: | --- | --- |
| 1 | diagnostics/reject detail | decide whether `reject_reason` belongs under neutral diagnostics or stays plan-local |
| 2 | scan/skeleton observation | decide whether `scan_shapes`, `feature_facts`, and `skeleton_facts` move together |
| 3 | exit-block recipes | decide whether `exit_only_block` / `return_prelude` are facts owners or recipe builders |
| 4 | route fact payloads | keep `LoopFacts` and route facts plan-owned until recipe ownership changes |

## Boundaries

- No broad mechanical rewrite from `plan::facts` to `control_flow::facts`.
- No cross-family moves in a single card.
- Do not change Facts extraction order or planner acceptance.
- Keep `control_flow::facts/mod.rs` from growing compatibility re-export shelves.

## Acceptance

- The original broad migration todo can be marked done as a decision, not a code
  migration.
- `CURRENT_STATE.toml` points at this card.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `git diff --check` passes.

## Result

- Closed the broad migration item as a boundary decision.
- Preserved the current two-owner facts layout.
- Fixed the next work shape: future facts cleanup must be family-sized, not
  bulk path rewriting.

## Verification

```bash
rg -n "control_flow::plan::facts" src/mir/builder/control_flow src/mir -g'*.rs' | wc -l
rg -n "control_flow::plan::facts" src/mir/builder/control_flow src/mir -g'*.rs' | cut -d: -f1 | sort -u | wc -l
rg -n "control_flow::facts" src/mir/builder/control_flow src/mir -g'*.rs' | wc -l
rg -n "control_flow::facts" src/mir/builder/control_flow src/mir -g'*.rs' | cut -d: -f1 | sort -u | wc -l
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
