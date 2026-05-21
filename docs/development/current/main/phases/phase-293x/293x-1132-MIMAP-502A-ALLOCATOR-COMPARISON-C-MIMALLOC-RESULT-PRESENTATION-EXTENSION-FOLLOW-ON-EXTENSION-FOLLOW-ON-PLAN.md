# 293x-1132 MIMAP-502A Allocator Comparison C Mimalloc Result Presentation Extension Follow-On Extension Follow-On Plan

Status: landed
Date: 2026-05-22

## Decision

Define the next extension-follow-on boundary over the landed MIMAP-498A
presentation extension follow-on extension pilot report.

This row should specify what a later extension-follow-on owner is allowed to
state, which follow-on-extension pilot fields are normative inputs, and which
stop lines remain closed. It must not open another behavior row itself.

## Scope

- Define the owner boundary for a later presentation extension follow-on
  extension follow-on row.
- Record the required input evidence from the landed MIMAP-498A presentation
  extension follow-on extension pilot report.
- Define the stable report fields for a future presentation extension
  follow-on extension follow-on owner.
- Keep this row planning-only; do not execute or publish a wider presentation
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

A later presentation extension follow-on extension follow-on owner may consume
only the accepted MIMAP-498A follow-on-extension boundary:

- `pilot_present == 1`
- `accepted == 1`
- `follow_on_extension_present == 1`
- `accepted_extension_follow_on_present == 1`
- `follow_on_extension_ready == 1`
- `follow_on_extension_memory_outcome_present == 1`
- `follow_on_extension_metrics_snapshot_present == 1`
- `extension_follow_on_present == 1`
- `extension_present == 1`
- `follow_on_present == 1`
- `conclusion_present == 1`
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

## Future Extension Follow-On Contract

The later presentation extension follow-on extension follow-on row may:

- classify whether the landed follow-on-extension-ready evidence is sufficient
  for an additional follow-on report
- publish a narrow scalar follow-on report derived from landed pilot fields
- restate the provisional memory-side outcome and metrics snapshot in deeper
  follow-on form only

The later presentation extension follow-on extension follow-on row must not:

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

1. Define the future presentation extension follow-on extension follow-on owner
   boundary over the landed presentation extension follow-on extension pilot
   report.
2. Record the exact accepted evidence and closed stop-line inputs that a later
   row may consume.
3. Keep the plan row free of benchmark reruns and broader execution behavior.
4. Select a later implementation row only after the plan is settled.

## Result

Landed. MIMAP-503A is selected as the next row-selection card.

## Next

MIMAP-503A should choose whether the next row is a presentation extension
follow-on extension follow-on pilot, a plan closeout, or a presentation-only
extension row.
