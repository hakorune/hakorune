# String.length の軽い操作：調査と公開形の案

Status: **survey / maintainer decision pending**。基点 main `4c1be7c347`、branch `string-length`。2026-11-10。
この文書は実装許可・新しい仕様の採択ではない。production・test・C shim・runtimeは変更していない。

## 判断用の要約

**案A（推奨）:** 既存のtyped Text operandを受ける非terminator `StringLength` を一つ追加する。意味はcode point count。命令にFault frame/辺を持たせず、元のsource proofと独立した行内lifetime検証で支える。1操作だけの別operation enumを作らず、今回の責務が最小になる。

**案B:** `StringRead { operation: Length }` とする。ArrayRead::Countと外形が揃い、将来本当に別の非Fault String readが必要になった時に同familyへ足せる。今回一操作だけのenumを先行追加する費用が約10–30行あり、安全性・runtime・JSON/Cの仕事は案Aと同じ。新機能を先取りしてwhitelistを増やす案ではない。

両案とも既存checked_length_v1のframe検査契約を維持するなら、新しいframe不要のCP計数ABIが必要。既存checked_length_v1をnull frameで呼ぶ案は不成立。runtimeの計数coreを共有し、現在の全String複製をなくす。既存MirCallの公開だけを広げる案は、typed Text搬送を失い、既に不採用のため候補に数えない。

観測上、置く場所でraw/checked/拒否が分かれる。`.length()`という名前やREAD/PureReadからNoFaultを作らず、元のCodePointCount law＋receiverの正確なTextと生存の証拠を、共通評価から各consumerへ渡す必要がある。

## 基点・方法・範囲

- `$R=/mnt/workdisk/public_share/hakorune-take-fix`、`$L=/mnt/workdisk/claude-scratch/string-length`、`$S`=Opus の作業用 scratchpad(手元だけにある)、`$S0`=前の session の scratchpad。
- 使用binary `$L/hakorune.base`、SHA-256 `5a39364fe4f3cd666dc3f816ddd2242a362770c125007db5a9b7322127b34af6`。元buildはa303a0cc1d、4c1be7c347との差はdocsだけ。前回の最終binaryとhash一致、src/lang/cratesにそれ以後の変更なし。docs-onlyなのでCargoは起動していない。
- `HAKO_ROOT=NYASH_ROOT=$R`、`NYASH_BIN=$L/hakorune.base`、`NYASH_EMIT_EXE_NYRT=HAKO_LIFECYCLE_NYRT=$R/target/quick`、`HAKO_AOT_FFI_LIB=$R/target/release/libhako_llvmc_ffi.so`、pure-first、compat replay none、plugins無効。dumpと正規`--backend mir --emit-exe`を別に測る。並行は最大4、runtime timeoutは8秒。
- 既存dump1788、EXE229、apps100、384組、Opus4群677、未見18をsource pathでunionすると2718入力。全path/group一覧はpopulation.json。147候補を現binaryでdump＋EXE測定し、未選択かつ旧dump対象外923入力もdumpで調査した（新しいString length発見0）。旧dump1788の非候補の証拠は同source/hashの前回logを再利用した。
- 候補は`.length/.len/.size()`のsource綴り、または旧dumpのStringBox length／checked String Length。Array/Mapやuser methodも候補へ含むため、候補数をString受理数と誤認しない。前段で拒否される入力の隠れたcalleeや到達不能部分まで「Stringを使わない」と断定しない。
- 6場所×6receiverの36probe＋空/結合文字/emojiの3probe。文境界は改行。productionの試作はしていない。元input・log・hash・command runnerはLに保存した。

## 現行の経路（基点のfile:line）

`src/mir/`相対の位置:

| 段階 | 場所 | 決めている物／不足 |
| --- | --- | --- |
| 名前/意味の正本 | lang/src/runtime/meta/core_method_contract_box.hako:298–313、src/mir/core_method_result_kind.rs:188–216 | StringBoxのlength/len/size・arity0を同じStringLen rowへ解決。user boxの同名methodには流用しない |
| 元callの対応 | normal_callable_semantic_package/core_method_source/ordinary.rs:70–145 | 元resolved MethodCallのowner/site/Arcとsource ledgerを照合し、同じmanifestからStringBoxText targetを発行 |
| source意味 | resolved_semantics/core_method_instance_target.rs:450–468 | 同じmanifest/arity0/StringBoxText/結果i64/CodePointCountを確認。型と意味であり生存証拠ではない |
| 文字列の型 | normal_callable_semantic_package/ordinary_new_borrowed_formal_source_sum_graph.rs:419–435,524–552 | 元のtext-only SUMと同じsource/method/manifestをClosedOriginalCoreSourceへ。MIR型からの逆算ではない |
| literal/local | resolved_semantics/home_expression_leaves.rs:459、home_expression_value_preparation.rs:54,87–88 | original literal、static/residence/loop text reference |
| field | resolved_semantics/home_new_prefix_field_read.rs:524–562 | closed Text field sourceをprobeし、同じfield batchで借りる。読取copy取得のFault/EndはLengthとは別 |
| formal | normal_callable_semantic_package/ordinary_new_argument_expression_lending.rs:599–728、resolved_semantics/home_expression_leaves.rs:425–450 | 元formal read、flow、alias origin、finite entry requirementとplanを保持。実EXE activationは別 |
| concat/call結果 | resolved_semantics/home_expression_evaluation.rs:15–28、builder/ordinary_new_admission/selected/expression_value.rs:510–560 | 順序付きacquired/temporary/Endと同じ正常結果を保持。receiver取得はFaultしうる |
| local選択 | resolved_semantics/home_new_prefix_initializer_expression.rs:78–108 | 根の式を共通評価へ入れるactivation。目的ごとの別判定が残る |
| return/式内 | resolved_semantics/home_terminal_scalar_evaluation.rs:32–75 | bare method根を一般選択する条件はない。Add等や既存callee/所有条件で共通評価を選ぶ |
| print | resolved_semantics/home_new_prefix_print.rs:64–73、home_print_expression.rs:96 | 共通式評価。Print自体のFault責任を分ける |
| if | resolved_semantics/home_bool_condition.rs:117 | 同じ比較用のcall/Core準備。rawに残る場合もある |
| loop条件 | resolved_semantics/home_prefix_source_loop.rs:747 | source loop Bool productへ投影。一般receiver/条件の受理は保証しない |
| loop別経路 | source_call_target/core_method.rs:139–146,232–253 | body receiverはlocal一回initializer/no-rebind等に限定、parameterは未対応。共通Coreと同じと見なさない |
| frame query | resolved_semantics/home_expression_evaluation.rs:33–60 | coresはArrayCount以外ならFault frame必須。calls()だけではStringLenを見落とす |
| 共通writer | builder/ordinary_new_admission/selected/expression_value.rs:522–560、normal_callable_semantic_package/ordinary_new_local_commit/root_call_entry/expression_source_values.rs:426 | Array Countだけ非Invoke。StringLenはString(Length) Invoke＋require_fault_frameへ |
| raw選択 | generic_method_route_plan/collection_read_routes.rs:456–485 | receiver origin/旧len surfaceからStringBox経路を選ぶ。元NoFaultのsource issuerではない |
| raw helper | generated/generic_method_route_descriptors.rs:217–227 | StringLen→nyash.string.len_fast_h、read_ref/observe.len。軽さは意味証拠ではない |
| MIR | instruction/invoke_string.rs:7–38,76–99,151 | StaticText/OwnedString/BorrowedText/TextOperand等の既存operand、Lengthはi64。checked InvokeはREAD＋Control |
| source→publication | compiler/normal_default_pipeline/published_backend_view/physical_program_json.rs:383–385,433–436 | ArrayCountはarray_length、一般Callはbirth以外call-not-birth。この後段境界と今回の実測first stopは区別する |
| checked JSON | compiler/normal_default_pipeline/published_backend_view/physical_program_json_operation.rs:85–104,461–463 | Invoke.operation={kind:string_length,value,site}、Text operandは既存のtyped serialize |

C/runtime側の位置:

| 段階 | 場所 | 現状 |
| --- | --- | --- |
| strict JSON schema | lang/c-abi/shims/published_mir/hako_llvmc_ffi_published_lifecycle_instruction_schema.inc:253–265 | kind/value/siteのexact keys、Text専用operand、SSA使用位置 |
| C liveness | lang/c-abi/shims/published_mir/hako_llvmc_ffi_lifecycle_v4_indexed_flow.inc:109–120 | Invoke側のText状態検査。通常行にも新読取を接続する必要 |
| LLVM発行 | lang/c-abi/shims/published_mir/hako_llvmc_ffi_lifecycle_v4_string_emit.inc:214–228 | checked_length(frame,site,operand,out)→status0 Normal/1 Fault/他invalid |
| static globals/一時名 | 同string_emit.inc:21–46,135–158 | inlineではconst_stringだけ、String operandはterminator走査。新行のstatic/global/out slotも要る。同block内複数Lengthでblock+role名を使い回さずrow/dstで一意化 |
| checked runtime | crates/nyash_kernel/src/exports/fault_checked_string.rs:39–48,132–162,234–243 | frame必須。text_operand→UTF8/lease検証→String複製→chars.count。SourceFaultは書かないがInvalidContractは返す |
| raw runtime | crates/nyash_kernel/src/exports/string.rs:46–49、string_helpers.rs:96–111,149–166 | ready handleはtext.len()のbyte数。fallback/hookmiss/0もあり、正しいText・生存を証明しない |
| 仕様 | docs/reference/language/strings.md:5–18,107–124 | language CP。legacy default byte/NYASH_STR_CP noteとcanonical CPを混同しない。Dynamic/Faultの別責務はdynamic-operators.md／failure-outcome-relations.md |

現在のraw 13入力は実EXEで全て**source側の先行拒否**になった。今回call-not-birthをその13件の実測first stopとして報告しない。新probeのliteral-if/concat-ifはexplicit compatibility provider ny-llvmc未設置で停止したので、言語/実装の拒否やEXE成功に数えない。過去の相談記録の「rawがEXE手前で拒否」は観測範囲の記録であり、あらゆるraw backendの不可能性の証明ではない。

現行MIRの抜粋（全命令列ではない）:

```text
raw:     %3: Integer = call_method StringBox.length(%1) [recv: %1] [Known]
checked: invoke String(Length { value: StaticText { utf8: "あい" } })
         frame=%3 normal=BasicBlockId(2) fault=BasicBlockId(3)
         %4 = invoke.normal BasicBlockId(0)
```

## 生存検証：現状と必要な追加

`src/mir/verification/invoke_string_lifetime.rs:480–522`はowned値の許可use/回数を検査し、`:536–549`でNormal/Fault別Live状態をCFG探索する。しかし`:550–592`のblock内loopが扱うのはStringResidenceInstallだけ。Text operandの生存は**terminator Invoke**の`:610–645`で検査する。新しい通常行を追加してused_valuesに列挙するだけではuse-after-Endを防げない。

既存Live/alias/borrowed formalの検査を元の行順で再利用し、Static UTF8、BorrowedTextのdeclared entry、OwnedStringの同じ生きたpair、TextOperandの元descriptorを別々に照合する。結果i64の一意定義、全operandの支配、型、Fault後・replacement/hold/call_pending中の禁止、消費/取得が無いことを検査する。End後の長さ、別owner/token、alias/PHIの片側だけEnd、順序改変、frame/Fault偽装を拒否する。source側のloanを信じて独立検証を省略しない。

## ArrayRead::Countの前例と限界

| 段 | file:line | 保証 |
| --- | --- | --- |
| source | src/mir/resolved_semantics/core_method_instance_target.rs:429–434 | ArrayCount法則、arity/結果i64/引数なし |
| receiver | src/mir/normal_callable_semantic_package/core_method_source/ordinary.rs:168–183 | 同じplanの評価済receiver |
| writer | src/mir/builder/ordinary_new_admission/selected/expression_value.rs:522–536 | 同じfield-array recipe/source照合後ArrayRead::Count |
| MIR | src/mir/instruction.rs:292,377–381 | Count operation、frame/辺なし。READ effectはNoFault証拠ではない |
| Rust独立検証 | src/mir/verification/invoke_map.rs:198–201,288–295 | 主に追跡Map keys snapshotのlive handle。任意Array全般の証明ではない |
| JSON | src/mir/compiler/normal_default_pipeline/published_backend_view/physical_program_json.rs:383–385 | {op:array_length,array,dst} |
| C | lang/c-abi/shims/published_mir/hako_llvmc_ffi_published_lifecycle_instruction_schema.inc:677–682、hako_llvmc_ffi_lifecycle_v4_indexed_rows.inc:174–179 | exact keys/SSA＋live snapshotかfield-array read、faulted/birth拒否 |
| C生成/runtime | 同hako_llvmc_ffi_lifecycle_v4_emit.inc:425–427、crates/nyash_kernel/src/plugin/array_runtime_aliases.rs:236–237→crates/nyash_kernel/src/plugin/array_compat.rs:55–56 | slot_len_h。runtimeの不正handle→0はsource安全性の証拠ではない |

外形は良い前例だが、Stringにも寿命検査の抜けをコピーしてよいという意味ではない。

## 規則の案（未採択）

SL1. lengthの意味は既存の正確なStringLen/0 CodePointCount法則から得るUnicode code point数とする。
SL2. NoSourceFaultは元sourceのexact Text・UTF8不変条件・使用地点での生存と借用区間から出し、opcode・MirType・effect・PureRead・receiver名から出さない。
SL3. receiverをsource順に一度だけ評価し、field copy・concat・callの取得/Fault/Endを同じ評価ownerへ残す。
SL4. 軽いLength命令は非terminatorでFault frame operandもNormal/Fault辺も持たず、既存Root ABI・Print・receiver取得のframeを消す許可にはしない。
SL5. 同じCore source loanの結果i64をlocal・return・print・条件・式へ渡し、目的ごとのlength producerや再分類を足さない。
SL6. Lengthはreceiverの所有権を取得・移転・消費せず、元のEnd数・順序・Normal/Fault cleanupを保持する。
SL7. 独立MIR検証とC状態検証は通常行の各使用位置でText生存・alias/PHI・型・支配を確認し、借用区間の終了後やEnd後の不正使用を拒否する。
SL8. ABI破損は結果を出さずInvalidContractとし、source Fault・ゼロ長・retry・legacyへ変換しない。
SL9. 正常なTextにはStringify・provider・Dynamic tagの推測・全String複製を入れず、CP計数を同じ借用callback内で終える。
SL10. 未証明receiverやuser boxの同名methodは軽いStringLengthにせず、元の契約によるchecked/名前付き拒否を保持する。
SL11. raw source lengthは軽いcanonical経路が同じ入力で閉じてからcallerを切り替えて削除し、他のbyte API/compat callerが使うexportは残す。
SL12. 新旧比較はsource意味のCP修正と構造の軽量化を分け、ASCII一致だけでUnicodeや全backendの不変を主張しない。
SL13. 新公開形とruntime ABIはmaintainer決定後にだけ実装し、旧readerは未知rowを黙認せずartifact生成前に拒否する。
SL14. 既存Text operand型と既存のsource/query/Live ownerへ接続し、新しい正規形・型分類器・寿命台帳を作らない。

## 公開形の案（2案、いずれも未実装）

### 案A：専用StringLength（推奨）

- MIR: `StringLength { dst: ValueId, value: StringInvokeOperand }`。dst必須i64、定義/使用/置換/pretty printer/serde/backendのmatchへ接続。印字案 `%d: i64 = string.length.cp <typed-text>`。既存operandの4 Text形だけを許しInteger/Bool/Dynamic/Carrierは拒否。
- JSON行案: `{"op":"string_length","dst":7,"value":{"kind":"static_text","utf8":"あい"}}`。owned/borrowed/text_operandは既存{kind,value}を使用。frame/normal/fault/siteを意味上不要なキーとして足さない。
- schema案: `hako.published-lifecycle-physical-program.v2`の閉語彙に上記のexact rowを追加し、既存rowの意味/shapeは不変、Fault ABI versionは1のまま。旧Cはunknown opで拒否する。v3へ全面更新するより小さい変更として提案するが、公開version方針はmaintainerの承認対象。
- 独立検証: 前節の行順lifetime、exact operand/use回数とi64結果、非terminator、Fault/frameキーや不正なresultを拒否。新opがあるという事実からNoFaultを逆算しない。
- C: exact keys、Text専用operand/SSA使用位置、i64 dstと一意性、現在地点のlease/descriptor生存を検査。literal globals/output slotをinline行走査へ追加し、operand emitterの名前にfi/block/rowまたはdstを使用する。LLVMは既存descriptorを作りframe不要のCP関数を呼び、status0で次行、それ以外は既存invalidへ。source Fault辺は発行しない。
- runtime案: `uint32_t nyrt_string_code_point_count_v1(const NyrtStringOperandV1*, int64_t* out)`、export `nyash.string.code_point_count_v1`。正常0/InvalidContract2、frame/siteなし、全inactive fieldとUTF8/handle-token pairを検証。正常時だけoutを書き換える。descriptor/out/静的bytesには有効な呼出し期間のstorageを渡す既存unsafe ABIの前提を保持し、任意の不正アドレスをstatus検査で安全化できるとは扱わない。checked_length_v1とのprivate count coreを共用し、staticは借用UTF8、owned/borrowedは既存lease callback内でcountしてString複製/新lease/Endをしない。
- `checked_length_v1`のnull frame呼出しは不可（fault_checked_string.rs:236–238）。旧public ABIは互換callerがある間保持する。CP countはUTF8 bytesに対してO(n)であり、枠なしをO(1)・ロックなし・性能勝利とは呼ばない。
- Array Countと揃う点は非terminator/typed source/i64/closed row/直接leaf呼出し。違いはText descriptorとEnd/borrowの生存、UTF8/CP、InvalidContractの保護。bare i64 handleを流用しない。

### 案B：StringRead::Length

- MIR: `StringRead { dst: ValueId, value: StringInvokeOperand, operation: StringReadOperationV1::Length }`。印字案 `%d: i64 = string.read Length <typed-text>`。操作enumはLengthだけ。他のfallible StringInvokeOperationを丸ごと受ける箱にはしない。
- JSON行案: `{"op":"string_read","kind":"length","dst":7,"value":{"kind":"static_text","utf8":"あい"}}`。kindの列挙もexact、frame/normal/fault等は拒否。schema v2の閉語彙をこの1行だけ増やす案で、Fault ABIは不変。
- source・独立検証・C/LLVM・runtime ABI・4 operand domain・InvalidContractの分岐は案Aと同じ。operation kindを追加する時も個別のsource法則を必須とし、enum membershipでNoFaultを作らない。
- ArrayReadと同じread+operation familyになる利点がある。現在一操作だけの新enumとmatchが増え、実在しない将来操作への一般化を先行しやすい危険がある。今回はAより約10–30行追加になる見積もり。

### 共通のInvalidContractとroot境界

`crates/nyash_kernel/src/exports/fault.rs:131–134`はNormal0/Fault1/Invalid2。C `hako_llvmc_ffi_lifecycle_v4_emit.inc:720–725`はordinary/birthでFault=ret1とInvalid=ret2を分け、rootではFaultだけreport、Invalidは既存frameをdisposeしてexit70。新Lengthは0以外を既存invalidへ送る（1も意味Faultへは送らない）。これがNoSourceFaultでも不正artifactを正常値に化けさせない経路になる。Root V4 frame自体は今回の案で除去しない。

### 行数・影響・危険（見積もり、実測diffではない）

| 層 | 案A 追加production | 案Bとの差／注意 |
| --- | --- | --- |
| MIRとvisitors/use-def/印字/型/serde/backend | 70–140 | Bはoperation enum等で+10–30 |
| 既存source Core/queryとconsumer/projection | 50–100 | 目的×receiverの専用fileを作らない |
| Rust独立lifetime・SSA・型 | 60–120 | 通常行loopとowned use許可へ接続 |
| JSON/strict schema/publication | 30–60 | 一般Callの許可拡大で代用しない |
| C globals/indexed state/LLVM | 60–120 | 同block複数Lengthの一時名衝突を避ける |
| runtime/header/shared count core | 30–60 | cloneなし、callback内にpublish/Endを入れてdeadlockさせない |
| A追加合計 | 300–600 | 各rangeの単純和は300–600、Bは310–630。未実装 |
| raw source-route退役候補 | −20〜−60 | 下記のcaller-zero後のみ。generated表はgeneratorから再生成 |
| test | +80〜+180程度 | 別集計。既存String/ABIの正負・改変検査へ統合し重複を削る |

案Aのfeature差分は暫定+240〜+580、Bは+250〜+610程度であり、純減を保証しない。小さいenumの差より検証の接続の方が主な費用。受理の未接続・loop/Formalの前段不足はこの見積もりの不確実性であり、全36組が同じ費用で直る見積もりではない。

既にEXEが動くchecked経路はCPの出力を維持できる設計。NoFault length部分のInvoke/normal_result/辺を除去するが、whole-functionの正確な命令削減値は未試作のため未測定。Print・field copy・concat・callのframe/EndとRoot ABIは残る。rawはbyte→CPへ意味が修正されるため、全backendの出力不変を契約にしない。今回の既存入力にraw成功EXEはなかったが、compat provider未設置もあり一般化しない。

### 実装する場合の順番（選択待ち）

1. maintainerがMIR/JSON/ABIを承認し、既存Core loanのText+生存証拠にNoSourceFault queryを接続する。
2. Rust/Cの行内lifetime・schema・codegen・runtimeを一つの既存Local Length EXEまで閉じる。空/ASCII/多byte/結合文字/emojiとEnd後/foreign token/型/複数同block/未知keyの拒否を既存testへ統合する。
3. 同じsource評価からreturn/print/if/loop/式へ結果を渡し、既存receiver取得とcleanupを保持する。formalとloopの未接続を明示し、別のframeやValueIdを偽装しない。e17は開けない。
4. 旧source StringLen Callのcallerを共通source routeへ切替え、old raw writer/selectorとその専用testを同seriesで退役する。collection_read_routes.rs:456–485のString分岐、生成descriptor:217–227、該当C generic length分岐が候補。
5. len_fast_h export自体、Cのsubstring/indexOfのbyte長利用、VM/reference/llvm_pyのcompat callerはcaller-zeroを別に確認するまで残す（例: string.rs:47、same_module_generic_method_string_emit.inc:83、same_module_generic_method_collection_emit.inc:607、src/llvm_py/generated/generic_method_route_registry.py:168）。他APIのbyte契約を今回のCP計数へ黙って置換しない。

## 実測：36組＋Unicode3本

I/B/F/Vはmodule全体の命令/block/FaultFrameEnter/Invoke。Lengthそのものの差とは別。`dump-refused`で0/0/0/0なのはMIRを得られなかったためで、軽量化/NoFaultの証拠ではない。formal6件は共通してpinned-text-backend-frame-mismatchであり、borrowed Stringの型/lawが存在しないという結論にはしない。call-result-localはsource-unobserved、call-result-expressionはEXE成功という差もある。


| receiver / 場所 | dump route | I/B/F/V | 正規EXE／first stop |
| --- | --- | --- | --- |
| literal-local | checked | 8/5/1/2 | EXE exit 2 |
| literal-return | dump-refused | 0/0/0/0 | main-arguments/root-completion |
| literal-print | dump-refused | 0/0/0/0 | ordinary-new/local-commit/print-expression/source-unavailable |
| literal-if | raw | 9/3/0/0 | compat-provider未設置（実装拒否とは別） |
| literal-loop | dump-refused | 0/0/0/0 | callable-loop/route-not-front-selected |
| literal-expression | checked | 10/5/1/2 | EXE exit 3 |
| local-local | checked | 8/5/1/2 | EXE exit 2 |
| local-return | dump-refused | 0/0/0/0 | ordinary-new/root-instance-call-new-missing |
| local-print | dump-refused | 0/0/0/0 | ordinary-new/local-commit/print-expression/source-unavailable |
| local-if | raw | 9/3/0/0 | ordinary-new/object-return/normal-projection-exit |
| local-loop | dump-refused | 0/0/0/0 | callable-loop/route-not-front-selected |
| local-expression | checked | 10/5/1/2 | EXE exit 3 |
| field-local | checked | 71/58/2/22 | EXE exit 2 |
| field-return | raw | 29/16/2/6 | ordinary-new/local-commit/artifact-source-unavailable |
| field-print | dump-refused | 0/0/0/0 | ordinary-new/local-commit/print-expression/source-unavailable |
| field-if | raw | 35/18/2/6 | ordinary-new/local-commit/artifact-source-unavailable |
| field-loop | dump-refused | 0/0/0/0 | callable-loop/route-not-front-selected |
| field-expression | checked | 73/58/2/22 | EXE exit 3 |
| formal-local | dump-refused | 0/0/0/0 | mir/resolved-child/pinned-text-backend-frame-mismatch |
| formal-return | dump-refused | 0/0/0/0 | mir/resolved-child/pinned-text-backend-frame-mismatch |
| formal-print | dump-refused | 0/0/0/0 | ordinary-new/scalar-loan/original-missing |
| formal-if | dump-refused | 0/0/0/0 | ordinary-new/scalar-loan/original-missing |
| formal-loop | dump-refused | 0/0/0/0 | mir/resolved-child/pinned-text-backend-frame-mismatch |
| formal-expression | dump-refused | 0/0/0/0 | mir/resolved-child/pinned-text-backend-frame-mismatch |
| concat-local | checked | 16/12/1/5 | EXE exit 2 |
| concat-return | dump-refused | 0/0/0/0 | main-arguments/root-completion |
| concat-print | dump-refused | 0/0/0/0 | ordinary-new/local-commit/print-expression/source-unavailable |
| concat-if | raw | 11/3/0/0 | compat-provider未設置（実装拒否とは別） |
| concat-loop | dump-refused | 0/0/0/0 | callable-loop/route-not-front-selected |
| concat-expression | checked | 18/12/1/5 | EXE exit 3 |
| call-result-local | dump-refused | 0/0/0/0 | ordinary-new/borrowed-static/source-unobserved |
| call-result-return | dump-refused | 0/0/0/0 | mir/main-import-view/selected-header-missing |
| call-result-print | dump-refused | 0/0/0/0 | mir/main-import-view/selected-header-missing |
| call-result-if | dump-refused | 0/0/0/0 | mir/main-import-view/selected-header-missing |
| call-result-loop | dump-refused | 0/0/0/0 | ordinary-new/borrowed-static/source-unobserved |
| call-result-expression | checked | 25/15/2/6 | EXE exit 3 |
| unicode-empty | checked | 8/5/1/2 | EXE exit 0 |
| unicode-combining | checked | 8/5/1/2 | EXE exit 2 |
| unicode-emoji | checked | 8/5/1/2 | EXE exit 1 |


Unicode実測は「あい」CP2/byte6、空0/0、`é` CP2/byte3（grapheme1）、emoji CP1/byte4。checked EXEのlocal形はそれぞれexit2/0/2/1。legacy helperのbyte数は実装読取で確認したもので、新raw Unicode EXEの正常出力を実測できたとは主張しない。

## 入力一覧（既存2718入力union内）

60入力でString lengthを特定（raw13、checked47）。checkedのうち45が正規EXEへ到達、2は別の先行拒否。raw13は全てsourceの先行拒否。候補の残りは66件の前段拒否・型/操作未分類と、21件のString Length形がないdump受理である。923追加dumpから候補漏れのString Length発見は0。

下の各sourceはpopulation.jsonに元の群(dump1788/EXE229/apps100/384/Opus/unseen)を保持する。EXE成功8件は--dump-mir側と経路が違うため、実際のbackend入力TempFileを読取りコピーしてJSONのInvoke string_lengthを確認した（capture-results.json）。JのI/B/F/Vは公開JSONの命令数で、MIR数と取り違えない。全source/hash/length行/関数別MIR数/生logはmeasurements.json、各out/<key>.*に残す。


| source | 経路 | I/B/F/V | EXE／first stop |
| --- | --- | --- | --- |
| /mnt/workdisk/claude-scratch/audit-bool/in/d1.hako | checked | M 61/44/3/17 | EXE exit 0 |
| /mnt/workdisk/claude-scratch/audit-bool/in/d4.hako | checked | M 66/49/3/19 | EXE exit 0 |
| /mnt/workdisk/claude-scratch/audit-bool/in/d5.hako | checked | M 81/62/3/24 | EXE exit 0 |
| /mnt/workdisk/claude-scratch/audit-bool/in/d7.hako | checked | M 70/51/3/20 | EXE exit 0 |
| /mnt/workdisk/claude-scratch/audit-bool/in/s1.hako | checked | M 57/42/3/16 | EXE exit 0 |
| /mnt/workdisk/claude-scratch/basic-expr/targets/e08.hako | checked | M 8/5/1/2 | EXE exit 3 |
| /mnt/workdisk/claude-scratch/bool-result/in/agg1.hako | checked(JSON実測) | J 2648/2031/16/716 | EXE exit 0 |
| /mnt/workdisk/claude-scratch/bool-result/in/agg2.hako | checked(JSON実測) | J 2648/2031/16/716 | EXE exit 0 |
| /mnt/workdisk/claude-scratch/bool-result/in/d1.hako | checked | M 62/46/3/18 | EXE exit 0 |
| /mnt/workdisk/claude-scratch/bool-result/in/d10.hako | checked | M 70/53/3/21 | EXE exit 0 |
| /mnt/workdisk/claude-scratch/bool-result/in/d2.hako | checked | M 64/46/3/18 | EXE exit 0 |
| /mnt/workdisk/claude-scratch/bool-result/in/d3.hako | raw | M 59/44/2/17 | ordinary-new/object-return/normal-projection-exit |
| /mnt/workdisk/claude-scratch/bool-result/in/d8.hako | checked | M 70/53/3/21 | EXE exit 0 |
| /mnt/workdisk/claude-scratch/bool-result/in/d9.hako | checked | M 72/53/3/21 | EXE exit 0 |
| /mnt/workdisk/claude-scratch/bool-result/in/g3d.hako | checked | M 143/91/5/34 | mir/invoke/call-result-type-drift |
| $R/apps/boxtorrent-mini/main.hako | checked(JSON実測) | J 6165/5019/42/1721 | EXE exit 0 |
| $R/apps/json-stream-aggregator/main.hako | checked(JSON実測) | J 2648/2031/16/716 | EXE exit 0 |
| $R/apps/tests/phase29bq_selfhost_blocker_parse_stmt_skipws_min.hako | checked | M 52/25/1/7 | EXE exit 0 |
| $S0/ir/in/app_boxtorrent-mini.hako | checked(JSON実測) | J 6165/5019/42/1721 | EXE exit 0 |
| $S0/ir/in/app_json-stream-aggregator.hako | checked(JSON実測) | J 2648/2031/16/716 | EXE exit 0 |
| $S/audit-text-field/f_read_typed.hako | raw | M 43/29/2/11 | ordinary-new/object-return/normal-projection-exit |
| $S/audit-text-read/in/a3.hako | checked | M 63/50/2/19 | EXE exit 3 |
| $S/audit-text-read/in/b1.hako | checked | M 107/92/2/35 | EXE exit 3 |
| $S/audit-text-read/in/c2.hako | raw | M 29/16/2/6 | ordinary-new/local-commit/artifact-source-unavailable |
| $S/audit-text-read/in/c9.hako | raw | M 37/19/2/6 | ordinary-new/local-commit/artifact-source-unavailable |
| $S/audit-warn/in/w1.hako | checked | M 72/53/3/21 | EXE exit 0 |
| $S/audit-warn/in/w10.hako | checked | M 78/56/3/21 | EXE exit 0 |
| $S/audit-warn/in/w14.hako | checked | M 102/77/3/31 | EXE exit 0 |
| $S/audit-warn/in/w3.hako | checked | M 79/55/3/21 | EXE exit 0 |
| $S/audit-warn/in/w6.hako | checked | M 62/46/3/18 | EXE exit 0 |
| $S/audit-warn/in/w7.hako | checked | M 52/30/3/12 | ordinary-new/local-commit/artifact-source-unavailable |
| $S/audit-warn/in/w8.hako | checked | M 91/58/3/21 | EXE exit 0 |
| $S/audit-warn/in/w9.hako | checked | M 72/53/3/21 | EXE exit 0 |
| $S/audit-warn/in/x2.hako | checked | M 57/43/2/17 | EXE exit 0 |
| $S/audit-warn/in/x6.hako | raw | M 59/44/2/17 | ordinary-new/object-return/normal-projection-exit |
| $S/audit-warn/in/x7.hako | checked | M 64/46/3/18 | EXE exit 0 |
| $S/fixed-16/app_boxtorrent-mini.hako | checked(JSON実測) | J 6165/5019/42/1721 | EXE exit 0 |
| $S/fixed-16/app_json-stream-aggregator.hako | checked(JSON実測) | J 2648/2031/16/716 | EXE exit 0 |
| $S/forms/in/r05.hako | checked | M 63/50/2/19 | EXE exit 3 |
| $S/forms/in/r07.hako | raw | M 29/16/2/6 | ordinary-new/local-commit/artifact-source-unavailable |
| $S/forms/in/r09.hako | raw | M 37/19/2/6 | ordinary-new/local-commit/artifact-source-unavailable |
| $S/mir-chain/t/len.hako | checked | M 58/42/3/17 | EXE exit 3 |
| $S/plain/in/m12.hako | checked | M 60/24/1/6 | EXE exit 0 |
| $S/plain/in/p6.hako | checked | M 48/22/1/6 | EXE exit 0 |
| $S/plain/in/t2.hako | checked | M 9/5/1/2 | EXE exit 0 |
| $S/plain/in/t6c.hako | checked | M 31/11/1/2 | EXE exit 0 |
| $S/tr/in/r1.hako | raw | M 43/29/2/11 | ordinary-new/object-return/normal-projection-exit |
| $S/tr/in/r2.hako | raw | M 29/16/2/6 | ordinary-new/local-commit/artifact-source-unavailable |
| $S/tr/in/r3b.hako | raw | M 29/16/2/6 | ordinary-new/local-commit/artifact-source-unavailable |
| $S/tr/in4/l1.hako | checked | M 63/50/2/19 | EXE exit 3 |
| $S/tr/in4/m1.hako | checked | M 53/38/3/15 | EXE exit 3 |
| $S/tr/in4/m2.hako | raw | M 47/32/2/12 | ordinary-new/object-return/normal-projection-exit |
| $S/tr/in6/c2.hako | raw | M 6/1/0/0 | ordinary-new/object-return/normal-projection-exit |
| $S/tr/in6/c4.hako | checked | M 6/3/1/1 | EXE exit 3 |
| $S/tr/in6/c5.hako | checked | M 14/10/1/4 | EXE exit 2 |
| $S/tr/in6/c6.hako | checked | M 63/50/2/19 | EXE exit 3 |
| $S/tr/in8/w5.hako | raw | M 34/20/2/8 | ordinary-new/local-commit/artifact-source-unavailable |
| $S/tr/in9/e6.hako | checked | M 65/50/2/19 | EXE exit 4 |
| $S/tr/in9/e7.hako | checked | M 98/77/2/29 | EXE exit 6 |
| $S/triage-work/fc3.hako | checked | M 21/15/1/6 | EXE exit 0 |


同じ群でlength綴りを持つがString経路を特定できない/観測しない候補も省かず示す。Array/Mapやuser methodをStringへ誤分類しない。


| source | 観測 | EXE／first stop |
| --- | --- | --- |
| /mnt/workdisk/claude-scratch/audit-bool/in/d2.hako | dump-refused | ordinary-new/borrowed-static/incoming-missing |
| /mnt/workdisk/claude-scratch/audit-bool/in/d6.hako | dump-refused | ordinary-new/local-commit/print-expression/source-unavailable |
| /mnt/workdisk/claude-scratch/audit-claim/in/c10.hako | dump-refused | ordinary-new/borrowed-static/source-actual-unavailable |
| /mnt/workdisk/claude-scratch/basic-expr/targets/e07.hako | dump-refused | ordinary-new/root-instance-call-new-missing |
| /mnt/workdisk/claude-scratch/basic-expr/targets/e10.hako | dump-refused | array-holder/append-source |
| /mnt/workdisk/claude-scratch/bool-result/in/d4.hako | dump-refused | mir/resolved-child/pinned-text-backend-frame-mismatch |
| /mnt/workdisk/claude-scratch/bool-result/in/d5.hako | no-string-length-in-MIR | EXE exit 0 |
| /mnt/workdisk/claude-scratch/bool-result/in/d6.hako | dump-refused | mir/callable-semantic-package/port |
| /mnt/workdisk/claude-scratch/bool-result/in/d7.hako | dump-refused | ordinary-new/borrowed-result/field-component-unsealed |
| /mnt/workdisk/claude-scratch/bool-result/in/g3u.hako | dump-refused | ordinary-new/borrowed-entry/source-use-completion-pending |
| /mnt/workdisk/claude-scratch/mw-audit/in/w06.hako | dump-refused | array-holder/append-source |
| /mnt/workdisk/claude-scratch/mw-audit/in/w09.hako | dump-refused | ordinary-new/local-commit/print-expression/source-unavailable |
| /mnt/workdisk/claude-scratch/mw-audit/in/w14.hako | dump-refused | published-lifecycle-program/array-set-unsupported |
| /mnt/workdisk/claude-scratch/s4-audit/in/p09.hako | dump-refused | ordinary-new/local-commit/print-expression/source-unavailable |
| $S0/ir/in/text_birth_string.hako | no-string-length-in-MIR | EXE exit 5 |
| $S0/ir/in/text_literal.hako | no-string-length-in-MIR | EXE exit 5 |
| $S0/ir/in/text_nullable.hako | no-string-length-in-MIR | EXE exit 5 |
| $S0/ir/in/text_untyped.hako | no-string-length-in-MIR | EXE exit 5 |
| $S/audit-text-field/a_lit_i64.hako | no-string-length-in-MIR | ordinary-new/local-commit/artifact-source-unavailable |
| $S/audit-text-field/b_map.hako | no-string-length-in-MIR | ordinary-new/local-commit/artifact-source-unavailable |
| $S/audit-text-field/c_obj.hako | no-string-length-in-MIR | ordinary-new/local-commit/artifact-source-unavailable |
| $S/audit-text-field/d_ok.hako | no-string-length-in-MIR | EXE exit 5 |
| $S/audit-text-field/e_read_untyped.hako | dump-refused | callable-scalar/return-type-mismatch |
| $S/audit-text-field/g_main_read.hako | no-string-length-in-MIR | ordinary-new/local-commit/artifact-source-unavailable |
| $S/audit-text-field/h_null.hako | no-string-length-in-MIR | ordinary-new/local-commit/artifact-source-unavailable |
| $S/audit-text-field/i_opt_formal.hako | no-string-length-in-MIR | ordinary-new/local-commit/artifact-source-unavailable |
| $S/audit-text-field/j_get.hako | dump-refused | ordinary-new/root-instance-call-result-kind-unavailable |
| $S/audit-text-field/k_local.hako | dump-refused | ordinary-new/borrowed-result/field-component-unsealed |
| $S/audit-text-field/n_return_len.hako | no-string-length-in-MIR | EXE exit 5 |
| $S/audit-text-read/in/c3.hako | dump-refused | ordinary-new/root-instance-call-new-missing |
| $S/audit-text-read/in/c4.hako | dump-refused | ordinary-new/local-commit/print-expression/source-unavailable |
| $S/audit-triage/in/s6.hako | dump-refused | ordinary-new/borrowed-static/source-actual-unavailable |
| $S/audit-warn/in/w11.hako | dump-refused | ordinary-new/borrowed-result/source-not-i64 |
| $S/audit-warn/in/w12.hako | dump-refused | ordinary-new/borrowed-result/source-not-i64 |
| $S/audit-warn/in/w13.hako | dump-refused | ordinary-new/local-commit/lexical-call-prior-home-unavailable |
| $S/audit-warn/in/w2.hako | dump-refused | mir/callable-semantic-package/port |
| $S/audit-warn/in/x1.hako | dump-refused | ordinary-new/borrowed-result/field-component-unsealed |
| $S/audit-warn/in/x3.hako | dump-refused | ❌ MIR compilation error: [mir/callable-semantic-package/issue] Batch { _error: Resolver(SourceBoundSelectedCallableResolverRejectV1 { source: Callable { identity: CallableDeclarationIdentityV1(()), diagnostic_owner: Some("Main"), diagnostic_name: "main" }, error: Function(AppMainDirectCall(TargetMissing)) }) } |
| $S/audit-warn/in/x4.hako | dump-refused | mir/resolved-child/pinned-text-backend-frame-mismatch |
| $S/audit-warn/in/x5.hako | dump-refused | ordinary-new/borrowed-result/source-not-i64 |
| $S/audit-warn/in/x8.hako | dump-refused | ❌ MIR compilation error: [mir/callable-semantic-package/issue] Batch { _error: Resolver(SourceBoundSelectedCallableResolverRejectV1 { source: Callable { identity: CallableDeclarationIdentityV1(()), diagnostic_owner: Some("Main"), diagnostic_name: "main" }, error: Function(AppMainDirectCall(TargetMissing)) }) } |
| $S/fixed-16/text_birth_string.hako | no-string-length-in-MIR | EXE exit 5 |
| $S/fixed-16/text_literal.hako | no-string-length-in-MIR | EXE exit 5 |
| $S/fixed-16/text_nullable.hako | no-string-length-in-MIR | EXE exit 5 |
| $S/fixed-16/text_untyped.hako | no-string-length-in-MIR | EXE exit 5 |
| $S/forms/in/r08.hako | dump-refused | ordinary-new/local-commit/print-expression/source-unavailable |
| $S/plain/in/a1.hako | dump-refused | array-holder/append-source |
| $S/plain/in/a6.hako | dump-refused | array-holder/append-source |
| $S/plain/in/a7.hako | dump-refused | array-holder/append-source |
| $S/plain/in/m1.hako | no-string-length-in-MIR | ordinary-new/object-return/normal-projection-exit |
| $S/plain/in/t10.hako | dump-refused | ordinary-new/borrowed-entry/source-use-completion-pending |
| $S/plain/in/t6.hako | dump-refused | callable-loop/route-not-front-selected |
| $S/plain/in/t7.hako | dump-refused | callable-loop/route-not-front-selected |
| $S/plain/in/t7c.hako | dump-refused | ordinary-new/local-commit/print-expression/source-unavailable |
| $S/plain/in/t9.hako | dump-refused | ordinary-new/borrowed-static/source-actual-unavailable |
| $S/plain/in/x1.hako | dump-refused | callable-loop/route-not-front-selected |
| $S/red-sort/in/mg.hako | no-string-length-in-MIR | ordinary-new/object-return/normal-projection-exit |
| $S/tr/in/r3a.hako | dump-refused | ordinary-new/borrowed-result/field-component-unsealed |
| $S/tr/in/r5a.hako | dump-refused | ordinary-new/borrowed-result/source-not-i64 |
| $S/tr/in/r6.hako | dump-refused | ordinary-new/root-instance-call-result-kind-unavailable |
| $S/tr/in3/u1.hako | dump-refused | ordinary-new/root-instance-call-result-kind-unavailable |
| $S/tr/in3/u3.hako | dump-refused | callable-scalar/return-type-mismatch |
| $S/tr/in3/u4.hako | no-string-length-in-MIR | ordinary-new/local-commit/artifact-source-unavailable |
| $S/tr/in4/p1.hako | dump-refused | ordinary-new/local-commit/print-expression/source-unavailable |
| $S/tr/in4/p2.hako | dump-refused | ordinary-new/local-commit/print-expression/source-unavailable |
| $S/tr/in5/g3.hako | no-string-length-in-MIR | ordinary-new/local-commit/artifact-source-unavailable |
| $S/tr/in6/c1.hako | dump-refused | ordinary-new/local-commit/print-expression/source-unavailable |
| $S/tr/in6/c3.hako | dump-refused | ordinary-new/root-instance-call-new-missing |
| $S/tr/in9/h1.hako | dump-refused | ordinary-new/local-commit/print-expression/source-unavailable |
| $S/triage-work/fc1.hako | dump-refused | forwarded-copy/root |
| $S/triage-work/fc2.hako | dump-refused | forwarded-copy/root |
| $S/triage-work/fc4.hako | dump-refused | forwarded-copy/root |
| $S/write-compare-b/b1_tokenizer.hako | dump-refused | ❌ Parse error in $S/write-compare-b/b1_tokenizer.hako: Unexpected token STRING("let"), expected `(` or `{` after `Type::Variant` at line 20 |
| $S/write-compare-b/b2_parser.hako | dump-refused | ❌ Parse error in $S/write-compare-b/b2_parser.hako: Unexpected token COLON, expected LBRACE at line 67 |
| $S/write-compare-b/b3_checker.hako | dump-refused | ❌ Parse error in $S/write-compare-b/b3_checker.hako: Unexpected token COLON, expected method or field name at line 8 |
| $S/write-compare-b/b4_span_pass.hako | dump-refused | ❌ MIR compilation error: [mir/callable-semantic-package/issue] Batch { _error: ResolverDeferred(SelectedCallableResolverDeferredBatchV1 { first: SelectedCallableResolverDeferredV1 { source: Callable { identity: CallableDeclarationIdentityV1(()), diagnostic_owner: Some("SpanPass"), diagnostic_name: "coalesce" }, observation: Located { cause: UnsupportedStatement { kind: "LoopRange" }, site: Statement(SourceStmtSiteV1(SourceNodeSiteV1([Body(1)]))) } }, rest: [SelectedCallableResolverDeferredV1 { source: Callable { identity: CallableDeclarationIdentityV1(()), diagnostic_owner: Some("SpanPass"), diagnostic_name: "widenBlock" }, observation: Located { cause: UnsupportedStatement { kind: "LoopRange" }, site: Statement(SourceStmtSiteV1(SourceNodeSiteV1([Body(0)]))) } }, SelectedCallableResolverDeferredV1 { source: Callable { identity: CallableDeclarationIdentityV1(()), diagnostic_owner: Some("SpanPass"), diagnostic_name: "paramsSpan" }, observation: Located { cause: UnsupportedStatement { kind: "LoopRange" }, site: Statement(SourceStmtSiteV1(SourceNodeSiteV1([Body(1)]))) } }, SelectedCallableResolverDeferredV1 { source: Callable { identity: CallableDeclarationIdentityV1(()), diagnostic_owner: Some("SpanPass"), diagnostic_name: "countSame" }, observation: Located { cause: UnsupportedStatement { kind: "LoopRange" }, site: Statement(SourceStmtSiteV1(SourceNodeSiteV1([Body(1)]))) } }, SelectedCallableResolverDeferredV1 { source: Callable { identity: CallableDeclarationIdentityV1(()), diagnostic_owner: Some("DiagPass"), diagnostic_name: "indexOfMin" }, observation: Located { cause: UnsupportedStatement { kind: "LoopRange" }, site: Statement(SourceStmtSiteV1(SourceNodeSiteV1([Body(1)]))) } }] }) } |
| $S/write-compare-b/b5_points.hako | dump-refused | ❌ Parse error in $S/write-compare-b/b5_points.hako: Unexpected token SOME, expected identifier at line 171 |
| $S/write-compare-b/b6_results_args.hako | dump-refused | parser/move_exact_root_required |
| $S/write-compare-b/b7_symbols.hako | dump-refused | ❌ Parse error in $S/write-compare-b/b7_symbols.hako: Unexpected token LBRACK, expected LBRACE at line 24 |
| $S/write-compare-b/r1_tokenizer.hako | dump-refused | ❌ Parse error in $S/write-compare-b/r1_tokenizer.hako: Unexpected token STRING("let"), expected `(` or `{` after `Type::Variant` at line 20 |
| $S/write-compare-b/r2_parser.hako | dump-refused | ❌ Parse error in $S/write-compare-b/r2_parser.hako: Unexpected token COLON, expected LBRACE at line 66 |
| $S/write-compare-b/r3_checker.hako | dump-refused | ❌ Parse error in $S/write-compare-b/r3_checker.hako: Unexpected token COLON, expected method or field name at line 5 |
| $S/write-compare-b/r4_span_pass.hako | dump-refused | ❌ MIR compilation error: [mir/callable-semantic-package/issue] Batch { _error: ResolverDeferred(SelectedCallableResolverDeferredBatchV1 { first: SelectedCallableResolverDeferredV1 { source: Callable { identity: CallableDeclarationIdentityV1(()), diagnostic_owner: Some("SpanPass"), diagnostic_name: "coalesce" }, observation: Located { cause: UnsupportedStatement { kind: "LoopRange" }, site: Statement(SourceStmtSiteV1(SourceNodeSiteV1([Body(1)]))) } }, rest: [SelectedCallableResolverDeferredV1 { source: Callable { identity: CallableDeclarationIdentityV1(()), diagnostic_owner: Some("SpanPass"), diagnostic_name: "widenBlock" }, observation: Located { cause: UnsupportedStatement { kind: "LoopRange" }, site: Statement(SourceStmtSiteV1(SourceNodeSiteV1([Body(0)]))) } }, SelectedCallableResolverDeferredV1 { source: Callable { identity: CallableDeclarationIdentityV1(()), diagnostic_owner: Some("SpanPass"), diagnostic_name: "paramsSpan" }, observation: Located { cause: UnsupportedStatement { kind: "LoopRange" }, site: Statement(SourceStmtSiteV1(SourceNodeSiteV1([Body(1)]))) } }, SelectedCallableResolverDeferredV1 { source: Callable { identity: CallableDeclarationIdentityV1(()), diagnostic_owner: Some("SpanPass"), diagnostic_name: "countSame" }, observation: Located { cause: UnsupportedStatement { kind: "LoopRange" }, site: Statement(SourceStmtSiteV1(SourceNodeSiteV1([Body(1)]))) } }, SelectedCallableResolverDeferredV1 { source: Callable { identity: CallableDeclarationIdentityV1(()), diagnostic_owner: Some("DiagPass"), diagnostic_name: "indexOfMin" }, observation: Located { cause: UnsupportedStatement { kind: "LoopRange" }, site: Statement(SourceStmtSiteV1(SourceNodeSiteV1([Body(1)]))) } }] }) } |
| $S/write-compare-b/r5_points.hako | dump-refused | ❌ Parse error in $S/write-compare-b/r5_points.hako: Unexpected token SOME, expected identifier at line 168 |
| $S/write-compare-b/r6_results_args.hako | dump-refused | ❌ Parse error in $S/write-compare-b/r6_results_args.hako: Unexpected token COMMA, expected `(` or `{` after `Type::Variant` at line 33 |
| $S/write-compare-b/r7_symbols.hako | dump-refused | ❌ Parse error in $S/write-compare-b/r7_symbols.hako: Unexpected token LBRACK, expected LBRACE at line 24 |
| $S/write-compare/t4a_take_rewrite.hako | dump-refused | ❌ Parse error in $S/write-compare/t4a_take_rewrite.hako: Unexpected token LBRACE, expected tuple constructor `Op::Lit(...)` at line 52 |


## 未確認・maintainerに決めてほしい物

- 案AかB、公開op/pretty/JSONの正確な名前、v2語彙拡張のversion方針、新CP ABI名とstatus0/2契約。
- typed Textの既存4 operand domainを初回から対象とする設計と、formal/loopの前段不足をどこまで同seriesへ含めるか。未証明receiverを軽いopへ通す案ではない。
- raw byte sourceからCPへの変更は明示的な意味の是正として受け入れ、VM/compat byte helperは別scopeに残す方針。
- prototypeはなし。新opのLLVM/EXE・実装LOC・whole-function削減・実行時間/RSSの改善は未測定。新ABIの安全性/性能を実装済みとは主張しない。
- raw2新probeのcompat provider、formal6件のpinned frame、loop family選択、printの前段拒否は修理していない。これらの未受理をNoFault証明で隠さない。
- docs-onlyなのでCargo/unit/full-lib/固定suite/Python v4の再実行は範囲外。既存binaryでの調査だけ。RULES §6とworker共通ルールに従う。

監査資産: L/start.json、population.json、selected.json、probes.json、measurements.json、discovery.json、capture-results.json、out/、captures/。MIR全数の古い証拠はnormalize-1b/dumps-final（同一source/binaryの正規化済み版）、この回のselected測定はLを正本とする。production/testの差分0、docsのcommitだけを着地する。

## ChatGPT Proへの相談とユーザー決定（2026-11-10）

Opusが基点`6ea4f9437e`で相談文を作った（7問: A/BかCか、AかB、runtimeの約束、O(n)とcache、独立検証の共通化、version、順番）。ChatGPT Proが答え、Proが引いた42か所はOpusが`6ea4f9437e`で読み、全て記述どおりだった。

### Proの答え（要点）

- **Aを今足す。** 前回の「軽い操作を先に通してからrawを退役」の理由のうち「動いている軽いEXEを守る」は、raw 13本が全てsource側で止まる（上の入力一覧）ので成り立たない。`call-not-birth`（`physical_program_json.rs:434`）は別の障壁で、13本の実測の停止ではない。Aを選ぶ理由は、意味が確定した操作（`core_method_contract_rows.rs:153`、CodePointCount・I64・PureRead）を共通の評価器から公開の形まで運ぶこと。Cも成立するが、「Faultが起きない演算は軽い形」の方針からの長い例外になる。
- Faultの枠は関数ごとに1つを共有する（`function_fault_frame.rs:1,30`）。lengthごとの費用はInvoke・NormalResult・Normal/Faultの辺・runtimeのframe検査。
- Aは機能の追加で、`RULES.md:98`により全機能commitに純減を要求しない。旧経路の整理は別に集計して純減で閉じる。
- **AとB:** A。JSONは`array_length`（`physical_program_json.rs:383`）と同じ平らな`{"op":"string_length","dst":D,"value":<Text>}`。印字も`string_length`で、frame・normal・faultを持たない。`StringInvokeOperand`（`invoke_string.rs:7`）の流用と全variantの受理は分け、StaticText・OwnedString・BorrowedText・TextOperandだけを受ける。次の軽いString読みの候補は1引数`indexOf`（CodePointSearchFromZero、`core_method_contract_rows.rs:178`）。`contains`と2引数`indexOf`はUnprojected、比較はTypeErrorのFaultを持つ（`fault_checked_string.rs:177`）ので一括しない。将来を見越した一要素のenumは今の重複を減らさない。
- **runtime:** `nyash.string.code_point_count_v1(value, out) -> status`。正しい生きたTextなら数を書いて0。検出できるdescriptor・lease・出力の違反は2で、`out`を変えない。1は返さない。frame・siteを受けず、Faultを記録せず、handle・tokenを取得・移転・消費しない。Cは0だけを次の行へ、それ以外を既存のinvalidの出口へ送る（`hako_llvmc_ffi_lifecycle_v4_emit.inc:720`、rootはframeをdisposeして70）。InvalidContractはartifact・ABI呼び出し・compilerとruntimeの不変条件の違反で、sourceのFaultではない。dangling pointerを2に変えられる約束にはしない。Stringの全文の複製（`fault_checked_string.rs:129`）をやめ、`with_end_authorized_text_pair`（`dynamic_v2_lease.rs:92`）等の借用のcallbackの中で数える。callbackの中でpublish・drop・再入をしない。
- **cache:** 公開の形にも最初の回にも入れない。後でruntimeの不変のbackingに、handleのgenerationを含むidentityに結びつけて持つ（`host_handles.rs:78,260`、`lease_identity.rs:108`）。StaticTextの`from_utf8`検査は別のO(n)。NoSourceFaultは、Endをまたぐ移動や投機実行の証拠ではない。
- **独立検証:** Textを読んでよいかの判定をInvokeと通常行で共通にし、状態遷移（InvokeのNormal・Faultごとの取得・移転・End）は分けたままにする。既存の`(block, faulted)`とLiveのsolver（`invoke_string_lifetime.rs:534`）を使い、length専用の台帳を作らない。検査を起動する選択条件（`:281`）、block内の通常行（`:550`）、Invoke側の読みの検査（`:610`）、BorrowedTextのformalの起源（`invoke_string.rs:104`）、一般のMIR検査（dst・使用・型・支配・visitor）を一緒に直す。C側も行ごとにTextの読み取り可能性を検査する（`hako_llvmc_ffi_lifecycle_v4_admission.inc:190`）。Cの名前は今blockとoperandのroleで作っている（`hako_llvmc_ffi_lifecycle_v4_string_emit.inc:21,85`）ので、関数・block・行で一意にし、slotはfunctionのprologueで確保する。PHIは既存のedge forwarding（`nyrt_c_abi_v0.md:358`）を使う。
- **version:** v2への追加。前例はCompareの通常行の追加（`nyrt_c_abi_v0.md:323–340`）。
- **順番:** 変えない。最初の回にcache、36組の全部、loop全般、借りた引数の全経路、raw 13本のEXE到達を入れない。`(s + t).length()`では連結のFaultと一時StringのEndが残る。既存testのFault注入（`published_native_array_runtime_probe.c:229`）はchecked lengthを包んでいるので、実際にFaultを持つ操作へ移す。
- Proが確かめていない物: 45本と13本の原ログの全件、行数の見積もり、frame・複製・cacheの実時間の効果、tokenizerの頻度、repositoryの外の旧artifactの扱い。

Opusの追加の確認: frameの要否の判定で、Core操作の例外は`checked_array_count_law_v1`だけ（`home_expression_evaluation.rs:47`）。Coreのprojectionの戻り値は`InvokeOperation`（`expression_source_values.rs:393`）で、Arrayのcountはwriter（`expression_value.rs:522`）とsourceの対応検査（`expression_source_values.rs:224`）で、projectionの前に別に扱われている。

### ユーザー決定（2026-11-10）

- 案Aを採る。非terminatorの`StringLength { dst, value }`、JSONの`string_length`、runtimeの`nyash.string.code_point_count_v1`（status 0/2）、lifecycleのprogram v2の語彙への追加。文法と意味（code pointの数、`docs/reference/language/strings.md:121`）は変えない。
- 旧checked lengthは同じseriesで消す（Proの案は、exportを薄い包みとして残し、旧artifactを切る時にreaderと一緒に退役させる、だった）。消す物は`StringInvokeOperation::Length`（`invoke_string.rs:76`）、そのJSON、Cの受理、`nyash.string.checked_length_v1`。生成する所を全部`StringLength`へ移して0にしてから消す。物理のJSONはcompilerとC shimの間の一時文書で、語彙が閉じているので、消した行を持つ古い文書は知らない行として拒否され、別の意味で読まれることはない。このためschemaの名前はv2のままにする。
- sourceのlengthからrawのbyteの選択（`src/mir/generic_method_route_plan/collection_read_routes.rs:456–485`、`match_generic_len_route`のStringBoxの部分）を退役させる。`len_fast_h`などVM・compatのbyteのhelperは、呼ぶ所が0なら消し、残る時は呼ぶ所を記録する。
