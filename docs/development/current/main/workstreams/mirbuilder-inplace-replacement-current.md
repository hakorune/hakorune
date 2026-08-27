---
Status: Fast — exact owner-pack selection I0; aggregate discovery fate parked
Date: 2026-08-27
Decision: MIRBUILDER-INPLACE-REPLACEMENT0
Policy:
  - docs/development/current/main/design/mirbuilder-inplace-replacement-policy-ssot.md
North star:
  - docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md
Call owner:
  - docs/development/current/main/design/mir-canonical-callsite-lane-ssot.md
Active card:
  - docs/development/current/main/investigations/mir-call-d1b-root-lineage-exact-target-loan-d0-2026-08-26.toml
Task map:
  - docs/development/current/main/investigations/mirbuilder-inplace-replacement0-task-map-2026-07-28.md
---

# MirBuilder In-Place Replacement Workstream

## Goal

最終production authorityを一本へ収束させる。

```text
Resolve -> Observe -> Facts -> Recipe -> Verify
        -> Lower -> Seal -> Collect -> Atomic Publish
```

現在のMirBuilderを稼働させたまま、一つのproduction responsibilityとその旧辺を
同じbounded seriesで交換する。第二MirBuilder、consumer-zero route、fallback、
fixture由来のacceptance、新しい文字列authorityは作らない。

## Current six-line brief

Decision:
  CanonicalV1 Value S0は着地済み。VM fate監査はRust engineの最終撤退を受理したが、
  MirBuilder本線を優先し、WpreをbypassするCoreDirectとforce-hv1だけを先に閉じる。
  CoreDirect D0の6行分類、ProductAot substring S0、R0のone-state post-decode terminal/no-retry deletionは着地し、force-hv1の有限caller fateを凍結した。ProductAot S0はsource attemptがarray_element_writeでpure-firstに拒否されたため、backend capability ownerを設計するまで停止する。

Source authority + canonical issuer:
  各suite ownerがproduct AOT、semantic reference、Stage1 proof、compile-only、retireの
  どれを必要とするかを発行する。route selectorは選択済みterminalだけを一回発行する。

Non-authority:
  raw text、alias map、physical symbol、function table、EffectMask、registry、
  `caller=None`、methodize、`args[0]`、optimizer/backend repair。

Fail-fast boundary:
  retired selectorはfamily-level tag/rcを一つだけ返し、retry/fallbackは0。Wpreは一つの
  strict root、decoder、terminalだけを選び、malformed/conflictを別schemaで再解釈しない。

Smallest next slice:
  `FORCE-HV1-R0A-GUARD-CLOSEOUT-I0` is landed: focused 86/90 census tests, retired-path disjointness, three direct exceptions, and future-row dispatch are green; no production route changed. `FORCE-HV1-GUARD-CURRENT-LIFECYCLE-I0` is landed: explicit phase dispatch, one body-derived summary, registry wiring, and focused tests are green; no production route changed.
  The zero-match D0 is accepted and `SMOKE-OWNER-PACK-EXACT-SELECTION-I0` is now the scoped fast row; its five-entry phase2050 pack is linked to the registry row and aggregate caller. Stage1 AOT boundary and separate PHI/provider R0b fate remain closed.
  `SMOKE-OWNER-PACK-AGGREGATE-DISCOVERY-D0` now selects one finite policy: phase2050 `run_all.sh` is `ExplicitOnlyAggregate` in a one-row manifest, excluded only from parent integration discovery; its five leaves remain integration-owned and direct aggregate invocation remains the exact owner pack. The design card is `docs/development/current/main/investigations/smoke-owner-pack-aggregate-discovery-d0-2026-08-27.toml`; implementation remains a separate bounded I0.
  `FORCE-HV1-CENSUS-PER-LEAF-SCHEMA-S0` is landed。checked-in bodyから116 leaves / 120 lexical sitesと33/33 direct、44/45 conditional、35/35 explicit-core、4/7 dynamicを再導出するv1 observationを固定した。helper envのunsetはCoreからhv1へ意味を変え得るためauthorityにせず、conditionalのままowner fateを要求する。
  `FORCE-HV1-DIRECT-HISTORICAL-DELETE-R0a` is landed: 30 direct HistoricalDelete leaves were retired with body hashes and projection updates. The active body-derived inventory is now 86 leaves / 90 lexical sites (direct 3/3, conditional 44/45, explicit-core 35/35, dynamic 4/7). The PHI witness and provider route remain explicit R0b exceptions; legacy non-force residual consumers are still non-authority.
  R0a changed only phase2047-2050 projections and owner docs/guard; phase2051/phase2100, Stage1, conditional/dynamic families, startup, fallback, Wpre, and Call schema remain closed. phase2170 ProductAot is still blocked at array_element_write. The owner-pack exact-selection I0 is now active; R0b and Stage1 fate remain later blockers.

Design-only follow-up:
  force-hv1 censusは各leaf本文とsealed environment contractからbody_sha256、
  lexical_entry_sites、route basis、ambient preemptionを導出し、reviewed fateをowner証拠で
  別管理する。旧`hv1 failure -> Core fallback` edge、numeric rc、runtime loop repetitionは
  authorityから除外する。A④ sentinel/non-growthとD ledger/index/dead-link checksはcleanup
  task mapへ登録済み。有限phase/summaryを固定する
  `FORCE-HV1-GUARD-CURRENT-LIFECYCLE-D0/I0`は着地済み。`quick` 0-matchを証拠にしない
  `SMOKE-OWNER-PACK-ZERO-MATCH-D0`も受理済みで、現在はその exact-selection I0だけを実装する。
  その後に post-emission と実行terminalを分離した `FORCE-HV1-STAGE1-AOT-BOUNDARY-D0` を開く。

`SMOKE-OWNER-PACK-ZERO-MATCH-D0` design brief (accepted; I0 fast-open):
  Decision: keep `execution_profile=quick` as runtime policy and add an explicit
  `owner_profile=integration` only for exact owner-pack discovery. Reuse the
  existing suite loader; do not create a second filter/glob parser.
  Source authority + canonical issuer: one checked-in suite manifest issues
  exact owner membership; the runner issues the nonzero discovery result before
  the first test effect. The aggregate caller owns which pack it requests.
  Non-authority: `Done` text, wildcard/filter strings, old zero-match rc=0,
  leaf runtime rc, inherited env, and profile changes that alter timeout/config.
  Finite state: `SameProfileLegacy` preserves old filter behavior;
  `CrossProfileWithoutSuite` rejects before discovery; `ExactNonEmptyPack`
  proceeds; `Missing|Empty|Duplicate|Foreign|StalePack` rejects before run;
  `UnresolvedOwner` (phase2120's three absent names) stays a blocker and is
  never inferred or resurrected.
  Fail-fast boundary: owner-profile vocabulary is closed, cross-profile lookup
  requires `--suite`, every manifest entry resolves exactly once, and partial
  skip/zero match cannot become a passing execution.
  Ordered tasks: D0 landed -> `SMOKE-OWNER-PACK-EXACT-SELECTION-I0`
  runner seam + one exact phase2050 five-entry pack ->
  `SMOKE-OWNER-PACK-AGGREGATE-EXCLUSION-I0` (one explicit phase2050 row;
  no broad run*.sh change) -> phases2047-2051 packs
  -> phase2100 packs -> reusable structural guard. Then
  `FORCE-HV1-STAGE1-AOT-BOUNDARY-D0` (post-emission boundary; issuer VM is
  ParkedSealed), Stage1 lifecycle guard, and exact AOT S0. Force-hv1 PHI/
  provider/conditional/dynamic fates follow separately; Wpre waits for their
  caller-zero evidence; only then the fixed Call/MirBuilder spine runs.
  Non-claims: no phase semantics, force-hv1 fate, VM/Call/Wpre change, broad
  `run*.sh` conversion, fixture resurrection, or implementation permission.

Non-claims:
  broad/default Rust VM、vm-hako、PyVM、HMI、typed Global、observer/loan、
  Method/receiver、Call schema、performance、Loop/M8/M9、warning cleanup。

Census boundary:
  public route selector -> selected executor/retirement terminal, plus external
  Rust `MirInterpreter` constructor -> engine. The fate Decision covers
  CoreDirect/force-hv1/fallback/broad/default/reference/vm-hako/PyVM; LLVM
  internals、archive、llvmlite G3は境界外。

## Current architecture decision

Final Call shape:

```rust
enum Callee {
    Global(CanonicalGlobalTargetV1),
    Method { receiver: ValueId, /* existing typed method fields */ },
    Value(ValueId),
    Extern(String),
}

Call {
    dst: Option<ValueId>,
    callee: Callee,
    args: Vec<ValueId>,
    effects: EffectMask,
}
```

```rust
enum CanonicalGlobalTargetV1 {
    Builtin(CanonicalBuiltinGlobalV1),
    SameModule(CanonicalSameModuleGlobalTargetV1),
}

enum CanonicalBuiltinGlobalV1 { Print }

enum CanonicalSameModuleGlobalTargetV1 {
    FreeFunction { name: Box<str>, arity: u32 },
    StaticBoxMethod { owner: Box<str>, method: Box<str>, arity: u32 },
}
```

`Print`はexact `print/1`だけ。finite runtime providerは`panic/1`と`exit/1`
を同名Externへ出す。bare `error`と`now`はauthority不在でrejectし、explicit
`env/nyash.console.error`だけがExternになる。mathはMethod、GCは未実行Global
producerをretireしてrejectする。
import aliasはfinal moduleのexact declarationに着地した時だけSameModuleとし、
distinct Imported/Helper/Generated/Legacy variantは作らない。JoinIRは生成関数
宣言とcallを同じownerでco-sealし、SameModule FreeFunctionを使う。

```text
legacy text
  -> owner-private compatibility resolver exactly once
  -> typed target
  -> canonical Call
```

Canonical Call-corridor MIR JSONはexact `schema_version = "2.0"`。Global targetはfamily付き
object、`args`/`effects`は必須、`flags`/`func`/aliasは不可とする。effectsは
coreのcanonical順で重複・未知語彙なし。exact `1.0`とv0はowner-private
compatibilityであり、明示schemaのparse失敗を別schemaでretryしない。v2のop cohortは
`const/copy/copy_owned/destroy_owned/newbox/field_get/binop/compare/branch/jump/phi/ret/mir_call`
だけで、他opはtyped unsupported。full MIR-v2 vocabularyはこのlaneのclaim外。

## Current finite state

```text
Global B0
  BuiltinPrintReady
  SameModuleFreeFunctionReady
  SameModuleStaticBoxMethodReady
  CompatibilityTextReady
  MissingSourceRelation
  ForeignModule
  DuplicateOrCollision
  AliasUnresolved
  WrongNamespace / WrongArity
  UnsupportedForWireOrCompiledConsumer
  TypedRejectBeforeEffect
  ExternOrMethodOrConstructionOwner

Ingress Wpre
  SharedArtifactProfile
  DirectMirProfile
  CanonicalV2Selected
  CompatibilityV1Selected
  CompatibilityMirV0Selected
  ProgramV0OwnerSelected
  CanonicalV2ParserUnavailableBeforeB1
  FamilyForbiddenAtEntranceRejected
  SelectedDecoderRejected
  MalformedJson / MalformedSchema
  ConflictingMarkers / UnsupportedVersionOrShape
  TypedRejectNoRetry

D1B lifecycle
  ObserverOnlySiteRecorded
  OwnerObservationComplete
  DispositionReady
  TargetReadyForLoan
  KnownNonDirect
  TypedRejectBeforeArguments
  ParkedCompatibility
  PackageAbortBeforeInstall
  RawLoanInstalled / Consumed / Exhausted
  ResolvedFallthroughForbidden
```

Only a typed target crosses into argument descent. `KnownNonDirect` exits through
its typed owner. Reject/park issues no target. Incomplete inventory aborts before
install. Resolved fallthrough and residual loan are guard failures, not terminals.

## Adversarial correction

Ordinary `FunctionCall`のobserver/package contractも受理済み。selected shadow
profileのDeferred edgeは次の一回限りtransitionで置換する。

```text
profile-gate-adjacent observer-only FunctionCall
  -> record existing site/name/arity
  -> observe arguments in the same traversal
  -> issue no target
  -> allow owner observation to complete
  -> require total disposition before package install
```

No second AST walk、package-external scratch、target issuance、semantic profile
widening、BodyEffect inference。Package installはtotal dispositionを要求する。
後続affine loanは既発行Calleeだけをmoveし、同じcellでdirect
`CatalogedTargeted` payloadを削除する。

## Ordered frontier

```text
0. MIR-CALL-GLOBAL-TARGET-B0-FINITE-IDENTITY-DECISION        (architecture accepted)
   three structural shapes, bounded exact v2, future one-way symbol projection,
   observer contract, and the finite readiness queue below are accepted

1. MIR-CALL-INGRESS-SCHEMA-LIFECYCLE-GUARD-S0               (landed)
   reusable fail-closed guard; phases wpre_readiness/wpre_i0/typed_global_b1/r7

1a. MIR-CALL-B0-PROVENANCE-TOMBSTONE-R0                     (landed, docs-only)
    inventory every section removed by 9bff1a1ff2 that carried review_source,
    A/B/C disposition, QualifiedStaticPayloadAbsent, TargetPayloadMissing, or
    terminal_role_split; map each to the current B0 owner with superseded_by /
    retained historical token, and record the user-supplied Pro review plus the
    six read-only audit roles integrated by 45bff917e3. No transcript is invented

1b. MIR-CALL-GLOBAL-TARGET-B0-MACHINE-CENSUS-G0             (landed, fast guard-only)
    turn the finite family matrix, compiled-consumer owner inventory, Wpre/wire
    impact table, and exact print/1 attribution into one machine-readable manifest
    plus reusable fail-closed guard. Unknown owner/family/path and stale manifest
    fail; this guard is evidence only and grants no Wpre/B1 implementation permission

1c. MIR-CALL-INGRESS-SCHEMA-SELECTOR-WPRE-D0-PROFILE-ROOT-DECODER-CONTRACT (design stop)
   bound Wpre-I0 to shared runner family-unknown entrances and freeze profile x root
   matrix, one parsed Value ownership, decoder signatures, strict duplicate-key owner,
   and exact delete set. No parser or fallback code changes.

1d. MIR-CALL-INGRESS-SCHEMA-SELECTOR-WPRE-D0-OUTSIDE-FATE-CLOSE (completed, docs-only)
   split Stage1 arbitration from captured payload, make core-direct an in-scope blocker,
   add reference child re-entry and actual C-ABI/LLVM/runtime callers, and give every
   outside row owner/status/reason/reopen/non-authority. No implementation.

1e. MIR-CALL-INGRESS-SCHEMA-SELECTOR-WPRE-D0-STAGE1-DIRECT-ARBITRATION (completed, design-only)
   explicit Stage1 CLI + any JSON CLI conflicts; multiple JSON CLIs conflict; one explicit
   JSON CLI beats ambient Stage1; no explicit JSON CLI keeps existing Stage1 selection.
   Captured MIR-v0/Program-v0 stays family-selected compatibility.

1f. MIR-CALL-INGRESS-SCHEMA-SELECTOR-WPRE-D0-REFERENCE-CHILD-REENTRY (completed, design-only)
   freeze vm-hako child route-environment isolation and canonical-v1 emission family; no
   public Wpre/hv1 re-entry or retry is accepted.

1g. MIR-CALL-INGRESS-SCHEMA-SELECTOR-WPRE-REFERENCE-CHILD-PRIVATE-TRANSPORT-I0 (landed)
   move the reduced MiniVm payload to the two private one-of transport keys, scrub public
   Wpre/hv1/Stage1/fallback/Program-JSON/trace selectors, pin TOML/VM policy, and migrate
   the three live reference monitors plus the active alias probe atomically. Guard and
   cleanup evidence are closed; no later row opens automatically.

1h. MIR-CALL-INGRESS-SCHEMA-SELECTOR-WPRE-REFERENCE-CANONICAL-V1-VALUE-S0 (landed)
   after transport I0, emit one explicit CanonicalV1 serde_json::Value, normalize the same
   owned Value once, subset-check once, and project once. Ambient profile selection,
   temporary v1 file/readback, and production raw String parses are removed.

1i. CORE-DIRECT-RETIRE-D0 (landed design-only) -> CORE-DIRECT-SUBSTRING-PRODUCT-AOT-S0
   (landed) -> CORE-DIRECT-RETIRE-R0 (landed)
   D0 classifies the six active smokes into ProductAot / SemanticReference /
   HistoricalDelete. S0 moves the one product-observable substring case to an
   exact EXE/AOT owner. Pre-Wpre R0 uses one post-decode stderr terminal
   ([core-direct/retired], rc=1); unavailable is ParkedSealed until Wpre owns a
   family issuer. R0 deletes raw probe/child/in-proc reparse/VM fallback.

1j. MIR-CALL-INGRESS-SCHEMA-SELECTOR-WPRE-D0-FORCE-HV1-FATE (design accepted, implementation closed)
   retire production force-hv1; the legacy arithmetic snapshot is 116 lexical leaves (33 direct,
   74 textual helper, 9 wrapper-only), 78 force reachers / 80 invocations, and 38 non-force
   residual consumers with migration blockers=9. Body/environment observation is 120 lexical sites:
   33/33 direct-force sealed, 44/45 conditional-force candidate, 35/35 explicit-core residual,
   and 4/7 unresolved dynamic; conditional/unresolved rows are CutoverBlockerOpen and cannot be
   ParkedSealed. Standard-v1 reroute is forbidden.
   Design order: body-derived census schema -> direct HistoricalDelete R0a owner review/exception validation -> Stage1 Proof/AOT
   -> Map exact AOT -> Array push capability -> Array exact AOT -> four dynamic fates
   -> narrow helper cut with explicit-core residuals preserved -> startup tombstone and closeout.

1k. MIR-CALL-INGRESS-SCHEMA-SELECTOR-WPRE-S0-VALUE-SEAMS (design accepted, implementation closed)
   strict_root.rs owns recursive duplicate/trailing rejection; SelectedIngress is owned and
   non-Clone; decode consumes it and borrows one Value without raw-string reparse or retry.

2. MIR-CALL-INGRESS-SCHEMA-SELECTOR-WPRE-I0
   shared runner parses JSON root once; selects exact v2/v1/MIR-v0/Program-v0 once; deletes raw
   substring selection, canonicalize/reload, dispatch-local cascade, swallowed
   core-direct error/re-entry, and explicit-v1-error -> v0 retry

3. MIR-CALL-MIRCALL-CALLFLAGS-RETIRE-R0
   replace the live one-stage transport and retire reader-zero flags

4. MIR-CALL-EFFECT-AUTHORITY-E0
   freeze source/provider/wire-owned effects before any target reroute: Print=IO,
   panic/exit=IO|CONTROL, Extern=declared exact mask or reject, same-module=
   semantic package/body mask, compatibility=explicit owner mapping with no default

5. MIR-CALL-SAME-MODULE-SOURCE-IDENTITY-PRESERVE-R0
   preserve exact free/static source identity through
   `VerifiedTrivialDirectCallTargetV1` and `VerifiedResolvedOwnerHeaderV1`;
   delete parse-back, but retain exactly one guarded old one-way String
   projector/publication until B1 consumes and types it

6. MIR-CALL-IMPORTED-STATIC-EXACT-RELATION-R0
   alias plus exact final-module declaration -> retained structural source row;
   alias-only/foreign/ambiguous input rejects before arguments

7. MIR-CALL-COMPAT-GLOBAL-RESOLUTION-W1-R0
   v1/v0/Program-v0 text resolves once to a finite target disposition or reject;
   keep only a guarded old physical publication until B1, never a new String issuer

8. MIR-CALL-JOINIR-GENERATED-FREE-TARGET-J0
   co-seal JoinFuncId, exact JoinFunction name/arity and declaration; delete
   generated-name fallback, alias fanout and target Const, but retain exactly one
   guarded old Global(String) physical publication until B1 types it

9. MIR-CALL-BOUNDED-GC-FALSE-GLOBAL-RETIRE-R0
   delete `gc_collect/gc_stats` Global publication and reject before arguments;
   future GC semantics stay parked behind their own source owner

10. MIR-CALL-D1B-SELECTED-FUNCTIONCALL-OBSERVATION-COMPLETION-I0
   delete the selected Deferred edge, complete owner/package issuance, abort
   incomplete disposition before install, and issue no target from observation

11. MIR-CALL-GLOBAL-BUILTIN-EXTERN-DISPOSITION-R0
    exact print/1 retains one guarded Global publication for B1; panic/1 and
    exit/1 reroute to same-name Extern; bare error/now/println reject; explicit
    declared console/env/host providers remain Extern

12. MIR-CALL-D1B-ALL-LINEAGE-PRE-EFFECT-RETIRE-R0
   six lineages + Unlocated/Relationless become exact target / KnownNonDirect /
   typed reject / ParkedSealed before arguments; delete caller=None, Resolved,
   unique/tail recovery, target Const, and arbitrary legacy publication while
   retaining the Cataloged direct payload for the next affine replacement

13. MIR-CALL-D1B-CATALOGED-SOURCE-RELATION-AND-AFFINE-LOAN-I0
    exact site/owner/catalog co-seal -> non-empty stack-owned loan -> take_once
    -> arguments once -> Call once -> residual zero; direct CatalogedTargeted
    payload deleted in the same cell after late recovery is already zero

14. MIR-CALL-METHOD-CORRIDOR-R0
   receiver lives only in Callee; args are source args; consume the already
   selected effect authority; close StageB instance, rewrite/known, and
   ordinary-new birth producers with receiver ABI, then delete prepend/strip/
   autoscan/args[0], Method(None), methodize, guard repair, UnknownBox,
   optimizer Global->Method, and VM Global recovery

15. MIR-CALL-SAME-MODULE-ALL-PRODUCER-DISPOSITION-R0
    classify static method terminal, generic CorePlan GlobalCall, and env-gated
    arithmetic/comparison/unary Global publishers. Exact final-module declarations
    retain one guarded old publication; authority-free publishers reject/retire
    before effects. No formatted owner/name/arity or plan `func: String` is authority

16. MIR-CALL-GLOBAL-TARGET-DEAD-TEXT-CALLSHAPE-S0
    move only the call-shape matcher out of the 790-line owner; no behavior change

17. MIR-CALL-GLOBAL-TARGET-B1-CURRENT-HEAD-C0
    enumerate the finite surviving exact String publications and every compiled
    consumer by owner/action; arbitrary publisher/recovery/methodize count is zero.
    Any new hole inserts an owner-specific S0/R0, then C0 reruns; only exhausted C0
    with all remediation rows closed may open B1

18. MIR-CALL-GLOBAL-TARGET-B1-CUTOVER
    add the serde-free defs type; atomically change both `Callee::Global` and
    `CallTarget::Global`; type the retained exact publications; add bounded v2
    codec, sole projection and one MirModule lookup; adapt/delete/isolate every
    compiled consumer without formatter, reparse, registry, fallback, or retry

19. MIR-CALL-WIRE-CONSTRUCTION-TERMINAL-R0
    close isolated noncanonical compatibility and construction terminals:
    Constructor -> NewBox and Closure -> NewClosure/Value. Canonical ignored/default
    effects are already zero at E0/W1/B1 and cannot be deferred to this row

20. MIR-CALL-R6-CURRENT-HEAD-RECENSUS-C0
    writers, func readers, optional Callee/receiver, construction variants,
    sentinels, wire/backend retry, and guards recounted at current HEAD

21. MIR-CALL-CORE-SCHEMA-CUTOVER-R6
    atomically delete func, Option<Callee>, optional receiver, INVALID/0 target

22. MIR-CALL-LEGACY-GUARD-CLOSEOUT-R7
    legacy fixtures move to compatibility ingress; impossible-state guards,
    stale comments, README/reference/current history close

23. MIRBUILDER-POST-CALL-INTEGRATION-R0
    recovery context deletion -> root/recursion state localization -> finite
    CompilationContext/metadata/raw-port/adapter/barrel owner cleanup

24. remaining selected pipeline rows -> final repository convergence audit
```

The VM retirement sibling may perform read-only census work earlier, but its
broad implementation begins only after row 23:

```text
route selection de-ambient
  -> legacy vm-compat / PyVM product hook / current vm-hako retirement
  -> broad/default Rust execution caller migration
  -> independent AOT HMI artifact and reference cutover
  -> Rust MirInterpreter caller zero and physical deletion
```

`vm-active-lane-retirement-ssot.md` owns that backend sequence. This workstream
owns only the pre-Wpre CoreDirect/force-hv1 bypass closure and the Call spine.

No later row can be pulled before an earlier authority boundary. Local green,
worker review, textual caller-zero, or schema compile errors are not permission.

## Source and ownership budget

Do not append semantic code to these owners:

```text
src/mir/builder/raw_invocation_source_transport.rs      778
src/mir/builder.rs                                      741
src/mir/builder/normal_callable_semantic_loan_port.rs   710
src/mir/builder/raw_expression_dispatch/mod.rs          706
src/mir/builder/calls/unified_emitter.rs                 711
src/mir/string_dead_text_region_plan.rs                  790
src/mir/builder/control_flow/plan/normalizer/helpers_value/lower.rs 744
```

The 778-line transport requires a behavior-neutral owner split before touch.
The 790-line dead-text owner has the exact call-shape S0 named in row 16.
`builder.rs`, the 744-line lowerer, and `unified_emitter.rs` are
deletion/delegation-only. Target,
inventory, handoff, loan, and recursive capability code goes into small
owner-specific siblings. Every touched/new source stays `<760`; `>=800` stops.

Until Call closure, keep these integration owners because they are current
production seams, not cleanup mistakes:

```text
RawExpressionDispatchPortV1                    sole AST matcher
RawInvocationChildPortV1                       recursive capability root
NormalCallableSemanticPackagePortAdapterV1     package/raw integration seam
CompilationContext                             facade pending finite owner split
FunctionMetadata                               127-row consumer manifest owner
normal_default_root_catalog_lifecycle          install/source orchestrator
```

After Call R7, shrink them in this order:

```text
delete method-tail recovery context
-> localize root_is_app_mode
-> unify recursion-depth scope after old emitter retirement
-> classify raw root capability vs recursive frame
-> split CompilationContext one closed cohort at a time
-> split FunctionMetadata only from its 127-row manifest
-> caller census -> retire -> test home -> compatibility shelf -> barrel shelf
```

## Reduction forecast

Finite read-only census estimates the Call corridor can remove roughly
1,200–1,700 gross source lines after the authority cutover. A separately
verified caller-zero `externals.rs` retirement raises the gross range to about
1,600–2,100. These are forecasts, not acceptance evidence; typed identity and
loan preparation may temporarily add code before old-edge deletion.

The following are not current deletion claims:

- `json_v0_bridge` still has multiple live caller families;
- `variable_accum` and `source_coverage.rs` are production-live;
- JoinIR merge code cannot be retired from text references alone;
- `_p0` proof files require evidence migration, not suffix-based deletion.

## Production invariants

```text
named production caller required       = yes
same-series selected old-edge deletion = yes
target decision before arguments       = yes
route selection per request            = exactly 1
RootLower execution per request        = exactly 1
canonical rejection -> retry/fallback  = 0
partial product publication            = 0
source AST clone/reparse                = 0
new semantic target/route acceptance   = 0
source/check file line limit            < 800
```

## Compatibility anchors

These IDs remain here because reusable guards consume them. They are boundaries,
not a copied landed ledger.

```text
CALL-BOX-KIND-ANALYZER-COMPAT-SUNSET-001
  state: active
  owner: CalleeBoxKindPolicyContextV1::ResolverExtendedCompiler
  growth: forbidden

NESTED-BOX-RAW-BODY-COMPAT-SUNSET-001
  state: closed

NORMAL-DEFAULT-GENERAL-MODULE-COMPAT-SUNSET-001
  state: closed

MIRCOMPILER-ARBITRARY-AST-COMPAT-SUNSET-001
  state: closed

RUNTIME-MIRBUILDER-AST-JSON-COMPAT-SUNSET-001
  state: closed

SCRIPT-EXISTING-ROOT-LOWER-COMPAT-SUNSET-001
  state: Parked; Compatibility origin has no canonical replacement owner

STAGE1-DIRECT-POST-MACRO-NONPROGRAM-COMPAT-SUNSET-001
  state: closed
  retired by: STAGE1-DIRECT-POST-MACRO-WHOLE-FILE-PROGRAM-SEAL0-I0-R0

RAW-NONPROGRAM-ROOT-COMPAT-SUNSET-001
  state: closed
```

## Parked boundaries

- PyVM/reference/Python production activation and non-selected backend
  activation/parity. Compiled Rust core-schema consumers, including WASM and
  non-selected consumers, still require B0/B1 adaptation/isolation/retirement.
- RawCompatibility, bounded GC, Main exact-i64/FullFunction, Brand/special,
  ExplicitExtern, ASTNode::Call, and ArrayElementWrite until their owners select.
- warning/dead-code/chronic measurement work in the cleanup map.
  Its fixed order is `CHRONIC-MEASUREMENT-REFRESH-D0` ->
  `PANIC-SURFACE-CENSUS-D0`/`DEAD-CODE-REMEASURE-D0` ->
  `CHRONIC-MEASUREMENT-EXPECTATION-I0` -> `ASTCLEAN-STALE-GUARD-SUPERSEDE-R0`:
  reproduce/classify first, emit one expectation TSV/refresh guard second, then
  tombstone stale per-script thresholds. Exact 334 is observation, never a
  production ceiling; A④ sentinel/non-growth and D ledger/index rows remain
  parked until their owners/evidence are registered.
- performance, mimalloc, llvmlite, Hako converter, Loop/M8/M9, and physical-type
  follow-ups until `CURRENT_STATE.toml` reselects them.
- broad Context/metadata/port/barrel cleanup until Call R7.

Reopen only on a selected current row, a new production caller, or an accepted
owner-specific Decision. Parked code/tests never grant implementation permission.

## Detached territory audit queue (2026-08-27)

The 16-territory read-only audit produced the following named follow-ups. They
are taskized here without changing the selected reference-child I0; no row below
authorizes code until `CURRENT_STATE.toml` selects it.

1. `MIR-CSE-SAME-BLOCK-STATS-DETERMINISM-R0` — high correctness candidate.
   `cse.rs` currently walks `HashMap` blocks and can reuse a non-dominating value
   across sibling blocks; its elimination counter also counts non-rewrites.
   Owner: `src/mir/passes/cse.rs` plus the semantic-simplification consumer and
   SSA-focused tests. Safe policy is conservative same-block CSE only: no exact
   same-block/dominance proof means no rewrite and zero count. Positive/negative
   determinism and actual-`Copy` statistics are required. This is outside the
   reference-child transport I0.

2. `CONC-ENV-TASK-SPAWN-OWNER-D0` — compatibility design stop.
   `env.task.spawn` is publicly reachable but currently echoes a clone or returns
   `Ok(None)` without issuing a Future/scheduler task. Its authority is the
   existing `nowait`/Future/TaskGroup contract, not the C-ABI Future route.
   First close it as explicit typed unsupported/arity failure with zero side
   effects; a real spawn implementation needs a separate issuer/ABI decision.

3. `MIR-CALL-V1-FUNC-SENTINEL-R6` — Call schema blocker, not current I0.
   v1 explicit-callee ingress still writes `func=ValueId::new(0)`, which pollutes
   `used_values`/JoinIR remap. The R6 cutover must use the canonical constructor,
   remove the dummy field atomically, and prove Global/Method/Extern/Value inputs
   do not create a false operand. Do not patch parser/schema files during the
   private transport row.

4. `MIR-REFERENCE-LENGTH-MISSING-METADATA-D0` — reference-only semantics choice.
   Hako reference handlers still turn missing/unsupported `length` metadata into
   zero. Decide strict typed reject versus an intentionally retained stub, then
   add valid-size and missing/unsupported negative evidence. This is not selected
   Rust VM behavior and cannot be used to reopen VM production work.

5. `MIR-CALL-RECEIVER-ABI-ROUNDTRIP-R0` — later receiver retirement family.
   The emitter prepends a Method receiver while VM/Hako handlers strip, autoscan,
   or use `args[0]`. After receiver ABI authority is fixed, delete the prepend
   and the six stripping/recovery sites together; preserve the receiver in
   `Callee::Method` and source-only arguments. No partial removal is safe.

6. `PARSER-LOGICAL-OP-LEXICAL-BOUNDARY-D0` — language design consultation.
   `normalize_logical_ops` rewrites source before tokenization, misses single-
   quoted literals, and shifts spans. The tokenizer already owns `&&`/`||` and
   accepts bare `and`/`or`; the open decision is whether word aliases remain
   canonical, compatibility-only, or rejected, plus quote/span guarantees.
   Design-only matrix first; no implementation permission. This is the one item
   that may need an explicit user/Pro language decision.

7. `MIR-COMPILE-TIME-PERF-BASELINE-P0` ->
   `MIR-SOURCE-OBSERVATION-SINGLE-PASS-D0` — measurement before cleanup.
   AST deep clones and repeated environment reads are code-backed candidates but
   hotness/frequency are not proven. Establish a reproducible baseline and a
   finite clone/env census first; only then consider a session snapshot or lazy
   clone removal. Keep this separate from semantic Call and reference transport.

## Parked guard contract and retirement queue (2026-08-27)

This is the sole successor queue for the existing
`GUARD-SURFACE-CONSOLIDATION-D0`; do not create one card or shell guard per
finding. It does not preempt the selected force-hv1 design stop.

```text
Decision: keep execution authority in guard_rows.toml plus typed specs, keep
  reverse inventory as a non-authority projection, stop new unregistered public
  guards first, and retire old guards only after successor coverage and
  caller-zero are both observed.
Source authority + canonical issuer: the git-tracked eligible surface, flattened
  guard manifest/spec graph, and owner-reviewed disposition.
Non-authority: raw LOC, filename prefixes, profile names, the hand-written index,
  generated reverse output, historical thresholds, or executable mode alone.
Fail-fast boundary: a new unregistered public guard, duplicate/dangling graph
  edge, stale expectation, missing execution caller, or unproved retirement
  stays red/retained.
Smallest next slice: GUARD-CONTRACT-GRAPH-D0, then a structure-only registry
  ratchet which executes no member guard.
Non-claims: no full registry migration, quick-static activation, bulk chmod,
  compiler behavior change, or grep/count-authorized deletion.
```

Current source-backed observation is 3,748 tracked check paths and 356,395
shell lines. The full inventory classifies 92 stable entries, 106 manifest
command paths, 212 proof-app paths, and 3,338 `unknown_retain` paths. The
narrow public-guard boundary is 2,741 entries: 91 are manifest-mapped and 2,650
are not. These are different denominators and must not be reported as one
coverage percentage. `quick-static` has 21 declared rows (including the exact
owner-pack data row) and no profile caller;
the profile name is not execution evidence.

Required order:

1. `GUARD-CONTRACT-GRAPH-D0` — freeze the two-plane model, contract-v1 metadata,
   declared-profile versus reachable-gate distinction, and the existing six
   inventory dispositions. Generated reverse output never becomes authority.
2. `GUARD-REGISTRY-RATCHET-I0` — extend the existing inventory owner, register
   the already-CI-reachable `ci_feedback_tier_policy_guard.sh`, lower the
   public-unregistered cap from 2,650 to 2,649, reject newly added unregistered
   public guards relative to the PR base, and wire only this structural check to
   required CI. It must not run registry member commands.
3. `DEAD-CODE-REMEASURE-D0` -> `CHRONIC-MEASUREMENT-EXPECTATION-I0` ->
   `ASTCLEAN-STALE-GUARD-SUPERSEDE-R0` — use one token-aware scanner and one
   per-file expectation TSV. Exact-form 334/111 is diagnostic; inclusive
   attribute grammar currently observes 351/126 and is the required D0 scope.
   Remove the 13 obsolete source-wide numeric clauses rather than relaxing them;
   retain living leaf checks, and explicitly supersede ASTCLEAN-007 by
   ASTCLEAN-013 before deleting 007.
4. `GUARD-MANIFEST-MODEL-R0` -> `GUARD-REGISTRY-HEALTH-R0` — BoxShape-consolidate
   manifest loading, define argv-derived executable semantics, and classify the
   44 non-manifest hako-alloc closeout wrappers as register, consolidate, retire,
   or retain. No mass chmod follows from the current 0644 baseline.
5. `GUARD-REVERSE-INDEX-I0` — extend
   `tools/docs/guard_surface_inventory.py` with guard/invariant/path queries and
   forward/reverse edge checks. Migrate two existing rows first. Keep
   `check-scripts-index.md` human-facing and keep its legacy compatibility block
   byte-stable until its callers reach zero.
6. `SOURCE-LINE-BUDGET-CENSUS-D0` -> `SOURCE-LINE-BUDGET-SPEC-I0` — rederive
   target, threshold, kind, and focused caller group; do not freeze the stale
   estimate of 77 guards. Move one MIRBuilder family to the existing typed-spec
   runner, then make old focused guards delegate exactly once before removing
   their inline `wc`/threshold authority.
7. `GUARD-FAMILY-RETIREMENT-R0` — process bounded families only. A guard may be
   removed when its invariant is owned by a named successor or the guarded route
   is physically impossible, all CI/manifest/parent/docs callers are zero, and
   the supersede/retirement edge is recorded. Otherwise it remains
   `unknown_retain`; inactivity and non-registration are not deletion evidence.
8. `QUICK-STATIC-QUALIFY-D0/I0` — finite-classify all 19 rows for side effects,
   latency, and current green status. Only after qualification may the existing
   anti-wiring contract close and the whole profile become a CI entry.

Acceptance for the series is monotonic: new public unregistered guards = 0;
unknown inventory does not grow silently; each selected family loses its local
duplicate expectation in the same slice that gains central coverage; deleted
guards have named successors and caller-zero; generated forward/reverse edges
round-trip; and no family migration changes compiler or test semantics.

## Short closed tail

- normal-root T0/C0/R0 and atomic source-backed cutover are closed.
- JSON-v0 Call target resolution and Program late target rewrite are closed.
- Callee operand/use/escape/ownership/query projection rows are closed.
- selected optimizer/Rust VM/printer/JSON/native Call terminal prerequisites are
  closed at their owning manifests; PyVM remains outside.
- canonical Call writers through D1X, late callsite target rewrite retirement,
  duplicate projection validation, exact-target child, recursive compatibility
  shelf, and package-only BridgeReady are closed.
- FunctionMetadata owner split is closed at 718 lines; the 127-row consumer
  manifest remains the future sub-owner census authority.

Exact commits, test receipts, and per-row counts live in Git and the linked
investigation/archive owners, not in this rolling card.

## Reusable checks

```bash
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/run_row_guard.sh --only mir-call-d1b-targeted-variant-split
bash tools/checks/run_row_guard.sh --only mir-call-d1b-cataloged-affine-loan-lifecycle
# manifest row: mir-call-ingress-schema-lifecycle-guard
bash tools/checks/mir_call_ingress_schema_lifecycle_guard.sh
# manifest row: mir-call-global-target-b0-machine-census
bash tools/checks/mir_call_global_target_b0_machine_census_guard.sh
# manifest: tools/checks/manifests/mir_call_global_target_b0_machine_census.toml
bash tools/checks/mir_call_d1b_selected_normal_duplicate_projection_guard.sh
git diff --check
```

Cargo gates are run only by an accepted fast/closeout row. This guard-only fast
pointer does not turn a green guard into semantic implementation permission.
