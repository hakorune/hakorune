# APP-mimalloc-lite

Status: landed EXE acceptance. Date: 2026-10-25
Scope: 未変更mimalloc-liteの正規EXE生成・実行・期待出力/exit。benchmark promotionはparkedのまま。
Related: [RULES](../../../RULES.md)、[現在地](../CURRENT_STATE.toml)、[次のboxtorrent](app-boxtorrent-mini.md)。

## 着地
- 明示binary/runtime: `NYASH_BIN=target/quick/hakorune NYASH_EMIT_EXE_NYRT=target/quick`。同じ現在のC FFIを使用。
- 未変更mimallocの正規LLVM C API/pure-first EXE/run/期待stdout/exit0 PASS。固定suite内155.983秒。free-list再利用と会計を既存smokeで確認し、compat replayなし。
- 固定suite `run.sh --profile integration --owner-profile integration --suite real-apps-exe-boundary --timeout 420` は9/11、459.647秒、実EXE8本＋空marker1本、timeout/SKIPなし。allocator-stressも192.662秒で実EXE/run PASS。
- 残る赤はboxtorrent owner51 `stored-child/receiver-unavailable`とJSON owner22 `borrowed-entry/source-use-completion-pending`。次はAPP-boxtorrent-mini。

## 同じsourceから最終consumerへの接続
- 元forward claimのcall/target/ordinal/site/bindingからflow.readをSUMグラフへ保持。Scalar guardでpoint証明を失わず、不一致時にformal SUMへfallbackしない。
- Usesの元NewArgument行・signed domain・独立したentry/alias値・最終Birthを照合する。完了したsigned entryのactualはgeneric受け側でもtag1、混合guarded値は元のpairを保持する。
- opaque borrowed pairからinteger_viewへのCallは元tagを保持し、直前のkind1検査で失敗をcaller自身のMIR Fault cleanupへ送る。callee入口検査、元guard/支配・最終用途数検証も保持。
- usize writeは同じborrowed formal/Copyのchecked Integer viewも使える。baseの正確なclass、slot、予約領域、符号/範囲、Normal/Fault義務は保持。任意のI64計算結果へ拡張しない。

## 検証と片付け
- CLI `cargo quick-build-fast`43.11秒、C FFI build PASS。既存new_argument_testsは9PASS/1FAIL、最終borrowed_uses18/18PASS。残るnegativeは従前と同じ早期refusal/期待診断不一致で未完。
- 既存C borrowed-ingress一式PASS。`CC='cc -fuse-ld=lld'`と同じquick runtimeを使用。独立したCallタグ失敗のcaller cleanupとprojected usize正/負範囲を守るため、既存ケースへ4実行と既存拒否変異4検査を追加。既存の型偽装/class/nullable/所有権negativeは保持。
- positiveのBirth表現期待は、元Integer callerのentry viewとの一致を検証する形に更新。新しいtest file/関数/for_test/checker/adapterはなし。
- 一時診断コードと取得JSON/OBJ/EXEは撤去済み。詳細はこのcardのGit履歴と既存smoke/testの再実行へ集約する。

## 保持する未完条件
- source domain1PASS/1FAIL、guard1PASS/2FAIL、input12PASS/1FAIL、solver0PASS/1FAILの既知赤と、他のsource/publication/Loop検証は未完。全体greenとは扱わない。
- 局所adapter退役・共有R6/R7・最終pipelineは別途必須。現在の9/11やmimallocの着地だけで有限製品goalを閉じない。
- 歴史的PASSとgeneric JSON/EXE境界は過去の記録。今回の未変更sourceからの実EXE証拠で置き換え、benchmark promotionを自動再開しない。
