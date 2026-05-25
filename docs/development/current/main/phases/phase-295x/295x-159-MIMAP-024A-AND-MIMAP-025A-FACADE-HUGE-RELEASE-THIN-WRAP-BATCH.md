---
Status: Landed
Date: 2026-05-25
Scope: thin-wrap the MIMAP-024A and MIMAP-025A mimalloc facade huge release guard roots into impl-backed wrappers.
Related:
  - tools/checks/k2_wide_mimalloc_facade_huge_release_exe_guard.sh
  - tools/checks/k2_wide_mimalloc_facade_huge_release_failfast_exe_guard.sh
---

# 295x-159 MIMAP-024A and MIMAP-025A Facade Huge Release Thin-Wrap Batch

## Blocker

```text
MIMALLOC-COMPARISON-MIMALLOC-BENCH-MALLOC-LARGE-WORKLOAD-CONTRACT-295X-001
```

## Decision

Thin-wrap the MIMAP-024A facade huge release guard root and the MIMAP-025A
facade huge release fail-fast guard root. The batch keeps the validation
semantics unchanged and moves both real shell bodies into `tools/checks/impl/`.

Selected roots:

- `k2_wide_mimalloc_facade_huge_release_exe_guard.sh`
- `k2_wide_mimalloc_facade_huge_release_failfast_exe_guard.sh`

## Cleanup

- Keep the root scripts as thin wrappers that exec their impl bodies.
- Keep the huge-release route and fail-fast route semantics unchanged.
- Leave the current mimalloc comparison blocker unchanged.

## Result

The mimalloc facade huge release guard roots are easier to scan at the root
level.

## Stop Line

This batch does not open real allocator release execution, fail-fast proof
claims, provider activation, process replacement, hook installation, backend
matcher wiring, global allocator installation, or winner claims.

## Validation

```bash
bash tools/checks/k2_wide_mimalloc_facade_huge_release_exe_guard.sh
bash tools/checks/k2_wide_mimalloc_facade_huge_release_failfast_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
