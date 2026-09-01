---
Status: Follow `docs/development/current/main/CURRENT_STATE.toml`; this rolling file is not the active pointer
Date: 2026-09-01
Decision: MIRBUILDER-INPLACE-REPLACEMENT0
Policy:
  - docs/development/current/main/design/mirbuilder-inplace-replacement-policy-ssot.md
North star:
  - docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md
Call owner:
  - docs/development/current/main/design/mir-canonical-callsite-lane-ssot.md
Active card:
  - use `CURRENT_STATE.toml.latest_card_path`
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

## Current restart pointer

`CURRENT_STATE.toml` is the sole mode and active-row authority. The Method
corridor is Exhausted: Rust exact-1 methodize, the Global-to-`Method(None)`
reissuer, the receiverless Builder terminal, `current_static_box + has_method`,
and the caller-zero shared `Resolved` corridor are retired. The final RET0
landed at `598530d23b`: RawRootMain ordinary direct injection now reaches
`RawRootMainRetired` before effects, while the Resolved variant/consumer and
its exclusive resolver/recovery/name-Const/tail-env owners were physically
deleted. Cataloged/AppMain and special precedence remain green; no replacement
target was issued.

Two independent read-only audits then corrected item 15: its old three labels
were semantic families, not an exhaustive file list. The finite Rust Builder
boundary has nine production families plus one test-only outside owner; physical
thunk, default-on `rewrite/known`, ordinary-new birth, and cataloged provider
are separate blockers. Only the exact static receipt family has a closed mapping.
The exact static-receipt I0 landed at `e5120589dc`: its three production callers
already owned `CanonicalSameModuleCallableKeyV1`, projected the typed Global
target before argument/recipe descent, and passed that target to a
consumer-only receipt terminal. The wider SameModule parent remains
blocker-open; ordinary static, CorePlan, operators, rewrite, thunk,
birth/provider, VM/backend, and Call schema remain closed until each has its
own exact issuer or typed pre-effect retirement.

Current execution:
`CURRENT_STATE.toml` temporarily selects `LEGACY-TESTS-RETIRE-R0` as a
shrink-only T0 row. The selected-C DeclaredInstance admission remains a
CoverageMissing design stop: its exact source-backed tuple census is zero, so
no Method(Some), backend view, fixture, receipt, or second pipeline may be
opened. The legacy row owns only the explicitly disabled pre-JoinIR test
feature and returns here after its default-feature baseline is unchanged.

The canonical backend decision remains source -> published MirModule ->
lossless borrowed typed view -> selected backend; direct package-to-Hako is
ParkedSealed. `legacy-tests` has no in-repo caller or CI owner; the retirement
delete set is the four cfg barrels, 34 roots, and nine exclusive support files.

Current execution history:
the exact lib receipt now records 7554/7386/139/29 after the
production-neutral PlannerContext mismatch test retirement at `3e4f66ee82`
and the earlier binding-shadow retirement at `70b061f8f2`. The 139 failure
names/SHA remain unchanged; the exact-match PlannerContext successor and
three retained negative paths each passed 1/1. The existing MIR builder
CallTarget owner guard was rehomed at `cde0250481`: its retired
`src/mir/builder/calls/method_resolution.rs` requirement is gone, while root
ownership and current policy consumers remain checked. Full quick remains
separately owned and is not claimed green; it now reaches the separate
K2-wide OSVM first-row guard. The role-aware CorePlan
varmap census remains closed at raw=22, test-only=16, disconnected=1, live=5,
canonical=1, reseal=4, remove_or_clear=0. The receiver crosswalk at
`dc68fa2910` remains valid and
Global/manual-prefix output is unchanged. The bounded verification row
`MIR-BUILDER-CALLTARGET-GUARD-REHOME-R0` is landed: it rehomes the existing
guard to resolver/boxcall/preflight policy consumers without restoring the
retired file or changing compiler semantics. Its closeout returned to the
selected-C CoverageMissing design stop. Only after the selected-C
caller-first boundary gains one exact live `me.method` source/wire/object tuple
may Method(Some) admission resume. The selected physical owner is the existing
daily ny-llvmc Boundary after canonical MirModule publication. The former
direct package-to-Hako physical-Recipe proposal is superseded and ParkedSealed
as a future post-publish backend replacement; it is not a second pipeline.
The behavior-neutral
`MIR-BUILDER-EXTERN-ROUTE-SPEC-CATALOG-LOOKUP-BOXSHAPE-S0` landed at
`35f59702b5`: it moved only the derived route lookup block into a private
child and left the 47-row catalog, public API, and generated/C behavior
unchanged. The fixed baseline-refresh row landed at `ee886f4cc1`: three
fixed-stack observations and the comparator agree on 7554/7386/139/29 with
unchanged failure names after the exact eight-test loop-if-exit duplicate
cohort was retired at `ed1332f16d`. The reconciliation now stops at the
separate K2-wide OSVM first-row guard: its four existing boxcall compile tests
fail at explicit-extern/missing-resolved-relation, so whole quick is not
claimed green. The common owner retains nine contract tests; the
normalized-shadow production route and compiler semantics are unchanged, and
the current pointer is back at the selected-C CoverageMissing design stop.
This cleanup reduced the
daily root without issuing semantic authority; it did not reopen selected-C or
create a second pipeline. The later
production work will verticalize and delete that
family's old Global/manual-prefix and selected-C
edges in the same bounded series. `variable_map`, param0, `args[0]`, numeric
zero, AST reread, names, registry, JSON/C/backend metadata, fallback, and retry
remain non-authorities. The daily backend has a generic Method(Some) consumer,
but the Counter/minimal candidate is not a live source-backed caller: it enters
`Compatibility(AST)`, has no callable-source/package/TypedObjectPlan tuple, and
normal source-backed runs stop at `ExactSourceChanged` (or the later
resolved-binding publish-preflight imbalance). The existing D0 census is
therefore zero live tuples. ParkedSealed this vertical without another D0,
receipt, fixture, adapter, or guard; reopen only from an existing published
MirModule source caller with one lossless wire/object tuple. The daily backend
choice remains valid, but no Method(Some) implementation permission is open.
Whole-lib known-red health is separate. The
independent bounded row
`MIR-CSE-SAME-BLOCK-STATS-DETERMINISM-R0` landed at `25ab8fb58a`; it owns only
existing MIR CSE and makes no selected-C claim. The `.inc` no-growth baseline
prerequisite landed at `e3cfa78488`; the original baseline runner landed at
`1115d70687`, and its exact receipt refresh landed at `484d58585f`; whole quick
is still red at the separately owned CorePlan boundary guard.
The temporary FastMem verification prerequisite landed at `39d5188d9d`: the
test-only order-independent membership assertion passes in ten exact runs and
the 20-test module; whole-lib red remains separate and unaccepted.
The physical-thunk and selected-exact CorePlan audits remain `ParkedSealed`;
the physical-thunk closure is specifically `RelationPresentIssuerMissing`:
the two production callers and one test helper are reconciled, but no exact
same-session `main/0` Callee issuer exists. Ordinary-new is no longer an
unexplained carrier dead-end. Its bounded exact constructor cutover landed at
`d81d91d800`: the parser constructor product, package recipe, shared ABI, and
selected-normal consumer now replace birth-text/AST-scan/class-arity
reconstruction. No-claim compatibility and every other SameModule family
remain closed. The cataloged-provider audit leaves bare `exit` as `NoIssuer`;
the later `LANG-PANIC-TERMINAL-FAULT-D0` decision gives exact bare `panic/1`
an accepted terminal-Fault target (production 0), although no live issuer
exists yet. The canonical Call SSOT gives bare `error` and `now` unsupported
pre-effect contracts, recorded in one-name D0 cards. Qualified Extern, Math,
and GC owners stay separate. The parent finite producer census remains closed
while this contract boundary is reconciled. Bare `error` retirement landed at
`69680b983f` and bare `now` retirement landed at `3e35e4f39c`, both through the
existing pre-effect Rejected terminal; bare `panic` is a parked terminal-Fault
lane and bare `exit` remains a separate NoIssuer lane.
The user selected retirement for only the legacy
`NYASH_BUILDER_OPERATOR_BOX_{ALL,ADD,COMPARE}_CALL` route. Read-only worker
audits closed the boundary at three compiler ingresses, four target publishers,
six structural source ingresses, one prelude coupling, and finite repo-owned
writers. `NYASH_OPERATOR_BOX_*`, `--dev`, runtime/observe/adopt behavior, and
the existing direct arithmetic/comparison/unary MIR remain separate and live.

`StaticCurrentOwner me.method` landed at `2b7b3e7489`. True
`InstanceBoxMethod`/`DeclaredInstance` is now the selected design blocker and
retains ordinary `me.method` syntax. Its target relation is resolver-owned;
the package may only co-seal result/effect/full-lane contracts. The clean MIR
is `Method(Some(receiver))` plus N explicit source arguments, and the selected
C path remains one direct same-module call through the daily Boundary. The
fixed successor series is: borrow-only package physical view; caller switch to
Method(Some) before arguments; lossless route/effect/object preservation and
strict pure-first consumption; end-to-end proof plus selected old-edge/seed
retirement. No package-view commit may close without the rest of that series.

The bounded row
`MIR-CALL-SAME-MODULE-ORDINARY-STATIC-LEGACY-COMPAT-RETIRE-I0` landed at
`b1755febfb` with terminal-type and guard-ordering corrections at
`7c834d021a`. It retires only the unissued generic `StaticReceiver`,
`StaticThis`, `Me` `StaticFallback`, and `LoweredGlobal::Static` compatibility
fallbacks before argument effects. Exact Cataloged handoffs, scalar/inline
owners, qualified `Math`, Env/Extern, `Method(Some(receiver))`, and
`LoweredGlobal::Instance` remain live. It created no issuer or receipt and did
not alter VM/backend, JSON, Call schema, fallback/retry, or guard cleanup. The
SameModule parent returns to `design_stop`; any need to change a preserved
owner reopens the parent instead of widening this row.

Three read-only audits reconciled the Generic CorePlan child with the final
Loop architecture. The production-live legacy registry route
`LoopRouteId::GenericLoopV0/V1` is an internal execution route whose retirement
is already owned by the portable Loop M10b cutover; it is not a new language
decision. The source-backed canonical V1 path is a separate retained semantic
owner and must not be deleted with the registry route. The Call child is
therefore `ParkedSealed` until the 389-row accepted corpus has portable-owner or
approved typed-reject coverage and the M8/M9/precutover deletion manifest is
ready. No immediate GlobalCall-only or whole-loop retirement row is open.

The user-selected rewrite/known policy I0 landed at `7a6fb9e2db`: the optional
Known/Unique/equals instance-to-Global writers and selector pins are retired,
while the existing typed `Method(Some(receiver))` owner and explicit early
str-like route remain. The historical ON/OFF probes are evidence only, not a
parity gate; no new Global issuer, post-effect reject, fallback, or second
resolver was added. The ordinary-new exact-constructor cutover remains landed
at `d81d91d800`, and the parent design stop resumes.

Guard cleanup is a separate parked lane. The only currently proven
`SupersededDelete` was the stale whole-writer overlay removed by the landed
type-fact S0; the nine live checks are kept and five are rehomed. Future guard
deletion requires a finite invocation/profile census, caller-zero or an equal
or stronger successor, focused positive/negative proof, and a reopen trigger;
age, pilot-only status, or a red result alone never authorizes deletion.
A fresh read-only registry census on 2026-08-30 found no additional
`SupersededDelete` candidate (117 registered rows and 2646 unregistered
surfaces). Unregistered, pilot-only, and compatibility-navigation surfaces
remain `Keep`/`unknown_retain` until an owner, successor, and reopen trigger
are proven; no new guard-retirement row is opened. The parked follow-up is
deliberately small: when a guard family is next touched, record its finite
entry/profile/consumer boundary; only a proven `SupersededDelete` may remove
its guard, test, and index claim together, with focused proof and a reopen
trigger.

Preflight found that the inherited 786-line type-fact composite guard is
pilot-only and already red before operator work. Its 15 independent checks
are now fate-classified as `Keep=9`, `SupersededDelete=1`, `Rehome=5`,
`Unresolved=0`. The sole deletion is the stale whole-repo writer overlay with
20 drifted paths. The five live red checks move to current owners: the two
Const emission forms, literal lowering, production direct-call emission,
generic call receipt/post-success, and typed Map replay. The immutable fixture
and five-row anchor sweep remain. The bounded S0 landed at `24db147172`: the
stale overlay was removed, five live checks were rehomed, the parent is below
760 lines, and the retained guard is in quick-static; the composite guard
itself was not deleted.

With that proof surface green, the next atomic
`MIR-CALL-SAME-MODULE-OPERATOR-CALL-RETIRE-I0` parses all three selectors once
across three conceptual ingress families and five physical public methods,
then deletes the publishers, downstream reads, Builder prelude OR, and
repo-owned Builder writers together. Validator-only, publisher-only,
whole-guard deletion, and runtime-lane changes are not permitted.

## Historical rolling context (non-authoritative)

Closed detail for earlier VM, smoke, Wpre, Global-target, and guard phases is owned by Git history and the linked phase/archive records. It is not current selection authority. The current lane, blocker, selected backend role, and next cutover are stated above and in `CURRENT_STATE.toml`; do not restore historical prose here.

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

`Print`はexact `print/1`だけ。bare `panic/1`はCall/Externではなく、共通exit
transactionを通るterminal Faultのaccepted target（production 0）。bare `exit/1`
は未発行の別lane。bare `error`と`now`はrejectし、explicit providerだけがExtern
になる。mathはMethod、GCは未実行Global producerをretireしてrejectする。
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
    exact print/1 retains one guarded Global publication for B1; panic/1 leaves
    the Call lane for the parked terminal-Fault series; exit/1 stays unissued;
    bare error/now/println reject; explicit declared providers remain Extern

12. MIR-CALL-D1B-ALL-LINEAGE-PRE-EFFECT-RETIRE-R0
   the finite preflight classification is closed: four named compatibility
   origins remain explicit and generic UnclassifiedSource rejects before
   arguments. Their downstream Resolved/recovery descendants are outside this
   boundary and remain live; global caller-zero/deletion is not yet claimed

13. MIR-CALL-D1B-CATALOGED-SOURCE-RELATION-AND-AFFINE-LOAN-I0
    (unlocked only after the B1 structural Callee/CallTarget cutover below)
    exact site/owner/catalog co-seal -> non-empty stack-owned loan -> take_once
    -> arguments once -> Call once -> residual zero; direct CatalogedTargeted
    payload deleted in the same cell after late recovery is already zero

14. MIR-CALL-METHOD-CORRIDOR-R0
   complete/Exhausted. The Rust selector/reissuer, static-none producer, Builder
   terminal, named compatibility origins, and caller-zero Resolved corridor are
   retired through 598530d23b. Stage1/Hako/JSON and VM/backend remain separate
   named owners; this row claims no terminal/schema retirement outside Rust Builder.

15. MIR-CALL-SAME-MODULE-ALL-PRODUCER-DISPOSITION-R0
    blocker-open parent with nine production families: exact static receipt,
    ordinary static terminal, generic and selected-exact CorePlan, env operators,
    physical thunk, rewrite/known, ordinary-new birth, and cataloged provider.
    Exact final-module declarations retain one guarded publication; authority-free
    publishers reject before effects. The active bounded child is
    MIR-CALL-SAME-MODULE-STATIC-RECEIPT-TARGET-BEFORE-ARGS-I0: exactly three
    canonical-key callers project a typed target before descent, and the receipt
    terminal stops reconstructing it from owner/name/arity. No formatted text,
    plan `func: String`, env, symbol, candidate/header lookup, or retry is authority.

16. MIR-CALL-GLOBAL-TARGET-DEAD-TEXT-CALLSHAPE-S0
    move only the call-shape matcher out of the 790-line owner; no behavior change

17. MIR-CALL-GLOBAL-TARGET-B1-CURRENT-HEAD-C0
    enumerate the finite surviving exact String publications and every compiled
    consumer by owner/action; arbitrary publisher/recovery/methodize count is zero.
    Any new hole inserts an owner-specific S0/R0, then C0 reruns; only exhausted C0
    with all remediation rows closed may open the structural cutover required before
    D1B target issuance

18. MIR-CALL-GLOBAL-TARGET-B1-CUTOVER
    add the serde-free defs type; atomically change both `Callee::Global` and
    `CallTarget::Global`; type the retained exact publications; add bounded v2
    codec, sole projection and one MirModule lookup; adapt/delete/isolate every
    compiled consumer without formatter, reparse, registry, fallback, or retry.
    B1 corrective R0 must first keep the explicit `vm-reference` feature
    compiling, enforce the finite Global-family disposition, and name the nine
    transitional selected-symbol owners. This row is a prerequisite for item
    13; a new D1 String issuer is forbidden.

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

## Guard contract and retirement queue (2026-08-27)

This is the sole successor queue for the existing
`GUARD-SURFACE-CONSOLIDATION-D0`; do not create one card or shell guard per
finding. It does not preempt the selected force-hv1 design stop.

The 2026-08-27 audit's three hygiene findings and the phase2160 accounting
correction are taskized in
`docs/development/current/main/investigations/guard-execution-index-force-hv1-nongrowth-closeout-d0-2026-08-27.toml`.
The RawLegacy C I0 itself is landed. The selected bounded order is
contract-graph freeze -> existing inventory-row ledger check -> navigation
tombstones -> chronic scope -> Rust token scanner (landed)
-> scope reconciliation D0 -> tracked observation receipt -> site-owner map D0 -> one scope-labeled expectation TSV -> closeout
guard co-registration -> phase2160 legacy/support LOC attribution. The
force-hv1 leaf observer stays shell-body-only, no new force-specific TSV or
God guard is introduced, and none of these rows authorizes compiler, VM, or
backend behavior changes.

The landed RawLegacy nested Main C I0 has one queued same-family closeout
hygiene follow-up in
`docs/development/current/main/investigations/runtime-box-rawlegacy-nested-main-retire-i0-2026-08-27.toml`:
the query-before-clone shape is already confirmed; after the guard frontier,
sync the dispatcher/RawInvocation comments, record the no-more-fate-methods
stop line, and perform the finite `lower_static_main_box` caller census. This
follow-up cannot reopen the retired edge or broaden RawLegacy scope.

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
Smallest next slice: CHRONIC-MEASUREMENT-SITE-OWNER-MAP-D0;
  the 185-row evidence matrix is complete (A=79, B=46, C=60), A1 pinned-reference resolution is landed, and A2 must classify 697 owner/evidence references and normalize roles/successors before map authoring.
  Keep the tracked map, expectation pinning, stale-guard supersession, and source deletion as separate later stages.
Non-claims: no full registry migration, quick-static activation, bulk chmod,
  compiler behavior change, or grep/count-authorized deletion.
```

Current source-backed observation at `b4edff4c78` is 3,754 tracked check paths;
the finite registry graph has 112 flattened rows. The ratchet-eligible
immediate-child public surface is 2,740 tracked `*_guard.sh` paths, with 19
direct command-target edges and 74 typed wrapper-alias edges (93 mapped when
the edge kinds are kept separate); 2,647 eligible paths are currently
unmapped. `quick-static` has 24 declared rows and no profile caller. These are
different denominators and must not be reported as one coverage percentage;
the profile name is not execution evidence. The 93/2,647 values are the D0
baseline. `GUARD-REGISTRY-RATCHET-I0` landed at `9b49907937`: the current
tuple is now 20 direct targets, 74 typed aliases, 94 mapped paths, and 2,646
unmapped; the PR-only ratchet reports zero new unmapped paths and zero mapping
loss against an explicit base. The four navigation dangling names remain a
separate ledger row.

Required order:

1. `GUARD-CONTRACT-GRAPH-D0` — accepted: the two-plane model, finite public
   eligibility set, direct-command versus typed-alias edge kinds, explicit
   PR-base/absolute-measurement behavior, contract-v1 metadata, and the
   existing six inventory dispositions are frozen. Generated reverse output
   never becomes authority.
2. `GUARD-REGISTRY-RATCHET-I0` — landed at `9b49907937`: the existing
   inventory owner now registers the already-CI-reachable
   `ci_feedback_tier_policy_guard.sh`, rejects new unmapped public guards
   against an explicit PR base, and wires only this structure check to required
   PR CI. It does not run registry member commands or infer `HEAD^`.
3. `GUARD-NAVIGATION-TOMBSTONE-I0` — landed at `595882c065`: the compatibility
   block stayed byte-stable, four owner-reviewed tombstones resolve the former
   dangling names, and the inventory owner rejects missing/duplicate/conflicting
   navigation names without entering `guard_rows.toml` or executing members.
4. `CHRONIC-MEASUREMENT-SCOPE-D0` — accepted: finite panic/dead_code
   boundaries, independent compile-domain/role axes, and the shared
   expectation TSV contract are frozen. It does not authorize deletion.
5. `CHRONIC-RUST-TOKEN-SCANNER-I0` — landed at `8d2bcf0398`: the standalone
   `rust_source_topology` check crate emits a syn/span-based read-only report;
   malformed/unsupported/unknown ranges fail closed. Its counts are not a
   production baseline and no deletion is implied.
6. `CHRONIC-MEASUREMENT-RECONCILIATION-D0` ->
   `CHRONIC-MEASUREMENT-SITE-OWNER-MAP-D0` -> `CHRONIC-MEASUREMENT-EXPECTATION-I0`
   — scope split measured; track the 185-row observation receipt, then freeze
   canonical range/attribute key, provenance, tracked refs, and classification-only
   states before assigning site owners. No TSV, stale-threshold, or source deletion.
7. `ASTCLEAN-STALE-GUARD-SUPERSEDE-R0` — use one token-aware scanner and one
   per-file expectation TSV. Exact-form 334/111 is diagnostic; inclusive
   attribute grammar currently observes 351/126 and is the required D0 scope.
   Remove the 13 obsolete source-wide numeric clauses rather than relaxing them;
   retain living leaf checks, and explicitly supersede ASTCLEAN-007 by
   ASTCLEAN-013 before deleting 007.
5. `GUARD-MANIFEST-MODEL-R0` -> `GUARD-REGISTRY-HEALTH-R0` — BoxShape-consolidate
   manifest loading, define argv-derived executable semantics, and classify the
   44 non-manifest hako-alloc closeout wrappers as register, consolidate, retire,
   or retain. No mass chmod follows from the current 0644 baseline.
6. `GUARD-REVERSE-INDEX-I0` — extend
   `tools/docs/guard_surface_inventory.py` with guard/invariant/path queries and
   forward/reverse edge checks. Migrate two existing rows first. Keep
   `check-scripts-index.md` human-facing and keep its legacy compatibility block
   byte-stable until its callers reach zero.
7. `SOURCE-LINE-BUDGET-CENSUS-D0` -> `SOURCE-LINE-BUDGET-SPEC-I0` — rederive
   target, threshold, kind, and focused caller group; do not freeze the stale
   estimate of 77 guards. Move one MIRBuilder family to the existing typed-spec
   runner, then make old focused guards delegate exactly once before removing
   their inline `wc`/threshold authority.
8. `GUARD-FAMILY-RETIREMENT-R0` — process bounded families only. A guard may be
   removed when its invariant is owned by a named successor or the guarded route
   is physically impossible, all CI/manifest/parent/docs callers are zero, and
   the supersede/retirement edge is recorded. Otherwise it remains
   `unknown_retain`; inactivity and non-registration are not deletion evidence.
9. `QUICK-STATIC-QUALIFY-D0/I0` — finite-classify all 19 rows for side effects,
   latency, and current green status. Only after qualification may the existing
   anti-wiring contract close and the whole profile become a CI entry.

Acceptance for the series is monotonic: new public unregistered guards = 0;
unknown inventory does not grow silently; each selected family loses its local
duplicate expectation in the same slice that gains central coverage; deleted
guards have named successors and caller-zero; generated forward/reverse edges
round-trip; and no family migration changes compiler or test semantics.

The 2026-08-29 Call proof audit is queued without preempting the landed
RawScriptRoot old-edge deletion. The next production decision is the bounded
RawRootMain origin only; run one bounded
`MIR-CALL-GUARD-ACTIVE-SURFACE-PRUNE-R0` series: retire landed lifecycle-phase
handlers to explicit `superseded_by` tombstones instead of making old lanes
replayable at HEAD, keep only the active origin-retirement family, machine-check
that every changed test is covered by a nonzero focused filter, and add the two
missing `SiteCoverageMismatch` negatives. Then return immediately to the next
origin retirement. `legacy-tests` is a separate parked retirement census; its
two red `mir_static_box_naming` tests are not repaired or promoted to CI unless
a selected current acceptance owner is found. Exact evidence and stop lines
live in the current Method manifest.

## Short closed tail

- normal-root identity/forest gates and the installed App Main FreeStatic
  source-keyed affine handoff are closed; this does not generalize the loan to
  the four named compatibility descendants.
- JSON-v0 Call target resolution and Program late target rewrite are closed.
- Callee operand/use/escape/ownership/query projection rows are closed.
- selected optimizer/Rust VM/printer/JSON/native Call terminal prerequisites are
  closed at their owning manifests; PyVM remains outside.
- canonical Call writers through D1X, late callsite target rewrite retirement,
  duplicate projection validation, exact-target child, and the App Main
  package/loan/raw-port handoff are closed. Named Resolved/recovery descendants,
  Method ingress policy, Method(None), and final Call schema remain open.
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
