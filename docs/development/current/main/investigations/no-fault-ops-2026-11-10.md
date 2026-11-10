# Fault 不要操作の調査と公開形の候補（2026-11-10）

基点 `106480ad4f`。**調査・案だけ。公開契約の採用、source/C/runtime/testの変更、parked laneの再開はしていない。**

## 判断用の短い要約

1. **1引数IndexOf**を最初に検討する。exact Text receiver＋Text needle、CodePointSearchFromZero、non-suspending source policyが既にそろう。5入力/5 Invokeで観測。通常行とframe-free leafの追加はユーザー判断。
2. **exact Text/Textの6比較**が次。27入力/71 Invokeで観測し、軽量化余地は最も大きい。ただしDynamic/nullを同じ扱いにせず、sourceのMaySuspend/順序をどう通常行に保持・精密化するかを先に決める。
3. **Shared.Borrow**は22入力/23 Invoke。exact live anchorからの読みだけで、RCやEndは増えない。field正規化の後、支持元付きborrow検証を通常行へつなぐ時に扱う。後続FieldGetのFaultは残る。
4. **OwnedArray.Get**は専用owned-slotの限定候補。一般Array.getと別で、現受理は元のInteger literal index。1,788 dumpでは0件なので、owned-arrayアプリ選択時に検討し、先行して公開形だけ増やさない。

現予定 **field読取正規化 → basic-expr残り → Map契約**を維持し、その後にユーザーが選んだIndexOf→Text比較を置くのを勧める。2-Text借用helperだけは両候補で共有。Sharedはfield正規化後の必要箇所に合わせ、OwnedArray.Getは対応アプリまで保留できる。これは新しい必須goalや実装選択ではない。

## 分類の境界・数え方

- **A もう枠がない**: 現在の接続済みsource domainで通常行になっている。frame不要≠pure・移動可能・無条件受理ではない。
- **B 候補**: 元source法則とexact domainを限定すればsource Faultを除外する案を作れる。新しい公開形/leaf/検証は未承認・未実装。
- **C Faultが要る**: 現行domainに実際のsource Faultがある。reasonがcallee/child由来ならそのまま記し、架空の固定codeを付けない。制御protocol行2つは既存Fault/ABIの維持という意味。
- **D まだ決められない**: Unprojected、source NoFault法則未決、compat/parkedの混在。runtimeが0/2だけ、raw MIRに辺がない、opcode名/effectがPureRead、といった事だけでBにはしない。

0/2 leafの案は、ABI前提を満たす有効な記憶領域上で検出できるdescriptor/lease/out違反を2にするもの。dangling pointerやnative UBを安全な2へ変換する約束ではなく、fatal OOMやpanicが絶対に起きないという主張でもない。sourceのFault Outcomeと区別する。

重複する層を合算しない:

| 調べた語彙 | A | B | C | D | 計 |
|---|---:|---:|---:|---:|---:|
| InvokeOperationの葉（27 top-levelをnested5族で展開） | 0 | 3 | 51 | 6 | 60 |
| 通常MIR（MirInstruction62からInvokeを除く） | 18 | 0 | 2 protocol | 41 | 61 |
| Core契約（17行をarity別に展開、aliasは増やさない） | 2 | 1 | 3 | 13 | 19 |

`String.Compare`全体はC。**そのexact Text/Text部分を第4のB案として別掲**するので、候補案は4件だがInvokeのB葉は3件。CoreのIndexOfとInvokeのIndexOfは同じ仕事の別境界で、候補2件と水増ししない。

通常MIRのDには、構造上はFault辺のないlegacy/未活性形を含む。語彙が存在する事とsourceのNoSourceFault証明を混同しないための分類。AのBinOp/Compare等も後述の限定されたsource profileだけ。

## 根拠pathの略記

すべて基点で確認したfile:line。`K=crates/nyash_kernel/src/exports/`、`I=src/mir/`。

| 記号 | file |
|---|---|
| O | K/fault_checked_object.rs |
| S | K/fault_checked_string.rs |
| M | K/fault_checked_map.rs |
| MR | K/fault_checked_map_reads.rs |
| A | K/fault_checked_array.rs |
| OA | K/fault_checked_array_owned.rs |
| CA | K/fault_checked_array_carrier.rs |
| SH | K/fault_checked_shared.rs |
| DY | K/fault_checked_dynamic.rs |
| KR | src/mir/generated/core_method_contract_rows.rs |
| CB | lang/src/runtime/meta/core_method_contract_box.hako |
| TARGET | src/mir/resolved_semantics/core_method_instance_target.rs |
| OWN | docs/reference/language/ownership.md |
| DOP | docs/reference/language/dynamic-operators.md |
| F | docs/reference/language/failure-outcome-relations.md |
| LIFE | docs/reference/language/lifecycle.md |
| BIRTH | docs/development/current/main/design/constructor-birth-new-lifecycle-ssot.md |
| COL | docs/development/current/main/design/collection-literal-construction-ssot.md |
| C/emit | lang/c-abi/shims/published_mir/hako_llvmc_ffi_lifecycle_v4_emit.inc |
| C/string_emit | lang/c-abi/shims/published_mir/hako_llvmc_ffi_lifecycle_v4_string_emit.inc |

共通のsource契約: F:39–41（Faultは値ではない）、F:132–142（全outcomeと最初のFault/後続suppressed）、BIRTH:613–625（exact i64 admissionもFieldSet/HomeRelease/Reclaimのinfallibilityを証明しない）。Mapの`failed`はM:38–40から`record_static`へ、read_errorはMR:213–218から100/101へ到達する。objectの共通failedもO:100–111で100/101を写すが、表では個々のstorage helperが実際に返すcodeへ絞った。別fileのhelperをたどらずに「export内にrecordがない」と判断しない。

objectの終端までの照合: `crates/nyash_kernel/src/exports/typed_object_store_backend.rs:305–332` のallocateは100、`:337–356` get/`:376–396` exact set/`:401–410` reclaimは101。`crates/nyash_kernel/src/exports/typed_object_store_residence.rs:22–46` object install、`:134–150` dynamic read、`:155–189` object borrow、`:194–201` dynamic takeはいずれも101。invalid layout/profileの2をsource Faultへ数えない。Map teardownは `src/boxes/map_box_checked.rs:143–152,531–558` のpayload End/storage report→M:52–70のrecord_end_error→82–88のend_statusを通り、child由来Faultも保持する。

## Invoke全60葉の一覧

観測列は `$SL/dumps-final` の **Invoke行数 / その行を含む入力数**。0は語彙の不在・caller-zero・全repo未使用を意味しない。dump以前の拒否、canonical EXEまでの後段、今回の入力群外は観測しない。

| 操作 | 分類 | 観測 行/入力 | 定義と根拠（略記は上表） |
|---|---|---:|---|
| `Birth` | **C** | 772 / 558 | `src/mir/instruction/invoke.rs:347`; callee/body/初期化のFaultを伝播。LIFE:170–179、BIRTH:627–630。単一reasonでない。 |
| `Call` | **C** | 776 / 492 | `src/mir/instruction/invoke.rs:349`; calleeのOutcomeを伝播（LIFE:170–179）。exact result型だけでは実行成功を証明しない。 |
| `NewBox` | **C** | 838 / 615 | `src/mir/instruction/invoke.rs:354`; O:117–151→store allocate→failed:100–111、到達するsource Faultは100 allocation/storage（invalid layout/profileは2）。 |
| `IntrinsicArrayNew` | **D** | 6 / 6 | `src/mir/instruction/invoke.rs:358`; A:135–156は0/2だが、COL:157–167はNormal/Fault取得契約。NoFault取得法則の判断は所有/entry回。 |
| `ScriptArgumentsNew` | **D** | 74 / 74 | `src/mir/instruction/invoke.rs:361`; A:166–215は0/2のみ。src/mir/builder/normal_main_arguments.rs:64–105のentry Fault責任を再決定するまで保持。 |
| `Map.New` | **C** | 2 / 2 | `src/mir/instruction/invoke_map.rs:48`; M:112–116→root_reason:42–46、100 storage/capacity/order exhaustion。 |
| `Map.FieldKeys` | **C** | 0 / 0 | `src/mir/instruction/invoke_map.rs:51`; O:481–484→MR:25–31、field100/101＋snapshot reserve100。 |
| `Map.FieldHas` | **C** | 0 / 0 | `src/mir/instruction/invoke_map.rs:57`; CB:234–246 MapKeyPresence。O:496–500→MR:173–178,80–82,213–218、key/storage100・projection101。 |
| `Map.FieldGetIndexed` | **C** | 0 / 0 | `src/mir/instruction/invoke_map.rs:64`; O:512–516→MR:202–218、storage100・payload/projection101。 |
| `Map.FieldInstallIndexed` | **C** | 0 / 0 | `src/mir/instruction/invoke_map.rs:72`; O:534–548→M:451–470、key/reserve/install100/101。 |
| `Map.FieldDeleteIndexed` | **C** | 0 / 0 | `src/mir/instruction/invoke_map.rs:82`; O:567–571→M:520–535、key/detach100。 |
| `Map.PrepareKey` | **C** | 2 / 2 | `src/mir/instruction/invoke_map.rs:88`; M:158–165→MapKeyDomain::try_from_text、失敗100。 |
| `Map.InstallIndexed` | **C** | 0 / 0 | `src/mir/instruction/invoke_map.rs:91`; M:180–188→install_candidate:451–470、100/101。 |
| `Map.InstallValue` | **C** | 2 / 2 | `src/mir/instruction/invoke_map.rs:97`; M:272–275→install_candidate:451–470、100/101。 |
| `Map.InstallText` | **C** | 0 / 0 | `src/mir/instruction/invoke_map.rs:105`; M:312–316→install_candidate:451–470、100/101（fatal allocator終端とは別）。 |
| `Map.InstallEmptyArray` | **C** | 0 / 0 | `src/mir/instruction/invoke_map.rs:113`; M:324–350→install_candidate:451–470、100/101。 |
| `Map.InstallBorrowedArray` | **C** | 0 / 0 | `src/mir/instruction/invoke_map.rs:120`; M:372–385→reserve/install_candidate:451–470、100/101。 |
| `Map.CheckedGetI64` | **C** | 0 / 0 | `src/mir/instruction/invoke_map.rs:128`; MR:101–118、missingはNormal0だが非I64は104、key/storage100。 |
| `Map.ArrayLength` | **C** | 0 / 0 | `src/mir/instruction/invoke_map.rs:134`; MR:145–161、missing111・非Array112・key/storage100。 |
| `Map.ArrayIndexMap` | **C** | 0 / 0 | `src/mir/instruction/invoke_map.rs:140`; M:542–600、missing105・bounds106・非Array107・非Map108・storage100。 |
| `Map.MapGetText` | **C** | 0 / 0 | `src/mir/instruction/invoke_map.rs:147`; M:625–684、missing109・非Text110・storage100。 |
| `Map.EndOutcome` | **C** | 2 / 2 | `src/mir/instruction/invoke_map.rs:151`; M:730–732→detached payload.end(context)→record_end_error:52–70→end_status:82–88、100/101やchild固有Fault。 |
| `Map.End` | **C** | 8 / 2 | `src/mir/instruction/invoke_map.rs:154`; M:744–754→map.end(context)→record_end_error:52–70→end_status:82–88、100/101やchild固有Fault。 |
| `String.Add` | **C** | 111 / 51 | `src/mir/instruction/invoke_string.rs:64`; DOP:64–85、S:340–404→publish:166–170、301 allocation・302 Dynamic/null型違反。 |
| `String.Compare` | **C** | 71 / 27 | `src/mir/instruction/invoke_string.rs:69`; 全domainはS:178–229で302（Dynamic非Text、null ordering）。exact Text/Textだけ候補②。mixed Eq/Neは後述の未確認差分。 |
| `String.Materialize` | **C** | 194 / 174 | `src/mir/instruction/invoke_string.rs:71`; 独立所有結果。S:263–290→publish:166–170、301 allocation・Dynamicは302。 |
| `String.FieldMaterialize` | **C** | 82 / 69 | `src/mir/instruction/invoke_string.rs:74`; C/string_emit:217–231→O:609–623→typed_object_store_residence:134–150で101、その後S:263–290→301。fieldの借用だけでない。 |
| `String.Substring` | **C** | 18 / 15 | `src/mir/instruction/invoke_string.rs:76`; KR:169–175のCP clamped lawでも所有結果publish301。S:293–323、Dynamic bound302。 |
| `String.IndexOf` | **B** | 5 / 5 | `src/mir/instruction/invoke_string.rs:77`; KR:178–190＋TARGET:455–472のexact CodePointSearchFromZero/TextParameter。S:326–334とtext/operand helperはvalid Textで0/2。候補①。 |
| `String.Print` | **C** | 691 / 506 | `src/mir/instruction/invoke_string.rs:79`; DOP:88–91、S:407–439→write_all/newline:413–418、300 IO failure。 |
| `String.End` | **D** | 792 / 209 | `src/mir/instruction/invoke_string.rs:81`; S:442–455→dynamic_v2_lease:113–120,206–225→host_handles/call_lifetime:433–449は0/2。F:137–142のcleanup契約を特殊化するsource決定は未確認。所有/3b回。 |
| `String.EndIfOwned` | **D** | 0 / 0 | `src/mir/instruction/invoke_string.rs:84`; Staticはskip、Ownedは同じEnd（C/string_emit:232–241）。runtime0/2だけでsourceのEnd/Faultモデルを変えない。所有/3b回。 |
| `DynamicInvocation` | **C** | 0 / 0 | `src/mir/instruction/invoke.rs:369`; DY:89–150→src/runtime/dynamic_invocation.rs:281–312、400/detail2 missing provider・3 ambiguous・4 type・5 space。F:195–210。 |
| `DynamicArrayGet` | **C** | 0 / 0 | `src/mir/instruction/invoke.rs:372`; DY:153–179→src/runtime/dynamic_invocation.rs:333–357、400/5 carrier発行等。OOB=nullでも取得失敗は残る。 |
| `DynamicCarrierInteger` | **C** | 0 / 0 | `src/mir/instruction/invoke.rs:375`; DY:182–208、400/4 type・有効形のstale/unowned carrierは400/1。malformed raw token自体はDY:199–201で2。DOPのDynamicGreater等のsource型検査。 |
| `DynamicCarrierSubI64` | **C** | 0 / 0 | `src/mir/instruction/invoke.rs:378`; DOP:26–31、DY:212–239、400/4 type・400/5新carrier発行。wrapped Subの計算だけと区別。 |
| `DynamicCarrierEnd` | **C** | 0 / 0 | `src/mir/instruction/invoke.rs:380`; DY:319–334→src/runtime/dynamic_invocation.rs:103–116、二重Endはsource Fault400/1。 |
| `OwnedArray.Claim` | **D** | 0 / 0 | `src/mir/instruction/invoke_owned_array.rs:26`; OA:54–69は0/2だがsource NoFault claim法則未確認。COL:157–161は取得/claimのFault責任を保持。所有Array契約回。 |
| `OwnedArray.Append` | **C** | 0 / 0 | `src/mir/instruction/invoke_owned_array.rs:29`; OA:93–100 ReservationFailed→205。candidate HomeはNormalだけ移転。 |
| `OwnedArray.Get` | **B** | 0 / 0 | `src/mir/instruction/invoke_owned_array.rs:32`; OWN:495–527の専用owned-slot borrow law、OA:109–135は0/2。現sourceはoriginal Integer literal indexに限定。候補④。 |
| `OwnedArray.End` | **C** | 0 / 0 | `src/mir/instruction/invoke_owned_array.rs:34`; OA:153–174、逆順child_endのstatus1と元reasonを保持。 |
| `Shared.New` | **C** | 34 / 29 | `src/mir/instruction/invoke_shared.rs:21`; OWN:130–150、SH:71–88→allocation_fault:66–68、205。 |
| `Shared.Clone` | **C** | 9 / 9 | `src/mir/instruction/invoke_shared.rs:23`; SH:93–112、count overflow/reserve/ID exhaustion→205。新handleのEnd責任を取得。 |
| `Shared.Borrow` | **B** | 23 / 22 | `src/mir/instruction/invoke_shared.rs:25`; OWN:130–150、src/mir/resolved_semantics/home_new_prefix_field_read.rs:395–399のexact support。SH:116–129とdescriptor helperは0/2。候補③。 |
| `Shared.Release` | **C** | 117 / 29 | `src/mir/instruction/invoke_shared.rs:27`; SH:132–159、last handleのpayload End:154–158が任意のsource Faultを伝播。 |
| `DynamicCarrierArray.Claim` | **D** | 3 / 3 | `src/mir/instruction/invoke_owned_array.rs:85`; CA:32–49は0/2だがsource NoFault claim法則未確認。Array/所有モデル回で判断。 |
| `DynamicCarrierArray.Append` | **C** | 0 / 0 | `src/mir/instruction/invoke_owned_array.rs:86`; CA:70–82 ReservationFailed→205。既存carrierの移転責任。 |
| `DynamicCarrierArray.End` | **C** | 3 / 3 | `src/mir/instruction/invoke_owned_array.rs:87`; CA:112–120→checked_dynamic::end_array_slot、child/carrierのsource Faultを伝播。 |
| `ArrayStateContractClaim` | **C** | 0 / 0 | `src/mir/instruction/invoke.rs:388`; COL:157–161、A:115–132,235–237、200 StateConflict・201 ExistingElementMismatch。 |
| `ArrayElementWrite` | **C** | 0 / 0 | `src/mir/instruction/invoke.rs:390`; COL:161–169、A:242–257は202 element mismatch、357–366 setは203 type・204 bounds。現selected LiteralAppend/Pushと未selected Set/Insertを区別。 |
| `FieldGet` | **C** | 42 / 38 | `src/mir/instruction/invoke.rs:399`; O:73–111→store get_checked_indexed、source Fault101 mismatch（profile等は2）。received fallible read。通常ObjectFieldGet/0–2 ABIとは別。 |
| `ObjectFieldBorrow` | **C** | 60 / 42 | `src/mir/instruction/invoke.rs:406`; F:56–58 EmptyField、O:629–641→store_residence borrow→failed、source Fault101。 |
| `FieldSet` | **C** | 849 / 502 | `src/mir/instruction/invoke.rs:413`; BIRTH:613–625、O:156–171,346–421、100/101。置換後のold End Faultも残る。 |
| `ObjectFieldSet` | **C** | 264 / 165 | `src/mir/instruction/invoke.rs:423`; BIRTH:613–625、O:178–206→install_object_residence_checked、source Fault101。両edgeのin-flight child責任を保持。 |
| `HomeRelease` | **C** | 2139 / 535 | `src/mir/instruction/invoke.rs:430`; BIRTH:613–625、O:243–250,271–280→reclaim_checked_indexed、source Fault101、両outcomeで一度消費。 |
| `HomeReleaseIfLive` | **C** | 4 / 1 | `src/mir/instruction/invoke.rs:436`; liveなら同じrelease101。null skipだけでは全操作をNoFaultにしない。O:243–280、C/emit:520–551。 |
| `ObjectEndIfOwned` | **C** | 0 / 0 | `src/mir/instruction/invoke.rs:442`; Freshのfield/child Endとouter release。C/emit:524–534。任意child reasonを保持。一般finiはLIFE:288–293のtarget契約で、現bounded emitterの実行範囲と区別。 |
| `ReclaimUnpublished` | **C** | 772 / 558 | `src/mir/instruction/invoke.rs:448`; BIRTH:613–625、O:256–265→release:243–250→source Fault101。parent finiなし≠NoFault source契約。 |
| `OwnedFieldResidenceRelease` | **C** | 1099 / 146 | `src/mir/instruction/invoke.rs:455`; O:648–676、fieldtake101とold.endの任意Fault。LIFE:346–351。 |
| `OwnedObjectFieldRelease` | **C** | 585 / 132 | `src/mir/instruction/invoke.rs:466`; O:213–239のread単体は0/2でも、C/emit:574–594でchild complete Endの固有Fault（固定codeなし）とouter release101。 |

ArrayElementWriteの補足: enumのLiteralAppend/Push/Set/Insertは `I/instruction.rs:77–81`。current publisherは `physical_program_json_operation.rs:248–255` のLiteralAppend/LiteralまたはPush/MethodCall。set checked ABIの203/204がある事で未selectedのSet/Insertをsource受理したとは数えない。その一般化はbasic-expr/Map・Array契約回のD。

## Core契約19 arityの全件

CBがsemantic owner。result kind/PureReadだけでなくexact arityのlawを使う。Mapのbounded physical操作のCと、generic CoreのUnprojected/Dは別の問い。

| Core | source law | 分類 | 根拠 |
|---|---|---|---|
| `ArrayBox.length/0` | ArrayCount | A | `KR:32`（.hakoがowner、generated表は投影） |
| `ArrayBox.get/1` | Unprojected | D | `KR:43`（.hakoがowner、generated表は投影） |
| `ArrayBox.has/1` | Unprojected | D | `KR:54`（.hakoがowner、generated表は投影） |
| `ArrayBox.set/2` | Unprojected | D | `KR:65`（.hakoがowner、generated表は投影） |
| `ArrayBox.push/1` | Unprojected | D | `KR:76`（.hakoがowner、generated表は投影） |
| `MapBox.get/1` | Unprojected | D | `KR:87`（.hakoがowner、generated表は投影） |
| `MapBox.set/2` | Unprojected | D | `KR:98`（.hakoがowner、generated表は投影） |
| `MapBox.has/1` | MapKeyPresence | C | `KR:109`（.hakoがowner、generated表は投影） |
| `MapBox.delete/1` | Unprojected | D | `KR:120`（.hakoがowner、generated表は投影） |
| `MapBox.keys/0` | MapTextKeysSnapshot | C | `KR:131`（.hakoがowner、generated表は投影） |
| `MapBox.size/0` | Unprojected | D | `KR:142`（.hakoがowner、generated表は投影） |
| `StringBox.length/0` | CodePointCount | A | `KR:153`（.hakoがowner、generated表は投影） |
| `StringBox.substring/1` | Unprojected | D | `KR:164`（.hakoがowner、generated表は投影） |
| `StringBox.substring/2` | CodePointHalfOpenClamped | C | `KR:164`（.hakoがowner、generated表は投影） |
| `StringBox.indexOf/1` | CodePointSearchFromZero | B | `KR:178`（.hakoがowner、generated表は投影） |
| `StringBox.indexOf/2` | Unprojected | D | `KR:178`（.hakoがowner、generated表は投影） |
| `StringBox.lastIndexOf/1` | Unprojected | D | `KR:192`（.hakoがowner、generated表は投影） |
| `StringBox.contains/1` | Unprojected | D | `KR:203`（.hakoがowner、generated表は投影） |
| `StringBox.equals/1` | Unprojected | D | `KR:214`（.hakoがowner、generated表は投影） |

Unprojectedの決着: Arrayのget/has/set/pushはbasic-expr/Array契約、Mapのget/set/delete/sizeはMap契約、String substring1/indexOf2/lastIndexOf/contains/equalsは別のString API法則の決定。`equals/1`をoperatorの6比較と同時に活性化しない。Map.hasはpresence法則だけでkey確保/storageの100/101を消せず、keysはsnapshotの新しいArray取得もある。

## 通常MIR全61種（Invoke以外）

| 命令 | 分類 | 定義のfile:line |
|---|---|---|
| `Const` | A | `I/instruction.rs:303` |
| `BinOp` | A | `I/instruction.rs:308` |
| `UnaryOp` | A | `I/instruction.rs:317` |
| `Compare` | A | `I/instruction.rs:326` |
| `Load` | D | `I/instruction.rs:336` |
| `StaticDataLoad` | D | `I/instruction.rs:340` |
| `Store` | D | `I/instruction.rs:352` |
| `MapLiteralEntryWrite` | D | `I/instruction.rs:356` |
| `ArrayElementWrite` | D | `I/instruction.rs:363` |
| `ArrayRead` | A | `I/instruction.rs:377` |
| `ArrayStateContractClaim` | D | `I/instruction.rs:386` |
| `MemOp` | D | `I/instruction.rs:392` |
| `PinnedTextOp` | D | `I/instruction.rs:403` |
| `PinnedTextResidenceFinish` | D | `I/instruction.rs:412` |
| `ObjectFieldGet` | A | `I/instruction.rs:415` |
| `FieldGet` | D | `I/instruction.rs:423` |
| `FieldSet` | D | `I/instruction.rs:432` |
| `WeakFieldWrite` | D | `I/instruction.rs:440` |
| `VariantMake` | D | `I/instruction.rs:451` |
| `VariantTag` | D | `I/instruction.rs:462` |
| `VariantProject` | D | `I/instruction.rs:470` |
| `Call` | D | `I/instruction.rs:482` |
| `LegacyCallV0` | D | `I/instruction.rs:490` |
| `NewClosure` | D | `I/instruction.rs:501` |
| `Branch` | A | `I/instruction.rs:518` |
| `Jump` | A | `I/instruction.rs:530` |
| `Return` | A | `I/instruction.rs:538` |
| `InvokeNormalResult` | A | `I/instruction.rs:549` |
| `ReturnFault` | C | `I/instruction.rs:555` |
| `FaultFrameEnter` | C | `I/instruction.rs:559` |
| `CheckedCallOut` | D | `I/instruction.rs:564` |
| `PinnedTextResidenceEnter` | D | `I/instruction.rs:576` |
| `PinnedTextResidenceTrap` | D | `I/instruction.rs:585` |
| `CheckedCallOutNormalResult` | D | `I/instruction.rs:590` |
| `CheckedCallOutEnd` | D | `I/instruction.rs:599` |
| `CheckedCallOutFault` | D | `I/instruction.rs:606` |
| `Phi` | A | `I/instruction.rs:620` |
| `NewBox` | D | `I/instruction.rs:629` |
| `TypeOp` | D | `I/instruction.rs:638` |
| `Copy` | A | `I/instruction.rs:648` |
| `CopyOwned` | D | `I/instruction.rs:655` |
| `DestroyOwned` | D | `I/instruction.rs:661` |
| `ArrayResidenceRelease` | A | `I/instruction.rs:665` |
| `StringLength` | A | `I/instruction.rs:672` |
| `StringResidenceInstall` | A | `I/instruction.rs:673` |
| `LocalContractWrite` | D | `I/instruction.rs:676` |
| `RecordFieldContractCheck` | D | `I/instruction.rs:684` |
| `RecordValuePublish` | D | `I/instruction.rs:692` |
| `Debug` | D | `I/instruction.rs:704` |
| `KeepAlive` | A | `I/instruction.rs:710` |
| `ReleaseStrong` | D | `I/instruction.rs:716` |
| `Throw` | D | `I/instruction.rs:721` |
| `Catch` | D | `I/instruction.rs:728` |
| `Safepoint` | A | `I/instruction.rs:736` |
| `RefNew` | D | `I/instruction.rs:741` |
| `WeakRef` | D | `I/instruction.rs:746` |
| `Barrier` | D | `I/instruction.rs:754` |
| `FutureNew` | D | `I/instruction.rs:759` |
| `FutureSet` | D | `I/instruction.rs:763` |
| `Await` | D | `I/instruction.rs:767` |
| `Select` | A | `I/instruction.rs:772` |

Aのsource条件:

- `BinOp`: current-I64 Add/Sub/Mulのmodulo2^64、Divは元の正のInteger literal divisorだけ（DOP:196–214、`home_expression_operator_loan.rs:89–151`）。ゼロ/負の除数やMIN/-1、Dynamic、Float、一般Modをこの決定で無Faultにしない。BinaryOpの全語彙は `crates/hakorune_mir_core/src/types.rs:37–54`（Add/Sub/Mul/Div/Mod/BitAnd/BitOr/BitXor/Shl/Shr/And/Or）。選ばれていない物はbasic-exprでsource法則を照合する。
- `UnaryOp`: Neg/Not/BitNot（同types:59–67）のうち現在のexact source operand/operator loanを通った物だけ。`Compare`はEq/Ne/Lt/Le/Gt/Ge（types:72–79）のexact scalar/null判定。Dynamic/StringはInvokeの別法則。
- `ArrayRead`: Element/Count（I/instruction.rs:292–295,373–382）の既存bounded境界。CountはCB:122–134/TARGET:430–438から共通writerへ。generic ArrayGetのUnprojectedを解決するものではない。
- `ObjectFieldGet`: 元の宣言/closed SUM/成功writeと同一receiverのsource proofを既存field ownerで保持。`ordinary_new_field_read_source.rs:53`、`ordinary_new_field_reads.rs:233–314`。`nyash.object.checked_field_get_i64_v1`はO:40–70のframeなし0/2 ABI。received/dynamic/EmptyFieldのInvoke FieldGetを削除する根拠ではない。
- `StringLength`: 現SL1–14、TARGET:436–445、唯一のprojection `expression_source_values.rs:405–442`。Return rootがMethodCallのraw13本はbasic-expr残件で、このAを全source形の受理と読まない。
- `ArrayResidenceRelease`: COL:171–178の明示法則。WRITE/non-pure、frame/result/sourceFault辺なし、source finiを呼ばない。Object/Shared/OwnedArray Endと混ぜない。
- ReturnのHome Endは別命令で、Returnだけを見てscope全体をNoFaultにしない。StringResidenceInstallは同一residenceのmarkerで、置換のEnd/取得は別責任。Selectはloop更新用ternary、Safepointは現在の機構行で、投機・並べ替え許可ではない。

Dの確認先/決着回:

- Load/StaticDataLoad/Store/MemOp: region/FFI/fast-memory契約。PinnedText/Residence/CheckedCallOut族はS6Cのsource/lease/Fault契約（parked条件を維持）。成功markerの構造だけで新しい一般NoFault法則にしない。
- Call/LegacyCallV0/NewBox/NewClosure、CopyOwned/DestroyOwned/ReleaseStrong、raw FieldGet/Set: 3bの旧経路と所有モデル。callee/Endの意味をsource宣言へ結び付けてから整理。
- LocalContractWrite/RecordFieldContractCheck/RecordValuePublish、Variant族/TypeOp: language-v1/record/typed契約の個別決定。Checkとcast等をenum名だけで一括NoFault化しない。
- WeakFieldWrite/RefNew/WeakRef/Barrier、FutureNew/FutureSet/Await、Throw/Catch、Debug: それぞれweak/並行性/compat/IOの境界。現在のsource-supported profile・Fault mappingを全件確認したとは主張しない。
- raw MapLiteralEntryWrite/ArrayElementWrite/ArrayStateContractClaim: 上のbounded Invoke ownerと異なる旧経路。basic-expr/Map契約/3bで意味とcallerを閉じる。

`ReturnFault`は既存のFaultの伝搬、`FaultFrameEnter`は既存ABI/prologue。これら自身に新しいsource reason codeがあるとは主張しない。root ABIは今回の候補が純粋でも保持する。

## 候補の案（全てユーザー判断前）

### ① IndexOf/1: exact CodePointSearchFromZero

**sourceとdomain**: KR:178–190、TARGET:455–472はStringBoxText schema/receiver/ABI/NonSuspendingNonControl/TextParameter1つ/I64ToCallerまでexact照合。receiver/needleともStaticText・OwnedString・BorrowedText・TextOperandだけ。Integer/Bool/tagged Dynamic/opaque carrier、2引数indexOfを含めない。receiverとneedleをsource順で一度評価し、元のloan/Copy/formal/leaseを保持する。

**runtime補助**: S:326–334→text_operand:159–162→operand:57–66/text:131–155→`src/boxes/string_ops.rs:17–21,58–72,133–145`。valid TextはCP検索で空needle0/miss−1のNormal。source Fault記録は無く、残るdescriptor/lease/out違反はInvalidContract2。現在のsnapshot cloneのfatal allocationは返却Fault1とは別。

公開案:

```text
MIR: StringIndexOf { dst, value, needle }
JSON: {"op":"string_index_of","dst":D,"value":Text,"needle":Text}
ABI: nyash.string.code_point_index_of_v1(value,needle,out) -> u32
     0: Unicode CP index（空needle0/miss−1）、2: out不変。frame/site/Fault記録なし。
```

既存通常行/strict schema/C prologue一意名/PHI forwardingは再利用する。ただし今のCore projection Continueは `(Law, Operand)` 一個（`expression_source_values.rs:405–442`）なので、2つの元operandを保持するprivate read-planへ拡張が必要。writer/source対応/枠判定へ同じ判定を複製しない。旧IndexOf生成腕は同file:454–457。旧checked IndexOfの同時退役/古いartifact拒否はLengthの決定を自動適用せず、ユーザーに別途選んでもらう。

概算（未実装）: production追加220–350、削除25–55、test追加60–110。下の2-Text借用helper50–90を含む。機能と専有退役を別集計し、これを確定見積もりや純減保証にしない。

### ② exact Text/Textの6比較

**sourceとdomain**: DOP:98–119、`src/mir/dynamic_operator_contract/issuer.rs:82–99,409–420`。Unicode scalar列の等値/辞書順/prefix順、正規化なし、borrow/noescape、TrivialBool。現在のprojectionは `root_call_entry/scalar_condition_operation.rs:58–103` でStringとDynamicCarrierの腕を分ける。4種のText operandだけを新行へ渡し、Dynamic/nullのtyped-check境界は既存checked laneに残す。

**runtime補助**: S:178–204→comparable:208–229で、exact Textなら210–211のtext_operandへ行き、Dynamic非Text302/null ordering302に入らない。残る異常は2、outはNormalでだけ書く。

公開案:

```text
MIR: StringCompare { dst, op, left, right }
JSON: {"op":"string_compare","dst":D,"comparison":"lt","left":Text,"right":Text}
ABI: nyash.string.compare_text_v1(op,left,right,out) -> u32
     0: Bool表現0/1、2: out不変（不正opも2）。frame/siteなし。
```

**追加判断**: exact String envelopeにもMaySuspend/observableが残る（issuer:88、DOP:112–119）。NoSourceFaultとnon-suspension/順序は別に扱い、canonical issuerで精密化するか、通常行に残る順序/停止規約で表すかを決める。NoFaultをPure、投機可能、Endを越えて移動可能、と読まない。比較のためのframeが消えても、operand取得・連結・Call・EndのFaultはそのまま残す。

mixed Eq/NeはDOP:49–57,121–131ではwellformed非Stringは不等、malformedだけFault。一方S:224–227のopaque carrier snapshotは非Textで302、null Eq/Neだけ187–194でNormal。sourceのtagged Dynamicとopaque carrierへの全投影/到達domainまでは照合していないため、両者を同じ表としてruntimeがsource全体を裏付けるとは書かない。basic-expr/Dynamic契約で整合を調べる残件であり、今回のexact Text候補には混ぜない。

概算: 単独ならproduction追加200–340、削除0–25、test追加70–140。IndexOf後なら共通借用helper50–90を二度足さない。mixed比較のcallerが残るためchecked比較enum/export全部を消す見積もりにはしない。

### 2つのTextを借りる共通部分と公開enum

現checked readはS:129–150で各operandを別スコープでsnapshot cloneする。Lengthの `src/runtime/dynamic_v2_lease.rs:92–108` callbackはlease Mutexとhost read lockを保持するため、Owned/Ownedで単純に入れ子にすると同じMutexへ再入する。

IndexOf/Compareを複製なしleafにするには、**両token/generationを一つのスコープで検査するprivate two-Text借用helper**が必要。同じpair二重read、Static/Owned、Owned/Owned、lock順序を扱い、callback中publish/drop/再入なし、outはunlock後。raw-handle用TextReadSession.str_pairはtoken/generation証拠を代用できない。これは50–90 production行程度の設計概算で、実装もdeadlock試験も行っていない。

Proは「一要素のStringRead enumを先行しても今の重複は減らない」と答えた。今回もまずflat公開行案を示し、private借用/C row machineryを共有するのを勧める。IndexOf/比較/Lengthの3実操作をユーザーがまとめて選ぶなら公開StringRead familyへ移す案も比較可能だが、現StringLengthのMIR/全consumerの移行、I64/Bool/arityの独立検証費用を別に見積もる必要がある。一要素enumの否定を将来のfamily禁止と読まない。Core method issuerとoperator issuer、Shared/OwnedArrayの支持元法則まで一つの万能NoFault enumへ混ぜない。

### ③ Shared.Borrow

**source**: OWN:130–150はread-only payload、borrowはhandle/Endを増やさずthread-confined。`home_new_prefix_field_read.rs:395–399` のShared provenanceだけがsupportを得て、`builder/fields/checked_read.rs:23–50` と `ordinary_new_checked_field_read.rs:23–67` がSAME handle/descriptor.payloadを照合する。`ownership_ssa/verify.rs:181–190,261–263` はexact live anchorにviewを結び、借用中Releaseを拒否。

**runtime**: SH:116–129→descriptor:52–64、thread-local SLABでlive handle/descriptor/countを検査。payloadを借りて返すだけでRC/取得/End/allocなし。validは0、残る違反は2/out不変。

```text
MIR: SharedBorrow { dst, descriptor, handle }
JSON: {"op":"shared_borrow","dst":D,"value":H,"object_id":O,"profile":P,"abi":V,"end":...}
ABI: nyash.shared.borrow_payload_v1(descriptor,handle,out) -> 0/2
```

JSONのdescriptor欄は既存Shared descriptor搬送を維持する案で、最終key/schemaはユーザー決定前。架空ObjectIdや別cloneのsupportで穴を埋めない。StringLengthの通常行・C leaf形は使えても、結果はI64でなく**anchor付きobject view**。ownershipSSAの通常行borrow定義、独立Cの支持関係/last Release/descriptor照合が必要。後続FieldGetはfallibleのまま。

概算: production追加180–300、削除50–110、test追加60–110。候補中の所有リスクが高い。anchor move/last Releaseをまたいでviewを使わせない。field読取正規化後、Sharedアプリの必要箇所と同時に選ぶ。

### ④ 専用OwnedArray.Get

**source**: OWN:495–527はnamed append-only object-slot destination。missing/nullはnullableで返し、Home/retainを増やさずholder/committed slotに借用を結ぶ。`src/mir/resolved_semantics/named_array_append_only_source.rs:289–302` がInteger literal以外をIndexSourceとして拒否する。`install/owned_array.rs:179–222` の元Recipe/site/class/descriptor、現在は `original_read_integer_v1` による**元Integer literal index**だけ。`ordinary_new_local_commit/owned_array_read.rs:72–139`、同 `owned_array.rs:787–804` が同じlive holder・prior Home・元sourceを再照合。

**runtime**: OA:109–135→`src/boxes/array/owned_slots.rs:194–208`。phase/descriptorを検査しread lock内でslotを読む。負/範囲外はnull、allocate/clone/retain/Endなし。identity違反はOA:131–132で2。validは0/out nullable handle、他は2/out不変。

```text
MIR: OwnedArrayBorrow { dst, array, descriptor, index }
JSON: {"op":"owned_array_borrow","dst":D,"array":A,"descriptor":...,"index":I}
ABI: nyash.array.borrow_owned_slot_v1(array,descriptor,index,out) -> 0/2
```

一般ArrayReadやMap slotへ偽装しない。nullable object結果のholder/slot/child descriptorを通常行でも独立検証し、holder End/slot mutation/escapeとの整合を保持。一般Dynamic indexやCore ArrayGetを同時に受理しない。

概算: production追加250–450、削除100–250。testの既存fixture再利用範囲は未見積もり（新しいborrow/end負例が必要）。1,788件に0行なので、対応アプリを選んでからexact domainを縦に一度通す。旧Get Invoke/投影/専用Fault記録はcaller切替と同じseriesで退役する案。

## 既存dumpで数えた影響

対象は前回最終 `$SL/dumps-final/results.json` の1,788件。productionはこの基点と同じ。新しいcompile/build/テストはしていない。Debug印字のStatic文字列を除いて分類し、Compare71行は印字上すべてexact Text operand。**source法則の正しさは上のissuer照合が根拠で、印字タグから発行していない。**

| 候補の物理profile | 入力数 | Invoke行 | 命令純減(非合流) | block減 | Invoke純減 | cleanup減 | 条件つきframe減 |
|---|---:|---:|---:|---:|---:|---:|---:|
| `String.IndexOf` | 5 | 5 | 8 | 8 | 6 | 1 | 2 |
| `String.Compare` | 27 | 71 | 218 | 218 | 120 | 49 | 15 |
| `Shared.Borrow` | 22 | 23 | 0 | 0 | 23 | 0 | 0 |
| `OwnedArray.Get` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

後ろ5列は**新MIRの実測ではない**。同じCFG上で対象Invoke→Fault辺だけを削り、残る経路に共有されないblockを落とし、通常readのdstへ置き換えて不要なNormalResultを消す静的projection。block合流はしないため、残すNormal landingへ進むJumpを各readに1つ足し、その分を命令純減から引いている。旧全到達RとFault辺を消したR'の差だけをcleanup削除候補にし、残るCall/連結/Endの共有Fault suffixは保持する。entryはdumpの最小IDを仮定するため、実装時はcanonical entryで照合し直す必要がある。数字は実装完了/確定削減量/速度の主張ではない。

命令投影には追加frame削除を含めない。既存root ABIを保ち、non-rootで他のFault/呼出し/所有責任がないとsourceから再確認できた時だけ、右端のframe2/15をさらに除く可能性がある。normal landing合流やrecipe再構成による追加block減は未見積もり。4案を合算すると共有suffixを二重に数えるため、表はそれぞれ単独で適用した値。

IndexOf:5入力5行、NormalResult5＋専用Fault block8の命令8＝gross13行を削除し、Jump5を追加するので純減8命令/6 Invoke。Text比較:27入力71行、NormalResult71＋専用Fault block218の命令218＝gross289行削除、Jump71追加で純減218命令/120 Invoke、cleanup49。Shared.Borrow:NormalResult23削除とJump23追加が相殺し、非合流では命令純減0、Invokeは23減る。残るFieldGet等でframe/cleanupは減らない。OwnedArray.Get:0件はこの母集団に機会が見えないという意味で、source producerが存在しない意味ではない。

全入力/関数/block/操作の観測一覧は `$L/invoke-census.json`、候補ごとの入力と削除専用CFG投影は `$L/candidate-impact.json`、enum/Coreの母集合は `$L/catalog.json`。これらは既存dumpを数える一時の調査資産で、repoにcheckerや継続台帳は増やしていない。

## ユーザーに決めてほしいことと未確認

1. IndexOf/1を次の公開通常行として選ぶか、field/basic-expr/Mapの後に置くか。旧checked IndexOfを同時退役するか（他arity/互換callerを別途確認）。
2. Text比較はexact Textだけの通常行を採り、mixedをcheckedで保持するか。MaySuspend/順序のsource証拠をどう表すか。
3. flatな公開2行とprivate2-Text helper共有を採るか、3実操作がそろった時にStringRead family移行を比較するか。今はどちらも実装しない。
4. Shared.Borrowをfield正規化と接続する時期、OwnedArrayBorrowを対応アプリまで待つこと。いずれも支持元/borrow lifetimeの新しい通常行検証が条件。
5. Text End/EndIfOwned、Array/Script引数のallocation、owned/carrier Array claimのNoFault特殊化はsource決定が足りない。runtime0/2だけを理由に先行しない。

未確認: 新public schema/ABIの最終名、2-Text helperのlock順序/alias実装/実時間、追加/削除LOCの実値、calleeごとのFault集合、D41語彙の全profile、mixed Dynamic Eq/Neのsource-to-carrier到達整合、dump entry仮定の全関数照合、canonical JSONだけに現れる操作頻度、母集団外アプリの効果、benchmark wall/RSS。法則・domainを満たさない物をNoFaultと認定していない。実装・schema/test追加・NoFault化による検証の弱化は0。

## ChatGPT Proへの相談とユーザー決定（2026-11-10）

Opusが基点`78ca1419c9`で相談文を作った（8問: 分類、比較の仕様、公開の形、Textを2つ借りるhelper、mixedの`==`の食い違い、順番、古いchecked形、速さ）。ChatGPT Proが答え、Proが引いた根拠はOpusが`78ca1419c9`で読み、記述どおりだった。

### Proの答え（要点）

- **分類**はおおむね妥当。ただし「opcodeの分類」ではなく「source domainを含む操作の分類」として読む。同じfamilyの読みがNoFaultでも、終了（SharedのRelease、OwnedArrayのEnd）はNoFaultとは限らない（`fault_checked_shared.rs:116–159`、`fault_checked_array_owned.rs:109–174`）。
- **TextのEnd**は有望。`ArrayResidenceRelease`は、frame・結果・sourceのFaultの辺を持たない前例（`design/collection-literal-construction-ssot.md:171–178`）。runtimeのText Endも0/2だけ（`fault_checked_string.rs:442–455`）。ただしEndは所有の状態を変えるので、`frame_free_read_law_v1`や読みのhelperに入れず、別の小さな設計検討にする。確保とClaimはDのまま。
- **mixedの`==`の食い違いは、契約と実装の不整合として確認できる。** issuerはDynamicとStringの`==`/`!=`で、正しい形のString以外の値を不等とし、壊れたcarrierだけをFaultにする（`issuer.rs:160–184`）。runtimeは演算子に関係なく、Text以外のcarrierをTypeErrorにする（`fault_checked_string.rs:178–229`）。比較のprojectionはDynamicの呼び出し結果をcarrierとして渡し（`scalar_condition_operation.rs:80–96`）、Dynamicの`length`はInteger carrierを返す（`dynamic_invocation.rs:360–377`）。sourceからEXEまでの再現は未確認。直す方向はruntimeを仕様とissuerに合わせる。Text以外を一律にnullとして扱わない（今のnullは両方nullなら等しい）。物理のdescriptorの破損（status 2）と、sourceの契約の壊れたcarrierのFaultを区別する。②の軽量化より前に、独立した正しさの修正として行う。
- **比較の仕様**: 生きたexact Text同士に限って、NonSuspendingに決め直してよい。順序・observable・borrowは別の軸（`model.rs:66–92`）なので、MaySuspendを残す必要はない。ただし今のFaultの種類にNoFaultの選択肢がなく（`model.rs:106–112`）、`sealed()`はTypeErrorを入れる（`model.rs:128–141`）ので、「残るsourceのFaultが無い」こともissuerで明示的に発行する。NonSuspendingの定義は「確保・dispatch・停止点が無い」（`model.rs:76–82`）なので、今の全文を複製するruntimeのまま値だけ変えない。lease Mutexの待ちは言語の停止ではない（Lengthと同じ）。
- **公開の形**: 平らな`StringIndexOf`と`StringCompare`。`StringRead`のfamilyには移らない。既存のI64の`compare`の行（`nyrt_c_abi_v0.md:334–340`）は広げない。共有するのは、Text operandの検査、借りて読むruntime helper、通常行の結果型・定義・PHI・Cの出力の仕組み。Core projectionの`(Law, Operand)`は、receiverとneedleを持つprivateな計画へ広げる。検証側の`has_length_origin`等を操作ごとに複製しない。
- **Textを2つ借りるhelper**: lease tableのMutexは表全体で1つ（`dynamic_v2_lease.rs:100–131`）なので、別々のString同士でも入れ子は不可。lease tableを1回、host registryを1回だけ取り、両方のtoken・generation・exactなTextを確かめてから2つの`&str`で計算し、lockを離してから結果を書く。lockの順序はlease → registry。同じ値を2回読むのは許すが、検証は省かない。
- **古いchecked形**: IndexOfは条件付きで同じseriesで消してよい。canonicalのproducerが0であることに加え、Cのconsumer、header、別のbackend、test、互換の経路を確かめる。Compareのchecked形はDynamicとnullのために残す。exact Textの証拠が閉じた時に新しい行の発行に失敗しても、checkedへ戻るfallbackは作らない。
- **順番**: mixedの`==`の修正 → fieldの読みの正規化 → 2-Text helperと①IndexOf → 契約の精密化と②Text比較 → basic-exprの残り → Mapの契約…。①と②は別々に着地・検証できる段階に分ける。②の仕様決定が長引くなら①だけで進めてよい。③④は後回し。
- **速さ**: 全文の複製をやめる → 枠を外す → lengthのcache、の順。測る時は「checked＋複製 → checked＋借用 → 通常行＋借用」を比べる。

### ユーザー決定（2026-11-10）

- 順番: mixedの`==`の不具合の修正（最初にsourceからEXEまでの再現を作る）→ fieldの読みの正規化 → 2-Text helperと`StringIndexOf`（旧checked IndexOfは、producer・C・header・test・互換の経路が0であることを確かめて、同じseriesで消す）→ 比較の契約の精密化と`StringCompare`（複製しないruntimeと一緒に）→ basic-exprの残り → Mapの契約 → `edit` → 3bと旧経路の退役 → 徹底的な整理 → `.hako`のparserのparity → selfhost。
- 比較の仕様: exact String同士の6つの比較は、結果がNormal(Bool)だけ（比較そのもののFaultは無い）、NonSuspending、評価の順序と借用と所有は今のまま。左右の値を作る時のFault（連結、呼び出し、fieldの読み）はその操作の物。issuerのFaultの種類に「無し」を足し、枠の判定・命令の選択・sourceの対応の検査はその答えを使う。DynamicとStringの比較とnullの比較は今の仕様のままcheckedの形に残し、どちらを使うかはsourceの型の組で決める。
- 後回し: `Shared.Borrow`、専用`OwnedArray.Get`。TextのEndは、basic-exprの頃に小さな設計調査をする。

## 比較の正規化のChatGPT Proへの相談とユーザー決定（2026-11-10）

ユーザーが「比較を正規化して他の比較にも使えるようにするか」を聞いた。Opusが基点`57bde328c4`で相談文を作った（8問: 方向、決める持ち主の形、nullの等値、整数とBoolのenvelope、綴り、実装中の`StringCompare`への注意、3bとの関係、順番）。ChatGPT Proが答え、Proが引いた根拠はOpusが`57bde328c4`で読み、記述どおりだった。Proは非公開の`StringCompare`のbranchを読んでおらず、build・EXE・全producerの集計もしていない。

### Opusの前提の誤り（Proの指摘、Opusが確認）

- 整数の`compare`は「Faultの無い比較」だけではない。Cはtaggedのoperandも受け（`lang/c-abi/shims/published_mir/hako_llvmc_ffi_lifecycle_v4_indexed_rows.inc:128–132`）、kindが1でなければ`%fault`へ飛ぶ（`hako_llvmc_ffi_lifecycle_v4_emit.inc:273–289`）。整数の比較のenvelopeのTypeErrorは、このcheckedの見方では正しい（`docs/reference/language/dynamic-operators.md:146–173`）。
- `sealed()`が入れる初期値はTypeErrorだけで、suspensionは各行が明示している（`src/mir/dynamic_operator_contract/model.rs:128–151`）。BoolとIntegerの`MaySuspend`は明示の値。
- nullの`!=`が無いのは意図。最初の対応は直接の`if`の`==`だけで、`!=`は別のcapability（`dynamic-operators.md:225–228`、`issuer.rs:518–529`）。
- Boolの物理形: CはBool同士の`eq`/`ne`だけを枠なしの`compare`で受け、順序比較は拒否する（`hako_llvmc_ffi_lifecycle_v4_indexed_rows.inc:122–127`）。

### Proの答え（要点）

- **方向**: 賛成。ただし「issuerの表1つに意味と物理形を全部持たせ、独立検証もその答えを使う」は直す。意味はcanonical issuer（`src/mir/dynamic_operator_contract/mod.rs:1–5`の境界を守る）、実装形への変換はprivateな変換1つ、出来たMIRの検証は独立。命令とruntimeは平らなまま（整数、null、String、checked）。公開の比較命令を1つにまとめるのは、今回の問題（意味の推定のやり直し）には要らない、より広いMIRの再設計。
- **各層**: issuerは意味の契約（domain、法則、入力に要る検査・証拠、結果、Fault/suspension/順序/borrow）を返し、JSON名やC exportは持たない。変換はprivateな実行計画（概念: Scalar、NullObservation、Text、Checked）を作る。writerは計画を書くだけで型の組から選び直さない。sourceの対応の検査は計画とMIRを照合する。独立検証は実際のMIRを検査し、issuerの答えを免除証にしない。出発点は既にある共有の計画`scalar_condition_operation.rs:106–213`。
- **null**: serializerの`null_producer`（`src/mir/compiler/normal_default_pipeline/published_backend_view/physical_program_json.rs:285–301,456–475`）は検査ではなく形を選んでいる。長期形はMIRにnullを見る行を明示する方を勧める（JSONの`borrowed_null_compare`とruntimeはそのまま使える見込み）。物理の`borrowed_null_compare`はtaggedの他にhandle、borrowed/mixed object、slot borrow、opaque carrierも扱う（`hako_llvmc_ffi_lifecycle_v4_indexed_rows.inc:133–160`、`hako_llvmc_ffi_lifecycle_v4_emit.inc:290–322`）ので、全部を`Equal(Dynamic, Null)`に押し込まない。独立検証はEq/Neの制限、Boolの結果、operandの定義と支配、許すcarrier、生存と借用、Copy/PHIの対応を見る。Integerの0、Boolのfalse、nullを同じにしない（`dynamic-operators.md:232–243`）。
- **`EQUAL_DYNAMIC_NULL`のFault**: 本当の食い違い。仕様は「許すnon-nullの種類にTypeErrorを入れない」「壊れた搬送はcontract failure」（`dynamic-operators.md:236–237`）だが、envelopeは`sealed()`でTypeError（`issuer.rs:368–385`）。test（`src/mir/dynamic_operator_contract/tests.rs:123–151`）も一緒に直す。NoSourceFault化と`!=`の受理は別の変更にする。Opusの追加確認: 既にある`MalformedCarrierBeforeResult…`（`model.rs:111`、`issuer.rs:179`）は使わない。null比較で壊れたopaque carrierは`%fault`ではなく`%invalid`へ飛ぶ（`hako_llvmc_ffi_lifecycle_v4_emit.inc:314–316`）。
- **整数とBool**: exactな比較はNoSourceFault/NonSuspendingと正直に書く。ただし既にある整数のenvelopeを一括で変えない。「入力がexact I64」と「Integerの種類を確かめてから比べる」を分ける（値の種類`I64`と`NormalInteger`は既に別、`model.rs`の`DynamicOperatorValueClassV1`）。builderが`MirType::Integer`やCのkindを見て勝手にFaultを落とさない。exact Bool同士のEq/NeもNoSourceFault/NonSuspending。taggedの中身がBoolでも同じにはしない。監査する物は、Faultとsuspensionを読んで枠・借用・順序・refinement・再生成を決める所全部。比較本体がFault無しでも、左右の値を作る時のFaultと後始末は残る。
- **綴り**: 価値はあるが優先度は低い。今は変えない。`slt`の`s`は符号付きの意味を持つ。変えるなら`comparison`・`left/right`寄り。既存の鍵（`hako_llvmc_ffi_published_lifecycle_instruction_schema.inc:544`）の改名は破壊的変更なので、新しいversionが自然。旧と新の両方を受けるaliasは作らない。
- **実装中の`StringCompare`への注意**: sourceの契約から1回だけ選ぶ（失敗してcheckedへ戻らない）。NoSourceFaultは明示した契約だけに使い、`sealed()`の初期値を変えない。枠の判定・writer・sourceの対応の検査は同じ選択を使い、`is_text`の独立した分類器を増やさない。新しい行を既存の`Compare`の再生成（`src/mir/builder/ssa/local/materialize.rs:555–568`、`src/mir/builder/ssa/phi_input_materializer/edge_rematerialization.rs:198–203`）へ自動で入れない。C exportはoperandの数ではなくopcodeで選ぶ。JSONの綴りは決めた通り。
- **3b**: 拒否も意味の決定の1つとして、理由を3つに分ける（言語で禁止、許されているがまだ繋いでいない、証拠が足りない）。Bool同士のEq/Neはscalarの比較。nullable objectのnull testはobject同士の`==`の拒否（`docs/reference/language/status-index.md:581`）と別。objectの同一性はraw handleの整数比較へ落とさない。明示の同一性比較が今どこまであるかはPro未確認。
- **順番**: Opusの案（`StringCompare`の後、basic-exprの残りの前）を支持。正規化では、意味を決める所、最初に比較を書く所、SSAの再生成、検査・搬送・読む所の4つを分けて数える。再生成は別の意味の決定ではない。確かめる事: 6演算と左右の順、Boolの順序比較の拒否、nullと0とfalseの区別、tagged/nullの今の受理範囲、Copy/PHIの後の対応、短絡の右辺、型の検査のFaultと後始末、TextのEndの後の使用の拒否。意図した契約の訂正の他に、受理が広がっていない事。

### ユーザー決定（2026-11-10）

- 順番: `StringCompare`（実装中、このまま）→ **比較の正規化** → basic-exprの残り → Mapの契約 → `edit` → ADRの残り → 3bと旧経路の退役 → 徹底的な整理 → `.hako`のparserのparity → selfhost。JSONの綴りの統一は比較の正規化に入れない。
- nullの等値: MIRに専用の行を足す（公開MIRの追加として承認。名前と形は正規化の回の調べの後で決める）。JSONの`borrowed_null_compare`、C、runtimeは今のまま。serializerの`null_producer`による形の選択は退役させる。`!=`の受理は別の判断。`EQUAL_DYNAMIC_NULL`のFaultは、仕様に合わせてNoSourceFaultへ直す。
- 整数とBool: exactな整数同士の6つの比較と、exactなBool同士の`==`/`!=`は、exact Stringと同じ約束（結果はNormal(Bool)だけ、Fault無し、NonSuspending）。taggedの値をIntegerとして比べるcheckedの見方は、TypeErrorが起き得る約束のまま残す。既にあるcheckedのenvelopeを一括でFault無しに書き換えない。仕様の文書も正規化の回で直す。
