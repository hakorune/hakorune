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
9. **順は、入口を閉じる → 専用の道を消す → 余った下の部品を消す。** 1 回の大きさは、行数ではなく、独立して戻せる単位。消して不要になった専用の依存は、続けて消す(`cfg`・別の directory・stub へ移して残さない)。**「消した後に、残った入力だけで測り直して、さらに削る」は、しない。** 残す入力と契約は、先に固定する。減らす時は、それ自体を退役の判断として扱う。
10. **やってはいけない事:** test を通すために、拒否や検証を弱める事(独立の検証を通さない、古い道へ retry する、panic を全部「未対応」として扱う、期待する診断を、出た別の error へ差し替える)。提供する機能は減らしてよい。提供すると言った機能の保証は、薄めない。
11. **退役と、構造の磨き込みは、commit を分ける。** この掃除では、型の統合や責務の置き直しを、一緒にしない。
12. **試作((ウ))は、「次の selfhost の壁を越えるために、どの現役の使い手へつなぎ、何が通るようになり、代わりに何を退役させるか」を一言で言えなければ、実装を消す。** 目的、採らなかった理由、もう 1 度考える条件、参照する commit を、この文書に短く残す。

