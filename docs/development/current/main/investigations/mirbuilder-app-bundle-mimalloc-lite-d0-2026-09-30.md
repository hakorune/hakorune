# APP-mimalloc-lite

Status: closed; standalone and fixed-suite smoke PASS (2026-10-19).
Date: 2026-10-19
Scope: 未変更mimalloc-liteの正規EXE・実行・期待出力/exitと必要なprofile境界。
Related: [RULES](../../../RULES.md)、[現在地](../CURRENT_STATE.toml)、
[全体完了条件](../workstreams/mirbuilder-inplace-replacement-current.md)。

## 現在地

- `APP-mimalloc-lite`は着地。単体smoke約163秒、固定suite内約147秒でPASS。
- 固定suiteは6PASS/5FAIL。PASS内訳は実EXE5本と空marker1本、SKIP/timeoutなし。
- 変更前はgeneric JSON precheckが`required-integer-source-unproved`で停止した。
  EXEはClosedAppExecutable、generic JSONは未使用importも含むAllSourceという違いがあった。

## 修正した責務

- generic JSONは既にStatic I64 Invokeを受理する。旧「全Invoke拒否」の前提は不一致。
- アプリsmokeから古いserializer precheckを削除。V4・LLVM C API・no replay・EXE存在・全出力/exit確認は維持。
- その境界は既存source-publication testに統合：Static I64の実JSON出力成功と、object lifecycleのgeneric profile拒否。
  既存sourceケースを使い、test名・入力ケース・file・hook・checkerを追加しない。
- build timeout300秒とrun timeout120秒を分離。以前のbuild/run共通120秒は実測のEXE生成時間より短かった。
- Integer証拠、AllSource受理範囲、production Rust、アプリsourceは変更しない。

## 検証と次の一手

- `cargo quick-test-fast --no-run --message-format=json`成功。
- 既存`unannotated_borrowed_i64_result_publishes_call_forms`と
  `static_direct_return_records_one_root_invoke`は各1件PASS。test名一覧9263件不変。
- `bash tools/smokes/v2/profiles/integration/apps/mimalloc_lite_exe.sh`はPASS。
  source一致の既存quick CLIを再利用し、HAKO_BIN/NYASH_BINを同じ絶対pathへ設定。
  lifecycle runtimeとC API FFIも明示。変更Rustはcfg(test)内だけでCLI再buildは不要。
- pointer・builder layer・CallTarget guard、shell syntax、diff checkはPASS。
- `bash tools/smokes/v2/run.sh --profile integration --owner-profile integration --suite real-apps-exe-boundary --timeout 600`
  はsource一致のquick CLI、既存lifecycle runtime/C APIで完了。全体終了値1、約579秒。
  対象revisionは`751dae566b`。赤はuntyped-field、binary-trees、boxtorrent、allocator、JSON。
- untyped-fieldはユーザー判断で既存D0のParkedSealedを維持。次はallocator-stress。

## 保持する未完条件

- suite成功と全MirBuilder/Gate1完了は別。ObjectForward/field Birth/LOCAL初期化/receiver/cleanup/publicationの未完契約は維持。
- 共有の不足は現在アプリに必要なownerで直し、置換済み専有edge/adapterは証明成立時に同じseriesで削除する。
- [JSON/Textの既存契約](mirbuilder-gate1-static-text-input-s0-2026-10-17.md)は未受理のまま後続へ保持。
  未接続Return準備は取り下げ済みで、保留branchはない。
- 過去の完全な契約・履歴はこのpathの`0311400375`時点のGit履歴に保存済み。
