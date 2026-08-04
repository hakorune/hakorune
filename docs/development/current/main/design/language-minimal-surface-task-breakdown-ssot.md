---
Status: SSOT
Decision: accepted
Date: 2026-08-05
Scope: Task-sized backlog for the minimal Hakorune language surface.
Related:
  - docs/development/current/main/design/language-minimal-surface-ssot.md
  - docs/reference/language/ownership.md
  - docs/development/current/main/design/delegation-no-inheritance-ssot.md
  - docs/development/current/main/design/box-member-field-method-surface-ssot.md
  - docs/development/current/main/design/stage0-stage1-feature-responsibility-split-ssot.md
  - docs/development/current/main/design/language-feature-implementation-order-ssot.md
  - docs/development/current/main/design/language-result-propagation-and-exit-transaction-ssot.md
  - docs/development/current/main/design/box-lifecycle-cprime-terminal-home-finalization-ssot.md
  - docs/reference/language/grammar-contract.md
---

# Language Minimal Surface Task Breakdown SSOT

## Purpose

This document turns the language-design discussions into task-sized backlog
rows.

It is not the active allocator lane.
`GUARDLET-001 guard-let pattern sugar` is complete as the current Result/Option
control ergonomics row after RESULT-002D.

`ARRAY-RESULT-SSOT` is complete as the docs-only canonical surface decision for
`Array<T>`, `PackedArray<T>`, `Result<T,E>`, `Option<T>`, and
`Type::Variant`.

## Row rules

Feature admission policy:

```text
docs/development/current/main/design/language-minimal-surface-ssot.md
```

Before opening a row, apply the surface admission checklist there. Prefer
folding into an existing canonical family over adding a keyword or parallel
alias.

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

Every source-syntax implementation or retirement series must end with a
reference closeout row after the code and behavior gates are green. The series
is not complete when only parser/AST/lowering code has landed.

```text
<FEATURE>-REFERENCE-CLOSEOUT0-DOC0:
  update canonical EBNF and grammar/profile status
  update the feature reference and migration examples
  record canonical / compatibility / rejected spellings
  remove or mark historical every superseded reference claim
  run reference-link, grammar-contract, and current-pointer guards
```

At minimum inspect and update the applicable subset of:

```text
docs/reference/language/EBNF.md
docs/reference/language/grammar-contract.md
docs/reference/language/status-index.md
docs/reference/language/stage-profiles.md
docs/reference/language/LANGUAGE_REFERENCE_2025.md
docs/reference/core-language/**
docs/reference/boxes-system/**
grammar/language-v1-registry.toml
```

Reference docs describe the landed implementation, not the planned target.
Do not pre-close DOC0 from a design decision alone.

## Current status summary

| Area | Status | Next actionable row |
| --- | --- | --- |
| Minimal keyword surface | docs accepted | no immediate code row |
| Loop-only repetition | LoopRange MVP complete through carrier policy | no immediate loop row |
| Loop cleanup / PackedArray gate | complete through `293x-310` | no immediate cleanup row |
| No-inheritance delegation | Rust exposes lowering exists; Hako publication, semantic-carrier erasure, normal-pipeline admission, and selfhost value proof remain open | `DELEGATE-SELFHOST-VALUE0-D0` |
| Brand/type | brand checker complete; type alias parser capsule complete | `TYPE-002 Stage1 alias diagnostics` |
| Record literal | with-update lowering complete | no immediate row |
| Contracts | syntax metadata capsule complete; semantic activation is parked pending retain/park decision | `LANGUAGE-CONTRACT-FAMILY-PARK0-D0` |
| Enum transition lifecycle | metadata capsule complete; semantic activation is parked pending retain/retire decision | `LANGUAGE-TRANSITION-RETIRE0-D0` |
| Language surface shrink | consultation backlog only; no parser/runtime implementation is authorized from the consultation itself | `LANGUAGE-SURFACE-SHRINK-CENSUS0-D0` |
| Result/Option | guard-let narrow sugar complete | no immediate Result/Option row |
| Generic containers | generic type annotation metadata and arity checker complete | next substitution/semantics row deferred |
| PackedArray | source backend fail-fast complete | no immediate PackedArray row |
| Array / Result / Option canonical surface | docs accepted; LOCALTYPE/ENUMVAR/ARRAY/RESULT/GUARDLET rows complete | no immediate code row |
| Language v1 convergence | Grammar contract accepted; substrate active | `LANGV1-GRAMMAR-CONTRACT-SUBSTRATE-001` |
| Collections / automata | Map exists as ring1 visible owner; Set/FST are not Stage0/core prerequisites | `COLL-001` / `AUTO-001` docs rows, parked behind mimalloc unless blocking |
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
| `LOOP-003A Stage1 LoopRange route decision` | Complete as `293x-325`; fixes metadata/executable route and explicit no-desugar contract. | Stage1 route complete |
| `LOOP-003B Stage1 LoopRange lowering pilot` | Complete as `293x-326`; entry-bound capture, header index PHI, end-exclusive range, step=1, continue-safe step, carrier writes frozen. | Stage1 pilot complete |
| `LOOP-003C LoopRange verifier facts` | Complete as `293x-327`; publishes function-level `loop_range_facts` for index/bound/block facts and read-only index metadata. | Stage1 verifier complete |
| `LOOP-003D LoopRange carrier policy` | Complete as `293x-328`; accepts fresh body-local bindings while keeping index writes and loop-carried writes fail-fast. | Stage1 semantics complete |
| `LOOP-004 canonical loop formatter/docs` | Make paren-less `loop i in a..b` the canonical spelling; optional paren compatibility requires a separate decision. | docs/tooling |
| `LOOPCLEAN-001 loop cleanup phase` | Complete as `293x-289`; open BoxShape cleanup before PackedArray work. | docs |
| `LOOPCLEAN-002 while parser normalization` | Complete as `293x-290`; new parsed `while` returns `Loop`; old JSON `While` remains compat decode. | BoxShape parser cleanup |
| `LOOPCLEAN-003 while variant quarantine` | Complete as `293x-291`; quarantine `ASTNode::While` as legacy-only input and keep compat Program(JSON) Loop lowering. | BoxShape cleanup |
| `LOOPCLEAN-004 range parser helper commonization` | Complete as `293x-292`; share range-header parsing between canonical `loop i in` and legacy `for i in`. | BoxShape parser cleanup |
| `LOOPCLEAN-005 LoopRange AST rename` | Complete as `293x-405`; rename the stale internal `ForRange` AST variant to `LoopRange` while keeping legacy `"ForRange"` JSON decode compatibility. | BoxShape cleanup |

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
| `DELEGATE-SELFHOST-VALUE0-D0` | Run an AST-based census of exact instance-field forwarding wrappers in `lang/src`; classify static-module forwarding and wrappers with extra logic separately. Select at most one real selfhost facade for explicit-wrapper versus delegate parity. Zero candidates keeps production activation parked. | read-only design/evidence |
| `DELEGATE-VERIFIED-EXPANSION0-I0-R0` | After D0 selects a real consumer, implement one Hako-owned `ParsedDelegateDecl -> VerifiedDelegateExpansionPlan -> ordinary methods` pass. Consume the delegate semantic carrier before normal Resolver/MirBuilder admission; retain source provenance only in a non-authoritative diagnostic sidecar. Prove explicit-wrapper parity and zero delegate-specific MIR/runtime/backend handling. | Stage1/selfhost implementation |
| `DELEGATE-REFERENCE-CLOSEOUT0-DOC0` | After verified expansion and normal-pipeline admission are green, synchronize EBNF, grammar registry/contract, stage profiles, field/delegation reference, language reference, and historical inheritance pages with the exact landed scope. | post-implementation docs |
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
no hidden share or ownership repair in expansion
no second Home/callable ABI owner in delegate lowering
no semantic `DelegateDecl` reaching MIR/runtime/backend after verified expansion
no interface activation without an independently evidenced abstract method-set consumer
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
| `LANGUAGE-CONTRACT-FAMILY-PARK0-D0` | Re-decide `requires` / `ensures` / `invariant` as one family. Inventory real source users and semantic consumers; select retain-with-one-product or park-from-v1. If park is accepted, explicitly supersede `CONTRACT-002` before deletion. | design stop |
| `CONTRACT-003 contract runtime-check insertion` | Parked behind `LANGUAGE-CONTRACT-FAMILY-PARK0-D0`; insert runtime pre/post/invariant checks only if the family is retained. | Stage1 semantics parked |
| `CONTRACT-004 contract verifier discharge` | Parked behind `LANGUAGE-CONTRACT-FAMILY-PARK0-D0`; statically discharge proven checks only after one verified contract product exists. | Stage1 verifier parked |
| `TRANS-001 transition metadata capsule` | Complete as `293x-283`; parses canonical `transition Enum::A -> Enum::B by method` and transports box-local lifecycle relation metadata. Legacy `Enum.A` metadata is accepted and normalized by `ENUMVAR-001`. | Stage0 capsule complete |
| `LANGUAGE-TRANSITION-RETIRE0-D0` | Inventory transition syntax, carriers, and semantic consumers; decide whether ordinary methods plus derived verifier facts replace the metadata-only DSL. A retire decision must explicitly supersede `TRANS-001` before parser/AST/JSON removal. | design stop |
| `TRANS-002 transition legality checker` | Parked behind `LANGUAGE-TRANSITION-RETIRE0-D0`; check declared transitions only if the syntax family is retained. | Stage1 semantics parked |
| `TRANS-003 page lifecycle verifier pilot` | Parked behind the transition and contract family decisions. | Stage1 verifier parked |

Stop lines:

```text
no state keyword in MVP
state values are enum values
transition is lifecycle relation only
no Stage0 invariant or transition checker
```

## Language v1 surface-shrink reconsideration

This is a parked consultation pack. It records the 2026-08-04 proposal without
silently superseding accepted grammar/reference SSOTs. The current JoinIR /
MirBuilder blocker remains authoritative; none of these rows authorizes parser,
AST, runtime, or backend edits until `CURRENT_STATE.toml` selects the language
lane and the row's D0 closes.

| Task | Scope | Required successor |
| --- | --- | --- |
| `LANGUAGE-SURFACE-SHRINK-CENSUS0-D0` | Build one machine-readable static census per syntax family: canonical/compat/historical status, real `.hako` sites, parser producer, AST/JSON carrier, semantic/MIR/runtime/backend consumer, existing-language replacement, migration recipe, and exact delete edge. Usage count alone is not selection authority. | Select bounded family D0 rows only; no repository-wide delete row. |
| `BOX-MEMBER-PROPERTY-RETIRE0-*` | Reuse the accepted field/method Property retirement series in `box-member-field-method-surface-ssot.md`; do not duplicate it here. | `BOX-MEMBER-PROPERTY-REFERENCE-CLOSEOUT0-DOC0` after implementation. |
| `DELEGATE-SELFHOST-VALUE0-D0` | Retain-versus-park evidence for narrow explicit field/method delegation, with one real selfhost facade at most. Class inheritance remains rejected. | `DELEGATE-VERIFIED-EXPANSION0-I0-R0`, then `DELEGATE-REFERENCE-CLOSEOUT0-DOC0`. |
| `LANGUAGE-CONTRACT-FAMILY-PARK0-D0` | Decide `requires` / `ensures` / `invariant` together; do not activate one while the others remain consumer-free metadata. | A retain implementation series or a bounded retirement series, then family DOC0. |
| `LANGUAGE-TRANSITION-RETIRE0-D0` | Decide whether `transition` has a real verifier consumer or is replaced by ordinary methods plus derived facts. | Bounded retirement or retained verifier series, then transition DOC0. |
| `LANGUAGE-GATE-TOPLEVEL-ONLY0-D0` | Audit program-item, import, member, statement, and Rune gate forms. Prefer one pre-resolution top-level declaration-selection owner; do not infer that member/statement forms may be deleted before source and signature-parity census. | If accepted, one source-migration row, one atomic parser/carrier retirement row, then gate DOC0. |
| `LANGUAGE-RESULT-EXIT-C-PRIME0-D0` | Accepted 2026-08-05. Recoverable failure is `Result<T,E>`; unchanged propagation is exact Result-only postfix `?`; Option `?`, source `try`/`throw`/`catch`, and `RecoverableFailure` are rejected; Fault is terminal; standalone `cleanup {}` is the sole lexical registration. This closes the earlier `LANGUAGE-RESULT-ONLY-FAILURE0-D0` and `LANGUAGE-SINGLE-CLEANUP-SURFACE0-D0` questions without activating code. | `P0` -> Trivial `I0` -> `R0` -> Unique/field `HOME0-I0` -> Shared `HOME0-I0/S` -> mandatory `DOC0`. |
| `LANGUAGE-RESULT-EXIT-C-PRIME0-P0` | Read-only census of every current `?`/ternary, try/throw/catch/RecoverableFailure, handler/TryCatch carrier, cleanup spelling, scope-fini alias, environment gate, retry, fixture, and source caller across parser through backend/docs. | Every authority receives keep/migrate/delete/reject disposition; unknown blocks I0/R0. |
| `LANGUAGE-RESULT-EXIT-C-PRIME0-I0` | Split cleanup from TryCatch, seal one verified exit transaction and one typed Result propagation plan, then activate one exact Trivial-only Result consumer with backend capability/fail-fast. | Exactly-once evaluation, exact `E`, protected Trivial pending carrier, LIFO cleanup, first-Fault/suppressed-cleanup-Fault chronology, no dynamic QMark/direct Return, no unresolved ternary collision; Home-bearing routes reject before effects. |
| `LANGUAGE-RESULT-EXIT-C-PRIME0-R0` | Retire dynamic/arbitrary QMark, ternary `? :` producer/ambiguity after explicit `if`/`match` migration, try/throw/catch/RecoverableFailure, TryCatch/CatchClause cleanup encoding, local/postfix cleanup, scope-fini aliases, ambient gates, and compatibility retry. | Old producer/consumer counts zero before closeout. |
| `LANGUAGE-RESULT-EXIT-C-PRIME0-HOME0-I0` | After Home relation/ABI/CFG Flow, terminal DropPlan, and Unique/field finalization slices, activate only Unique Home Result payloads/errors, pending carriers, local release, verified owning-field teardown, lifecycle handoff, and fini-Fault chronology. | Unique payload/error Home exactly once, no hidden share, pending value destroyed on Fault, first Fault primary, remaining local/field/native teardown best effort; every Shared Home route rejects before effects. |
| `LANGUAGE-RESULT-EXIT-C-PRIME0-HOME0-I0/S` | After `HOME0-I0`, `OWN-HOME-SHARE0-I0`, `OWN-TERMINAL-HOME-DROP-PLAN0-S0/S`, and `OWN-LAST-HOME-FINALIZATION-C-PRIME0-I0/S`, activate Shared Home Result payload/error propagation and release in the same exit transaction. | Shared non-last release dispatches no hook; terminal winner dispatches exactly once; no fallback or re-inferred owner policy. |
| `LANGUAGE-RESULT-EXIT-C-PRIME0-DOC0` | Implementation-after reference closeout; never close from this docs-only Decision. | Runs only after P0/I0/R0/HOME0-I0/HOME0-I0/S and backend parity; EBNF/registry/corpus, Rust/Hako parser witnesses, status/support/topic references, examples, redirects, and diagnostics agree with landed behavior. |
| `LANGUAGE-LEGACY-SUGAR-RETIRE0-D0` | Partition `init`, `flow`, `include`, `box from`, `from` call, `override`, `peek`, `while`, scope-fini aliases, and cleanup sugars by semantic owner. | One bounded family/acceptance shape per implementation commit; no mixed mega-retirement. Each family ends in DOC0. |
| `LANGUAGE-OUTBOX-RETIRE0-D0` | Run only after Home result relation is implemented and real outbox users have a migration mapping. | Home-gated implementation and outbox DOC0. |
| `LANGUAGE-V1-SURFACE-REFERENCE-CLOSEOUT0-DOC0` | Final cross-family audit after all selected implementation and per-family DOC0 rows are green. Prove EBNF, registry, Rust/Hako parser witnesses, status index, stage profiles, feature references, and historical redirects agree. | `LANGV1-CONFORMANCE-CLOSEOUT-001`. |

Global stop lines:

```text
do not mix Property, delegate, contracts, transition, gate, failure, cleanup,
or legacy sugar retirement in one implementation row
do not delete accepted syntax from a consultation result without an explicit
superseding Decision in docs/reference/**
do not claim any RESULT-EXIT or Home reference closeout before implementation,
migration, and backend parity gates pass
do not promote interface / implements until a concrete abstract method-set
consumer proves delegate/exposes insufficient
```

## Result, Option, and guard-let tasks

| Task | Scope | Stage |
| --- | --- | --- |
| `RESULT-001 Result Option prelude` | Complete as `293x-314`; define built-in `Result<T,E>` and `Option<T>` enum surfaces plus dot-variant fail-fast diagnostics. | Stage1 prelude complete |
| `RESULT-002A prelude enum missing-arm diagnostics` | Complete as `293x-319`; improve missing-arm diagnostics for built-in `Option<T>` / `Result<T,E>` enum matches. | Stage1 diagnostics complete |
| `RESULT-002B prelude enum payload diagnostics` | Complete as `293x-320`; improve arity/payload diagnostics for `Ok`, `Err`, `Some`, and `None`. | Stage1 diagnostics complete |
| `RESULT-002C known-enum exhaustiveness underscore rules` | Complete as `293x-321`; keep `_` rules explicit for known enum exhaustiveness. | Stage1 diagnostics complete |
| `RESULT-002D generic enum expected-type diagnostics` | Complete as `293x-322`; diagnose ambiguous prelude generic enum local constructors without adding inference. | Stage1 diagnostics complete |
| `GUARDLET-001 guard-let pattern sugar` | Complete as `293x-323`; lower narrow `guard let Type::Variant(binding) = expr else { ... }` through existing Local / If / EnumMatchExpr pieces. | Parser sugar complete |

Stop lines:

```text
no null sugar
no try/throw/catch family
Result-only postfix ?; exact E and no custom Try protocol
Option ? rejected in v1
no Stage0 Result/Option special-case
```

## Generic, array, and PackedArray tasks

| Task | Scope | Stage |
| --- | --- | --- |
| `GEN-001 generic type annotation metadata capsule` | Complete as `293x-285`; parses `Array<T>`, `PackedArray<T>`, `Span<T>`, generic records/interfaces as metadata. | Stage0 capsule complete |
| `GEN-002 generic arity check` | Validate parameter counts without full constraint solving. | Stage1 semantics |
| `ARRAY-RESULT-SSOT` | Canonicalize `Array<T>`, `PackedArray<T>`, `Result<T,E>`, `Option<T>`, and `Type::Variant`; no implementation. | docs/reference |

## Collections and automata tasks

Canonical placement:

```text
Map<K,V>:
  user-visible collection semantics; `HashMap` is not canonical source surface

Set<T>:
  ring1 collection semantic wrapper over Map, not Stage0

FST:
  compiler/std automata library, not language core
```

Task SSOT:

```text
docs/development/current/main/design/collection-set-map-fst-task-breakdown-ssot.md
```

| Task | Scope | Stage |
| --- | --- | --- |
| `COLL-001 Map/Set/HashMap placement SSOT` | Complete/maintain docs deciding Map canonical, HashMap implementation detail, Set as ring1 wrapper. | docs |
| `COLL-002 Set semantic wrapper` | Implement `Set<T>` as `Map<T,i64>` wrapper; no raw Set substrate. | ring1 collection |
| `COLL-003 Set proof app and guard` | Prove `add/has/remove/size/clear` visible contract. | ring1 proof |
| `COLL-004 key capability inventory` | Document supported key routes and fail-fast unsupported generic keys. | Stage1 / substrate boundary |
| `AUTO-001 FST placement SSOT` | Decide FST belongs to compiler/std automata library, not language core. | docs |
| `AUTO-002 FST record vocabulary` | Define `FstState` / `FstTransition` record shapes over Array/PackedArray. | library design |
| `AUTO-003 compiler keyword-table FST pilot` | Use FST for compiler dictionary only if evidence appears. | compiler library |

Mimalloc ordering:

```text
do not move Set or FST before MIMAP-008 by default
use existing MapBox only if a mimalloc row genuinely needs dynamic lookup
open Set only if unique membership becomes the blocker
```

Stop lines:

```text
no Stage0 Set/FST
no HashMap canonical spelling
no RawSet substrate first
no FST language keyword
```
| `LOCALTYPE-001 local type annotation metadata capsule` | Parse and transport `local name: Type = expr` without type meaning. | Stage0 metadata |
| `ENUMVAR-001 enum variant canonical surface` | Keep `Type::Variant` canonical; avoid dot variants and unqualified canonical constructors. | Stage1 enum surface |
| `ARRAY-001 typed context array literal` | Complete as `293x-313`; interpret `[]` and non-empty literals only under `Array<T>` local typed context, with PackedArray no-fallback fail-fast. | Stage1 typed collection complete |
| `ARRAY-002A typed Array method contract` | Complete as `293x-315`; define canonical `Array<T>` methods (`push`, `get`, `set`, `length`) and diagnostics without element checker expansion. | Stage1 typed collection complete |
| `ARRAY-002B typed local Array element checks` | Complete as `293x-316`; track local `Array<T>` element contexts for literal and direct method values. | Stage1 typed collection complete |
| `ARRAY-002C unsupported Array inference fail-fast` | Complete as `293x-317`; keep `local x = []`, mixed literals, and unresolved `T` explicitly rejected. | Stage1 diagnostics complete |
| `ARRAY-002D ArrayBox JSON v0/backend guard` | Complete as `293x-318`; guard that ordinary `Array<T>` lowers through ArrayBox while `PackedArray<T>` never silently falls back. | Stage1/backend guard complete |
| `RESULT-001 Result/Option prelude diagnostics` | Complete as `293x-314`; keep `Result<T,E>` / `Option<T>` as enum surfaces with explicit `Type::Variant` and prelude lookup. | Stage1 enum/prelude complete |
| `PACKED-001 PackedArray eligibility gate` | Complete as `293x-293`; fail-fast if packed residence cannot be proven for declaration type metadata. | Stage1 CorePlan |
| `PACKED-002 PackedArray non-escaping auto-use pilot` | Complete as `293x-324`; emit metadata-only source `PackedArray<Record>` pilot rows by consuming existing C209 plans. | Stage1 CorePlan complete |

Stop lines:

```text
no silent Boxed fallback for PackedArray
no Stage0 PackedArray planner
no generic constraint solver in MVP
```

## Language v1 pre-freeze convergence packet

These rows close contract drift before the selfhost language v1 surface is
called frozen. The active order and executable substeps live only in:

```text
docs/development/current/main/workstreams/language-v1-convergence-current.md
```

This backlog records macro-row boundaries and summary acceptance. The
workstream owns current detail when wording drifts. Open one numbered card for
the current row; do not create cards for inventories, consultations, fixtures,
or reruns.

### LANGV1-CONSTITUTION-001 seven-law language charter

Create one normative charter for same-syntax/same-guarantee, meaning versus
representation, absence/failure/Fault, identity versus lifetime, exactly-once
sugar, explicit compatibility, and fail-fast-before-effects. This row changes
no parser or runtime behavior. Complete as 3457.

### LANGV1-SEMANTIC-KERNEL-001 Outcome, Place, and evaluation law

Define one `Outcome` algebra and one `Place` model. Fix source order,
exactly-once evaluation, cleanup precedence, and guard-let NoFallthrough. The
first implementation slice replaces compound-assignment AST cloning with one
Place read-modify-write path and side-effecting fixtures. Current action is
`LANGV1-GRAMMAR-CONTRACT-SUBSTRATE-001`; the accepted grammar basis is normative
in `docs/reference/language/grammar-contract.md`.

### LANGV1-GRAMMAR-001 canonical grammar and dual-parser conformance

Current evidence is split:

```text
canonical EBNF: guard expr else, match; no try/from/peek surface row
legacy topic page: guard cond ->
Rust parser: guard else; try is compatibility-gated; from still parses
selfhost parser: try and peek parser boxes remain live
v1 freeze doc: previously listed try as required
```

Required work:

1. Create one machine-readable grammar-surface manifest. Classify every row as
   `canonical`, `compatibility_only`, `reserved`, or `rejected`, including
   `guard else`, `guard ->`, `match`, `peek`, `try`, postfix `catch/cleanup`,
   `from`, and delegation syntax.
2. Make `docs/reference/language/EBNF.md` the only canonical grammar owner;
   topic/profile/freeze docs may describe semantics or migration only.
3. Keep `Canonical` as default and allow aliases only through explicit
   `Compat2025`; aliases normalize immediately to canonical shape.
4. Run the same golden corpus through the Rust and selfhost parsers in both
   profiles and compare a shared span-free `ParseWitness`.
5. Compare accept/reject result, stable diagnostic tag, and normalized shape.
   Missing rows and profile drift fail fast.
6. Migrate or quarantine live selfhost sources that require compatibility
   syntax; do not make legacy syntax canonical merely to pass the suite.

Acceptance:

```text
canonical_grammar_owner_count = 1
default_parser_conformance = 1
compatibility_profile_explicit = 1
golden_accept_reject_parity = 1
golden_ast_json_shape_parity = 1
new_surface_syntax = 0
silent_legacy_acceptance = 0
default_profile = Canonical
implicit_compatibility_count = 0
```

### LANGV1-TYPE-GUARANTEE-001 annotation guarantee matrix

Current annotations mix metadata and semantic checks. The target decision is:

```text
annotation omitted -> Any
x: T -> gradual semantic contract T
representation/planner hint -> MIR facts, Plan, or Rune
```

Existing narrow checks include exact numeric field writes, record construction,
typed `Array<T>` elements, and Weak fields. Many parameter, return, local, and
ordinary Box-field annotations remain metadata and require migration.

Required work:

1. Publish one matrix with rows for parameter, return, local, Box field, record
   field, static table element, ordinary collection element, typed `Array<T>`
   element, and Weak field.
2. For each row record parser transport, compile-time check, MIR verifier,
   runtime check, backend support, failure tag, and unsupported-backend policy.
3. Classify each guarantee as `metadata_only`, `compile_time`, `mir_verified`,
   `runtime_checked`, or an explicit combination. Never use `typed` alone as a
   proof claim.
4. Add positive and negative fixtures for every non-metadata guarantee. A
   backend that cannot preserve a live guarantee must fail fast.
5. Derive later implementation rows only from matrix cells whose desired v1
   guarantee differs from current behavior. Migrate representation-only hints
   before activating the gradual contract; do not introduce a broad static
   type checker under this row.

Acceptance:

```text
annotation_site_set_closed = 1
guarantee_kind_explicit_per_site = 1
metadata_not_semantic_truth = 1
live_guarantees_fixture_backed = 1
unsupported_backend_fail_fast = 1
broad_static_type_checker = 0
annotation_semantic_contract = 1
unannotated_value_contract = Any
metadata_hint_spelled_as_type_annotation = 0
```

### LANGV1-FAILURE-OUTCOME-001 absence, failure, Fault, and Unit

Define `Option::None` as value absence, `Result::Err` as recoverable failure,
`Fault` as non-implicit contract/runtime failure, and `Normal(Unit)` as a
successful no-result computation. Classify and migrate live `null` sites by
meaning before restricting `null` to explicit `Compat2025`. Source catch and
`RecoverableFailure` retire rather than becoming a compatibility failure
authority.

### LANGV1-OWNERSHIP-IDENTITY-001 field ownership and identity decision

The accepted `BoxIdentity` contract remains representation-only. The 2026-08-04
Home direction supersedes the earlier assumption that every ordinary field is
Shared. Exact owning field, Shared field, replacement, and container rules now
belong to `OWN-FIELD-CONTAINER-DEST-D0`; only a verified owning field releases
a child Home during parent teardown.

Accepted C′ plus provisional Home storage decision order (2026-08-05):

1. A verified owning field may receive one Home token; a verified Shared field
   may store an independent Shared token. Exact surface and replacement
   semantics remain D0. A field release invokes the child hook only when it is
   the child's terminal Home release.
2. Define overwrite, parent `fini {}` hook, alias escape, cycle, partial
   `birth` failure, and exactly-once finalization for every field ownership
   kind.
3. Use one identity relation for strong Box values, WeakRef tokens, host
   handles, Dead objects, and future reused slots. `BoxIdentity(handle,
   generation)` is the durable contract; current `Arc::ptr_eq` is only the
   current storage projection.
4. Define WeakRef equality and weak-to-strong upgrade against that identity,
   including Dead/Freed behavior and generation mismatch.
5. Add VM/runtime fixtures and backend fail-fast coverage before changing
   cascade behavior or adding syntax.

Accepted v1 law:

```text
ordinary owning field = one forwarded owner
Shared field = independent Shared owner
parent hook runs before verified owning-field releases = 1
verified owning fields release in reverse declaration order = 1
child hook dispatch requires child terminal Home = 1
direct obj.fini() and ordinary fini(...) method = forbidden
terminal DropPlan user-hook authority count = 1
take/return Home forward invokes fini = 0
normal Box manual physical free/reclaim surface = forbidden
StaticUnique / LocalRc / SharedRc source visibility = 0
StaticUnique terminal reclaim uses the sealed C′ DropPlan
```

A future source-exclusive capability or raw-memory facility must reopen a
separate language Decision. It is not an implementation detail of
LANGV1-OWNERSHIP-IDENTITY-001 and is not a dependency of C′ closeout.

Runtime/Ownership SSA implementation order is taskized in:
`docs/development/current/main/investigations/hakorune-home-ownership-task-2026-08-04.md`.

Acceptance:

```text
field_ownership_kinds_closed = 1
cascade_fini_owner_explicit = 1
shared_alias_not_implicitly_finalized = 1
box_weak_identity_relation_count = 1
generation_aware_identity_retained = 1
cross_backend_lifecycle_fixture = 1
```

### LANGV1-CAPABILITY-EFFECT-001 authority, effects, and promises

Keep `uses X`, observed/transitive `EffectSummary`, and Rune contract promises
as three distinct axes. Verify actual effects are within declared authority,
verify promises independently, reject unknown capabilities, and allow backends
to consume only verified Plans.

### LANGV1-CONFORMANCE-CLOSEOUT-001

Run dual-parser ParseWitness conformance and VM/EXE semantic packs, prove zero
implicit compatibility, close v1 freeze, then unpark selfhost and MirBuilder
3456.

## v1 convergence order

```text
LANGV1-CONSTITUTION-001
  -> LANGV1-SEMANTIC-KERNEL-001
  -> LANGV1-GRAMMAR-001
  -> LANGV1-TYPE-GUARANTEE-001
  -> LANGV1-FAILURE-OUTCOME-001
  -> LANGV1-OWNERSHIP-IDENTITY-001
  -> LANGV1-CAPABILITY-EFFECT-001
  -> LANGV1-CONFORMANCE-CLOSEOUT-001
  -> selfhost language v1 freeze closeout
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

When `CURRENT_STATE.toml` explicitly switches to language-surface work, the
re-entry order is:

```text
LANGUAGE-SURFACE-SHRINK-CENSUS0-D0
  -> exactly one selected family D0
  -> its bounded migration / implementation / retirement rows
  -> that family's post-implementation REFERENCE-CLOSEOUT0-DOC0
  -> return to the census for the next family
  -> LANGUAGE-V1-SURFACE-REFERENCE-CLOSEOUT0-DOC0
  -> LANGV1-CONFORMANCE-CLOSEOUT-001
```

The older numbered sequence below records landed implementation chronology and
remaining historical rows. It must not override that re-entry selector:

1. `DEL-001 legacy delegation status reconcile`
2. `LOOP-002 Stage0 LoopRange parser capsule`
3. `DEL-002 Stage0 delegate syntax metadata capsule`
4. `DEL-003 Stage1 delegate exposes lowering`
5. `LOOP-003A Stage1 LoopRange route decision` (complete as `293x-325`)
6. `LOOP-003B Stage1 LoopRange lowering pilot` (complete as `293x-326`)
7. `LOOP-003C LoopRange verifier facts and read-only index proof surface` (complete as `293x-327`)
8. `LOOP-003D LoopRange carrier policy` (complete as `293x-328`)
9. `PACKED-003 source PackedArray direct-read consumption` (complete as `293x-329`)
10. `PACKED-004 source PackedArray backend fail-fast hardening` (complete as `293x-330`)
9. `BRAND-001 Stage0 brand declaration metadata capsule` (complete as `293x-275`)
10. `BRAND-002 Stage1 brand constructor unwrap policy` (complete as `293x-276`)
11. `BRAND-003 Stage1 brand mismatch checker` (complete as `293x-277`)
12. `TYPE-001 Stage0 type alias metadata capsule` (complete as `293x-278`)
13. `REC-001 Stage0 explicit record literal shape capsule` (complete as `293x-279`)
14. `REC-002 Stage1 record construction/read lowering` (complete as `293x-280`)
15. `REC-003 record with-update lowering` (complete as `293x-281`)
16. `CONTRACT-002 contract syntax metadata capsule` (complete as `293x-282`)
17. `TRANS-001 transition metadata capsule` (complete as `293x-283`)
18. `USES-001 method-level uses metadata capsule` (complete as `293x-284`)
19. `GEN-001 generic type annotation metadata capsule` (complete as `293x-285`)
20. `GEN-002 generic arity check`
21. `ARRAY-RESULT-SSOT` (complete docs-only)
22. `LOOPCLEAN-001 loop cleanup phase` (complete docs-only)
23. `LOOPCLEAN-002 while parser normalization` (complete as `293x-290`)
24. `LOOPCLEAN-003 while variant quarantine` (complete as `293x-291`)
25. `LOOPCLEAN-004 range parser helper commonization` (complete as `293x-292`)
26. `PACKED-001 PackedArray eligibility gate` (complete as `293x-293`)
27. `ASTCLEAN-017 runner/provider/runtime dead_code rationale pass` (complete as `293x-310`)
28. `ENUMVAR-001 enum variant canonical surface` (complete as `293x-311`)
29. `LOCALTYPE-001 local type annotation metadata capsule` (complete as `293x-312`)
30. `ARRAY-001 typed context array literal` (complete as `293x-313`)
31. `RESULT-001 Result/Option prelude diagnostics` (complete as `293x-314`)
32. `ARRAY-002A typed Array method contract` (complete as `293x-315`)
33. `ARRAY-002B typed local Array element checks` (complete as `293x-316`)
34. `ARRAY-002C unsupported Array inference fail-fast` (complete as `293x-317`)
35. `ARRAY-002D ArrayBox JSON v0/backend guard` (complete as `293x-318`)
36. `RESULT-002A prelude enum missing-arm diagnostics` (complete as `293x-319`)
37. `RESULT-002B prelude enum payload diagnostics` (complete as `293x-320`)
38. `RESULT-002C known-enum exhaustiveness underscore rules` (complete as `293x-321`)
39. `RESULT-002D generic enum expected-type diagnostics` (complete as `293x-322`)
40. `GUARDLET-001 guard-let pattern sugar` (complete as `293x-323`)
41. `PACKED-002 PackedArray non-escaping auto-use pilot` (complete as `293x-324`)

This order keeps early wins concrete while avoiding Stage0 semantic growth.

## LOOP-003 split update (2026-05-14)

`LOOP-003` is split to keep the route decision separate from executable
semantics:

```text
LOOP-003A:
  landed route decision and JSON bridge fail-fast receiver

LOOP-003B:
  next lowering pilot with entry-bound capture and continue-safe step

LOOP-003C:
  later verifier facts and read-only index enforcement
```

## LOOP-003B update (2026-05-14)

`LOOP-003B` landed the first executable JSON v0 bridge LoopRange pilot:
entry-bound capture, header index PHI, end-exclusive compare, fixed step 1, and
continue-to-step routing. `LOOP-003C` then published function-level
`loop_range_facts`; carrier writes are governed by `LOOP-003D`: fresh body-locals are allowed, loop-carried writes remain fail-fast.

## mimalloc blueprint handoff (2026-05-14)

The next allocator-facing lane should not wait for every optional language
feature. It should start with docs/inventory rows that use upstream mimalloc as
an oracle:

```text
MIMAP-001 upstream source pin
MIMAP-002 source concept inventory
MIMAP-003 lifecycle rewrite blueprint
MIMAP-004 substrate and representation gap ledger
```

Executable mimalloc slices can now start after MIMAP inventory selects the first near-transcription slice.

Canonical handoff board:

```text
docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md
```
