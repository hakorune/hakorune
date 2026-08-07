---
Status: Historical
Role: Supporting
Decision: superseded-as-global-pipeline-authority
Scope: 旧BoxShape pipelineで得たParser・ValueId・PHI境界の補助メモ。現行pipeline順序のauthorityではない。
Related:
  - docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md
  - docs/development/current/main/design/recipe-first-entry-contract-ssot.md
  - docs/development/current/main/design/binding-ssa-first-control-lowering-ssot.md
  - docs/development/current/main/design/phi-lifecycle-ssot.md
  - docs/development/current/main/design/joinir-observation-layer-ssot.md
---

# Compiler Pipeline — Historical BoxShape Note

## Authority redirect

Canonical source ingressからatomic MIR publicationまでの唯一のglobal
pipeline-order authorityは次である。

```text
docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md
```

この文書が以前示した、

```text
Parser
-> RecipeStore
-> Observation Views
-> ShapeDecider
-> Verifier
-> SSA/PHI Lower
-> Codegen
```

は、当時のBoxShape責務を説明するhistorical modelである。現行の実行順、
portable Recipe、DraftSeal、collector、module publicationを規定しない。

特に旧`RecipeStore`はASTを保持するBuilder-side source envelopeであり、
AST-freeなportable Recipe authorityではない。両者を同じ`Recipe`として
扱ってはならない。

## Retained boundary lessons

この補助メモから引き続き有効な教訓は、owner別の現行SSOTへ従属する。

### Parser / observation

- ParserはsourceからASTとsource locationを作る。
- AST rewriteで受理形を作らない。
- 条件や更新の正規化はanalysis-only viewとして観測する。

現行authorityは`joinir-observation-layer-ssot.md`と
`recipe-first-entry-contract-ssot.md`である。

### ValueId / Binding SSA

- `ValueId`はfunction scopeに閉じる。
- allocationやtype annotationだけではdefinitionにならない。
- emit失敗を握りつぶして未定義のghost `ValueId`を返さない。
- provisional PHI destinationはphysical PHIが成立するまでpartial binding
  truthとして公開しない。

現行authorityは`binding-ssa-first-control-lowering-ssot.md`と
function-local session契約である。

### JoinIR / PHI

- observationやRecipeはlogical ports、edge、merge義務までを表す。
- real predecessor、`BasicBlockId`、`ValueId`、PHI materializationはverified
  lowering sessionが所有する。
- backendやVMは構造をrepairしない。

現行authorityは`phi-lifecycle-ssot.md`、portable Recipe reference、
`mirbuilder-final-pipeline-ssot.md`である。

## Retirement condition

上の固有教訓がowner SSOTへ完全に移り、live backlinkが0になった時、この
supporting noteをarchiveする。それまでもglobal pipelineの判断には使わない。
