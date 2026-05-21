# 293x-1102 MIMAP-472A Allocator Comparison C Mimalloc Result Presentation-Only Conclusion Shaping

Status: landed
Date: 2026-05-21

## Decision

Shape the presentation-only boundary over the landed MIMAP-468A first
conclusion pilot report.

This row defines what a later presentation row may display from the provisional
conclusion pack without changing the provisional outcome or reopening any closed
allocator/provider seam.

## Scope

- Define the later presentation owner boundary over the landed MIMAP-468A pilot.
- Record the stable provisional conclusion and metric fields a presentation row
  may consume.
- Preserve all benchmark, allocator, and provider stop-line fields as closed.
- Keep this row shaping-only; do not add new execution or change the provisional
  conclusion outcome.

## Presentation Inputs

A later presentation row may consume only these landed MIMAP-468A pilot facts:

- `conclusion_present == 1`
- `accepted == 1`
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

## Validation

Validation profile: `L0 planning`.

Validated:

```bash
bash tools/checks/current_state_pointer_guard.sh
```

## Result

Landed. MIMAP-473A is selected as the next row-selection card.

## Next

MIMAP-473A should decide whether the next row is a presentation-only conclusion
pilot, a follow-on conclusion plan, or a shaping closeout.
