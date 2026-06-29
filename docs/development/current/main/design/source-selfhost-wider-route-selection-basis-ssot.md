---
Status: SSOT
Decision: accepted
Date: 2026-06-29
Scope: Consultation-gated wider route-selection basis for Source Selfhost and
  the current runner vocabulary that must not become a second semantic owner.
Related:
  - docs/development/current/main/design/source-selfhost-runner-and-route-task-breakdown-ssot.md
  - docs/development/current/main/design/artifact-policy-ssot.md
  - docs/development/current/main/design/vm-active-lane-retirement-ssot.md
  - docs/development/current/main/design/derived-to-native-hako-artifact-model-ssot.md
  - docs/development/current/main/design/rust-to-hako-converter-implementation-role-ssot.md
  - docs/development/current/main/phases/phase-296x/1799-SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001.md
  - docs/development/current/main/phases/phase-296x/1800-SOURCE-SELFHOST-RUNNER-AND-ROUTE-TASK-BREAKDOWN-001.md
---

# Source Selfhost Wider Route-Selection Basis

## Purpose

Define the consultation-gated basis used to decide whether Source Selfhost can
resume through a machine-derived route repair or must remain stopped. This is
not a route-family selection and not an adoption decision. It is the compact
current vocabulary that keeps VM, interpreter, EXE/AOT, and `.hako` runner
roles separated.

one .hako source/projector:
  the selected meaning source for the current stage

## Current State

```text
current_blocker:
  SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001

candidate_pool_state:
  Blocked

reason_token:
  NoEligibleNativeAdoptionCandidate

allowed_resume:
  ConsultationGatedWiderRouteSelection
  MachineDerivedRouteRepair
```

The basis is derived from the route matrix, HakoAdopted decisions, projector
stage-state fixtures, the migration roadmap, and the artifact / runner policy
SSOTs. It deliberately does not use route membership, coverage percentage, or
bundle size as proof.

## Runner Vocabulary

```text
semantic owner:
  native .hako source when HakoAdopted
  Hako projector when a converter stage is HakoMainline
  Rust source only while the family remains RustPrimary / bootstrap oracle

current vm-hako:
  internal semantic witness / debug / bootstrap-proof lane
  not a co-mainline product lane

EXE/AOT:
  mainline / distribution / product validation gate
  not a semantic owner

future interpreter:
  reserved usability lane
  not required for Python-to-Hako projector migration
```

Current runner ownership remains under `artifact-policy-ssot.md` and
`vm-active-lane-retirement-ssot.md`. This document only fixes the current
vocabulary used when Source Selfhost is stopped.

## Basis Categories

```text
ConsultationGatedWiderRouteSelection:
  the route matrix has no eligible native adoption candidate yet, and the
  next step is a consultation-gated wider selection basis rather than a
  family choice by hand

MachineDerivedRouteRepair:
  a concrete route-family row is inconsistent and can be repaired
  mechanically without widening Source Selfhost semantics

KeepSourceSelfhostStopped:
  no eligible candidate remains after the current exclusions, so the queue
  stays stopped
```

## Decision Rule

```text
if route matrix has a concrete inconsistency:
  basis = MachineDerivedRouteRepair

elif exactly one family is candidate-eligible:
  basis = ConsultationGatedWiderRouteSelection

else:
  basis = KeepSourceSelfhostStopped
```

Manual family selection is never part of the basis.

## Non-Claims

```text
Source Selfhost = 0
Rust deletion = 0
runtime fallback = 0
new backend route = 0
new ABI = 0
new Python SemanticProjector = 0
future interpreter activation = 0
current vm-hako promotion to product mainline = 0
support-lane projector as adoption candidate = 0
manual family selection = 0
```

## Acceptance

```text
runner_semantic_owner = 0
single_hako_meaning_source = 1
future_interpreter_required_for_projector_migration = 0
exe_aot_gate_is_semantic_owner = 0
vm_hako_co_mainline_claim = 0
consultation_gated_wider_route_selection = 1
machine_derived_route_repair_allowed = 1
task_pack_p1_named = 1
```
