# CUT0-I0 ROOT0-CANON0 未決定質問

Status: **worker棚卸し済み — 回答待ち**
Date: 2026-07-22

## 背景

`COLLECT0-S0/BATCH0` で canonical source/header/catalog と physical
collector/receipt の co-seal は証明済みです。しかし、canonical single と
callable batchを `CompleteInvocationV1` へ昇格する route-specific product、
recursive capability の唯一owner、source-derived drain inventory は未実装です。

既存の `CompleteInvocationV1` と `ModuleLoweringInvocationStateV1` は
`MainPending -> MainCaptured -> Complete` 固定のRaw/Main専用stateです。これを
canonicalへ流用すると、canonicalに synthetic `main`/`condition_fn` を混入させ、
後続DRAIN0のcaller inventoryを温存するため、CANON0では流用しません。

## 質問

### Q1 source authority binding

現在の `ModuleInvocationTokenV1` は family/source witness だけを持ち、
canonical header/planそのものを所有していません。canonical completionで
foreign header/planを拒否するauthorityをどこで封印するか？

1. **C-prime（推奨）**: preflight済みの private canonical source continuation
   （verified plan + exact header + family）をtoken brandから一度だけ生成し、
   completion productまで保持する。tokenの既存5-family APIは壊さず、source
   authorityだけを別non-Clone productとして追加する。

2. family-only tokenのまま、canonical co-seal terminalでheaderを検証する

3. headerをcompletion時に再取得して照合する

### Q2 canonical completion product

どのowner chainでcanonical completionを閉じるか？

1. **C-prime（推奨）**: route-specific productを新設する

   ```text
   CanonicalSingleCollectedInvocationDraftSetV1
     -> CanonicalSingleCompleteInvocationV1

   CallableBatchCollectedInvocationDraftSetV1
     -> CallableBatchCompleteInvocationV1
   ```

   source continuation、実collector、collector-issued receipt、exact root
   witnessをby-valueで一度だけ保持する。

   receiptは `InvocationBranded::from_source` を外から呼んで後付けしない。
   `PreparedCallableCollectorBatchV1::collect_all_branded` など、実collectorの
   brandを内部取得する発行terminalだけをproduction候補にする。

2. Main専用 `CompleteInvocationV1` をenum拡張してcanonicalへ流用する

3. collected set とcomplete productを分離し、receiptを別ownerへ複製する

### Q3 recursive capability authority

`BindingSsaRecursive` の capability marker をどこで一度だけ封印するか？

1. **C-prime（推奨）**: sealed family/source plan が shell constructorへ factを渡し、
   recursiveだけ `CanonicalRecursiveCallableModuleCapabilityV1` を installして
   source・shell・complete productでco-sealする。Acyclicにはmarkerを作らない。

2. source proofだけに保持し、drain時にshell metadataを再観測する

3. shell metadataだけをauthorityにする

### Q4 canonical drain inventory

CANON0でdrain inventoryまで実装するか？

1. **C-prime（推奨）**: complete productがsource/catalogからprivateな
   `CanonicalSingleDrainPlanV1` / `CallableBatchDrainPlanV1` を導出する。ただし
   DRAIN0で実際のdrain wiringを行う。

2. caller supplied `symbols` / `require_main` / `ConditionFnPolicy` を継続する

3. module mapを再観測してexpected inventoryを作る

## 採択後の最小1 semantic row

Q1/Q2/Q3/Q4をすべてC-primeで採択した場合、`ROOT0-CANON0` は次の一行だけを実装します。

```text
sealed source/header/catalog
-> active branded collector
-> collector-issued exact receipt
-> canonical single or callable-batch completion product
-> exact owner/catalog root witness
```

必須条件:

```text
canonical single:
  exact owner key/header/symbol/arity
  CanonicalRejectDuplicate + Inserted only
  synthetic FunctionDraftKey::Main/SyntheticConditionFn = reject

callable batch:
  whole catalog cardinality/key/symbol/arity exact
  CanonicalRejectDuplicate + Inserted only
  recursive marker exactly once when recursive

all routes:
  foreign token/header/collector/receipt = pre-mutation reject
  late collision = collector delta 0
  caller inventory / Optional condition = absent
  production capture/drain/finalizer/commit consumers = 0
  touched source/check files < 800 lines
```

Canonical physical function named `condition_fn/N` is not rejected merely by
spelling; only synthetic key/policy is forbidden. This avoids introducing an
unrequested language-level name blacklist.

## 非claim

回答が来るまで、以下は実装しません。

```text
production canonical ingress
Main専用stateのcanonical流用
drain/finalizer/external commit wiring
module-map再観測によるinventory補完
```

関連: [ROOT0 design-stop brief](cut0-i0-root0-design-stop-2026-07-22.md),
[T-prime-r1 execution task](cut0-i0-t-prime-r1-execution-task-2026-07-22.md)
