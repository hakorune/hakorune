# Hako Alloc Allocator Comparison C Mimalloc Result Explicit Runner Planning Pilot SSOT

Status: Active
Decision: accepted
Updated: 2026-05-22
Owner: MIMAP-566A

## Purpose

Define the terminal explicit-runner planning pilot contract for `phase-293x`.

This SSOT keeps explicit C runner execution closed while fixing accepted,
blocked, missing-input, and stop-line/accidental-execution reasons.

## Input Contract

Input owner report:

```text
HakoAllocAllocatorComparisonCMimallocResultPresentationOnlyExtensionPilotReport
```

Accepted input requires:

- `accepted == 1`
- `reason == 0`
- `presentation_only_extension_present == 1`
- `accepted_input_pack_present == 1`
- `blocked_input_pack_present == 0`

## Output Contract

Output owner report:

```text
HakoAllocAllocatorComparisonCMimallocResultExplicitRunnerPlanningPilotReport
```

Accepted (`reason = 0`) must publish:

- `explicit_runner_planning_pilot_present = 1`
- `explicit_runner_contract_present = 1`
- `external_evidence_source = 1`
- `runner_output_contract_present = 1`
- `memory_evidence_contract_present = 1`
- `schema_anchor_present = 1`
- `runner_execution_performed = 0`
- `benchmark_rerun_executed = 0`
- `process_replacement_executed = 0`
- `hook_installed = 0`
- `backend_matcher_added = 0`
- `global_allocator_installed = 0`
- `provider_package_generated = 0`
- `worker_thread_executed = 0`

## Reason Vocabulary

```text
0 = accepted explicit runner planning pilot
1 = missing planning input
2 = missing runner output contract
3 = missing memory evidence contract
4 = blocked stop-line violation
5 = accidental execution seam opened
```

## Stop-Line Contract

Keep these closed in this row:

- repeated benchmark rerun
- process replacement
- hook/global allocator activation
- provider package generation
- worker/thread execution
- explicit C runner execution
