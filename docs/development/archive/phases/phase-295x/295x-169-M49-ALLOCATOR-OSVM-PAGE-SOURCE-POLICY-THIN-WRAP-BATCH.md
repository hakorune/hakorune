---
Status: Landed
Date: 2026-05-25
Scope: thin-wrap the M49 allocator OSVM page-source policy guard root into an impl-backed wrapper.
Related:
  - tools/checks/k2_wide_hako_alloc_page_source_policy_exe_guard.sh
---

# 295x-169 M49 Allocator OSVM Page-Source Policy Thin-Wrap Batch

## Blocker

```text
MIMALLOC-COMPARISON-MIMALLOC-BENCH-MALLOC-LARGE-WORKLOAD-CONTRACT-295X-001
```

## Decision

Thin-wrap the M49 allocator OSVM page-source policy EXE guard root. The
validation semantics stay the same while the real shell body moves into
`tools/checks/impl/`.

Selected root:

- `k2_wide_hako_alloc_page_source_policy_exe_guard.sh`

## Cleanup

- Keep the root script as a thin wrapper that execs its impl body.
- Keep the page-source policy owner note in the memory README.
- Leave the current mimalloc comparison blocker unchanged.

## Result

The M49 guard is easier to scan at the root level.

## Stop Line

This batch does not open real OSVM/page-source execution, unreserve, provider
activation, hook, or allocator replacement work.

## Validation

```bash
bash tools/checks/k2_wide_hako_alloc_page_source_policy_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
