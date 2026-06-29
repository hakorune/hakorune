---
Status: Active
Decision: accepted
Date: 2026-06-29
Scope: Document and guard the runner-role split and the task packs that may
  resume Source Selfhost after the wider route-selection design stop.
Related:
  - docs/development/current/main/phases/phase-296x/1799-SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001.md
  - docs/development/current/main/design/source-selfhost-runner-and-route-task-breakdown-ssot.md
  - docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-runner-and-route-task-breakdown-v0.json
  - tools/checks/rust_lifecycle_source_selfhost_runner_and_route_task_breakdown_guard.sh
---

# SOURCE-SELFHOST-RUNNER-AND-ROUTE-TASK-BREAKDOWN-001

## Goal

Keep the current Source Selfhost design stop intact while making the next
task packs explicit enough to resume implementation later without manual
family selection. Also fix the runner vocabulary so Python-to-Hako projector
migration does not wait for a new future interpreter and does not make VM,
interpreter, or EXE/AOT a second semantic owner.

```text
docs_only_closeout = forbidden
code_or_guard_delta_required = 1
```

## Resolution

```text
output_contract:
  rust-lifecycle-source-selfhost-runner-and-route-task-breakdown-v0

current_blocker_preserved:
  SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001

runner_decision:
  runners validate selected Hako meaning; runners do not own compiler meaning

task_breakdown:
  P0 runner/task breakdown
  P1 wider route-selection basis
  P2 machine-derived route repair
  P3 selected-family HakoAdopted decision
  P4 projector promotion lane
```

## Key Rules

```text
single_hako_meaning_source = 1
runner_semantic_owner = 0
exe_aot_gate_is_semantic_owner = 0
vm_hako_co_mainline_claim = 0
future_interpreter_required_for_projector_migration = 0
manual_family_selection = 0
```

The selected `.hako` projector/source is the meaning source for the selected
stage. EXE/AOT, current `vm-hako`, and any future interpreter are validation
or usability lanes only.

## Task Queue

```text
1. SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-001
   Define the basis that can derive a wider native route-family candidate.

2. <ROUTE-FAMILY>-ROUTE-MATRIX-REPAIR-001
   Only if the basis finds exactly one machine-derived repair row.

3. <SELECTED-FAMILY>-HAKO-ADOPTION-DECISION-001
   Only after a family is machine-selected as CandidateEligible.

4. <PROJECTOR-FAMILY>-HAKO-SHADOW-PROMOTION-DECISION-001
   Converter retirement lane; not a family-specific HakoAdopted candidate.
```

## Non-Claims

```text
Source Selfhost = 0
Rust deletion = 0
runtime fallback = 0
new backend route = 0
new ABI = 0
new Python SemanticProjector = 0
future interpreter activation = 0
manual family selection = 0
```

## Closeout

```text
output_contract=rust-lifecycle-source-selfhost-runner-and-route-task-breakdown-v0
current_blocker_preserved=SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001
single_hako_meaning_source=1
runner_semantic_owner=0
future_interpreter_required_for_projector_migration=0
task_packs_named=1
manual_family_selection=0
runtime_fallback=0
new_backend_route=0
new_abi=0
source_selfhost_claim=0
summary=ok
```
