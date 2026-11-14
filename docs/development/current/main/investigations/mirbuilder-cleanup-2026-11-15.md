# MirBuilder整理1: 実行経路の計測と未使用コードの整理

Status: 段1計測済み、段2未着手。Date: 2026-11-15。基点main `9e3f19026e`（productionは`7a6771e805`と同じ）。

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

上記6,114回に、EXE群の補足dump3,760回を追加。sourceは5,386本（群重複あり）。
固定suiteは8/8、timeout/SKIPなし。全libは8,913 PASS/225 FAIL/56 ignored（4thread）。
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
