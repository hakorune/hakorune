# S3b 2 段目の作り方を 2 つ(と 1 つ)比べる(読解の記録)

Status: 読解の記録です。authority ではありません。branch `s3b-analysis-options`(main `b75a4bb169` から)。source は変えていません。build も test も流していません(数えるのに使った script は読み取り専用で、文書には入れていません)。行番号は特に断らない限り `b75a4bb169` のものです。決めるのはユーザーです。

比べる物:

- **案 1**: 要求の集合と文の種類が、新しい解析の対応範囲に全部収まる関数だけを、`ScanRouteV1` の新しい variant で新しい解析(意図を保持した CFG の上の dataflow)に通す。残りは今の走査。
- **案 2**: 全部の関数で、文を順に辿る走査の進み方(`scan_statement_flow`、`walk_branch`)を、CFG を辿る形に置き換える。腕は CFG の node の上で呼ぶ部品として今のまま使う。
- **案 3**(私が足す 1 つ。7 節): 今の線形走査を残したまま、腕を契約ごとの遷移へ 1 つずつ置き換える。

## 0. 要約(ユーザー向け)

- **案 1 も案 2 も、今の形のままでは勧めません。** 両方とも悪い、というのが読んだ結果です。理由は別々です。
- **案 1**: 新しい解析に通る関数が少なく、同じ series で消せる旧い判定がほぼ 0 で、旧い走査と新しい解析が長く並びます。
  - 通る関数の数は、字面で数えて、実アプリ 100 本の関数 161 個のうち 29〜56 個(18〜35%)、固定 suite のアプリ 8 本の関数 58 個のうち 4〜25 個(7〜43%)です。幅は、`me` の field の読み書きと配列・Map を範囲に入れるかで変わります。Pro の範囲の文はこの点を決めていません。
  - 旧い走査が出す物(出力 5 つ、ledger への発行、readiness、`borrowed_formal_actuals`)を、新しい解析が全部作り直す必要があります。
- **案 2**: 腕を変えずに呼べるのは「node = 文」の時だけで、その粒度の CFG は、今の再帰の書き換えにしかなりません。
  - Fault の辺と評価順は、今は腕の中に埋まっています。CFG に出すには node を式の中の操作まで細かくする必要があり、その時は腕を変えることになります(案 2 の前提が崩れます)。
  - loop の走査は `scan_statement_flow` を再帰で呼んでいるので、案 2 に入れると S4 の範囲に踏み込み、外すと旧い走査が残ります。
  - 値の `&&` `||` は、案 2 では進みません。
- **案 3**: 歩 B と歩 A で効いた進め方を続ける案です。1 series ごとに旧い判定を消せ、受理を変えない検査(MIR の同一)がそのまま使えます。ただし CFG も `&&` も直接は作りません。
- **勧め**: 今は案 3 から始め、案 1 か案 2 の判断は、案 3 を 1〜2 series やった後に延ばすことを勧めます。ただしこれは「決められない」ことの言い換えではなく、理由があります(5 節)。**勧めが変わる条件**も 5 節に書きました。
- **未確認が大きい所**(6 節): 範囲内の関数が今の走査を通って受理されているかは見ていません。大きさ(行数)は測っていません。案 3 が腕のどれだけを契約の遷移にできるかは、27 の腕のうち数個しか中を読んでいません。

## 1. 今の走査の進み方

### 1.1 進み方の部品(全部)

| 部品 | file:line | 何をするか |
|---|---|---|
| 入口 | `home_new_prefix.rs:410`(`scan_new_home_flow`)、呼び出し `:605` | 状態(`PrefixLocalFlow`、`HomesV1`、`covered_statements`、`selected_seen`、`unavailable`)を作り、`scan_statement_flow` を 1 回回す。後で未走査の site に `unavailable` の理由を入れる(`:660-680`) |
| 文の列 | `home_new_prefix_scan.rs:9`(`scan_statement_flow`)、`for index in 0..body.statements().len()`(`:117`) | 文を前から順に 1 つずつ処理する。explicit exit で `Ok(true)` を返し、後の文は見ない |
| 使用後 move の掃き出し | `scan.rs:128-141` | 文の部分木のうち、`if` と loop の本体を除く全 local 参照が `MovedOut` なら `UseAfterMove`(地図の R1) |
| 実引数の準備 | `scan.rs:143-155`(`invalidate_dynamic_field_sources`、`stage_statement_actuals`) | 文ごとの前処理。借用の実引数を `borrowed_actuals` closure に積む |
| loop の出口 | `scan.rs:156-170`(`observe_source_loop_exit_v1`) | loop の本体の中で、break / continue の出口を記録する |
| explicit exit | `scan.rs:171-208`(`terminal::observe_terminal_statement`) | `return` など。`terminal_relations`、`exit_homes`、`result_prefixes` を書く |
| `if` | `scan.rs:209-249`(`branch::observe_if_statement`、`home_new_prefix_branch.rs:235`) | 6.2 節 |
| `print` | `scan.rs:250-269` | 腕 |
| Local 以外の腕の連鎖 | `scan.rs:270-580` | 下の「連鎖」 |
| Local の腕の連鎖 | `scan.rs:582`、`home_new_prefix_local_statement.rs:9` | 下の「連鎖」 |
| 文の後の take | `scan.rs:609`、`:320`(`commit_statement_takes_v1`) | 文のあとで `taken` を `MovedOut` にして `homes` から外す |
| 本体の終わり | `scan.rs`(`end_source_loop_body_v1`)と `home_new_prefix.rs:641` の implicit exit | 到達した状態を implicit exit の行にする |

連鎖の中身(地図の「27 の腕」は、この 2 つの連鎖を数えた物):

- Local 以外(`scan.rs`): binding の再代入(`:271`)、map field の install(`:285`)、名前付き配列の append(`:297`)、method call の statement(`:308`)の中の discard call と print(`:324`)、field write(`:341`)、text append(`:371`)、field call(`:388`)、loop(`:404`。4 種の `SelectedLoopHomeLoanV1`)、どれでもない(`:559`、`PrefixNotCovered`)。
- Local(`home_new_prefix_local_statement.rs`、変数ごとのループの中): share fresh / clone / move、`move` の local、map の field get(`:260`)、mixed call(`:304`)、borrowed nullable(`:319`)、received producer(`:361`)、local call の route(`:396`)、lexical i64(`:419`)、selected new(`:437`)、map literal(`:456`)、名前付き配列の初期化(`:485`)、初期化の式(`:496`)、declared read alias(`:510`)、class の観測(`:512`)、field call(`:514`)。

**連鎖は「先に true を返した腕が文を取る」順序で決まります。** 同じ文が 2 つの腕に合いうる場合に、どちらが取るかは並びで決まっていて、並べ替えは受理を変えうる(例: method call の statement は、discard call → print の順に試される。`scan.rs:308-340`)。この順序は、どこにも規則として書かれていません(確かめた限り)。

### 1.2 式の中の評価順

- 文の中の式の評価順は、走査ではなく、`ExpressionSourcePlanV1`(`resolved_value_profile/expression_plan.rs`)と `ExpressionChildReachV1::{Always,When}` に既にある。`home_expression_evaluation.rs`(788 行)と `home_expression_plan.rs`(1,246 行)がこれを読んで、式ごとの観測を作る。
- `new` の実引数と method call の実引数の評価順の使用後 move は `validate_pending_call_uses_v1`(`home_call_entry_transfer.rs:70-143`)が、`simulated` の局所状態で確かめる(地図の R2)。

### 1.3 腕が走査の進み方の何に頼っているか

腕は「次の文」を見ません。`body_stmt(index)` で自分の文を取る所は `scan.rs` の 1 か所だけ(`input.source().body_stmt(body, index)`)で、腕のファイルを grep しても、次の文や文の位置の番号を読む箇所は、テスト以外では見つかりませんでした(`body_stmt`、`.statements()` を grep した範囲)。

腕が頼っているのは次の 5 つです。

1. **入ってくる状態**(`locals`、`homes`)。腕は state を読み、書く。`prior_homes` は腕の中で `homes.to_vec()` として取る(`local_statement.rs:159`、`:211`)か、`HomesV1::fault_order` で作る。
2. **`unavailable`(最初の失敗の理由)**。腕は読み(`unavailable.is_none()` で staging の可否を決める)、`get_or_insert` で書く。`local_statement.rs` だけで 45 回出る。**最初の失敗が理由として残る**ので、診断の名前は走査の順序に依存する(規則 S3b-R7 が守る物)。
3. **文をまたぐ蓄積**: `local_calls`(`LocalCallObservationV1` の列)、`path_calls`(この経路で観測した local call の site)、`covered_statements`、`selected_seen`。`commit_statement_takes_v1` は直前の文の `local_calls` の行を読む(`scan.rs:609`)。
4. **出力の map**(`results`、`exit_homes`、`terminal_relations`、`argument_observations`、`result_prefixes`)。腕が site を鍵に書く。同じ site を 2 回書くと上書きになるので、走査は 1 経路 1 回を前提にしている。
5. **issuer の closure**(14 個を `scan_statement_flow` の引数に数えた。地図は 17 と書いている。差は確かめていない)。

腕ごとの表(読んだ範囲。`unavail`、`homes`、`prior`、`local_calls`、`path_calls`、`covered` は、その file の出現回数。数えたのは語の出現で、意味の強さではない):

| 腕(file) | 行数 | unavail | homes | prior | local_calls | path_calls | covered | 他に頼る物 |
|---|---:|---:|---:|---:|---:|---:|---:|---|
| `home_new_prefix_local_statement.rs` | 604 | 45 | 34 | 4 | 13 | 10 | 4 | `selected`(site → binding)、`result_prefixes` |
| `home_new_prefix_terminal.rs` | 771 | 24 | 30 | 0 | 11 | 7 | 4 | `exit_sites`、`terminal_relations`、`result_sites` |
| `home_new_prefix_branch.rs` | 778 | 24 | 36 | 0 | 9 | 9 | 23 | `selected_seen` 14、分岐の 6 つ組 |
| `home_new_prefix_new_local.rs` | 250 | 14 | 10 | 1 | 0 | 0 | 3 | `selected_seen` 2 |
| `home_new_prefix_named_array.rs` | 190 | 11 | 8 | 2 | 0 | 0 | 6 | |
| `home_map_value_flow.rs` | 500 | 10 | 12 | 3 | 0 | 0 | 0 | |
| `home_new_prefix_field_write.rs` | 464 | 2 | 6 | 0 | 8 | 3 | 0 | |
| `home_new_prefix_initializer_expression.rs` | 341 | 3 | 10 | 0 | 5 | 3 | 0 | |
| `home_map_flow.rs` | 783 | 2 | 17 | 0 | 3 | 0 | 17 | `MapEntryOwnership` |
| `home_local_call_flow.rs` | 765 | 3 | 28 | 27 | 0 | 0 | 0 | `CallEntryHomes` |
| `home_new_prefix_print.rs` | 113 | 0 | 6 | 0 | 4 | 3 | 1 | |
| `home_new_prefix_binding_rebind.rs` / `home_binding_rebind_expression.rs` | 138 / 674 | 0 / 1 | 9 / 17 | 0 / 8 | 0 | 4 / 0 | 0 | |
| `home_new_prefix_field_call.rs` | 605 | 0 | 12 | 0 | 0 | 0 | 5 | |

読み取れること:

- **腕の入出力は、`(locals, homes, unavailable, 蓄積, 出力 map)` の組を、文 1 つの上で更新する関数**の形で揃っている。文の位置や次の文には頼らない。**腕の入力が「node = 文」の CFG で足りる**のはこのため。
- ただし**腕は経路ごとの状態の複製を前提にしている**: `if` は `BranchPath`(`home_new_prefix_branch.rs:22-`)に 6 つ組(`locals`、`homes`、`covered`、`selected_seen`、`calls`、`unavailable`)を持たせて分岐ごとに複製する。出力の map は複製せず共有(`&mut`)したままなので、**同じ site を 2 つの経路が書くと上書きされる**(分岐の中の `return` が複数あれば site が別になるので今は起きない、と読んだが、実行して確かめてはいない)。
- **Fault の辺と評価順は腕の中にある。** `prior_homes`(Fault の辺で閉じる列)、`outward_fault`、`required_unwind`、`normal_homes` は腕が作って、`CallerNewHomePrefixV1` に入れる(`home_local_call_flow.rs:86-224`)。CFG の node が文の粒度だと、辺は「文の前の状態」と「文の後の状態」の 2 点だけで、文の中の Fault の辺は腕の出力のまま。

### 1.4 loop が走査に入り込んでいる所

- `scan_statement_flow` の呼び出しは 3 か所: 入口(`home_new_prefix.rs:605`)、`if` の枝(`home_new_prefix_branch.rs:100`)、loop の本体(`home_new_prefix_source_loop.rs:126`)。
- loop は走査の進み方の中に **記録の hook** を 6 種類持つ: `begin_source_loop_body_v1`、`visit_source_loop_statement_v1`(文の位置 `index` を渡す。`scan.rs:127`)、`observe_source_loop_exit_v1`、`record_source_loop_{return,call,expression,child,array_append,if}_v1`、`end_source_loop_body_v1`(定義は `home_prefix_source_loop_control.rs:326-507`、`home_prefix_source_loop_ports.rs:413`)。hook は `PrefixLocalFlow` の `source_loop` の欄(Some の時だけ動く)に書く。
- loop の file は 7 つ・3,602 行(`home_prefix_source_loop*.rs`。`home_new_prefix_source_loop.rs` と `home_new_prefix_search_loop.rs` は別)。

## 2. 案 1 で作る物

### 2.1 新しい解析が出さないといけない物

| 出す物 | 今どこで作るか | 新しい解析で新しく書く物 | 今の部品で済む物 |
|---|---|---|---|
| prefix(`CallerNewHomePrefixV1`、`results`) | 腕と `scan.rs`/`terminal.rs`。`prior_homes`、`normal_homes`、`required_unwind`、`outward_fault`、`covered_statements` | 辺ごとの残る責任の列を、CFG の dataflow から出す部分 | `HomesV1::fault_order`、`normal_order_after`(`home_ownership_transition.rs`) |
| RootHomeFlow(`exits`、`named_arrays`、`maps`、`local_calls`、`expression_plans`) | 同上 + `locals.named_array_observations()` | exit ごとの `RootHomeExitV1`(責任の列、`path_calls`、`covered`)を CFG の exit node から出す部分。`local_calls` の行を作る部分 | `RootHomeExitV1::implicit_from_live_v1` |
| terminal relation | `terminal.rs`(771 行) | `return` の種類ごとの `TerminalRelationV1`(63 file が読む) | 読む側は変えない |
| argument observation | `new_local.rs`、`local_statement.rs` | `new` の実引数の観測 | |
| result prefix | `terminal.rs` | 同上 | |
| ledger への発行(closure) | `ordinary_new_coseal_issue.rs:511-640` 付近の closure。`local_field_read` → `staged_reads`、`field_is_integer` → `staged_reads`、`argument_field_is_integer` → `argument_staged_reads`、`local_static_call`、`borrowed_actuals` → `borrowed_formal_actuals`、`loop_prestate` → `selected_loop_products`。走査の後で `merge_staged_*`(`:594-635`) | 新しい解析は同じ closure を同じ問いで呼ぶ必要がある(site を鍵にした問いなので再利用できる、と点検で書いた。**closure の問い方は走査の進み方(どの site をいつ問うか)に依存する場合があり、順序依存は確かめていない**) | closure 自体 |
| readiness の答え | `ordinary_new_coseal_issue.rs:343-370`(旧走査を probe として 1 回回す)。結果が `child_new_ready` として `OwnershipRouteV1::decide` の入力(`ordinary_new_coseal_issue_route.rs:80-88`) | 新しい解析の関数での readiness。**旧走査を probe として呼ぶ形は残せない**(新側の関数は旧走査を通らないため)。新解析で同じ答えを作る必要がある | 規則 S3b-R5 は probe を残すと書いている。新しい variant の関数では、probe を新解析に置き換えないと「1 つの関数の所有は 1 つの解析だけ」に反する |
| `borrowed_formal_actuals`(他の関数の lowering が読む) | `ordinary_new_coseal_issue.rs:433-445`(`!route.walks() && has_borrowed_source_calls` の時だけ)と、走査の中の `borrowed_actuals` closure | 新しい解析の関数で、同じ表を埋める部分 | |
| `unavailable`(最初の失敗の理由)と、未走査の site に入れる既定 | `home_new_prefix.rs:660-680` | 新解析での失敗の理由。**CFG の dataflow は到達順が文の順と同じとは限らない**ので、どの失敗が「最初」かを定義する規則が要る。拒否の名前が変わるのを S3b-R7 は禁じている | |

### 2.2 corpus で新しい解析に通る関数の数

数え方(source を読んだ字面の推定。型検査も実際の受理も見ていません):

- 入力は `runmir.sh` と同じ 329 件(`audit-birth-take/nr-audit/list.txt`、`fixed-16/*.hako`(`dyn_map` を除く)、`bool-claim/list.tsv` の和。再構成して 329 を確認)。内容が同じ file は 1 つ、同じ名前・引数・本体の関数は 1 つと数える。
- 関数は、コメントと文字列を消し、`名前(引数) [: 型] {` を括弧の対応で切って取る。
- **範囲外の印**(関数の本体に 1 つでもあれば外す): `if`、`loop`、`while`、`for`、`match`、`&&`、`||`、`edit`、`try`、`catch`、`break`、`continue`、`%{`、`share`、`null`、`from`、`?`。結果の注釈が i64/bool/Unit 以外、または注釈なしで `return` の式に `new`、文字列 literal、`null`、`[`、`%{` がある物。これが「緩い数」(点検の数え方と同じ。点検の数 55/160、25/58、41/75 は再現した)。
- **「厳しい数」**は、緩い数からさらに、`x.f = ..` 形の field write、`x.f`(呼び出しでない)の field read、`[`、`ArrayBox`、`MapBox` を含む関数を外した数。Pro の範囲の文に field と配列と Map は書かれていない(2 節 `s3b-ownership-cfg-consult` の範囲)。ただし scalar field の読みは要求の行ではない(`ScanRequestV1` の `MeObjectFieldRead` は **object** field)ので、厳しい数は下限、緩い数は上限の目安。真ん中のどこかになる。

| 集合 | file | 関数 | 緩い数(上限の目安) | 厳しい数(下限の目安) | うち `main`(全体 / 緩い / 厳しい) |
|---|---:|---:|---:|---:|---|
| apps 100 本 | 100 | 161 | 56(35%) | 29(18%) | 90 / 27 / 24 |
| 固定 suite のアプリ 8 本 | 8 | 58 | 25(43%) | 4(7%) | 8 / 4 / 3 |
| 固定 16 入力 | 16 | 75 | 41(55%) | 10(13%) | |
| 229 入力の一覧(213 行) | 213 | 313 | 252(81%) | 78(25%) | |
| 329 入力の全部 | 329 | 415 | 282(68%) | 102(25%) | 232 / 154 / 86 |

- **点検(`s3b-plan-review` 2.2 節)の「`main` は固定 suite の 8 アプリで全部 loop を持つので 0 個」は誤りでした。** `mimalloc-lite`、`ownership-child-typed-min`、`ownership-shared-min`、`typed-object-untyped-field-min` の 4 本の `main` は loop を持ちません(`grep` で loop 0 を確認)。ただしこの 4 本は小さい検査用で、大きいアプリ(`allocator-stress`、`binary-trees`、`boxtorrent-mini`、`json-stream-aggregator`)の `main` は loop か `if` を持つ。
- 範囲外の最大の理由は `if`(88 関数)と `loop`(64)、次に `null`(27)、`break`(22)。
- **このうち実際に新しい解析を通るのは、要求の集合が範囲内の物だけです。** 要求の行 19(`ordinary_new_coseal_issue_route.rs:9-48`)のうち、Pro の範囲の文と合わない行は読んで次の通り:
  - 範囲外: `MapLiteral`、`NamedArrayResidence`、`ObjectCallReturn`(結果は Unit/I64/Bool のみ)、`ConditionOperand`(`if` と loop の条件)、`BorrowedFormalWalk` と `FormalFieldRead`(借用は範囲に無い)。
  - 範囲内か決まらない: `MeObjectFieldRead`、`FieldScalarWrite`、`ReceiverHomeCall`、`StoredTerminal`、`DynamicLocalInitializer`、`DirectCallOwnerLoan`、`RequiredScalarSource`、`IntegerReturn`。
  - 範囲内: `TypedStaticCall`、`PrintOperand`、`ValueExpression`、`TextReturn`、`StaticSourceLocal`。
- **要求の集合は関数ごとにコンパイラの中でしか分からない**ので(`has_*` 相当の判定は契約の解決の後)、上の数は要求の集合で絞っていません。実際の数は、緩い数以下になる(確かな上限)が、下限は上の「厳しい数」とは限らない(厳しい数は字面で field を外した数で、要求の集合とは別の物)。
- **受理されている関数かどうかも見ていません。** 329 入力の MIR は 177 が通り 152 が拒否(`s3b-route` の検証)。拒否の中にも、範囲内で通る関数が含まれる可能性があるが、ファイル単位でしか分からない。

### 2.3 案 1 のその他

- **readiness と `borrowed_formal_actuals`** は、2.1 の通り新側でも作る必要がある。route の規則 S3b-R5/R6 は旧走査を前提に書かれているので、新しい variant を足す時に、R5 と R6 を書き直す(規則を先に)。
- 要求の行は「走査を要るか」と「seed を止めるか」の 2 つの性質しか持たない(route 文書の「Opus の確認」の最後の文)。新しい解析が対応できるかを要求の行から決めるには、**第 3 の性質「新しい解析が対応できるか」**が要る。これを足すのは新しい `has_` ではなく表の列。
- `OwnershipRouteV1::decide` の `walk` の式(`ordinary_new_coseal_issue_route.rs:141-143`)に、新しい variant を選ぶ項が増える。**この項が形を選ぶ分岐にならないよう**、「全部の要求が新しい解析に対応」を表の列から導く形にする必要がある。

## 3. 案 2 で作る物

### 3.1 腕をそのまま呼べる所と、変えないと呼べない所

| 場所 | そのまま呼べるか | 理由 |
|---|---|---|
| 腕(`print`、`binding_rebind`、`field_write`、`named_array`、`local_statement` の腕、`terminal`、`new_local` ほか) | 呼べる(node = 文の時) | 1.3 の通り、入力は「文 + 状態 + 蓄積」で、次の文や位置に頼らない |
| `commit_statement_takes_v1`(`scan.rs:609`、`:320`) | 呼べる | 文のあとの node か、文の node の末尾 |
| 使用後 move の掃き出し(`scan.rs:128-141`) | 変える | 「文の部分木から `if` と loop の本体を除く」条件が、走査が文ごとに見る前提。CFG の node ごとに「この node の中の参照」を引く形に書き直す |
| `if` の腕(`branch.rs:235-770`) | **腕ではなく、案 2 が置き換える物** | `walk_branch`(`:33`、再帰)、`BranchPath` の 6 つ組、join の約 130 行(`:648-770`)、`null_guarded_local`、`null_exclusion::issue`。join の中の `HomesV1::join`(`home_ownership_transition.rs:39`)は使える |
| 出力の map への書き込み | 変える | 経路を複数回訪れる(CFG の worklist では、join の前後で同じ node を再訪しうる)なら、上書きと重複を防ぐ規則が要る。1 回の RPO で訪れるなら不要 |
| 式の中の Fault の辺と評価順 | **node を文にすると CFG に現れない** | 腕の中の `prior_homes`、`required_unwind`、`outward_fault`。node を式の中の操作に細かくすると、これらを腕が作る代わりに CFG の辺として持つ形に、腕の中を書き直す必要がある(案 2 の前提が崩れる) |
| loop の hook 6 種(1.4) | 変える | 下の 3.2 |

### 3.2 loop を案 2 の中に入れるか外すか

- **入れる場合**: loop は CFG の back edge になる。1.4 の hook 6 種と、`home_prefix_source_loop*.rs` の 3,602 行を CFG の上の fixpoint に書き直す。これは S4 の範囲そのもの。
- **外す場合**: loop を「1 つの node」として扱い、その node の中で `home_new_prefix_source_loop.rs:126` が `scan_statement_flow` を再帰で呼ぶ。**そうすると `scan_statement_flow` は残る**(loop の本体を歩く物として)。案 2 の「置き換え」は loop の外側だけになり、旧い進み方が loop の中で生き続ける。
- **境目の候補**: loop を含む関数は、案 2 の対象外にする(= 案 1 と同じ「関数単位の担当」)か、loop を不透明な node とする。前者は案 2 が案 1 と同じ「新旧が関数で並ぶ」形になり、後者は `scan_statement_flow` が残る。**どちらでも「全部の関数で置き換える」にはならない**。
- どちらを選ぶか、規則に当たる決まりは SSOT の中で見つけていない(未確認)。

## 4. 比べる表

| 観点 | 案 1 | 案 2 | 案 3(7 節) |
|---|---|---|---|
| series の大きさ | 大。新しい file 多数(CFG の型、dataflow、出力 5 つの adapter、readiness、`borrowed_formal_actuals`)。触る既存 file は route(152 行)と `ordinary_new_coseal_issue.rs`(796 行)が中心。**行数は測っていない** | 中〜大。触る file は `scan.rs`(625)、`branch.rs`(778)、`source_loop.rs`(796)と、状態の持ち方(6 つ組)。腕は触らない前提。loop の扱いで大きく変わる(3.2)。**測っていない** | 1 series ごとは小〜中(歩 B/A は数百行)。全部では大。**測っていない** |
| 今通る形を落とす危険と見つける手段 | 範囲内の関数だけが危険。見つけるのは 329 + 1,459 本の MIR の同一で、**範囲内の関数が corpus に入っていれば**見つかる。**拒否の名前の順序**(最初の失敗)が変わる危険は、拒否 624+152 の MIR(エラー文字列を含む)の同一で見つかる(`mir.sh` は stderr も取る) | 全部の関数が危険。見つける手段は同じ(同一の比較)。危険の大きい所は **join と連鎖の順序**で、corpus に `if` の分岐の乖離(`HomeFlowBranchDivergent`)や分岐の中の move を含む入力がどれだけあるかは**数えていない** | 1 series の範囲が小さく、歩 B/A と同じ検査が使える。連鎖の順序(1.1)を変える時が危険 |
| 同じ series で消せる旧い判定 | **ほぼ 0**(範囲外の関数が旧い物を使い続ける)。消せるのは、新側で不要になる readiness の旧走査の呼び出しくらい | `scan_statement_flow` の再帰、`walk_branch`、`BranchPath`、join(約 130 行)、使用後 move の掃き出し(D6 の R1)。**地図の D1〜D6 と 25 か所、27 の腕は消えない**(腕は部品として残る) | 腕の分を 1 つずつ。D5(`move` の解釈 10 か所)、D6(使用後 move の 3 粒度)、D3(同 scope + `proves_original_move`)が、`b75a4bb169` でも残っている(`grep` で確認: `.moved(` の行 11、`was_moved_out`/`has_moved_out` の行 8、`proves_original_move` の行 3。テストと定義を除いた grep の行数で、意味の重複は区別していない) |
| 新旧が並ぶ期間と増える物 | 長い。新しい解析が全部の腕の機能を持つまで、関数単位で並ぶ。増える物: 第 2 の解析、第 2 の readiness、adapter | 短い(loop を外さない場合)。loop を外すなら、`scan_statement_flow` が loop の中に残る | 並ぶ物はない(同じ走査の中で腕を置き換える)。腕の中に新旧が混ざる期間が 1 series 内にある |
| 後の段階が同じ土台に乗れるか | 値の `&&` `||`: 新しい解析が式の中の操作まで持つので乗せやすい。ただし範囲を広げるたびに出力 5 つを再現する必要がある。`if` の合流・loop: 新しい解析に入れるたびに範囲が増える(2.2: `if` を入れると apps 緩い数が 56 から 74 付近) | `if` の合流: 入る(置き換えの本体)。loop: 3.2。**値の `&&` `||`: 進まない**(式の中の node が要るが、腕の中に埋まっている)。所有の String・object・nullable の結果: 腕の側の仕事で、案 2 は無関係。`edit`: 同様。新しい分類器が増えるかは、腕が増えるかによる | 腕を契約の遷移にすれば、後の段階は遷移を足す形になる。値の `&&` `||` は別に CFG 側の仕事が要る(案 3 の範囲外) |
| 決まりを守るか | 「1 つの関数の所有は 1 つの解析」: 守る。fallback なし。**形を当てない**: 範囲を決める表の列を作る時、操作と契約の名前で書ける必要がある(2.3) | 守る。ただし loop を外した形は、1 つの関数の中で新旧の進み方が混ざる(違反) | 守る。ただし連鎖の順序(first-claim-wins)を契約の表の dispatch に変える時、順序が暗黙に形を選んでいた所が表に出る(良い面もある) |
| lowering を `CanonicalCfgSessionV1` と `PhiTxn` に乗せる時期 | `finish` が関数の全 block の seal を要求する(`canonical_cfg/session.rs:371-379`)ので、乗せるのは**関数全体の lowering の切り替え**。案 1 では新しい variant の関数だけ session を通す形になり、lowering も関数単位で新旧が並ぶ。`emit_root_invoke` が拒む操作(`DynamicCarrierEnd` ほか)を足す必要(点検 1.3) | 同じ要求。全部の関数が同じ CFG なので、**lowering に渡す CFG が 1 種類**になる利点はある。ただし CFG が文の粒度だと lowering が要る操作の粒度とは違う | 乗せる時期は S3b の後。案 3 は CFG を作らない |

## 5. 勧めと、勧めが変わる条件

**勧め: 案 3 を先にやり、案 1 か案 2 の判断は 1〜2 series の後にする。**

理由:

1. 案 1 と案 2 は、どちらも**今の腕を残す**ので、地図の 25 か所の書く所と 27 の腕(D1〜D6 のうち D5、D6、D3)は消えません。S3b の目的(「新しい使い先をつないだ時に、新しい分類器と cleanup の持ち主が増えない」)に近づくのは、腕の中の判定を契約の遷移にする案 3 の方です。
2. 案 2 は、node = 文の CFG では今の再帰の書き換えにしかならず、`&&`(値)の役に立ちません。node を細かくすると腕を変える必要があり、そのとき案 3 の仕事を先にやる方が小さくなります(腕が「状態 + 契約 → 次の状態」の関数になっていれば、node の上で呼ぶのは簡単)。
3. 案 1 は範囲内の関数が少なく(2.2)、旧い判定を消せず、readiness と `borrowed_formal_actuals` を新旧で作る必要があります。`s3b-route` の「Opus の確認」が残した宿題(要求の行に「何の出力が要るか」を持たせる)も、先に要ります。
4. 案 3 は歩 B と歩 A と同じ進め方で、同じ検査(329 + 1,459 本の MIR の同一、拒否の名前を含む)がそのまま使えます。

**勧めが変わる条件:**

- 「値の `&&` `||` や `if` の新しい受理を、短い期間で出したい」が最優先なら、案 1(範囲を限った新しい解析)を先にします。案 3 は受理を広げません。ただしその時は、範囲を**今拒否している形**に取る読み(旧い走査は一切触らず、拒否の入口に新しい解析を足す)も検討に値します。今通る形を落とす危険が 0 になりますが、旧い判定を消せず「置き換え」でなく「追加」になるため、SSOT の sunset law(`mirbuilder-inplace-replacement-policy-ssot.md:371-373`)と合うかは**未確認**です。
- 案 3 を 1 series 試して、連鎖の順序(1.1)が暗黙の分類を持っていて、契約の dispatch に直せないと分かったら、案 3 は止めて案 2 を node = 文で作る(順序は CFG の辺の順序として固定される)。
- loop(S4)を先にやると決めるなら、案 2 の loop の入れ方(3.2)の問題が消えるので、案 2 を再検討します。

**分からない所:**

- 27 の腕のうち、何個が契約ごとの遷移にできるか(数個しか中を読んでいない)。
- 案 1 の範囲内の関数が、今の走査を通って受理されている数。
- 案 2 で出力の map の上書きが起きないか(実行して確かめていない)。
- 各案の行数。

## 6. 未確認と読んでいない所

- build、test、実行はしていません。数は字面の推定です(2.2)。
- `scan_new_home_flow` の closure は、`scan_statement_flow` の引数で 14 個数えました。地図と点検の「17」との差は確かめていません。
- 腕 27 のうち、中を読んだのは `local_statement.rs` の前半(`:60-240`)、`scan.rs` 全体、`branch.rs` の join(`:640-778`)、`source_loop.rs:100-175`、`terminal.rs` と他の腕は出現回数の grep のみです。
- 1,459 本の入力の中身は読んでいません(route 文書の「Opus の確認」の数字を使いました)。corpus に `if` の分岐の乖離や、分岐の中の move がどれだけあるかは数えていません。
- SSOT `mirbuilder-inplace-replacement-policy-ssot.md` は全文を読んでおらず、sunset law と retry/fallback の行(`:36`、`:241`、`:371-373`)を読みました。loop を新旧の境目に使えるかの規則は見つけていません。
- 独立 verifier と package 側の照合(`ownership_ssa/`、`ordinary_new_local_commit/**`)は、読んでいません。案 1 と案 2 のどちらでも、検証側が期待する列が変わらないかは、確かめていません。

## 7. 案 3: 腕を契約ごとの遷移へ 1 つずつ置き換える

内容: 歩 B で `HomesV1` と `PrefixLocalFlow` の遷移操作ができた(`home_ownership_transition.rs`、`acquire_*`、`release_to_callee`、`move_alias`、`transfer_out`、`rebind_text`)。腕の中に残っている「どの遷移を呼ぶかの判定」(`move` の解釈、使用後 move の 3 粒度、「同 scope + `proves_original_move`」)を、**契約(`move` の意味、take の入場、share の複製など)ごとに 1 か所の関数**にして、腕は契約の関数を呼ぶだけにする。1 series に 1 つの契約。

なぜ入れたか: 歩 B/A と同じ方法で、旧い判定をその series で消せるため。案 1 と案 2 の評価で、「消せる旧い判定がほぼ無い」が共通の弱点だった(4 節)。

弱点(良く見せないために書く):

- 受理を広げない。値の `&&` `||`、`if` の新しい受理、所有の結果は直接進まない。
- 連鎖の順序(1.1)が暗黙の規則になっている所があり、契約ごとにまとめる時に、順序を変えず契約の関数を挟めるかは、中を読んで確かめていない。
- CFG は作らない。lowering の乗せ替え(4 節の最後の行)は進まない。
- 腕 27 のうち、契約の遷移にできる数が分かっていない(5 節)。

最初の候補(読んだ範囲): D5(`move` の解釈。resolved_semantics に `home_new_prefix_local_statement.rs:177`、`home_call_entry_transfer.rs:79`、`home_new_prefix_new_local.rs:171`、`home_local_call_borrowed_actuals.rs:99`、`home_prefix_observe.rs:90` の 5 か所、package に `ordinary_new_borrowed_formal_actual_seed.rs:359` ほか 5 か所)と、D6(使用後 move: `scan.rs:128`、`home_call_entry_transfer.rs:108-114`、`local_statement.rs:187`、`:229`、`home_new_prefix_new_local.rs:234`)。この 2 つは「`move` の意味」という 1 つの契約にまとまる、と読んだが、package 側の 5 か所は読んでいない。

## 8. Opus の確認(2026-11-07、読んだだけ)

- `scan_statement_flow` を呼ぶ所は、本文のとおり 3 か所だった(`home_new_prefix.rs:605`、`home_new_prefix_branch.rs:100`、`home_new_prefix_source_loop.rs:126`)。loop の本体も同じ進み方を再帰で使うので、loop を外した案 2 では、旧い進み方が残る。
- `scan.rs:117` は、本文のとおり文の列を前から順に辿る `for` だった。
- `mirbuilder-inplace-replacement-policy-ssot.md:369-373` は、compatibility owner を「一つの selected pipeline の中で、移した owner と重ならずに 1 回だけ選ぶ分岐」としてだけ許し、第二の入口と双方向の retry/fallback を禁じている。本文の 5 節が「拒否の入口に新しい解析を足す」読みを未確認としたのは妥当で、この文に照らすと、その読みは第二の入口になりやすい。
- 固定 suite のうち `mimalloc-lite`、`ownership-child-typed-min`、`ownership-shared-min`、`typed-object-untyped-field-min` の `main` に loop が無い事を確かめた。点検(`s3b-plan-review-2026-11-07.md` 2.2 節)の「`main` は全部 loop を持つ」は誤りで、本文の訂正が正しい。
- 本文が自分で書いているとおり、27 の腕のうち中を読んだのは数個で、案 3 がどこまで効くかは 1 series やってみないと分からない。
