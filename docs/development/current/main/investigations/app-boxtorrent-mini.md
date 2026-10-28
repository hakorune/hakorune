# APP-boxtorrent-mini

Status: selected; design reviewed, implementation incomplete. Date: 2026-10-27
Scope: unchanged boxtorrent-mini, canonical EXE, expected output and exit.
Refs: [RULES](../../../RULES.md), [CURRENT_STATE](../CURRENT_STATE.toml), [Home contract](../../../../reference/language/ownership.md), [Map contract](../../../../reference/runtime/runtime-data-dispatch.md).

## 現在地
- 2026-10-28 JSON着地後のtree（`claude/json-substring-20261028`）では、MIR compileはsemantic packageの段（基準binaryは slot 57 `original-return/source-unproven` で停止）を通過し、後段の `[freeze:contract][callable-scalar/return-type-mismatch]` `BoxTorrentStore.allocatorBlocks/0`（slot 56、expected Integer、返り値のclassがUnknown）で止まる。以下のowner38記述より先にここを確認する。
  - 解決済み（`allocatorBlocks`、slot 56）: stored-child receiverのreturn callは、`issue_borrowed_i64_terminal_call`が関係を作り保存していたが、`retain_child_terminal_relations_v1`の述語が`Value(Handle)`・静的call・ループ早期returnしか残さず、emission時のlookupで消えていた（`[terminal-lookup] miss`）。scalar結果のloanを持つCall関係を、calleeのscalar証拠が検証済みなら残す条項を足した（`ordinary_new_coseal_issue.rs`）。後段が既に要求する`scalar-result-source-missing`と同じ条件。
  - 現在の停止（`chunkCount`、slot 55）: `return me.chunks.keys().length()`。builderは`keys`/`length`を動的`RuntimeDataBox` Callで発行し（`me.chunks`にorigin無し、`value_origin_newbox`はConstructor結果のみ）、dstはUnknown。packageはcore ArrayLen(MapKeys)のscalar Integerを公開済みなので`lowering.rs`の`return-type-mismatch`になる。terminal側は`[terminal-call-issue]`「source scalar結果なし」「lexical i64引数rowなし」、`[terminal-relation]`は`ReturnValueNotCovered`。
  - **意味契約の分岐（決定済み、実装は未着手）**: (1) 型付きCore laneをbuilderに作る（採用）。(2) packageが公開を控える（却下: callerが動的結果になり`DynamicAdd`の物理契約が未active）。(3) 名前から型を付ける（却下: `lookup_core_method_result_row_v2`は「selector lookupではない、意味ownerがsourceを照合してから使う」と注記、読取りworker案も同じ理由で不採用）。
  - 実測した不足（2026-10-28）: returnの評価は`observe_scalar_return_expression_v1`（`Subtract`二項式のみ）→`prepare_value_evaluation_v1`（`prepare_expression_calls_v1`は`ExpressionLeafProfileV1::Print`前提）。`return`のmethod callまでゲートを広げても全て「評価を準備できない」で未選択に戻り（既存laneは壊れない）、`chunkCount`は通らない。原因: `me.chunks.keys()`は**新規ArrayBoxのhandle**を返す中間結果で、`.length()`が消費した後に解放（End）が要る。評価器にhandle値の一時中間結果とそのEnd義務を扱う葉が無い。
  - 次（大）: handle値のCore callの一時結果（`MapKeys`）を評価leafとして受け、`ArrayLen`が消費した後のEnd（Normal/Fault両方）を既存のDynamic carrier/Array Home契約に接続する。`keys`のC側emit（`same_module_generic_method_collection_emit`）の戻りhandleの所有規則を先に確認する。.hakoの書換えで避けない。
  - 解決済み（Array範囲外Null）: `CheckedArrayGetBoundsSourceV1`をenum化し、`loop(i < n)`の`ids.get(i)`を範囲内と証明するLoop variantを追加（`ordinary_new_field_array_read_bounds.rs`）。条件: `n`は再代入なしの同一arrayの`length()`でloop前、`i`は非負literal初期化で全代入が同loop本体内の`i = i + 1`かつ`get`より後、入れ子loop/lambda/tryなし。縮小/漏れは、グラフが要素SUMを持つ条件（`full_batch_write_fields_v1`のBox全体census: push/set/get/length/hasのみ、漏れなし）が担う。`cid`のSUMは`{Text, Null}`から`{Text}`へ。既存の否定テスト群は不変、`counted_loop_certifies_only_the_exact_index_read`で正1・否定5を固定。 次の停止（実測）: slot 61（`seal()`の`me.root_id = ContentHash.digest(me.chunkListText())`）の`borrowed-static/text-expression-execution-pending`。引数の`me.chunkListText()`が完了済みinstance Text結果にならない（`[ordinary-result] none`: calleeの完了結果が照合できない）のは、callee（slot 65）のloop `Body(4)` `out = out + ids.get(i)`がHome prefixで覆われない（`PrefixNotCovered`）ため。原因（実測）: ループrecipeは配列の動的`get`の結果を3層で`Dynamic`値として扱う: 実行契約（`source_loop_execution.rs:162`）、スケジュール（`source_loop_schedule.rs`の`array_dynamic_core_slot_matches`が`value.class == Dynamic`を要求）、値の分類。一方、範囲内の証明で要素SUMが`{Text}`になると、plan側の`result_class()`（`core_method_source/ordinary.rs:292`）がnodeを`String`にし、契約が`execution-core-ownership`で弾く。契約だけ`String`を許しても次にスケジュール（`schedule-frame-call-slot` exit 9）で弾かれることを実験で確認（変更は戻した）。方向: loopの値は`Dynamic`のまま、`out + <Dynamic>`を既存の`String + DynamicCarrier`（suffix借用の足し算、`invoke_string.rs`のAdd、`runtime/dynamic_invocation.rs`の`string_add_suffix_v1`）に落とす。consumerのTextAdd検査（`source_loop_schedule.rs`の`LoopOperationV2::TextAdd`）がDynamicの右辺を受けるか、`out + get`の評価がDynamicAddのどのfamilyになるかを次に確認する。finite goalの「Loop」領域。
- 最新実行は `HakoAllocHeap.release`: `borrowed-entry/original-input-not-ready`。入力 cohort 未完、MIR 構築で停止し EXE 未生成。
- 上流の `BoxTorrentManifest.releaseFrom` / `materialize` は `ids = me.chunk_ids; n = ids.length(); loop(i < n)` 内で `ids.get(i)` を Text key の受け手へ渡す。元の element SUM は Text だが、範囲外 Null が残る。 現行の範囲証明は定数添字と直前の終了 guard 用。Loop の全 backedge、同じ Array、非縮小、支持 Home の生存をまだ結んでいない。
- 固定 suite は 10/11（合計11、SKIP/timeoutなし、JSON着地後の2026-10-28）。失敗はboxtorrent-miniのみ。mimalloc-lite・allocator-stress・binary-trees・JSONはPASS。

## 採用設計
- **既存仕様を維持して実装する。null-key の仕様回答はこのアプリの着手条件ではない。** 一般 Dynamic→Map の null 受理は別の未決事項であり、Text-only key の現在の経路へ勝手に stringify / TypeError を導入しない。
- **証明を三つに分ける:** (a) 同じ Array の添字範囲、(b) get 時点までの holder/field/Home 生存、(c) get 後の独立 carrier と OwnedText の取得・使用・End。範囲証明を所有権証明の代用にしない。
- 全 heap / 全 Array の payload graph 完成を先行条件にしない。対象の支持経路と実際の呼び出し区間に必要な effect だけを既存 source owner で閉じる。閉じない境界は未証明として残す。
- 実際の release 連鎖は Store → ContentChunk / Heap → Page。配列操作 get/set は長さを減らさず、set は置換または末尾追加。ただし selector 名だけで許可せず、元 call/target と core 契約を照合する。非縮小でも element の型・値・生存は変わり得る。
- 通常引数の noescape/call-only borrow 契約と既存の root Home を使う。entry loan だけでは完全な call-site ABI も生存期間も証明できない。隠れた retain/share、class 名の違いによる非 alias 推測、`Map.delete` の無条件許可はしない。
- checked ArrayGet は読んだ値を独立 carrier に保持し、Text materialize は独立 OwnedText を取得する。Text key の使用終了まで元 Array slot の借用を延ばさない。generic get 非対応の OwnedSlots にこの経路を流用しない。
- Map の local borrow は **get Normal → 実際の最後の借用 use** に限定する。関数全体の call/write 禁止を source identity と区間検証に分け、use 後の delete を過去の安全な read の拒否理由にしない。use 前・途中の置換/End/escape は拒否する。

```text
original resolver + source value flow + existing Home/call contracts
  -> Array identity / loop range / interval effects and live support
  -> source SUM refinement before first Core receipt issuance
  -> existing ArrayGet carrier -> existing Text snapshot -> existing Map key input
  -> Map local get/nonnull -> chunk.release -> nullable child argument
  -> independent Rust/C final validation -> unchanged EXE
```

## 実装タスク（同じ APP row 内）

先に下記のMap借用整理seriesを実施し、完了後に1〜5の機能接続へ戻る。
1〜3は一本の縦の実装series。1で元sourceの支持条件と後段照合義務を構成し、2はそのsource条件だけを使う。実命令の独立照合は3のlower後に閉じ、物理PASSを2の着手条件へ戻さない。

1. [ ] **支持 Home と呼び出し区間を既存 source に接続する。**
   `home_prefix_field_alias.rs` の `OwnedHome` / `InstanceEntry` 区別と、`ordinary_new_field_array_read_source.rs` の field/alias 正本を再利用。count/get が参照する同じ residence と caller の支持 Home を保持する。
   resolver の全 call observation を対象に、既存 incoming/target 情報を照合して実際の call closure を閉じる。incoming 成功行だけでは全 call の網羅にならない。未解決 direct/method、捕捉、外部効果、支持 root/field の置換・End を無視しない。
   effect は「長さを縮めない」と「支持を無効化しない」を別に判定する。`Map.delete` は payload の破棄を含み、支持経路への影響が不明なら後者は不成立。再帰/SCCは影響の閉鎖を別途確認し、未閉鎖なら未証明。SUMの収束をeffect証明に流用しない。
   完了条件: 元の実 call/引数/receiver と対応する支持が、get までの全経路と caller Normal/Fault で保持され、独立した最終 use 検証にも渡ること。source-only の未使用 receipt を成果にしない。
2. [ ] **Loop 範囲証明を初回 Core 発行前に接続する（1に依存）。**
   `source_value_flow` の LoopEntry/Backedge/Continue 全 predecessor を使い、i の初期0・同一 binding の更新・条件 i<n・n の同じ ArrayLen・再束縛不在を確認。非縮小と長さの非負/i64範囲から overflow も除外する。
   `ordinary_new_field_array_read_bounds.rs` を既存 source proof 内で拡張する。`ordinary_new_borrowed_formal_source_seed_preparation.rs` の初回 Core 発行前に、同じ graph へ証明を反映して必要な SUM/SCC を再解決する。
   証明の前提は OOB を残した元 source 情報で成立させる。自分の Text-only 結論を先に仮定して call/lifetime を証明しない。target identity は元 receiver/宣言/call から照合し、Text-only 引数の受理や完成済み入力・実行可能packetを前提にしない。未確定targetは未証明。既発行 Core receipt を後段で差し替えない。
   完了条件: 実アプリの get で OOB 由来の Null/strict Text だけが不要となり、element 本来の Null は残る。別配列、縮小、失効、未知 call、未完 predecessor では絞り込まない。
3. [ ] **既存 Text snapshot → key の不足接続だけを閉じる（2に依存）。**
   `home_expression_evaluation.rs::text_snapshot_root_v1` → `expression_value.rs::materialize_text_root` → 既存 Map key/ordinary argument consumer を通す。新しい materializer、専用 producer、並列台帳は作らない。
   完了条件: source Text-only と元取得を `root_call_entry/expression_projection.rs` が照合し、receiver-first/引数評価順、carrier と OwnedText 各一回の End、取得失敗時の out 不変と Normal/Fault cleanup が Rust/C の実命令でも一致する。
4. [ ] **Map local borrow を child 引数の最後の use まで接続する（3の key 接続後）。**
   `home_map_value_flow.rs::MapFieldGetLocalV1` の関数全体 veto を、元 get の identity と use-scoped lifetime に分離。`ordinary_new_lexical_instance_call_provenance.rs::borrowed_receiver_interval_v1` の既存 get→chunk.release 証拠を再利用する。
   `chunk.alloc_handle` は親が nonnull でも nullable。元 get/guard/field read/outer-call argument ordinal と支持 Map slot を同じ論理計画で保持し、allocator.release の呼び出し完了まで子の借用を守る。ReturnedField や owned Home へ付け替えない。
   `ordinary_new_local_commit/field_read_argument_interval.rs` と C `lifecycle_v4_field_interval.inc` に同じ区間/実 call 対応を独立検証させる。body index だけで nested argument の終端を決めず、途中の同じ slot の置換/delete/End・未知 call は拒否する。
   完了条件: use 後の `me.chunks.delete(cid)` を許し、use 前/途中なら拒否。Normal/Fault と支配関係を保つ。今回不要な Map-slot 関数外 Return ABI 拡張を前提にしない。
5. [ ] **未変更アプリを EXE/run まで通し、実際に残る依存だけを解く。**
   未完の free_stack/use_counts、integer ArrayGet の source/physical 不一致、NoIncoming formal は記録を再利用する。call closure に不要な全 allocator graph を先に閉じないが、選択 consumer が必要とする境界を省略もしない。
   完了条件: 下記 smoke 全契約と期待 stdout/exit が PASS。続けて固定 suite、必要な shared regression、局所退役を確認して対象変更を commit。JSON と有限ゴール全体の後続条件は残る。

## 次に実施するMap借用整理series
- MapLocalは元getを生成時に検証し、不変payloadの後段再走査を退役。別packageの同名field拒否は未検証の独立境界だったため既存composition caseへ最小追加。CLI・focused PASS、smokeは同じowner38停止。
- Map payload/receiverの同一Birth provider判定をfield-sourceへ統合し、旧2判定を退役。共有cache試作は17 files・production純増68行で未完、採用せず戻した。query共有と区間統合は残る。
- Map payloadはresult facts構築・lexical・Homeで繰り返し発行される。既存field-source ownerにco-seal単位でsource/selection/constructorを束縛し、要求時に共通化する。各get・entry・intervalは別照合。effect一覧とcall/newの一般対応は未sealなので走査削除の根拠にしない。
- lexical preflightはMap frame完成より先。完成frameの逆流は循環し、全関数vetoとget→call区間の受理条件も異なる。共通化は両者より前のresolver観測から行い、site区間と別calleeの判定を残す。
- 既存Map familyに最小caseを追加: 取得frame登録と実consumer lookupは既存caseで直接検証されず、再走査除去の独立境界に必要。
- probeと実walkのMap/NamedArray/借用引数dispatchを共通ownerへ統合し、実walkだけのArray append例外を保持。既存Map21・NamedArray5・borrowed callback1・direct Map1 PASS、CLI build PASS。例外を直接踏む独立testは未確認。
- 途中: 唯一initializer照合を共通化。Map get frameのsource/全体vetoを登録時に一度検証し、null判定とfield readの再走査を削除。元getの同site照合も1走査に統合。既存Map family 21/0、direct receiver・physical interval各1/0、CLI build PASS。smokeは同じowner38停止。lexical intervalとの統合は未完。
- [x] `DirectMapReceiverSourceV1::matches_original_v1` の再発行（initializer/ledger/Arc再走査）を退役し、mint時のadmission＋owner照合へ統合。`stored_map_get_keeps_its_original_local_source` と `direct_map_receiver_numeric_writes_close_the_original_callee_effects` PASS、boxtorrent smokeは同じowner38停止（受理/拒否の差分なし）。`main_direct_...`は既存のcallee-exit赤（同test内208行）で、照合assert（152行）ではない。
- [x] 元get/支持slot/区間内effectの保持とconsumer切替は上記6 commit＋今回の退役で完了。caller区間(`[acquisition..end]`)と callee全体vetoは受理条件が異なるため統合せず、それぞれ既存ownerに残す（統合しない判断をここに固定）。
- [x] 置換済みの再走査・再発行を削除: `matches_input`の重複呼出し元get site照合、`DirectMapReceiverSourceV1`の再発行、probe/実walkのdispatch二重化。`binding_is_unrebound`はresult-class ownerへ移動。独立Rust/C physical検証は保持。
- [x] 検証: `map_home_flow_tests` 21/0、`map_get_terminal` 13/0、`field_argument_interval` 1/0、`stored_map_get_keeps_its_original_local_source` 1/0、`direct_map_receiver_numeric_writes...` 1/0、CLI build PASS、boxtorrent smokeは同じowner38停止（受理/拒否の差分なし）。`main_direct_...`は既存callee-exit赤（208行）。
- [x] series全体のproduction純減: `d2a32dd08c..1a19885659` の src は production +111/−157（**net −46**）、test +79（別記）。旧caller切替と削除分岐は上記のとおり。→ **series完了。**
- [ ] 残る任意の重複: `source_call_arguments_v1` の `StoredMapGetSource` はmint済みloanを共有できるが、+10/−4程度で純減に寄与しないため今回は採らない。構造整理queueへ送る（機能追加の前提にしない）。

## owner38停止点の実体（次タスク4の入口）
- `entry_owner_view:306` → `checked_completed_original_instance_target_arguments_v1` が None。内側は owner38 `HakoAllocHeap.release` で `instance_source::reconstruct_instance_source_actuals_v1` が None（`pending_expression` 分岐: 元source slotが未qualify）。owner60/62も同経路、62はcall siteのactuals欠落が併発。
- 未qualifyの実引数は nullable child（`chunk.alloc_handle` の field read / Map local borrow）で、card タスク4の区間接続が次の実装。ここを閉じるまで新機能・別アプリへ進まない。
- [x] Map-held field 実引数を原source read として接続: `ReadActualSourceLoanV1::MapField`（read site + class）、pending gate/closure、executable constructor、`require_executable_v1` の ordinal 会計、class は field writer SUM から（null併存は単一classのみ許可）。owner38 の input cohort は通過。
- これで停止点が前進: `BoxTorrentStore.allocatorBlocks/0` の `callable-scalar/return-type-mismatch`（expected Integer / returns Unknown）。次はこの Integer return 推論。
- 残: map-field read の emission/interval（`EmittedReadArgumentV1` は MapField を kind-drift で拒否）＝タスク4後半。今回の admission だけではEXEに届かない。
- 新停止の切り分け: `allocatorBlocks` は**borrowed walkが走っていない**（`borrowed_terminal_arguments_v1` が owner56 で一度も呼ばれない）。よって zero-arg stored-child の Ready I64 row を terminal 経由で使う案は現状のownerでは到達不能。標準laneの `UnknownBox.outstandingBlocks/0` signature missが直接原因。
- 追加判明(2): owner56 の `verified_walk=true`（`stored_term=true`, `scalar_src=true`）。よって walk 自体は走っている。しかし `raw_ordinary_new_claim/terminal_call.rs` の**最初の分岐** `return_expression_evaluation_v1` が Some を返し、`take_borrowed_lexical_call_for_return_v1` に到達しない（＝expression評価経路が型をUnknownのまま出す）。実測追記: `emit_return_expression_v1` の call emitter（`lexical_i64_expression::emit_with_prefix`）も owner56 では**呼ばれない**。return文dispatchは `build_return_with_port_v1` で、`prepare_root_home_exit` が false の枝に terminal i64 **call** exit の試行が無いことを確認。fallback枝へ `emit_terminal_i64_call_exit` を追加＋zero-arg stored-child の Ready I64 row を terminal 引数として許す2点を一時実装したところ、`emit_terminal_i64_call_exit` への**到達は実測**できたが内部で None のまま（terminal relation/arguments 不在）で停止点は不変。両実装は撤回。`home_new_prefix_terminal` には `terminal_call` predicate（`observe_terminal_statement` の引数）を使う terminal-literal 分岐が既にあり、ここが false のため relation が発行されない。**確定（実測）**: 担当は `source_scalar_call::demand_v1`。owner56 で `demand_v1=Ready` となり `issue_borrowed_i64_terminal_call` が `TerminalI64CallReturnV1::issue_source_scalar` を発行している。`seal_lexical_i64_arguments_at` と `local_lexical_i64_call` は関与しない（seal到達なし・Ready armで return）。よって relation 自体は存在する。
**実測**: HEAD では `emit_terminal_i64_call_exit` 自体が呼ばれず（非root-homeの return 枝に call exit が無い）、fallback を足して呼ばせても `call_source_completion_for_owner_at(owner,[Body(0)])` が None（`scalar=false`）で relation を取得できない。relation は scan 側で `demand_v1=Ready` から発行済みなので、**格納先の写像と emission 側 lookup の不一致**が残件。ここで一旦停止し、suite 9/11 を維持したまま json_stream_aggregator へ移る（ユーザー境界判断）。再開時は scan の `terminal_relations` 挿入箇所と `terminal_relation_for_owner_at` の owner/site キーを照合する。
- 追加判明: owner56 には lexical target row が**既に在る**（call site `[Body(0), Value]`、stored_receiver=true、uses_existing_borrowed_result=true、result cohort も在り）。したがって `has_stored_terminal_v1` の入力側は揃っており、欠けているのは **zero-arg stored-child scalar return の terminal relation が発行されない**点（`borrowed_terminal_arguments_v1` が owner56 で一度も呼ばれない）。次は `home_new_prefix_terminal` 側でこの形に terminal を発行し、既存の borrowed result owner に型を出させる（walk trigger 追加や標準laneの型付けは採らない）。
- 直近suite: **9/11**（`--timeout 420`、mimalloc PASS 242s、FAILは json / boxtorrent、SKIP/timeoutなし）。

## 検証・整理
- 既存 `ordinary_new_borrowed_formal_result_composition_tests.rs` の count guard/OOB/別配列/途中 call、`container_field_uses.rs` の全 writer/capture、既存 Map/field interval のケースを再利用する。
- Map は既存 `direct_map_receiver_numeric_writes_close_the_original_callee_effects`、`field_argument_interval_rejects_early_end_mutation_and_transfer_but_allows_terminating_fault`、C の `published_lifecycle_v4_borrowed_source_execution_test.py` / `published_lifecycle_v4_borrowed_ingress_test.py` を再利用し、実際の slot 境界が未網羅なら同 family で補う。
- kernel `fault_tests.rs::checked_dynamic_invocation_publishes_and_ends_one_opaque_carrier` は非Text materialize の TypeError/out不変と独立 Text/End を既に検証。raw `slot_load_hi` の C get-view 試験を Dynamic/Text の証拠に流用しない。
- 新しい loop/区間境界に既存ケースが足りなければ同 family の最小追加だけ。新規 testファイル・checker・guard・card は増やさない。置換した専有判定/一時adapter は同じ実装seriesで退役する。
- 選択 smoke: `NYASH_BIN="$PWD/target/quick/hakorune" NYASH_EMIT_EXE_NYRT="$PWD/target/quick" bash tools/smokes/v2/profiles/integration/apps/boxtorrent_mini_exe.sh`。
- 既存 targeted check → 同じ binary/runtime の選択 smoke → 着地/広い共有変更時の `real-apps-exe-boundary --timeout 420`。Cargo は一つ、jobs4。今回の docs-only 整理では build/test を起動しない。
- 前回 focused 検証には source-only declared cohort、PageHeap required-integer、JSON callee-exit、physical/Text/terminal 等の未解消赤がある。`borrowed_formal_entry` は63 PASS/3既存FAIL。これらを今回の成功として扱わず、触った境界の差分を確認する。
- ArrayRead OOB の Null/strict Text と v4 raw `slot_load_hi` の0は同じ意味ではない。IntegerBox の unbox や InlineI64 名だけで primitive source を捏造しない。`reallocResult` の NoIncoming は非到達/型の証明ではない。
- 除去済みの未接続実験、旧 trace と詳細履歴は Git に保持。再調査せず上のタスクを接続する。一般 null-key の設計が必要になるのは、その受理を実際に広げる時だけ。
