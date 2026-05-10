---
Status: Done
Decision: accepted
Date: 2026-05-10
Scope: M53 allocator HookPlan vocabulary lock
---

# 293x-105 M53 Allocator HookPlan Vocabulary Lock

## Decision

`M53 allocator HookPlan vocabulary lock` is live-docs.

M53 adds a reserved HookPlan v0 SSOT and TOML fixture. It does not activate any
runtime hook or process allocator replacement.

Design owners:

```text
docs/development/current/main/design/allocator-hook-plan-v0-ssot.md
docs/development/current/main/design/allocator-hook-plan-v0.toml
```

## Owned

- Reserved HookPlan v0 vocabulary.
- Reserved TOML fixture.
- Coverage guard:
  `tools/checks/k2_wide_allocator_hook_plan_vocab_guard.sh`
- docs/taskboard/current pointers for M53.

## Not Owned

- Runtime hook install/uninstall implementation.
- Process allocator replacement.
- `#[global_allocator]`.
- Hook environment variables.
- `.inc` hook/facade/policy name matching.
- Pointer `fetch_add`.
- OSVM unreserve/release.
- Native pointer attr widening.

## Gate

```bash
bash tools/checks/k2_wide_allocator_hook_plan_vocab_guard.sh
bash tools/checks/k2_wide_allocator_replacement_hook_boundary_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The guard verifies:

- the HookPlan v0 SSOT and TOML fixture are synchronized;
- the TOML fixture is reserved-only and inactive;
- M52 boundary guard remains forward-compatible after latest-card movement;
- no allocator hook implementation symbols, env toggles, process allocator
  replacement, or `.inc` name matching exists.

## Result

Result on 2026-05-10:
`k2_wide_allocator_hook_plan_vocab_guard.sh` passes.
