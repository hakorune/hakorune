# MirBuilder Converter Coverage Hygiene Inventory

Status: draft
Date: 2026-06-22

## Scope

This note inventories the remaining MirBuilder lifecycle converter surfaces
that still carry raw Hako harness text or string-encoded execution payloads.
It is a design-consultation inventory only. It does not change runtime route
selection, nightly rustc adapter usage, or any generated artifact authority.

## What Is Already Clean

- `tools/rust_lifecycle/family_artifact_builders.py` is a renderer only. It
  consumes `FamilyArtifactSpec` and does not itself embed family-specific
  harness text.
- `tools/rust_lifecycle/family_artifact_spec.py` is the data model only. The
  `main_lines` field is the remaining carrier, not a source of string literals
  by itself. `BehaviorMethodSpec` still carries stringly typed
  `rust_operation`, `hako_operation`, and `emits` fields, so it remains part
  of the debt surface.
- `tools/rust_lifecycle/shared_family_generator.py` stays in the shared
  generator layer and does not add new raw Hako behavior.
- `tools/rust_lifecycle/mirbuilder_ordered_map_converter.py` is typed IR only.
- `tools/rust_lifecycle/mirbuilder_negative_converter_fixtures.py` is already
  fixture-first and is not the current raw-Hako problem.

## Remaining Raw-String Surfaces

### 1. `tools/rust_lifecycle/mirbuilder_family_artifacts.py`

Current raw harness locations:

- `binding_context_spec()` has one `main_lines` block.
- `variable_context_simple_map_spec()` has one `main_lines` block.
- `box_compilation_context_spec()` has one `main_lines` block.
- `variable_context_immutable_borrow_spec()` has one `main_lines` block.
- `variable_context_snapshot_restore_spec()` has one `main_lines` block.

String-encoded payloads still present:

- `variable_context_immutable_borrow_spec()` still uses `ReturnSource` in a
  static box method payload.

Why this is debt:

- The harness text is still authored as raw Hako source lines.
- The immutable-borrow slice still carries a raw alias contract rather than a
  typed owned-snapshot contract.
- The family artifact layer still has to understand harness text shape instead
  of just typed execution intent.
- `BehaviorMethodSpec` still allows literal string mismatches between the Rust
  op, the Hako op, and the emission text.

Suggested next task labels:

- `Convert BindingContext and VariableContext simple-map harnesses to typed execution harness IR`
- `Convert BoxCompilationContext harness to typed execution harness IR`
- `Convert VariableContext snapshot/restore harness to typed execution harness IR`
- `Replace VariableContext immutable borrow ReturnSource with owned snapshot contract`

### 2. `tools/rust_lifecycle/mirbuilder_carrier_snapshot_artifacts.py`

Current raw harness locations:

- `carrier_snapshot_spec()` has one `main_lines` block.
- `explicit_carrier_snapshot_spec()` has one `main_lines` block.

String-encoded payloads still present:

- both specs still build their acceptance harness as raw Hako text.
- both specs still carry `CloneOwnedMap` and carrier-transfer intent inside the
  family artifact object rather than a dedicated harness IR.

Why this is debt:

- The carrier snapshot path is still validated by hand-authored harness text.
- The harness text couples execution shape and acceptance behavior in one raw
  blob.
- The remaining carrier contract work is clearer when the acceptance harness is
  typed or isolated behind a shared builder.

Suggested next task labels:

- `Convert CarrierInfo snapshot harnesses to typed execution harness IR`
- `Convert explicit CarrierInfo snapshot harness to typed execution harness IR`

## Current Remaining Slice Count

The remaining raw-string debt is still roughly five slices:

1. BindingContext and VariableContext simple-map harness family
2. BoxCompilationContext harness
3. VariableContext snapshot/restore harness
4. CarrierInfo snapshot harnesses
5. VariableContext immutable-borrow ReturnSource contract decision

That count matches the existing task-order estimate that still leaves about
five converter-coverage-hygiene tasks after the representative consultation
bundle work.

## Consultation Coverage Map

The five slices above are now each anchored by a consultation-only card:

- BindingContext and VariableContext simple-map harness family
  - [296x-1583](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-296x/296x-1583-MIRBUILDER-TYPED-HARNESS-REWRITE-INITIAL-PATCH-SEQUENCE-BINDING-CONTEXT-VARIABLE-CONTEXT-SIMPLE-MAP-001.md)
- shared ordered-map family consultation closeout
  - [296x-1613](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-296x/296x-1613-MIRBUILDER-TYPED-HARNESS-REWRITE-SHARED-ORDERED-MAP-FAMILY-CONSULTATION-CLOSEOUT-CONTRACT-001.md)
- VariableContext snapshot/restore harness
  - [296x-1600](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-296x/296x-1600-MIRBUILDER-CONVERTER-COVERAGE-HYGIENE-VARIABLE-CONTEXT-SNAPSHOT-RESTORE-IMPLEMENTATION-TOUCH-SET-CONTRACT-001.md)
  - [296x-1610](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-296x/296x-1610-MIRBUILDER-CONVERTER-COVERAGE-HYGIENE-VARIABLE-CONTEXT-SNAPSHOT-RESTORE-HARNESS-CONTRACT-001.md)
- CarrierInfo snapshot harnesses
  - [296x-1601](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-296x/296x-1601-MIRBUILDER-CONVERTER-COVERAGE-HYGIENE-CARRIER-INFO-SNAPSHOT-HARNESS-IMPLEMENTATION-TOUCH-SET-CONTRACT-001.md)
- CarrierInfo snapshot consultation closeout
  - [296x-1615](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-296x/296x-1615-MIRBUILDER-CONVERTER-COVERAGE-HYGIENE-CARRIER-INFO-SNAPSHOT-CONSULTATION-CLOSEOUT-CONTRACT-001.md)
- VariableContext immutable-borrow ReturnSource contract decision
  - [296x-1602](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-296x/296x-1602-MIRBUILDER-CONVERTER-COVERAGE-HYGIENE-VARIABLE-CONTEXT-IMMUTABLE-BORROW-RETURNSOURCE-CONTRACT-DECISION-001.md)
- VariableContext immutable-borrow consultation closeout
  - [296x-1614](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-296x/296x-1614-MIRBUILDER-CONVERTER-COVERAGE-HYGIENE-VARIABLE-CONTEXT-IMMUTABLE-BORROW-CONSULTATION-CLOSEOUT-CONTRACT-001.md)
- BoxCompilationContext harness
  - [296x-1603](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-296x/296x-1603-MIRBUILDER-CONVERTER-COVERAGE-HYGIENE-BOX-COMPILATION-CONTEXT-HARNESS-CONTRACT-001.md)
- First representative easy-tier crate-level probe contract
  - [296x-1604](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-296x/296x-1604-MIRBUILDER-CONVERTER-COVERAGE-HYGIENE-BOX-COMPILATION-CONTEXT-FIRST-REPRESENTATIVE-CRATE-LEVEL-PROBE-CONTRACT-001.md)

## BoxCompilationContext Consultation Chain

The BoxCompilationContext path now has a consultation-only chain that stays
before implementation:

- harness rewrite contract
  - [296x-1603](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-296x/296x-1603-MIRBUILDER-CONVERTER-COVERAGE-HYGIENE-BOX-COMPILATION-CONTEXT-HARNESS-CONTRACT-001.md)
- first representative crate-level probe contract
  - [296x-1604](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-296x/296x-1604-MIRBUILDER-CONVERTER-COVERAGE-HYGIENE-BOX-COMPILATION-CONTEXT-FIRST-REPRESENTATIVE-CRATE-LEVEL-PROBE-CONTRACT-001.md)
- typed harness payload schema contract
  - [296x-1605](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-296x/296x-1605-MIRBUILDER-CONVERTER-COVERAGE-HYGIENE-BOX-COMPILATION-CONTEXT-TYPED-HARNESS-PAYLOAD-SCHEMA-CONTRACT-001.md)
- builder rendering contract
  - [296x-1606](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-296x/296x-1606-MIRBUILDER-CONVERTER-COVERAGE-HYGIENE-BOX-COMPILATION-CONTEXT-TYPED-HARNESS-BUILDER-RENDERING-CONTRACT-001.md)
- emitter consumption contract
  - [296x-1607](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-296x/296x-1607-MIRBUILDER-CONVERTER-COVERAGE-HYGIENE-BOX-COMPILATION-CONTEXT-TYPED-HARNESS-EMITTER-CONSUMPTION-CONTRACT-001.md)
- family artifact host contract
  - [296x-1608](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-296x/296x-1608-MIRBUILDER-CONVERTER-COVERAGE-HYGIENE-BOX-COMPILATION-CONTEXT-TYPED-HARNESS-FAMILY-ARTIFACT-HOST-CONTRACT-001.md)
- artifact manifest contract
  - [296x-1609](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-296x/296x-1609-MIRBUILDER-CONVERTER-COVERAGE-HYGIENE-BOX-COMPILATION-CONTEXT-TYPED-HARNESS-ARTIFACT-MANIFEST-CONTRACT-001.md)
- consultation closeout contract
  - [296x-1611](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-296x/296x-1611-MIRBUILDER-CONVERTER-COVERAGE-HYGIENE-BOX-COMPILATION-CONTEXT-TYPED-HARNESS-CONSULTATION-CLOSEOUT-CONTRACT-001.md)

## VariableContext Consultation Chain

The VariableContext path now has a consultation-only chain that stays before
implementation:

- snapshot/restore implementation touch set contract
  - [296x-1600](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-296x/296x-1600-MIRBUILDER-CONVERTER-COVERAGE-HYGIENE-VARIABLE-CONTEXT-SNAPSHOT-RESTORE-IMPLEMENTATION-TOUCH-SET-CONTRACT-001.md)
- typed harness contract
  - [296x-1610](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-296x/296x-1610-MIRBUILDER-CONVERTER-COVERAGE-HYGIENE-VARIABLE-CONTEXT-SNAPSHOT-RESTORE-HARNESS-CONTRACT-001.md)
- consultation closeout contract
  - [296x-1612](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-296x/296x-1612-MIRBUILDER-CONVERTER-COVERAGE-HYGIENE-VARIABLE-CONTEXT-SNAPSHOT-RESTORE-CONSULTATION-CLOSEOUT-CONTRACT-001.md)
- immutable-borrow ReturnSource contract decision
  - [296x-1602](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-296x/296x-1602-MIRBUILDER-CONVERTER-COVERAGE-HYGIENE-VARIABLE-CONTEXT-IMMUTABLE-BORROW-RETURNSOURCE-CONTRACT-DECISION-001.md)
- CarrierInfo snapshot harness implementation touch set contract
  - [296x-1601](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-296x/296x-1601-MIRBUILDER-CONVERTER-COVERAGE-HYGIENE-CARRIER-INFO-SNAPSHOT-HARNESS-IMPLEMENTATION-TOUCH-SET-CONTRACT-001.md)

## Design Stops

Do not widen this inventory into implementation decisions yet:

- no nightly rustc adapter opening
- no route selection changes
- no runtime fallback
- no new family selection
- no typed harness rewrite without a dedicated slice

The first typed harness rewrite slice is now the BindingContext and
VariableContext simple-map harness family. The rewrite contract, emission
contract, implementation boundary, and implementation entry contract are all
landed. The next useful action is to turn that into an initial patch sequence
in a separate task, but that is outside this inventory note.
