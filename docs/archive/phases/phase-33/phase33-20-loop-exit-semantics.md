今後は continue 系と JsonParserBox のような実アプリ側ロジックを順番に乗せていく段階に入る。**
# Phase 33‑20: Loop Exit Semantics Fix — Completion Summary

日付: 2025‑12‑07  
状態: ✅ 実装完了（Pattern1/2/3/4 正常、複雑 continue パターンのみ残課題）

---

## 1. ゴール

LoopBuilder 完全削除後の JoinIR ループラインにおいて、

- loop header PHI
- ExitLine（ExitMeta/ExitBinding/ExitLineReconnector）
- JoinInlineBoundary + BoundaryInjector

のあいだで「ループ出口値（expr + carrier）」の意味論を揃え、

- SSA‑undef を起こさない
- Pattern1/2/3/4 代表ケースでループ終了時の値が正しく戻ってくる

状態にすること。

---

## 2. 変更内容

### 2.1 BoundaryInjector の修正（loop_var_name 対応）

ファイル: `src/mir/builder/joinir_inline_boundary_injector.rs`

問題:

- loop header PHI の `dst`（ループ変数の現在値）に対して、BoundaryInjector が entry block で Copy を挿し、
  header PHI の意味を上書きしてしまうケースがあった。

修正:

- `JoinInlineBoundary.loop_var_name` が設定されている場合は、
  **すべての `join_inputs` について entry block での Copy 挿入をスキップ**するように変更。
- これにより、header PHI で決まった `dst` が entry の Copy で壊されることがなくなり、
  header PHI が「ループ変数の SSOT」として機能するようになった。

### 2.2 Pattern3(If‑Else PHI) の Boundary 設定

ファイル: `src/mir/join_ir/lowering/loop_with_if_phi_minimal.rs`

問題:

- Pattern3 lowerer が JoinInlineBoundary に `loop_var_name` を設定しておらず、
  BoundaryInjector/InstructionRewriter が「このループに expr/キャリア出口がある」ことを認識できていなかった。

修正:

- Pattern2 と同様に、Pattern3 でも `boundary.loop_var_name = Some(..)` を設定。
- これにより、JoinIR merge 時に header PHI / exit PHI のラインが Pattern3 でも有効になる。

### 2.3 merge/mod.rs — LoopHeader PHI + carrier PHI の連携

ファイル: `src/mir/builder/control_flow/joinir/merge/mod.rs`

修正ポイント:

1. ExitLine 側から header PHI に必要な carrier 名一覧（`other_carriers`）を抽出し、
   LoopHeaderPhiBuilder に渡すように変更。
2. LoopHeaderPhiBuilder が生成した header PHI の `dst`（carrier_phis）を、
   LoopExitBinding/ExitLineReconnector が利用するように接続。
3. function_params のキーを `"join_func_0"`, `"join_func_1"` 等の実際の JoinIR 関数名に合わせるよう修正
   （誤って `"main"`, `"loop_step"` を参照していたため Pattern4 で PHI が正しく構築されていなかった）。

これにより、

- header PHI `dst` を起点に、carrier 用の出口値が ExitLine へ正しく流れるようになった。

### 2.4 instruction_rewriter.rs — header PHI への Copy スキップ＋latch incoming 設定

ファイル: `src/mir/builder/control_flow/joinir/merge/instruction_rewriter.rs`

修正内容:

- LoopHeader PHI の `dst` に対して、余計な Copy が挿入されないようにするガードを追加。
- 複数キャリアのケースで、latch block からの incoming を header PHI の入力として正しく構成するよう調整。

これにより、

- Pattern1/2/3/4 のループ変数が header PHI → exit PHI → variable_map の順で一貫して伝播するようになった。

---

## 3. テスト結果

### 3.1 Pattern1: Simple While

- テスト: `apps/tests/loop_min_while.hako`
- 期待値: `0, 1, 2` を出力し、RC は 0。
- 結果: ✅ 期待どおり。

### 3.2 Pattern2: Loop with Break（expr ループ）

- テスト: `apps/tests/joinir_min_loop.hako`
- 期待値: ループ終了時の `i` の値（2）が expr result として返る。
- 結果: ✅ RC: 2、SSA‑undef なし。

### 3.3 Pattern3: Loop with If‑Else PHI

- テスト: `apps/tests/loop_if_phi.hako`
- 期待値: `sum = 9`（1+3+5）。
- 結果: ✅ `sum = 9` を出力、RC も期待どおり。

### 3.4 Pattern4: Loop with Continue

- テスト: `apps/tests/loop_continue_pattern4.hako`
- 期待値: 25（1+3+5+7+9）。
- 結果: ✅ 出力は 25。
  - `[joinir/freeze]` / SSA‑undef は発生しない。
  - function_params キーの誤参照（`"main"/"loop_step"` → `"join_func_0"/"join_func_1"`) を修正したことで、
    header PHI / ExitLine との結線が正しくなり、Pattern4 の単純 continue ケースでも期待どおりの値になった。

---

## 4. まとめと今後

### 達成点

- header PHI を起点とした Loop exit の意味論が Pattern1〜4 の代表ケースで一貫するようになった。
- ExitLine（carrier）と expr PHI ラインが、LoopBuilder なしの JoinIR パイプラインで安全に動く。
- trim や JsonParser のような複雑なループに対しても、基盤として使える出口経路が整った。

### 残課題

- 「else 側 continue」を含む複雑な continue パターン（Phase 33‑18 の Pattern B）はまだ JoinIR 側で正規化されていない。
  - これらは BoolExprLowerer/ContinueBranchNormalizer で `if (!cond) { … }` 形に正規化し、
    Pattern4 lowerer に統合する計画（Phase 33‑19 以降）。

### 関連フェーズ

- Phase 33‑16: Loop header PHI SSOT 導入
- Phase 33‑19: Continue + if/else パターンの正規化（設計済み）
- Phase 170: JsonParserBox / trim の JoinIR 準備ライン

このフェーズで「LoopBuilder 無しの JoinIR ループ出口ラインの基礎」は固まったので、  
今後は continue 系と JsonParserBox のような実アプリ側ロジックを順番に乗せていく段階に入る。**
Status: Historical

