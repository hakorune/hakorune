# APP-boxtorrent-mini

Status: selected app; original Map-local Call/result connection incomplete. Date: 2026-10-25
Scope: 未変更boxtorrent-miniの正規EXE生成・実行・期待出力/exit。
Related: [RULES](../../../RULES.md)、[現在地](../CURRENT_STATE.toml)、[直前のmimalloc着地](mirbuilder-app-bundle-mimalloc-lite-d0-2026-09-30.md)。

## 現在地
- 明示binary/runtime: `NYASH_BIN=target/quick/hakorune NYASH_EMIT_EXE_NYRT=target/quick`。現在のV4 FFIを使用。
- canonical smokeの停止はowner67 Body3 LoopBody0、releaseFromの `store.release(ids.get(i))` における `borrowed-result/source-not-i64`。refCountの数値結果はgroundingが進んだが、EXE未生成。
- 今回の固定suiteは `run.sh --profile integration --owner-profile integration --suite real-apps-exe-boundary --timeout 420` で9/11、467.876秒、実EXE8本＋空marker1本、timeout/SKIPなし。boxtorrentとJSONがsourceで拒否し、以前の実EXEは全て再PASS。

## 接続済み
- 唯一のattributed Plain Birth providerと同じwriter inventory/非weak未注釈fieldを照合。providerのclassと保存形式を分離し、DynamicResidenceのreceiverはchecked ObjectFieldBorrowで読む。typed field、異class/multiple/hidden writerの拒否は保持。
- Storedの直接Returnは元Return planで判別して既存result ownerへ残す。独立したscalar用途は同じcomputed Return loanとgrounded calleeで昇格し、callerの推測結果型を使わない。
- 元String Coreのclosed I64 receiptはOpaqueHandle向けの式入力にも使う。同じowner/site/Arc/schemaを照合し、qualified frameと最終descriptor/ABIは別途必要。Binaryは既存signed SUM条件を保持する。
- 現在の一意なselected Instanceと同じ元sourceを持つseedを照合する。保持済みPendingをSelectedとして偽らず、位相だけで元の証拠を落とさない。
- unconditional Textの分類は同じ元Static source/ExactString契約を使い、個別call-result行があれば一致を要求する。分類だけで入力・実行・Homeは与えない。
- smokeを旧selfhost/generic JSON経由から正規emit-exeへ変更。古いLLVM関数名pinは退役し、canonical C API/no replay・全期待stdout/exitと既存String source/C negativesを保持。root digest/roundtripがreadDataのText連結を検証する。
- Local初期化のI64 probeは、同じ元owning結果loan・roots・一意なprepared targetで確認済みのobject取得を既存object経路へ渡す。SourceObjectの実行許可は発行せず、入力/Completion/Normalは後続ownerが検証する。
- 同じ元Return planの単独literal Nullだけで汎用scalar seedとI64 probeを非選択にする。callee/batch/key/arityと元targetを照合し、nested Nullやscalar証拠のErrでは判定しない。明示ScalarReturnSourceの拒否は保持。Text型・入力・実行許可は発行しない。

- 全batch/constructor-only Birthのfield-use観測を既存container ownerへ移動。Map payload classは同じwriter inventoryの唯一Map Birthと全setの同一New classから判定し、has/get/delete/keysを型だけの観測として保持。既存MapStoredResultのhas/get/set限定と借用寿命は緩めない。
- 結果式のMap取得localはEntry Receiverと別のleaf。唯一initializer・非rebind/capture・元get Arc・同じfield source Rcとpayload/数値fieldのcanonical ObjectIdを照合する。Normalクラスだけを貸し、nonnull/取得/cleanup/物理読出しの許可は別ownerに残す。

## 次の一手
1. Store.releaseが返すremainの元 `chunk.release()` receiverを接続する。chunkは同じMap.get localだが、現claim_local_classは旧MapStoredResult rootsなしでは元targetを作れない。共有payload証拠と元get取得を保持した既存lexical receiver ownerへ結ぶ。
2. 型名だけの `(class, None)` でborrowed interval/物理supportを迂回しない。local Map-getをme Receiverに偽装せず、元slotの破壊/置換/alias、null、Normal/Fault cleanupを最終consumerまで保持する。
3. 正規EXE/runまで同じアプリを追う。JSONと局所/共有退役は未完のまま保持する。

## 検証と未完条件
- 今回CLI `cargo quick-build-fast`最終37.06秒、`cargo quick-test-fast --no-run`最終61秒PASS（共有変更直後は92秒/147秒）。既存source outcomes3/scalar pending7/object返却5/stored receiver6は全PASS。smokeは上記新停止を確認。新規test/fixture/checker/for_testなし、既存testは署名への引数1か所だけ追随。
- Textの繰返しHome再観測テスト1件は `argument-frame-current-drift` で赤。同日の作業前既存test binaryでも同じ行/拒否を再現したが、対応revisionは未確認なので未完のまま保持し、現在のgreenとは数えない。
- 結果昇格の挙動不変なowner移動は別commit。新moduleを増やさず、profile817行を800行未満へ戻した。テストで検出した移動時import欠落も訂正済み。
- 一時診断コード/adapterは追加していない。失敗した直接EXE診断の出力は不要になれば廃棄する。詳細はこのcardのGit履歴へ集約する。
- JSON owner22 `borrowed-entry/source-use-completion-pending`、他のsource/publication/Loopの赤、局所/共有R6/R7・最終pipeline条件は未完。mimalloc benchmark promotionはparkedのまま。
