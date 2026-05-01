---
Status: Active
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P142, return-profile blocker propagation for direct child targets
Related:
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - docs/development/current/main/phases/phase-29cv/P141-GLOBAL-CALL-SENTINEL-SUBSTRING-RETURN.md
  - src/mir/global_call_route_plan.rs
---

# P142: Global Call Direct Child Blocker Propagation

## Problem

After P141, `BodyExtractionBox.extract_balanced_body/2` is a direct
`generic_string_or_void_sentinel_body`, but parent return-profile diagnostics
still reported it as:

```text
target_shape_blocker_symbol=BodyExtractionBox.extract_balanced_body/2
target_shape_blocker_reason=generic_string_global_target_missing
```

That was not a real missing target. The return-profile blocker inventory only
filtered unknown child targets, then fell through to the missing-target fallback
for already-direct child targets.

## Decision

Split child target handling in the return-profile blocker scan:

- missing target -> `generic_string_global_target_missing`
- unknown target -> propagate the child blocker
- direct target -> no blocker

This keeps blocker propagation diagnostic-only and prevents a direct child from
masking the next parent boundary.

## Rules

Allowed:

- propagate blockers only from unknown child targets
- ignore direct child targets in blocker inventory
- keep route lowerability owned by the child `target_shape`

Forbidden:

- treating a direct child as missing
- promoting a parent solely because a child exists
- changing backend emission from blocker diagnostics

## Expected Evidence

The active route should no longer report `BodyExtractionBox.extract_balanced_body/2`
as `generic_string_global_target_missing`. The next blocker should move to the
actual parent return-profile boundary.

## Acceptance

- `cargo fmt --check` succeeds.
- `cargo test -q string_return_blocker_ignores_direct_string_child_targets` succeeds.
- `cargo test -q global_call_routes` succeeds.
- `target/release/hakorune --emit-mir-json ... stage1_cli_env.hako` exposes
  `BodyExtractionBox.extract_main_body/1` as the next parent blocker.
