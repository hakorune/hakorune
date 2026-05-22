# 293x-1188 MIMAP-558A Allocator Comparison C Mimalloc Result Presentation-Only Extension

Status: landed
Date: 2026-05-22

## Decision

Shape the presentation-only extension boundary over the landed MIMAP-552A
comparison-ready pilot report and the closed MIMAP-550A explicit C mimalloc
comparison plan seam.

This row defines what a later presentation row may display from the stabilized
comparison-ready pack without changing the comparison contract or reopening any
closed allocator/provider seam.

## Scope

- Define the later presentation-only extension owner boundary over the landed
  MIMAP-552A pilot.
- Record the stable comparison-ready and shared contract fields a presentation
  row may consume.
- Preserve all benchmark, allocator, provider, and explicit runner stop-line
  fields as closed.
- Keep this row shaping-only; do not add new execution or change the stabilized
  comparison contract.

## Presentation Inputs

A later presentation row may consume only these landed MIMAP-552A pilot facts:

- `comparison_ready_present == 1`
- `accepted == 1`
- `hako_alloc_report_contract_present == 1`
- `c_mimalloc_runner_contract_present == 1`
- `shared_workload_id_present == 1`
- `memory_evidence_fields_present == 1`
- `comparison_preconditions_present == 1`
- `allocator_id`
- `runner_kind`
- `workload_id`
- `allocation_count`
- `free_count`
- `requested_bytes`
- `peak_rss_bytes`
- `steady_rss_bytes`
- `exit_code`
- `evidence_complete == 1`
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
- No explicit C mimalloc runner execution.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Validation

Validation profile: `L0 planning`.

Validated:

```bash
bash tools/checks/current_state_pointer_guard.sh
```

## Result

Landed. MIMAP-559A is selected as the next row-selection card.

## Next

MIMAP-559A should decide whether the next row is a presentation-only extension
pilot, a deeper explicit C mimalloc runner planning row, or a shaping closeout.
