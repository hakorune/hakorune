---
Status: SSOT
Decision: accepted
Date: 2026-05-10
Scope: reserved allocator HookPlan v0 vocabulary before any runtime hook implementation.
Related:
  - docs/development/current/main/design/allocator-replacement-hook-boundary-ssot.md
  - docs/development/current/main/design/allocator-hook-plan-v0.toml
  - docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md
---

# Allocator HookPlan v0 (SSOT)

## Goal

Define the first backend-readable vocabulary for a future allocator replacement
hook without making any hook active.

M53 is a vocabulary lock:

```text
source / hako_alloc policy
  -> future MIR-owned HookPlan facts
  -> backend reader
  -> runtime/kernel hook seam
```

The current row stops at the docs/manifest vocabulary.

## Manifest

Schema fixture:

```text
docs/development/current/main/design/allocator-hook-plan-v0.toml
```

The fixture is intentionally reserved-only. It is not consumed by runtime code
and it is not a generated backend input.

## Required Fields

Top-level:

```text
schema_version = "allocator_hook_plan_v0"
status = "reserved"
active = false
```

Per plan:

```text
hook_id
state
entrypoints
policy_owner
substrate_routes
reentrancy_mode
requirements
fail_fast_diagnostic
activation
```

Allowed current values:

```text
state = "reserved"
reentrancy_mode = "not_active"
activation = "future_row_required"
```

No active HookPlan row exists yet.

## Meaning

`HookPlan` is the future truth for allocator hook lowering. It names what the
backend may read after a later implementation row activates a plan.

It is not:

```text
an environment variable
an app-name matcher
a .inc policy switch
a runtime hook install function
a #[global_allocator] declaration
```

## Future Activation Requirements

Any future active HookPlan row must add all of these in the same row:

- a fixture or smoke that exercises the hook seam;
- a fail-fast diagnostic for missing/invalid HookPlan facts;
- a runtime/kernel owner for hook mechanics;
- a proof that `.inc` consumes facts only;
- a rollback condition.

## Stop Line

M53 keeps these inactive:

- process allocator replacement;
- runtime hook install/uninstall body;
- hook environment toggles;
- `.inc` hook/facade/policy name matching;
- pointer `fetch_add`;
- OSVM unreserve/release;
- native pointer attr widening.

## Gate

```bash
bash tools/checks/k2_wide_allocator_hook_plan_vocab_guard.sh
bash tools/checks/k2_wide_allocator_replacement_hook_boundary_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
