# 1 つの持ち主にまとめる候補の調査（2026-11-11）

## 経緯

- basic-expr の残り（branch `basic-expr-2`）の監査で、式の評価の結果 `CheckedExpressionEvaluationV1` を作る所が、使う所ごとに 5 つある事が分かった。
  - 5 つの場所: `resolved_semantics/home_print_expression.rs`（print と Return）、`home_initializer_expression.rs`、`home_argument_expression.rs`、`home_binding_rebind_expression.rs`、`home_bool_condition.rs`。
  - それぞれに対になる `home_new_prefix_*` の file もある。
- ユーザーの問い（2026-11-11）: 「ここは式で SSOT 化するところじゃないのか」。
- ChatGPT Pro も、評価の持ち主を 1 つにまとめる方向に賛成した（下の「basic-expr-2 の状況」）。
- ユーザーの依頼で、Gemini 4 Argon（agy）に、main `d3c543bd7d` の他の候補を読むだけで調べてもらった。
  - Opus がその一部を code で確かめた。確かめていない所は、そう書いてある。
- 下の file:line は main `d3c543bd7d` の物。断りが無い path は `src/mir/` からの相対。

## basic-expr-2 の状況（main には未統合）

- 1 回目（`240888a29d`〜`e89720213c`）は監査で不合格。
  - Return の算術の判定が、目標の e16・e20 の形を当てていた。
  - `take` の Call を print や `+` の中に置くと、受理の後に所有権の検証で弾かれる MIR を作った。
- 直しの回（`4a3af727bf`〜`9392995fcb`）も不合格。
  - 前の 4 点は直った。
  - if の中で代入し直した local を、Return の算術で使う形が、main では通るのに拒否になった（`local-read/loop-point-missing`）。
  - 1,788 本には、この形が無かった。
- ChatGPT Pro への相談（2026-11-11）の答えの要点:
  - 回帰は、合流を観測する対象を集める所（`branch_scalar_sources_v1`、`root_flow().expression_plans()` だけを見る）から、Return の評価が漏れている疑いが高い。
  - 直し方の禁じ手: Return を中で `local tmp = e; return tmp` に変える事と、Return のためだけに別の証拠を作る事。
  - 順番:
    1. 合流の証拠の切れ目を直す
    2. 全部の評価の依存を 1 つの訪問でたどる
    3. 評価の本体を作る所を 1 つにする（Print と Return → Local → 引数 → 代入し直し → 条件）
    4. 整数の operand の証明を `value_origin` へ寄せる
    5. `take` の Call ごとの所有の遷移を共通の評価へつなぎ、補いの囲い（`call_argument_source.rs` の `validate_evaluation_entry_cleanup_v1`）を消す
  - 見込み: 段 2・3 で整形後 250〜500 行、段 4 で 20〜60 行の純減（概算）。
- 今は段 1〜4 を Codex が実装中。段 5 は次の回。main への統合は段 5 の監査の後。

## 候補（Gemini の見込みで、効果の大きい順）

行数は Gemini の概算で、候補どうしで重なる。合計は当てにならない。

| # | 分かれている責任と、主な場所 | 持ち主の案 | 見込み |
|---|---|---|---:|
| 1 | 式の MIR を書く所の入口と出口。builder の `ordinary_new_admission/selected/` の 7 つの file（print、initializer、rebind、return、argument、field write、bool condition）と、`expression_value/cleanup.rs` の文脈ごとの `finish_*_continuation`。記録の側の 8 つの `Emitted*Projection` 型（`root_call_entry/*_projection.rs` ほか） | `expression_value.rs` と 1 つの「結果の行き先」の契約。記録は `ExpressionPhysicalProjectionRefV1` を土台に 1 つ | 1,400〜1,700 |
| 2 | 式の値の種類の遷移と、演算子の変換表が何か所にもある（`resolved_value_profile/expression_class.rs`・`expression_projection.rs` のほか、SUM の二項、`source_result_classify.rs`、`callable_result_representation/expression_proof.rs`）。`integer_source_at` が 2 つ | `expression_class.rs` と `expression_projection.rs`、Dynamic は issuer | 950〜1,100 |
| 3 | `return` 文を見分ける所が 4 重（`resolved_semantics/home_new_prefix_terminal.rs` の先着順の連鎖、`home_terminal_relation.rs`、`home_terminal_returned_value.rs`、`home_prefix_source_loop_control.rs`）。`RootHomeCleanupOrderV1` の Return の種類ごとの 10 個の欄。`local` 文の先着順の連鎖と、古い scalar の式の道（`home_new_prefix_scalar_expression.rs`）の並存 | 返す値の評価と、関数の出口での所有の遷移の 2 つに分ける。10 個の欄は 1 つの enum | 1,100〜1,400 |
| 4 | CoreMethod の契約の表が 2 つ（`resolved_semantics/core_method_instance_target.rs` と `resolver_core_method_callable_contract.rs`）。Call の結果の種類を推論する所が何か所にもある。Call を書く所が文脈ごとに分かれている（`terminal_call/` の下など） | 生成された契約の行と `core_method_instance_target.rs`。結果の種類は `callee_result_kind.rs` | 900〜1,150 |
| 5 | 整数の `BinOp`・`UnaryOp`・`Compare` を書く所が 7・4・6 か所（`emission/loop_operation.rs`、`resolved_lowering/trivial_ssa/operation.rs`、`script_physical_exit/direct_static_entry_kernel.rs` ほか） | `ssot/binop_lower.rs`、`builder/ops/unary.rs`、`builder/emission/compare.rs` | 650〜800 |
| 6 | loop の中だけ、所有と後始末を別の仕組みで追う（`home_prefix_source_loop.rs`、台帳の `loop_return_text_ends`・`loop_call_text_ends`）。loop の CFG と PHI を作る道が複数 | `LiveHomeV1` と `FaultCleanupPlanV1`、CFG は `FunctionControlV1` | 700〜900 |
| 7 | field の読み書きと String の操作を書く所が複数（`builder/fields.rs`、`fields/checked_read.rs`、constructor の中、`common_v2_session/length_call.rs` の raw の length の Call など） | `fields/checked_read.rs` と `selected/checked_field_write.rs` | 550〜700 |
| 8 | 検証の中で CFG・支配の木・定義の表を何度も計算し直す（`verification.rs`、`verification/invoke.rs`、`ownership_ssa/verify.rs`）。root と child の検証の手順が 2 つ並ぶ（`ordinary_new_local_commit/root_validation.rs`） | 検証の中だけで共有する構造の解析を 1 回作る。root と child の検証は 1 つの関数 | 450〜550 |
| 9 | 台帳を引く時の「1 件だけ取り出す」処理を、何十か所で手で書く。source の点の部分木の判定も散らばる | `exact_one` と `at_most_one` を分けて用意。部分木の判定は `source_site.rs` | 400〜550 |

### Opus が確かめた事

- 候補 8: `ordinary_new_local_commit/root_validation.rs:138` と `:156` で、`verify_finished_borrowed_compares_v1` を同じ closure で 2 回続けて呼んでいる。
- 候補 8: `ownership_ssa/verify.rs:720-732` の `reachable_blocks` は、`Catch` の handler を辿らない。`verification/utils.rs` の `compute_reachable_blocks` は辿る。2 つの判断が食い違っている。
- 候補 2: `integer_source_at` が `normal_callable_semantic_package/array_i64_fields.rs:148`（148 行）と `resolved_semantics/named_array_residence.rs:380`（92 行）にあり、どちらも `INTEGER_SOURCE_DEPTH = 8` を持つ。ただし中身は 30 行ほど違い、丸ごとの写しではない。
- 候補 3: `home_new_prefix_terminal.rs:514` と `:544` で `finite_entry_return` を呼ぶ。2 回呼ぶのは本当。重複かどうかは確かめていない。
- それ以外の候補の file:line と行数は、まだ確かめていない。手紙を書く前に確かめる。

## 独立の検証として残す物（まとめない）

- source と MIR の対応の監査（台帳、`physical_boundary.rs`、`Emitted*Projection`）と、source を見ない MIR の検証（`verification.rs`、`verification/invoke.rs`、`ownership_ssa/verify.rs`）。
- 生成の側の `FaultCleanupPlanV1` の展開と、検証の側が実際の CFG の辺をたどる cleanup の検査（`root_call_entry/expression_terminal_cleanup.rs`、`root_home/validation.rs`）。
- 期待する命令を導く `scalar_condition_operation.rs` と、命令を書く所と、後で照合する所の 3 つ。
- 段階ごとの検証（意味の package、lowering の受理、`MirVerifier`）。
- 候補 8 は、この独立を崩さずに、検証の中での計算のし直しだけを減らす。

## 順番（Opus の提案。ユーザーが Opus に任せた、2026-11-11）

1. **basic-expr の残り**: 評価の持ち主の一本化（Pro の段 1〜4、実装中）→ `take` の共通モデル（段 5）→ 監査を通ったら main へ統合。
2. **検証の小さな直し**（候補 8 の一部）: 同じ検査の 2 回の呼び出しをやめる。`ownership_ssa` の届く block の判断を、共通の関数と揃える（`Catch` の食い違い）。builder とぶつからない小さな回。
3. **式の道の一本化の続き**（候補 1 と 3）: 評価の本体が 1 つになった直後に、MIR を書く所の入口・出口と記録の型、Return と local の先着順の連鎖をまとめる。
   - Map の契約や `edit` で使う所が増える前にまとめれば、新しい機能は 1 か所へつなぐだけで済む。だから、Map の契約より前に置く。
4. **Map の契約** → **`edit`** → **ADR の残り**。
5. **3b と旧経路の退役**: raw の道が消えると、候補 5・7 の書き手の数も減る。だから、その整理より前に置く。
6. **徹底的な整理**: 残りの候補を、Gemini の勧める順でする。
   - 候補 9・2・4 の前半（純粋な分類と問い合わせ）
   - 候補 5・7・4 の後半（命令の書き手）
   - 候補 6（loop）
7. `.hako` parser の parity → selfhost。

各回の決まりは今までと同じ。実装者は最初に subagent で経路を上からたどる。責任ごとに持ち主は 1 つ。整理の回は、整形後の production の純減で閉じる。監査では、実装者に見せていない入力を流す。
