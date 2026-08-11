---
Status: SSOT
Date: 2026-08-11
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

### Selected initializer materialization seam

pre-Builder semantic packageとLowerで初めて割り当てられる物理値は、互いを
再発行しない兄弟authorityである。selected callableがLoopへ入るときだけ、
次のrelationを一度co-sealする。

```text
installed-package selected semantic loan
  requires SelectedCallableSemanticRefV1::Dynamic
  + request-local completed local materialization
  + exact located Loop source/schedule
  -> one scoped selected Dynamic initializer admission
```

packageはcallable/Recipe/lifecycle意味を所有し、request-local stateは
`BindingRef -> ValueId`投影だけを所有する。located Loop boundaryは両者を
co-sealしてsole consumerへ渡すが、source semantics、Recipe、JoinSig、型を
再発行しない。Ordinary/Staticはこのcellを選択せず、既存の唯一のpost-success
TypeContext publicationとexact-MirType routeを保つ。Dynamicだけがpackage-loaned
programからbounded V2 routeへ入る。missing/foreign/duplicate relationはeffect前に
rejectする。新しいStatic/Dynamic closed sumやfamily arbitrationを作らず、
Dynamicを`MirType::Unknown`やlegacy GenericLoopで偽装しない。

selected Dynamicの最終source authorityはfinal exit-transaction co-sealから貸す
narrow initializer viewである。移行中のgeneric source seedは、cutoverでproduction
callerを0にするか、final programと一つのpackage-internal non-splittable co-seal
からだけborrow可能にする。二つのsource classifierを独立consumerへ公開しない。

admissionだけをcaller-zero productとして先行発行してはならない。最終co-seal
はnamed consumerと同じproduction replacement cellでissue/consumeし、旧selected
edgeを同時に削除する。selfhost header-result carrierとのbootstrap循環で
source-backed result/ABIが不足する場合、正本sourceへ明示result annotationを
置き、現在選択中のfrontendがnormalized header rowを一件だけ発行する。現在は
Rust final-source producer、selfhost parity後はHako producerをatomic cutoverで
選ぶ。同一compileで両方をadmitせず、frontend固有result receipt、body/Loop/MIR
inference、compatibility retry、fixture narrowingで循環を越えない。

明示`: i64`はdeclared-result syntax authorityであって、logical class `Dynamic`の
物理carrierを`Integer`としてReturnできる証明ではない。selected Dynamic corridor
の目標方式は`CHECKED-DYNAMIC-I64-ABI`で固定する。

```text
CHECKED-DYNAMIC-I64-ABI:
  boundary-local checked ABI/helper
  + producer-issued representation provenance

current unsupported behavior:
  RejectBeforeEffect

not selected here:
  global all-values-as-handles
  language-wide tagged representation cutover
```

bare `i64` bitsからraw integerとhandleを推測しない。runtime-polymorphicな物理値は、
producerが`ImmediateI64`、`IntegerBoxHandle`、またはprivateな
`TaggedCarrier(tag,payload)` provenanceを発行し、copy/rebind/PHI/currentを越えて
consumerまで保持する。欠落したprovenanceをReturn側がmetadata、runtime table、
TypeOp、sentinel-zero helperから修復してはならない。

この境界は二つの時刻へ分ける。

```text
pre-session demand:
  Completion sites + logical operands + required checked capability
  ValueId / BasicBlockId / MIRなし

session-local realization:
  exact demand row + producer representation + physical IDs
  -> normal exact i64 | terminal projection Fault
```

semantic ownerはsession IDsを持たず、session realizationはresult contract、return
site、logical operandを再分類しない。各required backend/representation cellは
`Direct | Checked | RejectBeforeEffect`のいずれかであり、fallbackは第四の分類に
ならない。projection Faultはresultを発行せず、cleanupとprimary/suppressed順序は
既存exit transactionが所有する。source annotationを理由に`MirType::Integer`を
後付けしてはならない。

### Bounded loop unification boundary

Dynamic full-body cohortがphysical-input/demandまで閉じた後も、common
physicalizerはRecipeからtransferを再推論してはならない。統一する核は
次の二つのcomplete protocolだけである。

```text
verified Recipe placement
  + JoinSig-owned logical transfer view
  -> prepared physical layout

complete operation/source-effect ledger
  -> complete physical demand
```

`physical_layout`/`recursive_after`は`LoopConditionV1`や`as_recipe()`から
Predicate/Jump/Backedgeを再構築せず、`segment_allocator`はRecipe条件を再走査
してHeader/Bodyを再分類しない。common physicalizerのstop lineは
`ReadyLoopAfterContinuationV1`であり、Callable profile-close、Tail、ABI、
Completionはcallable ownerが持つ。V1/V2を型変換するadapter、synthetic
`ItemKey`、名前・順序によるrepair、第二JoinSig/Recipe/physical plannerは
禁止する。

このcleanupは現在のDynamic-i64 representation design stopとは独立したparked
BoxShape laneであり、実行行を先取りしない。詳細なsubtaskとcaller-zeroの
退役条件は、active Dynamic cardの
`LOOP-UNIFICATION-AFTER-DYNAMIC-D0` sectionだけを参照する。

Durable order is representation demand, Loop authority cleanup, session-local
realization, then one production replacement. After the first production
cutover, semantic parity and performance promotion may proceed as sibling
proofs; every required sibling must be green before a selfhost producer is
activated. Exact task tokens and cleanup census remain in the active card.

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
2. named existing production callerはどれか
3. selected new ownerはどれか
4. 同じcommitで削除するold authorityはどれか
5. cutover後のfallback / retry / reselectionが0か
```

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

## Completion authority

MirBuilder再構築は、次がproduction graphで成立したときに着地する。

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
