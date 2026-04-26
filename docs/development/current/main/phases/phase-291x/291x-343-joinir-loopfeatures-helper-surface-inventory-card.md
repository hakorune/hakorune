---
Status: Landed
Date: 2026-04-26
Scope: JoinIR LoopFeatures helper surface inventory
Related:
  - src/mir/loop_route_detection/features.rs
  - src/mir/loop_route_detection/classify.rs
  - src/mir/loop_route_detection/mod.rs
---

# 291x-343: JoinIR LoopFeatures Helper Surface Inventory

## Goal

Inventory the remaining helper surface on `LoopFeatures` and
`loop_route_detection::classify`.

This is BoxShape-only. Do not change route behavior in this card.

## Findings

The following helper surface has no production caller:

```text
LoopFeatures::debug_stats(...)
LoopFeatures::total_divergences(...)
LoopFeatures::is_complex(...)
LoopFeatures::is_simple(...)
classify_with_diagnosis(...)
```

Actual references are self-contained:

```text
classify_with_diagnosis(...)
  -> features.debug_stats(...)

is_complex(...)
  -> total_divergences(...)
```

No route builder or lowerer calls these helpers.

## Decision

The remaining helper surface is dead diagnostic scaffolding.

`LoopFeatures` should be a plain feature vector, and the route classifier should
expose only the live decision entrypoint:

```text
LoopFeatures
  -> classify(...)
  -> LoopRouteKind
```

## Next Cleanup

Prune:

```text
LoopFeatures impl helper block
classify_with_diagnosis(...)
classify_with_diagnosis export
```

Preserve:

```text
LoopFeatures fields
classify(...)
LoopRouteKind
extract_features(...)
```

## Non-Goals

- No route classification behavior change.
- No diagnostic tag/log change.
- No route-kind enum change.

## Validation

```bash
rg -n "debug_stats\\(|total_divergences\\(|is_complex\\(|is_simple\\(|classify_with_diagnosis\\(" src tests -g '*.rs'
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
