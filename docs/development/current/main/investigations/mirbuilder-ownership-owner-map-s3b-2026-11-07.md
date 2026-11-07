# 所有の状態と Fault の後片付けを決める持ち主(S3b 段階 0。地図)

Status: 読解の記録です。authority ではありません。branch `s3b-survey`(main `ec9cd19681` から)。source は変えていません。build も test も流していません。行番号は `ec9cd19681` のものです。

設計は `mirbuilder-structure-refactor-queue-d0-2026-08-23.md` の節「流れの作り直し」、目標の流れは `mirbuilder-route-selection-survey-2026-11-06.md`、責任が移る時点は `result-contract-consult-2026-11-06.md` の回答 4。書き方は S3a の地図(`mirbuilder-value-class-owner-s3a-2026-11-06.md`)に合わせました。

読んで確かめられなかった事は、その場で「未確認」と書きます。

## 0. 先に結論(数字は 6 節)

- 所有の状態を決める所は **25 か所**(書く所。loop は別枠で 1 と数えた)。読んで使用後 move を判定する所が **9 か所**。呼ばれる側で `take` を決める所が **1 か所**。
- 状態は **2 つの入れ物に分かれて**います。`homes: Vec<LiveHomeV1>`(Fault で閉じる責任の並び)と `PrefixLocalFlow.locals: BTreeMap<_, StoredLocal>`(名前の状態)。どの書く所も 2 つを別々に更新します。
- Fault の後片付けを決める所は builder 側に **30 か所**(18 file)。End を実際に書く部品は **22**(形を知らない関数 12、書き込み口の実装 3、形を知っていて決める所と混ざる組立て 7)。
- `CanonicalCfgSessionV1` と `PhiTxn` は、呼び出しと分岐の所有 lowering(`builder/ordinary_new_admission/**`)では **一度も使われていません**。使うのは loop と trivial SSA と動的の物理 emitter だけです。
- 値の位置の `&&` `||` で足りないのは、writer ではありません。**値の位置の枠(frame)と、RHS が条件付きで届く事を持つ Fault の後片付けの型**です(4 節)。
- 分岐の合流の規則は仕様が決めています(`ownership.md` §7。片方だけ move したら合流の後は「maybe available」で拒否)。今の `HomeFlowBranchDivergent` は仕様どおりの拒否で、理由の名前だけ仕様の `home-maybe-consumed-after-branch` と違います。

## 1. 所有の状態を決める所(線形走査 `scan_new_home_flow`)

### 1.1 走査の作り

- 入口 `scan_new_home_flow`(`home_new_prefix.rs:377`)は 17 個の closure を受け取り、`scan_statement_flow`(`home_new_prefix_scan.rs:10`)を 1 回回します。closure は issuer 側の事実(`local_call_route`、`borrowed_actuals`、`view_use`、`object_return` など)を答えるだけです。
- 走査の状態は `PrefixLocalFlow`(名前 → `StoredLocal`、`home_prefix_local_flow.rs:88`)と `homes: Vec<LiveHomeV1>`(`Stable` / `Moved{origin,binding}` / `Text`)。`store`(`home_prefix_local_flow.rs:556`)が前者を書きます。後者は各 observer が直接 `push` / `retain` / 代入します。
- `StoredLocal` の variant は 25(`Consumed`、`MovedOut`、`Home`、`Shared`、`ReceivedHandle`、`ReceivedNullable`、`BorrowedNullable`、`BorrowedMap`、`BorrowedArray`、`FieldAlias`、`Handle`、`Map` など)。状態の名前は仕様の 5 種(生きた Home、移転待ち、移転済みで caller に責任が残る、借用、消費済み)と 1 対 1 ではありません。
  - 生きた Home: `Home`、`Shared`、`ReceivedHandle`、`ReceivedNullable`、`Map`、`NamedArrayHolder`。
  - 移転待ち: 専用の variant は **ありません**。`homes` の中に `Moved{origin,binding}` として残り(`home_call_entry_transfer.rs:57`)、`locals` 側は `MovedOut` になります。
  - 移転済みで caller に責任が残る: `MovedOut`(`home_call_entry_transfer.rs:14`)。
  - 借用: `BorrowedNullable`、`BorrowedMap`、`BorrowedArray`、`FieldAlias`、`StoredMapGet`。
  - 消費済み: `Consumed`(`home_call_entry_transfer.rs:8`)。`MovedOut` との違いは、後の読みが `UseAfterMove` になるか(`MovedOut` は `was_moved_out` で拾う)だけで、`observe` は両方 `None` を返します(`home_prefix_local_flow.rs:862`)。

### 1.2 状態を書く所(決める所)

「前の文の状態」は、その observer が受け取る `locals` と `homes` です。「形」はその observer が動く構文です。

| # | file:line | 決める状態 | 読む source の事実 | 形 |
|---|---|---|---|---|
| 1 | `home_new_prefix_new_local.rs:100-103` | `new` の実引数の Home を Normal 辺で `MovedOut` にし、`homes` から外す。Fault 用に `prior_homes` は外す前の逆順(同 79-85) | 実引数の `move` / 暗黙 transfer(同 184、`observe_new_arguments_v1` が出す `moved`) | `local x = new T(..)` |
| 2 | 同 `:105-107` | 新しい Home を `homes` に push し `install_selected_normal_home` | selected new の `construction_source` | 同上 |
| 3 | `home_new_prefix_terminal.rs:626`(と `:632` の prefix 構築) | 1 と同じ(戻り値位置の `new`) | 同上 | `return new T(..)` |
| 4 | 同 `:667-669` | 返した Home / MapLocal を `consume_home`(`Consumed`)にし、`homes` から外す | `TerminalReturnedSourceV1::{Home,MapLocal}` | `return x` |
| 5 | 同 `:320` | `*homes = remaining`(map literal の子の Home が transfer された後) | `map_literal_keys` と `observe_map` | `return %{..}` |
| 6 | 同 `:379` | 5 と同じ(call の実引数の map literal) | 同上 | `f(%{..})` の戻り値位置 |
| 7 | `home_new_prefix_local_statement.rs:174`(`install_shared_fresh_v1`、`home_prefix_local_flow_shared.rs:47-58`) | 新しい `Shared` を push | `shared_trivial_payload_v1` | `local s = share new T` |
| 8 | 同 `:138`(`retain_shared_clone_initializer_v1` → `install_shared_clone_v1`、同 `shared:72-83`) | `Shared` の複製を push。元は変わらない | `Shared` の acquisition | `local s2 = share s` |
| 9 | 同 `:221`(`install_shared_move_v1`、`shared:32-45`) | 元を `MovedOut`、`homes` から外して `Shared` を push | `moved(..)`(同 180)、同じ scope、`proves_original_move` | `local s = share move x` |
| 10 | 同 `:262`(`transfer_home_v1`、`home_call_entry_transfer.rs:35-61`) | `homes[i] = Moved{origin,binding:to}`、元を `MovedOut`、`to` に元の `StoredLocal` を移す | 同 scope、`proves_original_move`(同 249) | `local b = move a` |
| 11 | 同 `:393` | `ReceivedHandle` / `ReceivedNullable` を install し push | `LocalCallResultClassV1` | `local x = recv.m(..)` の Handle / Nullable の結果 |
| 12 | 同 `:482` | `*homes = remaining`、push、`install_map` | `observe_map` | `local m = %{..}` |
| 13 | `home_new_prefix_mixed_call.rs:46` | push | mixed の結果 | `local x = f(..)`(mixed の結果) |
| 14 | `home_local_call_route.rs:115` | push | `LocalCallRouteV1` | 経路が選ばれた local call |
| 15 | `home_new_prefix_named_array.rs:117` | push、`NamedArrayHolder` を store | holder の loan | 名前付き配列の初期化 |
| 16 | `home_new_prefix_initializer_expression.rs:247,259` | `Text` または `Stable` の Home を push | `CheckedInitializerExpressionFrameV1` | `local s = a + b`(owned text) |
| 17 | `home_new_prefix_binding_rebind.rs:131-133` | 旧 Home を外し、`Text` の Home を push | `is_text_residence` | `x = a + b`(再代入) |
| 18 | `home_new_prefix_owned_field_write.rs:75-76` | `consume_home` と `homes.retain`(同じ 2 行) | owned Array の field write 計画 | `me.f = move x`(owned Array) |
| 19 | `home_map_value_flow.rs:139-141` | `remaining.retain` と `consume_home`(同じ 2 行) | map の field install | map field install |
| 20 | 同 `:282` | 自由関数 `transfer_home_v1`(同 `:127`。`Consumed` にして外す) で `normal` を作り `*homes = normal` | install の `binding` | map field install |
| 21 | `home_map_flow.rs:681` | 同じ自由関数 `transfer_home_v1` で `remaining` から外す。`MapEntryOwnership::TransferHome` | `direct_available_home(child)` | map literal の子 |
| 22 | `home_map_descendant_flow.rs:97` | `*homes = remaining` | `observe_map` | ネストした map |
| 23 | `home_call_entry_transfer.rs:177`(`commit_statement_takes_v1`。呼び元 `home_new_prefix_scan.rs:321,610`、`home_new_prefix_terminal.rs:735`) | `taken` の各 binding を `MovedOut` にし、`homes` から外す | issuer の `CallEntryHomes` の答え(`taken`、`preserves_reads`) | メソッド呼び出しの文、`take` の formal |
| 24 | `home_new_prefix_branch.rs:688-697,723,748-765,769` | If の合流。`homes` は両方の fall-through で共通の物だけ残す(順序つき積)。`locals` は `join_branch_at`(`home_prefix_borrowed_flow.rs:156`)。食い違えば `HomeFlowBranchDivergent` | then / else それぞれの走査結果、`terminated` | 文の `if` |
| 25 | `home_new_prefix_source_loop.rs:123` と `home_prefix_source_loop*.rs` 一式(3,997 行。`home_new_prefix_search_loop.rs` を含む) | loop の本体と after の `homes`。**別枠(S4)**。今回は読んでいません | `SelectedLoopHomeLoanV1` | `loop` |

### 1.3 状態を読む所(使用後 move、前の文の状態)

| # | file:line | 何を読むか | 読む「前の文の状態」 | 形 |
|---|---|---|---|---|
| R1 | `home_new_prefix_scan.rs:129-141` | 文の部分木にある全 local 参照のうち `was_moved_out` の物。分岐と loop の本体は除外 | `locals`(`MovedOut` の有無) | 全文 |
| R2 | `home_call_entry_transfer.rs:70-143`(`validate_pending_call_uses_v1`) | 1 つの式の中の評価順で、move 済みの root / receiver を読んでいないか。`simulated`(`:87,:92`)に `MovedOut` を書く小さな状態機械を持つ | 走査の `locals` の複製 | method call、`new` の実引数 |
| R3 | 同 `:159-166` | R2 を呼ぶかの門(`taken` が空でない、または `has_moved_out`) | `locals` | method call の文 |
| R4 | `home_new_prefix_new_local.rs:247-251` | R2 を `new` の実引数に使う門 | `locals` | `new` |
| R5 | `home_new_prefix_local_statement.rs:190` | `share move` の元が `MovedOut` か | `locals` | `share move x` |
| R6 | 同 `:234` | `move` の元が `MovedOut` か | `locals` | `local b = move a` |
| R7 | `home_prefix_local_flow.rs:858-862` | `Handle(root)` の root が `Consumed` / `MovedOut` なら観測を `None` にする | `locals` | alias の読み |
| R8 | 同 `:1091-1092` | field read の候補に `Consumed` / `MovedOut` を残す(診断のため) | `locals` | field read |
| R9 | 同 `:322-396`(`stored_local_same`) | 合流で `Consumed`=`Consumed`、`MovedOut`=`MovedOut`、`Home` は acquisition と構造の同一性で比べる | 両枝の `locals` | `if` の合流 |

### 1.4 呼ばれる側

| # | file:line | 決める事 |
|---|---|---|
| C1 | `ordinary_new_coseal_issue_call_arguments.rs:33-39` → `call_argument_source.rs:240`(`source_take_homes_v1`) | どの実引数の Home が `take` で callee に渡るか(`taken`)と、callee の閉包が read loan を保つか(`preserves_reads`)。走査は答えを受けて #23 で状態を書く |

### 1.5 借用の状態(参考)

借用は Home の数に入れていません。状態を作る所は次の 12 か所です。

- `home_prefix_borrowed_flow.rs:166,173,179,184`
- `home_prefix_borrowed_nullable.rs:59,158,223,315`
- `home_prefix_field_alias.rs:148`
- `home_prefix_named_array.rs:66,86,116`

無効にする所は 3 か所です。

- `invalidate_declared_read_write_v1`(`home_prefix_borrowed_nullable.rs:357`)
- `invalidate_declared_read_call_v1`(同 `:379`。`home_call_entry_transfer.rs:151,157` から呼ばれる)
- `invalidate_dynamic_field_sources`(`home_prefix_field_readiness.rs:11`)

借用の貸し出し自体は issuer の `borrowed_actuals` closure が答えます。issuer 側(`ordinary_new_borrowed_formal_*`)は読んでいません(未確認)。

### 1.6 分岐の合流で今起きる事(`home_new_prefix_branch.rs`)

- 両枝が終端(`return`)なら、その `if` で枝を閉じる(`:661`)。片方が終端なら、もう片方の状態がそのまま続く(`:667-`)。
- 両方が fall-through なら、`homes` は **順序つきの共通部分**(`:688`)。`then.homes != joined` か `else.homes != joined`、または `join_branch_at` が失敗したら `HomeFlowBranchDivergent`(`:723`)。和も空の代入もしない(ファイル冒頭 `:5`)。
- 片方にしかない binding は `Uninitialized` に落ちる(`home_prefix_borrowed_flow.rs` の `join_branch_at`)。`nonnull` と `null_exclusions` は両枝に同じ物がある時だけ残る(同)。
- 欠けた `else` は入口の状態の複製(`:636` の `Err(_)` 腕)。
- **End は合流では出ません。** Home は関数の出口(`return` の root exit)でだけ閉じ、Fault は各 fallible 操作の suffix で閉じます(2 節)。枝の中で作った Home を枝の中で閉じない限り、合流は `Divergent` で拒否されます。

## 2. Fault の後片付け(End)を決める所と、書く所

### 2.1 書く部品

End は `InvokeOperation`(`String::End`、`DynamicCarrierEnd`、`HomeRelease`、`HomeReleaseIfLive`、`Map::End`、owned array の `End`)や `ArrayResidenceRelease` です。

| 区分 | 部品 | file:line | 入力 |
|---|---|---|---|
| 形を知らない | `cleanup_step_with_writer` | `root_graph.rs:325` | 1 つの操作、normal / fault の次の block |
| | `cleanup_action_step_with_writer` | `root_graph.rs:306` | 同 + `ArrayRelease` |
| | `emit_pending_root_fault_sequence` | `root_graph.rs:278` | 全操作の列と添字の列(同じ suffix は共有) |
| | `emit_cleanup_actions_to_fault_with_writer` | `root_graph.rs:222` | normal 列、fault 列、各 normal 後に残る fault の添字、acquisition 添字、fault の終端 |
| | `emit_cleanup_actions_with_writer` | `root_graph.rs:175` | 上 + `ReturnFault` の終端を作る |
| | `emit_cleanup_suffixes_with_writer` | `root_graph.rs:158` | `InvokeOperation` 版の薄い皮 |
| | `emit_cleanup_suffixes_to_fault_with_writer` | `root_graph.rs:204` | 既存の fault suffix に繋ぐ版 |
| | `cleanup_chain` / `cleanup_step` / `append_block` | `selected.rs:640,655,674` | 操作の列と終点 |
| | `emit_scalar_operation_with_pending_v1` | `expression_value.rs:740` | 操作、`pending`(Fault で閉じる列) |
| | `emit_scalar_operation_to_landing_v1` | `expression_value.rs:754` | 操作、normal / fault の着地(Invoke と `InvokeNormalResult` を書く) |
| 書き込み口 `RootGraphWriterPortV1` の実装 | `RawRootGraphWriterV1` | `root_graph.rs:48` | `builder.emit_instruction` / `function.add_block` を直接叩く |
| | `LocalizedCleanupWriterV2` | `source_loop_cleanup.rs:51` | loop 用 |
| | `CanonicalRootGraphWriterV1` | `static_loop_root_graph_writer.rs:418`(使うのは `static_loop_entry_session.rs:79` だけ) | `CanonicalCfgSessionV1` 経由(`emit_root_invoke` / `emit_root_fault`) |
| 形を知っていて、決める所と混ざる | `emit_root_graph_with_arguments_v1` | `root_graph.rs:368` | `return` の出口。ingress と実引数の End、`result_cancellation_v1` を知っている(`:417-453`) |
| | `prepare_operation_cleanup_v1` | `root_graph.rs:481` | `normal_ends`、cancellation、`unwind`。String End の重複を拒否する(`:493-498`) |
| | `finish_body_cleanup` | `expression_value/cleanup.rs:393` | 式の本体。置き換えで押し出される旧 Home(displaced)の順を決め直す(`:411-437`) |
| | `finish_new_arguments_v1` | `arguments.rs:592` | `new` の実引数の一時 Text |
| | `source_loop_cleanup::emit` | `source_loop_cleanup.rs:30-` | loop の plan |
| | `print_expression` の組立て | `print_expression.rs:38-44` | print の式 |
| | `terminal_call::emit_ingress` | `terminal_call.rs:267-` | 出口の call |

書き込み口の trait(`root_graph.rs:19-44`)は 5 つの method(`reserve_block`、`append_array_release`、`append_terminated`、`emit_current`、`insert_projection`)で、アルゴリズム(suffix の共有、Fault の ReturnFault 終端)は 1 つです。**実装は 3 つあり、そのうち 1 つだけが `CanonicalCfgSessionV1` を通ります。**

### 2.2 決める所(builder 側、30 か所 / 18 file)

「決める」は、その箇所で End の列(Normal の列、Fault の列、各 Normal 後に残る Fault の添字)を作る事です。

| file | 箇所(file:line) | 数 |
|---|---|---|
| `selected.rs` | `:163`(`temporary_ends_v1`)、`:176`(`cleanup_chain`)、`:182`(`cleanup_step`) | 1 |
| `selected/arguments.rs` | `:595-608` | 1 |
| `selected/map.rs` | `:68`、`:313` | 2 |
| `selected/owned_array.rs` | `:29`、`:116`、`:329` | 3 |
| `selected/map_field_install.rs` | `:87`、`:89` | 2 |
| `selected/initializer_expression.rs` | `:101` | 1 |
| `selected/field_array_append.rs` | `:56` | 1 |
| `selected/terminal_call.rs` | `:374`、`:477`(`deferred_argument_ends_v1` は `:39`) | 2 |
| `selected/terminal_map_get.rs` | `:86` | 1 |
| `selected/terminal_call/lexical_receiver.rs` | `:45` | 1 |
| `fields/checked_read.rs` | `:120` | 1 |
| `selected/terminal_call/lexical_i64.rs` | `:276-309`(`prepare_argument_cleanup_v1`。`lexical_nullable.rs:238`、`lexical_object_expression.rs:45` が使う) | 1 |
| `selected/checked_field_write.rs` | `:41` | 1 |
| `selected/root_graph.rs` | `:481`、`:368` | 2 |
| `selected/expression_value.rs` | `:366`、`:455`、`:507`、`:565`、`:639`、`:701` | 6 |
| `selected/expression_value/cleanup.rs` | `:386-390`(普通)と `:393-452`(displaced) | 2 |
| `selected/print_expression.rs` | `:38-44` | 1 |
| `selected/source_loop_cleanup.rs` | `:40` | 1 |

合計 30。**決める所と書く所が同じ関数に混ざっているのは、ほぼ全部です。** 例外は `expression_value/cleanup.rs:386-390`(決める)と `finish_body_cleanup:393`(書く)が別関数な所だけですが、書く側が displaced の順を決め直すので、そこも混ざっています。`return_expression.rs:41` と `binding_rebind_expression.rs:46` は `unwind` を渡すだけの経路で、数に入れていません。

### 2.3 規則が 1 つになっている所と、なっていない所

- 1 つになっている: **「Fault は、既に試した Normal 操作を除く全ての責任を保つ」**(`fault_indices_after_normal_step_v2`、`root_home_cleanup_order.rs:189`)。builder 側の呼び元は 2 か所(`arguments.rs:603`、`expression_value/cleanup.rs:434`)、package 側は 3 か所(`root_home_cleanup_order.rs:484`、`expression_terminal_paths.rs:700`、`source_loop_cleanup.rs:104`)。displaced の版(`displaced_replacement_fault_sequences_v2`、同 `:207`)の呼び元は 2 か所(`cleanup.rs:427`、`expression_terminal_paths.rs:684`)。
- なっていない: **添字を手で作る所**が `root_graph.rs:425-427`(実引数の End)、`:504-509`(`call_fault`)、`expression_value.rs:368,457,747`(`(0..pending.len())`)、`arguments.rs:600-601`(`first..first+len`)に残っています。
- 検証側(`normal_callable_semantic_package/ordinary_new_local_commit/**`)は同じ列を期待として **再導出**し、emit された CFG を歩いて照合します。`pending_operations(function, id, expected, frame)`(`root_cleanup_order_paths.rs:220`)の呼び元は 21 か所 / 10 file。builder 側の同名関数 `pending_operations`(`expression_value/cleanup.rs:4`)とは **別物**(builder 側は列を作り、package 側は CFG を歩く)。名前が衝突しています。
- 独立した verifier は別にあります(`src/mir/ownership_ssa/published_homes.rs`。status-index.md:391 の記述では、Home が 1 つの object で、handle 結果・text・複数 copy・edge 引数・managed operand の call を持つ関数は対象外)。未読です。

## 3. CFG と PHI の持ち主

### 3.1 `CanonicalCfgSessionV1`(`resolved_lowering/canonical_cfg/session.rs`)

| API | file:line | 役割 |
|---|---|---|
| `create_block` | `session.rs:80` | block を作り、`creation` に記録 |
| `select_block` | `:104` | 作った block に builder を移す |
| `emit_jump` | `:129` | Jump と predecessor の cache を一緒に書く |
| `emit_return` | `:150` | Return |
| `emit_checked_callout` / `emit_checked_callout_fault` | `:167` / `:303` | CheckedCallOut の 2 辺と、その Fault 着地の終端 |
| `emit_pinned_text_residence_*` | `:221,273,292` | pinned text の入退場 |
| `emit_branch` / `prepare_branch` | `branch.rs:57` / `:72` | Branch と predecessor |
| `emit_root_invoke` / `emit_root_fault` | `session/invoke.rs:12` / `:109` | root Home graph の Invoke と `ReturnFault` |
| `emit_i64_invoke` / `emit_invoke_fault` | `invoke.rs:128` / `:175` | i64 の fallible call と Fault 着地 |
| `seal_block` | `session.rs:337` | その時点の predecessor を MIR terminator から導出して証人にし、block を封印 |
| `finish` | `:371` | 全 block が封印済みで、封印後に predecessor が変わっていないか(`SealedPredecessorsChanged`)、封印した block が消えていないか(`SealedBlockRemoved`)を確かめる |

- **Fault の辺を作る所**: `emit_checked_callout`(Normal と Fault の 2 辺を書く)、`emit_root_invoke`、`emit_i64_invoke`。Fault の着地 block の終端は `emit_checked_callout_fault`、`emit_root_fault`、`emit_invoke_fault` が書く。いずれも、その着地が実際に宣言済みの Invoke / CheckedCallOut の fault 着地かを確かめます。
- **seal の時点**: 入ってくる辺が全て書かれた後です。例: `dynamic_loop_phi_close.rs:147-158` は Enter、body、terminal backedge、Header の順に封印し、Header の証人だけが PHI の完成を許す。`static_loop_tail.rs:118`(After)、`trivial_ssa/lowerer.rs:461`(各 block)など。
- **使われている所**: loop(`static_loop_*`、`loop_recipe_physicalizer/**`、`direct_accum_lowerer.rs`、`nested_predicate_*`)、trivial SSA(`trivial_ssa/lowerer*.rs`)、動的の物理 emitter(`selected_dynamic_physical_emitter/**`)、`common_v2_session/s6c_cursor_cfg.rs`。
- **使われていない所**: `builder/ordinary_new_admission/**` 全体(`grep` で `canonical_cfg` / `CanonicalCfgSession` / `PhiTxn` が 0 件)。つまり **呼び出しの前後と Fault の辺は、`builder.emit_instruction` と `function.add_block` で直接書かれ、封印の証人を持ちません**。predecessor は `function.update_cfg()`(`ops/logical_shortcircuit.rs:274` ほか)と、生成後の独立 verifier が確かめます。
- 分岐の lowering は 2 系統あります。
  - 値の join を持つ `located_if.rs`(`resolved_lowering`)は `IfCfgSessionV1`(`if_materialization.rs:52`)と `define_join_phis`(`:481`)を使い、`CanonicalFunctionLowererV1` の中で動きます。**所有(Home、End、Fault)を一切知りません**(`located_if.rs` に Home / Invoke / cleanup の語は無い。`with_cleanup` は失敗時の巻き戻し)。
  - 所有を持つ関数では、`if` は従来の経路(`stmts/if_statement_descent.rs` → `if_form`)で書かれ、Home の閉じ方は return の出口に付きます。

### 3.2 `PhiTxn`(`emission/phi_lifecycle.rs:133`)

- API: `begin`(`:144`)、`define_provisional_phi`(`:151`)、`patch_phi_inputs`(`:165`)、`rollback_pending_phi`(`:189`)、`commit`(`:213`)、`abort_on_err`(`:234`)。仮の PHI を定義し、入力を埋めるか巻き戻すかを順に強制する。
- 使われている所: loop の PHI 系(`loop_accum_*`、`loop_phi_materializer.rs`、`dynamic_loop_phi_close.rs`、`loop_recipe_physicalizer/**`)、`canonical_ssa/session.rs`、`nested_predicate_*`、`direct_accum_*`、package 側の `source_loop_physical.rs`。
- 使われていない所: 所有を持つ call と分岐の lowering、`ops/logical_shortcircuit.rs`(`builder.insert_phi` を直接呼ぶ。`:276`)。

## 4. `&&` と `||`

### 4.1 条件の位置(今ある物)

- 受理: `qualify_bool_condition_v1`(`home_bool_condition.rs`)。根が `IfCondition` の式だけを受ける(`ExpressionPlanRootV1::IfCondition`、`resolved_value_profile/expression_plan.rs:43`)。`&&` / `||` の節は `lhs` が `Always`、`rhs` が `When{condition: lhs, value: (And なら true)}` の reach である事と、節の class が Bool である事を確かめ、`logical` に積む(`home_bool_condition.rs:255-270`)。それ以外の節は全 child が `Always` でなければ受けない(同 `:242`)。
- 計画: `CheckedBoolConditionFrameV1`(同 `:19`)。`logical_nodes()` を持つ。
- emit: `BoolConditionEmissionV1`(`builder/ordinary_new_admission/selected/bool_condition.rs:15`)が **既存の短絡 writer の観測**を取る。writer 本体は `ops/logical_shortcircuit.rs` の `build_logical_shortcircuit_after_lhs_recorded_v1`(`:130`)。構造は `skip` / `eval_rhs` / `rhs_true` / `rhs_false` / `merge` の 5 block、merge の PHI は 3 入力(skip、rhs_true、rhs_false)で Bool、変数は `merge_modified_vars_multi` で合流させる。LHS は `@sc_lhs` に pin。
- 検証: `bool_condition_control.rs`(package 側)の `validate_logical`(`:8`)が分岐の形を照合し、`expected_inventory`(`:345`)と `expected_fault_inventory_v1`(`:606`)が命令の総数と Fault の総数を数える。
- **右辺の呼び出しの End の扱い**: 条件の位置では、RHS の call は `When` の reach を持つ(`home_new_prefix_condition.rs:74` は「short-circuit の call は既存の制御の持ち主が持つ」と書く)。一時値の End は `pending_operations`(builder、`cleanup.rs:4`)の線形の列(`temporary_order` の逆順 + homes)に載るので、**RHS だけが取得した一時値が、skip の道でも End 対象になる形にはなっていません**。今の受理は、比較の入力が owned Text でない、または一時値を作らない形に限られると読めます(`bool_condition_temporary_operations_v1`、`cleanup.rs:36-` が `owned_result_nodes` と照合する)。**未確認**: RHS が一時値を取る条件の実際の emit を build して確かめてはいません。

### 4.2 値の位置が止まる理由

`return x > 1 && y < 3` は `static-execution/completion-not-ready`(`direct_call_static_execution.rs:98`、`status-index.md:425`)で止まります。読んで確かめた原因は次の連鎖です。

1. `completion_ready` は `completion.returns_value()` と `root_flow().all_exits_ready()` を要求する(`direct_call_lifecycle.rs:348-353`)。
2. 戻り値の式の root exit は、走査の終端(`home_new_prefix_terminal.rs`)が出す。値を持つ式の根の qualify は `children_always_known_v1()` を要求し(`home_terminal_returned_value.rs:394`)、`&&` の `rhs` は `When` なので落ちる。
3. 同じ門が、実引数(`home_argument_expression.rs:190`)、再代入(`home_binding_rebind_expression.rs:209`)、print(`home_print_expression.rs:92`)、初期化(`home_initializer_expression.rs:242`)、call 内の walk(`home_new_prefix_condition.rs:83`、`home_local_call_borrowed_actuals.rs:410`)にある。**7 か所**。
4. `When` を受ける qualify は `qualify_bool_condition_v1` だけで、根が `IfCondition` / `LoopCondition` に限られる。

### 4.3 値の位置に足りない物

読んだ範囲での整理です。**短絡の CFG と Bool の PHI を書く writer は既にあります**(4.1)。足りないのは次です。

1. **根の種類**: `IfCondition` / `LoopCondition` 以外の根(戻り値、初期化、実引数、再代入、print)が `When` の reach を持つ frame を作れない(7 か所の門)。
2. **frame の型**: `CheckedBoolConditionFrameV1` は条件専用。値を返す枝と合流を持つ frame が無い。
3. **道ごとの Fault の後片付け**: `pending_operations` は一時値の線形の列。RHS が取得した一時値は RHS の道の中で End され、LHS の一時値は skip と RHS の両方の道で End される必要がある(または分岐の前に End する)。この「道ごとの列」を作る型が無い。`fault_indices_after_normal_step_v2` は線形の列を仮定する。
4. **検証**: package 側 `bool_condition_control.rs` は条件の census 専用。値の consumer(合流した Bool を使う側)の census が無い。独立 verifier(`ownership_ssa`)は edge 引数を持つ関数を対象外にしている(status-index.md:391)ので、Bool の PHI だけの関数が対象内かは **未確認**。
5. **CFG と PHI の持ち主**: writer は `insert_phi` と `update_cfg()` を直接使う。S3b の「seal を先に閉じない」を守るなら、merge block の predecessor(skip、rhs_true、rhs_false)の証人と PHI を `CanonicalCfgSessionV1` と `PhiTxn` に渡す必要がある。ただし Fault の着地は `ReturnFault` で終わり merge には戻らないので、predecessor の集合は 3 つで確定する(読んだ範囲の判断)。
6. 条件と値で writer を 1 つにする事: 既存の writer は結果の Bool を返すので、条件の位置の writer をそのまま値の位置に使える形です。branch の `Branch` terminator を作るか、PHI の結果を返すかの違いだけが残ります(条件の位置でも `build_logical_shortcircuit_after_lhs_recorded_v1` は値を作ってから Branch します)。

## 5. 分岐の合流の規則(仕様の位置)

- `docs/reference/language/ownership.md` §7「CFG Home Flow」(`:1152-1175`): 片方の枝で consume した Home は、合流の後 maybe available なので **拒否**。診断は consume した枝を名指す。loop は backedge に消費済みが届くなら拒否(同 `:1170-1175`)。診断の理由の名前は `home-maybe-consumed-after-branch`、`home-consumed-on-loop-backedge`(同 `:1340-1341`)。
- `ADR-002` 62 行目: loop は各 backedge に次世代の責任が 1 つ届く事を検証し、**live と empty が実際に合流する場合だけ** drop flag(PHI の発行元は `PhiTxn`)。84 行目: maybe 状態は CFG の必要がある場合だけ drop flag にし、loop の世代と Fault の前後を区別する。
- コード側の名前は `HomeFlowBranchDivergent`(`home_new_prefix.rs:101`、`home_map_flow.rs:177`)。`home-maybe-consumed-after-branch` と `home-consumed-on-loop-backedge` は **src に 0 件**。

結論: 片方の枝だけ move した時は **拒否**が仕様です(drop flag と合流 End は仕様上 If の合流では要りません)。今の挙動は仕様どおりで、変えるのは理由の名前と、枝を名指す事です。

## 6. 数

| 物差し | 数 |
|---|---|
| 所有の状態を決める所(書く) | **25**(1.2 の #1–25。loop は 1 と数えた) |
| 所有の状態を読む所(使用後 move) | **9**(R1–R9) |
| 呼ばれる側で `take` を決める所 | **1**(C1) |
| 借用の状態を作る所 / 無効にする所 | 12 / 3(参考) |
| 形ごとの観測の腕(1 つの形のためだけにある物) | **27**(`scan_statement_flow` に 13 腕、`local_statement::observe` に 14 腕)。うち 1.2 の表に現れる(所有の状態を書く)腕は **18**(loop と if の合流を含む。`scan.rs` 8、`local_statement.rs` 10) |
| Fault の後片付けを決める所(builder 側) | **30**(18 file) |
| 同(package 側の再導出と照合の呼び出し) | 21 か所(10 file)+ `expected_inventory` 系 2 |
| End を書く部品 | **22**(形を知らない関数 12、書き込み口の実装 3、形を知っていて決めと混ざる組立て 7) |
| `RootGraphWriterPortV1` の実装 / `CanonicalCfgSessionV1` を通る物 | 3 / 1 |
| `CanonicalCfgSessionV1` を使う所有 lowering | **0** |
| `PhiTxn` を使う所有 lowering | **0** |
| 値の位置で `When` を拒む門 | 7 |

数え方: 「腕」は `scan.rs` の `if matches!(statement.node(), ..)` と `observe*` 呼び出し、`local_statement.rs` の `else if` の連鎖を、読んで 1 つずつ数えた物です(`scan.rs`: terminal、if、print、binding rebind、field install、named array append、method-call の文(take を commit)、field write、text append、field call、static search loop、source loop、descendant maps。`local_statement.rs`: share fresh、share clone、share move、move、field get、mixed call、routed call(received を含む)、selected new、map literal、named array 初期化、initializer expression、field call、field read、scalar expression)。数え方が違えば数も変わりますが、傾向(形ごとに状態を書く腕が 18 ある)は変わりません。

## 7. 見つけた重複(同じ状態を 2 か所以上で決めている組)

| # | 組 | 場所 |
|---|---|---|
| D1 | Normal 辺での実引数の移転(`homes.retain` + `consume_argument_home`)と、その `prior_homes`(逆順)を持つ prefix の構築 | `home_new_prefix_new_local.rs:79-103` と `home_new_prefix_terminal.rs:626-640`。prefix の構築は 3 つ目が `home_new_prefix_local_statement.rs:163-167`(`ResultNewHomePrefixV1`) |
| D2 | `consume_home` + `homes.retain` の 2 行 | `home_new_prefix_owned_field_write.rs:75-76`、`home_map_value_flow.rs:139-141`、`home_new_prefix_terminal.rs:668-669`(3 か所) |
| D3 | 「元と先が同じ scope か + `proves_original_move`」 | `home_new_prefix_local_statement.rs:201-207`、`:240-249`、`home_prefix_local_flow_shared.rs:102`(3 か所) |
| D4 | 同名の `transfer_home_v1` が 2 実装。片方は `Moved{origin,binding}` + `MovedOut`、もう片方は `Consumed` + 外す | `home_call_entry_transfer.rs:35`、`home_map_value_flow.rs:127`(呼び元 `home_map_flow.rs:681`、`home_map_value_flow.rs:282`) |
| D5 | `move` 演算子の解釈(`expression_source().moved(..)`) | resolved_semantics に 4 か所(`local_statement.rs:180`、`call_entry_transfer.rs:79`、`new_local.rs:184`、`home_local_call_borrowed_actuals.rs:99`)、package に 5 か所(`ordinary_new_borrowed_formal_actual_seed.rs:359`、`call_argument_source.rs:59`、`ordinary_new_lexical_instance_call_provenance.rs:415`、`ordinary_new_borrowed_formal_new_field_source.rs:273`、`instance_construction.rs:516`) |
| D6 | 使用後 move の判定が 3 つの粒度。文単位の掃き出し(R1)、式の中の評価順(R2)、点の確認(R5、R6) | 1.3 |
| D7 | 状態が `homes` と `locals` の 2 つに分かれ、各書く所が両方を手で更新する(同期の規則が型に無い) | 1.2 の #1–#23 全部 |
| D8 | Fault の添字を手で作る所(2.3) | `root_graph.rs:425-427,504-509`、`expression_value.rs:368,457,747`、`arguments.rs:600-601` |
| D9 | 同名で別物: builder の `pending_operations`(列を作る)と package の `pending_operations`(CFG を歩く) | `expression_value/cleanup.rs:4`、`root_cleanup_order_paths.rs:220` |

D1–D6 が「同じ状態を 2 か所以上で決めている組」で、**6 組**です。D7–D9 は構造の観測です。

## 8. file の大きさ(S3b で触る可能性が高い物)

800 行を超えている file は増やさない規則です(`RULES.md`)。

| file | 行数 |
|---|---|
| `home_prefix_local_flow.rs` | 1,220(すでに超過) |
| `home_expression_plan.rs` | 1,246(すでに超過) |
| `home_new_prefix_terminal.rs` | 800 |
| `home_new_prefix_branch.rs` | 783 |
| `home_map_flow.rs` | 783 |
| `home_prefix_source_loop.rs` | 796 |
| `home_terminal_returned_value.rs` | 786 |
| `selected/expression_value.rs` | 777 |
| `selected.rs` | 733 |

## 9. S3b の最初の責務の候補(最大 3 つ。決めるのはユーザーと Opus)

読んだ事実から、小さい順に並べました。順序の推奨ではありません。

### 候補 1: 所有の状態の遷移を 1 つの型にする

- **移す物**: `homes` と `locals` の所有に関わる部分(`Home` / `Shared` / `MovedOut` / `Consumed` / 移転待ち)を 1 つの型(状態の領域)にまとめ、書く所(#1–#23)が `install`、`move_out(edge)`、`consume`、`transfer` の method だけを呼ぶ形にする。Normal 辺と Fault 辺で何が残るか(`prior_homes` と `normal_homes`)をこの型が返す。線形の走査はこの型を呼ぶだけで、まだ CFG の解析ではない。**後で CFG の dataflow が使う「領域」と「遷移関数」の土台**になる。
- **消せる旧判定**: D1(prefix の構築 3 つ)、D2(2 行 × 3)、D4(同名の 2 実装)、`consume_home` / `consume_argument_home` の別々の入口、`was_moved_out` / `has_moved_out` の直接参照の一部(R5、R6)。
- **触る file**: `home_call_entry_transfer.rs`、`home_prefix_local_flow.rs`(1,220 行。分割が先)、`home_prefix_local_flow_shared.rs`、`home_new_prefix_new_local.rs`、`home_new_prefix_terminal.rs`(800)、`home_new_prefix_local_statement.rs`、`home_new_prefix_owned_field_write.rs`、`home_map_value_flow.rs`、`home_map_flow.rs`。
- **危ない所**: (1) `MovedOut` と `Consumed` は `observe` では同じ(`None`)だが、`was_moved_out` は `MovedOut` だけ拾う。この違いは診断(`UseAfterMove`)に出ているので、1 つにまとめると診断の理由が変わりうる。(2) `prior_homes` は逆順で、Fault の列の順を決める。型に移す時に順序を保つ。(3) `home_prefix_local_flow.rs` が 1,220 行で、増やせない。(4) 借用と Text の状態(`StoredLocal` の残り)は別の責務に残す。

### 候補 2: Fault の後片付けの計画を 1 つの型で作る

- **移す物**: Normal の End 列、Fault の列、各 Normal 後に残る Fault の添字、acquisition の添字、displaced を、1 つの計画型(決める所)として 1 か所で作る。書く部品(2.1 の 12 関数)は計画を受けるだけにする。`pending_operations` と `fault_indices_after_normal_step_v2`、`displaced_replacement_fault_sequences_v2` をこの型の method にする。
- **消せる旧判定**: D8(添字を手で作る 4 か所)、`prepare_operation_cleanup_v1` と `emit_root_graph_with_arguments_v1` と `finish_body_cleanup` と `finish_new_arguments_v1` の中の組立て(2.1 の「混ざる」7 のうち 4)、`cleanup_chain` を使う 15 か所の列の組立て。
- **触る file**: `selected/root_graph.rs`、`selected/expression_value.rs`、`selected/expression_value/cleanup.rs`、`selected/arguments.rs`、`selected/terminal_call.rs` と `terminal_call/lexical_i64.rs`、`root_home_cleanup_order.rs`(package 側)。
- **危ない所**: (1) package 側の検証(21 か所)が同じ列を再導出している。計画型を共有して検証を弱めてはならない(生成と検証が同じ型を読むと独立でなくなる)。検証側は emit された CFG と source だけから期待を作る形を保つ。(2) `selected.rs`(733)、`expression_value.rs`(777)は 800 に近い。(3) `RootGraphWriterPortV1` の 3 実装の挙動(Raw と Canonical)を変えない。(4) 候補 1 の「Fault で残る責任」を受けるので、候補 1 より後が自然だが、独立にも進められる(入力は今の `prior_homes` と `unwind`)。

### 候補 3: 値の位置の `&&` `||`

- **移す物**: 4.3 の 1–5。根の種類を `When` を受ける形に広げ、道ごとの Fault の後片付けの型を足し、merge block の predecessor と PHI を `CanonicalCfgSessionV1` と `PhiTxn` に渡す。writer は既存の `logical_shortcircuit.rs` を条件と値で 1 つにする。
- **消せる旧判定**: 7 か所の `children_always_known_v1()` の門のうち `&&` `||` のためだけの分、`qualify_bool_condition_v1` の根の制限。
- **触る file**: `home_bool_condition.rs`、`home_terminal_returned_value.rs`、`home_argument_expression.rs`、`home_initializer_expression.rs` ほか門のある 7 file、`ops/logical_shortcircuit.rs`、`selected/bool_condition.rs`、`bool_condition_control.rs`。
- **危ない所**: (1) 候補 2 の「道ごとの列」が前提。(2) 受理を広げるので、規則を先に文で書く(`worker-common-rules.md` の「形を当てる実装をしない」)。(3) 独立 verifier が Bool の PHI を持つ関数を対象にするかは未確認。(4) 仕様は短絡(`EBNF.md` の Short-circuit と C197)で決まっており、意味の変更は要らない。

## 10. 未確認と読んでいない所

- loop の所有(`home_prefix_source_loop*.rs` 一式、3,997 行)は読んでいません。S4 の範囲として、1 行(#25)に置いただけです。
- issuer 側の借用(`ordinary_new_borrowed_formal_*`)と `source_take_homes_v1` の中身(`call_argument_source.rs:240-`)は、入口だけ読みました。
- `ownership_ssa/published_homes.rs`(独立 verifier)は、`status-index.md:391` の記述を引用しただけで、読んでいません。
- 4.1 の「RHS が一時値を取る条件」の実際の emit は、build して確かめていません。
- build も test も流していません。数字は `grep` と読解によるものです。
