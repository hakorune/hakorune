# 所有の解析の担当を、関数ごとに 1 か所で決める(S3b 関数ごとの切り替え 1 段目の規則)

Status: 規則の文書です。authority ではありません。branch `s3b-route`(main `83ef9aeee8` から)。実装は、この文書の文を写します。文に無い分岐が要る時は、先にこの文書へ行を足して理由を書きます。

背景: 点検 `s3b-plan-review-2026-11-07.md` の 1.4 節と 4 節の 1〜4、Pro の回答 `s3b-ownership-cfg-consult-2026-11-07.md` の 5 節、`mirbuilder-inplace-replacement-policy-ssot.md` の 255、356、369 行の付近。

性格: **受理を変えない置き換え**です。通る形、拒否の名前、生成される MIR は前と同じです。新しい解析は足しません(2 段目)。

場所: `normal_callable_semantic_package/ordinary_new_coseal_issue.rs` の `issue_ordinary_source_cohort_v1` の、関数ごとの closure の中(以下の行番号は `83ef9aeee8` のもの)。

## 担当の種類

- **S3b-R1 担当の型**: 1 つの関数の所有の担当は、1 つの型 `OwnershipRouteV1` の値で表す。
- **S3b-R1a 担当の中身**: 担当は、走査を通らずに発行する物(`PlainIssuanceV1`)と、走査の道(`ScanRouteV1`)の組である。
- **S3b-R1b 発行する物**: 走査を通らずに発行する物は 2 つで、`seed` と `app_main_integer` であり、互いに独立である。
- **S3b-R1c 走査の道**: 走査の道は 2 つで、`VerifiedWalk` と `PrefixOnly` である。
- **S3b-R1d 今ある担当と、選ぶ式の場所**:

| 担当 | 今それを選ぶ式 | 使う所 |
|---|---|---|
| `seed`(走査を通らない plain の completion を seed に入れる) | `seed_completion`(`:401-421`) | `issue_plain_completion`(`:429`、`ordinary_new_coseal_issue_completion_seed.rs:47-57`) |
| `app_main_integer`(app main の整数の結果の completion を root に置く) | `app_main_integer_result`(`:422-428`、`plain_main_integer_result_v1`) | 同(`completion_seed.rs:35-46`) |
| `VerifiedWalk`(走査つきの completion) | `verified_walk`(`:440-456`) | `:470` の枝、`verify_function_completion_with_new_homes_and_argument_observations_v1`(`:535`) |
| `PrefixOnly`(prefix だけの走査) | `!verified_walk`(`:470` の `else`) | `issue_new_home_prefixes_with_arguments_v1`(`:726`) |
| completion が失敗した app main | `VerifiedWalk` の中の `Err` 腕(`:703` 付近) | 同(`:714`)。担当ではなく、`VerifiedWalk` の結果の 1 つ。今のまま、同じ場所に残す |

- **S3b-R1e 発行と担当は別**: 他の関数に届く発行(`:457` の `borrowed_formal_actuals`)と、走査の前の readiness の probe(`:338`、`:350-364`)は、担当ではない。

## 決める所は 1 つ

- **S3b-R2 決める所**: 担当は `OwnershipRouteV1::decide` が 1 回だけ決め、`seed_completion`、`verified_walk`、`app_main_integer_result`、`child_new_ready`、`child_result_ready` を別々の式で作らない。
- **S3b-R2a 互いの否定になっていない所**: `seed` と `VerifiedWalk` は互いの否定ではなく、両方が真になる関数がある。
- **S3b-R2b 両方が真になる場合**: 両方が真になるのは、`owner_loan` を持つ app main(`seed` は `is_app_main || owner_loan.is_none()` で許す。走査は `owner_loan.is_some()` で要る)と、構築を持つ app main(`seed_eligible` で `child_new_ready` が偽の時。走査は `is_app_main && (new_sites || has_map || result_sites)` で要る)であり、今は `seed` を発行してから走査も行うので、この順序を変えない。
- **S3b-R2c 他の独立な物**: `app_main_integer` は `seed` とも `VerifiedWalk` とも独立で、`plain_main_integer_result_v1` の答えだけで決まる。
- **S3b-R2d 入力の事実**: 担当を決める入力は、`seed_eligible`、`is_app_main`、`owner_loan` の有無、`new_sites` と `result_sites` が空か、`top_level_input` の有無、readiness、要求の集合である。
- **S3b-R2e 式**: `seed` は `(seed_eligible || (top_level_input && new_sites が空 && result_sites が空)) && 要求のどれも seed を止めない && !child_new_ready && !child_result_ready && (is_app_main || owner_loan 無し)` で、`VerifiedWalk` は `要求のどれかが走査を要る || (is_app_main && (new_sites || MapLiteral || result_sites)) || (seed_eligible && (MapLiteral || child_new_ready || child_result_ready))` であり、今の式の項と同じ項を同じ意味で使う。

## 要求の表

- **S3b-R3 要求**: 走査が要るかは、関数の中の操作と契約が出す「要求」の集合で決める。
- **S3b-R3a 要求の 1 行**: 要求の 1 行は、操作か契約の名前と、2 つの性質(走査を要るか、`seed` を止めるか)を持つ。
- **S3b-R3b 性質**: `MapLiteral` は走査を要るかを単独では決めず(`VerifiedWalk` の式の `MapLiteral` の項が決める)、`DirectCallOwnerLoan` は `seed` を止めるのが app main でない時だけで、他の行は両方を満たす。
- **S3b-R3c 評価の順**: 要求を集める式の評価の順は、今の `has_*` を評価する順と同じにし、`?` で返る失敗の順番を変えない。
- **S3b-R3d 同じ問い**: 同じ問いに答える `has_*` は 1 行にまとめる。

### 今の `has_*` と要求の行の対応

| 今の名前(`:行`) | 要求の行 | 理由 |
|---|---|---|
| `has_typed_static_walk`(`:220`) | `TypedStaticCall` | 型の付いた static call |
| `has_map`(`:281`) | `MapLiteral` | Map literal(値としても実引数としても) |
| `has_integer_return`(`:326`) | `IntegerReturn` | 整数の return を借用の source が要る契約 |
| `has_formal_field_read`(`:328`) | `FormalFieldRead` | formal の field read |
| `has_dynamic_local_initializer`(`:331`) | `DynamicLocalInitializer` | Dynamic の呼び出しの local 初期化 |
| `has_borrowed_walk`(`:335`) | `BorrowedFormalWalk` | 借用の formal を持つ呼び出し |
| `has_receiver_home_call`(`:379`) | `ReceiverHomeCall` | receiver の Home の呼び出し |
| `has_me_object_field_read`(`:383`) | `MeObjectFieldRead` | `me` の object field の read |
| `has_stored_terminal`(`:387`) | `StoredTerminal` | 保存された終端 |
| `has_field_scalar_walk`(`:388`) | `FieldScalarWrite` | field の scalar の書き込み |
| `has_required_scalar_source_walk`(`:389`) | `RequiredScalarSource` | scalar の source を要る契約 |
| `has_static_source_local`(`:392`)と `has_current_owner_i64_source`(`:317`) | `StaticSourceLocal` | 同じ問い(static の source の local)。今も `||` で 1 つにしている |
| `has_print_expression`(`:395`) | `PrintOperand` | print の operand の式 |
| `has_condition_expression`(`:396`) | `ConditionOperand` | 条件の式 |
| `has_value_expression_walk`(`:397`) | `ValueExpression` | explicit copy と initializer の式(今も `||` で 1 つ) |
| `has_text_return`(`:398`) | `TextReturn` | Text の return |
| `has_array_residence`(`:399`) | `NamedArrayResidence` | 名前付き配列の residence の契約 |
| `owner_loan.is_some()`(`:443`) | `DirectCallOwnerLoan` | 直接呼び出しの owner loan |
| `callable_result_classes.has_object_call_return(owner)`(`:404`、`:444`。2 か所) | `ObjectCallReturn` | object を返す呼び出し(2 か所の式を 1 つにする) |
| `has_borrowed_source_calls`(`:323`) | (要求ではない) | 他の関数の `borrowed_formal_actuals` を埋める発行の条件(R6) |
| `child_new_ready`(`:365`)、`child_result_ready`(`:372`) | (入力の事実) | R2d。readiness は走査の結果なので、要求の行にならない(R5) |

- **S3b-R4 形の名前を足さない**: 要求の行の名前は操作か契約の名前で書き、新しい `has_*` を足さない。
- **S3b-R4a 名前が書けない物**: 操作か契約の名前で書けない物は今の名前のまま残し、理由を書く。`BorrowedFormalWalk` は「借用の formal を持つ呼び出し」の判定が `has_original_borrowed_walk_v1` の中の複数の条件に分かれていて、1 つの操作の名前にならないため、判定を呼ぶ関数を今のまま使う。

## 走査の前の probe

- **S3b-R5 readiness**: 担当を決める前に旧走査を回す probe(`:338`、`:350-364`)は、表で答えられる部分(`seed_eligible && new_sites が空でない` の条件)を担当の入力の事実として呼び出し側に残し、走査の結果でしか分からない物(`PrefixNotCovered` を含む prefix の全部が Ok か)だけを probe に残す。
- **S3b-R5a 残す理由**: probe を残す理由は、その答えが `child_new_ready` として `seed` と `VerifiedWalk` の両方の式に入り、probe の失敗(`?`)が今の受理に効くから(省くと失敗を受理に変える)である。
- **S3b-R5b 位置**: probe を評価する位置は、`has_borrowed_walk` の後で `has_receiver_home_call` の前のままにする。
- **S3b-R5c 数**: 担当を決める前に旧走査を回す所は、`:338` の経由の 1 つで、前と後で同じである。

## 他の関数への副作用

- **S3b-R6 発行**: `:457` の probe は担当ではなく、`borrowed_formal_actuals` を埋める発行であり、担当の型の外に置く。
- **S3b-R6a 走る時**: この probe が走るのは、担当が `PrefixOnly` で `has_borrowed_source_calls` が真の時で、走る位置は `issue_plain_completion` の後、走査の枝の前である。

## 診断

- **S3b-R7 診断**: 拒否の名前と、拒否が出る順番を変えない。

## Opus の確認(2026-11-07)

- 規則の commit `17eba45ac4` は実装より前。
- `decide` の式を、前の `seed_completion`、`app_main_integer_result`、`verified_walk` と項ごとに比べた。`candidates.is_empty()` は `new_sites` が空かと同じ(`new_sites` は `candidates` から作る)。`?` で返る判定の順番も前と同じ。
- 最後の binary と test binary は `f862402e71` の木から build されていて、その後に変わった source は無い。途中の `f21a33e949` も単独で `cargo check --lib` が通る。
- 前の監査で使った入力 1,459 本(内容の重複と 329 入力を除いた物。通る 835、拒否 624)で、歩 B/A の前(`57a26283b6` の木)、歩 B/A の後(`421b82d74c`)、この series(`f862402e71`)の 3 つの binary の `--dump-mir` を比べ、全部同じだった。
- 残り: 要求の行は「走査を要るか」と「seed を止めるか」の 2 つの性質だけを持ち、走査の何(出口の関係、prefix、局所の呼び出しの行など)が要るかは、まだ持たない。新しい解析が対応できる操作の集合と比べるには、この欄が要る。
