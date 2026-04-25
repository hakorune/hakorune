---
Status: Landed
Date: 2026-04-25
Scope: Move the explicit post-NewBox birth marker emission for collection literals behind a constructor-owned helper without changing lowering behavior.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-249-constructor-birth-compatibility-contract-card.md
  - docs/development/current/main/phases/phase-291x/291x-250-constructor-birth-carrier-design-card.md
  - docs/development/current/main/phases/phase-291x/291x-251-constructor-birth-owner-shape-decision-card.md
  - src/mir/builder/calls/emit.rs
  - src/mir/builder/exprs.rs
---

# 291x-252 Constructor-Birth Marker Helper Card

## Goal

Thin the array/map literal lowering sites by moving the transitional
post-`NewBox` `birth()` marker emission into one constructor-owned helper.

This is a BoxShape cleanup. It does not prune the generic-method `birth`
compatibility row and does not change runtime behavior.

## Change

Added:

```text
MirBuilder::emit_constructor_birth_marker(receiver, box_type)
```

in [`src/mir/builder/calls/emit.rs`](/home/tomoaki/git/hakorune-selfhost/src/mir/builder/calls/emit.rs).

Updated [`src/mir/builder/exprs.rs`](/home/tomoaki/git/hakorune-selfhost/src/mir/builder/exprs.rs)
so `ArrayLiteral` and `MapLiteral` call the helper instead of constructing the
`birth()` method call inline.

## Boundary

- No ABI exports changed.
- No `.inc` classifier rows changed.
- No runtime behavior changed.
- `birth` remains compatibility-only glue.
- The helper only centralizes the existing marker emission.

## Result

The literal lowering shape is now:

```text
NewBox(ArrayBox|MapBox)
  -> emit_constructor_birth_marker(...)
  -> literal-specific push/set population
```

This gives the next cleanup a single Rust-side place to inspect before any
attempt to retire the generic-method `birth` compatibility row.

## Next Work

Fix the deletion criteria for the remaining generic-method `birth` row:

- identify the fixture/smoke that proves array/map literal construction still
  emits the required initialization path
- decide whether metadata-absent boundaries still need the generic-method
  `birth` classifier
- only then attempt a prune or keep-review card

## Acceptance

```bash
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
