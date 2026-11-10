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
site/edge情報も一つのsource解析ownerが計算・保持し、consumerは同じ計画を参照する。
使用点ごとの情報が必要という理由で専用証明器の再導出を残さない。必要な固定点や
解析段階は同じowner内に保ち、変換後の実命令との独立照合とは区別する。

| 順序 | 同じ既存app/ownerで行う変更 | 退役と確認 |
| --- | --- | --- |
| 最初のMap借用整理（次に実施） | 既存受理/拒否を保って元get/支持slot/区間effectの重複観測を共有し、実consumerへ接続する。機能拡張より先に実施。 | 旧探索/relayを削除し、追加共通処理込みでproduction純減と既存検証が成立するまで閉じない。全体vetoの意味変更は後続appタスク4へ分離。 |
| 整理series後の接続 | app cardのArray支持・effect・boundsを同じsource graphへ接続。定数guardとLoopで別のArray identity分類器を作らず、既存field/alias queryを共用する。 | 共通queryへ移した元探索と専有relayを削除。site別前提、未知call拒否、独立した生成物検証は残す。 |
| family着地 | 共通Call/Newのtarget・receiver・入力mode・result・cleanupを既存計画から各consumerへ渡す。新しい形別producerを作らない。 | 旧形別producerの全callerを切替/停止して削除。cfg/reexport/外部readerを確認し、単なる名前やgrep0件で死亡判定しない。 |
| 広い共有退役 | 既存R7とowner-splitの条件で残りfamilyを一つずつ処理。native/arena最適化のための全体rewriteを先行させない。 | 既存契約を保った実削除・ビルドと必要EXEの確認で閉じる。新旧計画を永久併存させない。 |

最初の具体対象は `home_map_value_flow.rs` の `MapFieldGetLocalV1::matches_input`
と `ordinary_new_lexical_instance_call_provenance.rs::borrowed_receiver_interval_v1`。
既存 `matches_source_identity_v1` を土台に支持区間を共有し、接続済みconsumerの
置換分岐だけを削除する。他consumerのpermissionが残る関数を丸ごと消さない。
`ordinary_new_local_commit/field_read_argument_interval.rs::IntervalViewV1` は
immutable MIRとOnceCellで構造解析を再利用する既存例であり、独立physical検証として残す。

20万行以内は長期の削減目標であり、達成可能性・期限が実証された見積もりではない。
集計対象はまず上記src/mir全体（test込み）を比較基準にし、production/testを分けて示す。
別ディレクトリへの移動で総量が減ったとは主張しない。finite goalへの新しい数値条件にはしない。

最初のseriesはapp cardのチェックリストを実行正本とする。実装開始前に基点commitと
対象familyを固定し、追加された共通処理・移動先を集計範囲から外さない。
既存git diffの追加/削除行をproduction/test/docsに分け、混在ファイルはhunkで確認する。
コメント/空行整理や整形が混じる場合はコードの実削減と区別し、それだけで純減を通さない。
純減・実consumer接続・既存契約検証・置換経路撤去が全て成立して初めて整理完了。
途中commitは許可するが、純増や退役予定だけで閉じない。削減できなければ同じ範囲で
共通表現とconsumer分担を見直す。入力改変、検証削除、無関係な削除で通さない。

着地時は既存diffで、対象familyのproduction/test別の行・ファイル純増減、
消えた分類/探索/relay、残る旧callerを短く報告する。月次「削除ファイル数≥追加」は
傾向確認に使えるが、ファイル結合だけで達成できるため品質gateにはしない。
読む導線は入口からsource計画・lower・最終検証まで主要10ファイル以内を目安に
既存READMEへ示す。全依存ファイルが10以下という制限や、新しい巨大ファイルへの
結合ではない。最初のfamily切替で実際の導線を確認し、未達を達成扱いしない。
新しい定期checker・台帳・監査cardは作らない。正規化だけで性能改善を主張しない。

## 読みやすさの整理タスク（2026-10-28、優先順）

読み取りworker3人の結果（実行・build未確認、grep/目視）。目的は挙動を変えずに、
src/mir の読みにくさを減らすこと。RULES §2のとおり現appに不要な全repo整理は選ばず、
**現appが触るsource→SUM→loan経路のファイルに限り、そのappの実装と同じseriesで行う**。
着地ごとに production行の純増減と消えた関数/closure名を報告する。新ファイル・新trait・新基盤は作らない。

| 順 | 内容 | 場所（`normal_callable_semantic_package/`） | 見込み | リスク |
| --- | --- | --- | --- | --- |
| 1 | 「単一Text SUM」の手書き判定6か所を `SourceValueSumV1::text_only` に一本化。完全一致のclosure2つ（`..._static_source_actuals.rs:246-253,294-301`）と `..._argument_expression_lending.rs:373,505,562`、`..._initializer_text_source.rs:502` | 同左 | −30行 | 極低 |
| 2 | 有限Text cohort判定の共通化（`static_source_actuals.rs:261-265` と `static_input_finish.rs:111-115`） | 同左 | −8行 | 低 |
| 3 | `BorrowedFormalIngress{site,issue}` の `map_err` 反復（3ファイルで67か所）を、各ファイルの小関数1つへ | `formal_profile.rs`、`coseal_issue_lexical.rs`、`static_source_actuals.rs` | −60行 | 低 |
| 4 | 長い型パス・fully qualified pathを `use`/`type` に（`source_sum.rs` の11引数、`graph.rs` の `SourceScalarKind::`） | `source_sum*.rs` | −16行、120字超の行を削減 | 低 |
| 5 | solver `close` の「外部依存の値を引く」判定が`grounded`と`valid`の2passで重複。名前付き関数へ | `source_sum_solver.rs:104-130,191-230` | −35行 | 中（既存solver testが覆う） |
| 6 | 「通常call引数か」の真偽判定3か所を `ordinary_new_borrowed_call_result_source.rs` の1関数へ（真偽だけ。errorパスは各所のまま） | `argument_expression_lifecycle.rs`、`..._lending.rs` | −15行 | 中 |
| 7 | 巨大関数の分割: `borrowed_call_arguments_callback_v1`(約670行)、`prepare_instance_source_actuals_v1`(約520)、`construct_borrowed_call_actuals_v1`(約510)、`original_mixed_local_loan_v1`(約554)。葉のmatch armを名前付き関数へ。**そのappが実際にその関数を直すときだけ**、1関数ずつ | 各ファイル | 行数ほぼ不変、読む単位が短くなる | 中〜高（借用・所有境界が動く） |
| 8 | `allow(dead_code)` 303か所（145が `compiler/module_registry.in.rs` と `builder.rs`）に理由・利用予定・再判断条件を付けるか削除。grep上のcaller-zeroで確定したdeadは0件。`method_candidates_tail` は有力だが `tools/checks/manifests/chronic_measurement_observations_v1.json` の更新が要る | 横断 | 不明 | 低（削除前に必ずcaller確認） |

JSON着地後のレビュー（2026-10-28、読取りworker3人。未実行）。上の表に加えて、次の順で行う。

| 順 | 内容 | 見込み | リスク |
| --- | --- | --- | --- |
| 9 | `proof_declined`（`..._actual_method_result.rs`）を、`Option`を返すだけの印字helperから「不一致理由を返す小関数＋呼び側1か所のログ」へ。130字超の行も解消 | −15行 | 低 |
| 10 | 診断の足場（`note_invalid`/`explain_open_premise`/`report_formal_sums`/`reference_kind_name`/`note_finite_read`）が約130行。4種の書き方が混在。既存の作法（インライン`if builder_debug_enabled()`）へ寄せるか、card上の停止点が閉じたものから削除。新しい`debug_note!`枠は作らない | −80〜−120行 | 低（既定OFF） |
| 11 | `static_retention`の二重評価（filterとdebug loop）と、runeの一覧を`const`へ | −5行 | 低 |
| 12 | 800行超の既存ファイル。`ordinary_new_coseal_issue_lexical.rs`(1145、うち`borrowed_call_arguments_callback_v1`が約665行・18引数)、`ordinary_new_coseal_issue.rs`(855、`issue_ordinary_source_cohort_v1`が約780行の1関数)、`ordinary_new_coseal_issue_source.rs`(847、`probe_source_home_prefixes_v1`約208行を既存`preparation`へ)。先に置換済み判定を削除し、責務（query/plan/physical）で既存familyを再編する。形別の新ファイルは作らない | 各ファイルを800行未満へ | 中〜高 |
| 13 | `needs_declaration_walk_v1`が省略済みownerを1slotごとに全declarationから探す（O(n²)）。省略済みownerのslot集合を一度作って渡す | 行数ほぼ不変 | 低 |

済み（`entry_formal_reference`）: 入口formalの探し方4か所を1関数へ（本番−6行）。

統合しない（調査で否定）: `borrowed_actual_source_atom_v1` の呼び出し側（目的が別）、`bind_original_argument_plans_v1` の再導出
（独立検査としてScalar/SelfRooted/Home/ReceivedNullableの写像を持つ。再利用すると循環する）、`ordinary_core_source_v1` の2実装（型が別で約4行）。
非test本体に無条件の `eprintln!` は見つからなかった（116か所は抜き取りでdebug gate内）。

## 呼び出しと撤退の整理（2026-11-06、ユーザー承認）

新しいIR、全面の書き直し、ディレクトリの大移動はしない。やることは、同じ事実を決める場所を一つにすることと、新しい経路が通した分の古い経路を同じseriesで消すこと。順番の正本は[順番の決定](selfhost-call-order-consult-2026-11-06.md)。

実測（読むだけの調査、行番号は2026-11-06のmain）: 通らない呼び出しの形は少なくとも5つの別々の原因で、再帰は通る。呼び出しの許可の判定は少なくとも7か所、`seal_local`と`direct_call_lifecycle.rs:565-596`は同じ条件の封印。`home_local_source_scalar_call.rs:545-575`と`role_admits_v1`は位置ごとの手書きの許可表。`*`は3つの形だけ、`-`はrootごとの例外。`Object→Handle`の写しが3か所。述語の写しが7か所と8か所。形×段階のfileが約14,000行。ファイルの置き場所だけを変える再編は、`OrdinaryNewClaimLedgerV1`の`impl`が139 fileに散っているので効果がない。

| 事実 | 唯一の持ち主（既存） | 消す物 |
| --- | --- | --- |
| この呼び出しは何で、結果をどこで使うか | `LocalCallObservationV1`（宛先の`ExpressionValue`は既存） | `join_static_execution_v1`の`local_binding`のfilter、`static_local_route`の分岐 |
| 呼び出しを実行してよいか | `seal_local`。拒否名は条件ごと | `direct_call_lifecycle.rs`の同じ条件、総称の`local-contract` |
| 結果の型 | 結果契約。入口`ExactTrivialScalarAbiV1`を広げる。写しは型のmethod | 各所の`match`、`lexical_i64_result_call`、`exact_i64_return_expression_v1` |
| 式の評価 | `CheckedExpressionEvaluationV1`。文脈は使い先と制御辺だけを渡す | 述語の写し、位置ごとの手書きの許可表 |
| 演算の許可 | `home_scalar_operation_preflight.rs`の一つの表 | `Op::Multiply if ...`の腕、`-`のrootごとの例外 |

分けたまま残す: `physical_boundary.rs`の対応の検査、`check_binding`、`src/mir/verification`、C shim。生成後の独立検証は生成側と共有しない。

| series | 内容 | 製品の受理 | 完了の条件 |
| --- | --- | --- | --- |
| S1 | 述語、封印、結果の写し、`precedes`の統合。token cursorのparser経路の撤退（ユーザー決定）。caller 0の関数の削除 | 変えない（拒否の名前だけ） | 固定suite、比較入力の受理と拒否の入れ替わり0、cohortの新しい赤0、production純減 |
| S2 | 順番の①。位置の制限、生きたHome、methodの結果、`x - 定数`、loopの中の呼び出し。`qualified-preflight`の旧経路を、今通る形を測って移してから撤退。`DeclaredInstanceLocatorNotConsumed`の検査へbuilderの経路から届かせる | 広がる | Normal、後の引数のFault、呼ばれた後のFaultのEnd、MIR改変の拒否。`ordinary_new_*`のfileを増やさない |
| S3 | 順番の②。`: bool`と`: String`の入口、演算の表、基本の式、container。中身は下の「流れの作り直し」のS3も含む | 広がる | 各形のEXEとEnd。公開MIRの追加が要れば止めて聞く |
| S4 | 形×段階の整理。compareとmul、static finishの共通の2句、loopの位置別loan。中身は下の「流れの作り直し」のS4（loopの統合）を含む | 変えない | 消えた型と関数の名前、production純減 |

3bで撤退: `legacy_port.rs`、`raw_ordinary_new_claim`のlegacyのimpl、`birth-global-legacy-stopped`、`FromCall`、RustのWASM backend（2026-11-06ユーザー決定、[WASM roadmap](../design/wasm-hako-only-output-roadmap-ssot.md)）。撤退しない: `MirFinishScheduleV1::Legacy`（今の既定の経路）、`resolved_semantics/shadow/`、`resolved_lowering/`（canonicalのCFGとSSAを多くの所が使う）。micro seedのplan群は公開JSONの契約に関わるのでユーザー判断。

やらない: 行数の目標、行数だけのfile分割、置き場所だけの移動、`finished-sequence`を緩めること（`x - 定数`の直し先は下書きを作る側）、形ごとのfileやproducerの追加、「選ばれた後の欠け」を黙って`None`に落とすこと、`Static*InputFinishV1`を一つにすること、`freeze`と`reject`の一括のmacro化。

未確認: 式の位置の呼び出しを既存の`ExpressionValue`のemitで物理に載せられるか（足りなければ公開MIRかABIの追加なので、S2の最初に確かめて止めて聞く）。`x - 定数`で`Const`が消える理由。loopの中の呼び出しの理由（形ごとに別）。旧経路で今どの形が通っているか。深い再帰がexit 139で落ちること（Fault契約の外。順番の⑤までに扱いを決める）。

S1着地（2026-11-06、Sonnet実装・Opus監査、`1de52b773a`）: 述語2種、封印の共通部、結果の写し、`precedes`を統合し、token cursorのparser経路とcaller 0の関数26本を削除。production +133/−1926。固定suite 8/8、比較229入力は差0（拒否名の差も0）、cohortとpublished_backend_viewの失敗名は前と同じ。統合しなかった差（D4、D9、`seal_local`だけの条件）は報告`sonnet-mir-s1-report.md`の第2節と第8節。

S2着地（2026-11-06、Sonnet実装・Opus監査、`5b82d1db73`、docs `662b84ffa0`）: 物理MIRの追加なしで、入れ子の引数、生きたHomeがあるcallerの呼び出し、`print`・加算の項・`return`の値の呼び出しが通る（測った形14のうち8、[status-index](../../../../reference/language/status-index.md)）。旧`qualified-preflight`経路は撤退し`qualified-route-retired`の拒否名にした。Rustの独立検証が`I64`の呼び出し結果をcalleeの宣言と照らす（`call-result-type-drift`）。Fault側のHomeのEndは対応検査`check_fault_chain`が鎖を照合する。production +127/−260。比較229入力は差0、前に通っていた`apps/**`100本はexitと出力が前後同じ（Opusが測定）、cohortとpublished_backend_viewの失敗名は前と同じ。残り（`x - 定数`、`*`、条件、Assignment、instance methodの結果、loopの中、2つの位置判定の食い違い、撤退で残ったcaller 0の層、End欠落の独立検証）はS2bへ。

S2b着地（2026-11-06、Sonnet実装・Opus監査、`da20665f36`）: 撤退で残った層と位置の判定2つを消した（production +34/−517）。公開MIRだけからHomeのEndを検証する入口`ownership_ssa/published_homes.rs`を足し、Fault側のEndの欠落・二重、Normal側の欠落を拒否する（導出が一意にできない形は主張しない）。`check_call_result`はUnitとBoolも見る。引数の`x - 定数`、演算の下の呼び出し、Assignmentの値の呼び出しが通る（新しく18入力）。比較229入力・apps 100本は前後同じ、cohortとpublished_backend_viewの失敗名は前の部分集合。Opusが指示書に無い8形で試し、6形が正しい出力で通った（`print(a(1)+b(2)-a(3))`と`r += a(2)`は拒否のまま）。監査の指摘: 新しい位置の規則`site_stands_as_expression_value_v1`はpathの末尾のsegmentの一覧によるbooleanで、Proの勧め（位置を一つに確定し、担当は別に一度だけ決める）に届いていない。S2cで位置のenumと担当の一元化を行う。method の結果は構文の判定と結果cohortがどちらも相手より広く、すげ替えると受理が変わる（4 test）ので、S3aで値の種類の持ち主を一つにしてから両方を消す。

S2c着地（2026-11-06、Sonnet実装・Opus監査、`44eae090aa`）: 式の位置を、`SourcePathSegmentV1`の全variantを`_`なしで分ける`ExpressionPositionV1`（9種）にし、呼び出しの担当`CallExecutorV1`（文の書き手か、囲む式の検査済みの引数frameか）を位置から一度だけ決める。前の規則が値と見なしていた`BlockExprPrelude`・`TryBody`・`CatchBody`・`CleanupBody`の文は、文の位置になった。route記録はsiteごとの状態の表（`CallRouteTableV1`）になり、root packetへ渡した後の書き込みは拒否する。if条件の中のqualifiedなi64の静的呼び出しが通る（新しく13入力。前に通っていた物の変化は0。比較229入力・apps 100本は前後同じ、cohort 84とpublished_backend_view 12は同じ名前）。Opusが指示書に無い12形で試し、6形が正しい出力で通り、6形は名前つきの拒否で、誤った出力は0だった。未完: Local初期値の連鎖（約12の腕）は消えていない。腕は「呼び出しの種類×結果の値の種類」で分かれ、membershipの問いが`issue_*`の中にあって8層を通るので、S3aで値の種類の持ち主を一つにした後に、別のletterで担当を一度に決める形にして消す。同じsiteを複数の段階が書く（単一の書き手にすると既存test 23本が落ちる）ので、route記録の冪等な書き込みも残る。Opusは位置の対応を写しただけのunit test 2本を外した（RULES §6）。

赤いtestの整理（2026-11-06、Sonnet実施・Opus監査、`4e3bc7318d`）: 製品のコードは変えず、testと道具だけを直した。全libの赤は294→275（うち44本は全体実行でだけ落ちる。環境変数`NYASH_MIR_UNIFIED_CALL`を書き換えるtestと読むtestの競合で、単独では通る）、cohort 84→83、`parser_` 16→7、guard 5→2（残りは`logical_condition_surface`と`check_block_surface`）、`published_backend_view`は12のまま。消えた19本は、古い期待9、道具とfixture 9、消したtest 1（`for`の範囲のparser test。同じ性質は`tests/parser_grammar_profile.rs`が守る）。Opusの全lib（4 thread）は赤238で、全部が整理前から赤（新しい赤0）。残りの231本は失敗の最初の行と繋ぐ段階つきで[一覧](red-test-sort-2026-11-06.md)にある。製品の誤り（未修正）: 埋め込みのmodule snapshotが古い、入れ子のbuild gateで外側の`branch_member_ordinal`が内側の値になる疑い、`StringBox`の弱handleが元を落とした後もupgradeできる疑い、未解決の関数呼び出しの診断名が合わない、`Phase0MirJsonEnvGuard`が上の環境変数をprocess全体で書き換える。台帳`cargo_lib_red_baseline.*`は更新していない。

S3a着地（2026-11-06、Sonnet実装・Opus監査、`54d389986e`）: 型の規則を先に文で書き（[地図と規則](mirbuilder-value-class-owner-s3a-2026-11-06.md)）、「localの種類は初期値の種類」「`: i64`のformalは宣言の型」を結果cohortの式の木に入れた。構文で決めていた`lexical_i64_result_call`と`exact_i64_return_expression_v1`は消してcohortに聞くだけにし、S2bで落ちた4本の原因（exactな値同士のMulに借用のproductを要求していた）を直した。演算子の種類は`scalar_operator_kinds_v1`の1表にし、位置ごとのMultiply・Divideの特例は意味が決められず`used_here`に残した。methodの結果がi64かを独立に決める場所は4→3。229入力・apps 100本・固定suiteは同じ、cohortは新しい赤0。production +260/−211。Opusが指示書に無い16形を流し、間違った出力0（新しく通った2、前から通っていた5、名前つきの拒否9。[status-index](../../../../reference/language/status-index.md)に記録）。追加のtest 1本（localの連鎖がi64の行を持つこと）は、この規則の自動の証拠が他に無いので残した（RULES §6）。未完: 段階3の`: bool`・`: String`の結果は、callable headerの結果の種類が`i64`だけで、足すとdescriptorが変わるので止めた（ユーザー判断待ち）。段階4は連鎖を消すletterの後。`project_expression_class_v1`の同じ計画への射影は1回にならず増えた（34→42、32→60）。結果クラスのjoinごとにcohortを作り直すのが原因で、未着手。

結果の契約の順（2026-11-06ユーザー決定、[相談の記録](result-contract-consult-2026-11-06.md)）: 連鎖を消す → 警告の整理 → 結果の契約の土台と明示のBool → S3b → 所有のString・object・nullable → S4 → `edit`以降。土台は、既存の`VerifiedCallableResultContractRowV1`を型・返し方（値か読み借用か）・出所（宣言か移行用の証明か）の一つの契約に育て、header・呼ぶ側の受け取り・公開MIRをその投影にする。Boolは既存の`Bool`/`ordinary_bool`を使い、変えるのは内部のheaderの型だけ。S3bは確定した契約を入力にして、結果の型を推論し直さない。借用のString（`String from me`）は公開の結果契約が要るのでユーザー判断、それまで名前つきの拒否。

連鎖の除去着地（2026-11-06、Sonnet実装・Opus監査、`bd1f3fd5ed`）: Localの初期値の呼び出しは、packageが1回だけ答える`LocalCallRouteV1`にした（呼び出しの種類は名前解決、結果の種類はその種類の持ち主、installは結果の種類の1表。[規則と重なりの測定](mirbuilder-local-call-route-owner-2026-11-06.md)）。membershipのclosure 7本のうち6本と、その転送165→22か所を消し、連鎖の発行は11→5。route表は書き手を1つにし、2回目は`call-route/routed-twice`（同じlaneの後段は`continues`の2組だけ）。`project_expression_class_v1`のhitは半分（42→21、60→30）。受理は変えていない: 229入力・追加277入力・apps 100本・固定suiteは同じ、cohort 83とpbv 14は名前も最初の行も同じ、全libの差は単独で通る競合だけ。Opusが指示書に無い20形（直接・static・lexical・`me`・map・handle・nullable・Boolを、分岐・loop・演算の下に置いた物）を流し、前と後で全部同じ、通った7形の出力も正しい。production +506/−658、test +2/−42（membershipを聞かなくなったissuerのtest 1本を削除）。残り: `terminal_call`（Returnの消費者に結果の種類の根拠が要る）、borrowed portの2つの結果の出所（portに副作用があり、routeの中で聞けない。routeの前の3腕と後の1腕が残る）、`has_routed_static_local_for_owner_v1`（Localのrouteではない）、cohortを作り直す繰り返しの残り。食い違い: 本体にmap literalを持ち`new C`を返すdirectのcalleeを、発行側はMap、co-sealの`expected_class`はHandleと見る（受理を変えないので直していない）。S3aの退行（この監査で発見）: `len(): i64 { local k = me.name.length() return k }`が`borrowed-result/source-not-i64`で拒否される（`text_field_reads_compile_in_the_listed_shapes`。S3aの前は通っていた。S3aの監査でpbvを流していなかった）。警告の整理の最初に直す。

警告の整理着地（2026-11-07、Sonnet実装・Opus監査、`b02d363920`）: 先に3件を直した。C（S3aの退行）: Core呼び出しの受け手が宣言済みfieldの読みなら、その読みは呼び出しのoperandで、Coreのsourceが閉じる（[S3aの規則](mirbuilder-value-class-owner-s3a-2026-11-06.md)に行を足した。`length`の特例は無い）。A: 埋め込みのmodule一覧をscriptで作り直した（+11名、消えた名前0。作り直しを促すguardは無く、気づけるのはtest 1本だけ）。B: `Phase0MirJsonEnvGuard`を消し、Program(JSON v0) bridgeはMIR JSONのprofileを値で渡す（bridgeのloweringはbuilderを通らず、環境変数が効くのはemitだけだった）。builderは今も`NYASH_MIR_UNIFIED_CALL`を呼ぶたびに読むが、途中で書き換える物は無くなった。整理: production +402/−12,967、productionの`allow(dead_code)`などを486個外した。警告はlibのbuild 1,791→3,040（allowで黙っていた、testだけが使うコード4,755件が見えるようになった。[一覧](test-only-code-inventory-2026-11-07.md)、撤退はユーザー判断）、testのbuild 824→694。可視性の不一致135→0（itemを狭めると733件のerrorになるので、型を広げた）。両方のbuildに残る672件は、読まれないfieldと作られないvariant（消すと初期化とmatchが連鎖する）、戻したfeatureのitem、未使用の引数2件。監査: 全libの赤277→233、新しい赤0（消えた44本は競合42本とA、C）。Opusが指示書に無い形を流し、fieldを受け手にするlengthのlocalの5形は直って出力も正しい。**残り: 受け手がlocalのText（methodの中の`local s = "xyz"  local k = s.length()`）は、S3aの前は通り、今も`source-not-i64`**（次のletterの最初に直す）。`unresolved_call_result_type_reaches_the_owning_rejection_without_a_panic`の赤は、S3aで通るようになった`local y = x + 1 return y`（出力4で正しい）を拒否と期待する古いtest（同じく次のletterで直す）。監査で見つけた失敗1件: 既定のbuildの警告だけを見たので、`--features vm-reference`の中からだけ使うitemを消していた（41件のerror。mainは既知のerrorで先に止まるので、気づけなかった）。mainから戻し（236行、全部mainと一致）、errorはmainと同じ`type_ops.rs:8`だけになった。ほかのfeature 7つはerror 0。1つずつではbuildできないdirectoryごとの7 commitは、1つにまとめた。

結果の契約とBoolの土台の着地（2026-11-07、Sonnet実装・Opus監査、`ef0cc09982`）: 先に2件。D: Core呼び出しの受け手は、宣言済みfield・local・formal・literalのどれでも呼び出しのoperandで、Coreのsourceが閉じる（S3aの規則の行を受け手の種類で分けない文に書き換えた。受け手のleafは残すので、受け手の義務は今まで通り検査される）。methodの中のlocalのTextを受け手にする`length`・`indexOf`が通る。E: 古い期待のtestを直した。本題（[規則](mirbuilder-result-contract-bool-2026-11-07.md) R1〜R11）: 宣言の`: Bool`を結果の契約の行で受ける。`ExactTrivialScalarAbiV1`に結果だけのBoolの投影を足し（引数の型には使わない）、宣言したBoolは本体の証明が`ExactBool`の時だけ通る。宣言と違うscalarを返す出口は`DeclaredResultDisagrees`で拒否する（`: i64`で`return true`も同じ名前になった）。宣言の照合を1つの表にし（2か所の分岐と11行のmatchを消した）、static入力の5か所は1つのscalarの照合を読む。**宣言のBoolが通るのは、宣言の無いBoolが今通る形だけ**（Text入力のstatic。例: `json-stream-aggregator`の`boolField(..): Bool`）。exact-i64入力のstaticのBool（`local b = Main.t(4)`）、比較を返す本体、`if f()`、instance、再帰、`print(f())`は、宣言の有無にかかわらず拒否のまま。理由: 静的なscalarの呼び出しのclaimがI64しか持たない（`qualified_static_call_claim.rs`、`is_qualified_i64_source_v1`が42か所）、`ExactBool`が引数の要求を持たない、比較を返す出口のterminal relationが無い。次は、このclaimを結果の種類つきにする（Bool専用のfamilyを足さない）。公開MIR・ABI・descriptorの追加なし。production +151/−37。test: 宣言の有無で同じ道になること（R9）と、違う出口の拒否（R2）の2本、solverの宣言の照合の1本を、既存の証拠で示せない契約の正例・負例として残した。spellingだけを写すunit test 1本はOpusが消した。監査: 全libの赤233→231、新しい赤0（消えた2本はEと、空き容量が0だった時の`test_filebox_exists`）。229入力・apps 100本は基準と同じ。Opusの指示書に無い19形: 宣言したBoolの3形とDの3形が正しい出力、残りは基準と同じ拒否か`DeclaredResultDisagrees`、誤り0。

速さの調査（2026-11-07、Sonnet調査・Opus監査、`9c3999f384`、[記録](build-test-speed-survey-2026-11-07.md)）: 全libの遅さは、環境変数のlockを握る1本の待ちだった。その1本は41秒の契約の発行を2回していたので、testの3行で1回にした（caseとassertは減らしていない）。全lib 575→492秒、赤231は名前まで同じ。8 threadは488秒でほぼ縮まず、赤が30本増えたので4のままを提案（決めるのはユーザー）。残る大半はopt-level 0での`apps/mimalloc-lite`のcompile（1回60秒）と発行（41秒）で、再検証に時間が散らばり原因は1か所に絞れていない。

種類つきのclaimの着地（2026-11-07、Sonnet実装・Opus監査、`22a5713013`、[規則](mirbuilder-result-contract-bool-2026-11-07.md) R12〜R18）: 静的なscalarの呼び出しのclaimは、calleeの契約の行から結果の種類（I64かBool）を持つ。`StaticI64CallClaimV1`は種類つきの`StaticScalarCall…`になり、`is_qualified_i64_source_v1`の38か所は種類を返す1つの問いに置き換えて消した。Bool専用のstatement hookも消した。`ExactBool`は引数の要求を持ち、比較はoperandの要求を残す（R13）。exact-i64入力のstaticのBoolがEXEまで通る（`local b = Main.t(4)`と`if b`、比較・`local`経由・`if`と`return true`の本体、宣言あり・なし、`local k = Main.odd(n - 1); return k`の再帰）。拒否のまま: `if Main.t(4)`、`return Main.t(x)`（I64でも基準で拒否）、`&&`を値で返す形（意味は仕様の短絡で決まっている。値を分岐と合流で書くwriterが無い）、instanceと`me`、`print(Bool)`。公開MIR・ABI・descriptor・C shimの追加なし。production +716/−430（約70か所の機械的な改名を含む）。監査: 最後のcommitで全lib 233赤、新しい名前2件は単独で2回ずつ通る（並列の競合）。229入力・apps 100本は基準と同じ、固定suite 8/8。指示書に無い17形で誤り0（`b == c`、Boolを受けるi64の関数、2つのformalの再帰、`0 - 3`の実引数など10形が正しい出力。`local s = x * y`の後の比較などはI64でも基準で拒否）。Opusが、新しいtestと文書の例の空白だけの区切りを改行と`;`に直した。残り: 演算子から`CompareOp`への写しが3か所、R1が2つの道（宣言のi64は本体の証明を飛ばし、宣言のBoolは種類のseedで再帰を閉じる）。

S3bの歩Bと歩Aの着地（2026-11-07、Sonnet実装・Opus監査、`421b82d74c`、[規則](mirbuilder-ownership-transition-s3b-2026-11-07.md) S3b-R1〜R10）: 受理を変えない置き換え。所有の状態を変える操作を、責任の並び`HomesV1`（中身はprivate、変更は`local_flow`のfileだけ）と`PrefixLocalFlow`の遷移操作（取得、calleeへの移転、localのmove、返却と格納、置き換え）に集めた。型の外から`homes`を書き換える所は25→0（loopの別の並びの6か所はS4）。Faultの後片付けの計画は`FaultCleanupPlanV1`の1つにし、添字を手で作る所は4→0、書く部品は計画を受け取るだけ。重複はD1・D2・D4・D8が消え、D3は一部、D5・D6は答える問いが違うので残した。先に`home_prefix_local_flow.rs`（1,220行）を5 fileに分けた。演算子から`CompareOp`への写し（3か所）と結果の種類の写し（2か所）は1つの表にした。公開MIR・ABI・descriptor・C shimの変更なし。production +1,393/−1,120（分割の移動を除くと+222。新しい型2つが428行）、testは改名だけ。監査: 生成されるMIRは229入力とapps 100本の329入力で基準と全部同じ（`--emit-mir-json`はInvokeを書けないので`--dump-mir`で比べた）。229入力・apps 100本・固定suite 8/8は同じ。全libは赤232で、新しい名前は1本（手で組んだMIRをLLVMに通すtestで、変えたコードを通らず、単独で2回通る）。途中のcommitも単独でbuildできる。Opusが指示書に無い32形（move・take・`new`の実引数の呼び出し・文字列の一時値・分岐の中のmoveなど）を流し、前と後でMIRと出力と拒否の名前が全部同じ、通った9形の出力も正しい。残り: 責任のIDは新しい型でなく`LiveHomeV1`のoriginで持つ。3つの登録先は型でなく計画のconstructorで分ける。演算子の写しの4つ目が`ordinary_new_borrowed_compare_materialization.rs`に残る。両方の枝で同じHomeをmoveする`if`（`MoveUnsupported`）と、`if`の中だけのlocal（`artifact-unowned-lifecycle-site`）は前から拒否。次は関数ごとの切り替え。

S3bの担当の決定の着地（2026-11-07、Sonnet実装・Opus監査、`f862402e71`、[規則](mirbuilder-ownership-route-s3b-2026-11-07.md)）: 関数ごとの切り替えの1段目。受理を変えない置き換え。関数の所有の担当（走査を通らない発行`seed`と`app_main_integer`、走査の道`VerifiedWalk`と`PrefixOnly`）を`OwnershipRouteV1::decide`の1か所で決める。今まで5つの式（`seed_completion`、`verified_walk`など）と18個の`has_*`の変数で決めていた物を、操作と契約の名前の要求19行（`ScanRequestV1`）に置き換えた。`seed`と走査が両方真になるのはapp mainの2つの場合だけで、今と同じく`seed`を出してから走査する。走査の前に旧走査を回すreadinessのprobeは1つのまま（失敗が受理に効くので、新しい解析が答えるまで残す）。他の関数の`borrowed_formal_actuals`を埋めるprobeは担当の外に置いた。演算子の写しの4つ目も1つの表を読むようにした。`ordinary_new_coseal_issue.rs`は820→796行。production +211/−86。監査: 329入力のMIRは同じ、229入力・apps 100本・固定suite 8/8も同じ、全libは赤232で名前の差は前からの並列の競合1組だけ。Opusが前の監査の入力1,459本（通る835、拒否624）で、歩B/Aの前、歩B/Aの後、今回の3つのbinaryの`--dump-mir`を比べ、全部同じだった。途中のcommitも単独でbuildできる。残り: 要求の行は「走査の何が要るか」をまだ持たない（2つの性質だけ）。走査の入口は4つのまま。

S3bの2段目の作り方（2026-11-07、Sonnet比較・Opus確認、[記録](s3b-analysis-options-2026-11-07.md)）: 案1（Proの案。if・loopの無い関数だけを新しい解析に通す）は、通る関数がappsの18〜35%で、同じseriesで消せる旧い判定がほぼ0。案2（文を辿る走査をCFGを辿る形に替える）は、点が文1つだと再帰の書き直しにしかならず、Faultの辺は観測の中に埋まったまま、loopも同じ走査を使う。**ユーザー決定: 案3（観測の中の確認を契約ごとに1か所へ）を1回やり、その後は所有のString・object・nullableの結果へ進む。CFGの作り方（案1か案2）はS4で決める。**

moveの確認の着地（2026-11-07、Sonnet実装・Opus監査、`e1ce026a83`、[規則](mirbuilder-move-contract-s3b-2026-11-07.md)）: 案3の1回目。`move`かどうかの問い（`moved`）とoperandを読む所（`stored_value_operand`）は元から1つだった。重なっていたのは問いの後の確認で、`move`のoperandをlocalのbindingに解く所3→1、`UseAfterMove`を決める所5→1、同じscopeの確認2→1、`stored_value_operand`と同じ式1→0。package側の6か所はclaimの事実を作る別の問いなので残した。production +80/−35。監査: 329入力と1,459本のMIR、229入力、apps 100本、suite 8/8、全libの赤の名前（232）が全部同じ。Opusの指示書に無い9形（同じ呼び出しの中でmoveの後に読む、2回move、i64のmove、share moveを2回、ifの中のmove、moveの後のprint）も前と後で同じ。前からの拒否: `new Node(move a, a.get())`は`UseAfterMove`でなく`argument-source-unavailable`。

所有のStringの結果の着地（2026-11-07、Sonnet実装・Opus監査、`9a625e687a`、[規則](mirbuilder-result-contract-string-2026-11-07.md) S1〜S9）: 宣言の綴りを読む表を`DeclaredResultKindV1`の1つにし（`classify_result`はそのscalarの射影）、`: String`は宣言の無い関数と同じTextの結果の契約を通る。`Unannotated`だけを受けていたTextの判定は、宣言が無いかStringかの投影1つにした（`String?`の判定は範囲外で残る）。受け取りは前からある`OwnedString`の道で、新しい遷移も計画も足していない。公開MIR・ABI・descriptor・C shimの追加なし。production +79/−30。test: 宣言の表の1本（`String?`、`StringBox`などをStringと読まない事の証拠）、source-proofの期待1つを新しい契約へ直した。監査: 229入力は前から通る71本が同じで1本が新しく通る、apps 100本・suite 8/8は同じ、全libの赤232は名前まで同じ。1,459本で違いが出た8本は、全部`: String`の宣言を持つ物で、5本が新しく通り、3本は宣言の無い同じ本体と同じ拒否の名前になった。Python v4は17本が通り、4本は基準のtreeでも同じ`FileNotFoundError`（`-bool`のcaptureを書くtestが前から赤）。Opusが指示書に無い24形を流し、誤り0（`s + "cd"`の本体、受け取った後の`length()`、localを経た転送が正しい出力。Stringでない値を返す`: String`は全部拒否）。残り: `: String`で違う値を返すとEXEの入口では`qualified-route-retired`という名前になる（Boolと同じ）。使わないboxがmethodを持つと、宣言の有無にかかわらず`check-uncovered-function`で拒否される（前から）。`ordinary_new_borrowed_static_scalar_input_finish.rs`はBoolのseriesから805行で、800行を超えている。`declared_i64_source_agrees_v1`と`declared_bool_source_agrees_v1`は同じ形。

所有のobjectの結果の着地（instanceと`me`まで。2026-11-07、Sonnet実装・Opus監査、`1ba243ec53`、[規則](mirbuilder-result-contract-object-2026-11-07.md) O1〜O9）: 宣言の`: P`は、名前解決が`P`をboxと答えた時だけ宣言の表`DeclaredResultKindV1`の`OwnedObject`の行になり、宣言の無い同じ本体と同じobjectの結果の契約（`agrees_with_object_result`の投影1つ）を通る。`Unannotated`だけを受けていたobjectの判定2つはこの投影にした。宣言がboxを名指す時は、そのboxが結果のclassを決める。`declared_i64_source_agrees_v1`と`declared_bool_source_agrees_v1`は1つにし、805行のfileと809行になったfileを責任の名前で分けた。公開MIR・ABI・descriptor・C shimの追加なし。production +430/−353（分割で移した約330行を含む）。test: 宣言のboxと本体のboxの一致の1本、名前解決の表の1本。監査: 229入力は前から通る72本が同じで新しく通る物0（corpusの`: P`はstaticだけ）、apps 100本・suite 8/8・全libの赤の名前は同じ、1,459本の違い3本は全部staticの`: P`の拒否の名前の変化。Opusが指示書に無い37形を流し、誤り0、宣言あり・なしの答えは全部の組で揃った（nullを返す形だけ拒否の名前が違う）。**static**: objectを返すstaticの呼び出しを受ける入口が、宣言なしでも基準に無い（staticの道は3本に分かれ、どれも結果がscalarかText）。ユーザー決定で、先に道の調査をする。残り: instanceの道は宣言なしでも狭い（引数つきの`m.mk(5)`、受けた後の`a.get() + 1`、入れ直し、`next(): Maker`、`return me.f`は拒否）。規則の文書は最初の実装commitの後に書かれた（指示は先）。

staticの呼び出しの結果の道の調査（2026-11-07ユーザー決定「staticの道を先に調べる」、Sonnet調査・Opus確認、`df2d91cb61`、[記録](static-call-result-routes-survey-2026-11-07.md)）: staticの呼び出しの結果を受ける道は6本で、formalの種類と結果の種類の組で選ばれる。384組を流して106組が通り、qualifiedの`Main.mk(5)`がobjectを返す組は0。何も無い`main()`で`local a = Main.f(5); return a`だけを持つ形は、型つきの静的な実行の`context`（`direct_call_static_execution.rs:124-141`）に入らず拒否される。`new`か`print`を足すと通る。qualifiedのmainの経路を消す前（`fc0045c93d`）でも拒否されていた（前からの穴）。勧め: 結果のclassの表をinstanceとdirectで共有し（受理を変えない。相談の7「staticとinstanceを共通にする」に合う）、型つきの静的な実行に結果の種類を持たせ、受けたobjectの`a.get()`のため`initializer_class`にstaticの枝を足し、最後にTextの引数なしのpacketを消す。物理側にstaticのHandleの結果を拒む検査は見つからない（C shimの返却起源は未確認）。`ordinary_new_borrowed_static_packet_entry.rs`は前から844行で、routeの選択とco-sealで分けられる。

staticの結果を受ける関数の道（2026-11-07ユーザー決定「穴を先に塞ぐ」、Sonnet実装・Opus監査、`9c15e3b59c`、[規則](static-call-context-rule-2026-11-07.md) C1〜C8）: i64のformalのstaticの呼び出しの結果をbindingに受けるownerは、mainかどうかに依らず、型つきの静的な実行の`context`に入る（`additions`のownerを足す1文。production +6/−1）。何も無い`main()`で`local a = Main.f(5); return a`、宣言の無い同じ形、`print(Main.g(5))`で呼ばれる`g`の中の`local a = Main.f(v)`が通る。testの期待1つを新しい規則へ直した。監査: 329入力のMIRは差0、1,459本の違い7本は、前は拒否で今は通る4本、拒否の名前だけが変わった2本（calleeの乗算は`completion-not-ready`）、EXEが拒否していた生の`call_global`が`invoke Call`になった1本。229入力・apps 100本・suite 8/8・全libの赤231の名前・Python v4は前と同じ。Opusが指示書に無い15形を流し、誤り0（前から通る8形は`--dump-mir`が同じ）。残り: `return Main.f(5)`（root completion）、Stringのformalのstaticを受ける形（`qualified-route-retired`）、`loop`の中で受ける形（S4）、calleeの乗算。

staticの呼び出しの結果の種類（2026-11-07ユーザー決定「穴を先に塞ぐ→4段」の段1〜3、Sonnet実装・Opus監査、`d54780a8aa`、[規則](static-call-result-kind-2026-11-08.md) K1〜K14）: 呼び出しの結果の種類を、calleeの契約の行から1つの表（`LocalCallRouteV1::for_result`、`LocalCallResultClassV1::invoke_result`）で決め、instanceとbareの道がそれを読む（段1。329入力と1,459本のMIRは差0）。型つきの静的な実行が箱のHandleを運び、`initializer_class`にstaticの枝を足したので、`local a = Main.mk(5)`と`a.get()`が通る。scalarの判定は変えず、`Main.mk(5) + 1`は名前つきで拒否のまま。実装中にK13（受けたHandleはcall-prefixのgroupを持たない）とK14（箱の結果の行はconstructorの引数のi64の要求を持つ）を足した。production +269/−67、testは1本足した。監査: 1,459本の違い5本と329入力の違い2本は全部拒否から別の門、(c)は0。384組は前の121組が同じで21組が新しく通る。229入力・apps 100本・suite 8/8・Python v4は前と同じ、全libの新しい赤1本は前のseriesにも出た順序のflake。Opusが30形を流し、誤り0。止めた物: 引数つきのstaticが返すString（`home_local_call_flow.rs:158`のTextの観測の`assert!`と`physical_header.rs:91-110`のheaderの行）。残り: Handleの結果で引数が式の形（`source-route-identity`、i64の結果では通る）、段4（Textの引数なしのpacketを消す）。

段4（Sonnet、`7ab29d1121`、文書だけ）: 引数つきのstaticが返すStringを型つきの静的な実行に載せる事を3回試して止めた。Stringのlocalはrouteではなくinitializerのexpression frameのText residenceが持つ。引数なしの入力のprotocolはarity 0を課す。headerの行はscalarの結果だけに作られる（[規則](static-call-result-kind-2026-11-08.md) §5）。productionは変えていない。2026-11-08ユーザー決定「Stringを先に掘る」: 次は、i64とStringの引数のstaticのStringを受ける入口を作り、引数なし専用の道（R3）をその道の「実引数0個」の場合に畳む。

staticのStringの入口（Sonnet実装・Opus監査、`54181cfffd`、[規則](static-call-result-kind-2026-11-08.md) K15〜K22）: formalが全部i64のstaticが返すStringを、staticの入力のfinishのqualifiedのevidenceで受ける（案B。`uses`は無し）。受けたStringはinstanceや引数なしと同じText residenceが持つ。引数なし専用の道（R3）は「実引数0個」の場合に畳んで消した（staticの結果の道は6→5）。844行のfileはrouteの選択（388行）を分けて465行になった。移動を除くproductionは+45/−53。Opusの監査で、formalの使い方の証明を飛ばす条件がK21より広い（`Map`や`ExactText`のformalも含む）事を見つけ、直させた。329入力のMIRは差0。1,459本は前は拒否で今は通る2本だけ。384組は前の142組が同じで12組（`si`）が新しく通る。229入力・apps 100本・suite 8/8・Python v4は前と同じ。Opusが19形を流し、誤り0。止めた物: 宣言した`x: String`のformal（2本のlaneのABIで、呼び出し側のemitが無い）、受けた後のi64のstaticの呼び出し（Text residenceのfaultのunwind）。2026-11-08ユーザー決定「Stringの後、nullableの前」: 次は結果の種類を決める所を1つにするseries（Codexに渡す）。

結果の種類を決める所を1つに（Codex実装・Opus監査、`f60b74e371`、[規則](result-kind-ssot-2026-11-08.md) U1〜U34）: calleeの結果の種類は`CalleeResultKindCatalogV1`が1つ答え、lexical・direct・root・R1・staticのclaim・Unitがそれを読む。staticの契約の行、objectのclaim、instanceのscalar、遅く揃うTextとUnitとMapの証拠をこのcatalogに集め、正の証拠が食い違えば名前つきで止める。`construction_result_callee`と`nullable_exits_hold`は種類を決めず、可否だけを返す独立した検証として残した。結果の種類から物理の種類への写しは1つ（`invoke_result`）。K13は記録するownerの違いとして保ち、K14は箱のclassとconstructorの引数の要求を別の事実にした。受理は変えない: 329入力と1,459本のMIRは各段で差0、384組・229入力・apps 100本・suite 8/8も前と同じ。Opusが78形を流し、結果とMIRは全部同じ。production +280、test +37。残る注意（[Opusの確認](result-kind-ssot-2026-11-08.md)）: 証拠を作る所は別々のまま、Textなどの証拠は発行の途中で書き足される、`KnownValueClass`は包む形。次はnullable。

nullableのobjectの結果（Codex実装・Opus監査、`b4ba7283b6`、[規則](nullable-result-2026-11-08.md) N1〜N34）: instanceとstaticのfactoryが返す`new P(..)`か`null`をroot scopeのlocalで受け、`null`と比べ、methodを呼べる。種類はcatalogの`Nullable(P)`を読み、別の判定を作らない。`: P?`の宣言はobjectだけ・nullだけの本体も受ける。nullにmethodを呼ぶと、calleeに入る前に既存のCallのFault辺でTypeError（理由302）になり、取得済みのobjectは解放される。C側の新しい公開キー・opcode・ABIは無く、既存のinstanceの呼び出しの前にもnullの検査の分岐が1つ入る。早いreturnの後に受けたobjectを使う形は、非nullでも同じ規則で開いた（t02）。1,788入力のうち旧成功のMIRと出力の変化は0、前は拒否で今は通るのが4本、別の門へ移ったのが4本。384組・apps 100本・suite 8/8・Python v4は前と同じ。229入力は1本増えた。Opusの未見27形で誤りは0。production +230。残り: `a == void`（builderが`const void`を出す）、always-nullのfield getter、代入・alias・forward・分岐内の受け取り・配列・`String?`。次はS4のloop（共通CFG・PhiTxn）。

S4のloop（Codex実装・Opus監査、`aafc0a2abd`、[規則](s4-loop-2026-11-08.md) L1〜L73）: 関数ごとに`FunctionControlV1`が1つあり、PlanLowerer・Frag・If・Root・V2の各profileとloopのphysicalizerが、block・edge・Binding SSA・PHIをそこへ書く。`route_loop`はresolverの位置（`SourceStmtSiteV1`）を受け、構造の走査と同じ形のloopの曖昧拒否は消えた。消した物: `FragEmitSession`、`PlanBuildSession`、`phi_processing.rs`、`IncompletePhiV1`、V2 sessionの独自のCFGとPhiTxn、`canonical_control`の有無で分かれる枝。受理は変えない: 1,788本のMIRは同じ（拒否文の中のowner番号が2→1になった3本だけ、承認済み）、384組・229入力・apps 100本・suite 8/8・Python v4も前と同じ。Opusの未見482本も同じ。rustfmt後でproduction +505、test +348で、純減ではない（最初の報告の「−7」は詰めて書いた分を含んでいた。ユーザー決定「V2も寄せて最後にする」で着地）。全体実行の時だけの赤（correspondence）は検査の順を直して消えた。残り（[Opusの確認](s4-loop-2026-11-08.md)）: 前からある`PhiTxn::begin`（loop pointのmaterialized values・portable JoinSig）、薄いCFG委譲の記述量。次は基本式とfieldへ入れない`take`。

基本式の1回目（Codex実装・Opus監査、`d46b93c69b`、[規則](basic-expr-2026-11-09.md) B1〜B37）: i64の加算・減算の結果が、同じchecked評価からprint・local・returnへ届く（`local b = a + 1`→`print(b)`、`local c = 1 + 2`→`return c`）。fieldへ入れない`take`は、instanceとstaticの呼び出しで、受け取ったHomeを既存のEnd経路で全出口に終える。受理: 1,788本で前から通っていた物の変化0、新しく通った物16。Opusの未見20本・482本・全libの新しい赤0。rustfmt後でproduction +979、test +248。消した物: `select_static_scalar_local_routes_v1`、`return_expression_bindings_v1`、`emit_declared_return_read_v1`。残り（[Opusの確認](basic-expr-2026-11-09.md)）: 掛け算、式のreturn、Stringのmethod、Array・Map、fieldの比較、Birthに格納しない`take`、freshな`new`を渡す`take`。多くは、同じ操作のMIRの書き手が文脈ごとに分かれていて、寄せると前から通っていた入力のMIRが変わるので止めた。次は（ユーザー決定2026-11-09）MIRの書き手の整理（受理は変えず、操作ごとに書き手を1つにする）、その後で基本式の残り。

MIRの書き手の整理（Codex実装・Opus監査、`c3df413562`、[規則と記録](mir-writer-2026-11-09.md) W1〜W14）: 受理・拒否・1,788本のMIR・EXEはすべて不変。1回目は命令のliteralを包むだけの関数とReturnの同じ分岐が大半で、整形後production +268だったので、ユーザー決定で仕上げを挟み、W12（1つの命令の値を作るだけの関数は書き手ではない）〜W14で取り消した。残ったのは重複していた手順のまとまり4つ（Arrayのwrite・readの選び方、writeの後の観測とreceipt、Mapの読みのInvokeとNormalResult、object fieldのappend）。整形後production +54、test 0。基本式の残りの本当の壁は、軽い形のStringのlength（raw Call）をlifecycleのbackendが`call-not-birth`で拒否することと、Mapの契約表が`Unprojected`であること（2つの証拠の世界）。次は（ユーザー決定2026-11-09）正規化・共通化で綺麗になるかのChatGPT Proへの設計相談、その後で基本式の残り。

正規化の設計相談（2026-11-09、ChatGPT ProとGemini 4 Argon、Opusが根拠を確認、[記録](normalize-consult-2026-11-09.md)）: 揃えるのは命令の組み立て方ではなく、元の証拠・操作の法則・Faultと所有の約束。新しい正規形の型は作らず、既存の`CheckedExpressionEvaluationV1`等へ「Faultの枠が要るか」の問い合わせを足す。軽いNoSourceFault操作をlifecycleに通してから旧raw経路を退役し、常にInvokeにはしない。順番（ユーザー決定2026-11-09）: ①正規化の1回目（Faultが起きないscalarの式は枠なし、`TerminalI64AddReturnV1`と`TerminalI64FieldReturnV1`の専用一式を共通の評価器へ合流させて消す、純減で閉じる）→②Stringのlengthの専用の軽い操作（公開の形は調査後に決める）→③基本式の残り→④Mapの契約の投影→`edit`→3bと退役→徹底的な整理。

正規化の1回目（Codex実装・Opus監査、`a303a0cc1d`、[規則と記録](normalize-1-2026-11-09.md) N1〜N22）: 元の証拠でFaultが起きないと示せるscalarの式は、root以外の関数でFaultの枠なしで書く。`TerminalI64AddReturnV1`と`TerminalI64FieldReturnV1`の専用一式（5つのmodule、台帳の欄、hook、writer、dispatch）を、共通の評価器と既存のReturnの続きへ合流させて消した。1つのReturnの後始末は、既存のroot graphの組み立て役が1回だけ作る。整形後production −791行（`a74b40ba59`から）。1,788本で拒否の後退は0、85本のMIRが軽くなり、重くなった物は0。rootの枠はLifecycle V4の公開の約束なので残す。子のobjectのfieldを通るReturnの読みは、builderが受け付けて所有検証が拒否する（fieldの読みの正規化の回で直す）。次の順番（ユーザー決定2026-11-09の2回目、[記録](normalize-consult-2026-11-09.md)）: Stringのlength→fieldの読みの正規化（1つの要求・結果・物理の記述、証明のcallbackを1つの持ち主へ）→基本式の残り→Mapの契約→`edit`→3bと退役→徹底的な整理（terminalの先着順の連鎖を1つの振り分けへ）。

Stringのlength（Codex調査・ChatGPT Proに相談・Opusが根拠を確認、[記録](string-length-2026-11-10.md)）: 今EXEまで動くlengthは全部checkedのInvoke（45本）で、軽いrawの形（13本）は全部sourceの段で止まり、EXEに届く物は無かった。ユーザー決定（2026-11-10）: 非terminatorの`StringLength`（JSON `string_length`、runtime `nyash.string.code_point_count_v1`はframe無しでstatus 0か2）をlifecycleのprogram v2の語彙に足す。文法と意味（code pointの数）は変えない。旧checked length（`StringInvokeOperation::Length`、JSON、Cの受理、`checked_length_v1`）とsourceのlengthのrawの選択は同じseriesで消す。機能の追加と旧経路の整理は別に集計し、整理の分は純減で閉じる。最初の回にcache・loop全般・借りた引数の全経路は入れない。着地（Codex実装・Opus監査、`acebaf066b`、[規則と記録](string-length-op-2026-11-10.md)）: sourceのlengthは枠なしの`StringLength`の行になり、旧checked lengthのInvokeとexportは消えた。枠なしで書ける法則はArrayのcountと1つの判定を共有する。1,788本で後退0、38本のMIRが軽くなった。機能はproduction +468、整理は −43。13本はまだrawの`StringBox.length`のCallを出し、EXEの道では拒否される（ReturnのMethodCallのrootを共通の評価器が選ばないため。basic-exprの残りで直す）。次（ユーザー決定2026-11-10）: Faultが要らない操作の一覧の調査（文書だけ）。

Faultが要らない操作（Codex調査`78ca1419c9`・ChatGPT Proに相談・Opusが根拠を確認、[記録](no-fault-ops-2026-11-10.md)）: 候補は1引数の`indexOf`（5本）、exactなText同士の比較（27本、一番効く）、`Shared.Borrow`、専用`OwnedArray.Get`。調査の途中で、DynamicとStringの`==`で、仕様とissuerは「Stringでない正しい値は不等」なのに、runtimeはTypeErrorにする食い違いが見つかった。ユーザー決定（2026-11-10）: mixedの`==`の修正（再現を作ってからruntimeを仕様に合わせる）→ fieldの読みの正規化 → Textを2つ借りるhelperと`StringIndexOf`（旧checked IndexOfは同じseriesで消す）→ 比較の契約の精密化（exact String同士は結果がBoolだけでFaultなし、NonSuspending）と`StringCompare`（複製しないruntimeと一緒に）→ basic-exprの残り → Mapの契約。DynamicとStringの比較とnullの比較はcheckedの形に残す。`Shared.Borrow`と`OwnedArray.Get`は後回し、TextのEndはbasic-exprの頃に小さな設計調査。

mixedの`==`の修正（Codex実装・Opus監査、`297ea922b3`、[記録](mixed-equality-2026-11-10.md)）: runtimeの比較で、operandをText・null・それ以外の正しい値・壊れた入力に分けてから演算子を選ぶようにし、仕様どおりStringでない正しい値との`==`はfalse、`!=`はtrueにした。大小の比較はTypeErrorのまま。Integerのcarrierを使うsourceはEXEの前で止まるので、証拠はABIのtest。既存のEXEの出力の差分は0。次はfieldの読みの正規化。

fieldの読みの正規化（Codex実装・Opus監査、`097ba0fff4`、[記録](field-read-2026-11-10.md)）: fieldの証明のqueryを既存の`HomeFieldSourceConsultV1`1つに集め（`scan_new_home_flow`の`FnMut`は12→7）、要求・結果の重複wrapperと物理の再分類を消し、宣言の結果のscalar投影を1か所にした。整形後のproductionは整理で−669行（u16の機能+236行は別）。1,788本のMIRは全部同じ。所有の検証が、子のobjectのfieldを通る読み（`return n.leaf.value + 1`）の借用を実MIRと公開の宣言から証明するようになり、u16はexit 6。`local s = t.f`は閉じたまま。次は`StringIndexOf`。

徹底的な整理（ユーザー決定2026-11-09）: MirBuilderができたら徹底的に綺麗にする。時期は基本式の残り・`edit`・3bと旧経路の退役の後、`.hako` parserのparityとselfhostの前（前の「builderの調査」の枠）。数回の大きな回で行い、各回は実装者がsubagentで経路を上からたどる調査から始める。目標: 責務ごとに持ち主1つ、操作ごとにMIRの書き手1つ（W12〜W14の意味で数える）、形や文脈の名前のfamilyを作らない、退役した経路と互換のfallbackを残さない、productionのfileは800行以下、各回で整形後の行数を純減。既存のgateは保つ。

## 証明の作り（2026-11-06、ChatGPT Proの検討、Opusが引用を確認）

材料: 4つの形を端から端まで追うと、判定は25〜34個で、同じsourceの判断の出し直しと運搬が多い（呼び出しは所有が無いのに再導出10）。結果型I64は少なくとも10か所、field の格納の形は9か所以上で決め直している。所有をMIRの上で1passにする案は、10-21の判断どおり不十分（operationと辺ごとの取得済みprefix、Normal/Faultの移転が要る）。

判断の一元化と、物理MIRの検証の一般化は進める。ただし**source→MIRの対応検査は残す**。呼び出しを丸ごと消したMIRや、同じABIの別のcalleeへ取り違えたMIRは、所有の上では安全でもsourceどおりではない。

| 責務 | 判断すること |
| --- | --- |
| sourceの意味解析 | 何を評価し、誰から借り、どこで所有を移すか。事実ごとに最初の決定の場所が持ち、他は聞くだけ |
| source→MIRの対応検査 | 必要な評価、効果、順序、対象が生成物に保存されたか（例: `finalized_initializer_homes.rs:13`はpacketと命令が一緒に消えても検出する） |
| 公開MIRの独立検証 | 実CFGの上で型、借用、取得、移転、Endが整合するか。生成側の`Proof`型や判定関数を取り込まず、実MIRと公開宣言から状態を作る。loopと合流は固定点で解く |

- 同じsourceの判断を出し直す所は消す。生成物を調べて宣言や計画と一致するか確かめる所は消さない。receiptも、source siteと実命令を結ぶ役割なら運搬ではない。
- 今の検証の範囲: 呼び出しの結果型はCが確かめる（`hako_llvmc_ffi_lifecycle_v4_indexed_flow.inc:156`、`indexed_return.inc:117`）がRustは見ない（`return_outcome.rs:8`）。普通のobjectのEndはCが全関数で確かめるが、`ownership_ssa`は`move`、`share`、`take`のある関数だけ（`backend_capability.rs:105`）。`image.field_reads`は実MIRと照らし直している。
- 直す順: Cの必須検査を保つ → S2でRustに呼び出しの結果型の独立検査を足す → objectの資源の検証を全関数へ一般化する（結果の所有の契約、`initially_published`を公開の構築契約と実commitから照合する形へ）→ 対応検査との依存を整理する。
- Rustへ寄せること自体は独立性を生まない。Cの意味の検査を外すのは、新しい検証が同じ違反を実MIRから拒否し、公開の入力経路すべてで迂回できず、検証後の変換とJSON化の不整合も検出できると確かめた規則だけ。
- S2の最初の完了単位: 生きたobjectのHomeを持つcallerで、型付き呼び出しの結果を別の呼び出しの引数に使う。正規EXE、Normal・後の引数のFault・callee入場後のFaultのEnd、MIR改変の拒否（結果型、Fault側のEnd欠落、二重End。呼び出しの欠落は対応検査）、対象のsource判断の出し手をn個から1個へ、置き換えた旧edgeのcaller 0。削除した責務と追加した責務は分けて報告する。

## 流れの作り直し（2026-11-06、ユーザー承認、ChatGPT Proの検討）

調査とProの意見は[経路の選び方の調査](mirbuilder-route-selection-survey-2026-11-06.md)。構文の種類で分ける所は既に一か所ずつで、問題はその下で「誰が受け持つか」を先着の連鎖や形ごとの観測で選び、状態の更新と混ぜていること。承認済みの順番は変えず、S2bからS4を作り直しの段階として使う。

目標の流れ: 型と宣言の契約を確定し、sourceの意図（明示のmove/share/take/copy、Binding・Place・source site、スコープ、cleanupの登録順、評価順、commitの契約）を保持した未公開の関数CFGを作る → そのCFGの上のdataflow（loopは不動点）で、使えるbinding、生きた所有の責任・移転待ち・移転済み、借用、各Normal/Fault辺のEndを求め、既存のcleanupの部品でEndを具体化する → cleanupのFault辺まで含めて既存のCFG・PHIの持ち主（`CanonicalCfgSessionV1`、`PhiTxn`）で確定する → source対応検査と公開MIRの独立検証。sealを先に閉じない。独立のverifierをEndの生成器と兼ねない。

| 段階 | 中身 | 完了の条件 |
| --- | --- | --- |
| S2b | 一つの呼び出しに実行の担当を一つ。構造上の位置を`ExpressionPlanRootV1`で一度決め、誰が処理するかを別に一度だけ決めて呼び出しの観測が持つ。冪等なroute記録をその選択へ統合する。対象範囲の先着のemitの連鎖を消す | Assignmentや条件への呼び出しが位置ごとの新しいproducerなしで動き、選択と実行が一回、旧判定が同じseriesで消える。表で旧い連鎖を包んだだけなら未完了 |
| S3 | 型と演算の契約を共通にする（値の種類の再判定をなくす、演算子の表を一つ）。呼び出しと分岐の所有の状態とFaultのcleanupを、意図を保持したCFGの解析へ移す最初の責務を一つ | 新しい使い先をつないだ時に、新しい分類器とcleanupの持ち主が増えない |
| S4 | loopを既存の`operations + inputs`、CFG、JoinSig、`PhiTxn`へまとめ、call・Fault・Text/Dynamicの値・借用・cleanupを共通の操作契約に移して、family別のincomingの計算と入口を消す。`route_loop`にはresolverのsiteを入口から渡す | 移した範囲のfamily別の計算とlowering（`PlanLowerer`と`loop_recipe_physicalizer`の重なり）が消える |
| `edit`とADRの残り | 同じPlace・所有の状態・commit・cleanupの仕組みに足す。形ごとの観測に埋め込まない | |

移行中の決まり: 同じpipelineの中で効果を発行する前に担当を一意に決める、新旧の範囲を重ねない、選択後の失敗はエラーで再選択しない、移した範囲の旧判定と専有adapterを同じseriesで消す（[in-place policy](../design/mirbuilder-inplace-replacement-policy-ssot.md)の「bounded branch」）。先着の連鎖を「全部に聞いて2つなら拒否」に単純に変えない（優先順で通っていた入力を落としうる。優先が要るならまず明示の選択規則にする）。builderの調査は3bの後まで待たず、触る所ごとに調べる・置き換える・消すを一組で行う。規模は、呼び出しとloopをまとめた時に実際に減った量を測ってから見積もる。

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
`init { fields }` のsyntax退役は[field-surface owner](../design/box-member-field-method-surface-ssot.md#legacy-stored-slot-list-retirement-follow-up)
の既存文書内rowへ登録。typed化・native配置とは分離し、現在のacceptance入力は変えない。

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
