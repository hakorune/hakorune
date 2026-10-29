# APP-boxtorrent-mini

Status: **acceptance reached** (fixed suite 11/11). Date: 2026-10-30
Scope: unchanged boxtorrent-mini, canonical EXE, expected output and exit.
Refs: [RULES](../../../RULES.md), [CURRENT_STATE](../CURRENT_STATE.toml), [Home contract](../../../../reference/language/ownership.md), [Map contract](../../../../reference/runtime/runtime-data-dispatch.md).

## 現在地
- `boxtorrent_mini_exe` が PASS（期待出力13行＋`Result: 0`、exit 0）。固定 `real-apps-exe-boundary` は**11/11 PASS、5分50秒**（2026-10-30、`target/quick/hakorune`=`cargo quick-build --bin hakorune` の最適化版＋同`target/quick` runtime、`tools/smokes/v2/run.sh --profile integration --owner-profile integration --suite real-apps-exe-boundary --timeout 420`）。SKIP/空marker/timeoutなし。boxtorrentは139.7秒、他は11〜57秒。
- 共有Rust検証: `ordinary_new`/`resolved_semantics`/`loop_recipe` は変更前と同じ 失敗84/1/21（成功640/375/245）。JSON appは `json_stream_aggregator_exe_runtime_boundary` smoke PASS。素の `hakorune apps/json-stream-aggregator/main.hako`（VM実行）は `borrowed-dynamic-entry/physical-formal` で拒否される（`finalized_received_birth_actual.rs` を戻しても同じ。この着地の前からか未確認。smokeのEXE経路は影響なし）。
- 次の主題は**最適化フェーズ（なるべく早く）**。boxtorrentのコンパイルは最適化版140秒・開発用`quick-build-fast`(opt0)で9分48秒。時間の89%はRust側の意味検証（`corroborate_original_v1` 約40箇所から呼ばれ毎回BTreeMapを再構築、`borrowed_formal_*`の導き直し）で、Cは4%。

## この着地の内容（共有ownerへの追加と理由）
- `{Text,null}`静的callを`DynamicCarrier`/`ordinary_nullable_text`で運ぶ。native: kernel `checked_publish_text_or_null_v1`/`checked_is_null_v1`、Text比較のcarrier operand、C shimの返却/呼出し/null比較/Return。理由: 返却Textがnullを取り得る`readData`/`get`を実EXEで動かすため。
- String Addの右辺carrier、`Text + Bool`（`NormalBool` operand、tagged Bool表示を再利用しkernel変更なし）。理由: `"root_equal=" + root_equal`。
- loop text carrierのReturn/call-Fault End、置換後のFault順序（置換Homeは置換元の位置を引き継ぐ）をRust builder/検証/MIR検証/Cで一致させた。理由: Rust検証とC検証で順序前提が食い違っていた。置換End自身のFaultは取得順のまま。
- 条件・field: `<array field>.length() <cmp>`/`me.<field> <cmp>`の条件選択、`keys().length()`のfinished image、object-only borrowed entryのObject domain、received birth actualの`Outer`（`local x = new T(..)`）。理由: `chunkCount`/`firstChunkId`/`release`/`put`。
- C: SLOT_BORROW phi（構造のみ、使う側が再証明）、Map要素借用のreceiver（`has`/null guardの枝内のみ）、object_field_borrow由来のnullable引数、動的Object初期化（`value_tag=object`）、`string_materialize`のcarrier、Copyのcarrier。理由: `put`/`release`/`birth`。
- 借用区間の葉呼び出し: 引数なしで本体が**スカラfield読書のみ**（`dynamic_i64_replace`含む）の呼び出しはMap slotを変えないと証明できるため区間を切らない。それ以外のcall/birth_call/map操作は従来どおり拒否。
- 既定OFFの診断: `[condition-select]` `[result-class]` `[field-array-return]` `[received-birth]` `[loop-call-ends]` `[invoke-verify]` `[root-home]` `[borrowed-actual]`、`NYASH_BUILDER_DEBUG`時のRust文字列検証の拒否内訳、`HAKO_PHYSICAL_DEBUG`時のCの拒否内訳（`[physical-refused]`/`[lifecycle-v4-refused]`/`[lifecycle-v4-receiver-mismatch]`）。使い捨てプローブは全撤去（`TMP-`なし）。

## 純増と退役条件
- production行（このseries全体、概算）: 先行3commit +655/−74、今回 +482/−67 で**純増**（機能追加であり整理の主張なし）。
- 退役条件: `loop_return_text_ends`/`loop_call_text_ends`の記録は、ループ側が所有を直接持つ形に統合できた時。`BorrowedArraySetSourceLoanV1`・NamedArray Push markerは従来どおり。順序規約が4箇所（builder/検証/Rust MIR検証/C）に分散しているため、1箇所の規約表現へ寄せる（下の最適化フェーズと同時に検討）。
- 800行超（既存＋今回追加）: `expression_terminal_paths.rs` 1411、`root_home.rs` 914、`home_expression_evaluation.rs` 830、`hako_llvmc_ffi_lifecycle_v4_emit.inc` 813、`home_prefix_source_loop_control.rs` 1201、`source_loop_schedule.rs` 885。行数だけの分割はせず責務で再編する。

## 残件
1. [ ] **最適化フェーズ（優先・なるべく早く）**: 検証は減らさず計算を1回にする。`corroborate_original_v1`の再構築、`borrowed_formal_*`の導き直し、同じ不変入力の結果再利用（RULES §8）。結果キャッシュは、テストが同じ inventory を書き換えるため検査を弱めるので不可。目標: boxtorrent最適化版140秒→大幅短縮、opt0版9分48秒の解消。
2. [ ] VM実行の拒否（上記）が着地前から存在するかを切り分け、必要なら直す。
3. [ ] 残る重い関数の整理（上の800行超）と、順序規約の一元化。
4. [ ] 次アプリへpointerを切り替える前に、finite_product_goalの未完（R6/R7・Loop・物理退役・最終pipeline）は別goalとして維持。

## 検証コマンド
- 選択smoke: `HAKO_ROOT=$PWD NYASH_ROOT=$PWD NYASH_BIN=$PWD/target/quick/hakorune NYASH_EMIT_EXE_NYRT=$PWD/target/quick BUILD_TIMEOUT_SECS=1200 bash tools/smokes/v2/profiles/integration/apps/boxtorrent_mini_exe.sh`（最適化版で約140秒。opt0版は`BUILD_TIMEOUT_SECS`既定300秒を超えるので使わない）。
- 固定suite: 上記の`run.sh ... --timeout 420`。ビルド: `cargo quick-build --bin hakorune`（最適化CLI）、`cargo quick-build-lifecycle-fast`（runtime）、`bash tools/build_hako_llvmc_ffi.sh`（C shim）。`cargo quick-build-fast`は開発用opt0で`target/quick/hakorune`を上書きする点に注意。
- 詳細履歴（Map借用整理series、Set receipt arm、Bool Local等）はGitに保持。
