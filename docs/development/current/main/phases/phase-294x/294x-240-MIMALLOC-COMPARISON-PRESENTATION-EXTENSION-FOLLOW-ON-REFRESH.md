---
Status: Landed
Date: 2026-05-24
Scope: refresh the presentation extension follow-on pilot and closeout pack.
Blocker: MIMALLOC-COMPARISON-PRESENTATION-EXTENSION-FOLLOW-ON-PILOT-REFRESH-001
Related:
  - docs/development/current/main/phases/phase-294x/294x-239-MIMALLOC-COMPARISON-PRESENTATION-EXTENSION-REFRESH.md
  - docs/development/current/main/phases/phase-293x/293x-1122-MIMAP-492A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-PRESENTATION-EXTENSION-FOLLOW-ON-PILOT.md
  - docs/development/current/main/phases/phase-293x/293x-1124-MIMAP-494A-ALLOCATOR-COMPARISON-C-MIMALLOC-RESULT-PRESENTATION-EXTENSION-FOLLOW-ON-CLOSEOUT.md
  - tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_extension_follow_on_pilot_guard.sh
  - tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_extension_follow_on_closeout_guard.sh
---

# 294x-240 Mimalloc Comparison Presentation Extension Follow-On Refresh

## Decision

Close `MIMALLOC-COMPARISON-PRESENTATION-EXTENSION-FOLLOW-ON-PILOT-REFRESH-001`.

The existing MIMAP-492A presentation extension follow-on pilot and MIMAP-494A
closeout pack remain stable over the refreshed presentation extension boundary.

## Next Row

Select
`MIMALLOC-COMPARISON-PRESENTATION-EXTENSION-FOLLOW-ON-EXTENSION-PILOT-REFRESH-001`
as the next blocker. It should refresh the existing MIMAP-498A presentation
extension follow-on extension pilot.

## Stop Line

This row does not:

- rerun repeated or heavy benchmark packs;
- enable provider activation, host allocator replacement, hooks, backend
  matchers, provider package / DLL generation, or `#[global_allocator]`;
- open worker/TLS, atomics, remote-free stress, abandoned-heap stress, or native
  allocator replacement claims;
- promote the follow-on-ready extension report into a new benchmark conclusion.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_extension_follow_on_pilot_guard.sh --level L2
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_extension_follow_on_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
