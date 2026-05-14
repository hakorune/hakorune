---
Status: SSOT
Decision: accepted
Date: 2026-05-14
Scope: Task-sized backlog for the minimal Hakorune language surface.
Related:
  - docs/development/current/main/design/language-minimal-surface-ssot.md
  - docs/development/current/main/design/delegation-no-inheritance-ssot.md
  - docs/development/current/main/design/stage0-stage1-feature-responsibility-split-ssot.md
  - docs/development/current/main/design/language-feature-implementation-order-ssot.md
---

# Language Minimal Surface Task Breakdown SSOT

## Purpose

This document turns the language-design discussions into task-sized backlog
rows.

It is not the active allocator lane.
`GEN-002 generic arity check` is the current selected language blocker
unless the user explicitly switches to the language lane.

## Row rules

Every task must keep the Stage0/Stage1 split explicit.

```text
Stage0 rows:
  parse / metadata / trivial desugar only
  require a retire condition

Stage1 rows:
  own meaning / verifier facts / lowering / diagnostics

Forbidden:
  putting semantic ownership into Stage0
  adding duplicate canonical spellings
  silent fallback
```

Each implementation card must include:

```text
Decision:
Canonical syntax:
Owner:
Stage0 owns:
Stage0 does not own:
Stage1 owns:
Fixture / proof app:
Guard:
Unsupported backend behavior:
Stop lines:
Retire condition:
```

## Current status summary

| Area | Status | Next actionable row |
| --- | --- | --- |
| Minimal keyword surface | docs accepted | no immediate code row |
| Loop-only repetition | parser capsule complete | `LOOP-003 Stage1 LoopRange lowering` |
| No-inheritance delegation | exposes lowering complete | `DEL-004 legacy quarantine migration` |
| Brand/type | brand checker complete; type alias parser capsule complete | `TYPE-002 Stage1 alias diagnostics` |
| Record literal | with-update lowering complete | no immediate row |
| Contracts | syntax metadata capsule complete | `CONTRACT-003 contract runtime-check insertion` |
| Enum transition lifecycle | metadata capsule complete | `TRANS-002 transition legality checker` |
| Result/Option | planned | `RESULT-001 prelude and diagnostics` |
| Generic containers | generic type annotation metadata capsule complete | `GEN-002 generic arity check` |
| PackedArray | planned | `PACKED-001 eligibility gate` |
| Uses/capability | method-level metadata capsule complete | `USES-002 capability checker` |
| Span/view | planned later | `SPAN-001 Span API design row` |
| Module visibility | planned later | `MOD-001 using/module migration decision` |
| Check report | planned later | `CHECK-001 check report object design row` |

## Loop-only repetition tasks

Canonical surface:

```hako
loop cond {
    ...
}

loop i in start..end {
    ...
}

loop {
    ...
}
```

| Task | Scope | Stage |
| --- | --- | --- |
| `LOOP-001 loop-only control surface docs` | Decide no `while`, no `for`, no `repeat`, no `until`; docs and examples use `loop` only. | docs, complete via D201 |
| `LOOP-002 Stage0 LoopRange parser capsule` | Parse `loop i in start..end` and transport `LoopRange` metadata. | Stage0 capsule |
| `LOOP-002 status` | Complete as `293x-272`; parser accepts paren-less and parenthesized LoopRange headers and transports LoopRange metadata only. | Stage0 complete |
| `LOOP-002 status` | Complete as `293x-272`; parser accepts paren-less and parenthesized LoopRange headers and transports LoopRange metadata only. | Stage0 complete |
| `LOOP-003 Stage1 LoopRange lowering` | Entry-bound capture, block-local read-only index, end-exclusive range, step=1, continue-safe step. | Stage1 semantics |
| `LOOP-004 LoopRange verifier facts` | Expose index/bounds facts such as `i < end`; add conservative facts only. | Stage1 verifier |
| `LOOP-005 canonical loop formatter/docs` | Make paren-less `loop i in a..b` the canonical spelling; optional paren compatibility requires a separate decision. | docs/tooling |

Stop lines:

```text
no while keyword
no for keyword
no Stage0 range desugar to local/loop/increment
no array iteration in MVP
no custom step in MVP
```

## Delegation no-inheritance tasks

Canonical surface:

```hako
box Child {
    parent: Parent = new Parent()

    delegate parent exposes {
        method
        other as publicOther
    }
}
```

| Task | Scope | Stage |
| --- | --- | --- |
| `DEL-001 legacy delegation status reconcile` | Reconcile `box Child from Parent`, `override`, `from Parent.method`, multiple delegation, and field-visibility proposal status. | docs |
| `DEL-001 status` | Complete as `293x-271`; legacy `from`/`override` docs are historical, not canonical. | docs complete |
| `DEL-002 Stage0 delegate syntax metadata capsule` | Complete as `293x-273`; parses `delegate field exposes { method, method as alias }` and transports metadata. | Stage0 capsule complete |
| `DEL-003 Stage1 delegate exposes lowering` | Complete as `293x-274`; resolves typed delegate target fields, checks method existence, rejects collisions, and generates forwarding methods. | Stage1 semantics complete |
| `DEL-004 legacy quarantine migration` | Map internal `extends` naming to delegation metadata without behavior changes; define retire path. | docs/code-shape |
| `DEL-005 interface MVP` | Define method-set contract and static conformance metadata only after delegation works. | Stage1 later |
| `DEL-006 delegate implements Interface` | Use interface method set as the forwarding list and reject missing methods/collisions. | Stage1 later |
| `DEL-007 generic interface metadata` | Generic arity and substitution metadata for interface signatures. | Stage1 later |
| `DEL-008 where constraints` | Constraint solving and `where` clauses. | deferred |

Stop lines:

```text
no inheritance
no extends as canonical syntax
no super
no origin
no inherited fields
no property forwarding
no wildcard exposes * in MVP
no automatic collision resolution
no Stage0 conformance checker
```

## Brand and type tasks

| Task | Scope | Stage |
| --- | --- | --- |
| `BRAND-001 Stage0 brand declaration metadata capsule` | Complete as `293x-275`; parses `brand PageId: i64` and transports underlying storage metadata only. | Stage0 capsule complete |
| `BRAND-002 Stage1 brand constructor unwrap policy` | Complete as `293x-276`; lowers `BrandName(value)` and `BrandName.unwrap(value)` to explicit Program JSON v0 brand nodes with arity fail-fast. | Stage1 semantics complete |
| `BRAND-003 Stage1 brand mismatch checker` | Complete as `293x-277`; rejects same-program brand-typed call argument mismatches and unbranded values passed to brand parameters. | Stage1 verifier complete |
| `TYPE-001 Stage0 type alias metadata capsule` | Complete as `293x-278`; parses `type Bytes = usize` and transports target type metadata only. | Stage0 capsule complete |
| `TYPE-002 Stage1 alias diagnostics` | Keep alias non-semantic but improve diagnostics and facts. | Stage1 diagnostics |

Stop lines:

```text
no implicit brand conversion
no Stage0 brand checker
no MirType-as-language-semantics expansion
```

## Record tasks

| Task | Scope | Stage |
| --- | --- | --- |
| `REC-001 Stage0 explicit record literal shape capsule` | Complete as `293x-279`; parses `RecordName { field: value }` and transports field-shape metadata only. | Stage0 capsule complete |
| `REC-002 Stage1 record construction/read lowering` | Complete as `293x-280`; validates missing/extra fields and lowers identity-free construction/read metadata. | Stage1 semantics complete |
| `REC-003 record with-update lowering` | Complete as `293x-281`; lowers `value with { field: next }` as replacement, not mutation. | Stage1 semantics complete |
| `REC-004 record shorthand literal decision` | Decide whether `RecordName { field }` is worth adding. | deferred |
| `REC-005 record array element update decision` | Keep `metas.set(i, metas.get(i) with {...})` as MVP; field write-through is later. | deferred |

Stop lines:

```text
record is not ordinary box
box is not auto-recordified
record methods/delegate/interface are not MVP
```

## Contract and lifecycle tasks

| Task | Scope | Stage |
| --- | --- | --- |
| `CONTRACT-001 assert runtime-check sugar decision` | Decide exact `assert cond : message` runtime fail-fast lowering. | Stage1 or Stage0 trivial sugar |
| `CONTRACT-002 contract syntax metadata capsule` | Complete as `293x-282`; parses `requires`, `ensures`, `invariant` metadata only and transports it through AST/JSON. | Stage0 capsule complete |
| `CONTRACT-003 contract runtime-check insertion` | Insert runtime pre/post/invariant checks at defined boundaries. | Stage1 semantics |
| `CONTRACT-004 contract verifier discharge` | Statically discharge proven checks and keep diagnostics stable. | Stage1 verifier |
| `TRANS-001 transition metadata capsule` | Complete as `293x-283`; parses `transition Enum.A -> Enum.B by method` and transports box-local lifecycle relation metadata. | Stage0 capsule complete |
| `TRANS-002 transition legality checker` | Check legal state transitions from enum values. | Stage1 semantics |
| `TRANS-003 page lifecycle verifier pilot` | Apply transition/contract facts to allocator page lifecycle. | Stage1 verifier |

Stop lines:

```text
no state keyword in MVP
state values are enum values
transition is lifecycle relation only
no Stage0 invariant or transition checker
```

## Result, Option, and guard-let tasks

| Task | Scope | Stage |
| --- | --- | --- |
| `RESULT-001 Result Option prelude` | Define canonical `Result<T,E>` and `Option<T>` surface over enum. | Stage1 prelude |
| `RESULT-002 enum diagnostics and exhaustiveness` | Improve match diagnostics and exhaustiveness for known enum shapes. | Stage1 diagnostics |
| `GUARDLET-001 guard-let pattern sugar` | Lower `guard let Pattern = expr else { ... }` through match/pattern rules. | Stage1 semantics |

Stop lines:

```text
no null sugar
no try/throw family
no Stage0 Result/Option special-case
```

## Generic, array, and PackedArray tasks

| Task | Scope | Stage |
| --- | --- | --- |
| `GEN-001 generic type annotation metadata capsule` | Complete as `293x-285`; parses `Array<T>`, `PackedArray<T>`, `Span<T>`, generic records/interfaces as metadata. | Stage0 capsule complete |
| `GEN-002 generic arity check` | Validate parameter counts without full constraint solving. | Stage1 semantics |
| `ARRAY-001 Array typed container semantics` | Define normal typed array behavior. | Stage1 semantics |
| `PACKED-001 PackedArray eligibility gate` | Fail-fast if packed residence cannot be proven. | Stage1 CorePlan |
| `PACKED-002 PackedArray non-escaping auto-use pilot` | Use packed ArrayBox for eligible non-escaping record arrays. | Stage1 CorePlan |

Stop lines:

```text
no silent Boxed fallback for PackedArray
no Stage0 PackedArray planner
no generic constraint solver in MVP
```

## Const, capability, Span/view, module, and proof tasks

| Task | Scope | Stage |
| --- | --- | --- |
| `CONST-001 const fn const assert design row` | Define const evaluator scope, purity, and table generation. | Stage1 later |
| `USES-001 method-level uses metadata capsule` | Complete as `293x-284`; parses `uses osvm`, `uses atomic`, `uses rawbuf` metadata. | Stage0 capsule complete |
| `USES-002 capability checker` | Check allowed host routes and backend capability gates. | Stage1 semantics |
| `CAP-001 cap block decision` | Add block-scoped `cap` only if method-level `uses` is insufficient. | deferred |
| `SPAN-001 Span API design row` | Start with `Span<T>` API over bounded RawBuf views. | Stage1 design |
| `VIEW-001 scoped view syntax decision` | Add `view` only if no-escape needs syntax, not just API. | deferred |
| `MOD-001 using/module migration decision` | Decide migration from current `using` to package/module visibility. | docs |
| `MOD-002 module visibility semantics` | Package layout, visibility, duplicate import rejection, alias rebinding rejection. | Stage1 later |
| `CHECK-001 check report object` | Define labeled report object beyond scalar `check`. | Stage1 later |

Stop lines:

```text
no unsafe keyword
no cap block MVP
no view keyword until Span API is insufficient
no long-term duplicate import spelling without migration plan
```

## Suggested language-lane order

When the allocator M212/M213 lane closes or the user explicitly switches to
language work, start here:

1. `DEL-001 legacy delegation status reconcile`
2. `LOOP-002 Stage0 LoopRange parser capsule`
3. `DEL-002 Stage0 delegate syntax metadata capsule`
4. `DEL-003 Stage1 delegate exposes lowering`
5. `LOOP-003 Stage1 LoopRange lowering` (open; requires JoinIR/CorePlan route, not source-level desugar)
6. `BRAND-001 Stage0 brand declaration metadata capsule` (complete as `293x-275`)
7. `BRAND-002 Stage1 brand constructor unwrap policy` (complete as `293x-276`)
8. `BRAND-003 Stage1 brand mismatch checker` (complete as `293x-277`)
9. `TYPE-001 Stage0 type alias metadata capsule` (complete as `293x-278`)
10. `REC-001 Stage0 explicit record literal shape capsule` (complete as `293x-279`)
11. `REC-002 Stage1 record construction/read lowering` (complete as `293x-280`)
12. `REC-003 record with-update lowering` (complete as `293x-281`)
13. `CONTRACT-002 contract syntax metadata capsule` (complete as `293x-282`)
14. `TRANS-001 transition metadata capsule` (complete as `293x-283`)
15. `USES-001 method-level uses metadata capsule` (complete as `293x-284`)
16. `GEN-001 generic type annotation metadata capsule` (complete as `293x-285`)
17. `GEN-002 generic arity check`

This order keeps early wins concrete while avoiding Stage0 semantic growth.
