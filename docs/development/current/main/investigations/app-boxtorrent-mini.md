# APP-boxtorrent-mini

Status: selected app. Date: 2026-10-25
Scope: 未変更boxtorrent-miniの正規EXE生成・実行・期待出力/exit。
Related: [RULES](../../../RULES.md)、[現在地](../CURRENT_STATE.toml)、[直前のmimalloc着地](mirbuilder-app-bundle-mimalloc-lite-d0-2026-09-30.md)。

## 現在地
- 明示binary/runtime: `NYASH_BIN=target/quick/hakorune NYASH_EMIT_EXE_NYRT=target/quick`。FFIは同じ着地済みV4 ownerを使う。
- 固定suite内の現在の停止はsource側owner51 `ordinary-new/stored-child/receiver-unavailable`。EXE未生成。
- mimalloc-liteとallocator-stressは共有のsource/ABI接続で実EXE/run/output/exitまでPASS。これをboxtorrentの受理証拠へ転用しない。

## 次の接続
1. 同じsmoke `bash tools/smokes/v2/profiles/integration/apps/boxtorrent_mini_exe.sh` と元source行を照合する。
2. 既存read-only調査とworkerを使い、元のparent/field/childと同じreceiver読みをsource ownerからCallまで結ぶ。文字列名やMIR型からclass/所有権を推測しない。
3. 正規EXE/runまで追い、必要な既存focused検証を再利用する。最終receiver/argument/Normal/Fault cleanup検査を保持する。

## 保持する条件
- `.hako`変更、AST rewrite、失敗後retry、silent fallbackを使わない。新しいchecker・test file・for_test hookや内部責務cardを増やさない。
- JSONのowner22 `borrowed-entry/source-use-completion-pending`は別アプリの未完として保持する。
- 既知のsource/publication/Loop検証の赤、局所adapter退役と共有R6/R7・最終pipeline条件は未完。固定suite成功だけで有限製品goalを閉じない。
- mimallocのbenchmark promotionはparkedのまま。これまでの詳細は直前app cardのGit履歴へ集約する。
