---
Status: Open external consultation packet; not implementation authority
Date: 2026-07-29
CurrentStop: NORMAL-DEFAULT-ROOT-CATALOG-PREFLIGHT0-D0
Baseline: 3c7e6696c7
Exception: User-requested independently shareable question packet. Distill the answer into the rolling card, then archive or delete this packet.
ParentCurrentCard: docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
---

# NORMAL-DEFAULT-ROOT-CATALOG-PREFLIGHT0-D0 — 設計相談

## 依頼

次のselected production edgeを原子的に削除する、中立なtyped handoffを
選んでください。実装はまだ行いません。

```text
ExistingGeneralModuleCompatibilityV1::lower
-> session.builder_mut().build_module(ast)
```

回答では、success/rejection owner、source保持、実行順、atomic delete setを
確定してください。

## 現在地と不変条件

次の4 production siteは切替済みです。

```text
execute_mir_mode
execute_mir_json_minimal
LLVM source compiler
Wasm source compiler

-> NormalCompileRequestV1
-> NormalDefaultPublishedPipelineV1
-> one candidate / finish / publication
```

selected-normalからgeneric Legacy ingressへの到達は0です。VM、REPL、
Program JSON v0、Stage1、reference lanesは移動していません。

現在の `MirBuilder::build_module` は次の順を所有します。

```text
1. Programだけ VerifiedRawRootExpansionV1::from_program
   non-Programはpreflightをskip
2. prepare_module
3. ast.clone()をsnapshotとして一度作成
4. VerifiedSameModuleCallableDeclarationCatalogV1::seal_root(snapshot)
5. catalog install
6. lower_root_after_callable_catalog_install_v1(ast, snapshot)
7. finalize_module
```

必ず維持する契約：

```text
root expansion failure precedes prepare_module effects
catalog failure follows prepare_module effects
catalog install precedes indexing/body lowering
current snapshot clone count = 1
current reportable verification/result policy is unchanged
fallback / retry / reselection = 0
```

再利用可能：

```text
VerifiedRawRootExpansionV1::from_program
VerifiedSameModuleCallableDeclarationCatalogV1::seal_root
lower_root_after_callable_catalog_install_v1
prepare_module / finalize_module
ModuleBuilderInvocationSessionV1
```

再利用不可：

```text
OwnedRawSourceV1 / NarrowV1
  -> normal/default全域より狭い

InstalledPreloopStageBContextV1
lower_root_with_preinstalled_catalog_v1
  -> Stage-B固有authority

normal_source_plan whole-function products
  -> production caller 0でfreeze済み
```

`module_lifecycle.rs` は799行です。新責務はsibling moduleへ置きます。

## 推奨案 Candidate A′

```text
session open
-> selected-normal root sourceをseal
   Program:
     existing VerifiedRawRootExpansionV1::from_programを一度実行
   non-Program:
     current compatibility acceptanceを明示保持

-> sibling lifecycle owner
   -> prepare_module
   -> currentと同じ一回だけAST snapshot clone
   -> catalog seal/install
   -> existing port-aware root lower
   -> finalize_module

-> existing finish/publication
```

source ownerはowned ASTとprivate sealを保持します。borrowed expansion receiptを
保存しません。

snapshotは次の二役に限定します。

```text
original owned AST = source/rejection retention + catalog snapshot
one cloned AST     = consuming root-lowering input
```

これによりcatalog、lower、finalize failureでも元sourceをdiscard-only
rejection ownerへ保持できます。追加clone、AST reconstruction、reparseは0です。

推奨failure stage：

```text
RootExpansion
PrepareModule
CatalogSeal
CatalogInstall
RootLower
FinalizeModule
```

rejection terminalは `stage / error / discard` のみです。

## 不採用候補

```text
B. catalogをprepare_module前にseal
   -> failure precedenceが変わる

C. NarrowV1またはStage-B ownerを流用
   -> grammar narrowingまたはroute authority混入

D. 新しい名前でbuild_moduleへforward
   -> residual/old authorityが減らない
```

## Atomic acceptance

```text
Decision candidate = A′
Ceremony           = T2
New grammar        = 0
Result policy delta= 0
Fallback/retry     = 0
New per-row guard  = 0

selected:
  ExistingGeneralModuleCompatibilityV1
  -> .build_module(ast)
  = 0

new:
  ExistingGeneralModuleCompatibilityV1
  -> verified root/catalog lifecycle handoff
  = 1

explicit compatibility:
  legacy_candidate_session -> build_module(ast)
  remains unchanged

global build_module caller count = non-claim
```

単なるrename/forwarding facadeにはreplacement creditを与えません。

## 回答してほしい項目

```text
1. Candidate A′を採用してよいか
2. exact success/rejection product
3. Program/non-Program source partition
4. one-clone snapshot law
5. six failure stagesとfailure precedence
6. sibling module/APIの責務境界
7. atomic old-edge delete set
8. focused parity/failure/reuse evidence
```

Hard stop：

```text
normal/default grammar narrowing
catalog sealをprepare_moduleより前へ移動
追加AST clone、reparse、source reread
NarrowV1またはStage-B authority流用
compilerへmutable Builder internals公開
second candidate/publication path
fallback / retry / family reselection
selected build_module edgeの残存
module_lifecycle.rsが800行以上
new per-row guard
```

回答はこのpacketへ直接追記せず、選択内容を返してください。採用結果は
`mirbuilder-inplace-replacement-current.md` の四ブロックbriefへ圧縮します。
