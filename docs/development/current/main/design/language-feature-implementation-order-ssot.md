---
Status: SSOT
Decision: accepted
Date: 2026-05-14
Scope: Durable implementation order for low-level Hakorune language features.
Related:
  - docs/development/current/main/design/language-minimal-surface-ssot.md
  - docs/development/current/main/design/language-minimal-surface-task-breakdown-ssot.md
  - docs/development/current/main/design/delegation-no-inheritance-ssot.md
  - docs/development/current/main/design/stage0-stage1-feature-responsibility-split-ssot.md
  - docs/reference/language/low-level-capabilities.md
  - docs/development/current/main/design/stage1-mir-authority-boundary-ssot.md
  - docs/development/current/main/design/type-system-policy-ssot.md
  - docs/development/current/main/design/record-and-packed-array-lowering-ssot.md
---

# Language Feature Implementation Order SSOT

## Purpose

This document preserves the full planned feature set and fixes the implementation order.
It prevents Stage0 from accumulating semantic ownership while still allowing thin bootstrap syntax and metadata capsules.

The minimal canonical surface is:

```text
docs/development/current/main/design/language-minimal-surface-ssot.md
```

The task-sized backlog is:

```text
docs/development/current/main/design/language-minimal-surface-task-breakdown-ssot.md
```

The companion ownership map is:

```text
docs/development/current/main/design/stage0-stage1-feature-responsibility-split-ssot.md
```

## Ordering rule

One row must add one durable semantic slice.
Do not mix a Stage0 capsule with a Stage1 semantic implementation in the same row unless the card explicitly declares a docs-only transition and has no behavior change.

Every row must name:

```text
Decision:
Owner:
Stage0 owns:
Stage0 does not own:
Stage1 owns:
Fixture / guard:
Unsupported backend behavior:
Retire condition:
```

`Retire condition` is required for every Stage0 capsule.

## Wave A: Stage0 thin syntax and metadata capsules

Wave A rows may not create new language meaning.
They exist to let source code carry future shapes while Stage1 becomes the semantic owner.

| Order | Row | Slice | Stage0 owns | Stage0 does not own | Retire condition |
| --- | --- | --- | --- | --- | --- |
| A1 | `L201 loop-only control surface SSOT` | docs-only decision: no `while`, no `for`, no `repeat`, no `until` | no new syntax | all loop semantics | n/a docs-only row |
| A2 | `L202 loop-range parser capsule` | `loop i in start..end { ... }` | parse and LoopRange metadata transport | semantic lowering, index mutability policy, continue stepping, bounds proof | Stage1 parser and loop-range transport own the same shape. |
| A3 | `L203 brand decl metadata capsule` | `brand PageId: i64` | parse brand and underlying type metadata | constructor, unwrap policy, mixed-brand rejection | Stage1 brand parser and semantic checker own the declaration. |
| A4 | `L204 type alias metadata capsule` | `type Bytes = usize` | parse alias and transport metadata | alias checker, type equality policy | Stage1 type metadata transport owns alias declarations. |
| A5 | `L205 record literal shape capsule` | `RecordName { field: value }` | parse explicit field-shape metadata | shorthand literal, construction/read lowering, field validation, layout | Stage1 record literal lowering owns construction semantics. |
| A6 | `L206 contract syntax metadata capsule` | `requires`, `ensures`, `invariant` | complete as `293x-282`; syntax parse and metadata capsule | verifier facts, runtime contract insertion, invariant checking | Stage1 contract lowering owns the metadata shape. |
| A7 | `L207 transition metadata capsule` | `transition Enum.Value -> Enum.Value by method` | complete as `293x-283`; transition declaration parse and enum-reference metadata | transition legality, lifecycle verifier, `state` keyword | Stage1 transition checker owns lifecycle facts. |
| A8 | `L208 uses metadata capsule` | method-level `uses osvm` / `uses atomic` / `uses rawbuf` | syntax parse and capability metadata | capability policy, backend gate, `cap` block syntax | Stage1 capability checker owns policy. |
| A9 | `L209 generic type annotation metadata capsule` | `Array<T>`, `PackedArray<T>`, capability-bearing type annotations | parse and metadata transport | typed array semantics, PackedArray planner, fallback policy | Stage1 typed container planner owns the facts. |
| A10 | `L210 module header metadata capsule` | deferred `module` / `use` / `export` / `private` minimum | bootstrap header and export/private metadata only if needed | visibility semantics, duplicate import policy, alias rebinding policy | Stage1 module resolver owns package and visibility semantics. |

## Wave B: Stage1 semantic nucleus

Wave B rows create the first useful semantic core for allocator safety and lifecycle verification.

| Order | Row | Slice | Owner |
| --- | --- | --- | --- |
| B1 | `S1-1 brand semantic checker` | reject mixed brands such as `PageId` where `BlockId` is required; define constructor and unwrap policy | Stage1 type/verifier lane |
| B2 | `S1-2 assert runtime-check insertion` | lower `assert cond : message` to fail-fast runtime check and expose verifier fact candidates | Stage1 contract lane |
| B3 | `S1-3 invariant runtime-check pilot` | attach box/record invariant metadata and insert runtime checks at defined boundaries | Stage1 contract/verifier lane |
| B4 | `S1-4 requires ensures contract lowering` | caller-side precondition and return-side postcondition lowering | Stage1 contract/verifier lane |
| B5 | `S1-5 enum transition semantic facts` | use enum values as state values and check declared transition legality | Stage1 lifecycle lane |
| B6 | `S1-6 page lifecycle verifier pilot` | apply state/contract facts to page lifecycle rows such as decommit, recommit, reactivate, reuse | Stage1 lifecycle lane |
| B7 | `S1-7 record literal construction read lowering` | validate fields and lower construction/read without ordinary box identity | Stage1 record lane |
| B8 | `S1-8 record with-update lowering` | lower `meta with { field: value }` as identity-free replacement | Stage1 record lane |
| B9 | `S1-9 Result Option prelude and diagnostics` | define standard `Result<T,E>` / `Option<T>`, match diagnostics, and exhaustiveness basis | Stage1 enum/prelude lane |
| B10 | `S1-10 guard-let sugar` | lower `guard let Pattern = expr else { ... }` through Stage1 pattern binding and match rules | Stage1 pattern lane |

## Wave C: Stage1 low-level and CorePlan wave

Wave C rows require verifier, CorePlan, or backend capability gates.
They must not be implemented in Stage0.

| Order | Row | Slice | Owner |
| --- | --- | --- | --- |
| C1 | `S1-11 PackedArray eligibility gate` | decide whether `PackedArray<T>` can reside packed and fail-fast otherwise | Stage1 CorePlan lane |
| C2 | `S1-12 PackedArray non-escaping auto-use pilot` | choose packed ArrayBox for eligible non-escaping record arrays | Stage1 CorePlan lane |
| C3 | `S1-13 const fn const assert` | const evaluator, purity restriction, generated static tables, compile-time asserts | Stage1 const lane |
| C4 | `S1-14 uses capability checker` | check `uses osvm`, `uses atomic`, `uses rawbuf`, and backend capability gates; keep `cap` blocks deferred | Stage1 capability lane |
| C5 | `S1-15 Span API and view no-escape decision` | start with bounded Span APIs; add scoped `view` syntax only if no-escape needs a syntax boundary | Stage1 raw-view lane |
| C6 | `S1-16 delegation-no-inheritance closeout` | canonicalize `delegate field exposes`, quarantine `from` / `override` / internal `extends`, and reject inheritance mental models | Stage1 core-language docs lane |
| C7 | `S1-17 delegate exposes lowering` | resolve typed delegate fields, reject collisions, generate forwarding | Stage1 delegation lane |
| C8 | `S1-18 interface impl static conformance deferred` | method-set checking and static dispatch only after delegation is insufficient | Stage1 interface lane |
| C9 | `S1-19 delegate implements Interface deferred` | use interface method set as a forwarding list after interface MVP exists | Stage1 delegation/interface lane |
| C10 | `S1-20 module visibility semantics` | package layout, visibility, duplicate import rejection, alias rebinding rejection; include migration plan from `using` if needed | Stage1 module lane |
| C11 | `S1-21 check report object` | labeled proof report object and diagnostics integration | Stage1 proof lane |

## Delegation track

This track is governed by:

```text
docs/development/current/main/design/delegation-no-inheritance-ssot.md
```

| Order | Row | Slice |
| --- | --- | --- |
| D1 | `delegation-no-inheritance SSOT` | docs-only decision: no inheritance, no `extends`, no `super`, no implicit field merge |
| D2 | `Stage0 delegate syntax metadata capsule` | parse `delegate field exposes { method, method as alias }` and transport metadata |
| D3 | `Stage1 delegate exposes lowering` | method existence checks, collision checks, forwarding generation |
| D4 | `interface MVP` | method-set contract and static conformance metadata |
| D5 | `delegate field implements Interface` | interface method set becomes forwarding list |
| D6 | `generic interface metadata` | generic arity and substitution metadata |
| D7 | `where constraints` | deferred |

## Feature inventory

| Feature | Priority | Primary reason | First allowed implementation |
| --- | --- | --- | --- |
| loop-only control surface | highest | keeps repetition as one family and avoids `while` / `for` split | docs-only decision, then LoopRange parser capsule |
| `brand` | highest | prevents allocator scalar mix-ups such as page/block/ptr/generation confusion | Stage0 metadata capsule, then Stage1 checker |
| `assert` / `invariant` | highest | moves proof-app conditions toward verifier-readable contracts | Stage0 assert sugar only if needed, Stage1 contract semantics |
| enum state values / `transition` | highest | makes page lifecycle visible to verifier and docs without adding `state` keyword | Stage0 transition metadata capsule, then Stage1 lifecycle facts |
| record literal | high | cleans metadata store construction while preserving identity-free aggregate meaning | Stage0 explicit field-shape capsule, then Stage1 lowering |
| loop range | high | improves metadata and page scans without adding `for` keyword | Stage0 LoopRange metadata capsule, then Stage1 lowering |
| `type` | medium | documents scalar intent without brand strength | Stage0 metadata, Stage1 diagnostics |
| `Result` / `Option` | medium | standardizes failure-heavy allocator APIs | Stage1 prelude over enum surface |
| `guard let` | medium | ergonomic `Result` / `Option` handling | Stage1 only |
| `Array<T>` | medium | typed container surface | Stage0 annotation metadata, Stage1 semantics |
| `PackedArray<T>` | medium | record performance without source ugliness | Stage1 CorePlan, fail-fast on unsupported backend |
| `const fn` / `comptime` / `const assert` | medium | allocator table generation | Stage1 only |
| `uses` | medium | low-level effect permission and backend gates | Stage0 metadata, Stage1 policy |
| delegation no-inheritance | medium | existing composition needs a clean canonical surface before interface work | docs first, then Stage0 metadata, then Stage1 lowering |
| `Span<T>` / deferred `view` | later | bounded raw view instead of C pointer style | Stage1 only |
| `interface` / `impl` | later | static host/substrate policy contracts only after delegation is insufficient | Stage1 only |
| module visibility | later | package hygiene after bootstrap route is stable; `using` remains current import | Stage0 minimal header, Stage1 visibility |
| `check report` | later | richer proof output | Stage1 proof object |

## Parking lot and forbidden shortcuts

Do not implement these as shortcuts:

```text
Stage0 brand checker
Stage0 invariant checker
Stage0 state transition checker
Stage0 PackedArray planner
Stage0 const evaluator
Stage0 interface conformance
while / for / repeat / until keywords
extends / super / origin keywords
inherited fields
property forwarding
automatic delegate conflict resolution
state keyword MVP
cap block MVP
ordinary box auto-recordification
record as ordinary box
PackedArray silent boxed fallback
capability as backend route selector
Result / Option null sugar
Rust-side heavy borrow checker as the view model
```

If a row appears to need one of these, split it and move the semantic slice to Stage1.

## Relation to allocator current lane

This document does not consume allocator row `M211 purge candidate policy inventory`.
It only fixes the durable language-feature order that future cards must follow.
