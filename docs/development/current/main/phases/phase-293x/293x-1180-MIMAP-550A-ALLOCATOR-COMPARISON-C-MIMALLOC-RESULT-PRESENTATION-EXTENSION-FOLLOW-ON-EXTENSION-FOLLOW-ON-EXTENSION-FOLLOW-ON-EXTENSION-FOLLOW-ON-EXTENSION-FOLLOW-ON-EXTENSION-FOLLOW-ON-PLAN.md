# 293x-1180 MIMAP-550A Allocator Comparison C Mimalloc Result Presentation Extension Follow-On Extension Follow-On Extension Follow-On Extension Follow-On Extension Follow-On Extension Follow-On Extension Follow-On Plan

Status: landed
Date: 2026-05-22

## Decision

Define the next follow-on comparison boundary over the completed MIMAP-548A
deeper-extension pack.

This row should specify how a later comparison-ready owner may consume the
landed MIMAP-546A/548A evidence, what a future explicit C mimalloc runner is
allowed to report, and which output fields must match the hako_alloc side
before any later behavior row can claim the reports are comparable. It must
remain planning-only and keep allocator/provider ladders closed.

## Scope

- Define the role of the future explicit C mimalloc runner as an external
  executable evidence source rather than a provider, hook, allocator
  replacement, or `#[global_allocator]` path.
- Record the stable hako_alloc-side evidence fields that a later comparison row
  may consume from the landed MIMAP-546A/548A pack.
- Define the shared comparison-ready output contract and the meaning of the
  memory-use fields a later row may publish.
- Define the comparison preconditions that must hold before any later row may
  claim the hako_alloc and C mimalloc reports are comparable.
- Keep this row planning-only; do not rerun benchmarks, execute a C runner, or
  publish a winner.

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

A later presentation extension follow-on extension follow-on extension
follow-on extension follow-on extension follow-on extension follow-on
extension-follow-on owner may consume only the accepted MIMAP-546A
deeper-extension-ready boundary:

- `pilot_present == 1`
- `accepted == 1`
- `follow_on_extension_follow_on_extension_follow_on_extension_follow_on_extension_present == 1`
- `accepted_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_present == 1`
- `follow_on_extension_follow_on_extension_follow_on_extension_follow_on_extension_ready == 1`
- `follow_on_extension_follow_on_extension_follow_on_extension_follow_on_extension_memory_outcome_present == 1`
- `follow_on_extension_follow_on_extension_follow_on_extension_follow_on_extension_metrics_snapshot_present == 1`
- `extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_present == 1`
- `follow_on_extension_follow_on_extension_follow_on_extension_present == 1`
- `extension_follow_on_extension_follow_on_extension_follow_on_present == 1`
- `follow_on_extension_follow_on_extension_present == 1`
- `extension_follow_on_extension_follow_on_present == 1`
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

## Future Explicit C Mimalloc Runner Contract

The future C mimalloc side remains an explicit external executable evidence
source. A later row may define or implement it only under this contract:

- it is launched explicitly rather than injected as a provider or process-wide
  allocator replacement
- it does not rely on hooks, `LD_PRELOAD`, hidden discovery, DLL/provider
  packages, or `#[global_allocator]`
- it reports stable structured output over stdout or an equivalent explicit
  report channel
- it uses the same workload identifier family as the hako_alloc side
- it publishes memory-use evidence only as data, not as a performance winner

## Shared Comparison-Ready Report Contract

Any later comparison-ready row must converge both hako_alloc and explicit C
mimalloc evidence onto the same semantic report shape.

Minimum shared fields:

- `allocator_id`
- `runner_kind`
- `workload_id`
- `allocation_count`
- `free_count`
- `requested_bytes`
- `peak_rss_bytes`
- `steady_rss_bytes`
- `exit_code`
- `evidence_complete`
- `reason`

The future hako_alloc side may continue to carry richer lane-local fields, but
the fields above are the stable comparison contract that later rows must honor.

## Memory Evidence Semantics

The future comparison rows must interpret memory evidence conservatively:

- `requested_bytes`: total payload bytes requested by the workload
- `allocation_count`: number of allocation calls issued by the workload
- `free_count`: number of frees issued by the workload
- `peak_rss_bytes`: peak resident/working-set style measurement over the
  process lifetime
- `steady_rss_bytes`: resident/working-set style measurement after workload
  cleanup and a short steady checkpoint

This row fixes the field semantics only. It does not require a specific OS API
or measurement implementation yet.

## Comparison Preconditions

A later row may claim the hako_alloc and explicit C mimalloc reports are
comparison-ready only when:

- both reports set `evidence_complete == true`
- both reports use the same `workload_id`
- both reports publish `allocation_count`, `requested_bytes`, and
  `peak_rss_bytes`
- both reports preserve the closed stop-line fields from the normative input
- neither report claims a winner or materializes a performance or memory
  conclusion yet

## Future Follow-On Contract

The later presentation extension follow-on extension follow-on extension
follow-on extension follow-on extension follow-on extension follow-on
extension-follow-on row may:

- classify whether the landed deeper-extension-ready evidence is sufficient for
  a comparison-ready report
- define the workload identity contract shared by hako_alloc and explicit C
  mimalloc evidence
- validate accepted, missing-contract, and stop-line-violating comparison
  reports in a narrow pilot

The later row must not:

- rerun or expand benchmark packs
- execute worker/thread paths
- reopen allocator/provider activation ladders
- install process allocator replacement, hooks, backend matchers, or
  `#[global_allocator]`
- generate provider packages or DLL surfaces
- publish a memory or performance winner

## Validation

Validation profile: `L0 planning`.

Validated:

```bash
bash tools/checks/current_state_pointer_guard.sh
```

## Task Order

1. Define the explicit C mimalloc runner role and the shared comparison-ready
   report schema over the landed MIMAP-546A/548A pack.
2. Record the exact accepted evidence and closed stop-line inputs that a later
   row may consume.
3. Fix the memory-use field semantics and comparison preconditions without
   executing a runner or benchmark pack.
4. Select a later implementation row only after the plan is settled.

## Result

Landed. MIMAP-551A is selected as the next row-selection card.

## Next

MIMAP-551A should choose whether the next row is a presentation extension
follow-on extension follow-on extension follow-on extension follow-on
extension follow-on extension follow-on extension follow-on pilot, a plan
closeout, or a presentation-only extension row.
