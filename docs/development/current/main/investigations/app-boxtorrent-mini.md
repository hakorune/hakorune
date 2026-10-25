# APP-boxtorrent-mini

Status: selected app; source input closure incomplete. Date: 2026-10-25
Scope: 未変更boxtorrent-miniの正規EXE生成・実行・期待出力/exit。
Related: [RULES](../../../RULES.md)、[現在地](../CURRENT_STATE.toml)、[直前のmimalloc着地](mirbuilder-app-bundle-mimalloc-lite-d0-2026-09-30.md)。

## 現在地
- 明示binary/runtime: `NYASH_BIN=target/quick/hakorune NYASH_EMIT_EXE_NYRT=target/quick`。現在のV4 FFIを使用。
- canonical smokeの停止はowner66 Body4 LoopBody1 Initializerの `borrowed-result/source-not-i64`。owner51の早すぎる実行引数要求は解消したが、EXE未生成。次は元sourceで結果の用途を確認する。
- 直前commitまでの固定suiteは `run.sh --profile integration --owner-profile integration --suite real-apps-exe-boundary --timeout 420` で9/11、468.515秒、実EXE8本＋空marker1本、timeout/SKIPなし。boxtorrentとJSONがsourceで拒否し、以前の実EXEは全て再PASS。

## 接続済み
- 唯一のattributed Plain Birth providerと同じwriter inventory/非weak未注釈fieldを照合。providerのclassと保存形式を分離し、DynamicResidenceのreceiverはchecked ObjectFieldBorrowで読む。typed field、異class/multiple/hidden writerの拒否は保持。
- Storedの直接Returnは元Return planで判別して既存result ownerへ残す。独立したscalar用途は同じcomputed Return loanとgrounded calleeで昇格し、callerの推測結果型を使わない。
- 元String Coreのclosed I64 receiptはOpaqueHandle向けの式入力にも使う。同じowner/site/Arc/schemaを照合し、qualified frameと最終descriptor/ABIは別途必要。Binaryは既存signed SUM条件を保持する。
- 現在の一意なselected Instanceと同じ元sourceを持つseedを照合する。保持済みPendingをSelectedとして偽らず、位相だけで元の証拠を落とさない。
- unconditional Textの分類は同じ元Static source/ExactString契約を使い、個別call-result行があれば一致を要求する。分類だけで入力・実行・Homeは与えない。
- smokeを旧selfhost/generic JSON経由から正規emit-exeへ変更。古いLLVM関数名pinは退役し、canonical C API/no replay・全期待stdout/exitと既存String source/C negativesを保持。root digest/roundtripがreadDataのText連結を検証する。

- Local初期化のI64 probeは、同じ元owning結果loan・roots・一意なprepared targetで確認済みのobject取得を既存object経路へ渡す。SourceObjectの実行許可は発行せず、入力/Completion/Normalは後続ownerが検証する。

## 次の一手
1. owner66 Loop内初期化の元call/結果sourceを照合し、I64 probeとText/nullable等の結果用途を既存authorityで分ける。拒否の捕捉や非I64エラーの無視で経路を選ばない。
2. 以前のSourceObject停止はfinisher前のHome source probeで発生していた。exact_rowsの探索漏れではなく、探索範囲は変更しない。入力の全incoming・signature・Completionは引き続き必要。
3. 正規EXE/runまで同じアプリを追う。JSONと局所/共有退役は未完のまま保持する。

## 検証と未完条件
- 今回CLI `cargo quick-build-fast`38.87秒、`cargo quick-test-fast --no-run`49.39秒PASS。既存owning入力境界2/object返却5/scalar pending7は全PASS。smokeは上記新停止を確認。固定suiteは今回未再計測。test/fixture/checker/for_test追加なし。
- Textの繰返しHome再観測テスト1件は `argument-frame-current-drift` で赤。同日の作業前既存test binaryでも同じ行/拒否を再現したが、対応revisionは未確認なので未完のまま保持し、現在のgreenとは数えない。
- 結果昇格の挙動不変なowner移動は別commit。新moduleを増やさず、profile817行を800行未満へ戻した。テストで検出した移動時import欠落も訂正済み。
- 一時診断コード/adapterは追加していない。失敗した直接EXE診断の出力は不要になれば廃棄する。詳細はこのcardのGit履歴へ集約する。
- JSON owner22 `borrowed-entry/source-use-completion-pending`、他のsource/publication/Loopの赤、局所/共有R6/R7・最終pipeline条件は未完。mimalloc benchmark promotionはparkedのまま。
