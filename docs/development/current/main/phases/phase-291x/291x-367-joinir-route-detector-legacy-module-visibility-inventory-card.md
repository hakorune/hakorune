---
Status: Landed
Date: 2026-04-26
Scope: JoinIR route detector legacy module visibility inventory
Related:
  - src/mir/loop_route_detection/mod.rs
  - docs/development/current/main/phases/phase-291x/291x-366-joinir-route-detector-export-surface-closeout-review-card.md
---

# 291x-367: JoinIR Route Detector Legacy Module Visibility Inventory

## Goal

Inventory whether `src/mir/loop_route_detection/mod.rs` still needs to expose
the `legacy` module directly.

This is BoxShape-only. Do not change visibility in this card.

## Findings

The parent module currently declares:

```text
pub mod legacy;
```

and re-exports selected compatibility modules:

```text
pub use legacy::{...};
```

Repository search found no active source caller using:

```text
crate::mir::loop_route_detection::legacy::...
```

Remaining `legacy` references are docs/phase notes and route cleanup cards.

## Decision

`legacy` no longer needs to be a direct public route surface.

The next prune card should test changing:

```text
pub mod legacy;
```

to:

```text
mod legacy;
```

while preserving selected parent compatibility exports.

## Next Cleanup

Make `legacy` private and validate with `cargo check -q`.

If Rust visibility rejects re-exporting selected modules through a private
owner module, stop and document the exact boundary instead of widening the
surface again.

## Non-Goals

- No visibility change in this card.
- No deletion of legacy support modules.
- No change to selected parent compatibility exports.

## Validation

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
