---
Status: Landed
Date: 2026-04-26
Scope: JoinIR LoopFeatures route surface review
Related:
  - src/mir/loop_route_detection/features.rs
  - src/mir/loop_route_detection/classify.rs
  - src/mir/loop_route_detection/mod.rs
  - src/mir/join_ir/lowering/loop_route_router.rs
---

# 291x-351: JoinIR LoopFeatures Route Surface Review

## Goal

Review the remaining `LoopFeatures` surface after the pruning series through
291x-350.

This is BoxShape-only. Do not change route behavior in this card.

## Current Surface

`LoopFeatures` now contains only live route facts:

```text
has_break
has_continue
has_if
carrier_count
is_infinite_loop
```

Live producers:

```text
LoopForm extractor
  -> has_break / has_continue

AST feature extractor
  -> has_break / has_continue / has_if / carrier_count / is_infinite_loop

Case-A / LoopViewBuilder stubs
  -> carrier_count only, with defaults for everything else
```

Live consumer:

```text
classify(...)
```

## Decision

The code surface is thin enough for the current lane. Do not remove additional
`LoopFeatures` fields without a new route-owner change.

Remaining cleanup is documentation/comment accuracy:

```text
loop_route_detection::mod docs still read like classify owns every route kind.
loop_route_router docs still imply LoopFeatures classify can reach NestedLoopMinimal.
classify.rs comment still references both routers too broadly.
```

## Next Cleanup

Do a comment/docs-only pass:

```text
src/mir/loop_route_detection/mod.rs
src/mir/loop_route_detection/classify.rs
src/mir/join_ir/lowering/loop_route_router.rs
```

Clarify:

```text
LoopFeatures classify owns flat route families.
NestedLoopMinimal is selected by the AST/StepTree route path.
LoopForm router only observes LoopForm facts.
```

## Non-Goals

- No `LoopFeatures` field removal.
- No classifier behavior change.
- No route-kind enum change.

## Validation

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
