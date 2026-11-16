# 比較の実装1: 小さい独立builder

設計の正本は `../design/mirbuilder-blueprint.md`。既定builderは変更せず、明示選択時だけ別のmoduleを原子的に作る。既存の入口が拒否した後で切り替えない。

## 実装の規則

- 宣言と署名はDeclarationTable、sourceの意味と拒否はResolveだけが所有する。ResolvedFunctionは構造のある型付き本体で、物理ValueId/blockを持たない。
- Lowerは一つの式入口・一つのCall入口・一つのloop入口と、一つのSSA作業場で契約を実現する。書かれた式の順を保ち、場所別の道を作らない。
- scope、break/continue/returnの行き先はResolveで固定する。範囲loopは境界を入口で各一度評価し、半開区間、step+1、continue→step、indexは書換え不可。
- この比較のi64実行は現行Integer laneの契約を使う（types.mdの型名/storage lockとdynamic-operators.mdのcurrent-I64）。Add/Sub/Mulはwrapping。未接続のexact numeric checked演算を型名から推測して選ばない。
- 一般Div/Moduloの異常条件には現行のsource契約が不足しているため、新しい規則を暗黙に作らない。原本の正整数literal除算以外は、この回で契約を発明して通さない。
- Verify cの期待はResolvedFunctionから読む。Lowerの対応記録は座標と実値/辺だけで、期待演算・callee・順の自己申告を持たせない。余分な操作・落ちた操作・重複・条件外実行を完成MIRから拒否する。
- 構造検証は既存のstrict Document policyを利用する。これは旧sealed Integer-only corridorを主張しない入口で、CFG/SSA/支配/PHI/Invokeの検査を保つ。新しい対応検証が正確なi64/Bool署名・実引数・結果を検査する。
- 旧SSA作業場は旧MirBuilder/家族の証拠に依存するため流用せず、設計図にある小さいsealed-block SSAを比較builder内に置く。
- backendは完成MIRから既存のcall-route/definition metadataだけを投影し、既存canonical JSONと汎用C backendへ渡す。全semantic_refreshや家族ごとのsource preflightを呼ばない。BoolをIntegerへ偽装しない。

## 現在の範囲

最初の縦の入力は型注釈付きの `return 1 + 2`。宣言/Resolveの段から接続を始める。後続の受理範囲と結果は、実行確認後にここを書き換える。
