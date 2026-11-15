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

