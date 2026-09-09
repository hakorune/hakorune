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
- **Current implementation status:** canonical Call and explicit legacy ingress
  are separated; current work is the finite source acceptance reconciliation.
- **Next ordered task:** the active workstream selects remaining source and
  published-boundary cutovers. Constructor/Array execution and retirement
  evidence belongs to their owner SSOTs; no lifecycle V2 waiting state is implied.
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
status = fast_open
implementation permission = true
current cohort = `acceptance_source_reconciliation_i0`

The [workstream](../workstreams/mirbuilder-inplace-replacement-current.md#acceptance-incident-and-bounded-repair-order-2026-09-05)
and [lifecycle Decision](constructor-birth-new-lifecycle-ssot.md#typed-c-program-handoff-decision)
own the root/Birth handoff BoxShape. Pair result BoxCount landed at
`181d7f8e92`: one exact source terminal relation is retained during
`return_scalar`/Completion issuance. Raw field/binary/return remains
non-authority until its dedicated consumer. No view/frame/C extension is
permitted. The [terminal consumer design](constructor-birth-new-lifecycle-ssot.md#terminal-consumer-design-2026-09-06)
fixes the task order: scalar coverage repair, connected terminal consumer,
then ABI/body handoff and C execution. Stop at the requested planning boundary. Pair
EXE30/linked OBJ30 and selected old-edge retirement remain open.

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
| 8 cutover + admission repair landed | `array_element_write_published_c_cutover_i0` / Promote+Delete | typed four-kind C consumer, native projection callers 3→0, shared OBJ/EXE capability preflight, and write-only source/MIR/OBJ/EXE/reject evidence; explicit llvmlite and readback reader remain outside scope | landed; `ArrayBox.get/length` remains the separate `mir_call_no_route` terminal; next cohort is `vm_legacy_call_terminal_collapse_i0` |

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

`9cb7a6c71a` and its follow-up admission repair are closed for the bounded
write-only scope: source/MIR row order, native OBJ relocations, MIR EXE
execution, semantic Void transport, and typed-array reject-before-artifact are
evidenced through the production caller. `ArrayBox.get/length` remains the
separate `mir_call_no_route` reader terminal and is not part of this row;
llvmlite remains explicit compatibility. No new guard/receipt/fixture or
baseline change was introduced.

##### VM legacy call terminal collapse (current execution)

Bounded VM `LegacyCallV0` terminal collapse landed at `a74648f0b3`: both VM
instruction paths now use one existing typed rejection
helper. The bounded delete-set is the old `handle_call`/`execute_callee_call`
dispatch and VM-only legacy trace display; canonical `MirCall` and the
`LegacyCallV0` schema remain unchanged. Acceptance is the existing nine call
handler tests, `cargo check --features vm-reference`, structural zero callers
for the deleted symbols, and unchanged fixed baseline/pointer guards; new guard=0,
new receipt=0, and the fixed failure-name set unchanged.
No VM feature parity, Hako/WASM/JSON ingress, R7 schema deletion, or whole
repository-green claim belongs to this cohort. The remaining MIR/JSON boxcall
and JSON egress readers are `ParkedSealed__SharedCompatibilityCallersNoExclusiveDeleteSet`; reopen only with an existing owner-specific decision or a caller-zero asset, not a new census or D0.

Ordered follow-through remains the workstream's **Ordered frontier**, with
Call/R7, Loop closure and selfhost proof kept distinct.

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
inside this existing card. Its first finite product scope is the existing
`tools/smokes/v2/suites/integration/real-apps-exe-boundary.txt` owner at
repository `72f2496568fcd555499fdcb26fef2d8f1df03089` and manifest revision
`fb27826bbf81694bb86056a5fbf389b6216b5eea`. Run exactly
`tools/smokes/v2/run.sh --profile integration --owner-profile integration
--suite real-apps-exe-boundary`; record the ten source paths and the one
unsupported-boundary probe selected by that manifest, expected stdout/exit,
toolchain/backend, and source/script hashes. The required owner
and evidence are the suite manifest, runner, and each existing smoke; missing
evidence remains open. Exclude language-v1 corpus,
Loop/M8-M9 parity, non-delegating selfhost proof, WASM, unselected backends,
and the whole-library known-red baseline. Expand discovery once into this
fixed list; do not create a parallel ledger, fixture, guard, or synthetic
acceptance. This queued scope is not a whole-MirBuilder completion claim.
Accepted source programs may not be changed to rejection cases merely to close
a migration; that requires an explicit language/profile decision. Known-red
names stay separately owned. Post-freeze additions require a recorded scope
decision and cost impact; raw test counts and synthetic MIR tests cannot prove
source-to-artifact coverage. Reuse the corpus owner's manifest and tests;
create no parallel acceptance ledger or per-case guard.

#### Acceptance evidence and current blocker (2026-09-10)

The exact owner suite was run at current `1752f973e9` and at its pinned parent
`72f2496568fcd555499fdcb26fef2d8f1df03089` in an isolated worktree with the
same release `hakorune`/`ny-llvmc` tool pair. Current results are `1/11` pass
and `10/11` fail; the pinned parent is `6/11` pass and `5/11` fail. The
unsupported-boundary probe passes in both runs. The five common reds are
`boxtorrent_mini_exe`, `binary_trees_exe`, `mimalloc_lite_exe`,
`json_stream_aggregator_exe_runtime_boundary`, and `allocator_stress_exe`;
they remain separately owned baseline/source-boundary failures.

The five current-only reds are split by owner. `typed_object_newbox_min_exe`
is outside the exact canonical storage vocabulary because it declares
`IntegerBox`; the language type SSOT treats `IntegerBox` as an object identity,
while the old metadata compatibility planner treated it as an inline i64.
`typed_object_untyped_field_min_exe` and
`typed_object_birth_param_min_exe` use legacy `init { ... }` slots; the
canonical package issuer currently ignores `init_fields`, so metadata commit
rejects the declaration projection before layout. `typed_object_birth_min_exe`
reaches the C consumer and stops at the unimplemented `FaultFrameEnter` shape.
`typed_object_method_min_exe` depends on the typed layout result and must be
reclassified after those upstream owners are fixed. The existing
source-backed package -> `take_object_definitions` -> `ModuleDraftCollector`
transfer is present; RawCompatibility must remain an adapter and must not gain
canonical fallback, AST re-inference, or old metadata authority.

The bounded design order is therefore: (1) decide and issue `init_fields`
membership in the existing object-definition issuer with its source order,
duplicate, weak-field, and foreign/brand rejection rules; (2) reconcile the
`IntegerBox` direct-EXE contract between the language type SSOT and the typed
layout reference, choosing an explicit source migration or an explicit
representation contract; (3) handle `FaultFrameEnter` in its existing C
consumer owner; then rerun this exact suite and reclassify the method Birth
case. These are separate slices: no new receipt, RawCompatibility fallback,
or accepted-source-to-rejection mutation is allowed, and no production
cutover claim is made while the current-only reds remain.

##### `MIRBUILDER-INIT-FIELDS-CANONICAL-PROJECTION-I0`

Decision: legacy `init_fields` names join the existing source declaration
projection once, preserving source order and existing duplicate/weak rules.
Source authority + canonical issuer: `ASTNode::BoxDeclaration` consumed by the
existing `instance_constructor_semantic::object_definition::issue`.
Non-authority: `CompilationContext` metadata, MIR observations, layout
inference, RawCompatibility, and C consumers. Fail-fast boundary: source
coverage/duplicate/weak/foreign drift rejects before package transfer or
metadata commit. Smallest next slice: add the issuer normalization and focused
projection tests; do not decide untyped storage. Non-claims: `IntegerBox`
vocabulary, `FaultFrameEnter`, method Birth, and final EXE acceptance remain
separate rows.

I0 evidence (landed at `f4b387de48`): the existing object-definition issuer now
projects legacy `init_fields` after explicit fields exactly once, preserving
source order, duplicate elimination, and weak-field marking before the
`CanonicalObjectDefinitionV1` crosses the package boundary. The focused issuer
suite is `6/6` green, including an AST-level weak-field mutation because the
default parser grammar intentionally rejects the legacy weak spelling. This
slice does not claim typed storage, C execution, or source-to-EXE acceptance.

##### `MIRBUILDER-TYPED-OBJECT-STORAGE-CONTRACT-D0` (accepted)

Decision: the canonical typed-object EXE route accepts explicit exact numeric
field declarations, with `i64` as the current live scalar contract. `IntegerBox`
is an object identity in the language type SSOT and is not an i64 alias; existing
typed-object source using that historical spelling must migrate explicitly to
`i64`. Legacy `init { ... }` names remain source membership with no declared
storage type and are unavailable to the canonical layout until a separate
dynamic/opaque slot contract is accepted.

Source authority + canonical issuer: the language type/reference contract and
`ASTNode::BoxDeclaration` consumed by the existing object-definition issuer.
Non-authority: old metadata planners, `MirType`/MIR observation, layout
inference, RawCompatibility, and C consumers. Fail-fast boundary: unresolved
`IntegerBox` and untyped storage stop before canonical layout/package transfer;
they must not be silently treated as i64 or inferred from writes. The accepted
legacy inference remains compatibility-only. Non-claims: dynamic slots,
`FaultFrameEnter`, method Birth, whole-suite recovery, and final MirBuilder
completion.

##### `MIRBUILDER-TYPED-OBJECT-SOURCE-MIGRATION-I0`

Decision: migrate the two existing canonical typed-object smoke sources that
declare `IntegerBox` fields (`typed-object-newbox-min` and
`typed-object-method-min`) to explicit `i64`, preserving field order, Birth
evaluation order, and expected exit values. Do not change the untyped
`init_fields` fixtures in this slice; they remain the separate dynamic-storage
design row below.

Source authority + canonical issuer: the checked-in `.hako` source fixture and
the existing `object_definition::issue` path. Non-authority: compatibility
planner aliases, MIR storage observations, and test-only source rewrites at
lowering time. Fail-fast boundary: the migrated source must produce the same
canonical declaration and typed layout without a compatibility retry. Smallest
slice: source/comment migration plus focused parser/layout and the two existing
EXE smoke callers. Exclusive old edge: the two `IntegerBox` spellings in these
canonical fixtures. Non-claims: untyped fields, dynamic/opaque slots, C
`FaultFrameEnter`, and method Birth beyond the reclassification exposed by the
focused smoke run.

I0 evidence (landed at `89199dbc3c`): both canonical fixtures now declare
explicit `i64` fields. `typed_object_newbox_min_exe` reaches the selected pure
first EXE and exits `30`; `typed_object_method_min_exe` passes the old
`IntegerBox`/layout blocker and is now isolated at the existing MIR JSON
`FaultFrameEnter` egress stop. The canonical layout negative suite is `3/3`
green, including explicit `IntegerBox` and untyped-field rejection. No source
fixture was changed to a rejection case.

##### `MIRBUILDER-FAULT-FRAME-ENTER-JSON-EGRESS-I0`

Decision: carry the existing `FaultFrameEnter { dst, mode }` relation through
the selected MIR JSON egress as the already defined `fault_frame_enter` row.
Preserve `root_owned` versus `borrowed`; do not erase or synthesize a frame in
the emitter. Source authority is the existing function fault-frame owner and
compiled-entry contract. Canonical consumer is the existing
`src/runner/mir_json_emit` owner; C v4 remains a physical consumer and must not
reclassify the mode. Non-authority: RawCompatibility, C defaults, function
names, and a retry route. Fail-fast boundary: unsupported mode, duplicate/missing
frame, or mismatched `fault_frame` use remains rejected before artifact output.
Smallest slice: add the allowlist/emitter case and focused JSON positive/negative
coverage, then rerun the existing method smoke to expose its next owner. Do not
open child cleanup, dynamic slots, or untyped storage here.

I0 evidence (landed at `36cdbb08c8`): the selected MIR JSON allowlist and
emitter now carry the existing `fault_frame_enter` row with its `dst` and
`root_owned`/`borrowed` mode. The focused emitter test is `1/1` green. After
refreshing the release `hakorune` binary, `typed_object_method_min_exe` passed
the FaultFrameEnter boundary and stopped at the next named owner:
`MIR JSON emit contract violation: unsupported terminator Invoke`. This is a
reclassification only; no Invoke fallback or compatibility retry was added.

##### `MIRBUILDER-INVOKE-LIFECYCLE-JSON-TERMINATOR-D0`

Decision (accepted): selected lifecycle `Invoke`, `InvokeNormalResult`, and
`ReturnFault` use the existing `hako.published-lifecycle-physical-program.v2`
transport. Do not add `Invoke` to generic `src/runner/mir_json_emit`; that
transport is a harness/generic lane and its explicit rejection remains the
fail-fast boundary. The physical encoder already emits the complete triplet,
including operation, frame, result, and landing relations.
Source authority + canonical issuer: `PublishedMirBackendView` binds the
existing `CompiledEntryContractV1` into one
`PublishedLifecyclePhysicalAbiInputV1`, then
`published_backend_view/physical_program_json.rs` emits the physical document.
Canonical consumer: `LifecycleInvocationInputV1` →
`compile_published_lifecycle_physical_v4` → the C V4 parser/admission/index/
emitter. Non-authority: generic MIR JSON, RawCompatibility, function names,
MIR observations, C reclassification, and any retry route.
Fail-fast boundary: target/arity/receiver mismatch, missing or orphaned
normal/fault landing, frame mismatch, unsupported operation/result kind,
duplicate or missing normal projection, and physical schema/key drift reject
before artifact output.
Smallest next slice: route the existing `typed_object_method_min` production
EXE/OBJ caller through `emit_published_view_exe`/`compile_published_view_object`
and record physical JSON positive/negative evidence. Keep the generic
`selfhost_build.sh --mir` lane explicit and unsupported for lifecycle Invoke;
do not add a second JSON schema or CLI fallback. Do not open child cleanup,
dynamic/opaque storage, Map extensions, or whole-suite acceptance here.
Non-claims: arbitrary Invoke support, a new runtime ABI, generic JSON lifecycle
execution, child Birth execution beyond the existing physical cohort, or final
MirBuilder completion.

##### `MIRBUILDER-INVOKE-LIFECYCLE-PHYSICAL-V2-CUTOVER-I0`

Decision: use the existing selected physical lifecycle caller for the method
cohort. The source/MIR module is admitted by `PublishedMirBackendView`, bound
as `PublishedLifecyclePhysicalAbiInputV1` with one runtime session, serialized
once by `physical_program_json`, and consumed by C V4 for OBJ/EXE. The generic
`selfhost_build.sh --mir` route remains a compatibility/harness probe and must
continue to reject lifecycle `Invoke`; it is not a second production route.
Issuer: existing source-backed normal pipeline and
`PublishedMirBackendView::issue_lifecycle_physical_abi_input`.
Consumer: `compile_published_view_object` / `emit_published_view_exe` through
`LifecycleInvocationInputV1` and `compile_published_lifecycle_physical_v4`.
Fail-fast: selected route, runtime archive/session, physical schema, target
mapping, Birth/ordinary target, frame, landing blocks, projections, cleanup,
and artifact result must all be accepted by the existing V4 checks. No name
repair, generic JSON retry, or CLI-side operation inference.
Smallest slice: make the existing method acceptance caller invoke this
selected physical owner, add one positive method physical JSON/OBJ/EXE check
and the existing negative contract checks, then record exit `30` and linked
OBJ evidence. Do not alter C child gates, dynamic storage, Map semantics, or
generic MIR JSON vocabulary.
Non-claims: generic JSON lifecycle execution, arbitrary Invoke families, new
runtime ABI, child cleanup expansion, and whole-suite completion.

##### Physical V2 probe evidence (2026-09-10)

The selected physical caller was exercised directly with the existing CLI
entry (`--backend mir --emit-exe --emit-exe-nyrt`) and a runtime directory
containing both required archives. `apps/typed-object-birth-min/main.hako`
reached the canonical physical caller, linked an executable, and exited `30`.
This is positive evidence for the existing physical V2/C V4 Birth cohort; it
does not close the method row.

`apps/typed-object-method-min/main.hako` still stops before the published-view
callback with
`[freeze:contract][ordinary-new/local-commit/artifact-source-unavailable]`.
The same source reaches a complete diagnostic MIR dump containing the root
`Pair.sum()` method call, but its root result/cleanup ownership is not admitted
as a finished artifact. Therefore this is an upstream source-backed ordinary
method Call/root-cleanup owner boundary, not a physical JSON or C V4 parser
failure. Do not add a generic-JSON Invoke case, a retry, or a C-side default to
work around it. The method physical OBJ/EXE acceptance remains open until the
existing Call/root cleanup owner supplies a completed artifact handoff.

The owner-scoped terminal Call probe is already fixed at `c8887bbda2`; the
focused test
`mir::normal_callable_semantic_package::direct_call_lifecycle_tests::physical::terminal_call_probe_is_scoped_to_its_source_owner`
passes `1/1`. That finding is closed and must not be re-opened as the method
artifact blocker.

##### `MIRBUILDER-INVOKE-LIFECYCLE-ROOT-METHOD-CALL-D0`

Decision: pause before implementation until the source-backed target relation
for a lexical receiver in the selected static root is named and verified. The
existing `VerifiedResolvedMethodCallSourceV1` preserves site/receiver/selector
facts only; `AppMainDirectCallDispositionLoanV1` is issued from direct-call
observations, while `DeclaredInstanceCallRelationV1` covers instance-method
callers and does not cover `Main.main -> pair.sum()`. Therefore the current
method stop is a missing source-to-target issuer, not a C/V4 transport fault.
Source authority + canonical issuer: the resolver's exact MethodCall row plus
the existing typed-object/source declaration relation must co-seal the lexical
receiver's selected callable target before any lifecycle row is issued. The
ordinary-New Completion/root cleanup remains the consumer; no second target
registry or backend name lookup is allowed.
Non-authority: `MirInstruction::Call`, MIR type observations, receiver names,
generic JSON, C-side defaults, compatibility retry, and the existing
`DeclaredInstanceCallRelationV1` when its caller/receiver contract does not
match the static root.
Fail-fast boundary: missing/foreign/duplicate method site, lexical receiver
not proven to the selected New object, target declaration/arity/result contract
mismatch, or missing completion/cleanup rejects before artifact publication.
Smallest next slice: census the existing typed-object declaration/layout and
selected-New source products for a lossless `pair.sum()` target crosswalk, then
either bind that existing issuer to `AppMainDirectCallDispositionLoanV1` or
record `NoSafeSlice` with the exact missing authority. Only after this closes
may the root terminal callback emit the existing ordinary lifecycle `Invoke`.
Non-claims: no new MethodCall semantic receipt, no dynamic/opaque storage, no
child cleanup, no generic MIR-JSON Invoke support, and no OBJ/EXE acceptance.

The D0 census is now closed as `NoSafeSlice` for implementation: the existing
products are individually real but do not co-seal the required relation.
`VerifiedResolvedMethodCallSourceV1` has site/receiver binding/selector/arity;
`ResolvedInitializerRelationV1` has binding-to-initializer site; and
`OrdinaryNewCandidate`/`OrdinaryNewAdmissionClaimV1` has the exact New site,
`Pair` class, destination binding, object, and Birth key. The callable catalog
and selected map have the `Pair.sum/0` key and result/signature rows, but none
of these products joins the receiver binding to that selected New and target.
`DeclaredInstanceCallRelationV1` is limited to instance-method callers, and
`AppMainDirectCallDispositionLoanV1` is limited to direct-call observations;
neither is a valid issuer for `Main.main -> pair.sum()`.

The next D1 design task must name one package-private source issuer and prove
this finite chain without a new semantic authority:

```text
MethodCall site
 -> exact receiver binding
 -> initializer relation
 -> selected New site
 -> canonical object/class
 -> selector + arity
 -> exactly one selected instance declaration
 -> result/signature contract
```

It must reject reassigned or foreign bindings, unselected/opaque New values,
class/target-owner drift, ambiguous selector/arity, absent selected target, and
missing result contract before lifecycle publication. If the existing products
cannot be joined without reissuing meaning, retain `NoSafeSlice` and leave the
physical V2/C V4 consumer unchanged.

##### `MIRBUILDER-INVOKE-LIFECYCLE-ROOT-METHOD-CALL-D1`

Decision: keep the source-to-target crosswalk in the existing
`ordinary_new` co-seal owner. It already owns the selected root Completion,
ordinary-New claims, and terminal ownership; it may issue one package-private,
affine root-method disposition row for the existing lifecycle terminal Call.
That row is a handoff/consumption product, not a second source authority or a
public semantic receipt. The direct-call loan remains direct-call-only unless
the D1 proof shows an in-place extension preserves its source contract.

Source authority + canonical issuer: the resolver's exact
`VerifiedResolvedMethodCallSourceV1`, the same function's initializer and
assignment facts, the existing `OrdinaryNewAdmissionClaimV1`, the source-backed
declaration catalog, and the selected batch/result/signature products. The
`ordinary_new` issuer must co-seal the following finite relation before the
terminal callback can return true:

```text
MethodCall site
 -> lexical BindingRef (same owner)
 -> one initializer for that binding
 -> one selected OrdinaryNew claim at that initializer site
 -> claim class/object
 -> instance declaration lookup by class + selector + arity
 -> one selected catalog identity/batch row
 -> one matching completion/result/signature contract
```

The target key is accepted only as the source-catalog lookup result and must
be checked against the selected identity and result owner. It is never rebuilt
from MIR symbols, receiver spelling, or C input. The row is consumed by the
existing root terminal lifecycle validation and leaves no residual row.

D1 inventory finding: the source crosswalk facts are available for comparison,
but the existing `AppMainDirectCallDispositionRowV1` also owns a canonical
physical emission issued from `VerifiedCallableHeaderV1` and the root direct
call index. A lexical instance target is not admitted by that direct-call
index. The generic selected-instance emitter and receiver ingress exist, but
D1 must still prove a root lifecycle projection that preserves the catalog
key, physical signature, result owner, receiver lane, and cleanup contract, or
stop at `NoSafeSlice`; it may not fabricate a header from a name, MIR type, or
receiver spelling.

Finite disposition before implementation:

| state | issuer condition | allowed terminal |
| --- | --- | --- |
| `Ready` | every link above is unique, same-owner, selected, and contract-complete | existing lifecycle `Invoke` path |
| `Unavailable` | receiver is not a direct lexical local, New is unselected/opaque, or the supported result contract is absent | existing source-unavailable stop; no C ingress |
| `Rejected` | owner/site drift, reassignment, duplicate initializer/target, class/arity mismatch, foreign selected identity, or result/signature mismatch | fail-fast before artifact publication |
| `NoSafeSlice` | joining the products would require reissuing source meaning, name/MIR recovery, or a synthetic physical header | retain D1 stop and leave physical V2/C V4 unchanged |

Non-authority: `AppMainDirectCallDispositionLoanV1` when its direct-call
observation contract does not match, `DeclaredInstanceCallRelationV1` for
instance-method callers only, `MirInstruction::Call`, MIR types, user-box
route metadata, generic JSON, C defaults, compatibility retry, and receiver
names. No child cleanup, dynamic/opaque storage, generic MIR-JSON `Invoke`, or
physical backend change is included.

Fail-fast boundary: the issuer rejects before lifecycle publication and before
the root terminal callback admits `Invoke`. Positive acceptance is the
existing typed-object method source through the selected root handoff; negative
acceptance covers reassigned/foreign locals, multiple matching New claims,
unselected New, wrong class or arity, missing selected target, and missing or
drifting result/cleanup contract.

Smallest next slice: read-only D1 implementation inventory in
`ordinary_new_coseal` and its existing source/result products, ending in one
named package-private row shape and one focused positive/negative test plan.
Do not edit C, physical transport, or generic MIR JSON until that inventory
proves the row can be issued without a new semantic authority. If it cannot,
record the exact missing source relation and keep `NoSafeSlice`.

Current D1 inventory status: the resolver, initializer, New claim, selected
identity, and result-contract facts can be compared without name recovery. An
existing generic selected-instance projection is present, but it is not a root
lifecycle consumer: `DeclaredInstanceCallLocatorScopeV1` can lend the selected
instance key and exact receiver binding, `take_exact_receiver_value()` can
provide the already-materialized `ValueId`, and
`emit_canonical_instance_value_terminal_v1()` can form
`Callee::SameModuleInstance { key, receiver }`. Those pieces currently emit a
plain generic `MirInstruction::Call` and serve the `me.method(...)` caller
mode; they do not issue the root `Invoke`/cleanup product for `Main.main`.

The root direct index is still free-static-only, and
`VerifiedCanonicalDirectCallEmissionV1` requires its verified callable header.
Therefore D1 remains `NoSafeSlice` for code until the existing source-backed
instance target facts are joined by the root owner and a package-private
lifecycle projection is explicitly designed with its own owner. The existing
root terminal Call shape is also scalar-direct-only:
`TerminalI64CallReturnV1` retains literal arguments without a receiver, while
`RootHomeExitEntry::Call` validates a direct emission with no receiver lane.
`Pair.sum()` needs the selected instance key plus its receiver `ValueId` and
physical receiver lane. It cannot be squeezed into the direct scalar row.

The bounded physical bridge is now named, without authorizing implementation:

```text
source-issued instance key + exact receiver binding
  -> existing ledger receiver ValueId
  -> Callee::SameModuleInstance { key, receiver }
  -> existing terminal_call::emit_ingress()
  -> Invoke(Call{I64}) + InvokeNormalResult + root cleanup graph
```

`terminal_call::emit_ingress()` and `root_cleanup_graph::call::ingress()` are
existing lifecycle consumers. The remaining physical boundary is separate:
the current ordinary-call physical program and C V4 schema carry source
arguments but no receiver lane, and `physical_program::ordinary_callable_key()`
accepts only `Callee::Global`. A future physical-consumer card must choose one
explicit receiver projection and validate the instance signature lane; it must
not add a generic MIR-JSON `Invoke`, infer the receiver in C, or reuse the
direct global row. Rust MIR receiver-bearing construction and OBJ/EXE
acceptance are therefore separate claims.

The next design slice is the package-private root lifecycle projection and its
source-owned crosswalk. Its acceptance must prove exact owner/site, selected
instance key, receiver `ValueId`, result/signature/cleanup agreement, and
direct-vs-instance exclusivity. Only after that slice closes may the physical
receiver projection and `Pair.sum()` OBJ/EXE exit-30 test be opened.

The active I0 has one upstream contract prerequisite: the current
`apps/typed-object-method-min/main.hako` declares `Pair.sum()` without a return
annotation. The existing completion/result issuer maps an unannotated return to
no usable result contract, even though the body is an i64-shaped Add. The I0
positive path must therefore either use the existing source contract (the
fixture's `sum(): i64` declaration) or stop in a separate design row for an
explicit unannotated-return authority. It must not infer `i64` from MIR or
invent a backend result row. Missing, Void, non-i64, owner-drift, and signature
drift remain fail-fast negatives.

##### ROOT-METHOD-I0 integrity follow-up queue (2026-09-10, expanded 2026-09-10)

The following findings were verified against `814629bb` and are queued after
the active physical receiver-lane D0. They are behavior-preserving owner/ordering fixes;
they do not change the selected source shapes, add a semantic receipt, or open
the physical receiver lane. Execute them in the listed order after the physical
receiver-lane D0 has reached a natural closeout boundary.

| order | bounded task / owner | change and fail-fast boundary | acceptance / non-claims |
| --- | --- | --- | --- |
| 1 | `ordinary_new` terminal-probe family owner scope (`add` / `literal` / `field`, with `unit` explicitly audited) | Every probe must query the requested owner's `TerminalRelationV1` and `Completion` only. A missing relation for that owner returns `None` so the next probe may run; a foreign relation is never detected by comparing an unbranded relative source node. Preserve true owner/site, completion, duplicate, and terminal-kind checks. The Unit arm must either consume an owner-indexed relation through an existing owner-scoped root/child consumer or remain an explicit root-only contract with a named stop; it may not silently fall through for child owners. | Add a matrix covering root/child owners for Call, I64Add, IntegerLiteral, I64Field, and Unit dispositions. Same-relative-site functions reach their own arm without false `return-site-mismatch`/`owner-drift`; a branded owner/site mismatch still rejects; a child I64Add's field reads remain consumable. No terminal-family widening, no new semantic receipt, no cross-owner scan, and no fallback based on a missing row. |
| 2 | `ordinary_new_coseal` field-read accumulator | Replace the AppMain `field_reads = staged_reads` overwrite with the same owner-scoped additive merge used for children. Apply the merge for child `I64Field` and `I64Add` terminal relations, validate every relation's staged reads before publication, and reject duplicate sites at merge time. Processing order must not determine whether child rows survive. | Exercise child-before-AppMain and AppMain-before-child declaration orders with I64Field and I64Add rows; both retain all owner-scoped reads, while duplicate, foreign-site, and missing-read inputs fail before completion publication. No new field-read authority or completion product. |
| 3 | `ordinary_new_root_instance_call` expected-state scope | Replace the process-wide `root_instance_call_expected: Cell<bool>` observation with an owner-scoped observation (or an existing owner-derived relation) and make every caller query it with its owner. A child terminal probe must not observe the AppMain method's unavailable target and turn it into `artifact-source-unavailable`; the root method still stops when its own selected target/result contract is unavailable. | Add a root-with-missing-method-contract plus a terminal-home child case, and the inverse declaration order; only the root owner receives the source-unavailable stop. Preserve selected root method success and existing direct-call behavior. No new resolver, no retry, no child-method physical support. |
| 4 | `published_backend_view::physical_program::collect_ordinary_calls` | Traverse blocks and instructions through the existing deterministic block/instruction order (the same sorted block ids used by `issue_function`) before issuing ordinary-call order. Do not use `HashMap::values()` as a numbering source. Preserve membership, destination, arity, and signature checks. | Compile the same multi-callee module repeatedly and assert identical ordinary physical function order and diagnostic-site order; retain malformed destination/membership rejection. No new physical numbering scheme and no C/backend change. |
| 5 | `global_call_route_plan::same_module_static_helper_contract` PHI convergence | Use deterministic sorted block order for PHI propagation and do not publish a partially propagated contract after the iteration budget. If the finite propagation cannot reach a fixed point, return the existing unavailable/typed failure before routing; do not silently consume a HashMap-order-dependent intermediate. Keep the existing bounded alias walk separate. | Construct a PHI chain longer than the current budget and a reordered-block equivalent; both either converge to the same contract or fail with the named unavailable outcome before route admission. Retain existing mixed-contract rejection. No new route kind, no budget increase used as a semantic fix, and no backend fallback. |
| later / measured | C physical invocation index reuse | After a selected physical caller is active and a profile names this hot owner, share the invocation-local definition/block index and validated value facts between V2 structural checks, dominance checks, and V4 emission. Remove only redundant scans; keep each trust-boundary check. | Requires a before/after measurement and an existing physical caller. Do not count this queue item as a correctness fix, do not add a global cache, and do not claim compile concurrency or whole-backend speedup without measurement. |

Rows 1--4 are `BoxShape` refactors: the source authority,
canonical issuer, terminal consumer, and exclusive delete-set already exist.
Row 5 is a fail-fast convergence correction at an existing route owner.
While the physical receiver-lane D0 is active, these follow-ups must not be used to paper over
`artifact-source-unavailable` or to authorize OBJ/EXE acceptance.

##### `MIRBUILDER-INVOKE-LIFECYCLE-ROOT-METHOD-CALL-I0`

Decision: accept one bounded implementation slice for the Rust MIR/root
lifecycle side. The existing `ordinary_new` co-seal owner will issue a
package-private affine `RootInstanceCallDisposition` row after the selected
result and physical-signature cohorts are available. The row is a crosswalk
and physical handoff only; it does not become a second resolver authority or a
public semantic receipt. The existing direct-call row remains direct-only.

Issuer: the ordinary-New ledger, using the resolver's exact MethodCall and
initializer facts, its own still-live `OrdinaryNewAdmissionClaimV1`, the
source-backed declaration catalog, selected batch map, result contract, and
physical signature. The issuer must accept only a root lexical local receiver
whose binding has exactly one direct initializer at a selected New claim. It
looks up the instance key by the claim's canonical class plus the resolver's
selector and arity, then checks selected identity, result owner/identity,
instance mode, one receiver lane, and i64 result/Completion. Any binding
reassignment, foreign owner, unselected/opaque New, duplicate target, class or
arity drift, missing result/signature, or missing cleanup is rejected before
the row is published.

Consumer: the root lowering port takes the row exactly once, obtains the
receiver `ValueId` only through the existing owner-scoped ledger binding
lookup, and builds this existing MIR shape:

```text
MirCall::new(None, Callee::SameModuleInstance { key, receiver }, source_arguments)
```

It passes that call to the existing `selected::terminal_call::emit_ingress()` and
records the same root cleanup entry. Direct and instance rows are an explicit
exclusive pair in the private root-entry representation; the instance row
must never be coerced into `VerifiedCanonicalDirectCallEmissionV1`.

Focused acceptance for I0 is the selected typed-object source through the
Rust MIR terminal: exact `Pair.sum/0` key, receiver lane 0, argument order,
`InvokeOperation::Call { result: I64 }`, normal `InvokeNormalResult`, fault
frame, and root cleanup graph. Negative cases cover reassignment, wrong
class/arity, foreign or unselected New, duplicate selected target, missing
receiver materialization, and result/signature/cleanup drift. This slice does
not claim physical-program/C V4 support or OBJ/EXE exit 30; those belong to a
separate physical receiver-lane card after I0 closes.

Non-authority: MIR names/types, receiver spelling, generic JSON, C defaults,
the `me.method(...)` locator as a root issuer, compatibility retry, and any
new backend route. No source fallback or generic MIR-JSON `Invoke` is allowed.

Non-claims: this design does not authorize code, fixture, production switch,
OBJ/EXE acceptance, or any other MethodCall family.

I0 Rust closeout evidence (2026-09-10): the selected fixture now declares
`Pair.sum(): i64`; the existing result issuer supplies the selected
`InstanceBoxMethod` contract, and focused source tests cover both the ready row
and the unannotated/unavailable case. The common Invoke verifier accepts the
existing `SameModuleInstance` + `I64` shape only when the selected namespace
and source arity match. `--dump-mir` reaches the complete root lifecycle:

```text
NewBox -> Birth(Unit) -> SameModuleInstance(Pair.sum/0, receiver %11)
       -> InvokeNormalResult -> HomeRelease -> Return
```

This closes the Rust source/lifecycle I0. The selected physical caller still
rejects the receiver-bearing call at
`[freeze:contract][published-lifecycle-program/ordinary-call-callee]`; that is
the next physical receiver-lane design boundary, not a failed Rust I0.

##### `MIRBUILDER-INVOKE-LIFECYCLE-ROOT-METHOD-CALL-PHYSICAL-RECEIVER-LANE-D0`

Decision: accept one bounded physical projection using the existing lifecycle
wire. An `InstanceBoxMethod` ordinary function keeps role `ordinary_i64`, puts
its physical receiver in the already-supported function-level `receiver` field,
and leaves explicit source parameters in `params`. Its ordinary call carries a
`receiver` field alongside the existing `target`, `args`, and `dst` fields.
Static ordinary calls keep their current receiverless shape. This is a lane-0
projection, not a new schema revision or a second callable product.

Source authority + canonical issuer: the closed ordinary-New
`RootInstanceCallDisposition` and selected `InstanceBoxMethod` physical
signature. The published physical program consumes the existing
`Callee::SameModuleInstance { key, receiver }` once. It may use the existing
function parameter lane 0, but it must not derive the receiver from names,
MIR types, or C text. Non-authority remains generic MIR JSON,
`physical_program::ordinary_callable_key()` while it is Global-only, function
names, C defaults, and compatibility retry.

Consumer and delete-set: extend the existing physical-program issuer,
`physical_program_json`, V2 validator, V4 index/flow, and V4 emitter to
recognize the instance receiver field. Retain the static Global/FreeFunction
ordinary shape and delete only the receiverless rejection and static-only
arity assumptions for selected instance rows. Generic MIR JSON lifecycle
`Invoke` remains rejected.

Fail-fast boundary: instance namespace/owner/key mismatch, receiver lane not
exactly one, missing or duplicate receiver field, receiver unavailable at the
call site, source arity/signature drift, missing ordinary definition, or a
receiver-bearing call entering the static Global row must reject before
JSON/object output. C must treat the function-level instance receiver as a
borrowed handle and explicit params as i64 lanes.

Acceptance: the annotated `Pair.sum()` source emits lifecycle JSON, passes V2
and V4 admission, reaches LLVM/object/executable, and returns exit 30; existing
static ordinary calls retain their current output and result. Negative coverage
rejects missing/extra receiver, wrong namespace, duplicate target, wrong
receiver value, and arity/signature drift. No arbitrary instance methods,
dynamic/opaque receivers, child cleanup expansion, generic JSON lifecycle
support, or unrelated OBJ/EXE claims are included.

##### `MIRBUILDER-INVOKE-LIFECYCLE-ROOT-METHOD-CALL-PHYSICAL-RECEIVER-LANE-I0`

Implementation row: use the D0 projection above. The Rust issuer must accept
`Callee::SameModuleInstance` only for the selected instance namespace, check
`key.arity() == call.args.len()`, require exactly one physical receiver lane,
and project the receiver once into the existing physical function/call input.
The Rust JSON consumer and C validators/emitter must share that one lane; no
receiver prefix repair, name lookup, or generic fallback is permitted.

Focused gates: physical-program positive/negative unit tests, V2/C V4 focused
admission tests, static ordinary regression, then `Pair.sum()` OBJ/EXE exit 30.
The old Global-only path is removed only after these selected caller tests are
green. This row does not reopen the three owner/ordering refactors queued after
the physical receiver lane.

##### `MIRBUILDER-UNTYPED-OBJECT-STORAGE-D0` (queued)

The source issuer preserves `init_fields` membership, but canonical layout must
continue to reject missing storage type. A future row must choose either source
annotations/migration or an explicit dynamic/opaque slot and tagged runtime ABI;
MIR observation, constant-caller heuristics, and receiver-name inference cannot
be promoted into this authority.

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
