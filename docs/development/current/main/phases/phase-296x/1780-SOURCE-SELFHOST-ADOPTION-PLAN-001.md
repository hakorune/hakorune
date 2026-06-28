---
Status: Active
Decision: accepted
Date: 2026-06-28
Scope: Make the source-selfhost adoption plan explicit as a machine-checkable
  family-breadth gate after the artifact-selfhost checkpoint and the minimal
  path mainline pilot.
Related:
  - docs/development/current/main/design/mirbuilder-selfhost-checkpoint-roadmap-ssot.md
  - docs/development/current/main/design/derived-to-native-hako-artifact-model-ssot.md
  - docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/1779-ARTIFACT-SELFHOST-CHECKPOINT-001.md
  - docs/development/current/main/phases/phase-296x/296x-1763-MIRBUILDER-MINIMAL-PATH-MAINLINE-READINESS-RESOLVER-001.md
  - docs/development/current/main/phases/phase-296x/296x-1764-MIRBUILDER-MINIMAL-PATH-MAINLINE-PILOT-001.md
  - docs/development/current/main/phases/phase-296x/1770-MIRBUILDER-CONTEXT-HAKO-NATIVE-SOURCE-OWNER-001.md
  - docs/development/current/main/phases/phase-296x/1771-MIRBUILDER-CONTEXT-HAKO-ADOPTION-DECISION-001.md
  - docs/development/current/main/phases/phase-296x/1775-MIRBUILDER-NEXT-HAKO-ADOPTION-CANDIDATE-SELECTION-001.md
  - docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-adoption-plan-v0.json
  - tools/checks/rust_lifecycle_source_selfhost_adoption_plan_guard.sh
---

# SOURCE-SELFHOST-ADOPTION-PLAN-001

## Goal

Make the source-selfhost adoption plan machine-checkable after the artifact
selfhost checkpoint and the minimal-path mainline pilot. This row keeps the
plan explicit, preserves the blocked candidate-pool provenance, and ensures
the next family-specific `HakoAdopted` decision is derived from route-matrix
evidence rather than hand-picked.

```text
docs_only_closeout = forbidden
code_or_artifact_delta_required = 1
```

## Authority

```text
docs/development/current/main/design/mirbuilder-selfhost-checkpoint-roadmap-ssot.md
docs/development/current/main/design/derived-to-native-hako-artifact-model-ssot.md
docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-adoption-plan-v0.json
docs/development/current/main/CURRENT_STATE.toml
docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md
docs/development/current/main/phases/phase-296x/1779-ARTIFACT-SELFHOST-CHECKPOINT-001.md
docs/development/current/main/phases/phase-296x/296x-1764-MIRBUILDER-MINIMAL-PATH-MAINLINE-PILOT-001.md
docs/development/current/main/phases/phase-296x/1770-MIRBUILDER-CONTEXT-HAKO-NATIVE-SOURCE-OWNER-001.md
docs/development/current/main/phases/phase-296x/1771-MIRBUILDER-CONTEXT-HAKO-ADOPTION-DECISION-001.md
docs/development/current/main/phases/phase-296x/1775-MIRBUILDER-NEXT-HAKO-ADOPTION-CANDIDATE-SELECTION-001.md
tools/checks/rust_lifecycle_source_selfhost_adoption_plan_guard.sh
tools/checks/current_state_pointer_guard.sh
```

## Required Delta

```text
consume the checkpoint and mainline-pilot evidence as provenance
keep the source-selfhost plan separate from any concrete HakoAdopted decision
keep the next family-specific HakoAdopted decision machine-derived from route
matrix evidence, not manual selection
keep Python as oracle/bootstrap only
keep Rust as compatibility/reference source until a family is actually adopted
```

## Acceptance

```text
source_selfhost_adoption_plan_token = SOURCE-SELFHOST-ADOPTION-PLAN-001
artifact_selfhost_checkpoint_provenance = 1
mainline_pilot_provenance = 1
candidate_pool_state = Blocked
manual_family_selection = 0
next_family_specific_hakoadopted_decision_machine_derived = 1
python_oracle_retained = 1
rust_compat_reference_retained = 1
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
source_selfhost_claim = 0
```

## Non-Claims

```text
Source Selfhost = 0
HakoAdopted = 0
Rust bootstrap removal = 0
new Python SemanticProjector = 0
manual family selection = 0
runtime fallback = 0
new backend route = 0
new ABI = 0
```

## Next Follow-On

When this plan lands, proceed with the next machine-derived family-specific
`HakoAdopted` decision selected from the repaired route matrix. Do not hand
pick the family in task-order.

## Closeout

```text
output_contract=rust-lifecycle-source-selfhost-adoption-plan-v0
source_selfhost_adoption_plan_token=SOURCE-SELFHOST-ADOPTION-PLAN-001
current_blocker_token=SOURCE-SELFHOST-ADOPTION-PLAN-001
latest_card=SOURCE-SELFHOST-ADOPTION-PLAN-001
artifact_selfhost_checkpoint_provenance=1
mainline_pilot_provenance=1
candidate_pool_state=Blocked
manual_family_selection=0
next_family_specific_hakoadopted_decision_machine_derived=1
python_oracle_retained=1
rust_compat_reference_retained=1
runtime_fallback=0
new_backend_route=0
new_abi=0
source_selfhost_claim=0
summary=ok
```
