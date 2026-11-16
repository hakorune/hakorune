# 比較の実装1: 小さい独立builder

設計の正本は `../design/mirbuilder-blueprint.md`。既定builderは変更せず、明示選択時だけ別のmoduleを原子的に作る。既存の入口が拒否した後で切り替えない。

## 実装の規則

- 宣言と署名はDeclarationTable、sourceの意味と拒否はResolveだけが所有する。ResolvedFunctionは構造のある型付き本体で、物理ValueId/blockを持たない。
- Lowerは一つの式入口・一つのCall入口・一つのloop入口と、一つのSSA作業場で契約を実現する。書かれた式の順を保ち、場所別の道を作らない。
- scope、break/continue/returnの行き先はResolveで固定する。範囲loopは境界を入口で各一度評価し、半開区間、step+1、continue→step、indexは書換え不可。
- この比較のi64実行は現行Integer laneの契約を使う（types.mdの型名/storage lockとdynamic-operators.mdのcurrent-I64）。Add/Sub/Mulはwrapping。未接続のexact numeric checked演算を型名から推測して選ばない。
- 一般Div/Moduloの異常条件には現行のsource契約が不足しているため、新しい規則を暗黙に作らない。原本の正整数literal除算以外は、この回で契約を発明して通さない。
- Verify cの期待はResolvedFunctionから読む。Lowerの対応記録は座標と実値/辺だけで、期待演算・callee・順の自己申告を持たせない。余分な操作・落ちた操作・重複・条件外実行を完成MIRから拒否する。
- 構造検証は既存のstrict Document policyを利用する。これは旧sealed Integer-only corridorを主張しない入口で、CFG/SSA/支配/PHI/Invokeの検査を保つ。新しい対応検証が正確なi64/Bool署名・実引数・結果を検査する。
- 旧SSA作業場は旧MirBuilder/家族の証拠に依存するため流用せず、設計図にある小さいsealed-block SSAを比較builder内に置く。
- backendは完成MIRから既存のcall-route/definition metadataだけを投影し、既存canonical JSONと汎用C backendへ渡す。全semantic_refreshや家族ごとのsource preflightを呼ばない。BoolをIntegerへ偽装しない。

## 現在の範囲

最初の縦の入力は型注釈付きの `return 1 + 2`。宣言/Resolveの段から接続を始める。後続の受理範囲と結果は、実行確認後にここを書き換える。

最初の縦の確認: `HAKO_MIR_BUILDER=blueprint --backend mir --emit-exe` で型注釈付きMain.mainの `return 1 + 2` がEXEまで到達し、exit 3。既存builderでも同じ結果。runtimeの既存表示を `NYASH_NYRT_SILENT_RESULT=1`、`HAKO_NYRT_PLUGIN_HOST=off` で静め、sourceの期待出力（空）も一致した。最初に観測したruntimeのResult表示とplugin警告は、プログラムのprintとは分ける。

選択子は `HAKO_MIR_BUILDER=blueprint`（新経路）と `current`（既存経路）、未設定は既存経路。選択はsource preparationより前に一度だけ行い、既存経路を試してから切り替えない。backend用の二つのmetadata射影とcanonical JSON writerは既存処理を使い、旧builderそのものは変更しない。

## 共通のscalar制御と対応検証

Resolveが型付き束縛、3種類の静的呼び出し、複合代入、if、条件/無条件/範囲loop、break/continue、短絡を同じ関数の契約へ解決する。sourceのoperator語彙は共通ASTのものをそのまま保持し、後段専用の別名へ詰め替えない。SSA作業場はBindingId×blockの1表、可視scopeに生存する全BindingId（shadowされた外側も含む）を合流させる。

対応検証はResolvedFunctionから期待を読み、完成MIRの実cursorを歩いて全命令を一度ずつ確認する。ifの両枝、短絡のRHS/skip、loopの初期/戻り/continue/step/breakを分け、Phiは実際の終端blockとsourceから導いた値の全incomingを照合する。単純な命令数の一致では通さない。

focused確認: 正負のResolve、指定8種類のMIR改変拒否、入れ子control/Phiとshadowed outer bindingの保持を確認（3test、全PASS）。8改変のうち7種類は既存の構造検証も通るMIRのまま、対応検証が拒否。loop戻りの付替えは対応検証を直接呼んで拒否を確認した。EXEの広い比較はこの後に行う。

## EXE比較で見つかった物理境界

既存C backendはValueId 0を未指定operandとして扱うため、新builderの物理IDは全関数で1から割り当てる。sourceの束縛ordinalは0始まりのまま。引数番号と物理IDは同一視しない。この修正後、手計算47入力中43入力でEXEの出力とexitが一致し、fib/引数返却の誤動作は解消。残り4入力はBool formalのi64→i1 ABI変換不足と、generic backendがUnary Negを受けない点。まだ全範囲の成功とは数えない。focusedの3test（8種類の改変拒否を含む）は全PASS。

修正後の47入力は、すべてEXEの期待出力/exitに一致（既存builderと同結果7、新builderのみ40）。単項Minusはoperand一度評価→Const0→wrapping Subとし、対応検証は両命令とoperand/順を独立照合する。generic CのBool formalは既存i64 ABIを維持し、関数入口でi1に復元する（戻り値側の既存変換と対）。新opcode/JSON/ABI追加なし。既定経路の全比較は最終確認時に実施する。負のCLI入力15本はすべて非zero exit/成果物なし。うちprintを式位置に置く例とtake/fromの3本は共通parserで拒否、残り12本はResolveの名前付き拒否。

設計図mainの21/22節（2026-11-17追記）も適用する。Lowerの子の組合せ/深さ/場所/call form/app名による分岐は0、式/call/loop入口は各1。Resolveの既存Div literal安全契約1件とliteral trueの継続契約1件は、別に計上する。新しい専用経路とは数えず、全段で子kind検査0とは主張しない。

## 演算子1種類を追加する費用

`>=`を別commitで追加。productionはResolve::binary_type、Workspace::binary、Checker::binaryの3ファイル/3関数だけで、入口/型/専用経路の増加なし。追加前はUnsupportedOperator、追加後は元の入力がEXE exit6。focused3test PASS。追加で試したBool同士の比較は既存Cの共有icmpがi64固定だったため拒否され、51入力中50一致/1未到達。これを通ったことには数えず、別の共通backend境界修正で閉じる。
