---
Status: SSOT
Decision: accepted
Date: 2026-05-10
Scope: reserved allocator hook activation proof vocabulary before any runtime hook activation.
Related:
  - docs/development/current/main/design/allocator-replacement-hook-boundary-ssot.md
  - docs/development/current/main/design/allocator-hook-plan-v0-ssot.md
  - docs/development/current/main/design/allocator-hook-runtime-dry-run-ssot.md
  - docs/development/current/main/design/allocator-hook-activation-proof-v0.toml
---

# Allocator Hook Activation Proof (SSOT)

## Goal

Define what must be proven before allocator hook activation can exist.

M55 is a proof vocabulary row. It does not activate a hook.

## Decision

Allocator hook activation requires a named proof bundle. Runtime dry-run and
HookPlan vocabulary are not enough by themselves.

Current M55 state:

```text
activation proof vocabulary:
  reserved

activation:
  absent

process allocator replacement:
  absent

environment toggle:
  absent
```

## Proof Fixture

Schema fixture:

```text
docs/development/current/main/design/allocator-hook-activation-proof-v0.toml
```

The fixture is reserved-only. It is not consumed by runtime code.

## Required Future Proof Bundle

Any future active allocator hook row must prove all of these in one row:

```text
explicit_hook_plan_fact
runtime_dry_run_validated
default_inactive
no_app_or_facade_name_matching
no_hidden_environment_toggle
no_process_allocator_replacement_without_activation_row
reentrancy_guard_named
bootstrap_allocation_path_named
no_alloc_no_safepoint_contract_named
rollback_condition_named
fail_fast_diagnostic_named
```

The future fail-fast diagnostic root is:

```text
[allocator-hook/activation-proof-missing]
```

## Activation Rule

Activation is forbidden unless:

- a future row changes the activation proof fixture from reserved to active;
- a runtime/kernel owner validates the proof;
- the backend consumes HookPlan facts only;
- the row has a fixture/guard that proves activation remains explicit.

## Explicit Non-Goals

M55 does not add:

- runtime hook install/uninstall body;
- process allocator replacement;
- `#[global_allocator]`;
- hook environment variable;
- `.inc` hook/facade/policy name matching;
- pointer `fetch_add`;
- OSVM unreserve/release;
- native pointer attr widening.

## Gate

```bash
bash tools/checks/k2_wide_allocator_hook_activation_proof_guard.sh
bash tools/checks/k2_wide_allocator_hook_runtime_dry_run_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
