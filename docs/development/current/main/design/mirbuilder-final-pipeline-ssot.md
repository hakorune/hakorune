---
Status: SSOT
Date: 2026-09-03
Decision: MIRBUILDER-FINAL-PIPELINE-v1
Scope: canonical source ingressからatomic MIR publicationまでの唯一のglobal pipeline-order authority。Parser grammar、language semantics、Backend loweringの詳細は隣接ownerへ委譲する。
Related:
  - docs/development/current/main/design/recipe-first-entry-contract-ssot.md
  - docs/development/current/main/design/recipe-tree-and-parts-ssot.md
  - docs/development/current/main/design/joinir-loop-selfhost-recipe-pipeline-ssot.md
  - docs/development/current/main/design/loop-common-physical-demand-and-session-ssot.md
  - docs/development/current/main/design/mirbuilder-inplace-replacement-policy-ssot.md
  - docs/development/current/main/design/compiler-pipeline-thinning-ssot.md
  - docs/reference/language/function-exit-and-entry-result.md
  - docs/development/current/main/design/repo-physical-structure-cleanup-ssot.md
  - docs/development/current/main/design/mir-root-facade-contract-ssot.md
  - docs/development/current/main/investigations/function-exit-f1-draft-seal0-s0-execution-task-2026-07-25.md
  - docs/development/current/main/investigations/normal-source-plan0-design-stop-2026-07-26.md
  - src/mir/builder/README.md
---

# MirBuilder Final Pipeline

## Current Capsule

- **Current decision:** the final pipeline remains one-way, and canonical MIR
  calls converge on a typed structural target before argument or MIR effects.
- **Current implementation status:** the typed publication spine is landed for
  StaticBoxMethod, FreeStatic, FreeFunction, Builtin Print, and one root-lexical
  DeclaredInstance source family. The exact key survives Atomic Publish and the
  selected static/free/Print backend rows are typed. Group A now separates the
  canonical `Call(MirCall)` shape from the explicit `LegacyCallV0` outer shape;
  broad compatibility consumers remain reachable, and Hako has no borrow-only
  published-view ingress for SameModuleInstance. R6 Group B's VM canonical
  Print reader is landed; the post-Group-B census found no single next reader
  family with a complete cutover tuple.
- **Latest bounded work:** VM Global canonical cutover landed at `111216b539`;
  the selected next cleanup retargets the stale WSM-G4-min8 success probe to
  explicit pre-WAT rejection.
- **Production stop line:** no String formatter, opaque registry, second AST
  walk, post-argument resolver, optional/empty loan, or backend repair may fill
  a missing semantic target.
- **Retirement finish line:** canonical source families use the one pipeline,
  Call fallback/retry is zero, transitional integration hubs are reduced by
  finite owner, and each switched family repays its exclusive legacy
  code/tests/guards/docs before the next family. Closed detail lives in Git,
  not a copied current-tree archive.

## Decision

この文書は、canonical source ingressからatomic MIR publicationまでの
**唯一のglobal pipeline-order authority**である。Parser grammar／source AST
schema／language semanticsと、published `MirModule`を受け取る各Backendの
lowering詳細は隣接ownerが持つ。この文書はその詳細を吸収せず、受渡し境界と
authorityの向きだけを固定する。

MirBuilder再構築の最終目標は、replacement cell数、pack消化、Rust LOC、または
ファイル数ではない。

最終目標は、source semanticsの決定からfunction draftとmodule公開までを、
次の一方向のproduction authorityへ収束させることである。

```text
Hakorune AST
  -> Resolver
  -> VerifiedResolvedFunction
  -> Control-flow Observation
  -> Facts
  -> RoutePolicy
  -> RecipeComposer
  -> RecipeVerifier
  -> Verified Lowering Plan
  -> Plan / Body Lowering
  -> CanonicalSsaFunctionSessionV2::finish_for_draft_seal
  -> ReadyFunctionDraftSealV1
  -> OpenFunctionDraftSealV1::prepare
  -> PreparedFunctionDraftSealV1
  -> one infallible commit
  -> CompletedFunctionDraftV1
  -> ModuleDraftCollectorV1
  -> atomic module transaction
```

短縮形は次で固定する。

```text
Resolve
-> Observe
-> Facts
-> Recipe
-> Verify
-> Lower
-> Seal
-> Collect
-> Atomic Publish
```

人間向けの七段projectionは次で固定する。これは上のnormative chainを
並べ替える第二pipelineではない。

```text
1. Frontend
   Source -> AST -> Resolve

2. Semantic Observation
   Resolve -> Observe -> Facts

3. Verified Recipe
   Recipe -> Verify

4. Function Lowering Session
   Lower -> function-local finish

5. DraftSeal
   Ready -> prepare -> infallible commit

6. Module Transaction
   Completed drafts -> Collect -> Atomic Publish

7. Backend Boundary
   published MirModule -> VM / AOT / LLVM / other selected backend
```

`Verify`をRecipeより前へ移したり、Backendをsource semanticsのrepair ownerに
したりしない。七段projectionで省略された内部edgeのauthorityは、常に上の
normative chainが優先する。

`MIRBUILDER-INPLACE-REPLACEMENT0`は、この最終形へ現在のproduction
MirBuilderを移す方法である。replacement cellやstructural measurementsは
移行の観測手段であって、最終architectureの代わりではない。

## Current convergence spine

現在のCall seamは全体pipelineから独立した第二roadmapではない。`Resolve`
から`Lower`へauthorityを一方向にするための、次の直列 prerequisite である。

```text
typed Global target family / issuer Decision
  -> reusable ingress lifecycle guard
  -> shared-runner Wpre profile/root/decoder contract
  -> finite explicit-CLI arbitration and outside-fate closure
  -> reference child isolation and CoreDirect typed terminal policy
  -> force-hv1 finite census and selected retirement
  -> strict recursive root owner and by-value decoder seams
  -> strict one-shot schema selection with invalid-v1 retry = 0
  -> MirCall/CallFlags transport retirement
  -> source-owned effect authority for every promoted target family
  -> preserve exact free/static/import/compatibility target relations
  -> JoinIR declaration co-seal and false GC Global retirement
  -> observer-only FunctionCall completion contract and package install gate
  -> builtin/Extern disposition and finite preflight lineage classification
  -> package-owned pre-effect site/target disposition
  -> one affine raw handoff and direct-payload deletion
  -> exact Method(Some) receiver/argument ABI preservation
  -> named compatibility late-recovery retirement and overlapping Method(None) retirement
  -> remaining static/CorePlan/operator producer dispositions
  -> exact touched-owner shelf and finite current-HEAD B1 census
  -> typed Global common-core and all compiled-consumer cutover
  -> remaining wire / construction / selected-terminal closure
  -> current-HEAD consumer census
  -> mandatory-Callee Call schema + impossible-state guard retirement
  -> post-Call integration-hub and source-shelf cleanup
  -> remaining selected pipeline cutovers
  -> final repository convergence audit
```

Backend retirement is a sibling program, not another semantic pipeline. Only
CoreDirect and force-hv1 bypass closure precede Wpre. Broad/default Rust route
retirement, current vm-hako retirement, independent AOT HMI construction, and
Rust `MirInterpreter` physical deletion follow Call R7 and post-Call integration
in the order owned by `vm-active-lane-retirement-ssot.md`.

## Call completion and retirement program (2026-09-03)

This is the authoritative Call projection of the global pipeline above. It is
not a second MirBuilder, a second task ledger, or permission to recreate a
landed family. Historical `MS1-M` maps to `MS1-P`; historical `MS1-B` work is
split between the selected consumer gate and later backend migration. The
three completion levels are deliberately separate:

```text
MS1-P  producer / publication core
       every production source family issues one mandatory typed target or a
       named pre-effect rejection; target selection precedes arguments; the
       module is published atomically; semantic recovery/fallback/retry are 0

MS1-C  selected consumer and compatibility stop
       each selected product family consumes a borrowed typed publication or
       stops before effect/artifact; legacy readers are either stopped or
       explicit outer compatibility and cannot be entered after canonical
       admission. Backend feature parity is not required.

MS1-D  physical retirement
       after caller-zero, old Call schema, compatibility readers/reissuers,
       family-only tests/guards/docs, and disconnected Builder surfaces are 0
```

The critical finish path is fixed. Backend feature migration is a post-R6
sibling and cannot block the legacy-reader stop path:

```text
closed M0--M3 census/disposition/quarantine
  -> M4 MIR-CALL-MANDATORY-CALLEE-R6
  -> M7-S MIR-CALL-LEGACY-READER-STOP-R0
  -> M7 MIR-CALL-COMPATIBILITY-RETIRE-R7
  -> M8 MIRBUILDER-PHYSICAL-THINNING-R0
  -> M9 MIRBACKEND-LEGACY-RETIRE-R0

post-R6 optional migration, only with an exact tuple:
  M5 MIR-CALL-HAKO-PUBLISHED-VIEW-INGRESS-I0
  -> M6 MIR-CALL-BACKEND-FAMILY-CUTOVER-R0
```

`M7-S` may stop an unsupported legacy product reader without first providing
feature parity. In particular, stopping/quarantining the Rust WASM
`LegacyCallV0` reader is an R7 prerequisite; implementing Hako WASM W0 is not.
Likewise, backend `UnsupportedBeforeArtifact` does not invalidate canonical
MIR and must not reopen source semantics.

`NoSafeSlice` is evaluated per `(family, profile, reader boundary)`. The
current aggregate token means that there is no eligible R6 `Promote` family;
it does not prohibit an already-inventoried M7-S `Stop` cohort. The following
words are scheduling projections of the existing dispositions, not new
semantic states or a second task ledger:

```text
Promote -> Canonical through M4 when the exact reopen tuple exists
Stop    -> M7-S ExplicitUnsupportedBeforeArtifact or unreachable outer ingress
Delete  -> M7/M8/M9 only after caller-zero
Park    -> ParkedSealed until its observable reopen trigger fires
```

Parking one family never promotes another family and never grants a broad
schema cutover. It only returns selection to another already-inventoried
family/profile boundary.

## S-class completion gates (post-M9, non-executable navigation)

The M0--M9 order is the MirBuilder/product completion program. A stronger
release claim is allowed only after these five independent gates are observed;
they add no alternate pipeline and do not authorize work in the current R6
slice.

```text
S1 Enforce
   the critical Resolve→...→Atomic Publish boundaries have private
   constructors/capabilities or equivalent negative guards; an invalid
   pre-publish or backend-repair transition is not representable through the
   production API.

S2 Prove
   VM/reference and LLVM/AOT compare observable results, failures, and side
   effect order for the selected language corpus; optimized MIR is checked
   against the same witness.

S3 Delete
   LegacyCallV0 has zero production writers, reissuers, and readers; the
   compatibility ingress is explicit and caller-zero before physical removal.

S4 Bootstrap
   fixed stage0→stage1→stage2 selfhost output is reproducible and its
   semantic/identity comparison is machine-checked.

S5 Release
   a clean checkout with pinned tools builds and runs the documented sample
   ladder, records limitations, and publishes regression evidence for a
   third-party user.
```

These are completion gates, not current implementation rows. The active
`CURRENT_STATE.toml` and the linked owner documents remain the only execution
authority.

After M0 and before the next semantic implementation family, the independent
`DOCS-HISTORY-RETIRE-R1` repayment may remove its four pre-classified closed
cards. It is skipped on any census drift and does not block M1.

### Closed census and current disposition

```text
status = closeout
implementation permission = false
current result = NoSafeSlice__NoSingleRemainingCanonicalReaderFamily
```

M1/M2 and the post-Group-B census are complete. Their boundary is finite:

```text
start:
  every production-reachable MirInstruction::Call writer/reissuer
  and target-bearing construction owner

end:
  Atomic Publish plus every VM/WASM/LLVM/AOT/Hako/selected-C/JSON/object reader,
  compatibility repair/retry terminal, and artifact admission decision

includes:
  source/package issuers, structural readers, compatibility outer ingress,
  selected product readers, typed unsupported terminals, old-edge ownership

excludes:
  parser grammar, unrelated MemOp, test-only occurrence counts as production
  callers, backend feature parity, and repository-wide cleanup
```

Raw grep counts are diagnostic only. A structural visitor, test fixture, or
serializer occurrence is not an independent semantic producer.

The sole current-state vocabulary is:

| disposition | meaning | permitted action |
| --- | --- | --- |
| `Canonical` | exact source authority and mandatory typed target already exist | retain; mechanical schema adaptation only |
| `CompatibilityOuterIngress` | an explicit legacy/wire/reference boundary still owns the shape | stop or quarantine outside the canonical product path |
| `ExplicitUnsupported` | the selected profile intentionally cannot consume the shape losslessly | typed reject before effect/artifact; no fallback |
| `ParkedSealed` | zero/multiple owner, consumer, caller, or delete-set | no new D0/receipt/adapter/fixture/guard; reopen only by the exact trigger |

Historical `DeadDeleteCandidate` is not a fifth semantic disposition. A
caller-zero private asset stays `ParkedSealed` until its deletion evidence is
complete, then is removed in the owning family series.

### Finite family map

| family / boundary | disposition | authority / reason |
| --- | --- | --- |
| Global StaticBoxMethod, FreeStatic, FreeFunction, Builtin Print | `Canonical` | source/package key, mandatory typed callee, and Atomic Publish relation landed |
| root-lexical SameModuleInstance semantic issuance | `Canonical` | exact InstanceBoxMethod key plus mandatory receiver; backend coverage is separate |
| claimed Birth and direct NewBox/NewClosure construction | `Canonical` | construction issuer is typed; legacy Call-carried constructor/closure readers are a separate parked boundary |
| builder/PHI/SSA/verifier/printer call visitors | `Canonical` structural | copy or inspect an issued callee; never select a target |
| unified-OFF, `emit_legacy_call`, and each unclaimed ordinary-new fallback ingress | `CompatibilityOuterIngress` | individual legacy value/name ingress surfaces; cannot re-enter after canonical admission |
| each JoinIR, MIR-JSON v0/v1, and Program-JSON/selfhost JSON ingress | `CompatibilityOuterIngress` | individual wire compatibility surfaces, not source semantic authority |
| runtime string Method/Extern helpers and repair corridors | `CompatibilityOuterIngress` | names, headers, registries, `args[0]`, and backend success are non-authority |
| selected-C arbitrary UserBox and selected indirect Value/Closure | `ExplicitUnsupported` | no lossless selected-product consumer; reject before object |
| VM canonical non-Print target | `ExplicitUnsupported` | Print is the only landed canonical VM reader family |
| explicit/nested/upvar instance and deferred shadow profile | `ParkedSealed` | no single source/issuer/caller/delete-set tuple |
| ordinary-new multi-writer migration family and combined JoinIR/JSON migration family | `ParkedSealed` | aggregate cutover family has multiple outer owners; this does not reclassify each ingress surface above |
| Hako SameModuleInstance ingress and mixed LLVM/Hako/selected-C consumer | `ParkedSealed` | no borrow-only lossless Hako consumer/caller |
| generic CorePlan GlobalCall, physical normal-main thunk, Call-carried Constructor/Closure residual | `ParkedSealed` | missing sole issuer or exclusive delete-set |
| test-only PHI legacy candidate | `ParkedSealed` pending deletion evidence | private candidate; not a Call-schema completion shortcut |

A family may have canonical semantic issuance and an unsupported or parked
backend profile at the same time. That is not a conflict: backend capability
does not flow backward into source validity.

### Backend/profile disposition

| profile | canonical cohort | disposition |
| --- | --- | --- |
| selected-C typed published view | StaticBoxMethod, FreeFunction, Builtin Print | `Canonical` typed consumer |
| selected-C | arbitrary UserBox SameModuleInstance | `ExplicitUnsupported` before object |
| VM reference | Builtin Print | `Canonical` typed consumer |
| VM reference | other canonical calls | `ExplicitUnsupported`; legacy VM arm remains outer compatibility |
| Rust WASM | legacy reader | `CompatibilityOuterIngress`; stop/quarantine before R7 |
| Hako LLVM-text | current RecipeFacts/JSON/name routes | `CompatibilityOuterIngress`; no semantic authority |
| Hako published view | SameModuleInstance and general module ingress | `ParkedSealed` until one lossless caller exists |
| JSON canonical egress | typed structural display/transport | `Canonical` structural consumer |
| MIR/Program JSON ingress | legacy carrier production | `CompatibilityOuterIngress`; stop before R7 or keep outside product |

The current selected-C runner still has a dirty automatic transition when no
typed row is selected. A canonical call family must become either typed
consumption or `UnsupportedBeforeArtifact`; it must not fall through to the
JSON selected-C route. Zero-call physical admission is a separate concern.

### Exact reopen trigger

No further broad census is permitted. One existing family may reopen only when
all of the following are named at once:

```text
exactly one existing source authority
+ exactly one canonical typed issuer before argument descent
+ exactly one lossless selected-product/publication consumer
+ exactly one real production caller
+ one named fail-fast typed reject boundary
+ one finite family-exclusive old-edge delete set
+ every compatibility reissuer/reader outside the selected route enumerated
+ every touched or new owner below the 760-line source trigger
+ canonical rejection re-entry/fallback/retry/reselection = 0
+ existing focused positive/negative and lane-guard owners named
+ migration red classified separately from the known-red baseline
```

Ordinary `FunctionCall` additionally requires one-traversal observer-only
site/name/arity/argument facts and complete pre-install disposition. If the
tuple is zero or multiple, the family remains `ParkedSealed`; do not create a
new semantic `Verified*`/`Prepared*` product or a temporary fallback.

### Executable task contracts

The following are one dependency program, not simultaneously active cards.

#### M4 — `MIR-CALL-MANDATORY-CALLEE-R6`

Open only for one family satisfying the exact trigger. The bounded series is:

```text
take existing source/package target
-> own mandatory typed Callee before argument descent
-> lower arguments once
-> publish canonical Call(MirCall)
-> selected typed consume or UnsupportedBeforeArtifact
-> switch the named production caller
-> delete the selected semantic repair/re-entry edge
-> prove fallback/retry = 0
```

No `CallV2`, second Builder, new resolver, Global disguise for instance
methods, optional receiver, `args[0]` repair, name lookup, or backend retry is
allowed. Group A's instruction-shape split and Group B's VM canonical Print
reader are closed tombstones; they are not reopened.

#### M7-S — `MIR-CALL-LEGACY-READER-STOP-R0`

After the R6 canonical core checkpoint, stop each legacy product reader by one
of exactly three outcomes:

```text
TypedConsumer
ExplicitUnsupportedBeforeArtifact
ExplicitCompatibilityOuterIngress unreachable from product selection
```

The initial stop inventory is:

1. canonical-to-selected-C JSON automatic fallback;
2. VM `LegacyCallV0` product reader;
3. Rust WASM `LegacyCallV0` product reader;
4. Hako/LLVM legacy product reader;
5. MIR/Program JSON ingress that still produces a product-reachable legacy
   carrier.

Each stop is caller-by-caller and must name its delete-set. It does not require
feature parity or a replacement backend. A stopped unsupported profile is a
valid result. A compatibility route may remain only as an explicit outer
entrypoint and must never be selected after canonical failure. If that outer
entrypoint still has a live production caller, R7 remains closed until the
caller is removed or migrated.

##### Landed cohort — `MIR-CALL-WASM-LEGACY-GLOBAL-READER-STOP-R0`

```text
status = landed
implementation permission = false
implementation commit = 833eb87a80
```

This is the first bounded M7-S `Stop` candidate. It changes no source meaning
and adds no canonical WASM consumer. Its boundary is:

```text
start:
  Rust WASM observes MirInstruction::LegacyCallV0 with
  callee = Some(Callee::Global(_))

end:
  the shared WASM preflight rejects with
  [freeze:contract][wasm/legacy-global-call-stopped]
  before shape matching, WAT planning, binary emission, output creation,
  or selection of a Rust fallback route
```

Source authority and the canonical issuer remain the existing Resolver,
source/package products, mandatory typed `MirCall`, and Atomic Publish. The
WASM backend is only a consumer or typed reject owner. Function-name maps,
physical symbols, arity tables, zero padding, shape matches, route labels,
JSON, successful validation, and fallback success are non-authority.

The finite delete set is:

1. the `LegacyCallV0(Global)` lowering arm in
   `src/backend/wasm/codegen/instructions.rs`;
2. its name-based reachable-function traversal in
   `src/backend/wasm/codegen/mod.rs`;
3. the Global-only name/parameter-count/return-shape helpers and missing-arg
   `i32.const 0` repair left caller-zero by items 1 and 2;
4. the direct legacy-Global negative proof in
   `src/backend/wasm/tests.rs`, which exercises the typed pre-artifact
   rejection without deleting or rewriting an existing fixture. That old
   success probe is handled by the selected cleanup row below.

The acceptance boundary includes every Rust WASM caller through the shared
preflight; bypass or retry keeps the row closed.

Focused evidence reuses the call-free positives and direct
`legacy_global_call_rejects_before_wasm_codegen` test; no test, fixture,
receipt, adapter, route, or guard was added.

Implementation evidence: shared preflight and the old Global lowering,
reachability, signature-map, and zero-padding readers are gone; the recorded
check and 18-test WASM slice passed. Known red is not relabeled.

Owners remain below the 760-line trigger. No canonical WASM reader, Hako W0,
Extern/Method retirement, general fallback removal, R7 caller-zero, or
`LegacyCallV0` deletion is claimed.

##### Selected cleanup — `MIR-CALL-WASM-GLOBAL-PROBE-RETIRE-R0`

```text
status = landed
implementation permission = false  # implementation c9c62906b1
```

Retarget the stale WSM-G4-min8 success probe to the existing canonical
user-method fixture's explicit pre-WAT rejection. Update only its test marker,
smoke expectation, and historical lock; keep the direct Legacy Global negative
proof as the semantic reader-stop evidence. No new fixture, receipt, adapter,
or guard is allowed.

Acceptance: stale success expectation is zero; focused test and smoke retarget passed;
non-Global WASM families remain unchanged.

##### Landed cohort — `MIR-CALL-VM-GLOBAL-CANONICAL-CUTOVER-R0`

```text
status = landed
base_head = 36c7c15d87
implementation permission = false
implementation commit = 111216b539
focused evidence = 5 canonical dispatch + 1 legacy reject + 8 parameter + 9 return tests
```

Decision: extend the existing VM `MirInstruction::Call(MirCall)` Global
consumer from Builtin Print to the already-issued same-module FreeFunction and
StaticBoxMethod targets, while rejecting `LegacyCallV0(Global)` at the shared
call ingress. The VM remains a consumer; no second issuer or semantic receipt
was added.

Source authority + canonical issuer: existing Resolver/source products,
`CanonicalGlobalTargetV1`, mandatory `MirCall`, and Atomic Publish. Non-authority
is function-name lookup, physical symbols, `func`, arity completion, zero
padding, `ValueId(0)`, JSON, registry, `args[0]`, or fallback/retry.

Fail-fast boundary: `handlers/calls::handle_call` rejects legacy Global before
trace/dispatch/`execute_callee_call`; canonical Global calls go directly to
`execute_global_target`, which checks exact arity and the module function table.

Finite implementation/delete set: extend `handlers/mod.rs` canonical Global
admission; keep `handlers/calls/global.rs` as the sole canonical target
consumer; remove the VM legacy Global dispatch arm; convert the four existing
parameter/return contract fixtures to `MirInstruction::call`; retain
`execute_global_target` and its exact FreeFunction/StaticBoxMethod branches.

Acceptance: canonical FreeFunction and StaticBoxMethod VM calls execute;
legacy Global rejects before old dispatch; the four fixtures are green; the
legacy Global arm is zero. The focused slice passed with no new red classification;
existing active-surface and pointer guards are reused; both passed. No Call R6 schema deletion,
Method/Extern/Value/Constructor migration, JSON/Hako/LLVM work,
VM-Hako promotion, or R7-wide retirement is claimed. All touched owners remain
below the 760-line split trigger.

#### M7 — `MIR-CALL-COMPATIBILITY-RETIRE-R7`

Open only when every production legacy writer, reissuer, and reader is
caller-zero. Then remove in one isolated migration series:

```text
LegacyCallV0
Call.func and legacy target Const
callee=None
receiverless / name-based Method(None)
ValueId(0) missing sentinel
args[0] receiver inference and duplicate-strip repair
header/registry/name target reconstruction
typed failure -> JSON/backend fallback or retry
```

Compiler errors may expose mechanical readers, but are not the sole inventory.
Serializer/deserializer, verifier, optimizer, printer, SSA rewrite, C shims,
VM/WASM/Hako/LLVM readers, and public compatibility entrypoints must all be
accounted for before deletion.

#### M5/M6 — post-R6 backend migration

Hako published-view ingress and backend family replacement are optional
post-R6 migrations. They use the same published identity and borrow-only view;
they do not issue source meaning. They may run before or after R7 only if their
input no longer requires the deleted legacy carrier.

For WASM specifically:

```text
pre-R7 requirement:
  stop or quarantine the Rust LegacyCallV0 reader

not an R7 requirement:
  implement WASM-HAKO-W0-PUBLISHED-MIR-INGRESS-I0

post-R6 migration:
  W0 lossless Hako ingress -> W1 scalar FreeFunction -> W2 default cutover
  -> W3 caller-zero Rust codegen retirement
```

#### M8/M9 — physical thinning and backend retirement

After R7, remove builder barrel registrations, raw ports, `variable_map`
bypasses, stale wrappers, disconnected proof modules, and retired backend
consumers leaf-first. A file move or tracked archive copy receives zero
reduction credit. Each family deletes its private tests/guards/docs with the
old edge; durable history remains in Git.

### Evidence reuse and repayment

Do not add an R6/R7-specific guard. Reuse the existing lifecycle, canonical
corridor, Global-target, pointer, and diff guards. Canonical structure/read
coverage stays in the existing instruction, printer, ownership-SSA, compiler
Call, VM/WASM, JSON parser, and published-backend-view test owners.

For each family cutover:

1. keep canonical positive/negative tests;
2. quarantine compatibility tests while a public compatibility caller lives;
3. delete caller-zero private tests, fixtures, adapters, and guards;
4. retire an old guard only when an equal-or-stronger lane guard owns the rule;
5. compact closed docs to hash plus one-line outcome; do not copy an archive
   into the tracked tree;
6. record tracked files/lines and old-edge delta before choosing the next
   family.

The known-red baseline is separate from migration red. Every changed test is
named; unclassified new red blocks the cutover.

### Closed history tombstones

```text
M1/M2:
  finite producer/consumer census and family disposition closed on 2026-09-03;
  no unclassified production owner was found inside the stated boundary.

M3-A:
  canonical UnifiedCallEmitter core has no env read or legacy emission call;
  outer compatibility remains explicit.

M3-B/M3-C:
  ordinary-new and JoinIR/JSON multi-owner families are ParkedSealed; no new
  adapter or receipt was admitted.

R6 Group A:
  canonical Call(MirCall) and LegacyCallV0 are separate instruction shapes.

R6 Group B:
  VM consumes canonical Builtin(Print); wrong arity/non-Print fail closed.

MIR-CALL-R6-POST-GROUP-B-READER-CENSUS-C0:
  NoSafeSlice__NoSingleRemainingCanonicalReaderFamily.
```

The exact landed commits and command receipts remain in Git and
`CURRENT_STATE.toml`; they are not duplicated here. Source files at or above
760 lines must be split behavior-neutrally before semantic growth; 800 lines is
a hard stop.

## Authority map

| Boundary | Owns | Must not own |
| --- | --- | --- |
| Resolver | `BindingId` / `ScopeId` / `RegionId` / callable target / source provenance | MIR emission、route retry |
| Observation / Facts | sourceとcontrol-flowの観測結果 | MIR mutation、hidden acceptance policy |
| RoutePolicy / Recipe | 一度だけ行うroute選択とlowering義務 | real `ValueId` / `BasicBlockId`、publication |
| RecipeVerifier | omission、duplicate、coverage、exit、carrier、merge契約 | repair、別Recipeへのfallback |
| Verified-plan Lowering | CFG、operand、Binding SSA、edge、PHI materialization | ASTからのroute再判定、別ownerへのretry |
| Function-local Finish | CFG / semantic / If / Binding SSA / PHI / resolved-binding / Completion の全closeと `ReadyFunctionDraftSealV1` 発行 | profile選択、Return書込み、draft publication |
| FunctionDraftSeal | exit、PHI closure、type/signature/metadata、session closeのprepareとcommit | Recipe再解析、source route選択 |
| Draft Collector | `CompletedFunctionDraftV1`の完全集合 | open/prepared draftの公開 |
| Module transaction | candidate moduleのsuccess-only atomic publication | partial insertion、failure後のresume |

Facts、Recipe、Verifierの詳細契約は
`recipe-first-entry-contract-ssot.md`と`recipe-tree-and-parts-ssot.md`が持つ。
Function exit semanticsは`docs/reference/language/function-exit-and-entry-result.md`
が持つ。DraftSeal、collector、module publicationの現行実装とaccepted
evidenceはRelatedに列挙したsource owner／taskにある。この文書はそれらを
結ぶ最終pipeline authorityを所有する。

## Loop specialization navigation

この文書は全compilerの順序だけを所有する。Loop固有の再帰
Facts/Recipe/JoinSig/Verify/Lower順序は
`joinir-loop-selfhost-recipe-pipeline-ssot.md`、post-Recipeのphysical
demand/session境界は`loop-common-physical-demand-and-session-ssot.md`が
所有する。現在実行中のbounded profileとexact rowは
`CURRENT_STATE.toml`の`current_execution_design`へ辿り、ここへ複製しない。

### Delegated Loop boundary

Loop-specific selected initializer, parameter/result carrier, JoinSig,
physical-demand/session, A-prime, and common-physicalizer rules are owned only
by:

- `joinir-loop-selfhost-recipe-pipeline-ssot.md`
- `loop-common-physical-demand-and-session-ssot.md`
- the exact active Loop card selected by `CURRENT_STATE.toml`

This global pipeline retains only the ordering law:

```text
source/package semantic program
-> verified Recipe and JoinSig
-> Builder-free physical demand
-> session-local ValueId/BasicBlockId realization
-> DraftSeal
-> Atomic Publish
```

Loop lowering may not reconstruct Recipe transfer, ABI, result, continuation,
or source membership from MIR, names, ordering, metadata, or backend success.
Rust VM is not a production capability gate. A Loop family with no sole
source authority, consumer, caller, and old-edge delete-set is
`ParkedSealed`; it does not reopen the Call R6/R7 program or justify a second
planner, adapter, receipt, or fallback.

## Non-negotiable laws

### 1. Meaning is decided once

source shape、route、Recipe、exit／merge義務はVerifierより前で決める。
Verifierを通過した後に、LowerまたはDraftSealがASTを読み直して別の意味を
選んではならない。

```text
forbidden:
  Recipe
    -> Lower
    -> DraftSeal reclassification
    -> another Recipe / Legacy fallback
```

### 2. Lower consumes verified products

Lowerは`VerifiedRecipe`、verified `CorePlan`、または同じ責務を持つverified
lowering productだけを受け取る。名称が将来`LoweredRecipe`などへ縮退しても、
未検証入力をLowerが再判定しない契約は変えない。

### 3. Seal completes; it does not plan

canonical function pathでは、Body Loweringはexit operandとexact exit blockを
準備する。physical `Return`の唯一writerはDraftSealのdetached prepare projection
であり、`PreparedFunctionDraftSealV1::commit(self)`は検証済みprojectionをmove
するownership-only terminalである。

multiple source Returnでもこのownerは増えない。
`VerifiedFunctionCompletionV1::ExplicitReturns`がdeclared result分類とexact
ordered sitesのsole semantic ownerである。そのborrowed exact-result projection
から一方向に得たABI、各siteの`BindingRef` operandを
一つのmove-only setへco-sealし、既存Completion consumptionがsite-keyedな
physical claimをexactly onceで閉じる。DraftSeal prepareはdetached projectionの
各claimed exit blockへ一つのReturnを書き、全検証を完了する。commit後のfallible
workは0で、profile lowererはReturnを書かない。単に複数exitを一つへ集めるため
だけのsynthetic return-join/PHIは作らない。backend/MIR制約が別のverified owner
として要求した場合だけ、独立Decisionで開く。

`CanonicalSsaFunctionSessionV2`経路における`ReadyFunctionDraftSealV1`の
issuerは、target `finish_for_draft_seal`だけに集約する。各V2 profile
lowererがCFG／SSA／PHI／Completionのfinish順を手作業で複製して直接
`ReadyFunctionDraftSealV1::new`を呼ぶ形はreplacement debtである。非V2の
既存direct constructor callerもcompat debtとして増加禁止にし、最終退役で
production callerを0にする。
profile固有ledgerは先にprivate close receiptへ畳み、common finish terminalが
そのreceiptと全function-local ownerをconsumeして初めてReadyを発行する。

The current R0 audit is intentionally bounded: the V2 session has three
existing profile constructors (`trivial_ssa`, `direct_accum`, and
`nested_predicate`), while one non-V2 `CanonicalFunctionLowererV1` direct
constructor remains an explicit compatibility allowlist entry. R0 migrates
the three V2 paths only. A move-only profile-close receipt and sealed function
identity prevent terminal re-inference of body/site/target/current-block
facts. The guard contract is mechanical: V2 direct Ready-constructor callers
must be zero, the non-V2 allowlist must not grow, and every V2 finish order is
owned by the one terminal API. Physical Loop lowering, production selection,
retry/fallback retirement, and legacy deletion are later rows.

すべてのfallibleなexit、PHI、type、signature、metadata、verification、
session-close準備を`prepare`で終える。`commit(self)`はownership-onlyの
infallible terminalとする。

この契約へ未移行のproduction Return writerは互換完成形ではなく、
replacement debtである。

### 4. Publication is all-or-nothing

collectorが受理してよいのは`CompletedFunctionDraftV1`だけである。全draftが
揃う前にlive moduleへfunctionを直接挿入しない。

```text
success:
  completed drafts -> candidate module -> atomic publish

failure:
  discard candidate module and unpublished drafts
```

### 5. Authority never flows backward

後段は前段の決定をconsumeするだけである。

```text
Seal      -> Recipe      forbidden
Lower     -> RoutePolicy forbidden
Collector -> Lower       forbidden
Publish   -> retry       forbidden
```

## Responsibility diagram, not a file quota

上の箱は責務境界であり、各箱に専用Rust file、type、trait、guardを一つずつ
作る要求ではない。

- 一つのtypeが隣接する機械的段階を安全に表してよい。
- Plan LoweringとBody Loweringは実装上interleaveしてよい。
- ただしsemantic authorityの向きは逆流させない。
- 新しいwrapperやproof fileを作ること自体を進捗に数えない。

## JoinIR naming boundary

`JoinIR`という名前は、現在のrepositoryでactiveなBuilder
Recipe/CorePlan系とlegacy JoinModule系の両方に使われた履歴がある。

このSSOTでは、次の責務名を使う。

```text
Control-flow Observation:
  StepTree / ControlForm / CondBlockView / Loop / If / ExitLine observation

Verified-plan Lowering:
  verified Recipe/CorePlanからCFG / merge / carrier / Binding SSAをmaterialize
```

legacy JoinModuleを第二planner、第二acceptance truth、または最終pipelineの
別routeとして復活させない。

## Replacement-cell admission rule

新しいreplacement cellは、実装前に次へ答えなければならない。

```text
1. north-starのどの責務／edgeを前進させるか
2. structural / production-reachable / test-only / public-contractの各countは何か
3. censusのstart -> end、includes、excludesは何か
4. named existing production callerはどれか
5. selected new ownerはどれか
6. 同じcommitで削除するold authorityはどれか
7. cutover後のfallback / retry / reselectionが0か
```

`structural_sites > 0`を`production_reachable_callers > 0`へ読み替えない。
production reachが0ならreplacement I0ではなくcaller-zero reconciliation／RET0、
public contractが残るなら明示Decisionを選ぶ。test-only direct injectionは
production acceptanceではない。

最初のproduction replacement rowは
`H2-SELECTED-DYNAMIC-LOOP-CUTOVER-I0`である。
`MIRBUILDER-FIRST-PRODUCTION-CUTOVER`はそのrowが満たすmilestone名であって、
第二のswitch taskや別authorityではない。成功時は同じcellでselected legacy
Loop edgeを削除し、fallback/retryを0にする。

次はreplacement cellとして数えない。

```text
production caller = 0 のproof-only owner
old authorityを削除しないadapter追加
別production routeの建設
LOCだけを減らしauthority graphを変えない移動
```

判断は常に次の一問へ戻す。

> この変更は競合するauthorityを一つ消し、production経路を
> `Facts -> Recipe -> Verify -> Lower -> Seal -> Publish`
> へ近づけるか。

Noなら、cell数やLOCが良く見えても選択しない。

## Convergence credit and proof compression

MirBuilder移行の進捗単位は、新しい型・receipt・module・guardの数ではない。
次のproduction graph deltaだけを収束creditとして数える。

```text
named production caller switches to the selected owner
  + selected old edge becomes zero
  + fallback / retry / reselection become zero
```

source/Facts/Recipe/Joinの中間productは、この切替を安全にする証拠であり、
それ自体をfinish lineにしない。selected-normal、canonical、raw、compatibilityが
同じsource formを物理化できる期間はmigration windowである。新ownerのgreenだけで
閉じず、canonical consumer切替と旧consumer退役までを同じordered familyに置く。

専用のmove-only productを恒久化してよい境界は次に限定する。

```text
source authority or semantic ownership changes
physical effect becomes possible
candidate becomes commit/publication eligible
publication authority changes
lifecycle closes
```

同じtransaction内のread-only validation段階、単なるfield projection、テスト専用
canaryはprivate stateへ畳み込めるかを先に検討する。canary／migration receiptを残す
場合は、named production consumer、current caller count、`retire_when`を必須にする。
cutover後はcaller-zeroを確認してpromote、quarantine、またはretireの一つへ閉じる。

`FunctionMetadata`のfact／plan／seedも同じ規律に従う。familyごとに最低限、

```text
production producer owner/count
production consumer owner/count
backend consumer owner/count
last verified consumption revision
retire_when
```

を一つの機械可読inventoryで観測する。`producer > 0`かつproduction/backend
consumerがともに0のrowは性能成果ではなく、caller-zero migration debtである。
新しいbackend consumerはperf owner-first attributionが選んだ一familyだけに追加し、
inventoryだけを理由にoptimizerやfast pathを増やさない。

type authorityも逆流させない。

```text
semantic type
  -> verified physical representation
  -> verified ABI passing class
  -> verified storage layout
  -> one backend physical-type input
```

backendは`MirType`、metadata、ABI manifest、storage planを独立に再結合しない。
この四層を一つのbackend inputへ閉じる設計は
`value-repr-and-abi-manifest-ssot.md`が所有し、source semanticsやABIを同時に
変更するcleanup rowへ混ぜない。

## Completion authority

`MIRBUILDER-FINAL-PIPELINE-v1`全体は、次がproduction graphで成立した
ときに着地する。MirBuilder core単体のMS1-P境界は上のCall completion
programが所有し、backend coverageと物理retirementをcoreの再設計条件へ
逆流させない。

```text
accepted production source families enter one authority pipeline = all
normal/default runner enters one typed canonical source ingress   = yes
normal/default route-selection authority                          = 1
Legacy compile_with_source* production callers                    = 0
family-specific canonical entrypoints as competing prod fronts    = 0
Facts / Recipe / Verify decision authority                       = one each
unverified direct lower                                           = 0
Lower-side AST route redecision                                   = 0
DraftSeal-side Recipe / route redecision                          = 0
physical Return writer on canonical function path                 = 1
CompletedFunctionDraft-only collection                            = yes
partial module publication                                        = 0
fallback / retry / profile reselection                            = 0
canonical rejection -> Legacy retry/fallback                      = 0
selected old production owner / facade / edge                     = 0
full accepted corpus / backend parity                             = green
```

pack counters、replacement ledger、five-cell LOC、source/test measurements
は、このsemantic completionへ到達する過程の観測値である。増減だけで
implementation permissionやcompletionを決めない。

## Final repository convergence finish line

`MIRBUILDER-FINAL-PIPELINE-v1` の完了は Loop の production cutover だけで
終わらない。次の直列順を最終 finish line として固定する。

```text
CANONICAL-FUNCTION-FINISH-TERMINAL-R0
  -> LOOP-PHYSICAL-PREPARE-DESIGN-CORRECTION-R0
  -> caller-zero LOOP-PHYSICAL-PREPARE-P0
  -> Generic G0 prepare parity
  -> common physicalizer / caller-zero canary
  -> production selection
  -> M8/M9 coverage and parity
  -> M10b activation
  -> M11/M12 legacy retirement
  -> REPO-FINAL-CONVERGENCE-AUDIT0-G0
  -> repo-physical-structure-cleanup-ssot.md final convergence acceptance
```

最後の cleanup では、pipeline SSOT の一本化、`src/mir` root facade の
durable-only 化、Rust/.hako/compat authority の分類、Context の owner 分離、
`CURRENT_STATE` と設計 registry の収束、temporary proof/receipt/adapter の
promote/quarantine/retire、旧 D4/S-series ledger の archive 化まで確認する。
cleanup は Loop cutover 前に開かず、各実装 row は owning README/reference、
guard index、current mirror を同じ commit で更新する。詳細な row と stop
条件は上記 cleanup SSOT にのみ置き、ここに第二の task ledger は作らない。

## Explicit non-goals

```text
one box = one file / type / trait
new language semantics
new runtime or backend policy
independent second MirBuilder
legacy JoinModule revival
metric-derived architecture
DraftSealでのsource re-analysis
```
