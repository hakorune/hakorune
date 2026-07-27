---
Status: SSOT
Date: 2026-07-28
Decision: MIRBUILDER-FINAL-PIPELINE-v1
Scope: MirBuilderと直前・直後の境界を含む最終production authority
Related:
  - docs/development/current/main/design/recipe-first-entry-contract-ssot.md
  - docs/development/current/main/design/recipe-tree-and-parts-ssot.md
  - docs/development/current/main/design/mirbuilder-inplace-replacement-policy-ssot.md
  - docs/development/current/main/design/compiler-pipeline-thinning-ssot.md
  - docs/reference/language/function-exit-and-entry-result.md
  - docs/development/current/main/investigations/function-exit-f1-draft-seal0-s0-execution-task-2026-07-25.md
  - docs/development/current/main/investigations/normal-source-plan0-design-stop-2026-07-26.md
  - src/mir/builder/README.md
---

# MirBuilder Final Pipeline

## Decision

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

`MIRBUILDER-INPLACE-REPLACEMENT0`は、この最終形へ現在のproduction
MirBuilderを移す方法である。replacement cellやstructural ratchetは移行手段
であって、最終architectureの代わりではない。

## Authority map

| Boundary | Owns | Must not own |
| --- | --- | --- |
| Resolver | `BindingId` / `ScopeId` / `RegionId` / callable target / source provenance | MIR emission、route retry |
| Observation / Facts | sourceとcontrol-flowの観測結果 | MIR mutation、hidden acceptance policy |
| RoutePolicy / Recipe | 一度だけ行うroute選択とlowering義務 | real `ValueId` / `BasicBlockId`、publication |
| RecipeVerifier | omission、duplicate、coverage、exit、carrier、merge契約 | repair、別Recipeへのfallback |
| Verified-plan Lowering | CFG、operand、Binding SSA、edge、PHI materialization | ASTからのroute再判定、別ownerへのretry |
| FunctionDraftSeal | exit、PHI closure、type/signature/metadata、session closeのprepareとcommit | Recipe再解析、source route選択 |
| Draft Collector | `CompletedFunctionDraftV1`の完全集合 | open/prepared draftの公開 |
| Module transaction | candidate moduleのsuccess-only atomic publication | partial insertion、failure後のresume |

Facts、Recipe、Verifierの詳細契約は
`recipe-first-entry-contract-ssot.md`と`recipe-tree-and-parts-ssot.md`が持つ。
Function exit semanticsは`docs/reference/language/function-exit-and-entry-result.md`
が持つ。DraftSeal、collector、module publicationの現行実装とaccepted
evidenceはRelatedに列挙したsource owner／taskにある。この文書はそれらを
結ぶ最終pipeline authorityを所有する。

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
準備するが、physical `Return`の唯一writerは
`PreparedFunctionDraftSealV1::commit(self)`である。

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
Facts / Recipe / Verify decision authority                       = one each
unverified direct lower                                           = 0
Lower-side AST route redecision                                   = 0
DraftSeal-side Recipe / route redecision                          = 0
physical Return writer on canonical function path                 = 1
CompletedFunctionDraft-only collection                            = yes
partial module publication                                        = 0
fallback / retry / profile reselection                            = 0
selected old production owner / facade / edge                     = 0
full accepted corpus / backend parity                             = green
```

pack counters、replacement ledger、five-cell LOC、source/test ratchetは、この
semantic completionへ到達するための進捗・増殖guardである。いずれも単独では
completion authorityにならない。

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
