---
Status: SSOT
Date: 2026-09-05
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
- **Current implementation status:** canonical `Call(MirCall)` is separated
  from explicit `LegacyCallV0`; the shared Rust MIR JSON-v0 call ingress Stop
  is landed, and the M7-S owner-local Stop/Delete queue remains active.
  Stage1 writer removal is landed but its acceptance is parked; six invalid
  Stage1 smoke/fixture assets are the next physical repayment.
- **Latest bounded work:** fixed baseline `7555/7393/133/29`; commit
  `9a40ece824` stops both JSON-v0 call spellings before publication, removes
  the call/catalog owners and call-only tests, and preserves non-call loaders.
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

The sole semantic order is `Resolve -> Observe -> Facts -> Recipe -> Verify ->
Lower -> Seal -> Collect -> Atomic Publish`. Call completion is its current
projection, not a second roadmap:

```text
M4 mandatory Callee -> M7-S owner-local Stop/Delete -> M7 caller-zero schema
retirement -> M8 physical thinning -> M9 backend retirement
```

Backend migration is a post-R6 sibling. It cannot reissue source meaning or
delay stopping an unsupported legacy reader; its detailed order remains in
`vm-active-lane-retirement-ssot.md`.

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

`NoSafeSlice` is evaluated per `(family, profile, reader boundary)`. Scheduling
has two independent axes; neither is a new semantic state or task ledger:

```text
verification_health:
  Green | StableKnownRed | UnclassifiedRed

family_action:
Promote -> Canonical through M4 when the exact reopen tuple exists
Stop    -> M7-S ExplicitUnsupportedBeforeArtifact or unreachable outer ingress
Delete  -> M7/M8/M9 only after caller-zero
Park    -> ParkedSealed until its observable reopen trigger fires
T0      -> inventoried compiler-proven hygiene; no new authority/guard/receipt; src/ delta <= 0
```
Only `UnclassifiedRed` stops all semantic/cleanup work. A parked family returns
selection to another inventoried family; it never grants a broad cutover. A
closed census is evidence, not an active waiting row. Priority is
`verification recovery > Delete > Stop > Promote > required 760-line split >
T0`; T0 is never selected twice consecutively while a Delete/Stop exists.
Census-only commits are forbidden; the same series must Promote, Stop, or
Delete. Progress is the production old-edge delta, not row/guard/test count.

## S-class completion gates (post-M9, non-executable navigation)

The Call/M0--M9 order completes the Call program; the Loop handoff below is
also required for whole-MirBuilder completion. A stronger
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

#### V0 — reuse `DEV-GATE-LIB-BASELINE-REFRESH-R0` in reconcile mode

Closed tombstone: `917a078c6c` reconciled the five existing repair cohorts
without a new health row/guard/receipt; `c2681307dd` fixed the successor
baseline authority. The checked-in manifest and `CURRENT_STATE.toml` now own
`7555/7393/133/29` and the fixed failure-name set. Earlier observations and
per-cohort detail remain in Git. This closed incident authorizes no semantic,
BoxShape, performance, or test-retirement work.

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
status = landed
implementation permission = false
current cohort = `array_element_write_published_c_cutover_i0`

The selected cohort replaces the selected-C legacy projection for the existing
canonical `ArrayElementWrite`; it does not create a new semantic issuer.
`ArraySurface` plus `ArrayElementWriteOwner` remain authority, while one
borrowed `PublishedMirBackendView` row family carries exact site, kind,
receiver, optional index, value, destination shape, and WRITE effect to C.

After the R6 canonical core checkpoint, every compatibility boundary has one
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

Each cohort names one owner, terminal, caller, finite delete-set, and focused
acceptance. Acceptance executes the selected owner-to-terminal boundary; an
earlier terminal is dependency evidence, not acceptance, and never reopens a
downstream deletion. Feature parity is not a Stop prerequisite. The shared
guard owns only the parent/cohort token; source tests own semantics. Add no
per-cohort card, dispatcher, receipt, adapter, fixture file, or guard.

##### Closed M7-S tombstones

```text
f15098cf0b — Stage1 Return(Call) writer-repayment assets: six exclusive
probe files / 189 lines and stale discovery paths retired; no guard, receipt,
test, ignore, or baseline change.
9a40ece824 — shared MIR JSON-v0 op=call/op=mir_call stopped before
LegacyCallV0 publication; call.rs/catalog.rs and call-only tests retired;
14 focused tests passed and non-call loaders remained.
a33987e8e4 / 24ece062bb / 01a1a6bc83 — canonical Value, METHODIZE, and
singleton compatibility stops landed; fixed comparator and failure SHA stayed
unchanged. `99b4446cab` retired the Stage1 writer; its boundary remains parked
because the current import closure cannot reach `FuncLoweringBox`.
```

##### Finite reduction queue (worker-audited 2026-09-04)

| order | cohort / action | exact boundary and delete-set | acceptance / reopen |
| --- | --- | --- | --- |
| 1 landed | `direct_mir_json_duplicate_reader_delete` / Delete | `runner/dispatch.rs` duplicate `mir_json_file` branch; earlier `runner/mod.rs` branch terminates every state | landed at `ef3ee28bc5`; one direct owner; v1/v0 positives and malformed/Program negatives unchanged |
| 2 landed | `skip_ws_probe_reader_delete` / Delete | `skip_ws/dispatch.rs` and route-local MIR-vs-handwritten probe; both concrete arms ended at `build_skip_ws_joinir` | landed at `d4ce50b87c`; direct builder preserves generic-first and missing-target `None`; trim shared dispatcher unchanged |
| 3 landed | `canonical_value_fallthrough_stop` / Stop | `PublishedMirBackendView` canonical `Call(Value)` no-selection -> selected-C JSON re-entry | landed at `a33987e8e4`; `UnsupportedBeforeObject` before temp JSON/C/object; legacy Value compatibility unchanged |
| 4 landed | `methodize_fallthrough_stop` / Stop | `json_artifact` swallowed METHODIZE canonicalizer errors and `core_bridge::methodize_calls` | landed at `24ece062bb`; reject before parse/publication/backend; methodize reissuer 0, singleton/phi unchanged |
| 5 parked | `stage1_return_call_legacy_writer_stop` / Stop | writer/name/arity path deleted at `99b4446cab`; current import closure stops before the selected boundary | `ParkedSealed__SelectedBoundaryUnreachableThroughCurrentImportClosure`; reopen only when an unchanged direct route reaches `FuncLoweringBox` without new authority/fallback |
| 6 landed | `mir_json_v0_call_ingress_stop` / Stop | shared `module.rs` call/mir_call dispatch -> one pre-publication terminal | landed at `9a40ece824`; call/catalog owners and call-only tests deleted; boxcall/externcall/NewBox/non-call preserved; fixed failure-name set unchanged |
| 7 landed | `stage1_return_call_parked_assets_repay` / Delete | three invalid smokes plus three exclusive fixtures, 6 files / 189 lines; default directory discovery had reached them | landed at `f15098cf0b`; paths/discovery 0, Git owns detail, new test/guard/receipt=0 |
| 8 cutover + admission repair landed | `array_element_write_published_c_cutover_i0` / Promote+Delete | typed four-kind C consumer, native projection callers 3→0, and shared OBJ/EXE capability preflight; explicit llvmlite outside scope | source contents/order/Void acceptance remains open; use the production typed caller, not JSON-only `ny-llvmc` |

```text
status = landed
implementation permission = false
```

Tombstone completed evidence with its commit. In-boundary missing acceptance
stays open for repair; only outside dependencies may be ParkedSealed.

##### Dependency tail (not yet executable)

`9a40ece824` (JSON-v0 Stop) -> `f15098cf0b` (Stage1 asset repayment) ->
ArrayElementWrite typed selected-C cutover -> remaining MIR-to-JoinIR readers
-> M7 caller-zero schema deletion.
Open one owner at a time; do not repeat the broad census. Stage1 terminal
ownership/predicate naming is reopen-only: an unchanged route reaching
`FuncLoweringBox` keeps the existing tag and may rename `_has_return_call` to
its retired-writer-candidate meaning; otherwise delete the caller-zero owner.
If that route ever reopens, first narrow the body-wide Call-marker probe to the
exact Return-child shape (the current probe can overreject a separate
Call-then-Return statement); no test seam or fallback. Other closed
reader-stop/delete details remain in Git.

##### ArrayElementWrite acceptance correction and next execution

`9cb7a6c71a` retains the typed row/C consumer and native projection 3→0
cutover; llvmlite remains explicit compatibility. The follow-up admission
repair now applies the existing backend capability policy before physical
OBJ/EXE transport and proves typed-array rejection for both native entries.
The source-site repair is landed: standalone `MethodCall` keeps its exact site
only under the existing callable-root allowlist; raw/script roots stay
unlocated. The focused source compiler test observes all six write kinds.
Full source execution acceptance is still OPEN. The focused tests prove
synthetic rows, malformed Set and synthetic MIR→OBJ/optional EXE, not source
contents/order or source-level Void behavior. The prior “Void-result shape
reject” claim is withdrawn, not carried as evidence.

The standalone `tools/build_llvm.sh -> ny-llvmc` probe is JSON-only and carries
no typed row frame; its `published_array_write_row_mismatch` is therefore
transport absence, not a new admission owner. Source acceptance must use the
existing `hakorune` production caller that supplies `PublishedMirBackendView`.
No source workaround, alternate authority, or rejection-only fixture is allowed.

Then prove all four writes from source through OBJ/link/run and EXE with
observable contents/order, Void semantics and malformed/unsupported rejection
before artifact. A present Void destination is not itself invalid.
Use existing test owners; synthetic evidence stays separate. Reconcile the
fixed lib comparator and named test inventory delta, never blanket rebaseline.
The source-site repair is landed in this bounded series; only the named
published-row admission and artifact acceptance remain open.
`new guard=0`; `new receipt=0`; fixed failure-name set unchanged is the target,
not a claim that the new acceptance has already run.
Ordered follow-through is the workstream's **Ordered frontier**, with Call/R7,
Loop closure and selfhost proof kept distinct.

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
bypasses, stale wrappers (including the parked public `emit_global_call` candidate), disconnected proof modules, and retired backend
consumers leaf-first. A file move or tracked archive copy receives zero
reduction credit. Each family deletes its private tests/guards/docs with the
old edge; durable history remains in Git.

### Post-M7 improvement backlog (navigation only)

These are bounded candidates, not current execution rows. Select one only
through the admission rule in `agent-current-entry-contract-ssot.md` after the
M7/R7 frontier is healthy; do not add a per-candidate guard, receipt, or card.

| candidate | kind | owner / measurable finish | non-claims |
| --- | --- | --- | --- |
| `MIRBUILDER-CONFIG-SNAPSHOT-S0` | landed / no reopen | `CompilationContext.emit_debug_policy` owns the invocation snapshot; selected emit path env reads are 0 at `4ba9293900`, with no child/dispatch port axis added | transitive `emit_guard`/SSA/router reads remain a separate design question; no process-global cache or semantic flag change |
| `MIRBUILDER-EMITTER-FANOUT-S0` | ParkedSealed | `unified_emitter.rs` hops cross profile gating, lookup/map replay, recursion restoration, receipt/error conversion, and the physical Call writer; frame-count reduction alone has old-edge delta 0. Reopen only with one contract-preserving owner and a finite delete-set | no flattening, authority move, receipt/port/guard/test addition, or bypass of typed failure/legacy profile |
| `MIRBUILDER-DEAD-ANNOTATION-RETIRE-S0` | landed / bounded repeats only | caller-zero `_family_is_route_typed` helper, `dead_code` allowance, and unused import removed at `33b69f3e9e`; repeat only with a new compiler-proven caller-zero private asset | no broad purge, visibility widening, or semantic rewrite |
| `LANG-FASTMEM-SOURCE-FATE-D0` | ParkedSealed | finite census found 26 `fastmem` Proof/Test regions and 82 executable `mem.assume*` calls, with Product `.hako` callers=0 but parser/AST/Program-JSON/normal-script transport still live; eventual bounded rows are `FASTMEM-V0-SYNTAX-RET0` and `FASTMEM-ASSUME-RET0` | keep `MirInstruction::MemOp`, region metadata, verifier, access-plan, JSON transport, and LLVM lowering; no immediate source/parser deletion or MemOp removal |

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
5. at row-6 closeout, compact closed docs to hash plus one-line outcome and
   restore task-local headroom toward 900 lines; do not copy a tracked archive;
6. record tracked files/lines and old-edge delta before choosing the next
   family.

The known-red baseline is separate from migration red. Every changed test is
named; unclassified new red blocks the cutover.

### Closed history tombstones

```text
09149c0e88 — M1/M2 finite census and disposition closed.
474e8518b0/d7905cc70e — M3-A fallback fence landed; outer compatibility only.
754a06e7a2 — M3-B/C multi-owner families remain ParkedSealed.
45c6759962/dd8f33e013 — R6 Group A split canonical and legacy instruction shapes.
cce62db090/bb41e2e880 — R6 Group B installed the VM typed Print consumer.
92ed65334f — post-Group-B census found no single complete canonical reader tuple.
c214608280/b49087e91f — NoSafeSlice became family-local and M7-S owns finite Stop/Delete.
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
ときに着地する。Call producer/publication単体のMS1-P境界は上のCall completion
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
frozen accepted corpus / selected backend expectations            = green
```

pack counters、replacement ledger、five-cell LOC、source/test measurements
は、このsemantic completionへ到達する過程の観測値である。増減だけで
implementation permissionやcompletionを決めない。

## Final repository convergence finish line

### Cross-program handoff and finite acceptance (accepted 2026-09-05)

Use `Call/M8`, `Call/M9` for physical thinning/backend retirement and
`Loop/M8`, `Loop/M9` for portable Recipe coverage/parity. Bare milestone
numbers do not select work. Call MS1-P/C/D completion is not whole-MirBuilder
completion. After Call/R7, return to the first unfinished named row in the
Loop chain below; already-landed prerequisites are evidence, not rerun tasks.
The return task is existing `LOOP-PRODUCTION-SELECTION-D0` in
`joinir-loop-selfhost-recipe-pipeline-ssot.md`: consume its M10 prerequisite
list and the closed-status evidence in `generic-loop-source-to-portable-recipe-ssot.md`
and `loop-common-physical-demand-and-session-ssot.md`, select the first unmet
dependency by full task ID, then return to production selection. This is a
handoff task, not permission to activate M10b or repeat closed G0 proofs.
Call/M8 cleanup waits for the owning Call/Loop callers to reach zero;
Call/M9 backend replacement is a sibling, not a reason to delay Loop selection.

Before the next production cutover, close `MIRBUILDER-FINAL-ACCEPTANCE-SCOPE`
inside this existing card: pin an existing corpus manifest at a commit, list
exact program paths/hashes, language profile, selected backend and toolchain,
command, expected stdout/exit/effects or exact rejection tag and boundary.
Give each required case an owner and existing evidence path; missing evidence
is an open task. Specify exclusions (including deferred WASM and unselected
backend parity) explicitly. Expand directory/glob discovery into the fixed
case list at selection. This task is queued, not evidence that scope is frozen.

Accepted source programs may not be changed to rejection cases merely to close
a migration; that requires an explicit language/profile decision. Known-red
names stay separately owned. Post-freeze additions require a recorded scope
decision and cost impact; raw test counts and synthetic MIR tests cannot prove
source-to-artifact coverage. Reuse the corpus owner's manifest and tests;
create no parallel acceptance ledger or per-case guard.

Handoff after Loop retirement and repository convergence is owned by
`selfhost-parser-mirbuilder-migration-order-ssot.md#unified-resume-order`:
language conformance -> canonical mimalloc promotion -> authority migration
selection -> MirBuilder then parser migration -> non-delegating self-compile.
This records dependencies only; CURRENT_STATE still selects execution.

`MIRBUILDER-FINAL-PIPELINE-v1` の完了は Loop の production cutover だけで
終わらない。次の直列順を最終 finish line として固定する。

```text
CANONICAL-FUNCTION-FINISH-TERMINAL-R0
  -> LOOP-PHYSICAL-PREPARE-DESIGN-CORRECTION-R0
  -> caller-zero LOOP-PHYSICAL-PREPARE-P0
  -> Generic G0 prepare parity
  -> common physicalizer / caller-zero canary
  -> production selection
  -> Loop/M8 JOINIR-LOOP-ALL19-PORTABLE-RECIPE0-S6
  -> Loop/M9 SELFHOST-LOOP-PORTABLE-RECIPE-PARITY0-S7
  -> Loop/M10b JOINIR-LOOP-PORTABLE-RECIPE-CUTOVER0-I0-R0
  -> Loop/M11 RAW-LOCATED-LOOP-PORTABLE-HANDOFF0-R1
  -> Loop/M12 JOINIR-LOOP-LEGACY-FAMILY-ADAPTER-RETIRE0-R2
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
metric-derived architecture; DraftSealでのsource re-analysis
```
