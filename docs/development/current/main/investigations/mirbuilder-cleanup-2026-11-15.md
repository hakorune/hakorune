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
