---
Status: SSOT
Decision: current
Date: 2026-04-20
Scope: generic value-corridor optimization contract shared by string, bytes, scalar, array-lane, and map-lane planning.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/semantic-optimization-authority-ssot.md
  - docs/development/current/main/design/optimization-task-card-os-ssot.md
  - docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md
  - docs/development/current/main/design/string-semantic-value-and-publication-boundary-ssot.md
  - docs/development/current/main/design/lifecycle-typed-value-language-ssot.md
  - docs/development/current/main/phases/phase-137x/README.md
  - docs/development/current/main/phases/phase-289x/README.md
---

# Value Corridor Generic Optimization Contract

## Purpose

This document fixes the generic contract vocabulary for future optimization
cards. It does not open new implementation work.

Use it when a string-lane lesson needs to be described in terms that can later
apply to bytes, scalar immediates, array residence, and map residence without
turning helper names into architecture.

## Authority Rule

```text
Semantic owner = .hako
Optimization contract owner = MIR
Execution owner = Rust/runtime
Emit/codegen owner = backend/.inc/LLVM
```

Layer responsibilities:

- `.hako` owns language meaning and semantic demand.
- MIR owns the generic optimization contract and the verifier-visible proof.
- Rust/runtime executes already-approved mechanics only.
- `.inc` / backend / LLVM emit or optimize the approved shape only.

Hard rule:

- runtime must not decide legality
- runtime must not infer provenance
- backend must not rediscover routes
- helper names must not become MIR truth

## Semantic Demand

`.hako` may describe only semantic demand. It must stay above route names,
runtime helper names, and backend shapes.

Canonical demand axes:

- `source_preserve`
- `identity_demand`
- `publication_demand`
- `consumer_demand`

These axes answer "what is required", not "which helper should run".

## MIR Generic Contract

MIR is the only layer allowed to turn semantic demand into optimization facts.
Every value-corridor optimization card must be expressible through these terms.

### ValueFamily

- `Text`
- `Bytes`
- `Scalar`
- `ArrayLane`
- `MapLane`

`ArrayLane` and `MapLane` mean internal residence only. They do not redefine
public Array / Map identity.

### Carrier

- `Ref`
- `Plan`
- `Owned`
- `Cell`
- `StableObj`

Carrier meanings:

- `Ref`: borrowed/read-only value view inside a proven region
- `Plan`: delayed computation or placement plan
- `Owned`: unpublished owned payload
- `Cell`: lane/container residence
- `StableObj`: object-world representation after publication or promotion

### Effect

- `borrow.from_obj`
- `freeze.birth`
- `load.ref`
- `store.cell`
- `publish(reason, repr)`

Effects are contract verbs. Runtime may implement them, but it does not choose
whether they are legal.

### Metadata

- `provenance`
- `proof_region`
- `publication_boundary`
- `materialization_policy`
- `consumer_capability`
- `lane_eligibility`

Required interpretation:

- `proof_region` is where the fact is proven to hold.
- `publication_boundary` is where an executor may publish without widening.
- `consumer_capability` describes what the next consumer can accept.
- `lane_eligibility` describes internal residence permission, not public ABI.

## Consumer Capability

Use capability terms instead of string-only consumer names.

Initial vocabulary:

- `ReadOnly`
- `CompareOnly`
- `LengthOnly`
- `SinkStore`
- `NeedsStableObject`
- `NeedsHandle`

This is a capability set, not a total order. A future verifier may define a
stricter lattice, but optimization cards must already name the capability they
depend on.

Examples:

- `LengthOnly` may consume a `Ref` or `Cell` without object publication.
- `SinkStore` may consume an `Owned` or `Cell` if lane eligibility is proven.
- `NeedsStableObject` requires publication or objectization at the MIR-owned
  boundary.
- `NeedsHandle` requires object/handle world entry and must not be hidden in a
  helper-local fallback.

## Runtime Boundary

Rust/runtime owns physical mechanics only:

- residence mutation
- materialize
- objectize
- fresh handle issue
- cache
- store/load substrate
- runtime-private helper execution

Runtime may carry mirror enums or action plans when those are direct
translations of MIR-approved facts. It may not create a new proof, route, or
publication decision.

## Backend Boundary

`.inc` and backend code may keep:

- operand normalization
- emit variant selection
- ABI transport
- already-approved direct call emission

They must not keep:

- legality rediscovery
- provenance inference
- route planning
- helper-name based semantic classification

## Deferred Work

This SSOT does not open:

- `publish.any`
- typed map lane implementation
- heterogeneous / union array slot layout
- public ABI widening
- runtime-wide allocator / arena work

Those require separate phase gates with exact/middle/whole proof, rollback
notes, and verifier-visible MIR contracts.

## Use In 137x-H

The next owner-first optimization card must name:

- semantic demand axis from `.hako`
- MIR `ValueFamily`
- MIR `Carrier`
- MIR `Effect`
- MIR metadata fields used
- runtime executor touched
- backend emit surface touched
- rejected layer crossings

If a proposed cut cannot fill those fields, it is not ready for code.
