---
Status: Landed
Date: 2026-05-24
Scope: refresh the first-conclusion closeout pack.
Blocker: MIMALLOC-COMPARISON-FIRST-CONCLUSION-CLOSEOUT-REFRESH-001
Related:
  - docs/development/current/main/phases/phase-294x/294x-235-MIMALLOC-COMPARISON-FIRST-CONCLUSION-PILOT-REFRESH.md
  - docs/development/current/main/phases/phase-293x/293x-1100-MIMAP-470A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-FIRST-CONCLUSION-CLOSEOUT.md
  - tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_first_conclusion_closeout_guard.sh
---

# 294x-236 Mimalloc Comparison First-Conclusion Closeout Refresh

## Decision

Close `MIMALLOC-COMPARISON-FIRST-CONCLUSION-CLOSEOUT-REFRESH-001`.

The existing MIMAP-470A first-conclusion closeout pack remains stable over the
refreshed MIMAP-468A first-conclusion pilot.

## Next Row

Select `MIMALLOC-COMPARISON-PRESENTATION-ONLY-FOLLOWON-REFRESH-001` as the next
blocker. It should refresh the existing presentation-only follow-on path after
the first-conclusion closeout.

## Stop Line

This row does not:

- rerun repeated or heavy benchmark packs;
- enable provider activation, host allocator replacement, hooks, backend
  matchers, provider package / DLL generation, or `#[global_allocator]`;
- open worker/TLS, atomics, remote-free stress, abandoned-heap stress, or native
  allocator replacement claims.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_first_conclusion_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
