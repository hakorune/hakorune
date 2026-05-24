---
Status: Landed
Date: 2026-05-25
Scope: define allocator-body timing vocabulary before C or .hako implementation.
Related:
  - docs/development/current/main/phases/phase-295x/295x-74-MIMALLOC-COMPARISON-PROCESS-TIMING-PRESENTATION-CLOSEOUT.md
  - docs/development/current/main/design/mimalloc-comparison-execution-ssot.md
---

# 295x-75 Allocator Body Timing Contract

## Blocker

```text
MIMALLOC-COMPARISON-ALLOCATOR-BODY-TIMING-CONTRACT-295X-001
```

## Contract

Process timing and allocator-body timing are separate evidence surfaces.

Existing process timing remains:

```text
timing_repeat_kind=process-invocation-v0
timing_claim_kind=process-repeat-presentation-only
allocator_body_timing=0
```

Allocator-body timing must use a different repeat kind:

```text
body_timing_repeat_kind=workload-body-monotonic-v0
body_timing_scope=allocator-workload-body
body_elapsed_ns
body_elapsed_min_ns
body_elapsed_median_ns
body_elapsed_max_ns
```

The body timer starts after process/runtime setup and stops before report
formatting. It may include the selected workload loop and allocator calls only.

## First Implementation Order

Select C-side body timing first:

```text
MIMALLOC-COMPARISON-C-BODY-TIMING-PILOT-295X-001
```

The C runner can add monotonic workload-body timing without changing Hakorune
runtime behavior. `.hako` body timing should remain a follow-on selection
because it may require an app-visible clock seam or runtime support.

## Required Fields For Body Timing Evidence

```text
c_body_timing_available=1
hako_body_timing_available=0
body_timing_repeat_kind=workload-body-monotonic-v0
body_timing_scope=allocator-workload-body
body_timing_is_process_timing=0
process_timing_preserved=1
winner_claim=0
```

The first C pilot may expose only one workload, preferably
`representative-small-block-v0`, while preserving allocation/free/requested
count parity and all stop-line fields.

## Stop Line

This row does not add timers, compute speed winners, compute RSS winners,
require timing parity, change `.hako` runtime behavior, add an app-visible
clock, make `empty` the default runtime config, resume allocator semantics
porting, or open provider/DLL/replacement/hook/global allocator seams.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_allocator_body_timing_contract_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
