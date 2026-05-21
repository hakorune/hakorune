# Hako Alloc Allocator Comparison C Mimalloc Result Presentation Extension Follow-On Extension Follow-On Extension Follow-On Extension Follow-On Extension Follow-On Extension Follow-On Extension Follow-On Pilot SSOT

Status: accepted
Decision: accepted
Date: 2026-05-22
Owner: MIMAP-552A

## Decision: accepted

MIMAP-552A opens the first comparison-ready pilot over the landed MIMAP-546A
deeper-extension-ready report and the fixed MIMAP-550A explicit C mimalloc
contract.

The pilot accepts only explicit, accepted MIMAP-546A reports, verifies that the
hako_alloc and explicit C mimalloc contract fields are present, fixes the shared
comparison-ready report schema, and keeps benchmark, allocator, and provider
stop lines closed. It must not execute a C runner, rerun benchmarks, or publish
a winner.

## Reason Vocabulary

```text
0 = accepted comparison-ready follow-on pilot
1 = missing MIMAP-546A deeper-extension-ready report
2 = blocked MIMAP-546A deeper-extension-ready report
3 = missing hako_alloc / explicit C mimalloc contract input
4 = missing shared workload contract
5 = closed stop-line violation
```

## Stop Lines

- No repeated benchmark execution.
- No process allocator replacement.
- No hooks.
- No backend matcher additions.
- No `#[global_allocator]`.
- No provider package / DLL generation.
- No hidden discovery or process-global activation.
- No worker/thread execution.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Validation

```bash
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_pilot_guard.sh --level L2
```
