---
Status: Complete
Date: 2026-05-23
Scope: migrate secure-list diagnostics counters from `i64` to exact `usize`.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/secure_free_list_diagnostics_box.hako
  - tools/checks/k2_wide_mimalloc_secure_list_diagnostics_guard.sh
---

# 294x-39 Hako Alloc Usize Secure List Diagnostic Counters

## Decision

Migrate the `HakoAllocSecureFreeListDiagnostics` monotonic diagnostics counters:

- `scan_count`
- `ok_count`
- `fail_count`
- `out_of_range_free_block_count`
- `duplicate_free_block_count`
- `live_block_in_free_list_count`
- `free_count_mismatch_count`
- `local_free_count_mismatch_count`

These fields are diagnostics-only counters and do not carry negative sentinels.

## Stop Line

The `last_*` observation fields remain `i64` because they are binary flag
observations, not this row's counter group. This row does not open secure-list
encoding policy, entropy/cookie sourcing, hardening claims, provider
activation, host allocator replacement, hooks, or `#[global_allocator]`.

## Acceptance

```bash
bash tools/checks/k2_wide_mimalloc_secure_list_diagnostics_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
