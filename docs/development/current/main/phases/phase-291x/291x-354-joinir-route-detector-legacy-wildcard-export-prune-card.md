---
Status: Landed
Date: 2026-04-26
Scope: JoinIR route detector legacy wildcard export prune
Related:
  - src/mir/loop_route_detection/mod.rs
  - src/mir/loop_route_detection/legacy/mod.rs
  - docs/development/current/main/phases/phase-291x/291x-353-joinir-route-detector-legacy-export-inventory-card.md
---

# 291x-354: JoinIR Route Detector Legacy Wildcard Export Prune

## Goal

Remove the top-level `legacy::*` wildcard export while preserving the current
compatibility surface.

This is BoxShape-only. Do not change route behavior in this card.

## Change

Replaced:

```text
pub use legacy::*;
```

with explicit exports for:

```text
legacy modules used by existing callers
legacy route-shape functions
legacy convenience type re-exports
```

The top-level `crate::mir::loop_route_detection::...` caller paths remain
stable.

## Preserved Behavior

- No route-shape function changed.
- No legacy module moved or deleted.
- No caller path migrated.
- No route behavior changed.

## Boundary Improvement

The legacy compatibility surface is now auditable at the parent module.

Future cleanup can remove direct root convenience type exports separately from
module-path compatibility exports.

## Next Cleanup

Inventory direct legacy type re-exports from
`crate::mir::loop_route_detection::*`.

Do not delete the legacy modules in that inventory card.

## Non-Goals

- No direct type export deletion in this card.
- No legacy detector module deletion.
- No caller migration.
- No route classifier behavior change.

## Validation

```bash
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
