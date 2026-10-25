# APP-mimalloc-lite

Status: selected app; reopened after measured source regression. Date: 2026-10-25
Scope: 未変更mimalloc-liteの正規EXE生成・実行・期待出力/exit。benchmark promotionは再開しない。
Related: [RULES](../../../RULES.md)、[現在地](../CURRENT_STATE.toml)、[直前のbinary-trees着地](app-binary-trees.md)。

## 現在地
- 明示binary/runtime: `NYASH_BIN=target/quick/hakorune NYASH_EMIT_EXE_NYRT=target/quick`。
- 最新固定suiteは7/11、354.429秒、実EXE6本＋空marker1本。binary-treesは47.638秒で再PASS。mimalloc/allocatorは外側120秒timeout、JSON/boxtorrentはsource拒否。timeoutをEXE証拠にしない。
- 単独mimallocは元owner37 Body5のforward-source-identityとBirth ABIを越え、C `published-lifecycle-physical-parser/function-body`、function12 `HakoAllocHeap.allocateResult/1`で停止。EXE未生成。
- 2026-10-19のPASSは過去の結果。古いAllSource JSON precheckの撤去でsmokeを整えた着地であり、その当時から動いたEXE自体を新能力に数えない。現在の赤を歴史的PASSで閉じない。

## 必要な接続
- 元のforward claimのcall/target/ordinal/site/bindingから既存flow.readをSUMグラフへ登録済み。guardがScalar seedを作っても同じEntry/Join/Seedのpoint証明を失わず、未知のpointは拒否する。SUMをsite読みの代用にしない。
- flow.readがある場合はread＋origin＋point SUM signed-onlyが全て必要。不一致時にformal SUMへfallbackしない。サイト読みが無い場合も、登録済みformalのcorroborate済み閉SUMだけではsite対応を捏造しない。
- completed Usesの元NewArgument行（new site/ordinal/read site/binding/origin/formal）とsigned domainを既存Birth lenderへ接続済み。独立したentry/alias値・最終Birth全命令・guard支配/用途数検査は保持。
- guarded callのcaller-entry root gate、支配関係・guard対応・命令順序と、独立した最終argument/receiver/cleanup検証を保持する。
- 次の接続: allocateResultのentryは呼び元なしのtagged、guard後のallocate呼出もtagged actualだが受け側はinteger_view。発行JSONと既存C validate-only APIでparser拒否を再現済み。entry全体をsignedと偽らず、元per-use guardのchecked整数値を同じCall operand/ordinalとtransportへ結ぶ。
- 共通ownerの範囲で直し、同じ未変更アプリの正規EXE/runまで追う。別アプリを並行実装せず、未知の停止点は同じcardへ短く更新する。

## 検証と次の一手
1. 既存調査とsource ownerを再利用し、guard後の同じ引数の意味・値・ABIを最終consumerへ結ぶ。新しい台帳・checker・test file・for_test hookは作らない。
2. `bash tools/smokes/v2/profiles/integration/apps/mimalloc_lite_exe.sh`を同じ明示binary/runtimeで実行。sourceコンパイル、C/LLVM、リンク、実行、出力/exitを区別する。
3. 必要な既存focused検証を再利用し、着地時に固定suiteを再測定する。内部green、SKIP、markerをEXE証拠へ数えない。
- `cargo quick-build-fast`40.30秒、`cargo quick-test-fast --no-run`68秒PASS。既存new_argument_testsは6PASS/4FAILから9PASS/1FAIL、最終borrowed_usesは18/18PASS。テストの追加・変更なし。
- 残るnew_argument negativeは従前と同じ早期refusal/期待診断不一致。domain1PASS/1FAIL、guard1PASS/2FAIL、input12PASS/1FAIL、solver0PASS/1FAILも以前のログと同じ箇所/理由の赤として未完。全体greenとは扱わない。
- 一時診断コードは撤去済み。発行JSONは次のparser/Call照合まで保持し、その接続解消時に廃棄する。検証詳細はこのcardのGit履歴へ集約する。

## 保持する未完条件
- JSON: owner22 `borrowed-entry/source-use-completion-pending`。boxtorrent: owner51 `stored-child/receiver-unavailable`。allocator: 最新suiteは120秒timeout、先のsource停止を最新結果へ代用しない。
- 未完のsource/publication・Loop検証、局所adapter退役と共有R6/R7・最終pipeline条件は保持。詳しい直前の実測はbinary-trees cardとそのGit履歴へ集約する。
- historical generic JSON/EXE profile境界は既存publication検証がowner。拒否を現在の失敗へ差し替えず、source・期待出力・negativeを回避的に書き換えない。
- 過去の契約・着地履歴はこのcardのGit履歴へ圧縮済み。有限製品goalは未完。
