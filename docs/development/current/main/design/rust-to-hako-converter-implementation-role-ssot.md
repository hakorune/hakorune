---
Status: SSOT
Date: 2026-06-26
Scope: Rust-to-Hako converter implementation roles and Python growth control.
Related:
  - docs/development/current/main/design/derived-to-native-hako-artifact-model-ssot.md
  - docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md
---

# Rust-to-Hako Converter Implementation Role SSOT

This document separates two migration axes that must not be conflated.

```text
Family implementation:
  RustPrimary
    -> DerivedShadow
    -> DerivedMainline
    -> HakoAdopted
    -> RustCompatFrozen

Converter implementation:
  PythonBootstrap
    -> HakoShadow
    -> HakoMainline
    -> PythonFrozen / Retired
```

`HakoAdopted` makes native `.hako` source the edit/semantic authority for one
family. A converter implementation migration makes the tool that projects
facts/plans into typed artifact IR run in `.hako`. These are related but not
the same task.

## Role Classification

Python tooling is classified by role.

```text
FactsAdapter:
  Reads Rust/rustc/source/fixtures and projects source facts.
  Python is allowed as bootstrap/tooling.

SemanticProjector:
  Owns directability decisions, semantic transport selection, plan resolution,
  behavior recipe construction, or VerifiedHakoFamilyIR construction.
  Existing Python is allowed only as bootstrap/oracle until retired.
  New Python growth is forbidden without an explicit exception card.

DeterministicEmitter:
  Renders already-verified typed IR to deterministic text/manifest/fixtures.
  Python remains allowed initially.

GuardOrchestrator:
  Runs checks, diffs fixtures, reports hashes, or drives CI.
  Python/Shell remains allowed.
```

## Growth Freeze Rule

After the current `ConditionFnInjection` executable artifact slice lands, the
MirBuilder lane must insert a role-control checkpoint before continuing wider
derived artifact growth.

```text
checkpoint:
  PYTHON-SEMANTIC-PROJECTOR-GROWTH-FREEZE-001

rules:
  existing Python SemanticProjector = bootstrap/oracle only
  new Python SemanticProjector = forbidden by default
  HakoAdopted artifact write by Python = forbidden
  normal selfhost build dependency on Python SemanticProjector = forbidden
  CI / fixture / diff / oracle use of Python = allowed
```

This does not delete the existing Python converter. Existing Python remains a
valuable reference implementation, oracle, fixture updater, and bootstrap tool.
It only stops new compiler meaning from growing in Python by default.

## Language Boundary Contract

During migration, one family pipeline may temporarily touch both Python and
`.hako`. That is allowed only when the language seam is explicit and
file-backed.

```text
allowed seam:
  canonical JSON file

forbidden seam:
  in-memory Python calls into Hako projector internals
  Hako projector importing Python semantic helpers
  shared mutable state across Python/Hako stages
  backend/runtime fallback between Python and Hako semantics
```

The intended shadow shape is:

```text
source facts / plan JSON
  -> Python SemanticProjector as oracle
  -> Hako SemanticProjector as shadow candidate
  -> canonical JSON diff
  -> one selected authority after parity
```

The deterministic emitter may remain Python while the semantic projector is
being migrated. The emitter may consume typed projection JSON; it must not
re-decide directability, transport, lifecycle, or behavior recipe semantics.

## Temporary Chimera Budget

Mixed Python/Hako operation is a migration mechanism, not a steady state.

Rules:

```text
shadow_window_per_family:
  short; parity -> promotion/retire decision must be named

concurrent_hako_shadow_projectors:
  bounded; do not open another Hako shadow projector when an existing shadow
  has no retire token or parity gate

authority_during_shadow:
  Python = oracle/bootstrap
  Hako   = shadow candidate

authority_after_promotion:
  Hako   = mainline SemanticProjector for that family/stage
  Python = frozen oracle or retired

permanent dual authority:
  forbidden
```

Every Hako shadow projector must record:

```text
family_id
stage_id
input_json
output_json
python_oracle
hako_shadow
parity_gate
promotion_token
retirement_token
```

If a family cannot name a retirement token, it may not enter HakoShadow.

## Authority Handoff States

Converter implementation migration uses this stage vocabulary.

```text
PythonBootstrap:
  Python owns the implementation for a stage.

HakoShadow:
  Hako implementation exists, but Python remains oracle/bootstrap.
  Canonical output diff is required.

HakoMainline:
  Hako implementation is selected for that stage before execution.
  Python remains explicit oracle/bootstrap only.

PythonFrozen:
  Python implementation is kept only for compatibility/oracle.
  No semantic growth is allowed.

Retired:
  Python implementation for that family/stage is removed or unreachable from
  active regeneration.
```

Promotion is stage-scoped. A family may have:

```text
SemanticProjector = HakoMainline
DeterministicEmitter = PythonBootstrap
GuardOrchestrator = PythonBootstrap
```

This is not a violation as long as each stage has exactly one selected
authority and the boundaries are JSON/file based.

## Freeze Checkpoint Acceptance

`PYTHON-SEMANTIC-PROJECTOR-GROWTH-FREEZE-001` is green only when the repository
has a machine-checkable role inventory for active Rust-to-Hako converter tools.

Minimum acceptance:

```text
all active Python converter tools are classified:
  FactsAdapter
  SemanticProjector
  DeterministicEmitter
  GuardOrchestrator

existing Python SemanticProjector entries include:
  family/stage id
  allowed role = bootstrap/oracle
  retirement or HakoShadow follow-on token

new Python SemanticProjector growth:
  forbidden by default

HakoAdopted artifact Python write:
  forbidden

normal selfhost build Python SemanticProjector dependency:
  forbidden

CI / fixture / diff / oracle Python use:
  allowed
```

The freeze checkpoint must not delete existing Python converter code. Deletion,
retirement, or HakoMainline promotion is handled by later family-scoped cards.

## First Adoption Candidate

The first `HakoAdopted` decision should use an already-mainline derived family,
not a merely PlanOnly edge.

```text
candidate:
  MIRBUILDER-ALLOCATION-POLICY-HAKO-ADOPTION-DECISION-001

reason:
  prepared-state allocation-policy kernel is already DerivedMainline
  selfhost_mainline = derived_hako
  rust_bootstrap = retained
  fallback = Forbidden
  selected route closure = closed
```

`ReturnEmission` is not the first adoption candidate because it is not
DerivedMainline. It is a good first Hako converter-shadow candidate.

## First Hako Projector Candidate

```text
candidate:
  MIRBUILDER-RETURN-EMISSION-HAKO-SHADOW-PROJECTOR-001

input:
  mirbuilder-return-emission-plan-v0.json

output:
  typed projection / VerifiedHakoFamilyIR-compatible JSON

requirements:
  no Rust source rescan
  Python projector remains oracle
  Hako projector output is canonical-serialization equivalent
  no backend route
  no ABI
  no runtime fallback
```

The deterministic emitter may remain Python for the first pilot. The point of
the pilot is to move semantic projection logic first, not to rewrite every
tooling layer at once.

## Library-First Hako Support

When `.hako` projector work exposes missing ergonomics, add compiler-library
support before language syntax.

Priority:

```text
1. CanonicalJsonValue / CanonicalJsonWriter
2. TextBuilderBox / StringBufferBox
3. DiagnosticBuilder
4. typed Array / OrderedMap helpers
5. narrow File / Path / Env / SHA256 host APIs
6. language syntax only after repeated library insufficiency
```

Do not grow Python because `.hako` lacks convenience. Either add a small Hako
library/host boundary or keep Python as an oracle while the Hako projector
surface matures.

## Non-Claims

```text
delete_existing_python_converter = 0
rewrite_all_generators_at_once = 0
HakoAdopted_for_all_families = 0
source_selfhost_claim = 0
normal_build_runs_python_projector = 0
runtime_fallback = 0
```
