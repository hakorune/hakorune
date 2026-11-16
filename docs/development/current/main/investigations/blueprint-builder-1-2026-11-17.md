# 比較の実装1: i64/Bool builderの結果

Date: 2026-11-17
Status: 比較結果を記録。既定への切替・旧builderの退役は行っていない。
設計の正本: [MirBuilderの設計図](../design/mirbuilder-blueprint.md)（mainの21/22節も適用）。

## 通った範囲と入口

明示したi64/Boolの引数・戻り型、推論local、代入・複合代入、if、条件/無条件/範囲loop、break/continue/return、算術・比較・短絡、free/static me/Other呼び出し、i64 printを接続した。native entryは引数なしのi64 main。子の式は同じ再帰入口を使う。

選択は実行前に `HAKO_MIR_BUILDER=blueprint`（比較実装）または `current`（既存）で行う。未設定は既存。拒否後の再試行はしない。

手で期待出力/exitを決めた51入力は、すべて正規EXEで一致した。ただし当時のBool実引数はliteralとformalだけで、計算済みBoolを引数へ渡す範囲は未確認だった（1bで監査・修正）。内側callの結果を引数に渡す形、3種類のcallの混在、if/loop越えの読み、入れ子loop、break/continue、短絡の非実行と評価順、再帰/相互再帰、Bool carrier、shadow、range境界の一度評価、wrapping演算を含む。

| 比較 | 件数 |
|---|---:|
| 両方通って同じ出力・exit | 8 |
| 新実装のみ通る | 43 |
| 両方通って異なる / 旧実装のみ / 両方拒否 | 0 / 0 / 0 |

## 構造の監査

| 項目 | 実測 |
|---|---:|
| 式 / call / loopを下ろす入口 | 1 / 1 / 1 |
| Lowerの子kindの組合せ・深さ・場所・call form・app/fileによる専用経路 | 0 |
| source IDの型（宣言・束縛・node） | 3 |
| 同じ対象の段階別再ID化・別語彙 | 0 |
| 設計図にない部品 | 0 |
| `>=`追加で変更したproduction file / 関数 | 3 / 3 |

`>=`は型規則・MIR投影・独立照合だけを別commitで変更し、新しい入口や型を増やしていない。追加前は名前付き拒否、追加後は元入力のEXEがexit6。

「全段で子kind検査が0」とは主張しない。Resolveには既存のoriginal-positive-literal Div安全契約1件と、literal trueからloopの正常出口を確定する制御契約1件がある。一般Div/%の意味は発明しない。

| 責務 | 整形後production行 | struct/enum/type定義数 |
|---|---:|---:|
| 共通宣言・Resolved model | 165 | 18 |
| Resolve | 737 | 2 |
| LowerとSSA作業場 | 814 | 9 |
| Seal | 18 | 0 |
| source対応検証c | 750 | 4 |
| module公開 | 43 | 0 |
| 合計 | 2,527 | 33 |

testとcfg(test)登録は除外。Lowerは責務で3ファイルに分かれ、production全ファイルは800行未満。AST operatorは既存語彙を共有する。

## 検証と共有境界

Resolveが名前・型・制御の意味を確定し、物理IDは発行しない。Lowerは一つのBindingId×block表でSSA/Phiを作る。旧作業場は家族の計画に依存するため再利用していない。

cの期待はResolvedFunctionから読み、Lowerの記録からは座標と実値/辺の識別子だけを得る。完成MIRの操作・operand・callee・順・条件・辺・Phiを独立照合する。指定8種類のMIR改変はすべて拒否した（7種類は構造検証を通るMIRのまま）。既存strict Document検証でCFG/SSA/支配/Phiも検査する。

CLI負例15入力はすべて非zero exit・MIR成果物なし。12入力はResolve、printの式位置とtake/fromの3入力は既存parserで拒否。

共有backendの不具合も3ファイルで修正した。物理ValueId 0を未指定operandとして予約し、generic i64 ABIのBool入力をi1へ復元し、共有比較ではBoolを0/1のi64へ投影する。単項Minusはoperand一度評価→Const0→wrapping Subとし、cが展開を検査する。BoolをMIR上でIntegerへ偽装せず、新opcode/JSON/公開ABIは足していない。

既定経路の固定5,386入力はMIR・拒否・出力・exitの差0、timeout0。全lib直列は8,378 PASS・205 FAIL・52 ignoredで、FAIL名は基準と完全一致。固定suite8/8、Map18、既存C document検証24ケース、pointer/diff確認を実施した。featureのlight/optional構成は成功、vm-referenceとpluginsなしCLIは既存のビルドエラーが不変（成功とは数えない）。

一方、新実装の固定5,386入力での受理は0。署名不足2,507、範囲外宣言2,539、別の型73、別契約1、parser拒否266。旧実装で実行できる1,483入力も含む。新実装が既存corpusを動かせたとは主張せず、手書き51入力の結果と分ける。

## owner guardの未達と次の判断

基点35024f399dの見張りは赤。ユーザー決定により基準は変更せず、比較結果として残した。

- raw BinOp 44→47: 二項演算、単項Minus展開、range増分。
- raw Compare 41→43: 二項比較、range条件。
- raw UnaryOp 15→16: Not。
- definition-scan候補286→288: 既知headerのPhi確定と、既知blockへのPhi追加。後者は定義探索ではないが構文候補に該当。

増分はすべて新directory内で、旧writerは不変。見張りをPASSとは報告しない。

series全体はproduction純増2,632行、test純増280行。旧コード削減は0。別実装なのでsrc/mir全体の縮小やv1予算の達成とは数えない。scalar専用moduleなら旧builder/package/loop familyを通らずに済むが、混合型moduleのcallerは残るため、現時点で物理削除可能と証明した旧ファイルは0。

Box・所有・Fault・入れ物は次の比較の範囲。これらを通す前に、全面切替・速度の一般化・大規模退役の判断はしない。

## 比較の実装1b: scalar ABIと組合せ表

監査で計算済みBoolの実引数とrootのBool比較に不足が見つかった。scalar ABIの論理Bool判定、i64参照への投影、i1への復元を既存backendの一つのownerへ集約し、call実引数・比較・返却・formal/call結果の復元が利用する。handleもi64なので、物理i64だけではscalarと判定しない。既存Mapのkind/payloadは維持し、original scalar laneだけ同じownerへ寄せる。公開ABI/MIR/JSONは不変。

7監査入力の期待exitはsourceから再計算して61/111/42/35/2/2/7、出力は空。修正後に全一致、前回51入力とCLI負例15入力も確認した。

表はBool9カテゴリ/38variantとi64の7カテゴリ/52variantを、33 consumer列・関数内/entryの2文脈へ掛けた。coarse 1,056升、variant全5,940升。

| 分類 | 升数 | この段の結果 |
|---|---:|---|
| 生成した正のEXE | 3,580 | 全て期待一致 |
| 型/Div契約の負例 | 2,126 | 全て予定した名前付き拒否 |
| native entryの引数/戻り型により構造上作れない升 | 234 | 理由つきNAとして保持 |

値を作る側はliteral、比較、Not、And/Or、call、formal、if越えlocal、loop carrier、i64算術/符号反転。使う側は条件・実引数・return・比較の両側・Not・短絡の両側・local初期化・代入・算術の両側・range両端。source emitterとは別の小さいAST評価器で、wrapping i64、負数除算のゼロ方向丸め、短絡、左からの実行順とprintを期待として計算した。旧builderは期待値に使わない。

この表は1bのABI段で確認したもの。PHI変更後と最終の既定経路比較は、続く検証で区別して記録する。
