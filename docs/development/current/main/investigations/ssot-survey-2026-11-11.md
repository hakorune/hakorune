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
2b. **prefix の走査の倍々の遅さ**（Opus が見つけた、2026-11-12）: if の中で field を読む形で、時間とメモリが if 1 つごとに約 2 倍になる（下の「残る倍々の遅さ」）。合流で状態を 1 つにまとめ、道を数え上げない形にする。メモリが尽きる危険があるので、式の道の一本化の回より前に、小さな回で直す。
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
