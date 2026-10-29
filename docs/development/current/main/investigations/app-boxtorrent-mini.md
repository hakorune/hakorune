# APP-boxtorrent-mini

Status: selected; design reviewed, implementation incomplete. Date: 2026-10-29
Scope: unchanged boxtorrent-mini, canonical EXE, expected output and exit.
Refs: [RULES](../../../RULES.md), [CURRENT_STATE](../CURRENT_STATE.toml), [Home contract](../../../../reference/language/ownership.md), [Map contract](../../../../reference/runtime/runtime-data-dispatch.md).

## 現在地
- 停止引継ぎ（ユーザー依頼）: 未変更appは最終Map借用検証の `field-argument/parent-or-child-invalidated`、`HakoAllocPage.release/1` 内の `ArrayElementWrite(Set)` で停止。別途 `call-result-contract-not-connected` 2件も残る。EXE未達。Cargo/rustc停止確認済み。次は下記Setのsource/実命令照合。再開許可まで実装/buildしない。
- 引継ぎ整理: MapField source→emitted read、get/nonnull/child/引数slotと最終投影、呼出し構造と借用検証の分離、到達callee全体のeffect検査を既存ownerへ接続（8 Rust files、production +263/−26、機能追加であり純減整理の主張なし）。Map mutation/End・未知call・未証明Array writeは拒否を維持。一時 `[map-borrow-effects]` dumpは撤去済み。
- `d04bc06fb3`+`b1ff43dfbe`+`011d3f2462`: Set receipt arm（def mapでfield read/FieldGet/唯一Normal result、互換`FieldGet`は**宣言経由**でcanonical fieldへ解決、整数証拠はexact numeric facts又はfinalized value types）＋既定OFF診断。受理変化なし、累積production **+141**で純増。退役は`BorrowedArraySetSourceLoanV1`(formal-index専用)とNamedArray Push markerのまま。実測: `requested_sizes`はordinal 6へ解決するが、HakoAllocPageの整数Array censusは**[3,4,5]**。= この配列はdynamic writerを持ち整数証明の対象外。次は`array_i64_fields`の整数fixpointへ**untyped parameter writerのsource SUM**を渡すこと。同moduleは`issue_array_i64_fields_v1`がingress生成前に走るためSUMを持たず、plumbingは順序変更（cycle注意）を伴う設計規模。代替の「own boxのArrayBox Setを無条件許可」は記録済み設計（未証明Array write拒否）を覆すため採らない。
- 次手: 下の実装タスク1〜6。要点: integer census は SUM graph を入力にしない（循環ではない）。解けた`{SignedI64}`field集合を別集合でSet armへ渡し、要素クラス証明の省略は採らない（Set receipt armは受理を広げているため証明を絞る）。boxtorrent側は`materialize`入力cohortとnullable textのC shim。
- Array aliasと直接receiverは実Count/Get source loanを同じfield descriptorへ保持。直接receiverもReturnを含むsource field census→既存field issuer→exact親read/Copy→記録済みcanonical readへ接続。Countは既存read会計を使用。source-only拒否pin、最終noninteger aliasとdirect Arrayの拒否を保持。
- 分岐後Textは既存branch Phi/両incomingを検証し、元Home suffixから各incomingのcleanup候補を計算。全候補が一致する場合だけ一つのEndIfOwnedへ統合（順序推測なし）。Text取得prefixは別管理のDynamic carrierを除き、carrierの独立owned-flow検証は保持。既存Loop回帰・String Normal/Fault/escape検証は各1/0。
- 共通化済み: 元result-sourceによるdispatch、同じCall/Normal継続、Loop Null定数会計、Unit入力/暗黙終了、引数snapshot、String/carrier temporary順序、共通expression body、operation cleanup。Text field移転は同じRHS bodyを消費しscalar Copyを通さない。詳細はGit履歴へ圧縮。Array operation/provider・生存の最終gateは未完拒否を保持。
- 今回の接続: 完全な元mixed cohortの構造貸出と全useによる入力選択。prepared receiverは元target全体/receiver/domain/installed entryを照合し、完成側は全actual/実Call/Copyを再検証。返却local Newは元Return plan/唯一initializer/flow/witnessでreadだけを会計し、constructor引数と最終Home移転を別検証に残す。発行済みHome terminalの保持漏れを修正。discard Jumpの不要な入口Fault suffixを退役し、実評価/Normal cleanupのFaultは保持。
- entry loanだけで未知callee中の親生存は証明できない。snapshotとfield再読の同一性、支持Home、Normal/Faultを別々に閉じる。
- 着地済み: C Map null guard→child borrow（空Map/nullable child/Fault/誤分岐）、異なるslot借用の保守的alias検査、nullable slotの実payload null比較。Rust完成guardの到達・支配・再実行検査は完成時だけ行い、構築中の局所検査を維持。
- 既存接続: stored-child scalar return、Map keys count、put/readData/refCount、nullable Text需要、Text連結Loop、Null return、DiscardValue、delete。chunkCount独立validator、nullable Text C transport、Bool物理、全体cleanupとEXE実行は未完。
- 固定suite: 今回の整理後 `target/quick/hakorune`＋同quick runtimeで**10/11、890.32秒**。実EXE9本＋空marker、boxtorrentのみ上記Set拒否と結果契約2件、SKIP/timeoutなし。直前の旧binary10/11とFAIL集合は同じ。全アプリ成功・goal完了ではない。
- 引継ぎ検証: 同じquick CLI/runtimeでbuild36.40秒PASS。既存Map21/0、Map終端13/0、Copy改変拒否・joint MIR/recorded Fault欠落拒否・借用区間各1/0。現binaryの選択smokeは冒頭のSet拒否と結果契約2件を再現（timeout/SKIPではない）。固定suite10/11（890.32秒）、実EXE9本＋空marker、FAILは同じboxtorrentだけ、timeout/SKIPなし。
- 借用区間の安全性修正: String End/EndIfOwnedによる子alias/同field再読の失効を共通intervalで拒否。既存interval caseに2操作だけ追加（従来未検査の破棄境界）。同case1/0、joint MIR/recorded Fault欠落拒否1/0。Map用effect callbackも同じ到達区間を使い、従来consumerは追加effectなし。
- Bool Local: sourceのBool frameを既存評価値・scalar cleanup・exact Copy receipt・最終scalar visitorへ接続。比較の発行は条件式と共通化し、独立検証も同じsource plan/Invoke pairを使用。Text equality local→分岐の一時probeは真exit7/偽exit0、CLI build41.14秒PASS。既存initializer1/8、Text条件1赤は保存済み変更前binaryと同じ失敗集合/forwarded-copy拒否。比較Normal/Fault契約とFault欠落拒否は各1/0。通常callを含む比較、Bool文字列化、primitive比較の一般化は未達。
- 通常call比較: 比較operationの既存source queryを入力scheduleから分離。条件式の制限とMap receiver順序は維持し、完全expressionは既存order/Normal/Faultを使う。`page.read()==page.read()`のBool Local→分岐はEXE exit7、異なるText版はexit0。CLI build47.12秒、既存比較source8/0・比較Normal/Fault1/0・Fault欠落拒否1/0。アプリは上記MIR最終検証へ進展。
- 直接Array Return: 同じoperation loanを持つ唯一のReturn Coreから、完成packet全体・DynamicArrayGet/同carrier projection・field-read結果・sole providerを照合。未変更アプリはowner64の直接取得を越えowner65 aliasへ進展。CLI build44.71秒、local field23/3・descriptor4/1は変更前binaryと同じ失敗名、joint Fault欠落拒否1/0。小probeはこの照合を越えるが既存raw Array length publicationで拒否されEXE未達。別purpose/Count/aliasを一括許可しない。
- Array alias接続: 直接Returnの照合を既存expression bodyへ共通化し、Init/Rebind/PrintとCall引数も同じfield/operation組を貸出。元snapshot＋使用時Copyを照合し、Countは同NamedArray recipe/実read、Getは同Invoke/carrierを必須にする。purpose全体のNormal/Fault・provider最終検証を保持。現appは保持MapFieldの明示的な最終拒否へ進展。CLI39.29秒、local field23/3とdescriptor4/1は変更前と同じ失敗名、Copy改変拒否/最終Fault欠落拒否各1/0。小probeは別のsource境界で拒否され成功扱いにしない。
- 以前の未解消検証: source-use拒否test1赤は変更前commitでも同じ`p.sink(0)` assertion。Null境界は診断assert赤、Text alias赤は変更前後で同じforwarded-copy/root。Core source family8/3は未分類、initializer i64の3赤は既存失敗名集合内。これらを今回の成功に数えない。

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

Map借用・Loop範囲・Text snapshot・delete・Loop null・DiscardValue・{Text,null}静的call・Bool Localは着地済み（履歴参照）。残りは次の6点。担当: A=Array Set系(DeepSeek)、B=boxtorrent系(Claude)。同じworktreeの同時編集を避け、1担当ずつ動かす。
1. [ ] **A: integer array fieldをSUM証明で通す（循環は無い）。** census(`array_i64_fields`)は入力にせず、SUM graph(`graph.array_fields`、書込seedは型なしformalも含む)の解を、要素が`{SignedI64}`だけに閉じたfield集合として`prepare_source_preflight_v1`の外へ出し、ledgerの別集合`array_i64_sum_fields`に保持。Set arm(`field_read_argument.rs`)は census又は この集合のfieldだけ許可。Conflict/書込欠け/別名漏れは従来どおり拒否。案2(要素クラス証明の省略)は採らない。
   完了条件: `requested_sizes`の4書込(`page_heap_box.hako` 69/93/116/174)がSignedI64に閉じるか実測。閉じなければ停止点を記録。他4appの結果が不変。
2. [ ] **A: Set receipt armの証明を絞る（1の前でも可）。** Box名の文字列分解をやめ宣言/instance receiverで決める。index/valueは`exact_numeric_value_facts`のみ(`value_types`は不可)。配列がMap親子と別名でない確認を足す。受理を広げる変更は診断と別commitにする。
3. [ ] **B: `materialize`(slot 66)の入力cohort。** `selected_original_instance_input_kind_v1`(`ordinary_new_borrowed_object_input_finish.rs`)のsource result requestに、Local初期化子call＋calleeが`{Text,null}`完了結果の場合を足し、`[instance-input]`診断で確認。アンカーは古い可能性があるので再確認。
4. [ ] **B: `ordinary_nullable_text`のC shim。** callee返却は`ordinary_text`と同じ`out_text`(null=(0,0))、call後に`nyash.dynamic.checked_publish_text_or_null_v1`でcarrier化。対象`lang/c-abi/shims/published_mir/`のemit/return_emit/indexed_return/indexed_flow/index/instruction_schema/result_transport/physical_v2/indexed_rows。入れたら`nullable-text-native-pending`拒否(`physical_program_json_call_transport.rs`)を外す。
5. [ ] **B: EXE/run。** Bool物理(`StringInvokeOperand`のBool: JSON/C/kernel display)、`chunkCount`独立validator、deleteの実行時確認(欠けたkeyの空Ready解放、outcome_init)、Bool producerのif/else join test。FFI(`tools/build_hako_llvmc_ffi.sh`)とkernel(`cargo quick-build-lifecycle-fast`)を再build。完了条件: 下記smoke全契約と期待stdout/exit。
6. [ ] **着地確認と退役。** 固定suite全PASS(現HEADで再実測)、production行の純増減(Claude src +2395/−548、署名なし +3031/−1259、DeepSeek +433/−53)の集計、退役条件(`BorrowedArraySetSourceLoanV1`、NamedArray Push marker、不要診断`[if-refused]`等)を記録。800行超`home_prefix_source_loop_control.rs`(1201)・`source_loop_schedule.rs`(885)・`root_home.rs`(802)は整理候補。goal完了にはR6/R7・Loop・物理退役・最終pipeline(未着手)が別途必要。

## 完了したMap借用整理series
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

## 完了履歴の参照
- owner38の入力、allocatorBlocksのterminal lookup、旧9/11停止の詳細はこのcardのGit履歴へ圧縮。現在の停止と保持変更は冒頭だけを正本とする。

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
