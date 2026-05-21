# 293x-1108 MIMAP-478A Allocator Comparison C Mimalloc Result Presentation Follow-On Plan

Status: landed
Date: 2026-05-21

## Decision

Define the next presentation boundary over the landed MIMAP-474A
presentation-only conclusion pilot report.

This row should specify what a later presentation follow-on owner is allowed to
state, which presentation-pilot fields are normative inputs, and which stop
lines remain closed. It must not open another behavior row itself.

## Scope

- Define the owner boundary for a later presentation follow-on row.
- Record the required input evidence from the landed MIMAP-474A presentation
  pilot report.
- Define the stable report fields for a future presentation follow-on owner.
- Keep this row planning-only; do not execute or publish a broader presentation
  result.

## Stop Lines

- No repeated or heavy benchmark pack.
- No process allocator replacement.
- No hook installation.
- No backend matcher additions.
- No `#[global_allocator]`.
- No provider package / DLL generation.
- No hidden env, implicit discovery, or process-global activation config.
- No source-level worker-local or concurrency surface.
- No worker/thread execution.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Normative Input Evidence

A later presentation follow-on owner may consume only the accepted MIMAP-474A
presentation boundary:

- `pilot_present == 1`
- `accepted == 1`
- `presentation_present == 1`
- `conclusion_present == 1`
- `accepted_pilot_present == 1`
- `provisional_memory_conclusion_present == 1`
- `provisional_memory_winner`
- `provisional_memory_reason`
- `comparison_available == 1`
- `memory_evidence_present == 1`
- `hako_allocation_count`
- `hako_requested_bytes`
- `c_allocation_count`
- `c_requested_bytes`
- `c_peak_rss_bytes`
- `allocation_count_delta`
- `requested_bytes_delta`

It must also preserve the closed stop-line inputs:

- `performance_conclusion_made == 0`
- `memory_conclusion_made == 0`
- `repeated_benchmark_executed == 0`
- `process_replacement_executed == 0`
- `hook_installed == 0`
- `backend_matcher_added == 0`
- `global_allocator_installed == 0`
- `hidden_discovery_used == 0`
- `provider_package_generated == 0`
- `would_replace_host_allocator == 0`
- `would_install_hook == 0`
- `would_add_backend_matcher == 0`
- `would_run_thread == 0`

## Future Presentation Contract

The later presentation follow-on row may:

- classify whether the landed presentation-only evidence is sufficient for a
  broader presentation report
- publish a narrow scalar presentation report derived from landed pilot fields
- restate the provisional memory-side outcome in presentation form only

The later presentation follow-on row must not:

- rerun or expand benchmark packs
- reopen allocator/provider activation ladders
- install process allocator replacement, hooks, backend matchers, or
  `#[global_allocator]`
- materialize cross-function `Result` direct ABI or runtime sums

## Validation

Validation profile: `L0 planning`.

Validated:

```bash
bash tools/checks/current_state_pointer_guard.sh
```

## Task Order

1. Define the future presentation follow-on owner boundary over the landed
   presentation-only pilot report.
2. Record the exact accepted evidence and closed stop-line inputs that a later
   row may consume.
3. Keep the plan row free of benchmark reruns and broader execution behavior.
4. Select a later implementation row only after the plan is settled.

## Result

Landed. MIMAP-479A is selected as the next row-selection card.

## Next

MIMAP-479A should choose whether the next row is a presentation follow-on pilot,
a plan closeout, or a presentation-only extension row.
