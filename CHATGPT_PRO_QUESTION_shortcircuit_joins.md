# ChatGPT Pro 相談: short-circuit + joins / SSA/PHI 構造 (CorePlan)

## 背景
Nyash/Hakorune の JoinIR/Plan 正規化では、条件式 `a && b && c` / `a || b` を nested if に展開しつつ、if 末尾で `joins` (then/else の 2状態で変数をマージする PHI 相当) を生成しています。

ここで `&&/||` は意味論として 3経路になります:
- `a && b`: (a=false), (a=true,b=false), (a=true,b=true)

`joins` を outer に 1 回だけ付ける設計だと、outer then に (a=true,b=false) が混ざり、then_val が全経路で定義されず SSA undefined が発生します。

現状の修正は、short-circuit 展開の中間で **Copy を挿入**して 3経路を 2値に畳む SSOT (`CoreEffectPlan::Copy`) を導入しました。

## いまの SSOT (現状維持案)
- `CoreEffectPlan::Copy` を canonical solution として維持し、doc に Join Contract を固定。

## 相談したいこと
1. 「Copyで3経路→2値に畳む」方針は、SSA/PHI 的に長期的にも筋が良いですか？
2. Copy を将来消したい場合、最小の構造設計として何を SSOT にすべきですか？（例: block params/edge args, CFG sealing, JoinKey など）
3. 現行の CorePlan/Lowerer の責務分割（Skeleton/Ops/Flow の3層分離、PHI=block params）に移行する場合、段階移行の切り方（最初に導入すべき最小型/境界）は？
4. Fail-fast の不変条件（PHI入力数=pred数、dominance、join一意性など）で、実装コストが低く効果が高いチェックはどれ？

## 重要な制約
- AST rewrite（見かけ等価変形）禁止。analysis-only view は OK。
- silent fallback 禁止（planner_required で None→freeze）。
- 大規模変更は次フェーズ。今フェーズは最小差分で SSOT を固定して進めたい。

## 添付ファイル
この repo では以下のファイルが短絡+joinsのSSOTです（zipに含めます）:
- `docs/development/current/main/design/short-circuit-joins-ssot.md`
- `src/mir/builder/control_flow/plan/normalizer/cond_lowering.rs`
- `src/mir/builder/control_flow/plan/mod.rs`
- `src/mir/builder/control_flow/plan/lowerer.rs`
- `src/mir/builder/control_flow/plan/verifier.rs`
