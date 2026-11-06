# 値の種類を決める持ち主(S3a 段階 0。地図と型の規則)

Status: 測定と読解の記録です。authority ではありません。branch `mir-s3a`(main `f3c2e9bdb2` から)。設計は `mirbuilder-structure-refactor-queue-d0-2026-08-23.md` の S3、前の調査は `mirbuilder-route-selection-survey-2026-11-06.md`。

## 1. 問い

「この式 / この method の結果は i64 か」を決める場所が、形ごとに複数あり、どれにも「何を受理するか」の仕様がない。この記録は、決める場所を挙げ、同じ問いに答える組の差を書き、持ち主と型の規則を決める。

## 2. 値の種類を決める場所の地図(method の結果が i64 かを決める判定)

| # | 場所 | 入力 | 受理する集合(読んで書いた仕様) | 使う人 |
|---|---|---|---|---|
| A | `source_result_pending`(`ordinary_new_borrowed_formal_result_pending.rs` の `site_class` の腕の並び) | callee の return の式 1 つ | 整数 literal、exact な formal、checked return、Mul 単体(借用 Mul の product)、formal の field、call、field array read は即 I64 か Unresolved。それ以外は式の計画を `expressions` に積み `UnresolvedScalar` | 結果 cohort `BorrowedResultCohortV1` |
| B | `ResultExpressionSourceV1::class`(`ordinary_new_borrowed_formal_result_expression.rs`)と `grounded_with_fields_v1`(`..._composition.rs`) | A が積んだ式の計画と、leaf ごとの根拠 | leaf の種類(Integer の根拠、field、call、declared read、…)から `project_expression_class_v1` で I64/Bool を子から決める。Divide と比較も I64/Bool と決める(`binary_expression_class_v1`) | A の `class` を確定する(`resolve_pending_source_classes_v1`) |
| C | `lexical_i64_result_call` と `exact_i64_return_expression_v1`(`ordinary_new_coseal_issue_lexical.rs`、**今回消した**) | callee の return の式 | literal、exact formal、`me.<i64 field>`、その `+ - *`。local、call、`/`、比較は受理しない | lexical `recv.m(..)` の i64 結果の membership(封印より前の走査) |
| D | `uniform_call_result_kind`(callee の `TerminalRelationV1` の列から結果の kind を作る。`ordinary_new_lexical_instance_call.rs`) | callee の終端 relation | `IntegerLiteral`、`I64Add` など、終端で I64 と作られた物が全部 I64 | `InvokeCallResultKind::I64` を作る。C の membership の後に corroborate する |
| E | 呼ぶ側の `LocalCallResultClassV1`(`home_new_prefix.rs`)と `home_new_prefix_local_statement.rs` の腕の連鎖 | 呼び出しの種類(直接、static、lexical、received)と package の closure の答え | 結果の値の種類(I64、Bool、Nullable、Map、Handle、Text、Dynamic)ごとの腕。`LocalCallResultClassV1::I64` の言及は production で 41 行 | Local の初期値、print、条件 |
| F | 式の値の種類 `ExpressionValueClassV1` と `ExpressionLeafClassV1` | 式の計画の leaf | I64、Bool、String、Null、Dynamic。leaf が決める | B と呼ぶ側の式評価 |
| G | source-sum graph の `SourceValueAlternativeV1`(`ordinary_new_borrowed_formal_source_sum*.rs`) | formal の入力(実引数の seed)、field の writer | 閉じた和(SignedI64、Bool、Text、TaggedDynamic、…)。formal の和は実引数の seed の合併 | B の Integer の根拠(`integer_origin_at_v1`) |
| H | 物理: `ExactTrivialScalarAbiV1`(return の注釈)、`ExactTrivialParameterAbiV1`(formal の注釈) | 宣言の型の名前 | `i64`(と parameter だけ `usize`)。意味の型ではなく、物理 ABI の綴りの権威 | 宣言の検査、C の `all_formals_exact_i64` |
| I | 演算子の許可: `home_scalar_operation_preflight.rs`(位置と形つき)、`home_new_prefix_condition.rs` の 2 つの集合、`home_new_prefix_initializer_expression.rs` の `root_add`、`binary_expression_class_v1`、`exact_i64_return_expression_v1`(今回 `binary_operator_cannot_fault_v1` に変えた) | 演算子と位置 | `Multiply` の言及は production で 92 行 / 62 file | 位置ごとの gate |

### 数える基準
「method の結果が i64 かを、独立に決める場所」を数える。A、B(A を確定する側。1 つの場所として数える)、C、D の 4 か所が前。後は C が B の結果を聞くだけになり、3 か所(A、B、D)。E、F、G、H、I は問いの別の面(使い先の分類、式の根拠、入力の和、物理の綴り、演算子の許可)であり、この series の後半(段階 2、4)で扱う。

## 3. 組と受理する集合の差(読んで書いた。確かめたのは `lexical_i64_local_call_tests` と手書きの入力)

### 組 1: C と B(これが S2b で落ちた 4 本の原因)
| 形 | C(前) | B(前) |
|---|---|---|
| `return x * x - 3`、`(x + 1) * 2`、`a * 10 + b` | 受理 | 封印前の走査では `UnresolvedScalar`。Mul を借用 Mul の product として要求し、exact な formal だけの Mul に product が無く、封印で `mul-entry-owner` の Err |
| `return me.base + x`(`base: i64`) | 受理 | 受理(field の根拠が census にある時) |
| `return me.base + x`(`base` に型なし) | 拒否 | **受理**(writer の census が全部 i64 なら field も I64 と決める) |
| `return x / 2` | 拒否 | I64 |
| `return x > 2` | 拒否 | Bool |
| `local y = x return y` | 拒否(local を受理しない) | `UnresolvedScalar` のまま封印で解けない(local の leaf が根拠を持てない)。さらに formal `x` の source-sum が `None`(実引数の seed が exact な formal を数えない) |

差の意味:
- Divide は I64 の**種類**だが、zero で Fault する。C が受理しなかったのは、種類ではなく「Normal だけで値が出るか」(Fault の軸)を見ていたため。
- 型なしの field は census では I64 と**言える**が、物理の表現は raw i64 とは限らない(tagged dynamic の可能性)。C の「宣言が `i64`」は物理の軸。
- つまり C と B は別の軸を1つの集合に混ぜていた: 意味の種類(I64 か)、Normal だけか(Fault の軸)、物理が raw i64 か。

### 組 2: D と C
D は callee の終端 relation から I64 を作る。C が membership(封印より前)で、D が mint(封印の後)。同じ式を 2 回、別の言葉で判定している。今回は C を B に聞く形にし、D は変えない(段階 4 の候補)。

### 組 3: 位置ごとの演算子(段階 2 の対象)
- `home_scalar_operation_preflight.rs`: `Multiply` は「Local の初期値の root で、`(literal + trivial local) * trivial local`」か「`trivial local * literal`」の 2 形だけ。`Divide | Modulo` は「root で右が正の literal」だけ。これは**位置と形の特例**であり、演算子の表ではない。
- `home_new_prefix_initializer_expression.rs` の `root_add` は `Add` だけ。S2b の実験では `Add | Multiply` に広げると `local r = Main.a(1) * 2` が通った。
読んだ限り、位置ごとの差に意味がある証拠は見つかっていない(Multiply の 2 形は `normal_integer_mul_available()` を介した過去の段階の実装の名残に見える)。段階 2 で表に置き換える。

## 4. 持ち主の候補

新しい型を足す前に、今ある物で足りるか。

| 候補 | 足りるか |
|---|---|
| 結果 cohort `BorrowedResultCohortV1`(A+B) | 式の子から I64/Bool を決める木(`project_expression_class_v1`)と、leaf ごとの根拠を持つ。**足りる**。足りなかった所は 3 つ: local の leaf、exact な formal の source-sum、Mul の product の要求。いずれも持ち主の中の穴であり、別の分類器は要らない |
| `ExactTrivialScalarAbiV1` | 物理 ABI の綴り(`i64`)の権威。意味の型の持ち主にしない(軸を混ぜない) |
| `CheckedExpressionEvaluationV1` | Normal/Fault の軸の持ち主。今回の `binary_operator_cannot_fault_v1` は暫定の 1 か所で、段階 2 の表の列になる |
| `ExpressionLeafClassV1` | leaf の種類の語彙。B が使っている |

決定: **持ち主は結果 cohort の式の木(B)**。C はこれに聞くだけにする。新しい分類器は足さない。

## 5. 型の規則(構文の種類ごとに 1 行。実装はこの写し)

| 構文 | 規則 |
|---|---|
| 整数 literal | I64 |
| bool literal | Bool |
| 変数の参照(formal) | その formal の宣言の型。`: i64` なら I64(実引数の seed が無くても) |
| 変数の参照(local) | その local の sole initializer の式の型。rebind、capture、初期値なしの local は決めない |
| `me.<field>` | その field の根拠(census)が I64 の時 I64。物理が raw `i64` かは別の軸(使い先の制限) |
| 二項演算 | `binary_expression_class_v1` の表を引く(左の種類、右の種類 → 結果の種類か拒否) |
| 呼び出し | callee の結果の種類(宣言があれば宣言、無ければ本体の return から。再帰は不動点) |
| Core method の呼び出し(`recv.length()` など) | 結果の種類は Core method の表の行(`expression_result_class_v1`)。受け手が宣言済み field の読みなら、その読みは呼び出しの operand で、Core の source が閉じる。受け手を値の種類として分類しない(局所の初期値でも同じ) |
| `return e` | 宣言の結果の型と `e` の型が合わなければ拒否 |

### 使い先の制限(種類とは別の軸。1 か所に書く)
- exact-`i64` の lexical 結果の lane(`lexical_i64_result_call`)は、種類が I64 であることに加えて、(1) return の式が Normal だけで値が出る(call、`/`、`%`、shift、Unary、Conditional を含まない)、(2) 値が raw `i64`(literal、exact formal、宣言が `i64` の field、それらの local と `+ - *`)であること、を要求する。実装は `ResultExpressionSourceV1::is_raw_normal_i64_v1`。
- 意味の種類が I64 でも、この lane に入らない形(`x / 2`、型なしの field)は、以前の受理と同じく入らない。

## 6. 規則と実装の対応(段階 1 の結果)
- 「formal = 宣言の型」: `ordinary_new_borrowed_formal_source_sum.rs` の formal の和の確定。実引数の seed が無い `: i64` の formal の和は `{SignedI64}`。
- 「local = 初期値の型」: `ResultExpressionSourceV1` の leaf `Local`。初期値の式の計画から同じ木を作る(`local_initializer_plan_v1`)。
- 「Mul は種類の演算」: exact な値同士の Multiply は借用 Mul の product を要求しない(`corroborates_no_borrowed_mul_returns_v1`)。
- 「C は聞くだけ」: `lexical_i64_result_call` は `BorrowedResultCohortV1::is_normal_i64_result_v1` を引く。
