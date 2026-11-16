# MirBuilder整理1: 実行経路の計測と未使用コードの整理

Status: 段1完了、段2は確定できたfamilyを整理して終了。未判定候補・安全性検査は保持。Date: 2026-11-15。基点main `9e3f19026e`（productionは`7a6771e805`と同じ）。

## 段1: 変更前の実行経路

Rust 1.89/LLVM 20の`-C instrument-coverage`、quick opt1、独立targetで計測した。Rustup同梱LLVM toolを使用。
CLIと全lib testのprofileを分離し、CLIは群ごと・成功/拒否ごとに分離した。source変更なしで取得。
物理production分母はtest名file/外部test module/inline cfg(test)を除く近似。コメント・空行・未選択featureも含む。
LLVMの可測行とは別分母。未mappingを不要コードとみなさず、削除は別途参照・test・feature・安全責務で判定する。

| 範囲 | 行数 |
|---|---:|
| 物理production（静的近似） | 664,256 |
| CLI可測行 | 432,887 |
| CLI実行済み行 | 251,903 |
| 物理行の未観測合計（下記2群、不要という意味ではない） | 412,353 |
| CLI可測だが未実行 | 180,984 |
| CLI未mappingの物理行（未判定） | 231,369 |

### 大きいfamily

| family | 物理近似 | 可測 | CLI実行 | 可測未実行 | 未mapping |
|---|---:|---:|---:|---:|---:|
| builder/ | 206,891 | 134,858 | 64,226 | 70,632 | 72,033 |
| normal_callable_semantic_package/ | 150,761 | 111,979 | 93,284 | 18,695 | 38,782 |
| compiler/ | 66,876 | 40,681 | 16,644 | 24,037 | 26,195 |
| resolved_semantics/ | 56,404 | 36,422 | 29,004 | 7,418 | 19,982 |
| ./ | 39,624 | 24,038 | 9,224 | 14,814 | 15,586 |
| loop_recipe_contract/ | 24,942 | 16,366 | 10,823 | 5,543 | 8,576 |
| passes/ | 9,293 | 4,992 | 2,097 | 2,895 | 4,301 |
| global_call_route_plan/ | 7,823 | 5,404 | 2,358 | 3,046 | 2,419 |
| verification/ | 7,782 | 5,836 | 3,632 | 2,204 | 1,946 |
| control_tree/ | 6,749 | 3,514 | 1,036 | 2,478 | 3,235 |
| join_ir/ | 6,620 | 2,796 | 44 | 2,752 | 3,824 |
| resolved_value_profile/ | 6,368 | 4,119 | 1,672 | 2,447 | 2,249 |

### 入力群の差とtestだけで動く部分

| 集合 | 行数 |
|---|---:|
| いつもの検証のみ | 14,424 |
| selfhost compilerのみ | 4,898 |
| apps全入口のみ | 13,599 |
| 複数CLI群で共通 | 218,982 |
| 成功CLIでも通った | 226,239 |
| 拒否/失敗CLIだけで通った | 25,664 |
| CLI可測内でtestだけが実行 | 112,194 |
| test側のみmappingがあり実行（同じ物理近似内） | 5,029 |

成功はsource＋CLI modeごとのexit0（EXE modeは生成物存在も必要）。negative precheckの期待拒否は成功側へ入れない。
拒否で通った行も「必要」の証明ではない。dumpの成功も正規EXEやselfhost実行の成功を意味しない。

| 入力群 | 入力数 | CLI成功 | timeout |
|---|---:|---:|---:|
| dumps1866 | 1866 | 1139 | 0 |
| exe927 | 927 | 616 | 0 |
| pairs229 | 229 | 74 | 0 |
| apps100 | 100 | 100 | 0 |
| pairs384 | 384 | 157 | 0 |
| b5pairs392 | 392 | 303 | 0 |
| b5extras58 | 58 | 18 | 0 |
| opus363 | 363 | 236 | 0 |
| selfhost488 | 488 | 176 | 1 |
| apps1307 | 1307 | 162 | 0 |

上記6,114回に、EXE群の補足dump3,760回を追加。sourceは群間重複を除いて5,386本。総CLIは補足dumpとsuite内38回を含め9,912回。
固定suiteは8/8（空marker1件、実EXE smoke7件）、timeout/SKIPなし。全libは8,913 PASS/225 FAIL/56 ignored（4thread）。
222基準との差3件は全て単独再実行PASS。並列時の揺れと分離し、基準を変更していない。
selfhostの1本は180秒timeoutでprofile未保存。未観測部分を未実行と断定しない。

### 関数と未実行file

CLI mapping上のproduction source extentは44,177個、未呼出しは18,973個。
未呼出しextentに属する可測行のfile内union合計は134,152行。
LLVMの同一source extentをまとめた数で、macro/derive/generic instantiationを含む。全Rust宣言の厳密な個数ではない。

| CLI観測なしのfile（大きい順の標本） | 物理production近似 | 可測行 |
|---|---:|---:|
| compiler/nested_predicate_producer.rs | 782 | 663 |
| array_text_observer_region_contract/matcher.rs | 777 | 628 |
| function/metadata.rs | 747 | 0 |
| compiler/raw_root_decl_access.rs | 744 | 404 |
| builder/control_flow/plan/expression_port.rs | 730 | 425 |
| extern_call_route_plan/route_spec.rs | 723 | 42 |
| builder/raw_expansion_receipt_ledger.rs | 712 | 431 |
| compiler/normal_source_plan/module_source.rs | 698 | 449 |
| generic_method_route_plan/mir_json_routes.rs | 697 | 617 |
| callable_result_representation/located_legacy.rs | 696 | 453 |

完全なdirectory/prefix/file/function表とprofile・入力別成否はrepo外の計測成果物に保存した。
静的test除外には非標準cfgや明示path moduleの解決限界があり、物理分母は近似のままとする。
計測なしを理由に安全性検査・Fault cleanup・test専用品・feature専用品を削除しない。

## 段2: 確定できた未使用コードの整理

同じrustfmt条件で旧新を数え、MIR層はproduction −306行・test −17行（54file）。
runner/runtimeの保存field整理 −13行（3file）を含む全体はproduction −319行・test −17行。
未加工のsource差分は+56/−369（純減313行）。移動・改名・改行圧縮による削減は含まない。
production関数/method13、未使用test helper1、variant17、field25、import40項目を削除。
型全体/file削除0、新規Rust/test/checker/guard file0。test関数とassertionは削除していない。
削除候補はdefault/test/featureの実consumerまで確認し、Fault/所有権/借用/独立検証は保持した。

| 警告 | CLI前 | CLI後 | test前 | test後 |
|---|---:|---:|---:|---:|
| dead_code | 2,956 | 2,933 | 686 | 661 |
| unused_imports | 119 | 97 | 39 | 15 |

共通dead primary spanは681→630。残435fieldのうち421はDebugの実使用が未証明、2は所有した観測値の寿命が未証明、12は具体的な保持理由を確認した。
残160variantと35その他は拒否・検証・feature等の保持理由を記録。共通unused_importsは0。
本体のみwarningの4,594 spanを全てtest使用とは断定せず、今後の個別確認対象に残す。
workspaceのallow(dead_code)48属性は理由あり9/なし39。削除条件を確定できず保持し、allow追加も行わなかった。
これで未使用コード全体を整理し尽くしたとは主張しない。6時間の範囲で確定したfamilyを閉じた。

### 最終確認

source `b71ff229ff` の最適化CLIとtest binaryで確認した。sourceは最終検証後に変更していない。
全libは8,916 PASS/222 FAIL/56 ignored、失敗名は基準と一致。並列だけの追加2件は単独PASS。
通常群の3,691 dump・2,344 EXE入力で、記録した正規化後の受理・MIR・実行出力・exit差分0。ignored Map18ケースもPASS。
既存の診断表示にはHashMapから選ぶwitnessと一時artifact名の揺れがある。
同じMIRの全対象行集合・件数を検査して比較し、生の診断bytes一致とは区別した。
固定suite8/8（空marker1件、実EXE smoke7件）、owner/pointer/diff guard PASS。
feature2構成はPASS、既存失敗2構成は同じエラー集合を維持した。
CLI sizeは89,942,184→89,897,208 bytes。build条件のcache状態が違うため速度改善率は出さない。

### 次回の確認対象

CLI未観測fileを削除候補へ直結させない。大きい候補はnested_predicate_producer（782行）、
array_text_observer_region_contract/matcher（777行）、raw_root_decl_access（744行）などだが、
testで使われている。責務の後継を確認してからtest専用経路を退役する判断が必要。
本体buildで未使用と報告された安全性責務17件も保持し、本線の未接続理由と検査範囲を次回確認する。testから使うものも含む。
詳細な一覧・コマンド・binary SHA・ログは日本語の整理1報告に集約した。

## Opus の確認と統合(2026-11-15)

- Codex 実装、Opus 監査。`cleanup-1` を `--no-ff` で統合した。
- ビルド: 作業 tree で cargo を空で実行し、binary の SHA が報告と一致した(`2b836f67…`)。
- 入力: Opus の 363 本と、機能ごとの小さい入力 30 本を、前の binary と比べた。結果も、成功の MIR も、全部同じ。
- 行数: 整形後の production −336 行(Codex の数え方では −319 行)、test −17 行。
- lib の失敗 222 名は、前と同じ集合。`git diff --check`、private の走査は PASS。stash は 2 件のまま。
- 計測(実行された行の数)は Codex が測った。Opus は測り直していない。

### 測った結果の読み方(Opus。決定ではない)

LLVM が測れる行 432,887 のうち:

| 分け方 | 行 | 割合 |
| --- | ---: | ---: |
| CLI(今ある入力の全部、`lang/src/compiler` の 488 file、apps の入口 1,307 本)で実行された | 251,903 | 58% |
| CLI では通らず、lib の test だけが実行した | 112,194 | 26% |
| CLI でも test でも実行されなかった | 68,790 | 16% |

- CLI で実行された行のうち、拒否・失敗で終わった入力だけが通った行は 25,664。
- 本体の build で「使われていない」と報告される物 5,278 のうち、test の build でも使われていない物は 681。**残りの約 4,600 は、test だけが使っている。**
- つまり、builder の約 4 割は、今ある入力のどれも通らない。その 3 分の 2 は、test が支えている code である。
- 次に決める事(ユーザー): test だけが使う code を、test と一緒に消すか。どれを残すか。

## 整理2: 削除前の一覧（2026-11-15）

本回は文書だけで、本体/test変更0。main `ad9091898f` と整理1最終codeは同じ。
調査済み162塊・所有file957。CLI0の追加358fileを全て対応させた。現役file内部の未実行関数を全件読み切った意味ではない。

|区分|塊|整形後production専有|支えるtest行|
|---|---:|---:|---:|
|ア|1|268|118|
|イ|128|128393|51200|
|ウ|21|18862|15640|
|エ|12|7577|3335|

アは旧generic_loop_canon/condition 3leafのみ、268行＋親の型/宣言8行、test118行/5関数の退役見込み。実際の削除は0行。
入口0・CLI0・feature未使用・exact R1指定・旧caller削除6e88441c0bを確認。後発keepは配置先分類であり新callerではなかった。
guard3本の旧path参照とreceipt/censusを同じseriesで更新し、現役loop_condition_shape/UpdateCanon/step/updateの検証は残す必要がある。

安全性責務35項目を別表へ。範囲限定で現役検査・後継を確認18、潜在1、未確定16。確定した現役の穴の反例0は、全体安全性の保証ではない。
元17検査はいずれもtestで実行されている。cleanup/portable PHIの後継完全同値は未証明なのでUNKNOWNへ保守化した。

|selfhostの最初の壁（上位）|旧dump拒否file数|
|---|---:|
|CoreMethodSource/NamedArray(TextSourceMissing)|67|
|OrdinaryNew/BatchLoan|65|
|ordinary-new/borrowed-static/forward-source-missing|38|
|LoopBreakSource/UnsupportedAncestor|24|
|ordinary-new/borrowed-static/text-expression-execution-pending|9|

旧dumpは488中176成功/311拒否/1timeout。現在のEXE入口15件は生成1/拒否14、非入口473は測れない。
生成1はschema Mainのreturn0で、compiler.hako/compiler_stagebは拒否。selfhost達成とは数えない。
所有の印はShared専用7Rust 1112整形行＋1C include70物理行と、move/take混在owner372整形行を分離。削減候補へ加えない。
コードを消さず、ユーザーの判断用の全一覧・根拠・未確認範囲を日本語の整理2報告に保存した。

## Round 3: 未接続試作の退役と、次のfamily判断（2026-11-15）

基点 `1db505eacb`。段1・2は受理を変えない退役、段3は判断用一覧のみ。sourceはarchiveや無効featureへ移さずGit履歴に残す。Rust production 8,640行・test 13,570行・78 filesを削除。登録testは9194→8993（純減201）、旧定義206件の除去には保持保証の移設・置換も含む。

### 段1・2で消した物と、再検討の条件

PH47: 旧AST条件抽出canonを退役。外部Rust caller 0を再確認し、現役loop_condition_shape、UpdateCanon、step/updateを保持。再検討は共通解析で扱えない具体的sourceが出た時。`ae0bb57633`。
EX09: Raw callable source cursor。現役source解決から接続されず、source navigationを二重に保つ理由がない。
再検討: 通常source解決で表現できない具体的要求が生じた時。参照commit `b2de75856b`。
SC07: ny_main status adapter。EXE入口へつながらない将来adapterで、次selfhost壁と実callerを示せない。
再検討: 選択済み入口とstatus契約が具体化した時。参照commit `eb9b6e0aae`。
SC41: whole-module preflight。現役transactionとは別の未接続factoryで、次の製品結果を作らない。
再検討: 実callerと共通admissionで足りない契約が示された時。参照commit `e3c49c111a`。
EX04: SourceResult分類器。未接続の旧catalog/row分類を退役。現役expression-planの4testは既存test moduleへ保持。
再検討: 現役共通式表現で扱えない結果契約が具体化した時。参照commit `f291d61f62、9ae6d5b748`。
EX44: bounded body-effect coverage receipt。未接続のCall/Return専用witnessで、現役effect検査のauthorityではない。
再検討: 実consumerと独立な責務が示された時。参照commit `ee7c7f44ee`。
PH08: Generic source-lease模型。test内の段別witnessを現役解析へ接続する予定がない。実parameter型検証は保持。
再検討: selfhost壁・実consumer・旧道削除まで一文で示せる時。参照commit `98e0857711`。
PH27: 旧portable Loop PHI materializer。使い手の無い旧materializerと、同じ旧模型を試す子test/emitterを退役。現役SSA/PhiTxn/Recipe検証は保持。
再検討: 共通SSAで不足する具体的なloop要件が出た時。参照commit `e4464ed5c4、8e87060978`。
PH64: invocation route matrix。未接続のroute組合せ模型を退役し、現役family選択とvalidatorを保持。
再検討: 実callerと共通familyでは足りない意味が明示された時。参照commit `43a66024c8`。
EX06: Query body/conformance試作。未接続Queryチェーンを退役。DeclaredInstanceMethodIdentityV1は実effect/relationが使うので保持、Home ABI保証も保持。
再検討: 実Query構文と共通解決からの接続が選択された時。参照commit `add38206f2`。
EX31: Dynamic envelope catalog。未接続catalogと専用Refのみ退役。現役SourceLoan、effect/Home/lifecycle契約と二重発行拒否testは保持。
再検討: 現役envelopeでは不足する利用者契約が示された時。参照commit `099421880a`。
PH38: private Dynamic Loop canary。通常sourceから呼ばれない移行専用sessionを退役。現役physicalizer/SSAの失敗時保証は保持。
再検討: 同じ仕事を共通loopが扱えない具体的sourceが出た時。参照commit `ba6d62f51a`。
SC20: 旧result activation/claim-ledger/Located port。未接続のbatchと旧Located/Claimed形式を退役。raw/current port、SourcePublicationのtake-once/residual保証を保持。
再検討: 現役結果契約に不足する本番source要求が示された時。参照commit `82c0f8e0f1`。
SC25: typed acyclic module plan。未接続typed planだけ退役。現役NormalSourcePlanのgraph/SCCを保持し、publication/drain/foreign-brand保証を既存recursive fixtureへ残す。
再検討: 共通graphから別typed planを要する実callerが示された時。参照commit `e87fa48b82`。

専用helper・getter・reexportの閉包は `0022bca221`。guardは旧file/型/模型の肯定句・専用allowlistだけを同時に整理し、現役の禁止句・共有skip条件・独立検証の検査を保持した。混在guardを丸ごと無効化していない。観測記録の限定更新は `0e262d354f`。

### 残した未接続物と、実際には現役だった物

SC33 / PH03 / PH57 / EX01 / EX02 はS6C/common-V2の明示parkを保持。根拠は `docs/development/current/main/investigations/loop-production-selection-d0-2026-09-24.md:84`、`docs/development/current/main/design/joinir-loop-selfhost-recipe-pipeline-ssot.md:1684-1699`、`docs/development/current/main/investigations/s4-loop-survey-2026-11-08.md:86`。選択するproduction consumerとexclusive旧edge削除集合を決め、ユーザーが再開・破棄を判断するまで触らない。
PH65はA4のraw collection/foreign-family模型を保持。根拠は `docs/development/current/main/investigations/app-boxtorrent-mini.md:88`。所有契約の後継と検査を決め、ユーザーが保留を解除するまで削除しない。
EX03は誤分類を訂正して保持。`function_draft_entry.rs:319` が現役portable If mapperを呼び、selfhostの条件分岐を扱う。EX38も保持。`direct_accum_profile.rs:144` がeffect planを発行し、`direct_accum_lowerer.rs:90-94,134` のowner/frame照合へ渡す。将来の予定を理由に残したのではない。

### 段3: 次にユーザーが判断する25 family

行数は整形後の調査範囲上限で、削除保証ではない。混在file・共有部品・安全testは差し引く。今回は入口を閉じず、必要な言語機能も廃止しない。順位は入口の閉じやすさと依存の少なさを優先。

|family / 利用者から見た機能|production / test 行|入口・影響形の例|selfhost要件 / 共通道|案|
|---|---:|---|---|---|
|F01 旧MIR→JoinIR oracle API|2,888 / 657|LoopToJoinLowerer API（通常loopは対象外）|loopは要る / GenericG0|退役する|
|F07 文字列3連結の任意最適化|389 / 2|`a + b + c`|連結は要る / 通常String|退役する|
|F06 純粋scalarの任意Core13道|247 / 0|`return 1 + 2`|計算は要る / 通常scalar|退役する|
|F08 VM escape/barrier最適化|118 / 356|Box生成後のVM escape/barrier処理|最適化は任意 / VM保持|退役する|
|F02 StepTree/ANF debug要求|2,131 / 1,177|debug/ANF shadowの明示要求|debug機能 / 本線保持|退役する|
|F04 reference backendの明示要求3種|3,285 / 1,509|`--backend raw-vm-reference` 等3種|reference実行 / 共有VM保持|退役する|
|F16 旧fn/lambda式|814 / 268|`fn(x) { ... }`|旧構文未観測 / 仕様判断|退役する|
|F17 旧try/catch/throw|280 / 327|`try { ... } catch ...`|旧surface未観測 / Fault保持|退役する|
|F10 fastmem/layout宣言|3,552 / 1,829|fastmem宣言内のload/store|compiler488内0、別allocatorに有 / 配置契約|分からない|
|F11 Array<Text>観測・編集session|5,523 / 165|`arr.get(i)` を含むText走査loop|Array/Textは要る / collection共通道|退役する|
|F15 静的u16表の添字load|74 / 69|`DATA[i]`（静的u16表）|compiler488内0 / 添字共通道|分からない|
|F05 ProgramJSON v0 loop import|1,894 / 364|旧ProgramJSONのloop node|旧import API / 公開MIR保持|分からない|
|F09 旧RC自動挿入pass|415 / 0|旧RC挿入最適化要求|明示所有/Sharedを保持|分からない|
|F18 旧peek式|123 / 0|旧peek expression|compiler488内0 / canonical match保持|分からない|
|F20 Resultの?伝播|91 / 0|`call()?`|字句?は型・三項と区別 / Result要件未確定|分からない|
|F21 script-rootから静的call|1,496 / 1,325|script rootの `Box.method()`|静的callは要る / App静的call保持|分からない|
|F03 旧loop AST助言解析|3,651 / 3,118|`loop(i < n) { ... }`|loopは要る / GenericG0|分からない|
|F24 mainのcontinue/step loop|5,801 / 2,046|`loop(...) { continue }`|continueは要る / 共通loop|分からない|
|F25 static BoxのText走査root|7,640 / 1,322|static box method内のText走査|Text走査は要る / instance/main共通道|分からない|
|F22 callable内のloop|3,675 / 2,092|method内の累積loop|要る / callable共通loop|残す|
|F23 入れ子loop|4,048 / 1,142|loop本体内のloop|要る / 共通nested loop|残す|
|F19 Enum/Option/Result|716 / 211|enum variant / Option / Result|字句使用有 / enum共通道|残す|
|F14 extern呼び出し|1,296 / 907|宣言済みextern call|2compiler fileで使用 / 共通extern|残す|
|F13 Array/Map/String基本method|6,512 / 4,955|`arr.get(i)` / `map.get(k)` / `s.length()`|要る / sole method-route owner|残す|
|F26 条件分岐Recipe|2,123 / 378|`if cond { ... } else { ... }|要る / 共通If owner|残す|

25 familyは341 files、production 58,782 / test 24,219行（626 test関数）。残family 0。family化しなかった共有範囲は458 files、73,709 / 27,869行、主流ncsp/resolved内の別勘定21 filesは2,323 / 1,541行。既退役分は将来削減へ加算しない。
旧F12はoptional Hako shadowではなく実際のdecisionを返すと判明し、F13へ統合。通常sourceの成功は旧family選択の証拠ではない。fastmemの未呼出helperを含むEXE成功はMemOp実行を意味しない。reference3種は実際のselectorを別々に測り、全て名前付きfeature-unavailable / exit2（feature有効buildは未測定）。
sourceの字句探索は到達証明ではない。selfhostが使うloop・continue・Array/Map/String・条件分岐・externは要件として残し、旧専用道を閉じる前に共通道の受け入れへ接続する。ncsp/resolvedの未観測枝はfamily化せず、旧LLVM観測と現在sourceの行数を混同しない。

### 段4: 残す入力・契約と、前から赤いtest

固定入力は10群、重複をまとめた5,386入力のSHAを削除前に固定し、MIRは全5,386本、EXE比較は3,551本。群別所属は dumps1866 / exe927 / pairs229 / apps100 / pairs384 / b5pairs392 / b5extras58 / opus363 / selfhost488 / apps1307（所属が重なるため合算しない）。成功だけでなく拒否理由も固定し、途中で入力を減らさない。
保持する契約: ①通常sourceから唯一の生成ownerへ進む入口と名前付き拒否、②実MIRの型/SSA/PHI/支配・借用・lifecycle独立検証、③全Normal/Fault経路の取得・移転・Endの一回性と順序、④from宣言・支持元・receiver・借用区間、⑤公開MIR/JSON/C ABIの形・意味・strict schema、⑥実strong所有が無くなった後のWeak不在。
基点のlib赤222件は、旧内部配置期待110 / 今回退役実装だけ8 / 現契約の本物の不具合1 / UNKNOWN103 / flaky0。UNKNOWNを安全とも削除可とも扱わない。本物の不具合はTLS fresh-object cacheがArcを保持してWeakが生き残る例で、この掃除では直さず再現testを保持。

### 最終検証

source `0e262d354f`。最適化CLI build済み。featureは元から通る2構成PASS、元から落ちる2構成は同じerror集合。全lib直列は214 FAILで、基点222から今回退役8件だけを除いた名前集合と一致。ignored Map18件PASS。
全5,386入力のMIR・拒否理由、およびEXE対象3,551本の生成/実行出力/exitは固定基準と差分0。生成成功1,471本、timeout 0。固定suiteは8行中7実EXEと1空marker、SKIP/timeoutなし。見張りはraw BinOp上限70→69・definition scan 294→292へ下降し基点1dbでPASS（3b36299375）、pointer/diffもPASS。Rust/C/言語入力7272件のhashは最終検証後も不変。
既知の観測不整合は未修正: metadata field数130/131と旧anchor群、provider観測6/7のPython 2 FAIL。制御フローcensusは既存generatorで1865→1959へ再観測し整合PASS（以前からのdriftも含み、削減量に加算しない）。
比較はcompiler/artifact/PIDの出所表示と、事前固定したsource0 witnessの完全MIR行集合照合だけを正規化。raw表示差1件を隠さず記録し、比較規則を追加して合わせていない。

### Opus の確認と統合(Round 3、2026-11-15)

- branch `cleanup-3`(21 commit)を main に統合した。整形後の行数は、production と test を合わせて −22,210(file 名で分けると production −12,914 / test −9,296。file の中の test を分けると production −8,640 / test −13,570)。
- 独立の確認: 最終の binary は、報告の SHA-256 と一致。build した commit の後の変更は、文書と見張りの上限だけ。Opus の 393 入力(363 本 + 機能の 30 本)で、MIR・受理と拒否の理由・出力・exit が、前の基準と全部同じ。
- lib の赤は 214 名で、基準の 222 名から、この回で消した 8 名を除いた集合と一致(worker の比較の記録を読んだ)。hygiene の走査、`git diff --check`、stash 2 件は PASS。
- 見張りの変更は、上限を下げる向きと、消した旧 path の節を外す物だけ。
- 次: 上の 25 family について、メンテナが「退役する・残す」を決める。保留の 6 塊(S6C の一族と A4 の境界)も、メンテナの決定を待つ。

## お掃除4: 選択と生成の実測、最初の統合案（2026-11-16）

この回は文書だけ。production/testの変更・削除は0。基点`cc553601d4`、default build・pure-first・compat replayなし。固定5,386入力path（内容SHAは5,210種類）を減らさず、MIR全件とEXE対象3,551件を計測した。事前固定した比較規則で、計測の印によるMIR・拒否理由・出力・exitの差は0、EXE生成1,471件、timeout 0。入力群の所属には重複がある。

|専用family|選択イベント / 入力数|MIR生成完了イベント|strict公開EXE|専用道だけで成功する入力|
|---|---:|---:|---:|---:|
|F22 callable single loop|0 / 0|0|0|0|
|F23 nested predicate（whole-functionと共通winner両方）|0 / 0|0|0|0|
|F24 main0の3profile|24 / 12|24|0|0|
|F25 selected Dynamic static callable|0 / 0|0|0|0|
|F19 raw enum constructor/match|0 / 0|0|0|0|
|F26 If Recipe|0 / 0|0|0|0|

選択・生成イベントはdump/EXEの別実行を含む。F26のpreflight選択とadmission成功も0。候補関数の呼出しは選択に数えない。F24の12入力は全て元から`IncompleteOrdinaryNewCoverage`で失敗していた。旧F24を選択前に外すscratch実験では、同じ12入力が既存SourceLoop V2からEXEまで進み、strict公開に成功。通常exitは10件、残り2件は返値i64::MAXに対するEXE入口の範囲拒否（Fault reason 102）で、整数演算overflowとは別である。全固定入力でのF22/F24 bypass差はこの12件だけで、以前成功した物の後退0。新受理は次の変更で明示して審査する。

共通側はSourceLoop V2の選択435/局所生成完了425、IfForm物理入口10,153/最初の命令append観測10,118。前者はloopの生成、後者は短絡条件の内部CFGも含む物理処理で、sourceのIf個数と同一ではない。公開完了はmoduleの結果と分け、physical function名が分かる行だけ最終moduleの保持名とも照合する。F23の共通winner→専用Recipe→共有physicalizerは構造として存在するが、今回の入力では未選択。F24→共有segment生成を、共通V2と旧道が同じloopを二度生成した事とは数えない。
未選択はこの入力/profileでの事実であり、全featureで不要・機能が全て共通へ移ったとの主張ではない。特にF19はresolver手前の拒否を残し、共通VariantMakeの存在だけでenum matchの統合済みとはしない。

### 次の最小単位（提案、まだ実装しない）

**F24のin-body-stepを、既存SourceLoop V2へ切り替えてから専用道を消す。** 対象はInteger carrierの反復と、元の順で行う再代入・書くだけlocal。前後の文数、変数名、main/static/instance、関数内のloop数を新しい受理条件にしない。既存V2の表現を広げる必要は実験では見つからず、新しいproducer/Recipe/Loop基盤は作らない。
- 広げ先は既存root選択→source Loop入口。新規productionは0〜80行の見込み（共有source identity/loan検査の保持が必要な場合のみ）。新しいLoop計画の追加は0。単独in-body-step入力も共通V2でEXE exit 3を実測した。
- 専用候補11filesは整形後production 2,324/test 845行、test関数17。これは保証の移管前の候補上限で、全量削除の約束ではない。root route/reexportの専用glueも同seriesで退役し、兄弟profileが使う共有型は残す。
- 合格は元の固定入力を保ち、新受理差を区別し、出力/exit・評価と更新の順・Normal/Fault・sourceとPHI/辺・独立MIR/ABI検証を守る事。切替範囲では旧selector/producer/lowererの実生成0、共通V2の生成と公開を確認し、不要な呼び手/本体/専用testまで消す。
- 元owner/siteの拒否、BodyOnly/write-onlyの元source順、Header/break/backedgeとHome、非predecessor PHI拒否は既存のsource-route/handoff/initializer/ownership_ssa/typed_schema_v2のtestへ保証を集約する。旧内部配置だけの期待を写さない。

### selfhostの不足は別に特定する

488入力はdump成功176/失敗312。主なfirst-stopはNamedArray 67、BatchLoan 66、static forward-source-missing 38、LoopBreak UnsupportedAncestor 24。67を全て「文字列の出所だけの問題」、66を単一原因とは扱わない。
LoopBreakの24件はすべて`ProgramBody(1)`のportable path欠落だった。resolverの親子forestは既にあり、足りないのは`LoopSourcePathStepV1`と既存adapter/source-binding verifierのbare lexical block表現。最初の拒否を閉じる見込みはproduction +6〜20行、即時削減の確認量0、後段EXEは未測定。Serde source DTOの追加判断を別単位で行い、ScopeBodyへの偽装や新しいnested lowererで埋めない。Loop全体の作り直しをselfhostより上位の目的にしない。

### 保留6塊の照合（今回は全て保持）

|塊|整形後production / test|最初のV2統合への案|
|---|---:|---|
|SC33|462 / 108|採らない。唯一loopのadmissionを持ち込まない|
|PH03|4,253 / 3,495|採らない。既存V2/SSA sessionへもう一本を足さない|
|PH57|2,261 / 901|採らない。S6C専用corridorを一般反復へ広げない|
|EX01|2,496 / 667|採らない。特定Text走査のsource/Recipeを移植しない|
|EX02|980 / 57|採らない。専用物理representationの切替を伴わせない|
|PH65|211 / 310|採らない。旧ownership境界は別判断（専用test child246行も計数）|

再開は、既存V2に無い具体的な意味・現役の接続先・消す重複を示せる時だけ。この案は削除承認ではなく、実装はユーザーの判断まで保持する。

### 退役候補8族の閉鎖実験

8族の入口をscratchで閉じ、全5,386入力を再計測。F01/F02/F04/F06/F07/F08/F17は変化0。F16は2入力が変わり、うち1入力は従来MIRが出ていた（EXEは`closure-body-wire-unavailable`で未生成）。もう1入力は元から別の拒否で、拒否段が変わる。EXE成功の喪失は0だが、F16を「影響0」としない。
F16を閉じる時はEBNFの将来`fn[captures](...):T`を残し、旧`fn(x){...}`と分ける（EBNF:125、ownershipのcapture節）。拒否案は`LegacyLambdaSurfaceRetired`。F17は既に仕様で禁止されるcatchable try/catch/throwを閉じる案（quick-reference:44〜53、EBNF:21〜29/1088〜）。拒否案は`LegacyCatchableExceptionSurfaceRetired`、既存throw予約語拒否は保持。合法なcleanupだけのTryCatch carrier、Fault/End/fini/Result/?、現役callableの共有表現を一緒に消さない。
全て固定した通常profileでの結果であり、reference feature有効buildや任意passを強制した範囲の証明ではない。計測用の遮断exit 86を、製品の名前付き拒否の実装とは扱わない。

### Opus の確認と統合(Round 4、2026-11-16)

- branch `cleanup-4` は文書だけ(1 commit)。main に統合した。
- 読み方(Opus。決定ではない): 固定した 5,386 入力・既定の feature では、F22・F23・F25・F19・F26 は 1 度も選ばれていない。「機能は要るが、古い道にある」と見た 6 family のうち 5 つは、今の入力については、共通の道(SourceLoop V2、共通の If)だけで通っている。統合の決まりの 1 番(機能がもう共通の道だけで通るなら、後継を作らずに古い道を閉じて消す)の相手になる。既定でない feature と、Rust から直接呼ぶ API は、測っていない。
- F24 は 12 入力で選ばれ、12 本とも前から失敗。選ぶ前に外すと、12 本とも今ある V2 で EXE まで進む(新しく受け付ける差。メンテナの承認が要る)。
- 退役の候補 8 個のうち、7 個は閉じても変わる入力が 0。F16 だけ 2 入力が変わる(EXE の成功は減らない)。
- 次に決める事: 退役する family、F24 の切り替え、保留の 6 塊(本体 10,663 行、test 5,538 行)。
- **メンテナの決定(2026-11-16)**: (1) F01・F02・F04・F06・F07・F08・F16・F17 を退役する(古い `fn(x) {...}` の式は提供しない。変わる 2 入力は承認した差)。(2) F22・F23・F25・F19・F26 は、入口を閉じて消す(後継は作らない)。(3) F24 は、今ある共通の loop へ切り替えて、専用の道を消す(12 入力が新しく通る事は、承認した差)。(4) 保留の試作 6 塊は、再開の条件を文書に残して、実装の保存を終える。

## 捨てる掃除の決まり(ユーザー決定 2026-11-15)

方針: **今の主流の道に対応している test だけを残す。つながっているだけで、誰も通らない物は、消す。** 整理 2 が 276 行しか「消せる」と言えなかったのは、「どんな入力からも使われない」と証明する仕事と、「この機能は今後提供しない」と決めて入口ごと消す仕事を、同じ審査にかけていたため。下の決まりで、2 つを分ける。

1. **削除は 2 種類。** 「動きを変えない削除」(残す対象から使われていない実装と test を消す。残す対象の結果・MIR・拒否の境目が変わらない事を確かめる)と、「範囲を狭める退役」(ある道・機能を今後提供しないと決める。退役した物は決めた境目で拒否され、それ以外が変わらない事を確かめる)。**後者を、前者の条件(全部の入力で結果が同じ)で審査しない。**
2. **残す物は、測った結果からは決めない。** 残すのは、(a) 今後使う入口から、意味の決定・MIR の生成・公開・EXE へ進む処理、(b) その道の安全の条件と拒否の境目(独立の検証、Fault の片付け、型と所有の拒否。めったに通らない枝と、その枝から呼ばれる helper を含む)、(c) そこからまだ使われる共通の部品(古い場所にあっても残す)。directory の名前では決めない。計測は、候補を探す道具と、差を調べる道具に使う。
3. **退役は family の単位で、メンテナが決める。** 一覧に書く物: 何の機能か、入口をどこで閉じるか、閉じると拒否になる形の小さい例、その形が selfhost に要るか。**要る形は、要件として残す**(その機能を古い道で持ち続ける事をやめるのであって、機能そのものを諦めるのではない。作り直す時は、共通の道へ作る)。
4. **決まった family は、入口を切り替えるか、名前付きの拒否へ変えて、本体から呼ぶ所を 0 にした上で、本体・専用の test・専用の fixture・`_for_test`・その道だけの guard を、同じ series で消す。** 拒否は、その契約を元から決めている持ち主の所に置く(名前や file 名で弾かない。常に error を返す stub を残さない)。
5. **「test から呼ばれる」「後継がまだ無い」「将来使うかもしれない」だけでは、削除を止めない。** 止める時は、影響する「残す契約」と、具体的な依存先を示す。分からない所は、その部分だけを残し、関係の無い family を止めない。
6. **test は、何を保証しているかで残す。** 残す: 残す source の意味・出力・片付けの順、今の MIR の型・所有・PHI・Normal と Fault の不変条件、退役した道へ入らずに決めた拒否になる事(代表だけ)。消す: 古い producer・古い recipe・古い内部の型の動きだけを試す物。今の保証と古い内部の期待が混ざる test は、今の保証だけを、今ある現役の test へ移す。「主流を何行通ったか」では決めない。
7. **安全の検査は、今の契約に残る物を守る。** 廃止した形式と入口だけにある安全の検査は、形式と入口の廃止と一緒に消してよい。今受け付ける MIR の不正(2 回の解放、PHI の不整合、不正な借り)を見つける検査と test は、普通の source から出ない不正でも残す。
8. **前から赤い test は、「同じ名前が赤い」で残さない。** 退役する実装だけを試す物は消す。古い内部の配置を期待する物は、今の契約に要るなら直し、要らなければ消す。今の契約の本物の不具合は、再現を残して直す対象にする。触る範囲では、名前に加えて、失敗の段と理由を確かめる。基準の集合は、「前の集合から、承認して消した test を除いた物」に更新する。
9. **順は、入口を閉じる → 専用の道を消す → 余った下の部品を消す。** 1 回の大きさは、行数ではなく、独立して戻せる単位。消して不要になった専用の依存は、続けて消す(`cfg`・別の directory・stub へ移して残さない)。**「消した後に、残った入力だけで測り直して、さらに削る」は、しない。** 残す入力と契約は、先に固定する。減らす時は、それ自体を退役の判断として扱う。 固定した入力と契約での測り直しと、統合で不要になった依存の削除は、してよい(2026-11-16 に明記)。禁じているのは、削除の結果に合わせて残す対象を縮め、それを次の削除の根拠にする事。
10. **やってはいけない事:** test を通すために、拒否や検証を弱める事(独立の検証を通さない、古い道へ retry する、panic を全部「未対応」として扱う、期待する診断を、出た別の error へ差し替える)。提供する機能は減らしてよい。提供すると言った機能の保証は、薄めない。
11. **退役と、構造の磨き込みは、commit を分ける。** この掃除では、型の統合や責務の置き直しを、一緒にしない。
12. **試作((ウ))は、「次の selfhost の壁を越えるために、どの現役の使い手へつなぎ、何が通るようになり、代わりに何を退役させるか」を一言で言えなければ、実装を消す。** 目的、採らなかった理由、もう 1 度考える条件、参照する commit を、この文書に短く残す。

## 統合してから消す決まり(ユーザー決定 2026-11-16)

方針: **要る古い経路は、新しい経路に統合してから、削除する。要らない古い経路は、そのまま消す。最後に、きれいな 1 本の道にする。** 「1 本」は、1 つの大きい関数ではなく、「同じ意味を決める責任が 1 つで、その計画を下ろす共通の仕組みが 1 つ」という意味。25 family のうち、機能は要るが専用の道か古い道にある物(F22・F23・F24・F25・F19・F26)が、この決まりの相手。

1. **「要る機能」と「要る古い実装」を分ける。** 先に、何が足りないかを確かめる。機能がもう共通の道だけで通るなら、後継を作らずに古い道を閉じて消す。一部の入力だけが古い道を要るなら、その不足を共通の道へ足して消す。共通の道にも仕組みが無いなら、今ある意味と生成の持ち主を広げる。
2. **共通の道を、もう 1 本作らない。** 今ある共通の処理を選んで広げる。共通の入口の下に、形ごとの古い道を残して、それぞれが同じ source の意味を決め続ける形は、統合ではない。
3. **切り替えは、小さい範囲ごと。** 移す範囲を決める → 共通の処理で通す → その範囲の入口を共通の道へ切り替える → 古い道へ 1 度も入らない事を確かめる → 不要になった古い分岐・専用の部品・test を、同じ series で消す。
4. **同じ入力に、「共通の道を試して、駄目なら古い道へ戻る」をしない。** 移した範囲は共通の道に固定し、そこで失敗したら失敗として見せる。道の選択は、生成の副作用が始まる前に決める。file 名や app の名前で切らない。
5. **main は、残す入力が通る状態を保つ。** 統合の途中だからと、残す入力が落ちる main にしない。入口を先に閉じるのは、要らない機能の退役と、メンテナが範囲を決めて認めた時だけ。
6. **「統合した」の合格の条件。** MIR の全文の一致は求めない。EXE の出力だけでも足りない。(a) 前に通った物が通り、拒否する物は同じ理由と段で拒否される。(b) 出力と exit に加えて、評価の順、状態の変更、片付け、Fault の時の動きが契約どおり。(c) 出した MIR が、独立の検証と、ABI・公開の形の検証を通る。(d) source と MIR の対応(loop なら、条件を評価する順と回数、`continue`・`break`・`return`の行き先、辺ごとに渡る値、PHI の対応、出口ごとの所有と片付け)。(e) 移した範囲は古い道を使わず、不要になった呼び手・専用の実装・test が消えている。受け付ける範囲が広がった差は、認めた差として、変わらない事の確認と分ける。
7. **どの道を通ったかは、4 つに分けて測る。** 候補を調べた / 道を選んだ / 実際に MIR を生成した / 公開と検証まで終えた。関数が実行された事は、その道が選ばれた事ではない。単位は、入力だけでなく、関数と source 上の位置まで。移した範囲では、古い生成の入口へ入った回数が **0** である事を合格にする(「減った」では足りない)。測るための印は一時の物で、最後は古い入口と実装を消す。
8. **test は、保証を移す。1 対 1 で写さない。** 古い道の内部の形(どの専用の型が選ばれたか)を確かめる test は消す。共通の道でも要る保証(PHI と前の辺の対応、出口での片付け、公開前の失敗が状態を壊さない事)は、共通の道の test で守る。同じ保証はまとめ、違う失敗の境目は残す。移した test が、本当に共通の道を通っている事を確かめる。
9. **試作と、普通の道の重複。** 保留の試作は、共通の道の広げ先を決める時に、1 度だけ照らし合わせる。採る部分とつなぐ先が決まらなければ、再開の条件を文書に残して、実装は消す。普通の道の中の重複(同じ判定を 2 か所で作る)は、その回が触る判定だけを、その場で 1 つにする。意味を決める検査と、出した MIR を独立に確かめる検査は、別の責務で、まとめない。
10. **共有の部品は、切り替えのたびに、消した呼び手が使っていた物から見る。** 節目で、固定した入力で全体を測り直す。実行が 0 というだけでは消さない(めったに通らない Fault の道、型の drop、登録の表、feature の構成を確かめる)。
11. **loop は、設計を 1 つにして、受け付ける範囲を段で広げる。** 基本の反復(loop が順に並ぶ形も) → `continue`・`break`・`return` → 入れ子 → body の中の文字列・配列・Map・呼び出し。順は、family の番号ではなく、今の共通の道に足りない契約の順。main・static・instance で loop の実装を分けない。`continue` の時に、更新の位置を共通化の都合で動かさない。resolver が既に持つ loop の親子の関係から始める。
12. **loop の統合を、selfhost より上の目的にしない。** 測って、止まる原因が loop でなければ(型を書いていない static 呼び出し、配列の出所など)、そこを先に直す。

## お掃除5: 保存を終えた試作と再開条件（2026-11-16）

現役の安全契約は共通ownerに保持し、保留のためだけの実装保存を終える。旧実装はGit履歴を参照し、再開時に並列の生成経路を復活させない。F17（catch/cleanup/TryCatch）は仕様確認中のため、この回の退役対象から除外する。

### SC33

common-V2専用のadmission/skeletonは採用しない。旧実装の参照: `6daa220107`。
具体的なsource→entryの不足と既存owner、置換する旧writerが決まった時だけ再検討する。

### PH03

canonical sessionをもう一本開く代替実装は採用しない。旧実装の参照: `3e3d39d632`。
実際のTextCallout不足を既存V2/SSA session内で解く時に再検討する。

### PH57

S6C専用のphysical Text/residence sidecarは採用しない。旧実装の参照: `f81180f8b6`。
選択されたsourceのentry/residence不足が確認され、既存ownerと後継の安全性testが決まった時に再検討する。

### PH65

raw collectionの別co-seal模型は採用しない。旧実装の参照: `2dd06f9693`。
現役collectorの所有契約に不足が確認され、既存root-publicationの検証へ保証を移す時に再検討する。

### EX01

特定Text走査のpre-session aggregate/論理planを一般反復へ持ち込まない。旧実装の参照: `3496bcf218`。
共通表現で不足する元source→edgeの関係が測定され、旧writerの削除を同時に行える時に再検討する。

### EX02

scalar scan/TextEqの専用corridorは採用しない。旧実装の参照: `935377a0e8`。
現役のsource/ABI不足を共通の意味計画で解く時に再検討する。

段Aの着地検証: 最適化CLIで固定5,386入力のMIR/拒否/出力/exit差0（EXE生成1,471、timeout0）、固定suite 8/8、Map18、保持したC verifier/closure、owner数・pointer guard PASS。全lib直列は8,639 PASS/210 FAIL/56 ignoredで新規FAIL0。
整形後Rustは本体11,604行/test6,378行の純減、Rust67file退役。登録名は89件消失・S6C保証移管1件追加（純減88）、未説明差0。metadata台帳の130/131件ずれとtransfer guardの既存欠落は基点からの不整合として保持し、基準は緩めない。

### 段Bの旧pass・oracle・shadow退役

- F08: optional VM barrier-elision passを退役し、実際のbarrierとVM本体は保持。
- F07: optional concat3 rewriteと専用smokeを退役し、現役concat/Fault/End/runtime契約は保持。
- F06: Pure専用New/Unary/WeakRef分岐と専用testを退役。通常経路・型情報・評価順・IntrinsicArray remap、混在VMのDebug/null比較は保持。exception/awaitのPure専用skipだけを除去し、F17の面とcarrierは変更しない。
- F01: MIR→JoinIR逆lowering/oracle入口を退役し、現役のJoinIR境界・名前・source解析を保持。
- F02: normalized-shadow/ANF模型と専用testを退役し、既存StepTree capability検査を元のcallerへ直接接続。strict/dev/planner-required境界を保持。

最適化CLIで固定5,386入力のMIR/拒否/出力/exit差0（EXE生成1,471、timeout0）、focused24件、固定suite8/8、naming/owner数/pointer guard PASS。整形後Rustは本体9,601行/test3,043行の純減、Rust76file退役。登録旧名97件消失・IntrinsicArray保証移管1件追加（純減96）、全件一意な旧定義へ対応。全lib直列はseries最終で確認する。
旧台帳は退役したPASS92件/ignored4件だけを削除し、残るFAIL集合は不変。corridor guardのconstructor旧anchor欠落は基点から同じで、検査を弱めない。F04/F16は次の塊、F17は今回の対象外。

### 段Bの参照VM入口 F04 退役

明示backend 3種（raw-vm-reference・normal-file-vm-reference・normal-file-canonical-core-vm-reference）と専有adapter/testを退役。共有VM本体・VM/Hako実行・raw compile request・source continuationを保持し、現役testが読むinteger_0 fixtureも保持した。
最適化CLIで固定5,386入力のMIR/拒否/出力/exit差0（EXE生成1,471、timeout0）、focused22件PASS。featureの通る2構成はPASS、既存のvm-reference 1件とcli-no-plugins 11件のerror集合は不変。
整形後Rustは本体6,225行/test3,126行の純減、Rust40file退役。登録名75件減・全件一意な旧定義に対応。旧段階名/anchorに依存する一部guardの既存不整合は保持し、基準を緩めない。F17のparser・AST・TryCatch carrier・catch/throw/cleanupは変更なし。

### 段B F16: 古い lambda 生成の退役

旧`fn(...) {...}`は元のexpression dispatcherで`LegacyLambdaSurfaceRetired`として拒否し、専有のcapture/closure生成器とfree-variable collectorを退役した。将来の`fn[captures](...): T`は予約のまま無効。公開NewClosure/ClosureBodyId・JSON reader・closure call独立検証・source capture情報を保持し、F17の面とcarrierは変更しない。
固定5,386入力の差は承認済みの`phase29bq_selfhost_funcscanner_lambda_literal_min.hako`と`write-compare/t3_map_update.hako`だけ。両者は所定の拒否に変わり、残り5,384入力のMIR/拒否/出力/exitは不変。EXE成功1,471件を維持しtimeout0、dumpのみの後者も旧/新binaryでEXE未生成と補足確認した。
focused23件PASS。旧生成成功testを同じ入力の名前付き拒否・命令/closure非公開・変数不変のtestへ置換し、専有test12件を削除（登録8734→8722、旧13名/新1名・未説明差0）。Hako parserのlambda-pair smokeはJSON解析の保証なので保持。整形後Rustは本体1,022行/test350行の純減、Rust4file退役。旧台帳は退役PASS12件と置換名/hashだけを更新し、FAIL集合を維持した。
固定suite8/8、owner数・pointer guard・diff check PASS。全lib直列と全feature最終確認はseries最終で行う。

### 段C F22: CallableSingleLoop 専用経路

元のcanonical/static-child入口を閉じた最適化CLIで固定5,386入力のMIR/拒否/出力/exit差0（EXE1,471、timeout0）を確認してから、専用のsource/Recipe/physicalizer24fileを退役した。現役のsource-shape helper、共有loan/receipt・typed completion・profile countを保持した。
共有のowner/continuation/allocator/segment/recursive-after検査は既存GenericG0のverified fixtureへ移し、重複layout fixtureを既存test-supportへ集約。キーの発行権限を広げず、共有assertionを保持した。旧形式専有56testを削除し登録8722→8666・追加0・全名対応、旧台帳もそのPASS56件だけ除去しFAIL集合不変。
削除後focused232件・owner数/pointer/diff検査を確認。整形後Rustは本体4,342行/test3,214行の純減。JoinSig guardのstatic_loop_body未許可は基点から同一の既存不整合として保持する。削除後の全入力・全lib・固定suite最終確認はseries最終で行う。

### 段C F23: 差分検出による保持

旧NestedPredicate入口を閉じた固定5,386入力の比較で、4入力のdump/EXE拒否診断に差が出た。exit1・EXE未生成と`RowUnresolved(DirectAccum)`は同じだが、診断のadmission行一覧から旧Nested行が消えた。差0の条件を広げず、このfamilyの変更を全て戻し、専用実装・test・共有検証を保持した。対象は`if_phi_join_min`、`phase118_pattern3_if_sum_min`、`phase264_p0_bundle_resolver_loop_min`、既存regress入力`coalesced-loop`。公開する診断情報の扱いが決まった時に再検討し、今回はF25へ進む。

### 段C F25: selected Dynamic V2 専用物理経路

元のstatic-child入口を閉じた最適化CLIで固定5,386入力のMIR/拒否/出力/exit差0（EXE1,471、timeout0）を確認。専用emitter/ABI/capabilityと、呼び手が消えたA-prime source view・physical demandを同じ塊で退役した。private ABI構造の削除であり、公開MIR/JSON/C ABIは変更しない。current package issuerが使うlogical/Fault/cleanup co-sealとrunnerのroot fenceは保持。
削除後focused197件（Dynamic logical契約・external commit・targetless binding・独立検証・ownership）PASS、登録8666→8645・旧専有21件のみ削除。旧台帳はそのPASS21件だけ除きFAIL集合不変。整形後Rustは本体7,962行/test1,709行の純減、Rust32file退役。
旧fixture生成CLIと専用guardを退役し、共有のSSA/比較/単一append検査を既存guardへ移管。移管block・wire/text guard・C provenance driverとPython33件PASS。F22由来の旧writer guard参照漏れもここで解消した。既存authority guardのloan-port旧anchor欠落（基点から0件）とDraftSeal raw drop件数2/2対期待1/1は緩めず保持し、全guardがgreenとはしない。owner数・pointer/diff検査を確認し、全入力/全lib/固定suiteの削除後最終確認はseries最終で行う。

### 段C F19: 古い生の enum writer

元のFromCall/EnumMatch入口を閉じた最適化CLIで固定5,386入力のMIR/拒否/出力/exit差0（EXE1,471、timeout0）を確認してから、旧Variant writerと専有adapterを退役。Option/Result/enumの表現・payload/網羅性・公開MIR/JSONを保持し、nullish/arityのsource preflightを本文不変で既存semantic ownerへ移した。
ScopeBoxのno-retry/cleanup/temp binding保証3件は関数本文をbyte不変で現役ownerへ移管。旧writerとのparity2件のみ退役し、登録8645→8643、移管3組を含め未説明差0。削除後focused143件・owner数/pointer/diff検査PASS。整形後Rustは本体575行/test37行の純減。削除だけのstructured-child fileは未変更箇所の整形を保ち713行（全体rustfmtで827行へ膨らむ差分を加えない）。全入力/全lib/固定suiteの削除後確認はseries最終で行う。

### 段C F26: 選ばれていない If Recipe wire

入口遮断時と専有wire/mapper/physicalizer削除後の最適化CLIで、固定5,386入力のMIR/拒否/出力/exit差0（EXE1,471、timeout0）を確認。共通Ifのsource/Bool/coverage/実際の前駆辺・PHI・原子的publication検証は保持し、未使用の専用observerとfactsだけを同じ塊で退役した。公開IfRecipeDTOとF17のcarrierは保持。
既存nested source検証をsource path・原本binding・InlineI64の完全な組の照合へ統合。削除後focusedは161 PASS/既知1 FAIL・新規FAIL0、登録8643→8617（専有26件退役・保証5件改名、未説明差0）。vm-reference専用testをdefault実行の証拠とは数えない。整形後Rustは本体5,296行/test1,172行の純減。owner数・共通If guard・pointer/diff検査PASS。旧SSA guardの存在しないidentity.rs要求は基点からの不整合として保持し、変更したprofile-close検査は個別に実行した。全lib/固定suiteの最終確認はseries最終で行う。

### 段D in-body-step: 共通 SourceLoop V2 へ

旧selectorを閉じた最適化CLIで固定5,386入力を比較し、差は承認済みin-body-step 1本のみ（EXE exit3、EXE成功1,472、timeout0）。同一binary/環境の計測あり・なし比較も差0。対象の旧selector/producer/physicalizerは0回、既存V2の実命令生成→独立検証→公開を確認してから専用12fileを退役した。追加production glue0。
削除後の共有focused24件PASS、既存rootテストは8 PASS/残る既知3 FAIL・新規FAIL0。公開/PHI保証の正例2件は名前・source・assertionを変えず既知FAILからPASSへ改善した。旧tail-membership拒否だけの1件も専有testとして退役し、登録8617→8599（専有18件、未説明差0）。整形後Rustは本体2,487行/test892行の純減。計測印は全撤去、owner数/pointer/diff検査PASS。全lib/固定suiteと削除後の最適化CLI比較はseries最終で行う。

### 段D continue: 更新位置を保って共通 V2 へ

旧selector遮断後の最適化CLIで固定5,386入力の差は承認済みcontinue 7本だけ（累積EXE1,479、timeout0）。同一binary/環境の計測あり・なしは差0。上限2本のFault102/exit70を維持し、移管済み8本で対応する旧経路0回・共通V2の実命令生成/独立検証/公開を確認した。
専用10fileと15testを退役し、derived-predicateが使うneutral facts/map型だけを同じownerに保持した。追加production glue0。削除後focused47 PASS/残る既知2 FAIL・新規FAIL0、公開正例の名前/source/assertionを保持したまま更に1件がPASSへ改善。登録8599→8584、整形後Rustは本体2,790行/test884行の純減。owner数/pointer/diff検査PASS。最終の全lib/固定suite/clean最適化CLI照合はseries最終で行う。

### 段D derived-predicate と最後の専用 root dispatcher

最適化CLIの固定5,386入力比較で差は承認済みderived 4本のみ（累積12本、EXE1,483、timeout0）、計測あり・なしの差0。12本全てで旧3 familyのselector/producer/physicalizerは0回、共通V2の命令生成→独立検証→公開を確認。上限2本のFault102/exit70も保持した。
最後の専用dispatcher/Option facade・Main0専用terminal・observer・共有残片を退役し、App Mainは元のNone分岐と同じ現役wrapper経路へ直接渡す。現役のroot loan APIへ原本owner/identity・不一致時の未消費・一度だけの消費を既存test owner内の1件へ移管した。追加production glue0、公開MIR/JSON/C ABIの変更なし。
削除後focused31件PASS。公開正例5件は名前/source/assertionを保持し全てPASSへ改善、旧tail-membership拒否1件も専有testとして退役。登録8584→8565（旧20名退役・現役保証1名、未説明差0）。整形後Rustは本体3,047行/test951行の純減、Rust16file退役。計測印全撤去、owner数/pointer/diff検査PASS。共有collectorのMain/policy/identity契約は保持し、最終のclean全入力/全lib/固定suiteはこの後に確認する。

### 段E: F24退役後の完了 helper

3 profileの呼び手が消えた`complete_main0_root_draft_v1`と`commit_callable_single_loop_ready_to_pending_v1`、専用test1件を退役。正確なExit/ExitBlockAlreadyTerminated（block0）、親状態の復元、fresh sessionのopen/discardを既存の拒否testへ統合した。現役pending/abort/dropの21件と、共有Main/policy/identity・collector安全検査は保持する。generic key経由の共有collector armは閉じ切れないため削除しない。
統合後focused43件PASS、登録8565→8564・追加0・未説明差0。試案の追加fresh成功fixtureは新しいblockを固定IDで扱う誤りがあり採用せず、元の両testの保証を保持した。整形後Rustは本体53行/test49行の純減。pointer/diff検査PASS、最終の全入力/全lib/固定suite/owner確認はこの後に行う。

### 段E: F02の旧capture補助型

F02の実消費者が消えた`loop_route_detection::support::function_scope`のCapturedEnv/Kind/Varを退役した。全Rustの同名/別名module参照を確認し、定義・自己test・再export以外の呼び手は0。別crateのCapturedVar、現役AST capture、F03の広いcanonicalizer/route検査は保持する。
専用test5件も同じ変更で削除し、登録8564→8559・追加0・未説明差0。focused100件PASS、整形後Rustは本体107行/test69行の純減、Rust2file退役。pointer/diff検査PASS。この後はsourceを固定し、通常の最適化CLIと全lib/feature/固定suite/Map18/ownerで最終確認する。

### 最終全libで判明した保持境界と混在testの修正

最終全libは新規4 FAILを検出した。うちF22の通常API入力2件（Main.int_to_strのloop）は、固定5,386入力のcut差0だけでは拾えなかった本当の受理回帰。入力/期待値を変えず、必要な元のCallableSingleLoop経路・専用検証・pending helper/testを保持へ戻した。F22とE1のpending部分の先の削減値は最終実績から除く。Main0専用完了helperの退役、F25/D/Eの他の退役は維持する。
残る旧Lambda拒否優先順位parityと旧raw enum writerの混在testは、廃止形式の部分だけを整理。local失敗後の同一compiler再利用、exact source/scrutinee・一度だけの評価・拒否時無変更/no-retryを現役ownerで保持し、正規enum payload/resultは既存bridgeの保証へ対応付けた。登録名を変えず、修正後focused50件PASS。旧PASS登録57件と復元ownerのmetadataだけを戻し、FAIL基準は変更しない。最終の全入力/全lib/固定suiteは再検証する。

### Opus の確認と統合(Round 5、2026-11-16)

- branch `cleanup-5`(14 commit)を main に統合した。worker は 8 時間の上限で止まり、最後の全件比較だけ、source を変えずに後から完走した。
- 消した物: 保留の試作 6 個。family F01・F02・F04(入口だけ。VM の本体は残る)・F06・F07・F08・F16・F19・F25・F26。F24 の 3 つの profile(共通の SourceLoop V2 へ移した)。
- **残した物**: F22(固定 5,386 入力では 1 度も選ばれないが、消すと lib の test 2 本が通らなくなった。通常の API から来る入力が使っている)。F23(閉じると 4 入力の拒否の文が変わる)。F17(触っていない。ADR-004 の「古い例外の形の family を消す回」で扱う)。
- **学んだ事**: 「固定した入力で選ばれた回数が 0」は、「要らない」の証明にならなかった。入口を閉じる前に、通常の API から入る test も走らせる。
- 大きさ: `src/mir` の本体は、`cfg(test)` を除いた整形後の行数で 687,829 → 640,103(test の file を除いただけの生の行数で 735,555 → 681,122)。`src` 全体の整形後の差は、本体 −58,092、test −11,202(inline の test の分け方が worker の表と違う。worker の表は 本体 −50,745、test −18,549)。Rust の file は 281 減った。
- 確かめた事(Opus): binary の SHA が報告と一致し、今の source からの build が「変更なし」になる。手元の 393 入力は、結果も MIR も Round 3 と同じ。parser・AST・`MirInstruction` の定義に差が無い。stash は 2 つのまま。
- worker の log で読んだ事: lib は 8,359 PASS / 205 FAIL / 52 ignored。205 は Round 3 の 214 の部分集合(退役 4、緑になった 5、新しい赤 0)。固定 5,386 入力の全件比較は、承認済みの差(F16 の 2 入力の拒否、F24 の 12 入力の新しい受け付け)だけ。feature の build は、通る 2 構成が通り、前から通らない 2 構成の error は同じ。
- 確かめていない事: 既定でない feature の下での動き。Python の検査(v4)は、C 側を変えていないので走らせていない。
- 次の候補: F22 を共通の loop へ移す(統合してから消す)。F23 の拒否の文を、共通の道で同じ内容にしてから消す。まだ決めていない 9 family(F03・F05・F09・F10・F11・F15・F18・F20・F21)。使い手のいなくなった共有の部品。


## お掃除6 A: 使い手のいない準備部品（2026-11-16）

基点はmain `6a5f2d453f`。既定・通るfeature二構成・testの警告を照合し、名前・登録・macro・feature側の参照を別に確認した。
未接続のScript transactionとouter entry、static/Dynamic rebindの準備wrapper、Dynamic physical evidenceの未使用readout、Pinned Textの未使用Ingress/Lineage/install wrapperを削除した。
現役のScript sessionとその破棄・復元、originの検査とtest、source coverage、frame/carrierの発行・投影・独立検証は保持した。失敗するfeatureだけでも使い手があるVMの表示・JSON arena・数値処理は残した。F17は変更していない。

- 同じ整形条件でproduction **1,093行減**、test **18行減**、Rust file **2本減**。登録testは8,616件のまま。減ったtest行は未使用の準備wrapperであり、test関数・assertionの削除はない。
- MIR層の本体は、生のtest-file除外で681,122→680,043行、整形・cfg(test)除外で640,103→639,010行。33万行の目標は未達。
- 既定buildの警告は2,337→2,290件。警告を抑える属性や公開範囲の拡大はしていない。
- 最適化CLIで固定5,386入力のMIR・拒否・出力・exitの差0、timeout 0。全lib直列は8,359 PASS / 205 FAIL / 52 ignoredで失敗名集合も登録名集合も不変。
- 固定suite 8/8（空markerと実EXEのsmokeを区別）、ignored Map 18ケース、owner guard（承認基点6a5f2d453f）・pointer・差分checkはPASS。二featureはPASS、既存二FAIL構成はエラー集合を維持した。
- 既存のScript physical guardは、基点から欠けている`into_recipe`の目印で前後同じFAIL。現役部分の検査を弱めず、消した準備部品だけの節を除いた。metadata台帳の既存130/131不一致と520行anchorのずれは修正対象へ混ぜていない。 projection guardも、既存provider観測7件に対する期待6件の不一致を前後とも保持した。


## お掃除6 B/C/D: 保持した境界と単独閉鎖の調査（2026-11-16）

F22の切替試作では、元のAPI test二本と既存のreturn-0保護が通り、実際にcalleeを呼ぶ二入力もEXEでexit 7/37になった。しかし固定入力の比較でboxtorrentの既存EXEに回帰が出たため、試作・追加入力は保持用patchへ退避し、変更を戻した。F22の削減は0行。
原因は`release`の`local remain = chunk.release()`から、入れ子のIfを越えた`return remain`への対応。初期化時の証拠は存在するが、分岐入口と合流の正規PHIでSSA値が変わるため、Copyだけの原本Call証拠では足りない。実predecessor・全incoming・元Callを結ぶ共通join証拠が必要であり、到達性・支配関係の既存検査や必須initializer照合は弱めない。単純なAPI正例の成功を製品全体の成功と数えない。

F23は四入力を新しく計測した。dump/EXEの八組とも、診断内の`NestedPredicate(Declined ... NotNestedPredicateShape ...)`というadmission要素一件（1,363 bytes）だけが消えた。exit 1、EXE未生成、`RowUnresolved(DirectAccum)`は同じ。一方、閉鎖した全libではNested profileの受理・検証・状態復元を含む八本が新FAILになった。診断が試行履歴であることだけでは退役を正当化できず、F23は残す。

残る九familyは、一つのprocessでは一familyだけを閉じて調査した。全lib直列の新FAIL数はF03=11、F05=1、F09=1、F10=25、F11=11、F15=3、F18=4、F20=3、F21=0。未閉鎖の観測用binaryは8,359 PASS / 205 FAIL / 52 ignoredで基準と同じだった。
F03の十本は旧advisory classifier自体のtestで、残る一本はdev parity。F11の十一件も専用metadata producerのtestであり、古いsubjectを退役する判断時には一緒に扱える。ただしF09の失敗順序、F15の評価・無公開、F18の現行MatchExpr、F20の正規Script/QMark、F10の共有sourceと復元、F05の公開JSON bridgeは移管先が必要。F21の閉鎖で新FAILがないことも、共有issuerの契約が不要という証明にはしない。

全一時観測・閉鎖・機械的test変更は撤去した。九familyとF23の物理退役は行わず、F17にも触れていない。固定入力での選択・閉鎖差と最終検証の詳細は、この回の測定報告で確認する。

### Opus の確認と統合(Round 6、2026-11-16)

- branch `cleanup-6`(2 commit)を main に統合した。入ったのは、使い手のいない包み(本体 −1,111 行、整形後)と、上の記録だけ。メンテナの指示で、測定の途中で止めた。
- 大きさ: `src/mir` の本体は、`cfg(test)` を除いた整形後で 640,103 → 639,010(生の行数で 681,122 → 680,043)。build の警告は 2,337 → 2,290。
- **F22 の統合は失敗して、戻した(削減 0)。** 共通の道に足りない物は 4 つ: (1) 同じ loop に 2 つの事実(単純な while と、出口の無い条件 loop)が付くと、source を貸せない。(2) 上の階の関数の i64 の引数の証拠が、共通の入口へ渡っていない。実引数はリテラルだけが通る。(3) `return` が元の呼び出しの結果を読む証拠の接続。(4) `if` の合流を越える出所の証拠。1〜3 を足すと、前から通っていた `apps/boxtorrent-mini` の `release`(不変な local を、入れ子の `if` の後で `return` する形)が拒否に変わった。出所の証拠が Copy しかたどれず、合流の PHI で切れる。
- F23 は残す(閉じると lib が 8 本赤くなる)。9 family(F03・F05・F09・F10・F11・F15・F18・F20・F21)は、固定した入力では選ばれないが、閉じると lib が合わせて 59 本赤くなる。閉じた時の全件比較が終わったのは F09 と F11 だけ(差 0)。
- 読み方(Opus。決定ではない): 「使われていない物を捨てる」段は、ほぼ終わった。残りはつながっている。1 つの普通の形を足すのに、9 つの持ち主・15 file に手が入った事は、共通の道が、形ごとの細い証拠の積み重ねである事を示す。設計の相談を出した(公開の repo の外)。答えが来るまで、worker は止めている。
- 確かめた事(Opus): 今の source から作り直した binary で、手元の 393 入力の結果と MIR が Round 5 と同じ。parser・AST・`MirInstruction` の定義に差が無い。stash は 2 つのまま。binary の SHA は、worker の段 A の時の値と一致しなかった(実験の build の後の作り直し。段 A の後の差は文書だけ)。
- worker の log で読んだ事: 段 A の時、固定 5,386 入力の差 0、lib は 8,359 PASS / 205 FAIL / 52 ignored で、赤の名前は Round 5 と同じ。


## お掃除7 段1-a: 二つの道と照合の時点（基点5c323061fc）

数は以下で列挙した境界に限る。同じ関係を再確認する回数であり、安全検査の不要数ではない。
F22は既存API二本のhelper定義一つを単位とする（bound 0/1/3は同じ道）。
- 意味: `compiler/callable_single_loop_syntax_facts.rs:315`でownerと唯一のLoop、`callable_single_loop_source_map.rs:274`でorigin・frame・binding、`callable_single_loop_recipe_coseal.rs:261`でroleとRecipe/JoinSigを確定。
- 意味/再確認: `builder/normal_callable_semantic_source.rs:297`、`normal_callable_prepared_operation.rs:112`、`normal_callable_semantic_loan_port.rs:163`で同じcontext/signatureを照合。
- 作成/再確認: `builder/resolved_lowering/loop_recipe_physicalizer/callable_canary.rs:163`でheader/bindingを照合してprelude、`tail_completion.rs:183`でowner/binding/read siteを照合してtailを作る。さらに`normal_cataloged_box_method_lowering.rs:250`がprogramとphysical signatureのownerを照合する。以上の再確認境界は9。
- 中継は3箇所の受渡し部分: loan portの`root_capture.rs:68`の選択・検査後のreturn transfer、prepared operation programの`into_parts`、`normal_cataloged_box_method_lowering.rs`の検査後のsession/admission受渡し。関数全体を純粋な中継に分類する意味ではなく、owner検査・失敗時discard等は除く。
- 別collectionからの再検索は5境界: forest→source ledger、syntax→resolved loop、source→forest ledger、operation context→only loop、prelude→header indexとfunction binding。
- 作成順は`callable_lowerer.rs:160–239`のprelude→loop→tail/session終了。source/context照合はCFG完成前、最後の独立検証は`compiler/normal_default_pipeline.rs:657`で完成したmoduleを読む。

boxtorrentは`BoxTorrentStore.release`（main.hako:100–113）のCall由来localを読む経路を追った。
- 意味: `resolved_semantics/home_prefix_scalar.rs`でその時点のbinding/site/producerを決め、`home_new_prefix_condition.rs:691`がOriginal scalar referenceを保持する。
- 入口: `builder/raw_invocation_source_transport/bool_condition.rs:239`→`ordinary_new_local_commit/root_call_entry/expression_reference_read.rs:179`のprepare→同fileのscalar arm→`expression_reference_source.rs:8`。
- 元定義: `expression_scalar_origin.rs:5`→`ordinary_new_local_commit/call_validation_context.rs:605`で原本Call/siteと唯一の既存groupを照合、`materialized_values.rs`のCopy-chain loanへ渡す。
- このexpression prepare成功一回でinitializer照合3回（producer作成、producer検証、source検証）。各回で同じsource rowを二度引くためsource検索/比較6回、元group検索とCopy-chain照合は各3回。packet内部と後続再検証は数に含めない。
- 純粋な中継はこの範囲で1境界（scalar observation→with-branchesの委譲）。Copy-chainのloanは検査があるため中継に数えない。
- prepareはその式の命令発行前、親のbranch/PHIが未完成の時点で走る。依存列は初期化式の早期censusにも使う。照合を一行後ろへ移すだけでは済まない。
- 完成後の既存フックはexpression readのfinished dependencies、groupのfinishing検証、最終strict verifier。sourceの束縛・期待定義・Call契約は早期に残し、物理同値性だけをここへ移せるかを段2で調べる。
- 重要: 基点の`home_new_prefix_terminal.rs:527–532`はCallをplain returnのoriginal readから除外する。`return remain`で強い照合が走って壊れたのはお掃除6の試作であり、現行のreturn経路として数えない。F22の専用道も、この共通Copy照合とは別。

段1-bの規則: 実CFG・SSA一意性・支配を検査した不変な関数で、i64の実定義を基点にCopyと循環のないPHIの全incomingをたどる。同じ／別の基点／未解決を区別し、Box・token・借用・heap読取・loopの循環へ拡張しない。段1ではtest以外から呼ばない。

段1-bは`verification/scalar_origin.rs`（297行）へ実装した。既存CFG/SSA/Invoke検証とgeometryを再利用し、Copy・全incoming PHI・支配と順序・一意性を確認する。循環/不明/対応外は未解決、別の基点は別とする。型付きInvokeと実callee署名を照合し、raw Callの不明結果・tagged/u64/Boxは成功にしない。production callerは0。
独立検証ownerの責務別test10件を追加した（新しい解析の正負境界で、既存検証の登録名・assertionは削除していない）。F22の既存二本もPASS。固定5,386入力はMIR・拒否・出力・exit差0/timeout0、全lib直列は8,369 PASS/既知205 FAIL/52 ignoredで新しい赤0。
段1の増分は整形後production +296/test +382行。本体はcfg(test)除外で639,010→639,306、生のtest-file除外で680,043→680,342行。段2の切替とF22の穴1〜3はまだ実装していない。

### 段2で選ぶ接続の規則

- Source側はowner/site/binding・原本Call/引数/結果契約を先に確定する。i64 Call由来の式readは、元定義と現在のreadを保持し、Copy起源の証明だけを完成済みmoduleで必須検査する。別origin・未解決・不正CFGは拒否し、旧照合へのfallbackはしない。
- 通常compile/document/artifact共通のRootValidationと、finalized sourceの再検証にmoduleを貸す。module情報の有無と、Lowering/Finalizedの権限は分け、source-only依存inventoryの確認を同値確認済みの許可にしない。
- 既存operand correspondenceとfinisherの実定義照合を保持する。記録されたCopy一覧はinventoryとして借り、起源の証明にはしない。削除済みreadは、既存の省略許可・定義なし/useなしが成立した場合だけ扱う。単なる解析未解決は成功にしない。
- 原本calleeの実Integer署名・宣言と実Callを照合し、未注釈の既存i64結果も同じ経路に置く。Returnの事実はcanonical terminal内で一度保持して共有し、read/Call/exitの一回限りの権限は複製しない。F22穴1〜3とCall plain-return選択は変更しない。
- 完成MIRの厳格なuse/PHI edge検査は既存`uses::check`の一回の走査で計算し、geometryから借りる。旧SSA/dominance/mergeの結果と公開verifierの診断選択は変えず、queryだけが厳格結果を要求する。別の全命令走査は残さない。

### 段2の着地

i64 Call由来の対象readは、原本の意味契約を保持し、完成moduleの必須gateで実operand/Copy/非循環PHIの起源を検査する。事実はcanonical terminalの共有参照にし、一回の権限は複製しない。共有Copy helperには対象外callerが残るため全面退役とは数えない。
固定5386入力のMIR・拒否・出力・exit差0、timeout0、新規受理0。固定集合外の入れ子if正例は以前のinitializer-missingから正規EXE・exit7へ。publicationのoperand/PHI入力/生きた定義の改変3負例も拒否する。
全lib直列8370 PASS/既知205 FAIL/52 ignoredで赤の名前集合一致、suite8/8（空markerあり）とMap18 PASS。feature二構成はPASS、vm-referenceの既存1 error・cli-no-pluginsの既存11 errorsは基準内。owner全指標一致、definition-scan候補286のまま、基準変更なし。
本体はcfg(test)除外・整形後639010→639544（+534）、test347774→348275（+501）。生のtest-file除外本体680043→680597。機能追加を含み純減とは主張しない。F22の穴1〜3とF17は未変更。

### 段3下調べ（未実装）

**穴1: 共通loop許可**
特徴の一致数は診断に残すが、許可は同一owner/source site/一般loop契約に一つ、writerも一つにする。特徴の優先順位で選ばない。
既存V2 productとgeneric issuerの接続を整理する。候補は数ファイル・数百行規模の仮見積で、実装前にsource portを追う。
保存した5386入力ではzero-selected-family拒否76入力、特徴重複を明示する診断0。76は新規受理見積でも独立原因数でもなく、受理増の本数は未確定。

**穴2: i64引数**
literal/formal/local/Call結果を既存CheckedExpressionEvaluationV1の検査済み結果として同じ実引数へ渡す。
local-call flow・引数封印・direct-call projectionを接続し、literal専用分岐を退役する。数owner・数百行規模の仮見積。
左からの評価順、一回だけの発行、Normal/Faultの責任を保持する。MIRの形からsourceの型やcalleeを推測しない。

**穴3: return/未呼出宣言**
段2ではplain returnのCall producer選択を変えていない。このsource接続と、未呼出宣言の参加/membershipが残る。
原本Call定義と現在のsource readを確定し、同値は完成CFGへ渡す。未呼出宣言には偽のCall edgeを作らない。
terminal sourceとsealed definition inventoryの二責務で接続する。数ファイル・数百行規模の仮見積で、両方の保証確認後にF22専用routeを削除する。

**循環PHI**
Copy/PHIの循環成分について、外から入る全定義と全backedgeを実CFGで確認する案。
全外部入力が同じ指定基点で循環内に値を変える演算がない場合だけ同値候補とする。基点なしの循環・別基点・演算更新は成功にしない。
既存queryへ循環解決を加える別作業で、1〜2責務・百行単位の仮見積。source Call siteの一致を値の一致にしない。

**診断案**
sourceに初期化が無い場合と、初期化はあるが使用値との対応が証明できない場合を分ける。
後者の候補はi64-origin-unproved、「この読みが元の定義と同じ値であると証明できません」。不正CFGは既存構造診断を維持する。
未対応と意味違反も区別する。名前・文言の変更は別承認事項で、この回は変えない。

**既存F22 API二本の保証**
一つ目はbound 0/1/3、実counterのheader/backedge/条件と元Call結果のreturnを保持する。専用block配置や不要な命令数の固定は実pred/operand/結果の保証へ見直せる。
二つ目はPairの正規公開と未呼出helperが共存して物理ABIへ届く保証。helper呼出しを入力に足して代用しない。
Call一回発行・引数対応・Normal/Fault検証・return 0のRaw経路を保持する。未呼出宣言をどこまで検査/公開するかは独立契約として残す。

### 監査の確認と統合(Round 7、2026-11-16)

- 統合: `81c4522bb2`(`--no-ff`)。作業の commit は 4 つ(経路の記録、段 1、段 2、段 3 の下調べ)。
- **この回の狙いは、行を減らす事ではない。** 「値が同じである事」を、作る側が写して運ぶ証拠ではなく、できあがった MIR から読む、という形への最初の切り替え。行数は増えた(整形後・`cfg(test)` を除いて 639,010 → 639,544、+534。生は 680,043 → 680,597)。
- 監査で自分で確かめた事:
  - binary は統合する commit で作り直しても同じ(cargo が再 build 不要と答え、SHA-256 は報告と一致)。
  - 393 入力は、MIR・結果・拒否の文が Round 6 と同じ。違いは、出力先の path と一時 file の番号だけ。
  - 全 lib の赤 205 件は、名前の集合が Round 5 の基準と同じ。8,370 PASS。
  - parser・AST・`MirInstruction`・古い例外の family(F17)・`.hako` は、差分に無い。stash は 2 つのまま。
  - 固定の入力に無い 8 本を書いて、Round 6 と比べた。退行 0。**新しく通った形は 4 本**(下)。動かした結果は、手で計算した値と合う。
- **新しく通る形(受理が広がった所)**: i64 を返す呼び出しの結果を `local` に入れて、`if` の後(入れ子、`else` あり、深い入れ子、2 つの `local`)で読む形。前は `[freeze:contract][local-read/initializer-missing]` で拒否していた。固定の 5,386 入力には、この形で拒否されていた物が無いので、そちらの新しい受理は 0 件。
  - 変わらない物: 途中で別の呼び出しの結果を入れ直す形と、loop をまたぐ形は、前から通っていて同じ結果。呼び出しの引数に `local` を渡す形(`me.id(x)`)は、前と同じ拒否(F22 の穴 2)。
- 残っている事(次の段で見る):
  - 新しい道は、`deferred_i64` という印で、i64 の呼び出しの結果の読みだけを分けている。ほかの読みは、古い Copy だけの照合のまま。つまり今は道が 2 本ある。古い共通の関数は、使い手が残っているので 0 行も消えていない。
  - 問い合わせ 1 回ごとに、その関数の geometry を作り直して CFG の検査をやり直す。今の入力では compile の時間は変わらない(393 本で 78 秒、前は約 80 秒)が、読みの多い関数では重くなりうる。関数ごとに 1 回にまとめる余地がある。
  - 読みが消された時(定義も使用も残っていない時)は、問い合わせずに通す。これは仕上げ側の今ある許可を使っている。「解析が分からない」を成功にしてはいない事は、負の test 3 件で見た。
  - 警告は 2,290 → 2,291(test からだけ使う import が 1 つ)。
- 「正本を読むか、写しを作るか」の数(この回から数える): 値の関係を写して運ぶ所は、対象の読みで 1 つ減った(Copy の列を証拠にする呼び出し → 完成した MIR への問い合わせ)。事実の写しは 1 つ減った(`ScalarBindingReadProofV1.source` が複製から共有の参照へ)。持ち主の候補は 286 → 286。
- F22 を消すには、穴 1〜3 と、循環する PHI の決まりが要る(上の「段3下調べ」)。見積もりは「数 file から複数の持ち主、数百行」で、確かな数ではない。

## お掃除8: 段Aの規則

- 完成MIRのCopy/PHIの全入力から値依存closureを作り、検証済みの実基点から逆到達する。全nodeが基点へ届き、全基点が指定定義の時だけSame。別基点はDifferent、対応外/基点のない閉循環はUnresolved。実CFG/支配/全前駆の検査は既存geometry/usesを使う。
- Integer定数も実ValueIdの基点として扱う。同じ数値の別定義を同じ基点とはしない。循環の中の算術を計算して同値とみなさない。
- 不変localの根拠は既存source role ReadOnlyOperandとentry producer。Carrierの現在Phi・更新責任は保持し、新しい不変flagやsource形分類を作らない。

### お掃除8 段Aの着地

Copy/PHIの循環を、全入力の値依存closureと全実基点からの逆到達で確認する。基点のない部分循環を、有効な基点の兄弟があっても成功にしない。異なる基点と未解決を区別し、演算の数値的な同値は扱わない。
Call/Integer由来の不変localのexpression readは、元binding/ownerに結び付いた原定義locatorと完成MIR問い合わせへ統合。対象の旧Copy-chain照合は使わず、mutable carrier/source relation/Normal-Fault/operandの検査は保持。その他のi64 caller・deferred印・関数ごとの検査共有は段Eに残る。
問い合わせtest13（入れ子loop・loop内if・異なるbackedge・加算・異なる二入口・入力欠落・無基点部分循環を含む）、既存publication改変testとF22二本でfocused16 PASS。旧cycle testのanchored cycleは新規則でSameへ変更し、無基点と加算の拒否を別の負例に保持した。同数の別ConstはDifferent。
固定5386入力のMIR/拒否/出力/exit差0、timeout0、新規受理0。全lib直列8373 PASS/既知205 FAIL/52 ignored、赤の名前集合一致。通常loop/入れ子loopとお掃除7の正例も基準と同じexit。loop内ifのsource例には既存execution-root-purpose/variable-accum-recurrenceの拒否が残り、成功とは数えない。
owner指標は基準aa91632b9cと一致（定義走査候補286）、警告2291→2290。触ったRust fileは全体をrustfmtし、LocalProvenanceの問い合わせを既存value-slot格納から責務分割、元の可視範囲を保持。整形後本体639544→639605（+61）、test348275→348360（+85）。生のtest-file除外本体680597→681146で、整形による物理行増と区別する。F22/F17は未変更。

### 監査の確認と統合(Round 8、2026-11-17)

- 統合: `e43e30670a`(`--no-ff`)。着地したのは段 A だけ(循環する Copy/PHI の決まり。loop の間変わらない i64 の読みを、完成した MIR への問い合わせへ)。
- 監査で自分で確かめた事: binary は統合する commit で作り直しても同じ SHA。393 入力は MIR・結果が Round 7 と同じ。全 lib は 8,373 PASS、赤 205 件は基準と同じ名前。parser・AST・命令・F17 は差分に無い。stash は 2 つ。
- 新しく通る入力は 0。本体は整形後で 639,544 → 639,605(+61)。生は 680,597 → 681,146(+549。触った file を rustfmt の形にしたための行で、中身の増加ではない)。
- **段 B(引数)は、つなぐ前に止めた。** 見積もりが「追加 700〜1,100 行、20〜28 file、40〜55 関数」になり、手紙の「数百行を大きく越える段は、始める前に閉じて報告」に当たった。試作は patch に保存して戻してある。
  - 足りなかった物: 呼び出しの結果の source・式の葉・packet の保持・物理の照合が、receiver のある呼び出し(Method)を前提に作られていて、receiver の無い関数の呼び出し(`id(id(7))`)を同じ式の評価へ渡す接続が無い。
  - これは「形を 1 つ足す時に、何か所を直すか」の実測に当たる(未実装の見積もりで、20〜28 file)。**出所の持ち方だけを直しても、呼び出しの種類ごとに別の道がある事は残る**、という事が分かった。
- 道はまだ 2 本(`deferred_i64` の行 18 → 18、古い照合の呼び出し 6 → 6)。問い合わせを関数ごとに 1 回にまとめる事も、まだ。F22 は残っている。
- 次の進め方は、設計の相談の答えを見てから決める(段 B を 3 つに分ける案が、報告にある)。

## お掃除9: 段0の境界

公開handoff付きの既存publication testへ、実Compareから辿る全Copy（source snapshotとoperand wrapper）を消し、全useを別のCall結果へ置換する負例を追加。削除した定義/useは0、CFG/SSA/Invoke構造は有効のまま、全件を拒否した。元srcへ置換する正しいCopy除去も、このfixtureでは現在のcontrol-producer契約により拒否する（一般のCopy除去の禁止とはしない）。
読みのCopy除去後に元InvokeをJump＋同dstのConstへ替え、CFG cacheを更新して初期化Callの効果だけを失わせる負例も拒否。同calleeの独立した二Callの結果を、元PHIのincoming一つだけへ入れ替える負例も拒否。無基点のp=PHI(p)は既存query unitを再利用しUnresolved。production変更なし、問い合わせ13＋publication3＋F22二本のfocused18 PASS。
PHIは前blockごと（仕様と(BasicBlockId,ValueId)表現）。Branchの二辺が同じ行き先でも、successorは集合化されるため表現できる。strict PHI検査も前block集合と照合し、同blockからの重複incomingを拒否するため、並行する二辺に別のPHI値は持たせられない。この回は変更しない。
全lib直列は8375 PASS/既知205 FAIL/52 ignoredで赤の名前集合一致。productionは基点と同一（639605行）、testは独立したCopy消去/効果消去/同callee別結果の公開境界の証拠を既存familyへ統合して+201行。固定5386は同一production・同一最適化CLI SHAの既存結果を再利用して差0（再測定はしない）。

### お掃除9 段1の規則

完成後gateの関数scopeで既存geometryと不変moduleを借り、構造検証だけを最初の実queryで一度確認して共有する。対象readがない関数へ新gateを足さず、ValueId/期待基点ごとの回答は保存しない。scopeを出たら破棄する。
原本/現在定義のrelation照合と省略のdefinition_countも同じ索引へ接続する。原本captured・parameter衝突・完全な命令一致・use/return_envの照合を保持し、失敗時は従来どおり全定義Vecを作って診断を変えない。省略側のuse走査とcaptured原本の探索は残す。後続のfinalized再検証の反復は、この段では残し、段2で専用走査と合わせて外す。
段1着地検証: focused41 PASS、全lib直列8375 PASS/既知205 FAIL/52 ignoredで赤の集合一致。最適化CLIの固定5386はMIR/拒否/出力/exit差0、timeout0。50回ほどのCall結果readのcompile中央値（warmup除外5回）は58.7ms→52.5ms、MIR同一。全件測定は9分47.8秒→9分50.9秒で全体の高速化は主張しない。整形後本体639605→639716、test348561→348587。後続再検証はまだread毎、段2で対応する。新しい一時警告2件（test-only importと内部contextの可視性）は抑制せず段2の中継退役で解消する。
