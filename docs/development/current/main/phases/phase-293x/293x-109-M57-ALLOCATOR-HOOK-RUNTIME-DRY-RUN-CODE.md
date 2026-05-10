---
Status: Done
Decision: accepted
Date: 2026-05-10
Scope: M57 allocator hook runtime dry-run code
---

# 293x-109 M57 Allocator Hook Runtime Dry-Run Code

## Decision

`M57 allocator hook runtime dry-run code` is live-narrow.

M57 adds a diagnostic-only runtime validator at:

```text
src/runtime/allocator_hook_dry_run.rs
```

It reports missing HookPlan facts and missing activation proof facts, but it
never installs or replaces the process allocator.

## Owned

- `AllocatorHookDryRunRequest`
- `AllocatorHookDryRunReport`
- `validate_allocator_hook_dry_run(...)`
- unit tests for missing plan, missing activation proof, and ready-diagnostic
  cases.
- Coverage guard:
  `tools/checks/k2_wide_allocator_hook_runtime_dry_run_code_guard.sh`

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
bash tools/checks/k2_wide_allocator_hook_runtime_dry_run_code_guard.sh
bash tools/checks/k2_wide_allocator_hook_runtime_owner_guard.sh
cargo test -q allocator_hook_dry_run
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The guard verifies:

- runtime dry-run code exists only at the named owner path;
- dry-run code is diagnostic-only and always reports `would_install = false`;
- no `std::env`, `std::alloc`, `#[global_allocator]`, or `.inc` hook matching is
  introduced.

## Result

Result on 2026-05-10:
`k2_wide_allocator_hook_runtime_dry_run_code_guard.sh` passes.
