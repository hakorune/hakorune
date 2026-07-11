# 293x-667 HAKO-ALLOC-RESULT-API-003 Allocator Local-Free Remaining Result Guard-Let Boundaries

Status: landed
Date: 2026-05-18

## Decision

Apply the already-accepted local `Result<i64, i64>` + guard-let surface to the
two remaining local failure boundaries inside
`HakoAllocSegmentAllocationModeledLocalFreeIntegration.integrateLocalFree`.

## Owner

```text
lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_integration_box.hako
tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_integration_guard.sh
docs/development/current/main/design/guard-let-pattern-sugar-ssot.md
```

## Scope

- Keep the candidate boundary from HAKO-ALLOC-RESULT-API-002.
- Convert the apply-plan local check to a local `Result<i64, i64>` consumed by
  guard-let.
- Convert the page-apply local check to a local `Result<i64, i64>` consumed by
  guard-let.
- Preserve proof output and report record fields.
- Keep all `Result` values local to the same function body.

## Stop Lines

- No cross-function `Result` direct ABI.
- No broad allocator report rewrite.
- No implicit propagation sugar such as `?`, `try`, `throw`, or hidden fallback.
- No runtime sum object materialization.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No backend `.inc` matcher by allocator name.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_integration_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Implementation

- Kept the HAKO-ALLOC-RESULT-API-002 candidate Result boundary.
- Added local `plan_result: Result<i64, i64>` and consumed it with guard-let.
- Added local `page_apply_result: Result<i64, i64>` and consumed it with
  guard-let.
- Kept all Result values local to `integrateLocalFree`.
- Updated the local-free integration guard to require the three local Result
  guard-let boundaries.

## Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_integration_guard.sh
```

Result:

```text
[mimap119a-mir-json] ok
[k2-wide-hako-alloc-segment-allocation-modeled-local-free-integration] ok
```

## Closeout

Next row:

```text
MIMAP-148A
  post-local-free-Result-boundary row selection
```
