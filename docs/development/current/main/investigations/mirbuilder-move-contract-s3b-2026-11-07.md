# `move` の確認を 1 つの契約の関数にする(S3b 案 3 の 1 回目の規則)

Status: 規則の文書です。authority ではありません。branch `s3b-move`(main `2f26cc2640` から)。実装は、この文書の文を写します。文に無い分岐が要る時は、先にこの文書へ行を足して理由を書きます。

背景: `s3b-analysis-options-2026-11-07.md` の 1.3 節と 7 節(案 3)、規則 `mirbuilder-ownership-transition-s3b-2026-11-07.md`(S3b-R2b、R2g、R8)、仕様 `docs/reference/language/ownership.md` の `move` と使用後 move の節。

性格: **受理を変えない置き換え**です。通る形、拒否の名前、拒否の site、拒否が出る順番、生成される MIR は前と同じです。行番号は `2f26cc2640` のものです。

## 規則

- **M1 問い**: `move` かどうかは `ResolvedExpressionSourceV1::moved`(`expression_source.rs:357`)だけが答え、operand を読む所は `stored_value_operand`(`:351`)だけが答える。同じ式を他で書かない。
- **M2 move の入力の確認**: `move` の operand を local の binding に解く事は、`PrefixLocalFlow::move_operand_binding_v1` が答える。答えは binding か `MoveUnsupported(move の site)` である。
- **M2a 使えるかまで確かめる読み**: local の `move` と `share move` は、binding を解いた直後に、その binding がまだ使えるかを M3 の関数で確かめる(`live_move_operand_v1`)。
- **M3 使用後 move の規則**: 「使えなくなった(`MovedOut`)binding を読んだら `UseAfterMove`」は `PrefixLocalFlow::use_after_move_v1` が 1 か所で書く。文ごとの掃き出し、呼び出しの中の評価順、点の確認は、どの参照をいつ確かめるかだけを選び、規則はその関数を呼ぶ。
- **M3a 点の確認の扱い**: 点の確認(local の `move`、`share move`)が `UseAfterMove` を返す位置は、今のまま、binding を解いた直後で、同じ文の掃き出しより後である。掃き出しが先に同じ失敗を出していれば、その失敗が残る(`get_or_insert`)。2 つが出す site は違う(掃き出しは参照の site、点の確認は `move` の operand の site)ので、どちらも残す。
- **M4 同じ scope**: local の move と share move が責任を残せるかの「同じ scope」の確認は `same_scope_v1`(`home_ownership_transition.rs:70`)が答え、失敗を `MoveUnsupported(move の site)` にする所は `PrefixLocalFlow::move_scope_v1` の 1 つにする。
- **M4a 残す物**: `proves_original_move`(`local_statement.rs:125` の closure)は 1 つの closure を 2 か所が呼ぶ形で、既に 1 つである。share move と local move は元の取得の選び方(`home_acquisition` と `movable_acquisition_v1`)と、Shared の元に対する証明が違うので、その後の確認は 1 つにしない。
- **M4b 同じ scope の別の呼び出し**: `install_shared_clone_v1`(`home_prefix_local_flow_shared.rs:101`)の `same_scope_v1` は、遷移の中の前提の確認で、拒否の理由を返さない(`bool`)。M5 に属するので触らない。
- **M5 選ぶ遷移**: 確認が通った後に呼ぶ遷移(`move_alias`、`install_shared_move_v1`、`release_to_callee`、`transfer_out` ほか)は変えない。
- **M6 診断**: 拒否の名前、拒否の site、拒否が出る順番(最初の失敗が残る `get_or_insert` の順)を変えない。
- **M7 package 側**: package 側の `moved(` は、issuer が claim の事実(値の class、seed、provenance、construction の store)を作る所で、binding を解く事も使用後 move を決める事もしないなら、残す。同じ判断(M2〜M4)を出し直している所だけを 1 つにする。

## 確認の一覧(`moved(`、`stored_value_operand`、`UseAfterMove`、`MoveUnsupported`、`same_scope`)

分類: M1 = 問いの呼び出し(1 つにする物ではない)。M2/M3/M4 = 後の確認(1 つにする)。独立 = package 側の別の問い(残す)。

| file:line | 何を確かめるか | 出す拒否 | 分類 |
|---|---|---|---|
| `resolved_semantics/expression_source.rs:351,357` | `stored_value_operand`、`moved` の定義 | - | M1 の持ち主 |
| `home_new_prefix_local_statement.rs:177-189` | `share move x`: `moved` → operand を binding に解く → `MovedOut` なら拒否 | `SharedOperandUnsupported`(`moved` でない)、`MoveUnsupported(moved.site)`、`UseAfterMove(operand)` | M2、M3 |
| 同 `:197-199` | `share move` の同じ scope | `MoveUnsupported(moved.site)` | M4 |
| 同 `:200-205` | `share move` の元の取得と `proves_original_move` | `MoveUnsupported(moved.site)` | M4a(残す) |
| 同 `:222-231` | `local b = move a`: operand を binding に解く → `MovedOut` なら拒否 | `MoveUnsupported(site)`、`UseAfterMove(operand)` | M2、M3 |
| 同 `:233-235` | 同じ scope | `MoveUnsupported(site)` | M4 |
| 同 `:236-256` | 元の取得、`proves_original_move`(Shared の元は別の証明)、`move_alias` | `MoveUnsupported(site)` | M4a、M5(残す) |
| 同 `:125-133` | `proves_original_move` の closure。`stored_value_operand` を呼ぶ | - | M1 の呼び出し |
| `home_call_entry_transfer.rs:79-90` | 呼び出しの評価順の模擬: `moved` → operand を binding に解く → 取得が無ければ拒否 | `MoveUnsupported(site)` | M2(取得の確認は残す) |
| 同 `:108-115` | 評価順の中で、読む参照と receiver が `MovedOut` なら拒否 | `UseAfterMove(site)`、`UseAfterMove(receiver_site)` | M3 |
| 同 `:121-127` | 条件付きで届く子、未知の子を持つ式の拒否 | `MoveUnsupported(site)` | M2〜M4 でない(式の対応範囲の拒否)。残す |
| 同 `:173` | take が呼び出しの observation に無い | `MoveUnsupported(call)` | M2〜M4 でない。残す |
| `home_new_prefix_scan.rs:128-141` | 文ごとの掃き出し: 文の部分木の local 参照が `MovedOut` なら拒否 | `UseAfterMove(site)` | M3 |
| `home_new_prefix_new_local.rs:171` | `new` の実引数が `move` か(`is_some()`) | - | M1 の呼び出し |
| 同 `:234` | `validate_pending_call_uses_v1` を呼ぶ門 | - | M3 の呼び出し側(門)。残す |
| `home_prefix_observe.rs:90` | 実引数が `move x` なら operand の観測を読む | - | M1 の呼び出し(`observe(site)` が先で、`moved` が後の順序なので `stored_value_operand` には置き換えない) |
| `home_local_call_borrowed_actuals.rs:99-100` | `moved.map_or(site, operand)` | - | **`stored_value_operand` と同じ式を書いている**。M1 で置き換える |
| `home_map_flow.rs:176-179` | 出口の失敗のうち、所有の違反を取り出す。分岐の食い違いを `UseAfterMove` に読み替える | `UseAfterMove(site)` | M3 でない(失敗の分類・再解釈)。残す |
| `home_ownership_transition.rs:70`、`home_prefix_local_flow_shared.rs:101` | `same_scope_v1` の定義と、clone の遷移の前提 | `bool` | M4 の持ち主、M4b |
| package `ordinary_new_borrowed_formal_actual_seed.rs:359` | 実引数の seed の class は move の operand の class と同じ | - | 独立(M7) |
| package `call_argument_source.rs:59` | claim の `moved` 行が source の行と一致するか | reject | 独立(M7。claim と source の照合) |
| package `ordinary_new_borrowed_formal_new_field_source.rs:273` | 初期化が `move` の連鎖か | - | 独立(M7) |
| package `ordinary_new_lexical_instance_call_provenance.rs:415` | move は nominal class を保つ | - | 独立(M7) |
| package `instance_construction.rs:516-519`、`:414` | construction の store が `move` か、`take` の formal が文の中で読まれたら | `U::UseAfterMove` | 独立(M7。`take` の formal の別の問い。`MovedOut` の binding は読まない) |
| package `ordinary_new_field_write_claim.rs:574`、`..._borrowed_formal_source.rs:295`、`..._lexical_instance_call_provenance.rs:438` | `stored_value_operand` の呼び出し | - | M1 の呼び出し |

## 実装の対応

- M2、M2a、M3、M4 の関数は `home_call_entry_transfer.rs` の `impl PrefixLocalFlow`(move の状態の file)に置く。
- 消す物: local_statement の 2 か所の binding を解く `match`、2 か所の `was_moved_out`、2 か所の同じ scope の `if`。call_entry の binding を解く `let ... else`、2 か所の `was_moved_out`。scan.rs の掃き出しの `UseAfterMove` の構築。

## Opus の確認(2026-11-07)

- 規則と確認の一覧の commit `3e7beb4d0b` は実装より前。段階 1〜3 は 1 つの commit になった(報告に書いてある)。
- 足した 4 つの関数は、前の確認と同じ拒否を同じ site で返す(`MoveUnsupported` は `move` の site、`UseAfterMove` は operand の site)。`home_local_call_borrowed_actuals.rs` の `moved` は、後の行(`:169`)でまだ使われている。
- 最後の binary と test binary は `e1ce026a83` の木から build されていて、その後に変わった source は無い。329 入力と 1,459 本の MIR、全 lib の赤の名前を、Opus も log から読み直した。
- 指示書に無い 9 形でも前と後が同じだった。`new Node(move a, a.get())` は前から `argument-source-unavailable` で拒否され、`UseAfterMove` の名前にならない(この series の前からの挙動)。
