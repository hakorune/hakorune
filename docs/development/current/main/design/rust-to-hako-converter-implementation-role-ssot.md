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

## Machine-Checkable Inventory Snapshot

The current freeze inventory is stored as a structured JSON block in this
document so the guard can compare it against the active converter surface
without relying on handwritten prose.

```json
{
  "schema_version": 0,
  "kind": "PythonConverterRoleInventoryV1",
  "checkpoint": "PYTHON-SEMANTIC-PROJECTOR-GROWTH-FREEZE-001",
  "scope": "tools/rust_lifecycle",
  "role_buckets": [
    {
      "role": "FactsAdapter",
      "patterns": [
        "tools/rust_lifecycle/context_fact_extraction.py",
        "tools/rust_lifecycle/extract_*.py",
        "tools/rust_lifecycle/mirbuilder_allocation_policy_facts.py"
      ]
    },
    {
      "role": "SemanticProjector",
      "allowed_role": "bootstrap/oracle",
      "patterns": [
        "tools/rust_lifecycle/mirbuilder_*_artifacts.py",
        "tools/rust_lifecycle/mirbuilder_*_converter.py",
        "tools/rust_lifecycle/mirbuilder_*_selection.py",
        "tools/rust_lifecycle/mirbuilder_*_composition.py",
        "tools/rust_lifecycle/mirbuilder_*_lowering.py",
        "tools/rust_lifecycle/mirbuilder_*_publication.py",
        "tools/rust_lifecycle/mirbuilder_*_pipeline.py",
        "tools/rust_lifecycle/mirbuilder_*_verification.py",
        "tools/rust_lifecycle/mirbuilder_*_take.py",
        "tools/rust_lifecycle/mirbuilder_*_refresh.py",
        "tools/rust_lifecycle/mirbuilder_*_classifier.py",
        "tools/rust_lifecycle/mirbuilder_borrow_use_classifier.py",
        "tools/rust_lifecycle/mirbuilder_function_local_value_id_allocator.py",
        "tools/rust_lifecycle/mirbuilder_reserved_value_exclusion_policy.py",
        "tools/rust_lifecycle/mirbuilder_next_value_id_composition.py",
        "tools/rust_lifecycle/mirbuilder_allocation_policy_mainline_selection.py",
        "tools/rust_lifecycle/mirbuilder_minimal_execution_path_selection.py",
        "tools/rust_lifecycle/mirbuilder_minimal_execution_path_semantic_closure_report.py",
        "tools/rust_lifecycle/mirbuilder_condition_fn_injection.py",
        "tools/rust_lifecycle/mirbuilder_direct_shape_lowerer.py",
        "tools/rust_lifecycle/mirbuilder_mir_module_minimal_shell_transport.py",
        "tools/rust_lifecycle/mirbuilder_region_observer_variable_map_route.py",
        "tools/rust_lifecycle/mirbuilder_type_hint_provision.py",
        "tools/rust_lifecycle/mirbuilder_function_region_stack_pop.py",
        "tools/rust_lifecycle/mirbuilder_slot_registry_release.py",
        "tools/rust_lifecycle/mirbuilder_module_metadata_publication.py",
        "tools/rust_lifecycle/mirbuilder_record_packed_layout_refresh.py",
        "tools/rust_lifecycle/mirbuilder_record_packed_layout_refresh_artifacts.py",
        "tools/rust_lifecycle/mirbuilder_typed_object_plan_refresh.py",
        "tools/rust_lifecycle/mirbuilder_direct_state_plan_refresh.py",
        "tools/rust_lifecycle/mirbuilder_all_functions_phi_materialization.py",
        "tools/rust_lifecycle/mir_module_minimal_shell_artifacts.py",
        "tools/rust_lifecycle/mir_function_constructor_artifacts.py",
        "tools/rust_lifecycle/mirbuilder_prepared_state_install_artifacts.py",
        "tools/rust_lifecycle/mirbuilder_next_value_id_prepared_state_kernel_artifacts.py",
        "tools/rust_lifecycle/mirbuilder_ordered_map_crate_bundle_artifacts.py"
      ]
    },
    {
      "role": "DeterministicEmitter",
      "patterns": [
        "tools/rust_lifecycle/generate_mirbuilder_ordered_map_crate_bundle.py",
        "tools/rust_lifecycle/mirbuilder_family_artifacts.py",
        "tools/rust_lifecycle/mirbuilder_ordered_map_crate_bundle_artifacts.py"
      ]
    },
    {
      "role": "GuardOrchestrator",
      "patterns": [
        "tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py",
        "tools/rust_lifecycle/verify_*.py",
        "tools/rust_lifecycle/*_inventory.py",
        "tools/rust_lifecycle/*_readiness_inventory.py",
        "tools/rust_lifecycle/mirbuilder_box_compilation_context_crate_smoke_*.py",
        "tools/rust_lifecycle/*_runner.py",
        "tools/rust_lifecycle/mirbuilder_family_validators.py",
        "tools/rust_lifecycle/mirbuilder_generated_to_native_adoption_matrix.py"
      ]
    }
  ],
  "semantic_projector_follow_on_tokens": {
    "mirbuilder-minimal-execution-path-selection": "MIRBUILDER-MINIMAL-EXECUTION-PATH-SEMANTIC-CLOSURE-REPORT-001",
    "mirbuilder-minimal-execution-path-semantic-closure-report": "MIR-MODULE-MINIMAL-SHELL-DERIVED-HAKO-ARTIFACT-001",
    "mirbuilder-next-value-id-prepared-state-kernel": "MIRBUILDER-ALLOCATION-POLICY-HAKO-ADOPTION-DECISION-001",
    "mirbuilder-return-emission": "MIRBUILDER-RETURN-EMISSION-HAKO-SHADOW-PROJECTOR-001",
    "mirbuilder-record-packed-layout-refresh": "MIRBUILDER-RECORD-PACKED-LAYOUT-REFRESH-DERIVED-HAKO-ARTIFACT-001",
    "mirbuilder-condition-fn-injection": "MIRBUILDER-FUNCTION-REGION-STACK-POP-DERIVED-HAKO-ARTIFACT-001"
  },
  "non_claims": {
    "new_python_semantic_projector_growth": 0,
    "hako_adopted_python_write": 0,
    "normal_selfhost_build_python_dependency": 0,
    "runtime_fallback": 0
  }
}
```

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

Boundary rule:

```text
compiler meaning / JSON generation / text formatting:
  ordinary .hako module or library

OS, filesystem, environment, process, hash:
  Hako facade -> host C ABI

values crossing Hako <-> host/plugin:
  TypeBox ABI v2 + value-repr manifest
```

TypeBox ABI is not a convenience extension point for compiler libraries. It is
the representation and ownership contract for values crossing a host/plugin
boundary.

Priority:

```text
1. TextBuilderBox / StringBufferBox
2. CanonicalJsonValue / CanonicalJsonWriter
3. DiagnosticBuilder
4. typed Array / OrderedMap helpers
5. narrow File / Path / Env / SHA256 host APIs
6. language syntax only after repeated library insufficiency
```

Do not grow Python because `.hako` lacks convenience. Either add a small Hako
library/host boundary or keep Python as an oracle while the Hako projector
surface matures.

Initial compiler-library stack:

```text
ReturnEmissionHakoProjector / FunctionRegionStackPopHakoProjector
  -> CompilerProjectionValueBox
  -> CanonicalJsonWriterBox
  -> TextBuilderBox
  -> StringBox / ArrayBox / OrderedMapBox
```

Placement:

```text
compiler-only helper library:
  lang/src/compiler/lib/**

shared reusable JSON/text library:
  lang/src/shared/json/**
  lang/src/shared/common/**

host-backed facade:
  lang/src/runtime/host/**

low-level substrate implementation:
  lang/src/runtime/substrate/**

native compiler family authority:
  lang/src/mir/builder/**
  lang/src/compiler/**
```

First placement for the projector support libraries:

```text
lang/src/compiler/lib/text_builder.hako
lang/src/compiler/lib/projection_value.hako
lang/src/compiler/lib/canonical_json.hako
lang/src/compiler/lib/return_emission_projector.hako
lang/src/compiler/lib/function_region_stack_pop_projector.hako
lang/src/compiler/lib/slot_registry_release_projector.hako
lang/src/compiler/lib/typed_object_plan_refresh_projector.hako
```

Promotion rule:

```text
lang/src/compiler/lib/**
  -> compiler-only, free to evolve with compiler projector needs

lang/src/shared/json/**
  -> promote only after multiple non-projector compiler users need it

TypeBox ABI / value-repr manifest
  -> add only when values cross Hako <-> host/plugin boundary
```

Do not create a distribution ABI for these helpers yet. Distribution can start
as ordinary checked-in `.hako` modules. ABI packaging is a later boundary
decision, not a prerequisite for compiler-library usefulness.

`CompilerProjectionValueBox` is internal to compiler tooling. It should model:

```text
Null
Bool
I64
String
Array
Object
```

It is not public TypeBox ABI and does not need a host boundary.

`CanonicalJsonWriterBox` owns:

```text
string escaping
i64 / bool / null serialization
array serialization
object key ordering
stable whitespace policy
```

`TextBuilderBox` starts as ordinary Hako data:

```text
chunks: ArrayBox

append(text)
append_i64(value)
append_json_string(value)
finish()
```

Do not introduce `hako.buf`, Core C ABI, or TypeBox ABI for this v0. If
profiling later proves TextBuilder hot, replace its internal implementation
with `hako.buf`-backed storage behind the same Hako library API.

Host facades are allowed only for substrate-bound operations:

```text
FileFacade:
  read / write

PathFacade:
  directory walk / path normalization

EnvFacade:
  environment lookup

ProcessFacade:
  explicit process execution

HashFacade:
  SHA-256
```

Host facades must not decide:

```text
which family is selected
which transport is adopted
what is directable
how VerifiedHakoFamilyIR is built
```

Language syntax/spec additions are last resort. A helper graduates from
library to language only when multiple compiler families require it, the
library cannot preserve type safety or optimization semantics, and VM/AOT
behavior must be fixed as language meaning.

## Open Design Boundaries

No additional design consultation is required before the current freeze
checkpoint or before a docs-only placement card for the compiler library.

New consultation is required before any of these changes:

```text
TypeBox ABI exposure for compiler libraries
host ABI facade for JSON/Text/projector semantics
promotion from lang/src/compiler/lib to lang/src/shared/**
promotion from library helper to language syntax/spec
distribution/package ABI for compiler libraries
hako.buf-backed TextBuilder implementation
```

The next executable work should stay docs/inventory first:

```text
1. PYTHON-SEMANTIC-PROJECTOR-GROWTH-FREEZE-001
2. HAKO-COMPILER-TEXT-BUILDER-V0-001
3. HAKO-COMPILER-CANONICAL-JSON-VALUE-WRITER-001
4. MIRBUILDER-RETURN-EMISSION-HAKO-SHADOW-PROJECTOR-001
```

## Non-Claims

```text
delete_existing_python_converter = 0
rewrite_all_generators_at_once = 0
HakoAdopted_for_all_families = 0
source_selfhost_claim = 0
normal_build_runs_python_projector = 0
runtime_fallback = 0
```
