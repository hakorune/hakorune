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
