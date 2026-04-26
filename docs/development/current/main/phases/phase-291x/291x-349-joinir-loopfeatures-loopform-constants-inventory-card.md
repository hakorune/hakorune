---
Status: Landed
Date: 2026-04-26
Scope: JoinIR LoopFeatures LoopForm constants inventory
Related:
  - src/mir/loop_route_detection/features.rs
  - src/mir/builder/control_flow/facts/ast_feature_extractor.rs
---

# 291x-349: JoinIR LoopFeatures LoopForm Constants Inventory

## Goal

Inventory fixed default values emitted by the LoopForm feature extractor after
the `LoopFeatures` surface pruning series.

This is BoxShape-only. Do not change route behavior in this card.

## Findings

The LoopForm extractor still spells out fields that are not observable from
`LoopForm` and match `LoopFeatures::default()`:

```text
has_if = false
carrier_count = 0
is_infinite_loop = false
```

These values are correct for the current LoopForm contract:

```text
LoopForm
  -> break targets
  -> continue targets
```

AST-specific facts remain owned by the AST feature extractor:

```text
has_if
carrier_count
is_infinite_loop
```

## Decision

Keep the current behavior, but stop spelling out default-only constants in the
LoopForm extractor.

The LoopForm extractor should construct the live facts and use
`..Default::default()` for facts it cannot observe.

## Next Cleanup

Change:

```text
LoopForm extract_features(...)
  -> LoopFeatures { has_break, has_continue, ..Default::default() }
```

Also remove stale comments that still mention removed nesting fields.

## Non-Goals

- No `LoopFeatures` field removal.
- No route classifier behavior change.
- No AST feature extractor behavior change.

## Validation

```bash
cargo test -q loop_update_
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
