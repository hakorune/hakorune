---
Status: Landed
Date: 2026-04-26
Scope: JoinIR route detector legacy module visibility prune
Related:
  - src/mir/loop_route_detection/mod.rs
  - docs/development/current/main/phases/phase-291x/291x-367-joinir-route-detector-legacy-module-visibility-inventory-card.md
---

# 291x-368: JoinIR Route Detector Legacy Module Visibility Prune

## Goal

Make `loop_route_detection::legacy` private while preserving selected parent
compatibility exports.

This is BoxShape-only. Do not change route behavior.

## Change

Changed:

```text
pub mod legacy;
```

to:

```text
mod legacy;
```

Selected parent exports remain public through:

```text
pub use legacy::{...};
```

Also renamed the internal-only `DigitPosPromotionResult::CannotPromote` field:

```text
vars -> _vars
```

This preserves diagnostic payload shape inside tests while keeping `cargo
check` warning-free after `legacy` became private.

## Preserved Behavior

- Existing parent compatibility module paths remain supported.
- No legacy support module was deleted.
- No promotion result control flow changed.
- No route classifier behavior changed.
- No route lowerer behavior changed.

## Boundary Improvement

Direct access through:

```text
crate::mir::loop_route_detection::legacy::...
```

is no longer part of the public module surface.

The parent module now owns the compatibility boundary explicitly.

## Next Cleanup

Review the parent module docs now that `legacy` is private. The phrase
`legacy/` should describe implementation storage, not a public route entry.

## Non-Goals

- No deletion of selected compatibility exports.
- No caller migration.
- No route classifier API change.

## Validation

```bash
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
