# JoinIR Lowering (ExprLowerer / ScopeManager / Envs)

このディレクトリは JoinIR lowering の中でも、条件式や環境まわりの箱（ExprLowerer, ScopeManager, ConditionEnv, LoopBodyLocalEnv, UpdateEnv など）を扱う層だよ。コードを触るときは、以下の最小ルールを守ってね。

Read first:

1. [`src/mir/README.md`](../../README.md)
2. [`src/mir/join_ir/README.md`](../README.md)
3. [`src/mir/builder/README.md`](../../builder/README.md)

- ExprLowerer は **ScopeManager 経由のみ** で名前解決する。ConditionEnv / LoopBodyLocalEnv / CapturedEnv / CarrierInfo に直接触らない。
- 条件式から UpdateEnv を参照しない。UpdateEnv はキャリア更新専用で、header/break/continue 条件は ScopeManager→ConditionEnv で完結させる。
- ConditionEnv は「条件で参照する JoinIR ValueId だけ」を持つ。body-local を直接入れず、必要なら昇格＋ScopeManager に解決を任せる。
- Fail-Fast 原則: Unsupported/NotFound は明示エラーにして、by-name ヒューリスティックや静かなフォールバックは禁止。

## 名前解決の境界（SSOT）

このディレクトリの `ScopeManager` は「JoinIR lowering の中で」名前を `ValueId` に解決するための箱だよ。
同じ “名前” でも、MIR 側の束縛寿命とは問題が違うので混ぜない。

- **MIR（SSA/束縛寿命）**: `src/mir/builder/vars/*` が `{...}` のレキシカルスコープと `local` のシャドウイングを管理する。
- **JoinIR lowering（この層）**: `ScopeManager` が `ConditionEnv/LoopBodyLocalEnv/CapturedEnv/CarrierInfo` を束ねて解決順序を固定する。
- **解析箱**: `LoopConditionScopeBox` は「条件が参照して良いスコープか」を判定する箱で、名前解決そのものはしない。

詳しい境界ルールは `docs/development/current/main/phase238-exprlowerer-scope-boundaries.md` を参照してね。***
