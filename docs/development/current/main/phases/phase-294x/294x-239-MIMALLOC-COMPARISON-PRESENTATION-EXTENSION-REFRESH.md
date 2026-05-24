---
Status: Landed
Date: 2026-05-24
Scope: refresh the presentation extension pilot and closeout pack.
Blocker: MIMALLOC-COMPARISON-PRESENTATION-EXTENSION-PILOT-REFRESH-001
Related:
  - docs/development/current/main/phases/phase-294x/294x-238-MIMALLOC-COMPARISON-PRESENTATION-FOLLOW-ON-REFRESH.md
  - docs/development/current/main/phases/phase-293x/293x-1116-MIMAP-486A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-PRESENTATION-EXTENSION-PILOT.md
  - docs/development/current/main/phases/phase-293x/293x-1118-MIMAP-488A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-PRESENTATION-EXTENSION-CLOSEOUT.md
  - tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_extension_pilot_guard.sh
  - tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_extension_closeout_guard.sh
---

# 294x-239 Mimalloc Comparison Presentation Extension Refresh

## Decision

Close `MIMALLOC-COMPARISON-PRESENTATION-EXTENSION-PILOT-REFRESH-001`.

The existing MIMAP-486A presentation extension pilot and MIMAP-488A closeout
pack remain stable over the refreshed presentation follow-on pack.

## Next Row

Select `MIMALLOC-COMPARISON-PRESENTATION-EXTENSION-FOLLOW-ON-PILOT-REFRESH-001`
as the next blocker. It should refresh the existing MIMAP-492A presentation
extension follow-on pilot over the landed extension-ready pack.

## Stop Line

This row does not:

- rerun repeated or heavy benchmark packs;
- enable provider activation, host allocator replacement, hooks, backend
  matchers, provider package / DLL generation, or `#[global_allocator]`;
- open worker/TLS, atomics, remote-free stress, abandoned-heap stress, or native
  allocator replacement claims;
- promote the extension-ready report into a new benchmark conclusion.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_extension_pilot_guard.sh --level L2
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_extension_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
