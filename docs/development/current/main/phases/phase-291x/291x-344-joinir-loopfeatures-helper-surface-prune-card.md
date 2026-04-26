---
Status: Landed
Date: 2026-04-26
Scope: JoinIR LoopFeatures helper surface prune
Related:
  - src/mir/loop_route_detection/features.rs
  - src/mir/loop_route_detection/classify.rs
  - src/mir/loop_route_detection/mod.rs
  - docs/development/current/main/phases/phase-291x/291x-343-joinir-loopfeatures-helper-surface-inventory-card.md
---

# 291x-344: JoinIR LoopFeatures Helper Surface Prune

## Goal

Remove the dead helper surface identified by 291x-343.

This is behavior-preserving BoxShape cleanup.

## Change

Removed:

```text
LoopFeatures::debug_stats(...)
LoopFeatures::total_divergences(...)
LoopFeatures::is_complex(...)
LoopFeatures::is_simple(...)
classify_with_diagnosis(...)
classify_with_diagnosis export
```

Preserved:

```text
LoopFeatures plain fields
extract_features(...)
classify(...)
LoopRouteKind
```

## Boundary After Prune

The loop route detector exposes one live decision path:

```text
LoopFeatures
  -> classify(...)
  -> LoopRouteKind
```

Diagnostic text should be owned by live callers/log sites, not an unused
parallel classifier helper.

## Preserved Behavior

- Route classification logic is unchanged.
- LoopForm feature extraction is unchanged.
- No route shape is added or removed.

## Non-Goals

- No route-kind enum change.
- No log tag change.
- No StepTree behavior change.

## Validation

```bash
rg -n "\\b(debug_stats|total_divergences|is_complex|is_simple|classify_with_diagnosis)\\(" src tests -g '*.rs' || true
cargo test -q loop_update_
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
