# S3b の進め方の点検(ChatGPT Pro の案をコードに当てる)

Status: 読解の記録です。authority ではありません。branch `s3b-review`(main `356082526a` から)。source は変えていません。build も test も流していません。行番号は `356082526a` のものです。

対象: `s3b-ownership-cfg-consult-2026-11-07.md` の Pro の回答と Opus の確認。比べた地図: `mirbuilder-ownership-owner-map-s3b-2026-11-07.md`(以下「地図」)。

## 0. 要約(ユーザー向け)

- **Pro の目標の流れ(意図の CFG → dataflow → End の計画 → 今ある部品で具体化 → seal → PHI → 検証)は、コードと矛盾しません。変える必要はありません。** 前提の訂正 4 点も、コードで確かめて全部合っていました。
- ただし **Pro の「最初の series」は、1 つの series に入れるには大きすぎます。** 理由は 4 つで、どれも Pro が見ていない事です。
  1. 旧走査の出力は、`scan_new_home_flow` が返す 5 つの物と、17 個の closure が ledger に積む副作用です。読む file は、`TerminalRelationV1` が 63、`LocalCallObservationV1` が 44 あります。新しい解析は、選んだ関数について、これを全部出さないといけません。
  2. 担当を決める所が 1 か所ではありません。関数ごとに走査の入口が 4 つ(`:338`、`:457`、`:535`、`:714/:726`)と、走査を通らない「plain seed」の道があり、道を選ぶ条件は約 20 個の `has_*` の OR です(`ordinary_new_coseal_issue.rs:372-455`)。
  3. 呼び出しの lowering は、`CanonicalCfgSessionV1` を一度も使っていません。`finish` は関数の全 block の seal を要求するので(`session.rs:371-`、`draft_seal_close.rs:51`)、所有の部分だけを session に乗せる事はできません。`emit_root_invoke` は今の lowering が使う操作の大半を拒みます。
  4. 関数単位で切り替えると、範囲外の関数が旧い観測を使い続けるので、**同じ series で消せる旧い判定がほとんど無い**(地図の D1〜D6 と 25 か所は、1 つも消せません)。
- **最初の対応範囲に入る関数は、読んで数えると少なめです。** 実アプリ(apps 100 本)の関数 160 個のうち 55 個(34%)、固定 suite のアプリ 8 本の関数 58 個のうち 25 個(43%)。`main` は全部 loop を持つので入りません。入らない理由の最大は `if`(89 関数)と `loop`(63 関数)です。
- **勧め:** Pro の流れは採用します。順番だけ変えます。Pro の案の前に、**小さい 1 歩を 2 つ**入れます(3 節)。
  - 歩 A: Fault の計画を 1 つの型で作る(地図の候補 2)。
  - 歩 B: 所有の状態の遷移を 1 つの型にする(地図の候補 1)。

  どちらも今の線形走査を残したまま、旧い判定をその series で消せます。Pro の「新解析」の領域と計画の型になるので、その後の Pro の案(関数単位の切り替え)は、型を呼ぶだけで済み、小さくなります。Pro の案をそのまま最初にやる事も、できなくはありません。その時は、1 節の 6 つの不足を先に指示書に入れてください。

## 1. Pro の案が、コードのどこで合わないか

凡例: 「使える」はそのまま使える。「変える」は変えないと使えない。

### 1.1 段ごとの対応

| Pro の段 | 使えるコード | 変えないといけないコード |
|---|---|---|
| 1. 意図を保持した CFG | 式の木と到達性は既にある: `ExpressionSourcePlanV1`(`resolved_value_profile/expression_plan.rs:24-`)、`ExpressionChildReachV1::{Always,When}`(`expression_children.rs:95` 付近)。関数の出口の集合は `VerifiedFunctionCompletionV1`(`explicit_sites`、`implicit_body_end`) | **関数単位の操作列と Fault の辺を持つ型は無い。** 今の「流れ」は文の走査(`scan_statement_flow`、`home_new_prefix_scan.rs:10`)の中に埋まっている。直線の関数でも Fault の辺を作る必要がある |
| 2. dataflow | 状態の部品は `StoredLocal`(25 variant)、`LiveHomeV1`、`MovedOut`/`Consumed`。Pro の補正 1 は正しい: `MovedOut` は `homes` の組合せで意味が決まる(`home_call_entry_transfer.rs:58`、`:177`) | 状態が `homes` と `locals` の 2 つに分かれ、書く所が 25(地図 1.2)。領域の型が無い。合流は `homes` の共通部分と `join_branch_at`(`home_new_prefix_branch.rs:688-697`)で、直線の関数には要らない |
| 3. End の計画 | 規則は 1 つになっている部分がある: `fault_indices_after_normal_step_v2`(`root_home_cleanup_order.rs:189`)、`displaced_replacement_fault_sequences_v2`(`:207`) | 計画を作る所が builder に 30 か所(地図 2.2)。添字を手で作る所が 4 か所。検証側が同じ列を再導出する所が 21 か所(10 file)。計画の型が無い |
| 4. 今ある部品で具体化 | 形を知らない書く部品 12(`root_graph.rs:158,175,204,222,278,306,325`、`selected.rs:640,655,674`、`expression_value.rs:740,754`)。書き込み口の trait の実装が 3 つ | 1.3 節。`emit_root_graph_with_arguments_v1`、`prepare_operation_cleanup_v1`、`finish_body_cleanup`、`finish_new_arguments_v1` は決める所と書く所が混ざる |
| 5. seal | `seal_block`(`session.rs:337`)は predecessor を MIR terminator から導出する | **呼び出しと分岐の所有 lowering は session を使っていない**(`ordinary_new_admission/**` に 0 件)。1.3 節 |
| 6. PHI | `PhiTxn`(`phi_lifecycle.rs:133`) | 同上。直線の最初の範囲には PHI が出ない(`if`、`&&` を入れないため)。PHI の接続は 2 段目から |
| 7. 検証 | 独立 verifier `ownership_ssa/`、source 対応検査(`ordinary_new_local_commit/**`) | 検証側が期待する列を再導出する 21 か所が、計画の型の出力と一致するよう変わる。独立性を保つには、検証側が計画を読まず source と CFG から作る形を保つ |

### 1.2 `scan_new_home_flow` の出力を読む所(新しい解析が全部出す物)

`scan_new_home_flow`(`home_new_prefix.rs:377`)は 5 つの物を返し、さらに 17 個の closure を通して issuer 側に副作用を積みます。

| 出力 | 型 | 読む所(outside `resolved_semantics/home_*`) |
|---|---|---|
| prefix | `BTreeMap<OwnedExprSiteV1, Result<CallerNewHomePrefixV1, HomePrefixUnavailableV1>>` | 9 file(`prior_homes`、`normal_homes`、`required_unwind`、`outward_fault`、`covered_statements`) |
| RootHomeFlow | `named_arrays`、`exits`、`maps`、`local_calls`、`expression_plans`。`completion.attach_root_home_flow(homes)` で completion に付く(`function_control_new_homes.rs:142-167`) | `RootHomeFlow` 4 file、`RootHomeExitV1` 10 file、`LocalCallObservationV1` 44 file、`MapHomeObservation` 6 file、`NamedArrayHomeObservationV1` 14 file、`HomeExpressionPlanV1` 1 file |
| terminal relation | `BTreeMap<SourceStmtSiteV1, TerminalRelationV1>` | **63 file** |
| argument observation | `BTreeMap<OwnedExprSiteV1, SelectedNewArgumentObservationV1>` | 5 file |
| result prefix | `BTreeMap<OwnedExprSiteV1, Result<ResultNewHomePrefixV1, _>>` | 6 file |

closure の副作用(`ordinary_new_coseal_issue.rs:500-640`):

- `local_field_read` は `stage_local_field_read_batch` で `local_staged_reads` に積む。`field_is_integer` は `staged_reads` に積む。`argument_field_is_integer` は `argument_staged_reads` に積む。走査の後で `merge_staged_*` が ledger に入れる。
- `local_static_call`(`&mut`)、`borrowed_actuals` が `borrowed_formal_actuals`(`&mut`)を更新する。`loop_prestate` は loop の loan を `selected_loop_products` に発行する。
- したがって走査は、**「source から答えを出す純粋な関数」ではなく、issuer の ledger への発行を伴う走査**です。Pro の「解析結果を一度保持して、問い合わせを投影する」は、この副作用の発行元を決めないと成り立ちません。新しい解析も同じ closure を呼ぶ形(問い合わせは site を鍵にした closure なので再利用できる)にするのが、最小です。

### 1.3 呼び出しの lowering を `CanonicalCfgSessionV1` に乗せる時に足りない物

`emit_root_invoke`(`canonical_cfg/session/invoke.rs:12-104`)が受ける操作:

- 後片付け(cleanup): `String::End`、`HomeRelease`、`HomeReleaseIfLive`、`OwnedFieldResidenceRelease`、`OwnedObjectFieldRelease`、`Map::End`(`:26-34`)。
- 入口(ingress): `dst` が無い `Call`、`Map::CheckedGetI64`、`String::Add`、`String::Print`(`:35-43`)。
- それ以外は `unsupported root operation`(`:44-48`)。
- 着地 block の条件: Jump 終端で predecessor が無い。Normal 着地は cleanup なら空、ingress なら `InvokeNormalResult` が 1 つだけ(`:61-85`)。Fault 着地は空。

今の呼び出しの lowering が使う `InvokeOperation`(`builder/ordinary_new_admission/**` の出現数の多い順): `DynamicCarrierEnd` 7、`String::End` 8、`Call` 5、`DynamicArrayGet` 3、`Map`(`End`、`CheckedGetI64`、`FieldKeys`、`EndOutcome`)、`DynamicInvocation` 2、`DynamicCarrierSubI64` 2、`String::{Substring,Print,Materialize,Length}`、`Shared`、`OwnedArray::{Claim,Append,Get}`、`ObjectFieldBorrow`、`NewBox`/`New`、`FieldGet`/`FieldSet`、`ReclaimUnpublished`、`IntrinsicArrayNew`。

足りない物:

1. **`DynamicCarrierEnd`**(`pending_operations` が 7 か所で積む)、`OwnedArray` 系、`Shared`、`ReclaimUnpublished`、`String::{Substring,Materialize,Length}`、`Birth`/`New` 系が `emit_root_invoke` に無い。
2. `emit_root_invoke` は root の Home graph 専用と書かれている(`:9-11`)。`emit_i64_invoke`(`:128-`)は結果が I64 の `Call` だけ。
3. **`finish` は関数の全 block が seal 済みである事を要求する**(`session.rs:371-390`、`UnsealedBlockAtFinish`)。呼び出し元は `canonical_ssa/session/draft_seal_close.rs:51` の関数 draft の close。つまり session に乗せるのは「所有の部分」ではなく「関数全体」。今の所有 lowering は `builder.emit_instruction` と `function.add_block` で関数の全部を書いているので、print、local、scalar 演算、`return` の書き方を全部 session 経由にしないと `finish` が通らない。Pro の「選んだ関数について」は、この意味で関数全体の lowering の切り替えになる。
4. 書き込み口の trait `RootGraphWriterPortV1` に `CanonicalRootGraphWriterV1`(`static_loop_root_graph_writer.rs:418`)という実装が既にあるのは、使える。ただし使われているのは `static_loop_entry_session.rs:79` 1 か所で、その範囲の操作に限られる。

### 1.4 旧走査の前の probe を、新しい解析の関数で止める時に困る所

関数ごとに、`ordinary_new_coseal_issue.rs` の 1 つの closure の中で次が起きます。

| 行 | 何をするか | 困る所 |
|---|---|---|
| `:338`(probe の定義)、`:350-362`(readiness) | `probe(...)`(`probe_source_home_prefixes_v1` → `issue_new_home_prefixes_probing_fields_v1`、`ordinary_new_coseal_issue_source_home_probe.rs:75`)で旧走査を 1 回回し、`readiness` を作る | `readiness` は `child_new_ready` を通って `seed_completion` と `verified_walk` を決める(`:372-455`)。**担当を決める前に旧走査が走っている。** 新側の関数では、readiness を新解析から作る必要がある |
| `:401-423` | `seed_completion`(= 走査を通らない plain の completion)。約 20 個の `!has_*` の AND | **走査を通らない道がある。** 所有の対象が無い関数は、今は解析を通らない。新側で全関数を解析すると、挙動と費用が変わる |
| `:440-455` | `verified_walk`。約 20 個の `has_*` の OR | 道の選択が形のフラグ(`has_print_expression`、`has_condition_expression`、`has_text_return`、`has_borrowed_walk`、`has_dynamic_local_initializer` …)の集まり。担当を「操作と契約の集合」で決める(Pro の案)には、このフラグを集合の判定に置き換える必要がある |
| `:457-468` | `!verified_walk && has_borrowed_source_calls` の時だけ probe を再実行し、`borrowed_formal_actuals`(callee 側の lowering が読む)を埋める | 走査の副作用が **他の関数** の lowering に届く。新解析の関数もこれを埋める必要がある |
| `:535` | 本命の走査(`verify_function_completion_with_new_homes_and_argument_observations_v1`) | 1.2 節の全出力 |
| `:714`、`:726` | completion が失敗した app_main と、`verified_walk` でない関数の `issue_new_home_prefixes_with_arguments_v1`(prefix だけを出す走査) | 新側の関数でも、prefix だけを出す入口が要る |

走査の入口は 4 つ(`:338` 経由の probe、`:457` の probe、`:535`、`:714/:726`)。Pro が挙げた 4 行のうち、`:338` と `:457` は「走査の前」ではなく **走査を呼ぶ側**で、結果が道の選択に使われます。

## 2. 最初の対応範囲に入る関数の数

### 2.1 数え方

Pro の最初の範囲: 契約が確定した呼び出しと演算、local、局所の `move`、今の `take` の入場、今の契約で生まれる一時値、結果は Unit/I64/Bool、`return` と暗黙の出口。`if`、loop、値の `&&` `||`、`edit` は入れない。

source を読んで(実行せずに)、スクリプトで数えました(scratchpad の `count.py`。文書には入れていません)。

- 関数の取り出し: コメントと文字列を消し、`名前(引数) [: 型] {` の形を括弧の対応で切る(box の中の method、`main`、`birth`)。`if`/`loop` などの予約語は除く。1 行に書かれた box も拾う。
- 範囲外の印(関数の本体に 1 つでもあれば外す): `if`、`loop`、`while`、`for`、`match`、`&&`、`||`、`edit`、`try`、`catch`、`break`、`continue`(= 制御)。`%{`、`share`、`null`、`from`、`?`(Map literal、Shared、null、nullable)。
- 結果の種類: 注釈が `i64`/`bool`/`Unit` なら範囲内。注釈が無い関数は、全部の `return` の式に `new`、文字列 literal、`null`、`[`、`%{` が無ければ範囲内(「結果が Unit/I64/Bool らしい」)。それ以外の注釈(box 名など)は外す。
- 重複: 内容が同じ file は 1 つ、同じ名前・引数・本体の関数は 1 つと数える(複製のテスト入力が多い)。

これは **字面の推定**で、型検査も実際の受理も見ていません。`share` を外したのは Pro の範囲に書かれていないからで、入れる場合は数が増えます。注釈の無い関数の結果の種類は推定です。

### 2.2 数

| 集合 | file | 重複を除いた関数 | 範囲内 | 率 | 範囲内のうち `new`/`move`/呼び出しを含む物 |
|---|---:|---:|---:|---:|---:|
| 229 入力の一覧(`list.txt`。実際は 213 行) | 213(重複除き 203) | 250 | 193 | 77% | 152 |
| apps 100 本(`bool-claim/list.tsv`) | 100 | 160 | 55 | 34% | 35 |
| 固定 suite のアプリ 8 本(`apps/{allocator-stress,binary-trees,boxtorrent-mini,json-stream-aggregator,mimalloc-lite,ownership-child-typed-min,ownership-shared-min,typed-object-untyped-field-min}/main.hako`) | 8 | 58 | 25 | 43% | 14 |
| 固定 16 入力(`fixed-16`、`dyn_map.hako` を除く) | 16 | 75 | 41 | 55% | - |
| 全部(重複除き) | 295 | 351 | **222** | 63% | 173 |

- 一覧が 213 行で、指示書の 229 にならない: 一覧の file をそのまま数えた。229 の内訳は未確認。
- `fixed-16` は apps の複製を含むので、「全部」の行で重複を除いた。
- 固定 suite は `real-apps-exe-boundary.txt` の 8 つの script のうち、`apps/<name>/main.hako` を持つ 8 アプリ。`canonical_app_exe_cases.sh` の中の埋め込みの heredoc(`$TMP_ROOT.take.hako` など)は数えていません。
- 一覧(77%)は小さい検査用の入力が多く、本物のアプリ(34〜43%)より高く出る。

### 2.3 入らない関数の理由(重複を除いた関数 351 のうち)

| 印 | 含む関数 |
|---|---:|
| `if` | 89 |
| `loop` | 63 |
| `break` | 23 |
| `&&` | 5 |
| `\|\|` | 3 |
| `continue` | 5 |

範囲を広げた時の数(範囲内の数。推定):

| 範囲 | apps 100 | 固定 suite | 全部 |
|---|---:|---:|---:|
| Pro の最初の範囲 | 55 | 25 | 222 |
| + `if`(合流は両枝の Home が同じ時だけ) | 74 | 30 | 247 |
| + `if` と `&&` `\|\|` | 77 | 33 | 250 |
| + loop 全部 | 136 | 42 | 310 |

- `main` は固定 suite の 8 アプリで全部 loop を持つので、最初の範囲では 0 個。
- 「少なすぎる」かの判断: 最初の範囲は 222/351 関数(63%)でも、**実アプリ(apps、suite)では 34〜43%** で、アプリの主要な関数(`main`、走査する関数)はほとんど入りません。`if` を入れても 74(46%)で、大きく動くのは loop(136、85%)を入れた時です。決まり(関数の担当は 1 つ、形で選ばない)を守ったまま範囲を広げるなら、**`if` の合流(両枝が同じ状態なら受理、違えば既存と同じく拒否)を最初の範囲に入れる**のが最小です(+19 関数、apps で 12 ポイント)。ただし 1.2 の出力に分岐の `exits`/`covered_statements` が加わり、大きさは増えます。

## 3. もっと小さい、またはもっと良い最初の 1 歩

Pro の案(以下「P」)に対し、2 つの案を出します。作るためだけの案ではなく、1.1〜1.4 の不足を直接埋める物です。

### 案 A: Fault の後片付けの計画を 1 つの型で作る(地図の候補 2)

- 内容: Normal の End 列、Fault の列、各 Normal 後に残る Fault の添字、acquisition の添字、displaced を、1 つの計画の型として 1 か所で作る。書く部品(12 関数)は計画を受けるだけにする。`pending_operations`(builder)と `fault_indices_after_normal_step_v2`、`displaced_replacement_fault_sequences_v2` をその型の method にする。
- 旧い判定を消す: 添字を手で作る 4 か所(`root_graph.rs:425-427,504-509`、`expression_value.rs:368,457,747`、`arguments.rs:600-601`)、混ざる組立て 4 つ(`prepare_operation_cleanup_v1`、`emit_root_graph_with_arguments_v1`、`finish_body_cleanup`、`finish_new_arguments_v1`)の列の作り方、`cleanup_chain` を使う 15 か所の列の組立て。

### 案 B: 所有の状態の遷移を 1 つの型にする(地図の候補 1)

- 内容: `homes` と `locals` の所有に関わる部分を 1 つの型にまとめ、書く所が `install`、`move_out(edge)`、`consume`、`transfer` だけを呼ぶ形にする。Normal 辺と Fault 辺で何が残るか(`prior_homes`、`normal_homes`)をその型が返す。
- 旧い判定を消す: D1(prefix の構築 3 つ)、D2(2 行 × 3)、D4(同名の 2 実装)、`consume_home`/`consume_argument_home` の入口 2 つ、R5/R6 の直接参照。

### 比べた表

| 観点 | P(Pro の案) | A(Fault の計画) | B(状態の遷移) |
|---|---|---|---|
| 最初の series の大きさ(見込み。測っていない) | 大。新しい CFG と解析の型、関数単位の担当の決定(入口 4 つ、約 20 個のフラグ)、出力 5 つの adapter(63 file が読む型)、session の操作の拡張(1.3 の 1〜3)、関数全体の session 化。新規の file が複数、数千行の見込み | 中。触る file は builder の `selected/*` と `root_graph.rs` を中心に約 10 file。変更は数百行の見込み。package 側の検証(21 か所)は列の作り方が変わらないので触らない | 中。触る file は `resolved_semantics/home_*` の約 9 file。変更は数百行の見込み。`home_prefix_local_flow.rs`(1,220 行)は先に分割が要る |
| 今通っている形を落とす危険 | 中〜大。範囲外を旧側に残す方式なので、範囲内の関数だけの退行。ただし 1.4 の道(plain seed、readiness、borrowed actuals の副作用)を取りこぼすと、範囲外の関数にも効く | 小。受理は変わらない(列は同じ、作る所が 1 つになるだけ)。検証側の期待と 1 対 1 に一致させる必要がある | 小〜中。`MovedOut` と `Consumed` の違いが診断(`UseAfterMove`)に出ているので、まとめ方を誤ると理由が変わる |
| 同じ series で消せる旧い判定(地図) | **ほぼ 0。** 範囲外の関数が旧い観測を使い続けるので、D1〜D6 と 25 か所の観測は残る。消せるのは新側専用の物だけ | D8 の 4 か所と、30 か所のうち約 19 か所(混ざる 4 + `cleanup_chain` の 15)。D1〜D6 は残る | D1、D2、D4 と、25 か所のうち約 19 か所(`consume_*` 経路)。30 か所は残る |
| 後の段階(値の `&&`、`if` の合流、S4 の loop、所有の結果、`edit`)が同じ土台に乗れるか | 乗る設計だが、最初に全部を作るので、乗れるかは実際に動くまで分からない | **乗る。** 値の `&&` に必要な「道ごとの列」の型は A の拡張で作れる | **乗る。** dataflow の領域と遷移関数になり、合流規則(`if`)と loop の不動点が同じ型を使う |
| 決まりを守るか(新旧を同じ関数で混ぜない、fallback で両方を試さない、形を当てない) | 守る設計。ただし道の選択(約 20 個のフラグ)を残したまま足すと、フラグを増やす形になりやすい | 守る(置き換え。旧い組立てはその場で消える) | 守る |

### 結論

- Pro の目標の流れは採用します。
- 最初の series は **A、次に B**(または B、次に A)を勧めます。どちらも、旧い判定を同じ series で消せて、受理を変えず、Pro の案の部品(領域、計画)になります。A は builder 側だけ、B は resolved_semantics 側だけで、触る file が重なりません(ただし `prior_homes` の順序を A が受け取るので、B の後に A の方が自然かもしれません。入力は今の `prior_homes` と `unwind` なので、独立にも進められます)。
- その後の P(関数単位の担当の決定と新解析)は、A と B の型を呼ぶだけになり、1.2〜1.4 の不足(出力 5 つ、入口 4 つ、session の操作と全関数の seal)に集中できます。
- 「Pro の案のままでよい」は、流れと設計の方向について言えます。最初の series の範囲と大きさについては、上の理由で変えるのが良い、が点検の結果です。

## 4. Pro が見ていない事(根拠つき)

1. **走査を通らない道(plain seed)がある。** `seed_completion`(`ordinary_new_coseal_issue.rs:401-423`)。所有の対象が無い関数は解析を通らない。担当の決定は「新/旧」の 2 択ではなく「新/旧/解析なし」の 3 択になる。
2. **道の選択が約 20 個のフラグの AND/OR。** `:372-455`。Pro の「操作と契約の集合で決める」は、このフラグの置き換えを意味する。置き換えずに足すと、形で選ぶフラグが増える。
3. **走査は ledger に発行する。** `staged_reads`、`local_staged_reads`、`argument_staged_reads`、`borrowed_formal_actuals`、`selected_loop_products`(1.2 節)。結果を「一度保持して投影」するだけでは足りない。
4. **走査の副作用が他の関数に届く。** `borrowed_formal_actuals` は callee の lowering が読む(`:457-468`)。
5. **出力を読む file の数。** `TerminalRelationV1` 63、`LocalCallObservationV1` 44、`NamedArrayHomeObservationV1` 14、`RootHomeExitV1` 10、`CallerNewHomePrefixV1` 9。adapter の大きさの見積りに要る。
6. **session は関数全体の seal を要求する。** `finish`(`session.rs:371`)が `UnsealedBlockAtFinish`(`:379`)を返す。呼び出しと分岐の所有 lowering は session を使っていない(0 件)ので、移行は「所有の部分」ではなく「関数全体の lowering の切り替え」になる。Pro は「選んだ関数の topology が完成するまで block seal を遅らせる」と書いたが、遅らせる対象が関数全体の block になる事までは書いていない。
7. **`emit_root_invoke` が拒む操作。** `DynamicCarrierEnd`(`pending_operations` が 7 か所で積む)、`OwnedArray`、`Shared`、`ReclaimUnpublished`、`String::{Substring,Materialize,Length}` など(1.3 節)。Pro は「`emit_root_invoke` には operation の制限がある」と書いた(その通り)が、制限の大きさ(今の lowering の操作の大半)は数えていない。
8. **関数単位の切り替えでは、旧い観測が残る。** 範囲外の関数が同じ observer を使う。Pro の「選択した旧 edge がゼロ」は、新側専用の物と「選んだ edge」に限る。地図の D1〜D6(重複 6 組)と書く所 25 か所は、全関数が移るまで消えない。
9. **最初の範囲に入る関数が少ない。** apps で 34%、suite で 43%、`main` は全部入らない(2 節)。最初の series の成果が実アプリで見えにくい。
10. **file の大きさ。** `home_prefix_local_flow.rs` 1,220 行、`home_expression_plan.rs` 1,246 行はすでに 800 行を超える(地図 8 節)。B は先に分割が要る。
11. **名前の衝突。** builder の `pending_operations`(列を作る、`expression_value/cleanup.rs:4`)と package の `pending_operations`(CFG を歩く、`root_cleanup_order_paths.rs:220`)。Opus の確認が指摘した 11 file は後者。物差しを数える時は区別する。
12. **仕様の合流規則は「合流の後に使う場合」だけを拒否と書いている。** `ownership.md` §7(`:1152-1175`)。片枝 move で、合流の後に使わず、責任が片側に残る場合の規則は、仕様に見つからなかった(未確認)。Pro の表(drop flag を使う)はこの場合の提案で、仕様の決定ではない。Pro の補正 3 は、ADR-002:62 の drop flag が loop の live/empty の合流に向けた記述である事まで書いていない。

## 5. 確かめた Pro の主張(合っていた物)

- 補正 1(`MovedOut` の 2 つの意味): `home_call_entry_transfer.rs:58`(付け替え)と `:177`(take で除く)。
- 補正 2(値を返す短絡 writer): `logical_shortcircuit.rs` の `build_logical_shortcircuit_after_lhs_recorded_v1` が Bool の PHI を作る。`insert_phi` と `update_cfg()`(`:274`)を直接呼ぶ。地図の 4 節と同じ。
- 補正 3(`HomeFlowBranchDivergent` は現在の実装範囲): `home_new_prefix_branch.rs:688-723`。
- 再利用の部品: `emit_cleanup_suffixes_with_writer`(`root_graph.rs:158`)、`emit_cleanup_actions_to_fault_with_writer`(`:222`)、`cleanup_step_with_writer`(`:325`)、`CanonicalRootGraphWriterV1`(`static_loop_root_graph_writer.rs:9,418`)、`fault_indices_after_normal_step_v2`。
- 撤去する旧判断: `validate_pending_call_uses_v1`(`home_call_entry_transfer.rs:66-`)、`commit_statement_takes_v1`(`:137-`)、`pending_operations`(builder)。
- `pending_operations` の数: `selected/expression_value.rs` 6、`expression_value/cleanup.rs` 3、`print_expression.rs` 2 の計 11 か所、3 file。
- `prepare_operation_cleanup_v1` の「Call 自体の Fault では結果は無く、Call Normal 後の cleanup Fault では結果の cancellation が要る」(`root_graph.rs:504-509`)。

## 6. 未確認と読んでいない所

- 範囲内の関数が実際に今の走査を通るか(受理されているか)は見ていません。数は字面の推定です。
- 大きさの見込み(行数)は、触る file の行数と arms の数からの推定で、測っていません。
- 229 入力の内訳、`canonical_app_exe_cases.sh` の埋め込み heredoc は数えていません。
- 独立 verifier `ownership_ssa/` の対象範囲、loop の所有(`home_prefix_source_loop*.rs`)は読んでいません。
- build、test、実行はしていません。
