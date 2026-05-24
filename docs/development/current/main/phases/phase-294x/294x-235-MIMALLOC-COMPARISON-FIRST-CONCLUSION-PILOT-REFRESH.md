---
Status: Landed
Date: 2026-05-24
Scope: refresh the first-conclusion pilot.
Blocker: MIMALLOC-COMPARISON-FIRST-CONCLUSION-PILOT-REFRESH-001
Related:
  - docs/development/current/main/phases/phase-294x/294x-234-MIMALLOC-COMPARISON-FIRST-CONCLUSION-PLAN-REFRESH.md
  - docs/development/current/main/phases/phase-293x/293x-1098-MIMAP-468A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-FIRST-CONCLUSION-PILOT.md
  - tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_first_conclusion_pilot_guard.sh
---

# 294x-235 Mimalloc Comparison First-Conclusion Pilot Refresh

## Decision

Close `MIMALLOC-COMPARISON-FIRST-CONCLUSION-PILOT-REFRESH-001`.

The existing MIMAP-468A first-conclusion pilot remains stable over the refreshed
preflight, reporting, summary, result-ledger, and explicit C mimalloc runner
evidence chain.

## Next Row

Select `MIMALLOC-COMPARISON-FIRST-CONCLUSION-CLOSEOUT-REFRESH-001` as the next
blocker. It should refresh the existing MIMAP-470A first-conclusion closeout
pack.

## Stop Line

This row does not:

- rerun repeated or heavy benchmark packs;
- enable provider activation, host allocator replacement, hooks, backend
  matchers, provider package / DLL generation, or `#[global_allocator]`;
- open worker/TLS, atomics, remote-free stress, abandoned-heap stress, or native
  allocator replacement claims.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_first_conclusion_pilot_guard.sh --level L2
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
