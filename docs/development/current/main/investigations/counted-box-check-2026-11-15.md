# 数えるBoxの確認1: 操作・共通式・検証の設計案

Status: **操作名・Normal/Fault契約をユーザーが承認（2026-11-15）: Unit Retain、overflow は Fault、Release が減算と最後の End を持つ。段 F の Return の許可境界は未確認のまま。段 B は統合済み、C0 以降は未実装。**
Date: 2026-11-15
Base: main `25e9d39bd0`。根拠: [ADR-003](../../../adr/adr-003-counted-box-default.md)。
Scope: 段Aの設計案と段Bの物理Endだけ。C0〜G・既定切替・文法変更は実施しない。

## 1. 操作案（ユーザーが確認する表）

既存`InvokeOperation`に`CountedBox`群を追加する案。トップレベルopcode、
Shared cell、runtime sidecar、別のEnd writerは追加しない。

| 操作案 | 値・型の情報 | Normalで消費/取得 | Faultで消費/取得 |
|---|---|---|---|
| `CountedBox::Retain` | `object: CanonicalObjectIdV1`, `value: ValueId`, 元のnullability | 元を消費しない。`value`の独立した持分tokenを1つ取得 | 元を保持。新tokenは取得しない |
| `CountedBox::Release` | 同じ型付きvalue、既存`ObjectEndStepV1`のcomplete-End計画 | 入力tokenを1回消費。減算が0の時だけ中身のEnd・外側解放 | 入力tokenは消費済み。新取得なし。残りのcleanupを継続 |

nullの場合は空の持分slotで、加減算もEndもしない。nullabilityは元producer/field/
parameter契約へ照合し、無条件nullable化で誤ったnullを許可しない。
CanonicalObjectIdはmodule内の元型/layout座標であり、実objectや持分の同一性ではない。
新しい公開descriptorは設けず、既存の型・End planとの照合を使う。

**RetainはUnit形式を第一案とする。** 共通式評価が既存Copyで別SSA参照slotを作り、
RetainはそのslotのNormal辺にtokenを発行する。pointerそのものを作り直さず、
既存producer/Copy chainを真の値由来として保持できる。Copyだけではtokenを増やさない。
同じ実行経路でlive tokenがあるslotへの二重Retainは拒否し、別の持分には別のSSA slotを使う。
loopの別iterationや排他的なincoming辺を、静的な同slot二回として一律に拒否しない。
Fault時にも参照のbitは存在し得るが、取得tokenは存在しない。
pointer SSAの定義と辺ごとのtoken生存状態を分離し、Copy直後からglobal Ownedにはしない。
Retain時点では元borrowのsupportが有効で、途中のcall等でfieldが無効化されていない事を検査する。
Retain完了後の持分tokenは、元field/supportの破棄から独立する。
これは新しい`InvokeNormalResult`/返却originを先行追加しないための案で、未実装。

別案はRetainのNormal結果ValueIdを新tokenにする形。通常SSAとの対応は素直だが、
現在のCopy-only return transportに乗らず、追加の公開形式を要する可能性が高い。
この分岐はユーザー確認事項とし、どちらも勝手に実装しない。

**一般的なRetainをnonfaultとはしない。** u64 overflowを不可能と証明していないため、
overflow時は加算せずFault、新tokenなしを推奨する。wrapping加算は禁止。
上限が証明できた場所だけのNormal-only化は後の最適化であり、今回行わない。

**減算と最後のEndのownerはReleaseの一つ。** 既存`ObjectEndIfOwned`はFresh outcomeだけを
破棄し、Null/Forwardを消費しない。field由来のretain tokenをFreshへ偽装してはならない。
Releaseの後にObjectEndIfOwnedを別発行しない。物理の既存complete-End helperへ委譲する時は、
減算をそのhelper入口に一度だけ置き、呼出し側で先に減らさない。

根拠: `src/mir/instruction/invoke_shared.rs:18-27`の辺ごとの契約、
`instruction/invoke.rs:442-448`の既存条件付きEnd。Shared操作の実体は使わない。

## 2. 既存ownerを上からつなぐ導線

以下のsource pathは`src/mir/`相対、Cは`lang/c-abi/shims/published_mir/`相対。
行は基点の読み取り時点。worker3本と主担当で要所を確認した。

| 責務 | 現在の入口 | 案での役割 |
|---|---|---|
| 元の式・class | `resolved_value_profile/expression_class.rs:8-19`、`expression_projection.rs:25-44` | 元型・nullability・sourceを一度保持 |
| 共通評価 | `resolved_semantics/home_expression_value_preparation.rs:4-77` → `home_expression_qualification.rs:31-153` | 値、型、所有data、取得順序、各出口の義務を構成 |
| 責任遷移 | `home_expression_ownership.rs:34-140` | 既存ownership stateでtokenを取得/移転/消費 |
| 一時値 | `home_expression_evaluation.rs:363-375` | String/DynamicにBox tokenを加える |
| sole lowering | `builder/ordinary_new_admission/selected/expression_value.rs:213` `emit_body_with_prefix` | 同じ論理nodeを一度下降し、場所ごとに再判定しない |
| 完了観測 | `normal_callable_semantic_package/ordinary_new_local_commit/root_call_entry/expression_emission.rs:11-85` | 同じbody/value/tokenを行き先へ渡す |
| cleanup | `selected/fault_cleanup_plan.rs:35-124`、`selected/root_graph.rs:224,327` | 同じ順序付き義務を既存Normal/Fault writerへ |
| scope出口 | `home_prefix_source_loop_ports.rs:549-584` | scopeに移された同じtoken義務を各exitへ投影 |
| 独立検証 | `ownership_ssa/verify.rs:30`、`backend_capability.rs:105-131` | 公開MIR・実CFGからtoken収支を検証 |

新しい解析driver・source receipt owner・cleanup writerは設けない。
新しい内部dataと、既存owner間の受け渡しは必要。既存の独立検証は保持する。

## 3. 持分の独立検証

runtimeの数は読まない。実object identityと、各持分のSSA slot/tokenを分離する。
同じaddressへRetainを2回しても別slotの別token。`abi.aliases`によるCopy正規化で
二つのtokenを元の1個へ潰さない（現行`ownership_ssa/classify.rs:112-159`）。

- New/Birth完了や所有Call結果からの既存所有1、Retain Normalの取得、明示移転を識別。
- Retain前のread/acquisition Faultにはtokenなし。後続引数Faultでは取得済prefixだけを逆順Release。
- dereferenceには生存tokenか、有効なsupportからの借りを要求。Copyは所有を複製しない。
- Release・保存先への移転・Return・scope終了の各辺で、同じtokenをちょうど1回消費する。
- PHIは各incoming辺のtokenを1つ移す。PHIだけで+1しない。同型/layoutの別起源tokenも
  各辺から同じPHI slotへ対応させる。Sharedの「全incomingが同じtoken」制限をそのまま流用しない。
- loopは一時tokenを各iterationの全Normal/Fault出口で清算し、明示carrierだけbackedgeへ移す。
  合流stateを一致させ、iterationごとの増殖・欠落を拒否する。
- Nullableは空/非空の条件付きslotとして同じ辺の義務を追う。nullに所有を捏造しない。

現行`verify.rs:18-23,278-303,534-550`のUnpublished/Published/BirthFaultedは維持する。
Retain/ReleaseはPublished参照が対象。未公開parentは既存construction prefix cleanupと
ReclaimUnpublishedのまま。移転済childをparentの再走査でもう一度解放しない。

`published_homes.rs:54-90`は現在NoHomes/OutsideModelへ出る場合がある。
CountedBox操作が一つでもある関数は、そこへ逃がさず**token検証を必須化**する。
未対応のmanaged命令・返却・CFGを発見したら名前付き拒否。既存validatorを無効化しない。

拒否する変異: Retain削除（Copyへの置換を含む）、Release削除、二重Release、取得前Release、
Fault辺のRelease欠落、Normal取得をFault辺で使う改変、同tokenを二つのPHIへ移す改変、
Retainなしの所有保存/Return。全実行辺のexact-onceと使用時の生存を検査する。

## 4. 共通の式の結果と、位置別判定の退役

```text
式の結果 = (値, class, 元型/layout/nullability, 所有data, 由来, ordered temporary)
所有data = Owned(token) | Borrowed(support)
由来     = Fresh | Local | Field | Call | ...
文       = 式の評価 + 行き先(Bind/Rebind/Return/Branch/Pass/Store/Discard)
```

FreshはOwned/Borrowedと別軸。freshも所有値であり、移転の根拠になる由来。
数える方針はownership policyとtoken操作への投影に閉じ、各位置のcodeへ埋め込まない。
ADR-002へ戻っても、この型付き参照と借り/所有のdataは再利用する。

現行classはI64/String/Dynamic/Bool/Null/Opaqueだけ。一時値もString/Dynamicだけ。
ただし`root_call_entry/expression_read_value.rs:10-15,40-88`には既にDeclaredReadがあり、
非所有Alias/DeclaredFromが共通writerを通る。「Boxは一切通らない」という調査の表現は強すぎる。
不足は**型・所有区分・取得/解放義務を伴うBox参照**。OpaqueやNormalIntegerへ推測で落とさない。
`home_expression_leaves.rs:433-448`、`expression_value.rs:458-465,550-566`のfield投影は
Box-aware value dataへ置換し、元のsource/field/nullability検査は残す。

候補B（式の台帳）を採用し、Bind時に同じtoken義務を既存scope ownership stateへ移す
候補Cで補完する。CはFieldAliasを列挙してEndを足すだけではない。
FieldAliasは現在Home/releaseを負わないため、一律LiveHome化する候補Aは採らない。
String/Dynamic/Boxの並列台帳を増やさず、型付きの一つのordered temporaryへ寄せる。

| C0で共通dataが先に取る対象 | 同じ回で削除する候補 | 粗い削除見込み |
|---|---|---:|
| ordered temporary | acquired/dynamic_carriers/temporary_orderの並列突合せ | 15-30行 |
| Faultの義務操作 | owed_operationsの種類別再構成 | 20-40行 |
| 条件のtemporary | bool_condition_temporary_operationsの重複再分類 | 35-70行 |
| deferred temporary | 各consumerのdeferred-value分類 | 20-45行 |

C0は受理・MIR不変。移動だけを削減と数えず、source identity/順序/独立照合は残す。
値dataを広げただけで全位置を開くのではなく、共通が既存範囲を実際に取った分類を同じ回で消す。

`is_scalar_return_source_v1`（`qualified_static_incoming_source.rs:70-83`）は
Return根・元Call先頭という位置/形の判定。5消費点はterminal scalar、local source scalar、
static input selection、borrowed call result sourceの2点。**3cで削除する対象**であり、
C0で単に外して受理を変えてはならない。
共通Call nodeが元target/actuals/結果契約を一度保持し、そのI64結果を既存Add等へ渡せば、
`return Main.sq(2) + 1`も行き先だけになる。その接続と同時にpredicateと5利用点を削除する。
穴ごとのReturn producerや新たな特例は作らない。legacy scalar/Return連鎖は共通が取った範囲だけ退役。

file分割案: `expression_value.rs`は現777行。共通model/front、順序付きbody下降、exact readの
予約/commitを責務で分け、既存expression_value familyへ再編する。形別fileは増やさない。
`home_new_prefix_terminal.rs`は800行。Box分岐を追加せず、共通評価を先に取り、行き先適用・
責任移転・exit snapshotへ絞る。classifierを別fileへ丸ごと移すだけでは整理と数えない。
`home_expression_evaluation.rs`730行、cleanup696行も同じ責務境界で超過前に再編する。

## 5. 物理形式、Returnの許可境界、実験から移せる物

新操作のJSONは既存Invokeのoperation内に型付きRetain/Releaseとobject/value/nullability/
既存End planを載せる案。sourceの型所属と物理layoutを再照合し、C flowも同じtokenの
Normal取得・全辺消費を独立に確認する。新しい外部runtime ABI/descriptorは前提にしない。
+1/-1はdirect headerへの非原子inline load/add-or-sub/store/zero判定としてemitする。

**Returnは未接続の重要境界。** 現在のowned結果はFresh/OwnedCall/(NullableのNull)だけ。
`finalized_object_return_outcomes.rs:146-151`と`owned.rs:76-146`、
`physical_program_json_object_return.rs:56-117`、C `indexed_object_return_origin.inc:39-76`が
それぞれNewBox/ordinary_callの実producerを厳密照合する。RetainをFresh/OwnedCallへ偽装しない。

Unit Retain案なら、真のFieldBorrow Normal値→既存Copyの別slot→Retain Normal取得→Returnを
表せる見込み。値由来はborrowed_receiver_fieldと純Copy chainのまま保持する。
ただし現在のrole×origin固定条件を消すだけでは不可で、元のfield/support/支配の検証と、
Retainで得た同tokenを返す検証を結合しなければならない。
これは**承認された新操作の効果として許可できるか、Fの前にユーザーが確認する分岐**。
新origin kind/ABIが不可避とは確認できていないため、今回それを前提に追加しない。
実装時に新しい公開形式/意味契約が別途必要と判明したら、その部分を止めて報告する。
確認せずFが既存の検証だけで閉じると約束しない。C0〜Gのcodeは今回書かない。

実験の40-byte header時分割・zero前の非破壊・反復postorderは段Bへ移せる。
rc_handのfi/bb/value表、callee側一律保持、singleton sidecar、実験env、計数hookは移さない。
現在のADRはreceiverとcaller保持を明記し、比較相手Rn-argの理由も定義済み。
古いSonnet報告の「引数規則/比較相手が未定」を、そのまま現状の未決へ転載しない。

## 6. C0〜Gの範囲と見込み（未実装の推定）

| 段 | 主な変更family | 追加 / 置換削除の見込み |
|---|---|---|
| C0 | expression class/projection、home_expression群、evaluated value/ExpressionEmission、共通temporary/cleanup | +300-600 / -90-185行、10-14file。受理/MIR不変 |
| C | construction emission/validation、field RHS、ObjectFieldSet/物理flow | +150-300 / -30-80行、5-8file。birth Normal commitのみ |
| D | field source/leaves、共通temporary/cleanup、ownership_ssa/物理検証 | +250-450 / -60-140行、7-11file。条件/receiverと取得前Fault |
| E | argument/receiver、root_call_entry、同FaultCleanupPlan | +200-400 / -80-180行、6-10file。caller保持、calleeは数えない |
| F | terminal destination、object return/ExpressionEmission、Rust/Cの由来+token結合検証 | +150-300 / -100-220行、5-8file。前節の許可境界がある |
| G | field local/scope出口、source_loop_cleanup、共有input/独立検証/観測 | +180-350 / -30-80行、5-8file。全出口、共有を動的確認 |

同じfileが重なるためfile数を合算しない。test追加量は別で、既存行列へ独立した意味の証拠だけを統合。
Sonnetの+900-1500行はC0のBox value/所有dataとscope橋渡しの費用が薄い。
この見込みも実装量/純減の保証ではない。段ごとに同じ受理・検証と実際に消えた関数で判断する。

## 7. 今の意味との差と、段Bの限定

`home_call_entry_transfer.rs:137-141`はbare new引数のHomeを暗黙に消費する。
TreeNode入力の`make`もlocal left/rightを`new TreeNode(left,right,value)`へ渡す。
ADR-003の「元localも持ち続ける」切替は意味変更なので、段Bで行わない。
C以降ではlocal由来とfresh由来を値dataで区別し、初期化済slotの上書きは拒否のままにする。

段Bはallocのcount=1と、outer release/full End root/childの減算・zero判定だけ。
zero前にchild headerへparent/cursorを書かず、pop時には再減算しない。
header16、TreeNode40、field offset/descriptor/JSON、source・MIR・End順序は変えない。
non-directは`lv4_direct_layout`の別分岐。directの選択は
`function/object_definition.rs:97-122`がno-hookとweak/Array無しを要求するため、fini/weak型は変えない。

将来count>1にした時は、明示field cleanupを先に実行する旧outer-release経路へそのまま流さない。
新Releaseのfull-Endがzero判定から中身まで一体で所有する。段Bはcount=1の動作保存だけを検証し、
これで共有・fini・cycle・自動配置全体が完成したとは主張しない。

計数時はzero判定直後を観測する。時分割後/free時のword0はparentであり、RCではない。
計数専用の外部IRコピーで観測し、本番emitterへ実験hook/envを入れない。

## 8. C0: main 047f764b21での再追跡と実装範囲（2026-11-15）

操作案1〜4はユーザー承認済み。Returnのrole×origin境界は未承認のまま。
この回は受理・成功MIR不変。Retain/Releaseは追加しない。

| 現在の導線（read-only workerで確認） | C0で引き取る責務 / 残す照合 |
|---|---|
| `home_scalar_operation_preflight.rs:59-107` → `home_initializer_expression.rs:415-475` | Aliasの元class/rootを捨てず保持。既存alias/nullの元binding・source・順序を保持 |
| `home_expression_evaluation.rs:381-510,544-608` → `home_expression_leaves.rs:433-448` | 型付きBox参照classとsource由来をdata化。所有/借りは別軸、Opaque/Integerへ推測しない |
| `root_call_entry/expression_read_value.rs:40-88` → `selected/expression_value.rs:458-465,550-566` | DeclaredReadを型付き参照dataへ。exact read/definition/source/field/nullabilityの検査を保持 |
| `selected/expression_value.rs:44-59,65-111` → `expression_value/invoke.rs:16-68` | acquired/carriers/orderの並列記録を一つのtyped ordered temporaryへ。取得Normalで一度記録 |
| `fault_cleanup_plan.rs:35-63` → `expression_value/cleanup.rs:4-140` | 同じ台帳から逆順Endを投影。条件の選択node/packet/lifecycle/Core/Sub根拠は保持 |
| `expression_value/cleanup.rs:318-353` → deferred/transient prefix | source temporaryとpurpose取得は別のphase。同じnodeをキーに上書きしない。Dynamic延期は終了済みの子を復活させない |
| `root_call_entry/expression_acquisition_paths.rs` → terminal cleanup | 公開命令からの独立照合なので生成側の重複として削除しない |

同じ回で消す候補（移動行は削減に数えない）:

- `FaultCleanupPlanV1::owed_operations`のnode→String/Carrierの二重探索とcoverage再構成。
- `ordered_owned_temporaries_v1`のvaluesからの種類再分類。台帳の同じ順序を借りる。
- `bool_condition_temporary_operations_v1`の種類別End BTreeMapと順序再構成。
- `emit_string_value`のacquired.len前後比較＋別order追記。取得をtyped entry一回にする。
- terminalのscalar/Text評価後の重複登録。分類優先順位を保ち、共通評価の後で一度登録。

型と台帳は既存familyに置く。Boxの型/由来/借りsupportは元sourceの証拠を保持し、
SSA pointer定義とOwned tokenの辺ごとの生存を混同しない。Box-owned entryは表現のみ、
C0では生成しない。Endを要求されたら未接続の名前付き拒否とし、Fresh/Sharedへ偽装しない。

file責務の再編（新owner/driver/writerは追加しない）:

- expression_value: 結果/台帳model、exact-read予約・commit、既存の順序付き下降。
- home_new_prefix_terminal: 既存分類順を維持し、責任移転・exit snapshot適用を分離。
- home_expression_evaluation: frame、参照のseed/照合、資格判定を既存family内で分離。
- cleanup: 一時値の照合/移転、既存continuation graph適用、inline記録を責務で分離。

数え方の基準: source graph owner=1、checked評価record=1、physical評価body=1。
purpose wrapperと完成観測projectionは別欄で記録し、owner水増しに使わない。
body内temporary保存列は3→1、既存typed enumは1→1の拡張。
生成側の種類別再構築は上記3関数を同じ範囲で前後比較する。

volatileは現productionでは不要と判断。directはno-hook/非weak/非Array閉包で、
load/sub/store/zero間に外部callがなく、再入・thread共有の保証にもならない。
通常load/storeへ外し、数え上げ・Fault/free順序・機械語とA40交互12回を確認する。
基準の結果が変わったら進めず報告する。

C0実装の形: 既存EvaluatedExpressionValueにBoxReferenceとNullを分離して載せ、
Boxの所有dataはBorrowed(元support) / Owned(後続段のSSA-slot token)と由来を保持する。
元Alias/from/field証拠を参照し、canonical payloadの証拠が無い場合はlayoutをNoneのままにする。
宣言名からcanonical IDを作らない。Box-owned temporaryの取得はまだ無く、End投影は名前付き未接続拒否。
一つのsource順序台帳をFault/Normal/延期へ投影し、callbackの既存String-only契約には派生viewを渡す。
Dynamic rootの移転・materializeの別phase・独立した公開命令照合を保持した。
ReadInputsはexact readの束を渡すdataで、expression body/graph/cleanupのownerではない。
条件のpacket/source照合は維持し、重複Dynamic packetも旧境界と同じく拒否する。
既存test-only field source getterは、同じ不変descriptorを返すproduction投影へ改名して再利用した。
800行だったargumentsは、構築後の既存cleanup適用を責務で分離した（移動は削減に数えない）。

## 9. 段C: field格納・操作・独立検証の接続計画（2026-11-15）

基点main `5d2c409940`。実装前にread-only worker3名と入口から再確認した。
Cだけを実装し、D/E/Fの受理や返却契約は広げない。新しいowner/driver/writerは作らない。

| 責務 | 元のownerと接続する道 |
|---|---|
| 資格・元型 | `home_field_write_expression.rs:95-125`だけではbirthに届かない。birthは`ConstructionStoreRhsV1::Parameter`のsite/binding/provided childとformal契約を持つ。既存qualificationのseedへ元証拠を運び、別classifierを作らない |
| 持ち分の取得 | `expression_value.rs`の共通下降と`expression_value/model.rs`のBoxReference/typed temporary。Copyはpointer輸送のみ、RetainのNormalで別slotのtokenを取得 |
| Release投影 | 同modelのBox End投影→`FaultCleanupPlanV1`→既存root graph。Releaseが既存complete Endを1回持ち、ObjectEndIfOwnedを後置しない |
| source/物理照合 | construction stateの元storeと`expression_source_values.rs`、公開後の`verification/invoke.rs`/`ownership_ssa`、C indexed flow。生成側の証拠を独立検証の代用にしない |
| 行き先への移転 | `normal_callable_construction_state/emission.rs:11,43-56,509-537`が実際のbirth store。local保持は`home_new_prefix_new_local.rs:84,190-222`と`home_call_entry_transfer.rs:137-141`の元Home移転にも関係する |
| 操作・検証 | Invoke群→physical ABI/JSON→C schema/indexed flow/inline emit。`ownership_ssa::verify`の同worklistにtokenのNormal取得/両辺消費を接続し、Countedを含むBirthもNoHomes/OutsideModelへ逃がさない |

意味が変わる見込み: direct対象のbare constructorへlocalを渡した後も元localを保持する。
既存の暗黙moveと異なり、保存先の持ち分と元localのscope Endが別責任になる。
対象入力とEnd回数/順の差を全件比較して報告し、意味差を未分類のままmainへ統合しない。
明示move/take・fresh直接格納・null・未初期化parentのFault cleanupとの境界を保持する。
旧ObjectFieldSetはFault時のin-flight childをframeが片付ける契約なので、
Retain取得前Fault・取得後のstore Fault・Normal commitを同じtokenとして混同しない。

配置の選択は現在`normal_default_pipeline.rs:600,650-654`のMIR完成後にある。
`object_definition.rs:97-123`のno-hook/非weak/非Array/child閉包検査がdirect境界の正本。
既存選択ownerで一度解決したcanonical選択を上流の共通計画へ運ぶ必要がある。
source側でenvを再parse、型名の特例、完成MIRから元Homeを復活させる処理は入れない。

Copyのpointer originと持ち分tokenは別軸。Rustのabi.aliasesとC indexのalias_rootへ
Retain先slotを潰さず、取得はNormal辺だけ、Releaseは両辺で一度消費する。
Retain削除/Copy置換、Release欠落/二重/取得前、Fault片付け欠落、Normal取得のFault使用を
既存の独立検証で拒否させる。nullabilityと完全Endを元型/producerへ照合する。
Releaseの物理投影は既存`hako_lifecycle_end_ID`→`hako_direct_end`へ一度委譲する。
Retainはnull noop、u64上限なら加算前Fault、通常load/add/store（非原子、非volatile）。

## 10. 段C試作の停止（2026-11-15）

段Cは未達。field格納の接続だけで、操作・独立検証・汎用modelを除いた
実在差分が整形後+426行（未整形+222行）になり、指示書の+150〜300行を超えた。
両側を同じrustfmtで整形して既存の整形差を相殺した値。まだbirthの物理格納と
独立検証が未接続なので、これを完成や性能の証拠にしない。指示書の規模停止条件を適用した。
試作はscratchへpatchと新規fileを保存し、productionは基点へ戻した。実装commitは無い。

再開時は、direct選択の共通計画への伝達、birth元storeの共通評価への投影、
callerの暗黙move抑制を一つの既存計画で運ぶ形を先に整理する。
Rustの独立検証は同worklistへborrowed formal/Normal token/格納済fieldの責任を接続する。
Cはplain/takeをparameterの表現だけで区別できないが、新公開ABIが必要とは未確定。
callee全出口とcallerの残る義務を照合し、Retain削除を元localの二重消費として拒否する道がある。
Retain済fieldは旧caller-owned markerではなく実所有としてFault dischargeへ渡す必要がある。
D/E/Fは未着手。停止報告に未検証項目と試作の再現情報をまとめる。

## 11. 段Cの再設計案（2026-11-15、文書のみ・選択はユーザー待ち）

基点main `7e86e9c6f7`とrepoの外に保存した試作を、read-only worker3名と比較した。
productionは変更していない。以下の実測は退避試作の会計、見込みは未実装の設計値である。
**推奨は、C1（操作＋独立検証）→C2（ウ: 条件・receiverのfield読み）を同じseriesで閉じる案。**
格納側は後から接続し、当面は既存constructorの暗黙moveを保持する。
これはADR-003の本採用でも、Rn-arg一致条件の達成でもない。順序変更にはユーザー判断が要る。

### 11.1 +426行は何だったか

old/new両方を同じrustfmtで整形し、既存書式の展開差を相殺した相互排他的な純増。

| 役目 | 整形後の実測 | 主な試作の変更family |
|---|---:|---|
| canonical direct選択をMIR完成前へ運ぶ | 89 | normal_default_pipeline、object_definition、constructor batch、issuer |
| callerの暗黙moveを止める契約の伝達 | 73 | constructor formal需要、field-source consult、Home walk |
| constructorから共通frameを作り保持 | 95 | ConstructionPlan/execution、source-store finish |
| 元formal証拠の再包装・read adapter | 129 | ConstructionParameterRead、Reference、typed value、exact entry read |
| 共通評価入口・owned候補登録 | 40 | qualification、plan、re-export |
| 合計 | **426** | 操作・汎用model・独立検証は別枠 |

実際のRetain/Releaseは別枠+415側。field Normalへのtoken移転は未実装であり、
426行を「完成した数え上げ」とは数えない。40行は取得需要の準備で、物理取得そのものではない。
129行全部を無駄な再証明として消す事もできない。元source保持・境界照合を含む。

確認できた重複は、既存writerが持つsource graphの再発行。
`ordinary_new_field_write_claim.rs:118,353,439`は既に元inventory/indexと同じRcのgraphを持つが、
試作の`instance_construction_body.rs:474-497`はassignmentからgraphを再発行した。
その準備と再発行は整形後10行。代替queryにも費用があり、純減10行の証明ではない。
formal需要にもsite→candidate→source→constructor→formalの再探索が増えていた。
これらの再包装をまとめる純減50〜100行は**見込み**であり、129行全削除や300行以下を保証しない。

### 11.2 最初の使い手の比較

見込みはC1を除くsource接続のproduction純増。family数は責務群の概数でfile数ではない。

| 案 | 変える責務群・見込み | 既存成功入力への影響 | 先に必要な検証 |
|---|---|---|---|
| ア: 後で使うlocalだけ保持 | 約7〜10群、+500〜1000行。うち後続use/必要寿命解析だけでも+150〜350以上の不確実性 | 旧moveを保つだけなら既存挙動は維持。ただし新しい後続use判定は受理/意味の規則になる | 全CFGの後続use・alias・loopと必要寿命、意味上のEnd/Fault順、elision時のoverflow不発の証明 |
| イ: 格納で常に+1、localはscope末尾−1 | 約6〜9群、+350〜650行 | 旧bare引数のmoveがkeepへ変わる。Endの回数/順とOverflow Fault辺を意図的に変更 | sourceとplain/take、caller保持、Retain token、格納済field、Birth部分Faultの責任 |
| ウ: fieldの読む側を先にする | 約4〜6群、+180〜350行 | constructorは不変。選択directの読取MIRに一時tokenとoverflow Fault辺を追加 | 元field/supportが取得まで有効、取得Normalのみtoken、式末尾/Fault prefixで逆順Release |

現在のHome flowは前向きの消費と同一式の順序を持つだけで、関数全体のlast-use解析ではない
（`home_expression_ownership.rs:32,93`、`home_call_entry_transfer.rs:137`）。
アの「既存互換moveを当面維持」と「RC意味論に対する最適化」は分ける。
後のreadが無い事だけでは、親が先に消える時の元localの寿命・fini順・weakからの観測を守れない。
無印move elisionは、元の持分終端・fini/weak・全Fault経路・overflowの観測を変えない証明が必要。
no-hook/nonweakのdirect閉包は今回の限定であり、一般の許可ではない。
ADR-003:13の「足すか移す」だけを根拠に、:97,103のlocal保持契約をlast-useへ変更しない。
アなら木のstore対は0となり、確認1のRn-arg一致条件も変更判断が要る。
ウの途中比較はRb-arg相当の参考値にとどめ、格納をつなぐ前にRn-arg達成と数えない。

### 11.3 C1とC2を分ける条件と費用

C1は、sourceの受理を変えず、既存test内の手組みMIRを本物serializerとCのEXE経路へ渡す。
`ownership_ssa/tests.rs:17,45` → `physical_program_json_composite_tests.rs:117`のencoder →
`lang/c-abi/tests/published_lifecycle_v4_execution_test.py:29-54`のdriver/link/executeを再利用する。
test用にproductionのsource publicationを迂回する入口は作らない。操作だけ別のJSONに手書きし直さない。

Retain削除/Copy置換、Release欠落/二重/取得前、Fault片付け欠落、Normal tokenのFault使用を
Rust/C両方で拒否し、null・overflow・最後のReleaseのEXEを確認する。
Counted関数は`published_homes.rs:54-90`のNoHomes/OutsideModelやBirth除外へ逃がさない。
未対応のmanaged形は名前付き拒否。source計画を独立検証の代わりにしない。

別枠+415行のうち、再利用候補材料は119行（操作model/delegate62、physical17、schema13、
C emit/index/helper18、構造guard9）。C flow68行は未完成、残り228行はmodel/End/source接続等。
Rust token検証は試作に無く、Releaseのnull処理も未完。119行も動作確認済み部品ではない。
C1はproduction **+450〜850行**、test **+150〜350行**の見込み。415行で完成すると見積もらない。
plain/takeはRetainの存在から決めず、元契約とcallee全出口/caller義務を独立照合する。
既存公開情報では不足すると判明した場合だけ、追加ABIを別の承認事項として提示する。

「使い手なし」の例外として、同じ選択seriesで最初のC2まで続ける承認を条件とする。
C1だけで製品完了を宣言せず、C2が成立せずseriesを閉じるなら、C1専用の操作・dispatch・
schema受理・flow/emit・専用test・未接続glueを同じseriesで退役する。既存B/C0と既存安全性testは保持する。

### 11.4 一つの計画で証拠を運ぶ

格納側の結合点は既存`ConstructionPlanV1.execution`（`instance_construction.rs:175-219`）。
新しいCounted専用plan ownerは作らない。constructor batchは元writer inventoryを既に持つ
（`instance_constructor_semantic.rs:538,614`）。

1. 既存object-definitionのcanonical selectorでdirectを一度解決し、その検証済結果を貸す。envの再parseや型名からの所属推測はしない。
2. 同じwriter loanのRc source graphを、元store/fieldとformal ordinal・需要へ一度結ぶ。construction executionはその同じchecked frameを保持する。
3. callerはNew siteごとに同じconstructor planのformal需要を借りる。argumentごとにsource/constructor/formalを探し直さない。
   formal宣言の正本は既存BirthFormalContractのまま。field ordinalの需要列を引数ordinalとして流用せず、元formalとの対応を投影する。
4. birth writerは同じstore executionを共通evaluatorへ渡す。値/所有dataと行き先だけで取得・移転を決め、元Parameterを再分類しない。
5. source/physical対応は同じloan/frameを保持する。公開MIR/CFGからの独立検証は別に残す。

frameのsupportが保持先のfinal planを参照し返すRc循環を避け、元source/writer loanへ向ける。
50〜100行減っても試作の接続は約326〜376行で、未実装の格納consumerも残る。
この案だけで旧150〜300行へ戻るとは約束しない。

D/E/Fの共通単位はconstructor planそのものではなく、既存`CheckedExpressionEvaluationV1`、
`EvaluatedBoxReferenceV1`、`EmittedExpressionBodyV1`と一つのordered temporaryである。
Dは既存Field support、Eは既存Call-entryの評価順/取得済prefix、Fは同じOwned tokenと真の由来を渡す。
場所ごとの新しい証明器やconstructor adapterのコピーは追加しない。
Fの`finalized_object_return_outcomes.rs:146-151`のFresh/OwnedCall/Null制限は、別の許可判断が必要。
新origin/ABIが必要と判明したら、その公開契約をユーザーに分けて提示する。

### 11.5 段ごとの見込みと判断

全て整形後productionの**純増見込み**。C1の検証と早期選択を一度だけ計上し、既存B/C0は含めない。

| 段・推奨順 | 見込み | まだ確かめていない点 |
|---|---:|---|
| C1 操作・独立検証 | +450〜850 | 手組みMIR→同serializer→EXE、token slotとpointerの分離 |
| C2=ウ（Dの最初の式内読み） | +180〜350 | 既存の条件とreceiverの各1経路で、同じField support・式末尾/Fault prefixを接続 |
| Dの残り | +100〜250 | 枝/loop・複合式の既存支持関係まで一般化。最初のC2だけでD完了とは数えない |
| Cの格納を後から接続 | +250〜550 | 選択/共通評価は上で計上済み。イのlocal保持・Birth field責任 |
| E 引数とreceiver | +180〜400 | caller保持、後続引数の破壊/ Fault、callee保存 |
| F Return | +300〜650 | 公開role×origin境界。追加契約が要るなら別承認 |
| G local・共有・scope/loop | +200〜500 | 異なるtokenの合流、全出口、共有後の最後のEnd |
| 3c 位置別判定の退役 | −100〜+350 | 追加200〜450/削除100〜300。純減保証ではない |

C〜Gで約+1700〜3600行、3c込みで約+1600〜3900行、testは別に約+800〜1700行の見込み。
これはdirect/no-hook/nonweakの確認と3cの費用であり、weak/fini・containerや旧配置へのRC・既定切替・selfhost移行は含まない。
同じ処理を各段へ複製した場合の数字ではない。上限を新たに承認した表でもない。
3cは`qualified_static_incoming_source.rs:70`と5consumer等を共通Call nodeが取る同seriesで消す。
移動だけや検証削除で削減を数えない。

前の150〜300行は、格納writerへの短い接続を中心に見積もり、MIR完成後にしか無かった配置選択、
birthの別source表現、caller暗黙move、元formalと共通frameの境界費用を落としていた。
さらにRust独立検証のNoHomes/Birthと格納済fieldの責任が未実装だった。RC加減算そのものより、
意味と証拠を複数の既存境界へ運ぶ費用が支配していた。今回も削減量は実装で未検証である。

続ける価値は**限定したC1→C2を一度確かめる範囲ではある**。成功やコンパイラ純減は未証明。
同じowner/台帳で最初のfield読みも閉じなければ、無期限にadapterを増やさず確認不成立を検討する。
やめる場合は自動配置の試作/C1専用品を外し、ADR-002でも使えるBの反復EndとC0の型付き参照・共通台帳を保持する。
ユーザーが決める事: ウ先行、同series限定のC1先行例外、改訂費用で確認を続けるか。
アへの意味変更・Fの公開契約・ADR本採用は、今回の推奨に含めて決定しない。
