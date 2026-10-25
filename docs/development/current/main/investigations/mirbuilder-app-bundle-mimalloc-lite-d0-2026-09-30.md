# APP-mimalloc-lite

Status: selected app; reopened after measured source regression. Date: 2026-10-25
Scope: 未変更mimalloc-liteの正規EXE生成・実行・期待出力/exit。benchmark promotionは再開しない。
Related: [RULES](../../../RULES.md)、[現在地](../CURRENT_STATE.toml)、[直前のbinary-trees着地](app-binary-trees.md)。

## 現在地
- 明示binary/runtime: `NYASH_BIN=target/quick/hakorune NYASH_EMIT_EXE_NYRT=target/quick`。
- 最新固定suiteは7/11、122.454秒、実EXE6本＋空marker1本、SKIP/timeoutなし。binary-treesがEXE/run/output/exit0まで着地した。
- mimallocの現在のsource停止は `ordinary-new/borrowed-object/forward-source-identity`、owner37 Body5。EXE未生成。allocator-stressも同じ不足を踏む。
- 2026-10-19のPASSは過去の結果。古いAllSource JSON precheckの撤去でsmokeを整えた着地であり、その当時から動いたEXE自体を新能力に数えない。現在の赤を歴史的PASSで閉じない。

## 必要な接続
- 元のentry formal、同じargument candidate/read site、binding/root、元source identityをseed側で保持し、借用entry値の証拠へ接続する。SUMは型を供給し、site読みの代用にしない。
- flow.readがある場合はread＋origin＋point SUM signed-onlyが全て必要。不一致時にformal SUMへfallbackしない。サイト読みが無い場合も、登録済みformalのcorroborate済み閉SUMだけではsite対応を捏造しない。
- guarded callのcaller-entry root gate、支配関係・guard対応・命令順序と、独立した最終argument/receiver/cleanup検証を保持する。
- 共通ownerの範囲で直し、同じ未変更アプリの正規EXE/runまで追う。別アプリを並行実装せず、未知の停止点は同じcardへ短く更新する。

## 検証と次の一手
1. 既存調査とsource ownerを再利用し、entry formalの元サイト読みをseedから最終consumerへ結ぶ。新しい台帳・checker・test file・for_test hookは作らない。
2. `bash tools/smokes/v2/profiles/integration/apps/mimalloc_lite_exe.sh`を同じ明示binary/runtimeで実行。sourceコンパイル、C/LLVM、リンク、実行、出力/exitを区別する。
3. 必要な既存focused検証を再利用し、着地時に固定suiteを再測定する。内部green、SKIP、markerをEXE証拠へ数えない。

## 保持する未完条件
- JSON: owner22 `borrowed-entry/source-use-completion-pending`。boxtorrent: owner51 `stored-child/receiver-unavailable`。allocator: owner37 Body5の同じforward-source-identity。
- 未完のsource/publication・Loop検証、局所adapter退役と共有R6/R7・最終pipeline条件は保持。詳しい直前の実測はbinary-trees cardとそのGit履歴へ集約する。
- historical generic JSON/EXE profile境界は既存publication検証がowner。拒否を現在の失敗へ差し替えず、source・期待出力・negativeを回避的に書き換えない。
- 過去の契約・着地履歴はこのcardのGit履歴へ圧縮済み。有限製品goalは未完。
