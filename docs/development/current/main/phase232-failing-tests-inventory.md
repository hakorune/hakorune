# Phase 232: Failing Tests Inventory & Classification

このドキュメントは、2025-12 時点の `cargo test --release` で残っている 7 件の FAIL を  
「どの箱／どのパターン／どのレイヤの問題か」で棚卸しするフェーズ用のメモだよ。

---

## 1. 失敗テスト一覧（Task 232-1）

ベースコマンド:

```bash
cargo test --release
```

このセッションでの FAIL は次の 7 件だったよ（当時: 890 passed / 7 failed / 64 ignored）。  
**Update (Phase 233)**: `loop_update_summary` の 4 件を AST ベーステストに刷新して PASS 化し、いまは **894 passed / 3 failed / 64 ignored** だよ（残りは array_filter 系 3 件のみ）。

### 1.1 loop_update_summary 系ユニットテスト（4 件 → Phase 233 で解消済み）

対象ファイル: `src/mir/join_ir/lowering/loop_update_summary.rs`

1. `mir::join_ir::lowering::loop_update_summary::tests::test_analyze_mixed`
   - パターン: if-sum / Counter+Accumulation 検出ロジック（P3 if-sum 用の分析箱）。
   - 失敗種類: **期待値 mismatch（名前ベースヒューリスティック vs 新 AST ベース実装）**
     - 現行実装 `analyze_loop_updates()` は deprecated だが、テストはまだ古いヒューリスティック前提。
   - 備考: 実運用では `analyze_loop_updates_from_ast()` が主経路（Phase 219）。

2. `mir::join_ir::lowering::loop_update_summary::tests::test_analyze_single_counter`
   - パターン: CounterOnly（単一カウンタ）判定。
   - 失敗種類: **期待値 mismatch（deprecated API の挙動とテスト期待のズレ）**
   - 備考: `analyze_loop_updates()` 内で kind を全て AccumulationLike にしているため、テストの CounterLike 期待と合わない。

3. `mir::join_ir::lowering::loop_update_summary::tests::test_is_simple_if_sum_pattern_basic`
   - パターン: P3 if-sum の「i + sum」基本パターン検出。
   - 失敗種類: **期待値 mismatch（is_simple_if_sum_pattern の入力を deprecated wrapper で生成している）**
   - 備考: 本番 if-sum 経路は Phase 219 以降、AST ベースの update 検出に移行済み。

4. `mir::join_ir::lowering::loop_update_summary::tests::test_is_simple_if_sum_pattern_with_count`
   - パターン: P3 if-sum の「i + sum + count」マルチキャリアパターン。
   - 失敗種類: **期待値 mismatch（上と同様、テスト専用の古い summary 生成ロジック）**

Update (Phase 233): 上記 1–4 はすべて `analyze_loop_updates_from_ast()` を直接呼ぶ AST フィクスチャベースのテストに置き換え、`cargo test --release` で PASS になったよ。

### 1.2 ArrayExtBox.filter まわりの joinir mainline テスト（3 件）

対象ファイル: `src/tests/joinir/mainline_phase49.rs`

5. `tests::joinir::mainline_phase49::phase49_joinir_array_filter_smoke`
   - パターン: P3 if-phi / Array filter ループ（実戦寄り PoC）。
   - 失敗種類: **[joinir/freeze] 系エラー → accumulator `'sum' not found in variable_map` で panic**
     - メッセージ: `"[cf_loop/pattern3] Accumulator variable 'sum' not found in variable_map"`
   - 備考: JoinIR mainline に route したあと、Pattern3 PoC lowerer が `sum` を carrier として見つけられず Fail-Fast。

6. `tests::joinir::mainline_phase49::phase49_joinir_array_filter_fallback`
   - パターン: 同じ ArrayExtBox.filter の fallback 経路（Structure-only / legacy PoC の確認用）。
   - 失敗種類: **テスト側期待値「fallback compile should succeed」 vs 実際は Fail-Fast エラー**
     - つまり「今は array_filter の JoinIR 対応をまだ本番とみなさない」状態をどう扱うかのポリシーの問題。

7. `tests::joinir::mainline_phase49::phase49_joinir_array_filter_ab_comparison`
   - パターン: Array filter の route A/B 比較テスト。
   - 失敗種類: **Route A compile should succeed → 同じ `'sum' not found` エラーで失敗**
   - 備考: いずれも core バグというより「ArrayExtBox.filter の JoinIR 適用方針が途中」のフェーズ感のテストだよ。

---

## 2. 箱・レイヤごとの分類（Task 232-2）

ここでは「どのラインの箱から見た問題か」をざっくり割り当てるよ。

### 2.1 JoinIR core ライン

- ArrayExtBox.filter 系（テスト 5–7）
  - 主に Pattern3 if-phi / carrier detection / variable_map との橋渡しライン。
  - エラー文言から見えるのは:
    - `Accumulator variable 'sum' not found in variable_map`
    - → **LoopPattern + CarrierInfo + ExitLine の間の「carrier 定義/再接続」が足りない**。
  - ただし、これは「内部の SSOT が崩れている」というより、PoC lowerer と本線 lowerer が混在している歴史的ライン。

### 2.2 LoopPattern ライン

- loop_update_summary 系（テスト 1–4）
  - P3 if-sum / CaseA パターン検出の一部。
  - 現行本番経路:
    - AST ベースの `analyze_loop_updates_from_ast()` ＋ P3 if-sum 実装（Phase 212/213/220）。
  - テストが見ているのは:
    - deprecated な `analyze_loop_updates()`（名前ベース）と、それにぶら下がる is_simple_if_sum_pattern の挙動。
  - 位置づけとしては **LoopPattern (分析箱) の「旧 API テスト」**。

### 2.3 表現ライン（ConditionEnv / ExprLowerer / MethodCallLowerer / LoopBodyLocalEnv）

- 今回の 7 件の FAIL は、直接にはこの表現レイヤにはかかっていない:
  - Type error / SSA-undef / alias 解決の失敗ではなく、
    - 「古い分析 API のテスト期待値のまま」
    - 「PoC lowerer の carrier 認識が足りない」
    といったレベルの問題に収まっている。
- つまり Phase 232 時点では、ExprLowerer/ScopeManager まわりの **新実装が原因の FAIL はゼロ** という整理でよさそうだよ。

---

## 3. P0/P1/P2 分類（Task 232-3）

ここでは、次フェーズでどう扱うかの優先度分類をするよ。

### 3.1 P0: すぐ直したい JoinIR core バグ

- **今回は P0 該当なし** として扱うのがよさそう。
  - 理由:
    - SSA-undef / PHI 破損 / ValueId 衝突 / ExitLine 契約違反のような「core 崩壊系」の FAIL は含まれていない。
    - ArrayExtBox.filter 系も「PoC lowerer の制約に当たって Fail-Fast」が明示的に出ているだけで、  
      既存の JsonParser / if-sum ラインを壊しているわけではない。

### 3.2 P1: パターン拡張で解決できるもの

候補としては次の 2 グループだよ（Phase 232 時点の整理; その後の更新を併記するね）:

1. **loop_update_summary ユニット（テスト 1–4）** → Phase 233 で解消済み
   - 実際の本番 if-sum ラインは AST ベースで動いているので、
     - これらのテストを「deprecated wrapper の振る舞いテスト」ではなく、
     - `analyze_loop_updates_from_ast()` ベースのテストに差し替える  
     ことで自然に緑にできる。
   - つまり「core バグ」ではなく **テストの更新／仕様の追随** で解消可能。
   - **Phase 233 で実施済み（PASS 化）**。

2. **ArrayExtBox.filter 系（テスト 5–7）**
   - Pattern3 if-phi / Accumulation パターンとして見れば、  
     - `sum` を carrier として認識する
     - legacy PoC lowerer のエラー文言を **「意図的な Fail-Fast」か「サポートする方向」か** 決める  
     といった「パターン拡張 or ポリシー明確化」で解決可能。
   - ただし、こちらは実戦 ArrayExtBox.filter の仕様をどうしたいかに依存するので、  
     先に loop_update_summary 側を P1 で触るほうがスコープが小さくて良さそう。

### 3.3 P2: 当面 Fail-Fast に残すもの

Phase 234 の結論:

- **ArrayExtBox.filter 系 3 テスト（5–7）は P2 に固定**（意図した Fail-Fast / 将来拡張候補）。
  - 理由:
    - ArrayFilter は「BoxCall ベース Accumulation（out.push）」であり、数値 if-sum ラインとは別系統の設計が必要。
    - JsonParser / selfhost / ExprLowerer など、既に進行中のコアラインより優先して手を入れる理由がまだ弱い。
    - 現状の `[cf_loop/pattern3] Accumulator variable 'sum' not found in variable_map` は、「P3 if-PHI の外側にある PoC 領域」を示すラベルとして有用。
  - したがって Phase 234 では:
    - コード変更は行わず、
    - `phase234-array-filter-design.md` に ArrayFilter パターンと P3 if-PHI との関係、将来フェーズでの拡張案だけを整理する。

一方、loop_update_summary ユニット（1–4）は:

- P1（「次にテストを更新するときに一緒に触る」）として扱うのが妥当だよ。
  - 本番経路は既に `analyze_loop_updates_from_ast()` を使っており、  
    deprecated wrapper の仕様を保つ必要は低い。

---

## 4. 次フェーズ候補（Task 232-4）

Phase 232 の整理を前提にした「次の軸」の候補だけメモしておくね（コードはまだ書かない）。

- **Phase 233（P1: 分析箱テストのアップデート）**
  - ねらい:
    - `loop_update_summary.rs` のテスト 1–4 を `analyze_loop_updates_from_ast()` ベースに更新し、  
      実戦 if-sum ラインとテスト仕様を揃える。
  - やることの例:
    - AST フィクスチャ（簡単な if-sum ループ）を直接組んで `analyze_loop_updates_from_ast()` をテスト。
    - deprecated `analyze_loop_updates()` のテストは「歴史メモ」扱いに移すか、最小限だけ残す。

- **Phase 234（P1: ArrayExtBox.filter ラインの設計再検討）**
  - ねらい:
    - ArrayExtBox.filter を「P3 if-phi + Accumulation pattern」として本当に JoinIR で扱うかどうかを設計レベルで決める。
  - やることの例:
    - phase49 の mainline テストが期待しているシナリオを整理（route A/B、fallback の位置づけ）。
    - Pattern3 if-sum / array filter / map/filter 系の位置づけを `loop_pattern_space.md` に追記。
    - まだコードには触らず、「やるならこういう箱に分ける」という案まで。

- **Phase 235（P2: B 系パターンの扱い方の設計だけ）**
  - ねらい:
    - いまは Fail-Fast している B 系パターン（複雑ネスト / 多段メソッドチェーンなど）を  
      「今後も Fail-Fast のままにするのか」「別レイヤの箱で扱うか」を整理する。
  - やることの例:
    - `[joinir/freeze]` 系メッセージを洗い出して、B 系パターンの代表例を docs にまとめる。
    - async / catch / filter/map 連鎖などを、JoinIR 本体ではなく上位レイヤに逃がす設計案を書くだけに留める。

---

## 5. まとめ

- 現状の 7 FAIL は:
- 4 件: **loop_update_summary の deprecated API テスト**（Phase 233 で AST ベースに刷新し PASS 化済み）。
- 3 件: **ArrayExtBox.filter の PoC パターン3 lowerer に対する挙動確認テスト**（意図的 Fail-Fast に近い） ← 現在残っている FAIL。
- JoinIR core（PHI/ValueId/ExitLine/SSA）としての致命的な崩れは含まれていないので、  
  Phase 232 の結論としては「**今の FAIL は “どこをまだやっていないか” を教えてくれるラベル**」とみなしてよさそうだよ。
Status: Active  
Scope: 失敗テスト在庫（JoinIR/ExprLowerer ライン）
