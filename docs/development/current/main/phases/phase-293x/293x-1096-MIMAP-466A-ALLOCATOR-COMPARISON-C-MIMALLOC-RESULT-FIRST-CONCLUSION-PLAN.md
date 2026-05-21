# 293x-1096 MIMAP-466A Allocator Comparison C Mimalloc Result First Conclusion Plan

Status: landed
Date: 2026-05-21

## Decision

Define the first performance / memory-use conclusion boundary over the landed
MIMAP-464A first conclusion preflight report.

This row should specify what a later conclusion owner is allowed to state, which
preflight fields are normative inputs, and which stop lines remain closed. It
must not make the final conclusion itself.

## Scope

- Define the owner boundary for a later first conclusion row.
- Record the required input evidence from the landed MIMAP-464A preflight report.
- Define the stable report fields for a future conclusion owner.
- Keep this row planning-only; do not execute or publish the final conclusion.

## Stop Lines

- No repeated or heavy benchmark pack.
- No performance conclusion.
- No memory-use conclusion.
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

A later first conclusion owner may consume only the accepted MIMAP-464A
preflight boundary:

- `preflight_present == 1`
- `accepted == 1`
- `conclusion_ready == 1`
- `comparison_available == 1`
- `hako_ready_execution_present == 1`
- `c_ready_evidence_present == 1`
- `memory_evidence_present == 1`
- `allocation_count_delta > 0`
- `requested_bytes_delta > 0`

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

## Future Conclusion Contract

The later first conclusion row may:

- classify whether the landed evidence is sufficient for an allocator conclusion
- publish a narrow scalar conclusion report derived from landed preflight fields
- state a provisional comparison outcome in model space only

The later first conclusion row must not:

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

1. Define the future first conclusion owner boundary over the landed preflight
   report.
2. Record the exact accepted evidence and closed stop-line inputs that a later
   conclusion row may consume.
3. Keep the plan row free of benchmark reruns and final conclusions.
4. Select a later conclusion implementation row only after the plan is settled.

## Result

Landed. MIMAP-467A is selected as the next row-selection card.

## Next

MIMAP-467A should choose whether the next row is a first conclusion pilot, a
plan closeout, or a presentation-only shaping row after the first conclusion
contract is fixed.
