---
Status: Closeout — MIR-CALL-B0-PROVENANCE-TOMBSTONE-R0
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
  B0は受理済み。canonical Globalは`Builtin(Print)`または
  `SameModule(FreeFunction | StaticBoxMethod)`だけとし、旧wireは入口で閉じる。

Source authority + canonical issuer:
  `print(expr)`のsource contract、exact same-module関数宣言、または
  owner-private compatibility resolverが構造targetを一回発行する。

Non-authority:
  raw text、alias map、physical symbol、function table、EffectMask、registry、
  `caller=None`、methodize、`args[0]`、optimizer/backend repair。

Fail-fast boundary:
  schemaはJSON rootを一回parseして選ぶ。unsupported/malformed/conflicting
  schemaとtarget関係不足はarguments、MIR、wire/backend effectより前にreject。

Smallest next slice:
  `MIR-CALL-B0-PROVENANCE-TOMBSTONE-R0`。Ingress guardの完了後に、
  旧B0判断の証拠とsuperseded_byをdocs-onlyで復元する。

Non-claims:
  schema selector実装、typed Global、observer/loan、Method/receiver、EffectMask、
  backend parity、performance、Loop/M8/M9、warning/dead-code cleanup。

Census boundary:
  production `Callee::Global` issuer -> optimizer/wire/all compiled core-schema
  consumers -> selected VM/native terminal. Census has 271 matching lines in
  143 `.rs` files (266 under `src`, five under `crates`) plus five matches in two
  compiled `.inc` files. Tests/non-selected backends are not semantic authority,
  but every compiled schema consumer remains in B1 closure.

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
  CanonicalV2Selected
  CompatibilityV1Selected
  CompatibilityMirV0Selected
  ProgramV0OwnerSelected
  CanonicalV2ParserUnavailableBeforeB1
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

1a. MIR-CALL-B0-PROVENANCE-TOMBSTONE-R0                     (now, closeout)
    inventory every section removed by 9bff1a1ff2 that carried review_source,
    A/B/C disposition, QualifiedStaticPayloadAbsent, TargetPayloadMissing, or
    terminal_role_split; map each to the current B0 owner with superseded_by /
    retained historical token, and record the user-supplied Pro review plus the
    six read-only audit roles integrated by 45bff917e3. No transcript is invented

1b. MIR-CALL-GLOBAL-TARGET-B0-MACHINE-CENSUS-G0             (guard-only)
    turn the finite family matrix, compiled-consumer owner inventory, Wpre/wire
    impact table, and exact print/1 attribution into one machine-readable manifest
    plus reusable fail-closed guard. Unknown owner/family/path and stale manifest
    fail; this guard is evidence only and grants no Wpre/B1 implementation permission

2. MIR-CALL-INGRESS-SCHEMA-SELECTOR-WPRE-I0
   parse JSON root once; select exact v2/v1/MIR-v0/Program-v0 once; delete raw
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
  `CHRONIC-MEASUREMENT-EXPECTATION-I0` ->
  `ASTCLEAN-STALE-GUARD-SUPERSEDE-R0`: reproduce and classify first, emit one
  expectation TSV/refresh guard second, then tombstone the stale per-script
  thresholds. Exact 334 is an observation, never an automatic production ceiling.
- performance, mimalloc, llvmlite, Hako converter, Loop/M8/M9, and physical-type
  follow-ups until `CURRENT_STATE.toml` reselects them.
- broad Context/metadata/port/barrel cleanup until Call R7.

Reopen only on a selected current row, a new production caller, or an accepted
owner-specific Decision. Parked code/tests never grant implementation permission.

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
bash tools/checks/mir_call_d1b_selected_normal_duplicate_projection_guard.sh
git diff --check
```

Cargo gates are run only by an accepted fast/closeout row. This guard-only fast
pointer does not turn a green guard into semantic implementation permission.
