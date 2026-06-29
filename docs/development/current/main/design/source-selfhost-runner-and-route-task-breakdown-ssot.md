---
Status: SSOT
Decision: accepted
Date: 2026-06-29
Scope: Source Selfhost design-stop recovery task breakdown and runner-role
  separation after the wider route-selection stop.
Related:
  - docs/development/current/main/design/artifact-policy-ssot.md
  - docs/development/current/main/design/vm-active-lane-retirement-ssot.md
  - docs/development/current/main/design/derived-to-native-hako-artifact-model-ssot.md
  - docs/development/current/main/design/rust-to-hako-converter-implementation-role-ssot.md
  - docs/development/current/main/design/mirbuilder-selfhost-checkpoint-roadmap-ssot.md
  - docs/development/current/main/phases/phase-296x/1799-SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001.md
---

# Source Selfhost Runner and Route Task Breakdown

## Purpose

Keep the Source Selfhost lane stopped until there is either a
consultation-gated wider route-selection basis or a machine-derived route
repair. This document also fixes the runner vocabulary so `.hako` projectors,
VM/interpreter candidates, and EXE/AOT gates do not become competing semantic
owners.

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

This is not a converter failure. The converter has produced substantial
generated `.hako`, the minimal composed path is green, and several narrow
families have adoption decisions. The current blocker is route-family
selection: no machine-derived native adoption candidate remains after already
adopted, bounded-only, support-lane, and consultation-gated rows are excluded.

## Runner Role Separation

There is one compiler meaning source per stage. Runners validate that meaning;
they do not own it.

```text
semantic owner:
  native .hako source when HakoAdopted
  Hako projector when a converter stage is HakoMainline
  Rust source only while the family remains RustPrimary / bootstrap oracle

shadow/reference runner:
  may execute HakoShadow outputs to prove parity
  must not add policy, fallback, or family-specific interpretation

EXE/AOT:
  mainline / distribution / product validation gate
  does not become a separate semantic interpretation of projector facts

current vm-hako:
  internal semantic witness / debug / bootstrap-proof lane
  not a co-mainline product lane

future interpreter:
  reserved usability lane
  not active, not required for Python-to-Hako projector migration
```

The migration from Python projector logic to `.hako` does not wait for a new
future interpreter. It should run through the existing selected Hako execution
gates for parity and EXE/AOT where the active card requires it. A future
interpreter may improve usability, but it must not create a second place where
compiler policy is interpreted. Current VM/product-lane retirement remains
owned by `vm-active-lane-retirement-ssot.md` and `artifact-policy-ssot.md`.

## Single-Source Rule

```text
one .hako source/projector
  -> multiple validation lanes are allowed

multiple runner-specific semantic implementations
  -> forbidden
```

Allowed:

```text
Python oracle compares canonical JSON with HakoShadow output
HakoMainline projector is selected for a stage
EXE/AOT gate validates the selected `.hako` path
vm-hako observes/debugs the same selected `.hako` meaning
```

Forbidden:

```text
backend/interpreter reads compiler policy facts as a second semantic owner
runtime try-Hako-then-Rust fallback
VM-only projector behavior
EXE-only projector behavior
future-interpreter-only projector behavior
new Python SemanticProjector growth
```

## Task Breakdown

The design stop is split into task packs. Only the first pack is this
documentation/guard row.

```text
P0. Runner and route task breakdown
  status = this row
  output = source-selfhost-runner-and-route-task-breakdown-v0
  purpose = keep runner roles separate and name the allowed recovery packs

P1. Wider route-selection basis
  token = SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-001
  purpose = define the basis used to derive the next native route-family
  output = CandidateBasis | NeedRouteRepair | KeepStopped

P2. Machine-derived route repair
  token = <ROUTE-FAMILY>-ROUTE-MATRIX-REPAIR-001
  purpose = repair exactly one route-family row when the basis points at a
    concrete inconsistency
  output = CandidateEligible | Parked(reason)

P3. Family adoption decision
  token = <SELECTED-FAMILY>-HAKO-ADOPTION-DECISION-001
  purpose = Adopt | Defer(reason) | Reject(reason) for the machine-selected
    family

P4. Projector promotion lane
  token = <PROJECTOR-FAMILY>-HAKO-SHADOW-PROMOTION-DECISION-001
  purpose = move support-lane projectors toward HakoMainline or keep them as
    oracle/provenance
```

P1 and P2 are Source Selfhost recovery. P4 is converter implementation cleanup;
it can reduce Python semantic ownership but it is not a family-specific
HakoAdopted candidate by itself.

## Decision Tree

```text
if route matrix has a concrete inconsistency:
  select MachineDerivedRouteRepair

elif wider route-selection basis can derive exactly one family:
  select <SELECTED-FAMILY>-HAKO-ADOPTION-DECISION-001

elif only support-lane projectors remain:
  select projector promotion batch only as converter-retirement work
  keep Source Selfhost stopped

else:
  keep SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001 active
```

Manual family selection is never a resume condition.

## Non-Claims

```text
Source Selfhost = 0
Rust deletion = 0
new backend route = 0
new ABI = 0
runtime fallback = 0
new Python SemanticProjector = 0
future interpreter activation = 0
current vm-hako promotion to product mainline = 0
support-lane projector as adoption candidate = 0
```

## Acceptance

```text
runner_semantic_owner = 0
single_hako_meaning_source = 1
future_interpreter_required_for_projector_migration = 0
exe_aot_gate_is_semantic_owner = 0
vm_hako_co_mainline_claim = 0
manual_family_selection = 0
machine_derived_route_repair_allowed = 1
consultation_gated_wider_route_selection = 1
task_packs_named = 1
```
