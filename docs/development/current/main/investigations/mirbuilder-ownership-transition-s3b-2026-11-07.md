# 所有の状態の遷移と Fault の後片付けの計画(S3b 歩 B と歩 A の規則)

Status: 規則の文書です。authority ではありません。branch `s3b-ownership`(main `57a26283b6` から)。実装は、この文書の文を写します。文に無い分岐が要る時は、先にこの文書へ行を足して理由を書きます。

背景: 地図 `mirbuilder-ownership-owner-map-s3b-2026-11-07.md`、点検 `s3b-plan-review-2026-11-07.md` 3 節、Pro の回答 `s3b-ownership-cfg-consult-2026-11-07.md` 2〜4 節、仕様 `docs/reference/language/ownership.md` §7、ADR-002 の 62 行目と 84 行目。

性格: **受理を変えない置き換え**です。通る形、拒否の名前、生成される MIR は前と同じです。新しい形はつなぎません。

## 範囲

- 歩 B の範囲は、線形の走査 `scan_new_home_flow` の `homes`(Fault で閉じる責任の並び)と、`PrefixLocalFlow` が持つ所有の状態(`Home`、`Shared`、`ReceivedHandle`、`ReceivedNullable`、`Map`、`MovedOut`、`Consumed`)です。
- 歩 A の範囲は、builder の `ordinary_new_admission` の中で、End の列と Fault の添字を組み立てる所です。
- 範囲外: loop の所有(`home_prefix_source_loop*.rs`)、`if` の合流の規則、借用の状態、独立検証と package 側の照合の作り方。

## 歩 B: 所有の状態

- **S3b-R1 責任の ID**: 所有の責任は、binding の名前とは別の ID(Home の origin)を持ち、binding が使えるかどうかと責任がどこにあるかは、別の欄に持つ。
- **S3b-R1a 欄の対応**: binding が使えるかどうかは `StoredLocal` の欄(`MovedOut` と `Consumed` は使えない)、責任がどこにあるかは `LiveHomeV1` の欄(`Stable(origin)`、`Moved{origin,binding}`)に置く。
- **S3b-R2 状態を変える操作**: 所有の状態を変えるのは、1 つの型(責任の並び `HomesV1` と、それを `StoredLocal` の所有の欄と一緒に更新する `PrefixLocalFlow` の遷移操作)だけである。
- **S3b-R2a 取得**: 取得は Normal の辺でだけ責任を生み、呼び出し自体が Fault した時は責任を生まない。
- **S3b-R2b local の move**: local の move は責任をそのまま残し、元の名前を使えなくし、cleanup の順位を元のままにする。
- **S3b-R2c call-entry の take**: `take` は入場の時点で caller から外れ、その後は Normal でも Fault でも caller は閉じない。
- **S3b-R2d 実引数の移転**: `new` の実引数の移転は Normal の辺でだけ caller から外れ、Fault の辺は移転前の列を保つ。
- **S3b-R2e End**: 試した責任は、Normal でも Fault でも、もう一度は閉じない。
- **S3b-R2f 返却と格納**: field への格納と返却は、移転先が決まった時点で caller から外れる。
- **S3b-R2g 使えなくなる種類**: 使えなくなる種類は 2 つで、`MovedOut`(責任が caller に残るか、すでに外れた名前)と `Consumed`(移転先が決まって外れた名前)であり、後の読みで `UseAfterMove` になるかどうかは今のとおり `MovedOut` だけが決める。
- **S3b-R3 残る責任**: 各辺(Normal、Fault)で残る責任の列は `HomesV1` が返し、書く所は `homes` と `StoredLocal` の所有の部分を直接触らない。
- **S3b-R3a 順序**: Fault の辺の列は、取得の新しい順(今の `prior_homes` の逆順)で、Normal の辺の列は同じ並びから移転した物を除いた列である。
- **S3b-R8 診断**: 拒否の名前(`UseAfterMove`、`MoveUnsupported`、`HomeFlowBranchDivergent` など)は変えず、`MovedOut` と `Consumed` の違いが診断に出る点を守る。

## 歩 A: Fault の後片付けの計画

- **S3b-R4 計画**: Fault の後片付けの計画は 1 つの型 `FaultCleanupPlanV1` にし、持つ物は Normal の End の列、Fault の列、各 Normal の後に残る Fault の添字、取得の添字、置き換えられる値である。
- **S3b-R4a 書く部品**: 書く部品は計画を受け取るだけで、何を閉じるかを決めない。
- **S3b-R4b 入力**: 計画の入力は、歩 B の型が返す残る責任の列と、評価で取得した一時値の列である。
- **S3b-R5 順序**: cleanup の順序は、責任を登録した場所と元の順位で決まり、取得した時刻の逆順(一律の LIFO)にしない。
- **S3b-R5a 登録先**: 登録先は 3 つに区別する: 通常の評価の一時値、元の local の Home、引数の後片付けが終わるまで守られる呼び出しの結果。
- **S3b-R6 呼び出しの結果**: 呼び出し自体が Fault した時は結果はまだ無く、Call が Normal でその後の後片付けが Fault した時は結果の取り消しが要る。
- **S3b-R6a 添字**: 各 Normal の後に残る Fault の添字は「試した Normal 操作を除く全ての責任」(`fault_indices_after_normal_step_v2`)の規則で決め、手で組み立てない。
- **S3b-R7 検証の独立**: 独立検証と package 側の照合は、計画の型を読まず、emit された CFG と source から期待を作る形のままにする。

## 小さい共通化

- **S3b-R9 比較演算子の表**: 比較演算子から公開の `CompareOp` への写しは `ResolvedBinaryOperatorV1::compare_op` の 1 つの表で、比較でない演算子は `None` を返す。
- **S3b-R10 スカラ結果の表**: 結果の種類から公開の呼び出し結果への写しは `LocalCallResultClassV1::scalar_invoke_result` の 1 つの表で、I64 と Bool 以外は `None` を返す。

## 実装の対応

- 歩 B の型は `HomesV1`(`home_ownership_transition.rs`)で、中身の Vec は private、変更する操作は `local_flow` の file だけから見える。
- 歩 B の遷移操作は `PrefixLocalFlow` の `acquire_*`(取得)、`release_to_callee`(実引数と take)、`move_alias`(local の move)、`transfer_out`(返却と格納)、`rebind_text`(置き換え)で、`homes` と `StoredLocal` を同じ操作で更新する。
- `HomesV1` の外から `push`、`retain`、`remove`、代入で `homes` を直接書き換える所は 0 にする(`commit` は遷移操作が計算した並びを採用する操作)。
- 歩 A の型は `FaultCleanupPlanV1`(`selected/fault_cleanup_plan.rs`)で、builder の `ordinary_new_admission` に置く。
- builder 側の一時値と Home の列を作る関数は、package 側の同名の関数(`root_cleanup_order_paths.rs`)と別の物なので、計画の method `owed_operations` に移して名前を変えた。

## Opus の確認(2026-11-07)

- 規則の commit `4c8e1e2704` は実装より前。実装の後の `421b82d74c` は型の名前を合わせただけで、規則の中身は弱めていない。ただし「1 つの型」は、`HomesV1` と `PrefixLocalFlow` の操作の組になった。
- 最後の binary と test binary は `421b82d74c` の木から build されていて、その後に変わった source は無い。途中の `886ba2ca89` も単独で `cargo check --lib` が通る。
- 生成される MIR の比較(329 入力)を Opus も読み直し、全部が同じだった。指示書に無い 32 形(move の連鎖、`new` の実引数の中の呼び出し、`take` の 2 つの実引数、文字列の一時値と print、分岐の中の move、使用後 move)でも、前と後で MIR、出力、拒否の名前が全部同じだった。
- 独立検証と package 側の照合は計画の型を読まない(`FaultCleanupPlanV1` は `crate::mir::builder` の中だけで見える)。S3b-R7 を満たす。
- 全 lib の新しい赤 `published_array_element_writes_compile_object_without_void_result_leak` は、手で組んだ MIR を LLVM の入口に通す test。変えたコードを通らず、入口は環境変数を読むので、並列の競合と判断した。
- 残り: S3b-R1 の責任の ID は `LiveHomeV1` の origin で持ち、新しい型にしていない。S3b-R5a の 3 つの登録先は、型でなく計画の constructor で分けている。`HomesV1::entry` と `commit` は crate の中から使えるので、書き換えが 0 なのは `push`、`retain`、`remove` の形についてである。関数ごとの切り替えでは、この 2 つの入口も解析の型の中に入れる。
