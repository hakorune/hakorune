---
Status: SSOT
Decision: accepted
Date: 2026-05-14
Scope: Stage0 / Stage1 / Stage2-mainline responsibility split for future language features.
Related:
  - docs/development/current/main/design/stage0-llvm-line-shape-inventory-ssot.md
  - docs/development/current/main/design/stage1-mir-authority-boundary-ssot.md
  - docs/development/current/main/design/language-minimal-surface-ssot.md
  - docs/development/current/main/design/delegation-no-inheritance-ssot.md
  - docs/development/current/main/design/language-feature-implementation-order-ssot.md
  - docs/reference/language/low-level-capabilities.md
---

# Stage0 / Stage1 Feature Responsibility Split SSOT

## Decision

Stage0 is a thin bootstrap reader and transport lane.
Stage1 owns language meaning.
Stage2-mainline is the daily selfhost owner after Stage1 can emit the same shape.

This applies to every future language feature in the low-level Hakorune surface.
Stage0 may accept syntax or move metadata, but it must not become the semantic authority.

The canonical surface rule is defined by:

```text
docs/development/current/main/design/language-minimal-surface-ssot.md
```

Notable consequences:

```text
while / for / repeat / until are not canonical.
loop header shapes are the repetition family.
existing delegation is the concrete composition family.
interface / impl is later static conformance only.
state values use enum; transition carries lifecycle relations.
uses is the MVP capability surface; cap blocks are deferred.
```

## Stage split

| Stage | Owns | Does not own |
| --- | --- | --- |
| Stage0 | syntax acceptance, AST / metadata transport, trivial desugar to already-defined syntax, bootstrap route, diagnostics / fail-fast plumbing, host substrate such as OS / file / env / process / ABI / backend support | type system, brand checking, record layout policy, invariant checking, state transition checking, PackedArray lowering policy, capability policy, generic constraint solving, optimizer policy, allocator algorithm meaning |
| Stage1 | language meaning, verifier facts, CorePlan facts, semantic lowering, selfhost compiler structure, feature diagnostics | host OS substrate, ABI mechanics, process/file/env mechanics, backend plumbing that is not language meaning |
| Stage2-mainline | daily selfhost mainline and authority migration | new Rust-side feature meaning |

## Stage0 capsule contract

Every Stage0 feature row that accepts future syntax must be written as a capsule.

```text
owns:
  parse / metadata / trivial desugar only

does_not_own:
  semantic checking / verifier policy / lowering policy

retire_when:
  Stage1 parser and metadata transport emit the same shape
```

A capsule may fail-fast on malformed syntax or unsupported transport shape.
A capsule must not silently fallback to an ordinary runtime meaning.

## Decision tests

Use this checklist before opening a feature row.

| Question | Decision |
| --- | --- |
| Can the feature be lowered to existing syntax without new meaning? | Stage0 capsule may be allowed. |
| Does the feature create new type meaning? | Stage1 owns it. |
| Does it need verifier or CorePlan facts? | Stage1 owns it. |
| Does it require backend capability policy or unsupported-backend behavior? | Stage1 owns the language decision. |
| Does it decide allocator lifecycle meaning? | Stage1 owns it. |
| Is it syntax or metadata transport only? | Stage0 capsule may carry it with a retire condition. |

## Feature responsibility map

| Feature | Stage0 responsibility | Stage1 responsibility | Notes |
| --- | --- | --- | --- |
| `loop cond` | existing condition-loop parse | loop facts, verifier facts, optimizer metadata | `while` is not a canonical keyword. |
| `loop i in a..b` | parse and transport LoopRange metadata only | entry-bound capture, read-only index, continue-safe lowering, bounds facts, verifier facts | Stage0 must not desugar this shape because `continue` must still step the index. |
| `type` | alias declaration parse and metadata transport | diagnostics and exact type facts | Stage0 must not turn aliases into checker truth. |
| `brand` | declaration parse and underlying type metadata transport | brand constructors, unwrap policy, mixed-brand rejection, verifier facts | `PageId` and `BlockId` must not be interchangeable once Stage1 owns meaning. |
| `record` | declaration parse and transport | layout plan, scalar replacement, field validation, materialization boundary | Ordinary `box` must not be silently treated as `record`. |
| record literal | literal parse and field-shape metadata | construction/read lowering, missing/extra field rejection | Stage0 transports shape only. |
| `with` update | none by default, or metadata only if explicitly carded | record update semantics and packed-array integration | Array element write-through is deferred. |
| `assert` | optional fail-fast sugar only | runtime-check insertion, verifier discharge, diagnostics | Stage0 must not infer proof facts. |
| `requires` / `ensures` / `invariant` | syntax and metadata transport only | contract lowering, invariant checking, verifier facts | Invariants are Stage1 meaning. |
| enum state values / `transition` | transition declaration parse and enum-reference metadata transport | transition legality, lifecycle verifier integration | `state` keyword is not canonical MVP syntax. |
| `Result` / `Option` | enum surface only | prelude, known-enum diagnostics, exhaustiveness, sugar integration | Stage0 must not special-case them. |
| `guard let` | none | pattern binding and match desugar | Requires Stage1 pattern meaning. |
| `Array<T>` | generic type annotation parse and metadata transport | typed array semantics | Storage choice belongs to Stage1 / CorePlan. |
| `PackedArray<T>` | metadata transport only | eligibility gate, packed ArrayBox plan, unsupported-backend fail-fast | Silent boxed fallback is forbidden. |
| `const fn` / `comptime` / `const assert` | existing static const tables and simple const integer expressions only | const evaluator, purity restriction, table generation, const assert | Stage0 must not grow a const evaluator. |
| `uses` / deferred `cap` | `uses` syntax parse and metadata transport; no `cap` block MVP | capability checking, host route permission, backend capability gate | Must stay separate from provider activation and hooks. |
| `Span<T>` / deferred `view` | none | scoped view semantics, no-escape check, bounds facts, RawBuf integration | Start with Span APIs; `view` keyword is deferred. |
| `delegate field exposes` / deferred `interface` / `impl` | parse delegate declaration and transport metadata only | delegate target resolution, collision checking, forwarding generation, static conformance only if delegation cannot express the need | `from` / `override` are legacy quarantine, not new canonical spelling. |
| `using` / deferred `module` family | current `using` import remains; minimal module metadata only if needed | visibility, duplicate import rejection, alias rebinding rejection, package layout | Do not keep long-term duplicate import spellings. |
| `check report` | none | proof report object and diagnostics integration | Existing scalar `check` remains the MVP surface. |

## Stop lines

Stage0 must not implement these meanings:

```text
brand checker
invariant checker
state transition checker
PackedArray planner
const evaluator
interface conformance
delegate dispatch semantics
delegate forwarding generation
while keyword
for keyword
state keyword as MVP
cap block syntax as MVP
record-as-box runtime model
Result / Option null sugar
capability policy
allocator lifecycle policy
```

If a Stage0 task starts requiring one of these, stop the row and split it into a Stage1 task.
