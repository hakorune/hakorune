---
Status: Landed
Date: 2026-05-24
Scope: refresh the presentation follow-on pilot and closeout pack.
Blocker: MIMALLOC-COMPARISON-PRESENTATION-FOLLOW-ON-PILOT-REFRESH-001
Related:
  - docs/development/current/main/phases/phase-294x/294x-237-MIMALLOC-COMPARISON-PRESENTATION-ONLY-FOLLOWON-REFRESH.md
  - docs/development/current/main/phases/phase-293x/293x-1110-MIMAP-480A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-PRESENTATION-FOLLOW-ON-PILOT.md
  - docs/development/current/main/phases/phase-293x/293x-1112-MIMAP-482A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-PRESENTATION-FOLLOW-ON-CLOSEOUT.md
  - tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_follow_on_pilot_guard.sh
  - tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_follow_on_closeout_guard.sh
---

# 294x-238 Mimalloc Comparison Presentation Follow-On Refresh

## Decision

Close `MIMALLOC-COMPARISON-PRESENTATION-FOLLOW-ON-PILOT-REFRESH-001`.

The existing MIMAP-480A presentation follow-on pilot and MIMAP-482A closeout
pack remain stable over the refreshed presentation-only conclusion boundary.

## Next Row

Select `MIMALLOC-COMPARISON-PRESENTATION-EXTENSION-PILOT-REFRESH-001` as the
next blocker. It should refresh the existing MIMAP-486A presentation extension
pilot over the landed follow-on pack.

## Stop Line

This row does not:

- rerun repeated or heavy benchmark packs;
- enable provider activation, host allocator replacement, hooks, backend
  matchers, provider package / DLL generation, or `#[global_allocator]`;
- open worker/TLS, atomics, remote-free stress, abandoned-heap stress, or native
  allocator replacement claims;
- promote the presentation follow-on report into a new benchmark conclusion.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_follow_on_pilot_guard.sh --level L2
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_follow_on_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
