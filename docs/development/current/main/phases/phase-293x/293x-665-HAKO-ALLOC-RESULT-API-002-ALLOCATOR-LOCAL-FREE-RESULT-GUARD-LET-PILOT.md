# 293x-665 HAKO-ALLOC-RESULT-API-002 Allocator Local-Free Result Guard-Let Pilot

Status: selected current
Date: 2026-05-18

## Decision

Apply the now-accepted `Result` + `guard let` surface to one allocator owner
only, proving that a normal `.hako` failure-boundary style can reduce scalar
status/reason boilerplate without changing allocator behavior.

## Owner

```text
lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_integration_box.hako
apps/hako-alloc-segment-allocation-modeled-local-free-integration-proof/
tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_integration_guard.sh
docs/development/current/main/design/guard-let-pattern-sugar-ssot.md
```

## Scope

- Pick one small local-free integration helper boundary that currently carries
  success/failure through scalar fields.
- Return an explicit `Result<i64, i64>` or `Result<..., ...>` shape from that
  helper and consume it with `guard let Result::Ok(value) = ... else { ... }`.
- Preserve existing report record fields and proof app output.
- Keep direct MIR / VM proof green through the existing owner guard.

## Stop Lines

- No broad allocator report rewrite.
- No report schema/output change.
- No implicit `?`, `try`, `throw`, null, or fallback sugar.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No backend `.inc` matcher.
- No silent fallback.

## Required Evidence

```text
NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 120 target/debug/hakorune --backend vm apps/hako-alloc-segment-allocation-modeled-local-free-integration-proof/main.hako
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_integration_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
