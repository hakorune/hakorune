Status: reviewed simplification direction; execution selection belongs to CURRENT_STATE
Date: 2026-10-27
Parent: [RULES](../../../RULES.md), [CURRENT_STATE](../CURRENT_STATE.toml)
Current app: see CURRENT_STATE; this queue does not resume implementation

# MirBuilder structure refactor queue D0

## Decision

形ごとの分類・再探索・引渡しの重複を、既存の共通ownerへ統合する。
目標は「関数ごとの一つのsource owner、一つの論理実行計画、一つのphysical draft」。
全incomingや再帰の解析は同じmodule単位のsource graphを共有する。
新しい巨大IR、並列コンパイラ、先行wrapper群を追加する計画ではない。

2026-10-21のユーザー依頼により、Claudeのpipeline再設計案を3 workerで
read-only照合した。対象はHEAD `78363aa24d` と停止中のFiniteInput WIP。
Cargo、runtime、EXE、suiteは実行していない。WIPの検証済み範囲は
[app card](app-json-stream-aggregator.md)に従う。
型名・行数の全repo再集計は行っていない。
同日の再依頼では2 workerで再照合し、下記のsource protocol選択条件を補足した。
新しいbuildは行わず、app cardには前回実装中に得たWIP検証結果を反映した。

[final pipeline](../design/mirbuilder-final-pipeline-ssot.md#four-stage-reading-view)
に4段の読み方を集約する。Resolve/Facts/Recipe/Verifyの意味上の境界は保持し、
同じ不変入力に対する同じ検証だけを共通化する。実際に生成・変換されたMIR、
公開JSON、C consumerの検証はそれぞれの境界に残す。

## Proposal review

| 提案 | 判断と必要な修正 |
| --- | --- |
| sema → lower → seal → verify | 説明・責務整理として採用。sema内にsource解析と検証済み論理計画を保持し、最終verifyで生成物を検査する。 |
| 名前の型がgrepで0件だから段が未実装 | 不成立。`VerifiedSourceLoopRecipeProductV2`は既にsource、typed Recipe、Join、scheduleを束ねる。全関数への適用完了は別途必要。 |
| 呼び出しの直積を共通Callへ | 採用方向。canonical target、receiverの由来、入力mode、result、effect/lifetimeを別の軸として組み合わせる。Lexicalは参照形、StoredChild/MapSlotは支持元、Static/Instanceはtarget側なので一つの排他的receiver enumへ潰さない。Tyだけでborrowed/owned/Map-heldを同一視しない。 |
| 型検査1回、Recipeと再照合を廃止 | sourceの意味判定の一元化は採用。評価順・短絡・Join・Normal/Faultを定める計画責務は残す。immutable productの同じ証明を借り、再構築や全graph再検証を減らす。 |
| ownershipを1pass、Call/ReturnにEnd列 | 不十分。各operation/edgeの取得済みprefix、scope、commit前後、Normal/Fault transferとcleanupが必要。1 ownerは1巨大ファイルや1物理passを意味しない。 |
| Loopをcondition/body/flags/carrier集合へ | 共通structured productは採用。zero-trip/Backedge/Continue/Break/Return/After別のpayloadとcleanupを残す。構文観測の統合とSCC固定点の廃止は別。 |
| Refusalを15種、内部拒否をdebug_assertへ | 分類enum＋source位置は採用方向。15種の十分性は未証明。詳細reason/phase/ownerを保持し、releaseで必要な拒否をassertへ落とさない。 |
| 約13万行をgrepだけで削除 | 不採用。`raw_*`にもcanonical callerがある。再export・cfg・feature・外部readerを含む実際の利用を確認して限定削除する。 |
| 11/11後、入口3か所を切りコンパイルエラー側を消す | 不採用。共有Compatibility/schema/backendの既存契約とcaller切替・停止の確認が先。コンパイル成功だけでは挙動不変を証明しない。 |
| 105k LOC、段2–4で2週、全体8–12週 | 未検証の希望値。削除量・納期・有限goalの完了条件には採用しない。必要な最初の実変更で実測し、見積もりを更新する。 |

## Code evidence and normalization seams

- [Call packet source][call-source]は既にInstance/Staticを一つの窓口へ集約する。
  共通操作はここへ寄せ、異なるownership支持元とaffine publicationは保持する。
- [finite input entry][entry]は全incoming、元formal/uses、SUM、vetoを検査する。
  FiniteInputと[共通reference][reference]は形別の重複を減らす方向に沿う。
  監査時には新FiniteInputが既存source-only Static ownerを先取りする競合があった。
  監査時のWIP suite5PASS/6FAILでmimalloc/allocatorの回帰を確認した。
  その後の修正と検証の状態はapp cardに従い、ここへ結果を重複記録しない。
  元source protocolの所属と選択順を保持し、proof未完を別issuerへの迂回条件にしない。
- [Loop product][loop-product]の`produce_source_loop_recipe_v2`はsource検証、
  execution、typed Recipe、Join closure、incoming、scheduleを接続する。
  [Loop lifecycle][loop-lifecycle]は各operationのFaultと途中取得を保持する。
  [source SUM solver][sum-solver]の循環・再帰固定点は同一graph上の必要な計算。
- [received Return][return]はNormal transferとFault時の保持を区別する。
  [final lifecycle coverage][coverage]と[Invoke verifier][invoke]はLower/finish後の
  実命令の過不足を検査する。sourceでの型一致はこの証拠を代替しない。
- [constructor admission][constructor]はcanonical経路から
  `raw_invocation_source_transport`を呼ぶ。`raw_`接頭辞で専用legacyに分類できない。
- [common-V2 session][session]のcaller-zeroという古いコメントに反し、
  [physical-entry session][session-caller]にnon-testの内部callerがある。
  これは1-hop参照ゼロの反例。production rootからの到達性は今回未確定。
- `allow(dead_code)`付き`root_batch_slot`にも`compiler/source_entry_selection.rs`の
  callerがある。`invoke_string.rs`のmissing拒否は元sourceの借用権限を守る。
  許可属性・拒否名の末尾だけでは死亡判定やrelease検証の廃止はできない。

## 2026-10-17 semantic package and packet follow-up

既存リンクとtask ownerを維持する。以下は2026-10-21の調査を反映した統合queue。
このqueueは実装の選択や再開を指示しない。実行状態はCSTATEとユーザー指示に従い、
実装時はCSTATEの一つのappに必要な範囲を選ぶ。
内部手順ごとのrow/card、checker、台帳、新規testファイルは作らない。

| 順序・既存owner | 最初の範囲 / 開始条件 | 完了と削除 |
| --- | --- | --- |
| 現在の選択app（CSTATE） | 現在は boxtorrent の Array identity/支持Home・区間effect・bounds・Map借用を既存source計画へ接続する。下記の局所正規化をapp実装と一緒に実施。 | 共通consumerへ切替えた形別再探索・専有relayを同seriesで削除し、既存focused検証と未変更smokeへ接続。全体rewriteをappの前提にしない。 |
| `MIRBUILDER-SEMANTIC-PACKAGE-OWNER-SPLIT-D0` | 次に必要なowner境界で、共通source query、入力domain、Call/ownership計画、physical投影の実責務を分ける。まず現appが使う共通入力/referenceから。 | 一つの意味判定・一つのwriter。元source protocolの選択優先を保ち、証明成功によるissuer切替をしない。単なるkind×stageの機械分割を避け、重複した分類・探索・wrapperと専有adapterを実際に減らす。移動行と削除行を別記。 |
| `MIRBUILDER-PACKET-DISPATCH-COHESION-D0` | 必要なkind拡張時に、同じ操作を繰り返すmatchを一つのownerへ寄せる。既存Call packetを核にtable/private methods/sealed abstractionを比較。 | 正当な段別exhaustive checkと独立final consumerは維持。旧dispatch callerを切替え、不要なexclusive dispatchを削除。trait/table採用自体を完了にしない。 |
| Call/M7-S → R7 → M8 | 固定製品acceptance後の最初の整理候補は、共有Compatibility一族のcaller切替・停止と専有dead code退役。既に置換済みの局所edgeは全suite待ちにしない。 | 利用root・feature・schema/reader・保持すべき検証を確認し、対象familyだけ削除。既存focused＋必要suiteを通す。未判定候補は一括削除せず、次の利用判定を同じownerに残す。 |
| M8内の診断整理 | 最初の削除familyと共通Call ownerが確定したら、そのownerの既存診断だけを構造化。全freezeの機械置換を先行させない。 | category＋元reason＋source位置を保持。既存negative境界、必要な互換Display、release拒否が同じ。不要formatter/relayを削除。分類数は実例で決める。 |
| package owner-split内のownership整理 | 現appに必要なloan/transfer/cleanupを一つのsource計画へ集める。広い統合はそのfamilyの実行契約が閉じた後。 | operation/edgeごとのNormal/Faultと途中取得・End-once・alias/escapeを保持。physical End ownerと独立検証は残し、二重の計画生成を削除する。 |
| `LOOP-PRODUCTION-SELECTION-D0` / 既存Loop owner | [Loop共通session][loop-owner]の順序に従い、現appまたは残る必須Loop境界で同じsource product/Join/scheduleを使う。 | 観測・route分類の重複を統合し、全edge payload・短絡・cleanup・固定点を保持。選択旧adapterを削除。source範囲を広げる変更とはcommitを分ける。 |

固定suiteの11/11表示だけで共有schemaを消さない。空marker、SKIP、timeoutを除き、
全必要EXEと既存negative境界を確認する。MirBuilder完了にはさらに既存の
source接続・production切替・R6/R7・Loop・物理退役が必要。

## 縮小を伴う正規化（2026-10-27）

現treeの `.rs` を改行で数えると src/mir は1,015,417行/3,827ファイル、
builder は342,789行/1,454、semantic package は200,621行/542。
`ordinary_new`で始まるRustファイルは273本/103,617行（ディレクトリ内の別名ファイルは別集計）。
テストを含む物理行数であり、実行経路長・生存コード量・重複量ではない。
ファイル名の総数から「一つのnewが全ファイルを通る」とは結論しない。

正規化の単位は型だけでなく、元source identity、target、入力mode、支持元、
取得/移転/終了を分離した既存論理計画。関数の不変な意味情報は共有し、
各useの到達条件・alias・借用区間・Fault時の取得済みprefixはsite/edgeに保持する。
「値へ一度付ければ寿命も全siteで同じ」「所有検査は必ず一pass」は採用しない。

| 順序 | 同じ既存app/ownerで行う変更 | 退役と確認 |
| --- | --- | --- |
| 今の接続 | app cardのArray支持・effect・boundsを同じsource graphへ接続。定数guardとLoopで別のArray identity分類器を作らず、既存field/alias queryを共用する。 | 共通queryへ移した元探索と専有relayを削除。site別前提、未知call拒否、独立した生成物検証は残す。 |
| 同じappのMap借用 | 元getのidentityと実use区間を分離し、既存Map/field interval ownerを利用する。 | 置換した関数全体vetoと重複interval探索を切替後に撤去。早期delete/Endを拒否し、最後のuse後のdeleteを許可する既存検証を通す。 |
| family着地 | 共通Call/Newのtarget・receiver・入力mode・result・cleanupを既存計画から各consumerへ渡す。新しい形別producerを作らない。 | 旧形別producerの全callerを切替/停止して削除。cfg/reexport/外部readerを確認し、単なる名前やgrep0件で死亡判定しない。 |
| 広い共有退役 | 既存R7とowner-splitの条件で残りfamilyを一つずつ処理。native/arena最適化のための全体rewriteを先行させない。 | 既存契約を保った実削除・ビルドと必要EXEの確認で閉じる。新旧計画を永久併存させない。 |

最初の具体対象は `home_map_value_flow.rs` の `MapFieldGetLocalV1::matches_input`
と `ordinary_new_lexical_instance_call_provenance.rs::borrowed_receiver_interval_v1`。
既存 `matches_source_identity_v1` を土台に支持区間を共有し、接続済みconsumerの
置換分岐だけを削除する。他consumerのpermissionが残る関数を丸ごと消さない。
`ordinary_new_local_commit/field_read_argument_interval.rs::IntervalViewV1` は
immutable MIRとOnceCellで構造解析を再利用する既存例であり、独立physical検証として残す。

着地時は既存diffで、対象familyのproduction/test別の行・ファイル純増減、
消えた分類/探索/relay、残る旧callerを短く報告する。月次「削除ファイル数≥追加」は
傾向確認に使えるが、ファイル結合だけで達成できるため品質gateにはしない。
読む導線は入口からsource計画・lower・最終検証まで主要10ファイル以内を目安に
既存READMEへ示す。全依存ファイルが10以下という制限や、新しい巨大ファイルへの
結合ではない。最初のfamily切替で実際の導線を確認し、未達を達成扱いしない。
新しい定期checker・台帳・監査cardは作らない。正規化だけで性能改善を主張しない。

## Remaining parked responsibilities

過去の構造調査は再開許可ではない。次の未完責務を保持する。
古い一律800行test分割・新guard義務・census先行義務はRULESで置き換える。

| 既存候補 | 保持する境界 / reopen |
| --- | --- |
| `MIRBUILDER-DYNAMIC-FULL-BODY-RECIPE-BOUNDARY-SHAPE-D0` | 元のliteral compiler→Builder移動とtype-by-type reexport案は不採用。source/cleanup/demandの同じ支持関係を保つ一方向のaggregateが既存owner内で必要になった場合にだけ再検討。今のNoSafeSliceは別appの実装停止理由にしない。 |
| `MIRBUILDER-NORMAL-MODULE-SHELF-D0` | 次に触る一つのownerと実callerが決まった時に検討。normal_*/module_*という名前だけで全面移動しない。 |
| `MIRBUILDER-RAW-ROOT-BOUNDARY-D0` | compiler/builderのsource、physical、publication境界を保持。canonicalが使うraw transportをlegacyと誤分類しない。 |
| `MIRBUILDER-RING0-LOGGER-INJECTION-D0` | 実import/実行境界が阻害になる場合のみ。隠れglobal observerを増やさない。 |
| `MIRBUILDER-AST-VIEW-MIGRATION-D0` | 実際に必要なsource viewだけ。synthetic AST rewriteでsource意味を補わない。 |
| `MIRBUILDER-COMPILER-MODULE-TOPOLOGY-D0` | 宣言配置を触る必要がある時だけ。cfg/visibility、compiler::tests、module_registryの各ownerを保持。 |
| `MIRBUILDER-INVOCATION-TEST-HOME-R0` / `MIRBUILDER-RAW-ROOT-ELIGIBILITY-TEST-HOME-D0` | test行数だけを理由に実行しない。fixture/owner統合で実保守負担が減る時に既存TG ownerで扱う。 |
| `MIRBUILDER-LOOP-CALLERZERO-NAVIGATION-P0` | 対象ownerの次回必要更新に統合。古いtest-only/caller-zero表記と下記guard前提を現caller・後継契約で確認する。新規cardを作らず、拒否を消すだけでgreenにしない。 |
| metadata / broad Context・barrel | 行数や配置だけでは移動しない。既存R7/physical cleanupの条件を保持。 |

README圧縮は既存`SEMANTIC-PACKAGE-README-CAPSULE-R0`、test/guard等価統合は
[TG owner](test-guard-responsibility-retirement-task-2026-10-08.md)、docs/proof整理は
[H-1/H-4](docs-spec-adapter-artifact-hygiene-task-map-2026-10-08.md)、cache整理は
[physical cleanup](../design/repo-physical-structure-cleanup-ssot.md)へ集約する。
このqueueから同じ作業を二重登録しない。

## Compressed completed history

旧本文と当時のcommand/resultは `git show 78363aa24d:docs/development/current/main/investigations/mirbuilder-structure-refactor-queue-d0-2026-08-23.md`。
完了済みの設計書・guard・証拠を現treeへコピーしない。下記は過去の結果で、
現WIPの検証や追加実装の許可にはならない。

| 既存row | 保存する結果 |
| --- | --- |
| `MIRBUILDER-STRUCTURE-BASELINE-CENSUS-P0` / `MIRBUILDER-LIVE-README-PATH-RECEIPT-P0` | 当時の調査とREADME訂正完了。古い行数・caller数は現状の証拠にしない。 |
| `MIRBUILDER-BUILDER-TEST-HOME-R0` | test-home移動完了、production登録を保持。 |
| `MIR-BUILDER-TEST-INVENTORY-BINDING-SHADOW-DEDUP-R0` | 重複shadowing caseは現sourceに存在せず、three binding-id tests remain。lexical-scopeとscoped transactionの安全性契約を保持。 |
| `MIRBUILDER-LOOP-PHYSICAL-PREPARE-HOME-R0` | test-home移動完了、当時6 tests PASS。production activationの証拠ではない。 |
| `MIRBUILDER-COMMON-V2-SESSION-HOME-D0/R0` | 同じlogical moduleへの移動完了、当時63 tests PASS。全common-V2 caller-zeroの証拠ではない。 |
| `MIRBUILDER-COMPILER-TESTS-HOME-R0` | test-home移動完了。当時5PASS/14FAIL/6ignoredで親と一致。赤14を解決扱いにしない。 |
| `MIRBUILDER-IF-CONTROL-OWNER-SPLIT-D0/R0` | analyzer/product/use-ledgerを同じfacadeに整理済み。新しいproduction activationはしていない。 |
| `MIRBUILDER-DYNAMIC-FULL-BODY-RECIPE-DIRECTION-D0` | literal移動は却下。BOUNDARY-SHAPEの調査もNoSafeSliceで完了。将来のaggregate再検討条件は上表に保持。 |
| `MIRBUILDER-SCRIPT-DIRECT-STATIC-SHELF-D0` / `MIRBUILDER-SCRIPT-DIRECT-STATIC-SEMANTIC-SHELF-R0` | Recipe/claim-ledger atomsだけの移動完了。source/lookup/Join/publication/backend境界の全面移動は不採用。 |

## Verification boundary

この文書整理は既存pointer guard、参照先、diff、文書を読む既存構造guardで確認する。
2026-10-21: pointer、builder test-home、compiler tests-home、参照先、diffはPASS。
`rust_mirbuilder_loop_physical_prepare_home_r0_guard.sh`はtest-only前提でFAILし、
上記commitの変更前ファイルだけでも同じFAILを再現した。今回の退行ではないが、
未解決として上表の既存navigation ownerへ残す。Rust/C WIPは変更していない。
コード変更を選択した時は、対象caller/featureと既存focused検証、必要な実app/suiteを
その変更に合わせて使う。新しいcheckerや日次censusは作らない。
独立した拒否・所有権検証は削減量の帳尻で消さない。削減主張には実際の削除を伴わせ、
移動・rename、検証ケース数、実行結果、build/link時間を混同しない。

[call-source]: ../../../../../src/mir/normal_callable_semantic_package/ordinary_new_local_commit/root_call_entry/call_packet_source.rs
[entry]: ../../../../../src/mir/normal_callable_semantic_package/ordinary_new_borrowed_formal_entry_contract.rs
[reference]: ../../../../../src/mir/normal_callable_semantic_package/ordinary_new_local_commit/root_call_entry/expression_reference_read.rs
[loop-product]: ../../../../../src/mir/loop_recipe_contract/source_loop_v2.rs
[loop-lifecycle]: ../../../../../src/mir/loop_recipe_contract/source_loop_execution.rs
[sum-solver]: ../../../../../src/mir/normal_callable_semantic_package/ordinary_new_borrowed_formal_source_sum_solver.rs
[return]: ../../../../../src/mir/normal_callable_semantic_package/ordinary_new_local_commit/root_home_received_return.rs
[coverage]: ../../../../../src/mir/normal_callable_semantic_package/ordinary_new_local_commit/root_validation.rs
[invoke]: ../../../../../src/mir/verification/invoke.rs
[constructor]: ../../../../../src/mir/builder/normal_instance_constructor_admission.rs
[session]: ../../../../../src/mir/builder/resolved_lowering/common_v2_session/mod.rs
[session-caller]: ../../../../../src/mir/builder/resolved_lowering/physical_entry_session.rs
[loop-owner]: ../design/loop-common-physical-demand-and-session-ssot.md
