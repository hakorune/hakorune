# Derived-to-Native Hako Artifact Model

Status: SSOT
Date: 2026-06-20
Scope: How Rust compiler-family sources migrate through generated Hako
artifacts before becoming native Hako source.

## Decision

Adopt a two-stage migration model:

```text
Rust source
  -> derived Hako artifact
  -> shadow / selected mainline execution
  -> Hako-native adoption when the family is mature
```

The generated Hako artifact is a selected execution artifact. It is not the
final semantic/edit authority.

Final source selfhost still requires native Hako source for compiler semantic
families. Rust remains retained for bootstrap, platform bring-up, oracle
vectors, and explicit compatibility routes.

## Selfhost Levels

Use these terms precisely:

```text
Artifact Selfhost:
  checked-in generated Hako artifacts can run the compiler path without
  invoking the Rust semantic adapter during normal build.

Mainline Selfhost:
  the active selfhost execution route uses Hako artifacts for selected
  compiler families.

Source Selfhost:
  compiler meaning is edited and developed as native Hako source.
```

The derived model helps reach Artifact Selfhost and Mainline Selfhost. It does
not by itself prove Source Selfhost.

## Family State Machine

```text
RustPrimary
  ↓
DerivedShadow
  ↓
DerivedMainline
  ↓
HakoAdopted
  ↓
RustCompatFrozen
```

Meaning:

```text
RustPrimary:
  Rust implementation is the selected implementation.

DerivedShadow:
  generated Hako exists and is parity-checked only.

DerivedMainline:
  generated Hako is selected on the selfhost mainline. Rust remains bootstrap,
  oracle, and explicit compat route.

HakoAdopted:
  generated Hako has been adopted as native Hako source. The generator no
  longer overwrites it.

RustCompatFrozen:
  Rust implementation is retained only for bootstrap/platform/oracle/compat.
```

Permanent derived artifacts are acceptable for mechanical declarations,
serialization boilerplate, ABI bindings, and generated constants. Compiler
semantic families such as parser policy, resolution, lifecycle policy,
FlowPlanner, recipe/verifier, loop/PHI policy, and canonical lowering require a
Hako-native adoption decision before Source Selfhost is claimed.

## Frontier and Directability

The semantic frontier and the materialization decision are separate. The
frontier chooses the next source edge; directability chooses whether that edge
is a leaf that can be materialized now or a composite owner that must be
decomposed first.

Use these outcomes explicitly:

```text
AllowLeafArtifact:
  the analyzer points at a leaf owner and the child authority is explicit

DenyCompositeNeedsDecomposition:
  the analyzer still points at a composite owner

DenyMissingChildAuthority:
  a child owner exists, but no directability evidence exists yet
```

Do not hand-pin the next edge in task-order when the analyzer can derive it.
Do not turn a composite owner into one large derived artifact just because the
frontier reports it next. The slice shape comes from directability, not from
the frontier token alone.

Example:

```text
finalize_module.record_packed_layout_refresh:
  composite owner -> decompose first

finalize_module.typed_object_plan_refresh:
  first leaf owner under that composite -> may be materialized
```

Hako adoption is a separate axis from materialization. A family can be a
selected generated artifact, a derived mainline artifact, or a Hako-adopted
native source owner. Those states do not change the frontier rule above.

## Authority Vocabulary

During Derived phases:

```text
Rust source:
  editable reference / Rust oracle

rustc semantic adapter:
  Rust facts producer

Hako resolver / plan SSOT:
  Hako representation, borrow, cleanup, and lifecycle policy owner

Behavior recipe:
  explicit Rust operation -> Hako operation mapping

Verifier / parity:
  acceptance authority

generated Hako:
  selected execution artifact

Rust binary:
  bootstrap / platform / compat authority
```

During HakoAdopted phase:

```text
native Hako source:
  edit authority and compiler semantic authority

Rust:
  retained bootstrap / platform / oracle / compat
```

Do not call generated Hako the semantic authority. It is a generated execution
artifact accepted by verifier and parity evidence.

## Behavioral Converter Pipeline

Skeleton generation is not enough for build-line substitution.

Minimum behavioral pipeline:

```text
Rust semantic facts
  -> HakoLifecyclePlan
  +  HakoBehaviorRecipe
  -> CombinedVerifier
  -> VerifiedHakoFamilyIR
  -> DeterministicEmitter
  -> generated Hako artifact
  -> oracle parity and compiler gates
```

Owner split:

```text
rustc adapter:
  HIR / THIR / MIR facts only

HakoLifecyclePlan:
  representation, borrow, move, Drop, cleanup projection

HakoBehaviorRecipe:
  method body behavior and explicit Rust API -> Hako operation mapping

CombinedVerifier:
  checks all selected THIR nodes, MIR side effects, calls, borrows, moves,
  drops, signatures, and selected operations are accounted for

VerifiedHakoFamilyIR:
  declarations, fields, signatures, structured statements, resolved Hako
  operations, lifecycle attachments, and provenance

DeterministicEmitter:
  formatting and stable spelling only
```

Emitter must not infer lifecycle policy or behavior from Rust names, Hako names,
or type spellings.

## Artifact Provenance

Generated artifacts must be paired with a manifest.

```text
lang/generated/rust_derived/<crate>/<family>.hako
lang/generated/rust_derived/<crate>/<family>.artifact.json
```

The manifest records:

```text
family id
family state
Rust source paths and hashes
rustc commit / adapter version / emitter version
HIR hash
THIR hash
MIR facts hash
HakoLifecyclePlan hash
HakoBehaviorRecipe hash
verifier result hash
oracle vector hash
generated Hako hash
canonical MIR hash when available
```

Normal stable/selfhost builds use checked-in generated Hako artifacts. They do
not invoke the pinned nightly rustc adapter.

Regeneration is explicit. CI or guards may regenerate and diff artifacts
against checked-in output.

## Crate and Family Roles

```text
crate:
  transport, inventory, coverage ledger

module:
  provenance and focused materialization

family:
  behavioral conversion, parity, route selection, adoption decision

artifact bundle:
  packages green family artifacts for a crate route
```

Allowed route labels:

```text
derived_hako
native_hako
rust_bootstrap
rust_compat
host_substrate
unsupported
```

Runtime try-Hako-then-Rust fallback is forbidden. The selected route is explicit.

## BindingContext Pilot Sequence

```text
1511:
  DERIVED-TO-NATIVE-HAKO-ARTIFACT-MODEL-SSOT-001
  Design pivot only. No projection/emitter/mainline switch.

1512:
  BINDING-CONTEXT-DERIVED-HAKO-ARTIFACT-PILOT-001
  Rust facts -> HakoLifecyclePlan + HakoBehaviorRecipe -> verifier ->
  generated Hako artifact + manifest -> Hako parse/MIR/EXE gate -> Rust oracle
  parity. mainline_selected=0.

1513:
  BINDING-CONTEXT-DERIVED-ARTIFACT-MAINLINE-SELECTION-001
  Select generated Hako on a focused selfhost route. Rust bootstrap/oracle route
  remains explicit. silent fallback=0.

1514:
  BINDING-CONTEXT-HAKO-ADOPTION-DECISION-001
  Decide permanent_derived or HakoAdopted after regeneration and parity
  experience.
```

## Stop Lines

```text
do not edit generated Hako by hand
do not call a generated artifact the semantic authority
do not claim Source Selfhost while Rust remains edit authority
do not delete Rust source
do not remove Rust bootstrap or platform bring-up routes
do not let normal product/selfhost build invoke the pinned-nightly adapter
do not let emitter infer behavior or lifecycle policy
do not hide Rust API -> Hako API mapping inside emitter code
do not emit executable-tier TODO/null/Unsupported placeholders
do not accept a partially covered selected method body
do not keep Rust and Hako editable for the same family
do not let regeneration overwrite HakoAdopted source
do not select generated Hako without a provenance manifest
do not accept stale generated artifacts after source/plan/tool changes
do not use raw Rust MIR equality as Rust/Hako behavior parity
do not allow runtime try-Hako-then-Rust fallback
do not classify compiler semantic families as permanently derived without an
explicit family decision
```

## Summary

Derived Hako artifacts are a migration accelerator and reproducible build-line
artifact. Hako-native adoption remains the final exit for compiler semantic
families. Rust remains bootstrap, platform, oracle, and explicit compatibility
support.
