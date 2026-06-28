---
Status: Active
Decision: accepted
Date: 2026-06-28
Scope: Make the artifact-selfhost checkpoint explicit as a machine-checkable
  execution-graph boundary.
Related:
  - docs/development/current/main/design/mirbuilder-selfhost-checkpoint-roadmap-ssot.md
  - docs/development/current/main/design/fixtures/rust-lifecycle/minimal-mirbuilder-execution-path-semantic-closure-report-v0.json
  - docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-minimal-path-composed-execution-continuation-v2.json
  - docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-minimal-path-composed-prefix-advance-v1.json
  - docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-minimal-execution-path-frontier-resolution-v0.json
  - docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-minimal-path-mainline-readiness-resolution-v0.json
  - docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-minimal-path-mainline-pilot-v0.json
  - docs/development/current/main/design/fixtures/rust-lifecycle/next-hako-adoption-candidate-selection-v0.json
  - docs/development/current/main/design/fixtures/rust-lifecycle/artifact-selfhost-checkpoint-v0.json
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md
  - tools/checks/rust_lifecycle_artifact_selfhost_checkpoint_guard.sh
---

# ARTIFACT-SELFHOST-CHECKPOINT-001

## Goal

Make the artifact-selfhost checkpoint machine-checkable from the composed
execution evidence already landed in the MirBuilder lane. This row keeps the
checkpoint explicit, preserves the blocked derived-mainline candidate pool as
provenance, and does not claim source selfhost.

```text
docs_only_closeout = forbidden
code_or_artifact_delta_required = 1
```

## Authority

```text
docs/development/current/main/design/mirbuilder-selfhost-checkpoint-roadmap-ssot.md
docs/development/current/main/design/fixtures/rust-lifecycle/minimal-mirbuilder-execution-path-semantic-closure-report-v0.json
docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-minimal-path-composed-execution-continuation-v2.json
docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-minimal-path-composed-prefix-advance-v1.json
docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-minimal-execution-path-frontier-resolution-v0.json
docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-minimal-path-mainline-readiness-resolution-v0.json
docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-minimal-path-mainline-pilot-v0.json
docs/development/current/main/design/fixtures/rust-lifecycle/next-hako-adoption-candidate-selection-v0.json
docs/development/current/main/design/fixtures/rust-lifecycle/artifact-selfhost-checkpoint-v0.json
docs/development/current/main/CURRENT_STATE.toml
docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md
tools/rust_lifecycle/mirbuilder_minimal_execution_path_semantic_closure_report.py
tools/rust_lifecycle/mirbuilder_minimal_path_composed_execution.py
tools/rust_lifecycle/mirbuilder_minimal_path_composed_prefix_advance.py
tools/rust_lifecycle/mirbuilder_minimal_path_composed_execution_continuation.py
tools/rust_lifecycle/mirbuilder_minimal_execution_path_frontier_resolution.py
tools/rust_lifecycle/mirbuilder_minimal_path_mainline_readiness_resolver.py
tools/rust_lifecycle/mirbuilder_next_hako_adoption_candidate_selection.py
tools/checks/current_state_pointer_guard.sh
tools/checks/rust_lifecycle_artifact_selfhost_checkpoint_guard.sh
```

## Required Delta

```text
consume the semantic closure, composed prefix, frontier, readiness, and
candidate-selection evidence as checkpoint inputs
keep the composed execution graph explicit and machine-derived
keep the derived-mainline candidate pool blocked and reported as provenance
keep Python as oracle/bootstrap only
add or update a checkpoint-specific guard / fixture pair so the checkpoint is
machine-checkable
```

## Acceptance

```text
artifact_selfhost_checkpoint_token = ARTIFACT-SELFHOST-CHECKPOINT-001
composed_execution_evidence_consumed = 1
same_state_handoff_observed = 1
generated_hako_executable_closure = Closed
next_queue_item_machine_derived = 1
candidate_pool_state = Blocked
manual_next_owner_selection = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
source_selfhost_claim = 0
checkpoint_guard_green = 1
```

## Non-Claims

```text
Source Selfhost = 0
HakoAdopted = 0
Rust bootstrap removal = 0
new Python SemanticProjector = 0
manual next-owner selection = 0
runtime fallback = 0
new backend route = 0
new ABI = 0
```

## Next Follow-On

When this checkpoint lands, continue with the existing bounded lanes in order:

1. `MAINLINE-SELFHOST-PILOT-001`
2. `SOURCE-SELFHOST-ADOPTION-PLAN-001`

## Closeout

```text
output_contract=rust-lifecycle-artifact-selfhost-checkpoint-v0
artifact_selfhost_checkpoint_token=ARTIFACT-SELFHOST-CHECKPOINT-001
current_blocker_token=ARTIFACT-SELFHOST-CHECKPOINT-001
latest_card=ARTIFACT-SELFHOST-CHECKPOINT-001
candidate_pool_state=Blocked
composed_execution_evidence_consumed=1
same_state_handoff_observed=1
generated_hako_executable_closure=Closed
next_queue_item_machine_derived=1
manual_next_owner_selection=0
runtime_fallback=0
new_backend_route=0
new_abi=0
source_selfhost_claim=0
summary=ok
```
