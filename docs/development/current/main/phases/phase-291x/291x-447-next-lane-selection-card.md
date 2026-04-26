---
Status: Landed
Date: 2026-04-27
Scope: next compiler-cleanliness lane selection after rustfmt cleanup
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-291x/291x-446-rustfmt-drift-cleanup-card.md
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
  - src/mir/generic_method_route_plan.rs
---

# 291x-447: Next Lane Selection

## Goal

Choose the next compiler-cleanliness lane after making `cargo fmt -- --check`
green again.

This card is lane selection only. No behavior changed.

## Candidate Lanes

| Lane | Shape | Decision |
| --- | --- | --- |
| `GenericMethodRoute` surface/decision split | BoxShape | select next |
| `.inc` generated enum/table consumer migration | larger consumer lane | defer; avoid changing backend behavior before MIR carrier shape is cleaner |
| MapGet scalar lowering | proof/lowering lane | defer; no hot/warm lowering change without fresh owner evidence |
| Stage-B trace/body helper follow-up | BoxShape | defer; Stage-B thinning was just closed |
| JoinIR cleanup burst reopen | BoxShape | defer; normalized-shadow / normalization burst is closed |

## Decision

Select **`GenericMethodRoute` surface/decision split** as the next lane.

Reason:

- `291x-444` moved exported `route_id`, `emit_kind`, and `effects` tokens to
  `GenericMethodRouteKind`, but the main carrier still stores raw surface
  compatibility fields (`box_name`, `method`, `arity`) beside decided route and
  CoreMethod metadata.
- The next clean step is not `.inc` behavior change. It is making the MIR
  carrier shape say which fields are compatibility surface and which fields are
  decided compiler metadata.
- This is a bounded BoxShape lane: it can improve authority boundaries without
  adding accepted method surfaces, changing helper selection, or adding hot
  lowering.
- Keeping raw surface under a named sub-record makes later `.inc` enum/table
  migration easier because backend-facing decisions can ignore compatibility
  strings by construction.

## Next Card

Create `291x-448-generic-method-route-surface-inventory` before code edits.

The inventory must classify:

- raw surface compatibility fields that must stay in JSON for existing readers
- decided route fields that own backend metadata
- proof / return-shape / demand / publication metadata that must remain MIR-owned
- tests and JSON emitters that currently read surface fields directly

## Guards

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
