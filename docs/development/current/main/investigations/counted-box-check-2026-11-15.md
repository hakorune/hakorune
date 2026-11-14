# 数えるBoxの確認1: 操作・共通式・検証の設計案

Status: **提案。操作名・Normal/Fault契約はユーザー確認前、未実装。**
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
同じslotへの二重Retainは拒否し、別の持分には別のSSA slotを使う。
Fault時にも参照のbitは存在し得るが、取得tokenは存在しない。
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
