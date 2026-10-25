# APP-boxtorrent-mini

Status: selected app; Text field result/discard, nested Core keys-array and borrowed receiver completion incomplete. Date: 2026-10-26
Scope: 未変更boxtorrent-miniの正規EXE生成・実行・期待出力/exit。
Related: [RULES](../../../RULES.md)、[現在地](../CURRENT_STATE.toml)、[直前のmimalloc着地](mirbuilder-app-bundle-mimalloc-lite-d0-2026-09-30.md)。

## 現在地
- 明示binary/runtime: `NYASH_BIN=target/quick/hakorune NYASH_EMIT_EXE_NYRT=target/quick`。現在のV4 FFIを使用。
- 現CLIのcanonical停止はowner61 `BoxTorrentManifest.seal` Body1 Value、`borrowed-result/source-not-i64`。先行Body0の`me.root_id = ContentHash.digest(me.chunkListText())`でText置換のprefixが未接続。直接Text field返却の最小sourceは正規EXE/runまで通るが、seal全体とcallerのdiscardは未完。EXE未生成。
- 訂正: keys().length()はowner55でsource I64が閉じていた。旧owner56をchunkCountへ対応付けた記録は誤りで、実際はallocatorBlocksの元field-return plan欠落だった。同じ元計画へ正規化しowner56/57のsource I64照合を確認。keysの実評価・Array cleanupは引き続き未接続。
- 今回の固定suiteは `run.sh --profile integration --owner-profile integration --suite real-apps-exe-boundary --timeout 420` で9/11、473.748秒、実EXE8本＋空marker1本、timeout/SKIPなし。以前の実EXEは全て再PASS。boxtorrentは上記owner61へ移り、JSONはowner22のsource停止を保持。

## 接続済み
- 唯一のattributed Plain Birth providerと同じwriter inventory/非weak未注釈fieldを照合。providerのclassと保存形式を分離し、DynamicResidenceのreceiverはchecked ObjectFieldBorrowで読む。typed field、異class/multiple/hidden writerの拒否は保持。
- Storedの直接Returnは元Return planで判別して既存result ownerへ残す。独立したscalar用途は同じcomputed Return loanとgrounded calleeで昇格し、callerの推測結果型を使わない。
- 元String Coreのclosed I64 receiptはOpaqueHandle向けの式入力にも使う。同じowner/site/Arc/schemaを照合し、qualified frameと最終descriptor/ABIは別途必要。Binaryは既存signed SUM条件を保持する。
- 現在の一意なselected Instanceと同じ元sourceを持つseedを照合する。保持済みPendingをSelectedとして偽らず、位相だけで元の証拠を落とさない。
- unconditional Textの分類は同じ元Static source/ExactString契約を使い、個別call-result行があれば一致を要求する。分類だけで入力・実行・Homeは与えない。
- smokeを旧selfhost/generic JSON経由から正規emit-exeへ変更。古いLLVM関数名pinは退役し、canonical C API/no replay・全期待stdout/exitと既存String source/C negativesを保持。root digest/roundtripがreadDataのText連結を検証する。
- Local初期化のI64 probeは、同じ元owning結果loan・roots・一意なprepared targetで確認済みのobject取得を既存object経路へ渡す。SourceObjectの実行許可は発行せず、入力/Completion/Normalは後続ownerが検証する。
- 同じ元Return planの単独literal Nullだけで汎用scalar seedとI64 probeを非選択にする。callee/batch/key/arityと元targetを照合し、nested Nullやscalar証拠のErrでは判定しない。明示ScalarReturnSourceの拒否は保持。Text型・入力・実行許可は発行しない。
- 全batch/constructor-only Birthのfield-use観測を既存container ownerへ移動。Map payload classは同じwriter inventoryの唯一Map Birthと全setの同一New classから判定し、has/get/delete/keysを型だけの観測として保持。既存MapStoredResultのhas/get/set限定と借用寿命は緩めない。
- 結果式のMap取得localはEntry Receiverと別のleaf。唯一initializer・非rebind/capture・元get Arc・同じfield source Rcとpayload/数値fieldのcanonical ObjectIdを照合する。Normalクラスだけを貸し、nonnull/取得/cleanup/物理読出しの許可は別ownerに残す。
- MapStoredResultはpayload class/ObjectId/censusを同じRc sourceへ集約。単独Map.getもその同じpayload sourceと現在ownerのentry/get Arcから選択でき、Return producerを仮造しない。Return関連は独立したoptional sourceのSomeと全rootsを必須化し、元get/field inventory/entryの再照合を保持。
- get Localの物理matches_input（whole-functionのcalls/capture/mutation制限）は変更していない。型証拠だけではchunk.releaseのCall/lifetimeは完成しない。

- 借用receiverはOrdinaryCall/DirectMapGetを区別し、直接getは元source/declaration/binding/initializerを保持。同じsource-only区間（追加call/write/captureなし、calleeは自身のfield操作のみ）を検証し、全originをSourcePendingへ置く。DirectのUnit/物理呼出しは未完なので拒否を保持。
- 引数ゼロの結果依存は借用Uses定義を作らず、既存parameterless declarationと同じOriginalReturn loan、owner/batch/key/ゼロ引数、callee全体のsource projectionを照合する。mutable facts Rcは共有せず、callee結果groundingと入力/receiver/物理検証は別途必須。

## 次の一手
1. sealの元assignmentを、同じentry receiver/DeclaredField Rc・全writer Text SUM・既存CheckedExpressionEvaluationと`expression_plan::qualify_text_evaluation_v1`へ接続する。RHSのcall/途中Fault cleanupを保持してchecked FieldSetのText Replaceへ渡す。Text Replaceはoperation入口で候補を引き取り、両出口でcaller所有を消費する低層契約まで接続済み。precommit Faultだけ同じtransaction owner内で候補をEndし、postcommit Faultはslot所有を保つ。外側で再Endせず、RHS内の残りtemporaryだけNormal/Faultに逆順Endする。完了後、同じ元Return/結果/discard ownerでNormal取得後Endを検証する。
2. 元inner keys/outer ArrayLen・同じMap fieldの閉じたCore source/結果式は接続済み。実アプリの元Returnを式評価へ渡し、同じinventoryでTextArray値とString/DynamicCarrierを区別する。keysを一度だけ評価し、Array CountとNormal/Fault cleanupを独立した最終検証までつなぐ。
3. native checked Map/fieldのkeys snapshotは実装・runtime検証済み。FieldKeys物理操作・plain Text ArrayのNormal取得/length/End検証とC emissionも接続済み。共通cleanup列は現在Invoke End前提なので、同じownerで非FaultのArrayResidenceReleaseも表し、String Endへの誤変換を禁止してsource packetを発行する。legacy keys_hやMap.lenへの置換、object-child専用ObjectIdの捏造はしない。
4. DirectMapGetのlate receiver/非null/SSA/initializer/借用区間とI64 finisher、put内nested retainの同じbranch区間は未完。source-only区間が成立しても実行許可にはしない。JSONと局所/共有退役も保持する。

## 検証と未完条件
- 共通Text/scalar評価のowner移動（挙動不変）: terminalからexpression ownerへ同じ関数本体を移し、全callerを同じ評価器へ接続。新module/engineなし、移動前後の処理本体一致を確認。terminalは886→752行、expressionは748行で両方800行未満。CLI45.37秒PASS、同じ直接Text field返却の正規LLVM C API/no replay EXE/run/UTF-8 stdout/exit0 PASS。source Text RHSの新規受理はまだ行っていない。
- Text Replace低層: 既存DynamicValue(Replace,Text,TextHome)を両出口consumeとして定義。Rust/Cの独立lifetime検証は同じ最新candidateを要求し、旧Birth Text InitializeのFault保持は維持。C transactionは未commit outcomeをdisposeして候補End、commit後は旧outcome End/disposeへ進む。追加commit flag・runtime ABI・source receiptなし。
- この境界は旧Initialize/snapshot証拠では守れないため、既存Rust Stringケースと既存C factory/probeのみ拡張（Rust test関数/fixtureファイル追加なし、既存C probeにfault wrapperを追加）。Rust9件PASS、FFI C buildと既存C physical family全PASS。実EXE Normal/precommit Fault/postcommit End Faultで候補一回Endを確認。CLI `cargo quick-build-fast -j 4`89秒PASS、既存untyped-field EXE smoke PASS、boxtorrentは同じowner61で未達。固定suiteは前回9/11を保持し、source接続後に再測定する。
- 最終CLI `cargo quick-build-fast -j 4`46.28秒PASS、FFI C build PASS。単独sourceはroute traceでcanonical LLVM C API/no replayまで確認。固定suiteも上記9/11。テスト/guard関数・fixtureファイル・runtime ABI追加なし。一時source/EXE・途中診断を廃棄し、退避stashを本commitへ回収。root-test binaryはtransaction検証時のもの。今回のowner移動後にRust testを実行する際はCargoで再構築する。
- Text field読出し/返却: 最小source `Holder.read(){return me.text}`→`print(holder.read())`は正規EXE/run/UTF-8 stdout/exit0 PASS。未使用メソッドが同じfieldへIntegerを書くだけでも拒否する。元fieldと親をEndした後の独立snapshot利用、誤tagの読出しFault、storage/base/End変異の拒否は既存C physical familyでPASS。新しい独立copy/所有境界を既存I64/Map snapshot証拠だけでは守れないため、既存factoryを引数分岐で最小拡張（test関数/ファイル/checker追加なし）。
- source classからText walk/inputを選び、callee依存順序とreadiness/verified側で同じCompletionを貸す。最終actualsは同じ元入力cohort・signature・Completionと実行済みsnapshotを再照合し、空sliceを生成しない。workerのsource/入力/最終検証/C flowレビューで追加権限・循環なしを確認。
- focusedは元field SUM/Core/pending/constructor・field cached drift・String lifetimeの18件PASS。probeがforeign formalの旧scalar laneを先回りした退行1件を修正し、元7件の再PASSを確認。別のfield-batch duplicateケースは`"bad=" + c.echo(9)`の期待falseに対しtrueで失敗し、変更前HEADをcleanに戻した実buildでも同じassertを再現した既存赤として保持する。期待値変更/削除なし。
- WIP整理: 全writer RHSを同じ元source SUM graphで合流させ、再solve時も同じfield/sourceの事実だけ再利用する。これは型情報のみで、read/Home/実行許可は発行しない。Birthだけでは置換後のText型と他sourceの拒否を守れないため、既存ケースだけ最小拡張（test関数・ファイル追加なし）。
- 今回の切り出し後: `cargo quick-test-fast nested_result_fields_keep_object_identity_separate_from_numeric_writers -j 4` PASS（build41.86秒）、同binaryで既存Core2/pending7/constructor6 PASS、合計16 PASS。`cargo quick-build-fast -j 4`38.60秒PASS。明示CLI/runtimeの既存boxtorrent smokeは同じowner70 Body4で拒否、EXE未達。pointer guard・差分チェックPASS。固定suiteは再実行せず、上記直前測定を保持。
- 退避していたText field snapshot/Return接続は本変更へ回収。元receiver直下の全Return＋全writer Text SUMのみでscalar probeを非選択にし、同じ元field sourceをstage/final readへ保持。readは既存Dynamic get(kind4)で借り、checked materializeのNormalで独立Textを取得してReturnへ渡す。借りたfield自体をEndせず、新runtime ABI・並列台帳・adapterは追加しない。
- Birthの文字列literal RHSを元construction storeへ保持し、既存DynamicTextInitialize/Materialize/FieldSet ownerへ接続。literalはStaticTextのbytesを使い、ParameterのSUM・ordinal・借用ABIを捏造しない。Materialize Faultは先行fieldのみ、store Faultは取得済みTextをEndしてから先行field、store Normalはfield residenceへ移転。独立した最終照合を保持。
- 最終CLI `cargo quick-build-fast -j 4`27.46秒PASS。空文字/UTF-8文字列を初期化する一時programは正規EXE/run/exact stdout/exit0 PASS（確認後source/EXEを廃棄）。既存constructor/最終binding/partial Fault family13件PASS（test build88秒）。literalに続くstoreのFault順序は既存ケースだけ拡張し、test関数/ファイル/checker/runtime ABIの追加なし。元のParameter入力証拠だけではliteralの新規取得と先行residenceを守れないための最小追加。
- 広いconstructor filterの別1件は、未注釈integer fieldの期待FieldContractUnsupportedに対し既存DynamicI64Initializeが受理されるassertで失敗。exact-HEAD比較未実施、原因未分類の赤として保持し、期待値変更や削除はしない。boxtorrentは上記owner70、JSONはowner22に停止し、field SUM/read/write/Return/discard接続は未完。
- 元field subtreeを含む直接return callは、PendingCallのshortcutで元計画を失わず、既存ResultExpressionSourceで同じoriginal plan Rcを保持する。captureエラー、callee依存、元receiver/field/source同一性検証は維持。追加台帳・新規test/fixture/checkerなし。
- この正規化の検証: `cargo quick-test-fast original_return_field_requests_keep_nested_expression_plans_before_qualification -j 4` PASS。同binaryのoriginal-field/lost-return・nested-field・direct-call terminal各1件とpending7件を合わせ11 PASS。最終CLI `cargo quick-build-fast -j 4`45.81秒PASS。明示CLI/runtimeの固定suite9/11（実EXE8＋marker、timeout/SKIPなし）、boxtorrentは上記owner70へ進み、JSONはowner22のまま。source診断は撤去済みで一時adapterなし。
- WIP整理: 共通cleanup writerにInvoke End／非FaultのArrayResidenceRelease＋Jumpを表すactionを追加。既存Invoke callerは同じsuffix算法を再利用し、Array source packet・式評価・独立した構造/path検証への接続は未完。途中cursor変更は境界越えの危険があるため採用せず、従来の最終検証を保持。
- 非Fault Endの単一continuation・正確なinstruction記録・自己ループ拒否は既存Invokeだけの証拠では守れないため、既存canonical writerケースのみ拡張。test関数/ファイル/checkerは追加しない。
- 整理差分: `cargo quick-test-fast root_graph_writer::tests:: -j 4`は3 PASS（build63秒）、同binaryの`root_ordered_`6／`root_cleanup_order_`5 PASS。`cargo quick-build-fast -j 4`66秒PASS。明示CLI/runtimeのboxtorrent smokeは同じowner56 source-not-i64で停止、EXE未達。固定suiteは再計測せず上記直前結果のまま。
- 広いcanonical_root filterでMain0のin-body-step／derived-predicate 2件がIncompleteOrdinaryNewCoverageで失敗し、個別再実行でも再現。exact-HEAD比較は未実施で原因未分類の赤として保持し、baseline成功に読み替えない。
- MapTextKeysSnapshot/ArrayCountの生成lawとtyped Core契約を接続。keys結果はprimitive/Dynamic/nominal Objectへ投影せず、outerは同じinner source Rcを保持。構造依存は独立して閉じ、全request統合後にも最終inner Rcとの一致を再検証する。元field subtree以外の引数/receiverを隠さない。
- CLI46.44秒PASS、root test build99秒。既存Core source2/target7/scalar pending7は全PASS、manifest codegen checkと既存Python14件PASS。正しいMap providerでCountのI64 source sealを確認し、異種provider/複数writerは拒否。同じCore/targetケースだけ最小拡張し、新規Rust test関数/ファイル/checkerなし。
- SCC既存テストは0/1、String左辺と未知RHSのclass転送assertで失敗。以前の記録と同じassertだが旧binaryのrevisionは未確認のためexact-HEAD baseline証明とはしない。追加required_coreはこのケースでは空で、他のsource/publication/Loopの赤とともに未完として保持。
- FieldKeysはkeyなしの同じcanonical field/baseを運び、独立module検証も元field-read pinを要求する。Rust/CともsnapshotのNormal取得・生存中Count・一度だけのEndとterminal残存/escape拒否を検証。Cはsnapshot終了だけ既存checked_text_endへ送り、他Arrayの契約を緩めない。
- CLI56.67秒PASS、`cargo quick-test-fast map_tests::` は13 PASS/1既存ignored（build96秒）。`bash tools/build_hako_llvmc_ffi.sh` PASS。`CC="cc -fuse-ld=lld" python3 lang/c-abi/tests/published_map_physical_execution_test.py target/quick/libnyash_lifecycle_kernel.a` 全PASS（keys snapshotの実EXE0、既存Map実行/Fault/27 malformed含む）。その時点のboxtorrentはowner73のsource停止でEXE未達（現在は現在地を参照）。
- 新しいsnapshot lifetimeは既存Map pointer/owned-object Array検証で守れないため、既存RustケースのCount/End/escape変異と既存C familyの最小snapshot graph/終了変異を追加。Rust test関数/新規ファイル/checkerなし（C fixture factoryは追加）。これは物理層の証拠でありsource受理の証拠には数えない。
- native keys snapshot: Live Mapの同じgateでpublic-textキーを複製し、gate外で辞書順に整列して独立Text ArrayをNormal時だけ公開。fieldも同じ操作へ接続し、既存checked_text_endで終了する。source/Coreからのpacket発行はまだ未接続。
- `cargo test --profile quick -p nyash_kernel --no-default-features --features lifecycle-core --lib --config profile.quick.package.nyash-rust.opt-level=0 fault::checked_map::tests::` は15件PASS（build48.50秒）。`cargo quick-build-lifecycle-fast` PASS（初回7.33秒、field追加後2.46秒）。実archive連結の既存checked_map_runtime_abi_test.cはprofile1/2 PASS（Mapはprofile1）。
- snapshotの順序・数値/UTF-8キー・Map終了後の独立性・未公開out保持・Fault中End/二重End拒否は既存証拠で未検証の新境界なので、既存UTF-8ケースとC ABI検証だけを拡張。test関数/fixtureファイル/checkerの追加なし。runtime追加時の証拠であり、今回のsuite結果は上記を参照。
- 以前のCLI `cargo quick-build-fast`49.02秒、`cargo quick-test-fast --no-run`79秒PASS。既存source outcomes3/scalar pending7/object返却5/stored receiver6は全PASS。boxtorrent smokeは上記新停止でEXE未達。新規test関数/fixtureファイル/checker/for_testなし。
- 型/Return/receiver originの独立した権限分離を既存consumer証拠だけでは守れないため、既存Map検証の7ケースへ最小receiver probe/assertionを追加。Directが普通のCall loanを持たず元initializerを再照合すること、異class/escape/clearの拒否を確認。test関数/ケース数は増やしていない。
- Textの繰返しHome再観測テスト1件は `argument-frame-current-drift` で赤。同日の作業前既存test binaryでも同じ行/拒否を再現したが、対応revisionは未確認なので未完のまま保持し、現在のgreenとは数えない。
- 結果昇格の挙動不変なowner移動は別commit。新moduleを増やさず、profile817行を800行未満へ戻した。テストで検出した移動時import欠落も訂正済み。
- 一時source診断は取得元と区間の成立を測定後に撤去済み。adapterは追加していない。失敗した直接EXE診断の出力は不要になれば廃棄する。詳細はこのcardのGit履歴へ集約する。
- JSON owner22 `borrowed-entry/source-use-completion-pending`、他のsource/publication/Loopの赤、局所/共有R6/R7・最終pipeline条件は未完。mimalloc benchmark promotionはparkedのまま。
