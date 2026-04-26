---
Status: Landed
Date: 2026-04-27
Scope: normalized-shadow legacy lowerer boundary inventory
Related:
  - src/mir/control_tree/normalized_shadow/legacy/mod.rs
  - src/mir/control_tree/normalized_shadow/builder.rs
  - src/mir/control_tree/normalized_shadow/if_as_last_join_k.rs
  - src/mir/control_tree/normalized_shadow/post_if_post_k.rs
  - src/mir/control_tree/normalized_shadow/loop_true_break_once.rs
  - docs/development/current/main/phases/phase-291x/291x-394-joinir-carrier-update-legacy-emitter-prune-card.md
---

# 291x-395: Normalized-Shadow Legacy Lowerer Inventory

## Goal

Inventory the next normalized-shadow BoxShape seam before changing code.

This card is inventory-only. It does not add accepted StepTree shapes, does not
change lowering behavior, and does not move files.

## Current Shape

`src/mir/control_tree/normalized_shadow/legacy/mod.rs` is still one public
`LegacyLowerer` surface with mixed responsibilities:

```text
entry lowering:
  lower_if_only_to_normalized
  lower_return_from_tree
  lower_if_node
  lower_return_value

shared expression helpers:
  lower_assign_stmt
  parse_minimal_compare
```

The public callers are:

```text
builder.rs
  LegacyLowerer::lower_if_only_to_normalized

if_as_last_join_k.rs
  LegacyLowerer::lower_assign_stmt
  LegacyLowerer::parse_minimal_compare

post_if_post_k.rs
  LegacyLowerer::lower_assign_stmt
  LegacyLowerer::parse_minimal_compare

loop_true_break_once.rs
  LegacyLowerer::lower_assign_stmt
```

## Problem

The module name says `legacy`, but newer normalized-shadow route lowerers still
depend on it for reusable expression lowering. That makes the boundary hard to
read:

- route lowerers import a compatibility owner for non-legacy helper behavior
- shared helper ownership is hidden behind the larger legacy entry path
- future cleanup could accidentally change route semantics while moving legacy
  code

## Decision

Do not physically split the full legacy lowerer in one card.

First introduce a semantic shared-helper facade for normalized-shadow expression
lowering and migrate route files to that facade:

```text
normalized_shadow::support::expr_lowering
  lower_assign_stmt
  parse_minimal_compare
```

The legacy entry lowerer can continue to own the old if-only path until a later
card. This keeps the next slice BoxShape-only and avoids changing StepTree
acceptance.

## Next Cleanup

Add the normalized-shadow shared expression lowering facade and migrate only
these route helper imports:

```text
if_as_last_join_k.rs
post_if_post_k.rs
loop_true_break_once.rs
```

Leave `builder.rs -> LegacyLowerer::lower_if_only_to_normalized` unchanged in
that card.

Acceptance:

```bash
cargo check --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Non-Goals

- Do not add new StepTree shapes.
- Do not rewrite normalized-shadow route selection.
- Do not move the full legacy entry path yet.
- Do not change fail-fast tags or fallback behavior.
