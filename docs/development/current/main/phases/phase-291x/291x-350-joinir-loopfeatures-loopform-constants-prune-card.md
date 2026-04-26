---
Status: Landed
Date: 2026-04-26
Scope: JoinIR LoopFeatures LoopForm constants prune
Related:
  - src/mir/loop_route_detection/features.rs
  - src/mir/builder/control_flow/facts/ast_feature_extractor.rs
  - src/mir/loop_route_detection/tests.rs
  - src/mir/join_ir/lowering/loop_scope_shape/case_a_lowering_shape.rs
  - src/mir/join_ir/lowering/loop_view_builder.rs
  - docs/development/current/main/phases/phase-291x/291x-349-joinir-loopfeatures-loopform-constants-inventory-card.md
---

# 291x-350: JoinIR LoopFeatures LoopForm Constants Prune

## Goal

Stop spelling out default-only constants in the LoopForm feature extractor.

This is behavior-preserving BoxShape cleanup.

## Change

Changed the LoopForm extractor to emit only facts that `LoopForm` currently
owns:

```text
LoopFeatures {
  has_break,
  has_continue,
  ..Default::default()
}
```

Removed explicit default-only locals:

```text
has_if = false
carrier_count = 0
is_infinite_loop = false
```

Also removed stale comments that referenced already-pruned nesting fields.

## Boundary After Prune

LoopForm feature extraction now documents its real observed surface by code:

```text
LoopForm
  -> has_break
  -> has_continue
```

AST feature extraction remains the owner for:

```text
has_if
carrier_count
is_infinite_loop
```

## Preserved Behavior

- LoopForm `has_if` remains false by default.
- LoopForm `carrier_count` remains zero by default.
- LoopForm `is_infinite_loop` remains false by default.
- Route classification behavior is unchanged.

## Non-Goals

- No `LoopFeatures` field removal.
- No AST feature extractor behavior change.
- No route classifier behavior change.

## Validation

```bash
cargo test -q loop_update_
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
