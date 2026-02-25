# Phase 233: loop_update_summary テスト刷新

目的: deprecated な `analyze_loop_updates()` 依存の期待値を捨てて、Phase 219 で本番化した `analyze_loop_updates_from_ast()` に合わせてユニットテストを組み直すよ。ロジック本体は既に AST ベースで動いているので、テスト側の追随だけを行うフェーズだよ。

---

## 1. 旧 API (`analyze_loop_updates`) の前提

- 入力: carrier 名リストだけ（RHS や AST 構造を見ない）。
- ヒューリスティック: すべて AccumulationLike 扱い（名前ベースの Counter 推定は撤廃済み）。
- テストで見ていたもの:
  - `test_analyze_single_counter` / `test_analyze_mixed`: 名前だけで Counter/Accumulation を判断する期待。
  - `test_is_simple_if_sum_pattern_*`: deprecated wrapper で組んだ summary をそのまま `is_simple_if_sum_pattern` に渡していた。
- 問題: 本番 if-sum ラインは AST ベースに移行済みのため、これらのテストは現実の入力と乖離して FAIL していた。

## 2. 新 API (`analyze_loop_updates_from_ast`) の前提

- 入力: carrier 名リスト + ループ body の AST。
- 処理:
  - `extract_assigned_variables` で実際に LHS に現れた変数だけを対象にする。
  - `find_assignment_rhs` で RHS を拾い、`classify_update_kind_from_rhs` で CounterLike / AccumulationLike を決定。
  - ループインデックスっぽい名前（i/j/k/idx/index/pos/n）だけは Counter 扱いに固定。
- 実運用:
  - PatternPipelineContext から if-sum 判定に使われる唯一の経路。
  - Phantom carrier 解消（Phase 219）の一環として既に配線済み。

## 3. テストケースの扱い（棚卸し結果）

- `test_analyze_single_counter` → **AST ベースに組み直す**  
  - ループ body に `i = i + 1` を置き、`has_single_counter()` が真になることだけを確認。
- `test_analyze_accumulation` → **AST ベースに置き換え**  
  - `sum = sum + i` のような RHS を与え、非 index 名が AccumulationLike になることを確認。
- `test_analyze_mixed` → **Counter + Accumulation の AST で再構成**  
  - `i = i + 1` と `sum = sum + i` の 2 本で、counter=1 / accumulation=1 を確認。
- `test_is_simple_if_sum_pattern_basic` → **if-body に accumulator 更新を置いた AST で検証**  
  - `if (cond) { sum = sum + i }` + `i = i + 1` の組み合わせで true を確認。
- `test_is_simple_if_sum_pattern_with_count` → **accumulator 2 本の AST で検証**  
  - `sum` / `count` 両方が AccumulationLike になり、2 本まで許容する条件を確認。
- 旧 wrapper への期待は残さず、どうしても残す場合は `#[ignore]` で歴史テストにする方針。

---

## 4. 期待する着地

- ユニットテストは `analyze_loop_updates_from_ast()` を直接呼ぶ形に揃える。
- `is_simple_if_sum_pattern` は AST 由来の summary を入力として検証するだけにする（パターン検出ロジック本体の契約をテスト）。
- Phase 232 時点の FAIL（4 件）は、テスト刷新で 0 件にする。***
Status: Active  
Scope: ループ更新サマリ / テストリフレッシュ（JoinIR/ExprLowerer ライン）
