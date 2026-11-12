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

1. **basic-expr の残り**: 評価の持ち主の一本化（Pro の段 1〜4）→ `take` の共通モデル（段 5）→ main へ統合。**済み（`30a8e64d2b`、2026-11-12）。** production は整形後 +3,318 行。`take` の回の +1,628 行の大半は、共通の状態を使う所ごとの記録と検証の型へつなぐ code で、3 番目の回で減らす（ユーザー決定 2026-11-12）。
2. **検証の小さな直しと、持ち主の数の見張り**（候補 8 の一部、ユーザー決定 2026-11-11）:
   - 同じ検査の 2 回の呼び出しをやめる。`ownership_ssa` の届く block の判断を、共通の関数と揃える（`Catch` の食い違い）。
   - `tools/checks` に、持ち主の数が減る事しか許さない検査を足す（下の「再発を防ぐ」）。
   - コンパイルの時間が if の数の 3 乗で増える所を直す（下の「if の数とコンパイルの時間」、ユーザー決定 2026-11-12）。
   - builder とぶつからない小さな回。**済み（`67b78a4383`、2026-11-12）。**
2b. **prefix の走査の倍々の遅さ**（Opus が見つけた、2026-11-12）: if の中で field を読む形で、時間とメモリが if 1 つごとに約 2 倍になる（下の「残る倍々の遅さ」）。合流で状態を 1 つにまとめ、道を数え上げない形にする。メモリが尽きる危険があるので、式の道の一本化の回より前に、小さな回で直す。**済み（`6212cbb900`、2026-11-12）。**
2c. **loop の中の倍々**（Opus が見つけた、2026-11-12）: loop の中の、field を読む if で、時間が if 1 つごとに約 2 倍になる（下の「Opus の確認と統合（2b）」）。合流の DAG を、訪れた印を付けて 1 回ずつたどる形にする。**済み（`80f2a162ee`、2026-11-12）。**
3. **式の道の一本化の続き**（候補 1 と 3）: 評価の本体が 1 つになった直後に、MIR を書く所の入口・出口と記録の型、Return と local の先着順の連鎖をまとめる。`take` の回で足したつなぐ code（`expression_terminal_paths.rs`、`result_new_source.rs` など）と、残った計算・組み立ての関数 13 個も対象にして、純減で閉じる。
   - Map の契約や `edit` で使う所が増える前にまとめれば、新しい機能は 1 か所へつなぐだけで済む。だから、Map の契約より前に置く。
4. **Map の契約** → **`edit`** → **ADR の残り**。
5. **3b と旧経路の退役**: raw の道が消えると、候補 5・7 の書き手の数も減る。だから、その整理より前に置く。
6. **徹底的な整理**: 残りの候補を、Gemini の勧める順でする。
   - 候補 9・2・4 の前半（純粋な分類と問い合わせ）
   - 候補 5・7・4 の後半（命令の書き手）
   - 候補 6（loop）
7. `.hako` parser の parity → selfhost。

各回の決まりは今までと同じ。実装者は最初に subagent で経路を上からたどる。責任ごとに持ち主は 1 つ。整理の回は、整形後の production の純減で閉じる。監査では、実装者に見せていない入力を流す。

## 再発を防ぐ（ユーザー決定 2026-11-11）

ユーザーの言葉: 「SSOT 化はなんども言ってるのに、何度も出てきます」。

### 何度も出てきた理由
- 指示書が「この入力を通す」単位で、実装者には、今触っている使う所の file へ足すのが一番の近道だった。持ち主が 1 つかを確かめる手段が無かった。
- 使う所ごとの file の組（`home_X`・`home_new_prefix_X`・`selected/X`・`*_projection`）が既にあり、機能を足すたびに同じ物を組の数だけ書く形になっていた。
- 機能の行は別に数える決まりのため、機能の回で増えた写しが「後の徹底的な整理」へ回され、溜まった。
- Opus の監査は、動き・未見の入力・行数・形当てを見ていたが、同じ事を決める所や書く所の数を数えていなかった。

### これからの決まり
- **指示書**: 経路の調査で見つけた、つなぐ先の持ち主を名前で書く。2 つ目の持ち主を作るのは禁止。報告には、同じ事を決める・書く新しい型と関数を全部並べる。
- **監査**: 触った責任ごとに、持ち主の数を前と後で数える。古い物を消さずに新しい物を並べた回は不合格。
- **機械の見張り**: `tools/checks` に、数が減る事しか許さない検査を置く。main の数を基準にし、増えたら止める。
  - 数える物の例（main `d3c543bd7d`、test の file を除く、`src/mir/` の中）:
    - `CheckedExpressionEvaluationV1 {` で評価を作る所: 8 file・16 か所
    - 文脈ごとの `fn finish_*_continuation`: 10 個
    - `struct Emitted*ProjectionV1`: 7 個
    - `compute_dominators(` を呼ぶ所: 41 file
  - `MirInstruction::BinOp {` は 121 file に出るが、読む所や match も含む。作る所だけを数える方法は、実装者が考える。
  - 一本化の回で数が減った時は、同じ commit で基準を下げる。基準を上げるには、ユーザーの決定が要る。

## if の数とコンパイルの時間（Opus が測った、2026-11-12）

### 経緯
- basic-expr-2 の評価の一本化の回（`58eb38836c`）は、合流の証明を 1 つの問い合わせの中で 1 回だけ確かめる形にした（規則 C39）。
- これで、if の中で field を読む形を k 個並べた入力は、MIR を main と同じに保ったまま、倍々の遅さが消えた。
  - k=10: 1.93 秒 → 0.12 秒
  - k=11: 7.49 秒 → 0.17 秒
  - k=12: 29.2 秒 → 0.31 秒
- ただし、`if x > i { a = a + i }` を k 個並べて最後に `return a` する入力では、時間が k の 3 乗で増える遅さが残る。main からある物。

### 測った時間（`--dump-mir`）
| k | main `d3c543bd7d` | basic-expr-2 `58eb38836c` |
|---|---:|---:|
| 15 | 0.11 秒 | 0.10 秒 |
| 30 | 0.44 秒 | 0.42 秒 |
| 60 | 3.38 秒 | 2.90 秒 |
| 120 | 33.5 秒 | 26.0 秒 |

### 時間を使っている所（perf、k=60、自分の時間の割合）
- CFG の解析を何度も作り直す（`compute_dominators`、`compute_predecessors`、hash）: 約 25%
- 値の定義を、全部の命令から探す（`dst_via_meta`、`dst_value`）: 約 19%
- 合流の source を中身ごと `==` で比べて重複を除く（`BranchNormalScalarSourceV1::eq`、`branch_scalar_sources_v1`、completion の lookup）: 約 22%
  - 合流の source は、前の if の合流をたどれる形でつながっている。だから 1 回の比較が、前の if を全部たどる。

### 直す形
- **解析は関数ごとに 1 回作って借りる**: 支配の木、前の block、定義の索引、届く block を、MIR が出来上がった状態に対して 1 回作り、各検査が借りる。MIR が変わったら作り直す。常設の cache は作らない（Pro の答えの 3.2 と同じ注意）。候補 8 と同じ所。
- **定義は索引で引く**: 一意性・順序・支配の検査は、索引を作る時に 1 回行う。
- **合流の証拠は ID で比べる**: source の点と binding で決まる ID で比べ、重複を除く。前から 1 回たどって「確かめ済み」の表を作り、後ろの読みは自分のすぐ前の合流だけを見る。
- 独立の検証の中身は省かない。速くするために、検証を弱めたり、確かめた印を MIR の変更の後へ持ち越したりしない。
- 確かめ方: 上の k=15〜120 の入力で、時間がほぼ k に比例する事。目安は k=120 で 1 秒程度。出来上がる MIR は前と同じ。


## 検証の回の記録（2026-11-12、基点 5ba99418b0）

### 原因と一つにする持ち主

| 項目 | 実際の入口・原因（基点の行） | 置換先の持ち主 |
|---|---|---|
| R1 | `ordinary_new_local_commit/root_validation.rs:138,156` の child 比較検証が同一。root と child 全体は下表のとおり異なる | 同 file の共通 finished operation 検証、役割別 wrapper は保持 |
| R2 | `ownership_ssa/verify.rs:725` は通常 successors だけを辿り、`verification/utils.rs:192` は Catch handler も辿る | `verification/utils::compute_reachable_blocks`。所有状態の handler 伝播とは分離 |
| R3 | `verification.rs:299`、`verification/invoke.rs:352,504`、`ownership_ssa/verify.rs:31` が解析を再構築。`invoke_string_read.rs:132,141` は operand ごとに支配木と全定義走査 | 既存 `value_origin::indexed_value_definitions` と、一つの関数検証スコープの geometry |
| R4 | `root_call_entry/binding_rebind_projection.rs:263,294,385` の全評価・過去合流の再探索と線形 seen 検索。`expression_reference_branch.rs:212,278` は過去観測の幾何を再検証 | 原本の flow/value/binding/site による source 索引と、不変 MIR 内の観測検証スコープ |
| R5 | 同じ判断・書き手の増加を着地時に検出できない | `tools/checks` の持ち主数 guard と、同じ変更で下げる基準 |

現在の `home_prefix_scalar.rs:52` の source 等値性は、flow の同一性と value/binding/site の浅い比較である。過去の「一回の比較が DAG 全体を再帰比較する」という説明は現在の実装には当たらない。全 DAG の再訪問と線形 seen 検索の回数が増幅する。

| 最終検証の責務 | Child | Root |
|---|---|---|
| New / field / literal / Home exit | 個別、child の順序 | root body の順序 |
| unit return / main arguments | なし | あり |
| borrowed compare | 同じ呼出し2回を1回へ | 呼出しなし |
| scalar / mul / forwarded copies | あり | あり |
| terminal Call 再束縛・状態遷移 | child 用 | root 用 |

役割差と最初の拒否順序を保ち、同一の処理だけ共通化する。child を root body に丸ごと置換しない。

### 変更前の実測

基準は `basic-expr-2-take/hakorune.final`（SHA-256 `80050f468d05ab7833bf0512aa2721ab6eaa45c36ab0391445892a1ce94e555a`）。同じ機械で Cargo と重ねず `--dump-mir` を2回ずつ実行した。回数は基点に一時計数だけを入れた別 build で測り、計数を撤去してから実装する（計数時の時間は比較に使わない）。

| k | 秒（2回） | 支配木の回数 / 処理block数 | predecessor の回数 / 処理block数 | source照会 | Branch訪問 | seen比較 | 全評価visitor / 評価候補 | 物理Branch幾何 |
|---:|---|---|---|---:|---:|---:|---|---:|
| 15 | 0.0963 / 0.1005 | 1183 / 55403 | 1989 / 90012 | 1597 | 43119 | 290654 | 1799 / 52710 | 721 |
| 30 | 0.4026 / 0.3996 | 3688 / 354563 | 6849 / 629312 | 6352 | 362064 | 5157824 | 6704 / 399720 | 3016 |
| 60 | 2.7006 / 2.7358 | 12748 / 2494583 | 25344 / 4690062 | 25312 | 2961504 | 86617664 | 25964 / 3110640 | 12331 |
| 120 | 23.0569 / 23.1822 | 47068 / 18611423 | 97434 / 36174662 | 101032 | 23944584 | 1418691344 | 102284 / 24538080 | 49861 |

### 規則（実装より前に固定）

V1. 同じ不変入力に対する同じ検査は一回とし、root と child の責務差・拒否順序は保つ。
V2. CFG の到達判断は共通 owner に寄せ、Catch setup を通常 PHI predecessor や所有状態の伝播の証拠にしない。
V3. terminator と cached successors の不一致検出を保持し、通常辺と Catch-aware 到達性を混同しない。
V4. 一つの不変関数の検証スコープで既存の定義索引・predecessors・支配木・到達集合を一回作り、consumer は借りる。
V5. パラメータ、terminator、重複定義、定義前使用、PHI の predecessor edge 上の支配、Copy cycle の検査を保つ。
V6. MIR の変更で解析と物理観測の検証済み集合を破棄し、常設 cache や別 epoch への証明の持越しを作らない。
V7. Branch source の識別は既存の flow identity・value・binding・site の等値性に一致させ、原本所属と incoming の照合を省かない。
V8. source の索引と物理観測の検証済み集合は分離し、同じ source に異なる PHI を対応させた改変を拒否する。
V9. loop の補助 source、Return を含む既存の全評価 inventory、独立した Fault と所有権の検証を保持する。
V10. 持ち主数の guard は test を除き、読む pattern と作る式を区別し、数え方と実数を表示する。
V11. 持ち主数が減ったら同じ commit で基準を下げ、増加にはユーザーの判断を必要とする。
V12. 入力の名前や if の個数に依存せず共通表現で整理し、MIR・拒否・実行結果を不変にして整形後 production の純減で閉じる。

### Opus の確認と統合（2026-11-12）

main へ早送りした（`67b78a4383`）。

- **速さ**: Opus が 2 回ずつ測った。出来上がる MIR は、どれも前と同じ。
  - `if x > i { a = a + i }` を k 個並べる形: k=120 で 23 秒 → 1.38 秒。k の 2 乗くらいが残る。残りは MIR を書き換える合間の prefix の検証で、式の道の回の範囲（実装者の記録どおり）。
  - Opus の未見の if・else の連鎖（2 つの local）: k=80 で 15.75 秒 → 1.00 秒。
- **動き**: Opus の監査の入力 110 本で、MIR と EXE の結果が前の main と全部同じ。
- **持ち主の数**: `tools/checks/mir_owner_count_guard.sh` を Opus も走らせ、PASS（29 秒）。支配の木を作る呼び出しは 52 → 43、前の block は 12 → 8、定義を探す候補は 313 → 294。
- **行数**: production は −39（inline の test を test に数える方式）。test file だけを分ける方式の −1,160 は、既にある inline test を file へ移した分を含むので、減らした数に数えない（実装者もそう書いている）。test は +158。見張りの道具は別に +655。
- **設計の注意**: 台帳（`OrdinaryNewClaimLedgerV1`）が、`RefCell` と `Drop` の guard で、検証の scope と branch の source の索引を持つようになった（`physical_validation_scope.rs`、`ordinary_new_branch_source_index.rs`）。同じ MIR かどうかを、pointer と borrow の寿命で見ている。今は lexical な scope で正しく戻るが、明示的に渡さない文脈なので、式の道の回で明示的な引数へ置き換える候補にする。

### 残る倍々の遅さ: if の中で field を読む形（Opus が見つけた、main からある物）
- f10・f11・r12 と同じ形（`if x > i { if me.leaf.v > 0 { a = a + i } }` を k 個）を、k を増やして測った。
| k | main | basic-expr-2 | 検証の回 | 最大メモリ |
|---|---:|---:|---:|---:|
| 14 | 90 秒で打ち切り | 0.99 秒 | 0.61 秒 | 約 200 MB |
| 16 | 90 秒で打ち切り | 2.45 秒 | 2.27 秒 | 約 680 MB |
| 18 | 90 秒で打ち切り | 9.20 秒 | 9.14 秒 | 約 2.6 GB |
- 時間もメモリも、if が 2 つ増えるごとに約 3.8 倍になる（if 1 つで約 2 倍）。k=40 では 25 GB を使い、kernel に止められた（OOM、2026-11-12 10:57。止められたのはそのコンパイルだけ）。
- k=10〜12 で速くなったのは、倍々の底が小さくなっただけで、倍々は消えていない。
- perf（k=16）では、`source_value_flow` の `alias_terminal_with_expression_boundary`、`PrefixLocalFlow` の scalar の作り手、`BTreeMap` の丸ごとの複製と破棄が上位に来る。if ごとに、prefix の走査が状態を複製して両方の枝を別々にたどり、合流で 1 つにまとめていない疑いが高い（未確認）。
- selfhost のコンパイラ自身には、field を読む if が多い。このままでは大きな関数でメモリが尽きる。

## prefix の走査の倍々（2026-11-12、基点 084555a49f）

### P1: 原因と変更前の実測

入口は `home_new_prefix.rs:543` → `home_new_prefix_scan.rs:186` → `home_new_prefix_branch.rs:202`。枝のscalar要求は `home_new_prefix_binding_rebind.rs:94` から既存 `PrefixLocalFlow::scalar_producer_for_value_v1` へ届く。

`home_prefix_scalar.rs:229/238/324-338` が、同じ元source IDの合流を両incomingから再帰的に作り直し、各訪問でpath集合を複製する。`BranchNormalScalarSourceV1`は既にflow/value/binding/siteによる同一性とRcを持つが、生成時に過去の合流を共有しないためDAGを木へ展開している。resolverは `source_value_flow.rs:335-372/525-529` で元のIDとincomingを保持済みであり、新しいID体系は不要。

| k | PrefixLocalFlow複製 / locals行の合計 | scalar再帰 / alias照合 | join生成 / incoming | path複製要素合計 / 最大深さ | 秒2回 / 最大RSS KiB2回 |
|---:|---:|---:|---:|---:|---|
| 10 | 60 / 180 | 20,454 / 20,454 | 10,210 / 20,420 | 264,294 / 20 | 0.08 / 0.09 ; 64004 / 63768 |
| 12 | 72 / 216 | 81,892 / 81,892 | 40,926 / 81,852 | 1,302,642 / 24 | 0.18 / 0.19 ; 87984 / 87496 |
| 14 | 84 / 252 | 327,650 / 327,650 | 163,802 / 327,604 | 6,193,278 / 28 | 0.59 / 0.6 ; 210776 / 210180 |
| 16 | 96 / 288 | 1,310,688 / 1,310,688 | 655,318 / 1,310,636 | 28,704,906 / 32 | 2.23 / 2.21 ; 701880 / 702296 |
| 18 | 108 / 324 | 5,242,846 / 5,242,846 | 2,621,394 / 5,242,788 | 130,547,862 / 36 | 8.8 / 8.79 ; 2668628 / 2668452 |

複製したlocal行は1状態3行で線形。null/field/nonnullのmap複製行はこの入力では0。対してjoin生成はkが2増えるごとに約4倍になる。alias照合はscalar再帰内の `alias_terminal_with_expression_boundary` の実呼出し数であり、他consumerの呼出しを含めない。clone行数は要素数で、内部型のbyte数ではない。時間/RSSは一時計測を入れていない基準CLIで測った。全実行に `ulimit -v 4000000` と35秒timeoutを付け、小さいkから測定。計数hookはこのcommit前に全撤去した。

### J規則と接続先

J1. 合流のsourceの持ち主は既存のPrefixLocalFlow scalar producerとし、resolverのsource IDと既存Rcを用いる。
J2. 一回の不変なscalar照合の中で、bindingと元source IDが同じ完成済みのproducerだけを共有し、道を列挙しない。
J3. 両incomingの順序・元edge/valueとの照合・Boolの制約を通るまで成功結果を登録しない。
J4. activeなsource valueによる循環検出を保持し、失敗と未完成の結果は共有しない。
J5. local class・calls・expression plans・loop状態が変わり得る照合の外へ成功を持ち越さない。
J6. 枝ごとのHome/local状態は合流の既存の交差規則を保持し、片方の証拠で補わない。
J7. restore_if_normal_values_v1のNoneと取得・移転・Endおよび独立検証の責務は変更しない。
J8. 一時計数は撤去し、最終sourceのCLIでMIR・拒否・EXEの同一性と時間/RSSを確認する。

### P2: 同じ照合中の完成した合流を共有

既存 `scalar_producer_for_value_v1` の一回の不変照合へ完成表を閉じ込め、`(BindingRefV1, SourceValueRefV1)` ごとに既存producer/Rcを共有する。`scalar_producer_with_path_v1` はactive集合のinsert/removeで循環を検出し、全ての早期returnでもremoveする。失敗は登録しない。元flow/両incoming/Boolの確認とpath-local状態はそのまま。新しい型・関数・ID・構文・公開MIR・ABIは追加していない。

| k | scalar再帰 前→後 | join生成 前→後 | alias照合 前→後 | 最大深さ 前→後 |
|---:|---:|---:|---:|---:|
| 10 | 20,454 → 374 | 10,210 → 170 | 20,454 → 289 | 20 → 20 |
| 12 | 81,892 → 496 | 40,926 → 228 | 81,892 → 382 | 24 → 24 |
| 14 | 327,650 → 634 | 163,802 → 294 | 327,650 → 487 | 28 → 28 |
| 16 | 1,310,688 → 788 | 655,318 → 368 | 1,310,688 → 604 | 32 → 32 |
| 18 | 5,242,846 → 958 | 2,621,394 → 450 | 5,242,846 → 733 | 36 → 36 |

状態のclone回数/locals行数は全kで不変。path全複製は撤去した（後のpath_elementsはactive深さの合計であり、複製量ではない）。照合回数は同じ58回（k=18）で、各照合内で元のDAGを共有している。照合をまたぐprefix再検証は別の問題として残る。一時計数は全撤去し、最終CLIの測定と検証結果は今回の報告へ記録する。

### Opus の確認と統合（2b、2026-11-12）

main へ早送りした（`6212cbb900`）。

- **直った形**: 実装者の測った値（2 回ずつ）。
  - if の中で field を読む形: k=18 で 8.80 秒・約 2.6 GB → 0.11 秒・約 60 MB。k=40 は 0.37 秒、k=80 は 1.34 秒・83 MB で完走した（前は 4 GB の上限で止まった）。
- **Opus の未見の形**（メモリの上限 4 GB、40 秒で打ち切り）:
  - else のある形で、field を 2 つ読む: 前は k=20 で上限に当たった。今は k=20 で 0.18 秒、k=80 で 2.14 秒・99 MB。
  - field を条件と本体で読み、local を 2 つ更新する: k=80 まで、前も今も 0.06 秒以下。
  - Opus の監査の入力 110 本で、MIR と EXE の結果が前の main と全部同じ。
- **行数**: production は +35（inline の test の 10 行を含む）。
- **残り（2c）: loop の中の、field を読む if**
  - `loop(j < 2) { if x > i { if me.leaf.v > 0 { a = a + i } } … }` の形。メモリは 50 MB で増えなくなったが、時間はまだ if 1 つごとに約 2 倍（k=12: 0.06 秒、k=14: 0.30 秒、k=16: 2.30 秒、k=20 は 40 秒を超える）。
  - この入力はもともと拒否される（`object-input/callee-exit`）が、拒否までに時間がかかる。
  - 原因の見立て: `resolved_semantics/home_prefix_source_loop_ports.rs:91-135` の `producer_is_covered` が、`BranchNormal` の incoming を、訪れた印なしで再帰的にたどる。2b で前の if の合流を共有するようにしたので、共有された DAG を道の数だけたどる（実装者も、この walk を次の候補に挙げていた）。

## loop の中の walk（2026-11-12、基点 17c32b7a3c）

入口は home_new_prefix_scan.rs:453 → home_new_prefix_source_loop.rs:7/134 → home_prefix_source_loop.rs:680/700 → ports::verify_incoming。ports.rs:330からproducer_is_covered (:91-135) がBranchNormalのincomingとInitializer/BindingRebind内のscalar参照をvisited無しで再帰し、DAGを経路の数だけ読む。

| k | 問合せ数 | walk本体訪問 | 異なる保持objectの訪問合計 | 最大深さ |
|---:|---:|---:|---:|---:|
| 10 | 2 | 147,623 | 143 | 31 |
| 12 | 2 | 1,328,603 | 195 | 37 |
| 14 | 2 | 11,957,423 | 255 | 43 |
| 16 | 2 | 107,616,803 | 323 | 49 |

uniqueはquery内の保持Rc/参照objectのaddressを計数した値で、canonical IDの新設ではない。ifが2組増えるごとに本体訪問は約9倍、uniqueは小さく増える。Text coverage/tree4種・Recipe scalar/Text collector・Text unwindはこの拒否入力では全て0（Recipe/物理publication以前に止まる）。時間は一時計数無しの基準CLIで別に2回測る。計数hookはこのcommit前に撤去。

| 監査したwalk | 基点の状況 |
|---|---|
| ports scalar coverage :91-135 | visited無し。frame参照と両incomingの双方が再帰 |
| source_loop_v2.rs:333-338/381-387 | outputへ先に登録しshallow Eqでvisited済み。線形lookupだがbodyは1回。保持 |
| initializer_home_publication.rs:362-391 | Text unwindを全incomingへ再帰。全候補Vec一致は保持する |
| expression_reference_branch.rs:383-438 | Scalarはactive/completedと物理query境界あり。保持 |
| 同:299-351 | Textは各edgeから再帰しcompleted無し。edge検査を保持しbodyを共有する |
| source_loop.rs:89-165 / ports Text coverage :138-156 | Text witnessのcorroboration/owned/Header/containsが再帰。共通の1回のfoldへ |

L1. 問い合わせは固定のport、source prefixまたは不変MIRとprojectionに閉じ、完了集合を更新後へ持ち越さない。
L2. 同じsource座標とbindingの保持nodeは一回だけ評価し、別の証拠objectや物理receiptを同じnodeとして補わない。
L3. activeを完成結果の参照前に確認し、全return経路で解除して循環を拒否する。
L4. 全incomingの順序・元のedge/valueの照合・短絡する意味と元の拒否名を保持する。
L5. TextのHeader条件は各BranchNormal子自身にもHeader存在を要求し、rootに一つあるだけで補わない。
L6. Text unwindはqueryごとに全incomingの候補Vecの一致を確かめ、bindingごとの異なるunwindを混ぜない。
L7. 物理Textは各read edgeのloan/receipt/dependency照合を保ち、同じsourceでも異なる物理observationを省略しない。
L8. 共通化するのはqueryのactive/completed管理と同じText witness walkであり、sourceと生成後MIRの意味判定は統合しない。
L9. 一時計数を撤去し、最終commitの最適化binaryで拒否/MIR/EXEと速度を確認する。

### loop walkの接続と再計数

共通with_completed_source_node_v1はquery内のactive/成功結果だけを管理する。scalar coverageは同じ保持nodeのbodyを一回、Text witnessはcorroboration/owned/Headerを一つのfoldで計算。Text unwindはroot tree lawを閉じて全incomingの候補Vecを一致させる。物理Textは各edgeのloan/receipt/dependencyを保持し同じ物理observationのbodyだけ共有する。sourceの座標だけで別の保持証拠やreceiptを補わない。

| k | coverage body 前→後 | unique保持node 前→後 | 問合せ 前→後 |
|---:|---:|---:|---:|
| 10 | 147,623 → 143 | 143 → 143 | 2 → 2 |
| 12 | 1,328,603 → 195 | 195 → 195 | 2 → 2 |
| 14 | 11,957,423 → 255 | 255 → 255 | 2 → 2 |
| 16 | 107,616,803 → 323 | 323 → 323 | 2 → 2 |

深さは不変、後のbody訪問数とunique保持node数は全kで一致。Text/Recipe/physicalの未到達walkのCPU寄与を測ったとは主張しない。2つのRecipe collectorは既にvisitedがあり、型別の元義務も異なるので既存のまま保持した。Textのproducer/witnessだけを既存source_loop ownerの子moduleへ責務移管し、意味の持ち主は増やしていない。一時計数hookは撤去済み。

### Opus の確認と統合（2c、2026-11-12）

main へ早送りした（`80f2a162ee`）。

- 実装者の数え方: loop の中の field を読む if（k=16）で、walk の本体を訪れた回数が 107,616,803 回 → 323 回（訪れる node の数も 323）。拒否までの時間は 2.30 秒 → 0.03 秒、k=80 でも 0.18 秒。
- **Opus の未見の形**（受理される loop。メモリの上限 4 GB、40 秒で打ち切り）:
  - `loop(j < 2)` の本体に `if x > (i % 5) { a = a + i }` を k 個: main は k=40 で 40 秒を超えた。今は 0.23 秒。
  - 同じ形に else で `a = a - 1`: main は k=40 で 40 秒を超えた。今は 0.30 秒。
  - k=10・20 の EXE の値は、Python で計算した期待値と一致した（62・244、54・228）。k=40 は値が 255 を超えるので、lifecycle の I64 の入口の規則どおり、reason 102（`ExitCodeOutOfRange`）で `details[0]` に正しい値（968、936）を出して exit 70 になる。
  - Opus の監査の入力 110 本で、MIR と EXE の結果が前の main と全部同じ。
- **行数**: production は +271（純減ではない）。問い合わせの中の完成の表と active の管理、Text の 4 つの再帰を 1 つの fold にした分。新しい型は `CoverageNodeV1`・`TextBranchSummaryV1`・`TextObservationKeyV1`・`TextUnwindNodeV1`（ほかに既存の 4 つ組の alias `SourceBranchIdentityV1`）。持ち主の数の見張りの数は変わらない。
- 2b と 2c で、prefix の走査の倍々は、Opus が試した形では無くなった。

## 式の記録の一本化 1

Print と Return の式本体の保持形式を2種類から1種類へ置換する。builderの生成中状態とfinishing照合用の受領poolは別責務として残す。

| 消す物 | 移り先 |
|---|---|
| `EmittedPrintExpressionProjectionV1` | `ExpressionEmissionV1` と元のPrint frameの目的契約 |
| `ReturnExpressionEmissionV1` | 同じrecordと `ReturnExpressionContractV1` |
| 両旧型の values/calls/path_reads/reference_reads(original Return名 reads)/original/entry/inline_range/terminal(Print名 print)/continuation | 共通recordの各欄一箇所 |
| Printの分解・再構築、Returnの長い物理欄引数列 | 生成完了bodyを共通recordへ一度だけ移す入口 |
| 両旧型の expression() | 共通recordから借りる同じ ExpressionPhysicalProjectionRefV1 |

保持欄の宣言箇所は、上記9欄それぞれ用途別2箇所から共通1箇所へ。生成中bodyの1箇所は生成作業であって完成記録ではない。Printのpool受領bindingsはfinishing照合に必要なので余剰として削除しない。

| 用途 | 残す契約 |
|---|---|
| Print/discard | 原本Rc frameの同一性、表示かdiscard、操作命令、操作後の辺、元scopeと実CFG照合 |
| Return | 原本Rc評価、owner/exitと宣言された結果、返却値、residence/materialization、after-read、root Fault、所有移転とcleanup |

- X1: 式本体の完成記録は共通recordが一度だけ所有し、用途は固有契約だけを持つ。
- X2: Print評価をcloneして原本を作り直さず、元のRc frameから借りる。
- X3: 生成中の取得stack・homes等は完成記録へ持ち込まず、完成済みmaterializationとFault境界の証拠は保持する。
- X4: 元評価の検証、物理観測の照合、生成後MIRの独立検証を統合して弱めない。
- X5: 受理、拒否文言、cleanup責任と順序、公開MIRとABIは変えない。
- X6: 旧型への再梱包を作らず、旧保持型と重複view構築を同じseriesで削除する。
- X7: 新規共通処理とconsumerを含む整形後production純減が成立しなければ基盤追加を止め、依存を報告する。

worker経路: builder selected/expression_value.rs → selected/{print_expression,return_expression}.rs → local_commit/root_call_entry/print_expression_projection.rs と root_home_received_return.rs → expression_projection.rs の借用view → prefix pool / root exit → finishingと最終consumer。Local/Rebind/argument/field RHSと旧scalar laneは変更しない。

実装境界: `ExpressionEmissionV1<C>` の C は元Print frameのRc、またはReturn固有契約だけであり、旧packetを包まない。Return固有の契約・照合は `return_expression_emission.rs` へ分け、整形後のfile上限を守る。Printの `ordered_bindings(ledger)` / `validate_paths(ledger, ...)` は同じcontext付き入口へ直接繋ぎ、冗長な転送だけを削除する。
追加証拠の理由: 既存Print試験はframe・read/copy・保持の照合を守るが、両用途のCall ordinal、同値の別source、物理Endの欠落/重複、finishing後のproducer消失、Returnの未完保持を一緒に示していない。既存owner内の試験から共通の借用viewを変異させ、基点にも同じ試験を適用する。検証側へ期待命令生成器は足さない。

## 式の道の一本化の設計（ChatGPT Pro への相談、2026-11-12）

3 番目の回は大きいので、ユーザーは先に Pro に設計を相談する事を選んだ。要点:
- 前の回の「`take` の回で 50〜150 行減る」という見込みを、Pro は撤回した。共通の状態を作る費用だけを見て、古い保持の場所・引数・完了の処理・検証の入口を退役させる費用まで数えていなかった。
- 増え続けた主な理由: 状態の計算は 1 つにしたが、その状態へたどり着く道と、同じ観測の本体を使う所ごとに持つ記録は、1 つになっていなかった（`root_call_entry/expression_terminal_paths.rs`、`result_new_source.rs`、使う所ごとの `Emitted*` の記録）。
- 頼み方の決まり:
  - 消す型・欄・関数を、実装の前に名前で決める。
  - 新しい共通の部分とつなぎ先も含めた production 全体で、整形後に純減した時だけ main へ入れる。純減しない時は、共通の基盤をさらに足さずに止める。
  - 共通の記録から、古い使う所ごとの型へ詰め直す adapter と、古い型を包むだけの enum は禁止。元の source の契約を借りる薄い入口と、既にある唯一の writer への境界は許す。
- 持ち主の数の見張りは、同じ tree の中の基準と実数の一致しか見ていなかった。基点の基準と比べる事と、退役した名前の一覧を足す。
- 記録をやめて最終の MIR から探す案は採らない（source との対応は最終の MIR だけでは決まらない）。1 つの観測の本体を持つ record を作り、使う所には行き先の契約だけを残す。
- 段の順番（各段は、追加分を含めて純減した時だけ入れる）:
  1. Print と Return の式の記録
  2. Local の initializer の記録、代入し直しの記録
  3. 移った使う所の builder の終わりの処理
  4. 普通の Call の引数、`new` の引数、field の右辺の式の記録
  5. 物理検証の context の明示（要る時だけ）
  6. Return の返却内容の排他的な保持（`RootHomeCleanupOrderV1`。全部の `Option` を平らな enum にはしない）
  7. Local と Return の選択を、それぞれ 1 回にする（先着順の連鎖）
  8. 古い scalar の道の、対応済みの責務を退役
  9. 最後に Bool の比較の operand と条件の全体
- 相談文と答えの全文は、Opus の作業 dir に保存した（repo には入れない）。

### 1 段目: Print と Return の式の記録（Codex 実装・Opus 監査、`3101b5f5e9`、2026-11-12）

main へ早送りした。
- `EmittedPrintExpressionProjectionV1` と `ReturnExpressionEmissionV1` を退役させ、共通の `ExpressionEmissionV1<C>` に置き換えた。同じ観測の本体（値・Call・参照・元の命令列・入口・出口など 9 つの欄）を持つ完成の記録は、2 種類から 1 種類になった。
- 行数: production は整形後 −22（新しい共通の部分を含む）。純減の条件は満たした（Pro の見込みは 40〜100 行の減り）。test は +284（Print と Return の変異の拒否を示す契約の証拠）。
- 持ち主の数の見張りに、基点の基準との比較と、退役した名前の一覧（`tools/checks/mir_owner_retired_symbols.txt`）を足した。`emitted_projection_types` は 7 → 6。基準と実数を一緒に上げる変更、改名、別 file への移動が失敗する事を、実装者が試した。
- 変異（Call の ordinal の入れ替え、別の source の同じ値の producer、End の欠落と重複、finishing での producer の消失、記録の不在）は、前と後で同じ文言で拒否される（18 件）。
- Opus の監査の入力 118 本で、MIR と EXE の結果が前の main と全部同じ。Opus も見張り（基点 `ae144836c4`）を走らせて PASS。
- 次の段は Local の initializer の記録。dynamic の Call、install と materialization、owned な root と結果の契約が要る（実装者の報告）。

## 式の記録の一本化 2a

Local initializerの完成記録を既存の `ExpressionEmissionV1<InitializerExpressionContractV1>` へ置換する。

| 消す物 | 移り先・残す責務 |
|---|---|
| `EmittedInitializerExpressionProjectionV1` | 共通recordとLocal固有の契約 |
| values/calls/path_reads/reference_reads/entry/inline_range/terminal/continuation/original の9欄 | 既存共通recordの9欄（追加の保持なし） |
| Localの expression() の本体の再配布 | 共通recordから借り、Local契約のDynamic callだけを借用viewへ添える initializer_expression_v1 |
| builderの9欄への分解と長いconstructor引数列 | 完成bodyを共通from_bodyへ一度だけ移管 |
| frame/value/local/copy/install | 元Rc frame・正しいbinding/initializer/local・必要なCopyとresidence installのLocal契約 |
| dynamic_calls | Local契約。Dynamic carrierの元site・結果handoff照合に必要であり共通本体へ所有させない |
| loop_residency | Local契約。LoopのText carrierと当該LocalでのFault unwindの証拠 |
| purpose_acquisition | Local契約。Local snapshotのMaterialize対とinstall責任。Returnの類似対も既にReturn固有のresidence契約にあり、共通本体へは混ぜない |

完成recordの保持形式は共通＋旧Localの2種類から共通1種類へ、対象9欄は18宣言から9宣言へ。生成中bodyとfinishing用受領poolは別責務として保持する。

- Y1: Localの式本体は共通recordだけが所有し、Local契約は行き先固有の8欄だけを持つ。
- Y2: 元のinitializer Rc frameから評価を借り、同値の別frameを作り直さない。
- Y3: Dynamic・alias・Copy・loopの固有証拠はLocal契約に留め、共有viewへ渡す時も借用だけにする。
- Y4: Copy・install・元sourceとbindingの一致、Normal/Fault・finishing・独立検証の条件と拒否文言を保持する。
- Y5: 受理選択、旧scalar、Bool短絡、公開MIR/ABI、RefCell context、他用途の記録は変えない。
- Y6: 旧型・重複欄・再梱包を同じseriesで削除し、退役symbol検査へ追加する。
- Y7: 全新規部分とconsumerを含む整形後production純減を満たさなければ基盤追加を止め、依存を報告する。

worker経路: selected/initializer_expression.rs の prepare→PendingEvaluatedInitializerV1::publish→initializer_expression_projection.rs→local_binding_group.rs→local_entry.rs/finishing/finalized consumer。Local専用viewは既存new_with_dynamicと同じ借用情報を用い、Print/Returnの共通expression()は変更しない。
