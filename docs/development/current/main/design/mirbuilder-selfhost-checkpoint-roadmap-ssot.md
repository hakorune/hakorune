---
Status: SSOT
Date: 2026-06-25
Scope: MirBuilder Rust-to-Hako selfhost checkpoint roadmap.
Related:
  - docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md
  - docs/development/current/main/design/derived-to-native-hako-artifact-model-ssot.md
---

# MirBuilder Selfhost Checkpoint Roadmap

This file is a small roadmap. It is not a landed-history ledger and does not
replace the task-order SSOT.

## Three Endpoints

```text
1. Artifact selfhost
   Generated .hako artifacts run together without Rust adapter fallback.

2. Mainline selfhost
   Some normal compiler routes explicitly select generated/native .hako.

3. Source selfhost
   Compiler meaning and edit authority move from Rust to native .hako.
```

Current work has pivoted away from full Rust-to-Hako MirBuilder conversion as
the Source Selfhost main route. Rust is now the bootstrap oracle and parity
reference. The remaining work is small, hand-authored `.hako` native owners
whose correctness is checked against Rust-oracle parity fixtures.

## Current Transition

The work is moving from:

```text
Rust family
  -> generated .hako
  -> standalone AOT green
```

to:

```text
multiple generated families
  -> same module
  -> direct calls
  -> ArrayBox / MapBox / typed object / enum handle returns
  -> downstream generated consumers
```

This is why the current blockers are execution-graph edges, not only source
coverage rows.

## Near-Term Route

The list below is the progression that brought the lane to the current
checkpoint. Most of the support-lane items are already green; the remaining
work is family adoption and library/consultation gating.

```text
1. Same-module ArrayBox return contract
   Close MultiCarrierExitPhi ArrayBox return and restore full matrix green.

2. Newtype ID generator scalarization
   ValueIdGenerator / BasicBlockIdGenerator next and peek.

3. MirBuilder derived context bundle v1
   Compose accepted context families without mainline selection.

4. Minimal MirBuilder execution path
   ValueId creation, BlockId creation, basic block creation,
   Const / Copy / Return emission, and module finalization.

5. DerivedMainline pilot
   Explicitly select generated .hako for a bounded normal route.

6. HakoAdopted decision
   Promote only families whose native .hako source becomes edit authority.
```

## Current Remaining Work

The remaining selfhost work is now a bounded oracle/parity migration, not a
new generic converter route.

```text
current_migration_mode:
  RustOracleParityMigrationPolicyV1

manual_target_selection_allowed:
  yes, for small `.hako` native owner pilots

correctness_authority:
  Rust oracle fixture + parity gate

converter_role:
  read-only inventory / test-vector / parity-fixture / library-subset draft helper

forbidden_current_claims:
  Source Selfhost claim
  Hako adoption before parity
  native seed materialization from converter
  generated artifact as native edit authority
```

## Current Family Status

These family/stage checkpoints are already green or intentionally parked.
They are listed here so the next owner is selected from the remaining work,
not from memory.

```text
prepared-state allocation-policy kernel:
  HakoAdopted decision complete
  native source owner present
  derived mainline retained

BindingContext:
  HakoAdopted decision complete
  native source owner present
  derived mainline retained

BoxCompilationContext / context:
  HakoAdopted decision complete
  native source owner present
  generated artifact retained

ReturnEmission projector:
  HakoShadow parity complete
  ordinary compiler-library landing
  no adoption claim

FunctionRegionStackPop:
  derived artifact and HakoShadow parity complete
  no Source Selfhost claim

Canonical JSON / TextBuilder compiler library:
  landed support lane
  guard green
  no ABI / syntax promotion
```

## Next Candidate Pool

The next family-specific `HakoAdopted` decision must be selected from the
currently selected `DerivedMainline` route manifest entries, not from the
support-lane projector inventory.

Current route-manifest candidate source:

```text
MIRBUILDER-VARIABLE-CONTEXT-ROUTE-MATRIX-CLOSEOUT-001
  derives Closed | Parked(reason) | CandidateEligible | NeedsRouteRepair
```

Explicitly excluded from this pool:

```text
context
  already HakoAdopted; not a next-candidate row

allocation_policy_prepared_state_next_value_id
  already HakoAdopted; not a next-candidate row

binding_context
  already HakoAdopted; not a next-candidate row

variable_context_immutable_borrow
  Denied; replacement row only

variable_context
  currently parked until the full variable_context route matrix is green
  in the generated-to-native adoption matrix

ReturnEmission
FunctionRegionStackPop
compiler-library helpers
  support-lane / shadow-lane families, not yet the next adoption pool

minimal_path_composed_execution_closure
  already selected on the mainline pilot and closed by the composed-prefix
  readiness resolver; not a current candidate row
```

This inventory is a consultation aid, not the adoption decision itself. The
next candidate list must be derived from the route matrix closeout fixture,
not from a handwritten `none` row or route-manifest membership alone.

```text
1. Keep the Python SemanticProjector growth freeze in force.
   No new Python semantic projector growth unless an exception card is
   opened.

2. Continue family-by-family native `.hako` pilots for small owners.
   Target selection may be human/AI chosen, but correctness must be proven by
   Rust-oracle parity before any Hako adoption decision.

3. Add compiler-library support first when Hako ergonomics block progress.
   TextBuilder, CanonicalJson, projector helpers, and other small helper
   libraries stay under lang/src/compiler/lib/.

4. Keep TypeBox ABI / host ABI / syntax / distribution packaging decisions
   consultation-gated.
   Those are explicit design decisions, not default follow-on tasks.
```

## Consultation-Gated Inventory

The following boundary questions should be answered in a design consultation
before any new compiler-library ABI or syntax growth is started:

```text
TypeBox ABI exposure for compiler libraries
host ABI facade for JSON/Text/projector semantics
promotion from lang/src/compiler/lib to lang/src/shared/**
promotion from library helper to language syntax/spec
distribution/package ABI for compiler libraries
hako.buf-backed TextBuilder implementation
```

These are not active implementation tasks yet. They are the points where the
converter bridge would stop being a narrow bridge and start becoming a new
public surface, so they stay consultation-gated.

## Taskization

Treat this roadmap as checkpoint planning, not as the active implementation
queue. The active queue stays in the task-order SSOT.

```text
ARTIFACT-SELFHOST-CHECKPOINT-001
  Complete when generated artifacts can be composed in one execution graph
  without Rust adapter fallback.

MAINLINE-SELFHOST-PILOT-001
  Complete when a bounded normal compiler route explicitly selects
  generated/native .hako.

SOURCE-SELFHOST-ADOPTION-PLAN-001
  Complete when selected families have native .hako as edit authority and
  Rust is frozen as compatibility/reference source for those families.
```

Do not select these checkpoint tasks directly while a concrete semantic red
edge exists. Select the next red edge that moves the nearest checkpoint.

## Progress Signals

The percentages below are planning heuristics, not acceptance criteria:

```text
converter family coverage:
  substantial, but not sufficient for selfhost

artifact selfhost:
  entered; composed execution and adoption are the active work

mainline selfhost:
  selected for the minimal path; wider routes remain consultation-gated

source selfhost:
  future phase, still blocked on native Hako adoption breadth
```

Do not use broad coverage count as proof of selfhost. Use explicit composed
execution gates.

Planning horizon:

```text
artifact selfhost visible checkpoint:
  several semantic slices after same-module ArrayBox return

mainline pilot:
  separate follow-up lane after artifact checkpoint

source selfhost:
  later adoption phase, not a current implementation claim
```

## Required Discipline

```text
Do not:
  add callee-name backend branches
  add C shim special cases to hide converter type errors
  use Rust fallback as success
  treat generated artifacts as semantic/edit authority
  promote bundle size as coverage proof

Do:
  close one execution edge per slice
  keep source facts complete
  require body-wide return-contract agreement
  keep task-order below 800 lines
  stop for design selection before opening a new owner
```

## Next Checkpoint

The next meaningful checkpoint is the first new family-specific HakoAdopted
decision outside the already-adopted allocation-policy kernel, while keeping
the composed execution graph green and the Python semantic freeze in force.

```text
family-by-family HakoAdopted decisions
HakoShadow promotion / retirement token closure
compiler-library support only when ergonomics require it
consultation-gated ABI / syntax boundaries
```
