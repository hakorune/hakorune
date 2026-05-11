---
Status: Complete
Date: 2026-05-12
Scope: quick first-row guard cargo invocation cleanup
Related:
  - docs/development/current/main/phases/phase-294x/294x-09e-DEV-GATE-QUICK-PROFILE-SPLIT.md
  - tools/checks/dev_gate.sh
---

# 294x-09f QUICK-FIRST-ROW-CARGO-FILTER-GROUPING

## Decision

Quick first-row guards may group related `cargo test` filters when the grouped
filter still names one contract family clearly.

The shared helper is:

```bash
tools/checks/lib/cargo_test_filter_group.sh
```

This keeps guard scripts readable and reduces repeated cargo process startup.
The helper runs the main crate lib test target so quick does not spend time
discovering unrelated workspace test targets that only report `running 0 tests`.
It does not change the owned route locks, file locks, or semantic acceptance
surface.

## Boundary

- This row does not add or remove compiler behavior.
- This row does not weaken route/file `rg` locks.
- Do not use broad filters that only pass because unrelated tests happen to be
  green.
- Use this helper only for main crate lib-unit contract tests.
- Keep package-specific kernel tests explicit unless they have a clear shared
  contract-family filter.

## Acceptance

```bash
bash -n tools/checks/lib/cargo_test_filter_group.sh
bash tools/checks/k2_wide_osvm_first_row_guard.sh
bash tools/checks/k2_wide_intrin_first_row_guard.sh
bash tools/checks/k2_wide_hako_alloc_gc_trigger_policy_guard.sh
bash tools/checks/dev_gate.sh quick
```
