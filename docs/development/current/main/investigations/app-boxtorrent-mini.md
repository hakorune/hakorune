# APP-boxtorrent-mini

Status: **acceptance reached** (fixed suite 11/11). Date: 2026-10-30
Scope: unchanged boxtorrent-mini, canonical EXE, expected output and exit.
Refs: [RULES](../../../RULES.md), [CURRENT_STATE](../CURRENT_STATE.toml), [Home contract](../../../../reference/language/ownership.md), [Map contract](../../../../reference/runtime/runtime-data-dispatch.md).

## 現在地
- `boxtorrent_mini_exe` が PASS（期待出力13行＋`Result: 0`、exit 0）。固定 `real-apps-exe-boundary` は**11/11 PASS**（当初5分50秒、現在1分35秒）（2026-10-30、`target/quick/hakorune`=`cargo quick-build --bin hakorune` の最適化版＋同`target/quick` runtime、`tools/smokes/v2/run.sh --profile integration --owner-profile integration --suite real-apps-exe-boundary --timeout 420`）。実EXE10本＋空marker1本、SKIP/timeoutなし。（最適化後の値は残件1）
- 共有Rust検証: `ordinary_new`/`resolved_semantics`/`loop_recipe` は変更前と同じ 失敗84/1/21（成功640/375/245）。JSON appは `json_stream_aggregator_exe_runtime_boundary` smoke PASS（検証はEXE経路。VMはactive laneとして撤退済みで検証対象外）。
- 所有モデルの次段を2026-10-30に決定: 仕様は[ownership.md §0](../../../../reference/language/ownership.md)、順序は[ADR-002](../../../adr/adr-002-explicit-ownership-and-compiler-migration.md)の移行表（切替はselfhostより前、それまで既存アプリの意味は不変）。最初の実装候補は`move x`の受理＋移転後使用の拒否とShared型の追加。maintainerは今回レガシー退役(1)(2)を選択。残るparked workload/唯一の安全guardの削除は保持契約と代替検証の判断待ちで、所有step 1へ自動切替しない。

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
- 退役着地検証: `cargo quick-build --bin hakorune`28.53秒PASS、`cargo check --profile quick --features llvmlite-compat --lib`PASS、削除featureのcfg残存0・unexpected_cfgs警告0。固定suite11/11（98.71秒、実EXE10＋空marker、SKIP/timeoutなし）、cohortの失敗名不変。旧guard inventoryの未登録44件は前後同一、naming既存赤は未変更。
- レガシー退役(2): caller-zeroを再確認した歴史的状態/記録だけのguard2本（149行）を退役、入力workloadは保持。public k2 wideは1565→1563で下限300を維持。166候補の一部は実callerがあり、残りの実行/MIR/Fault/lifecycle・parked workload guardとRust直接fixture3本は保持。
- レガシー退役(1b): 未接続Rust6 filesと孤立plugin-loader bin（src合計1082行、非test882行）、空feature5件を削除。cohortは失敗84/1/21・成功640/375/245、失敗名も前後一致。旧LLVM/JIT・plugins-only/aot-plan-importは現役callerがあり保持。extern_box/LAYER_GUARDは既存manifest未同期の再生成が約2万行を巻き込むため保持し、手編集なし。
- レガシー退役(1a): ファイル名・basename・module mapの参照を再確認し、ビルド対象外268 filesを削除（archiveへ移さずGit履歴を保持）、現行入口7文書を修正。legacy-selfhost/lang VMは保持。pointer guard・diffチェックPASS、docs-slimはPASS（既存inventory drift警告）。naming guardとguard inventoryは未変更入力で既存赤。
1. [~] **最適化フェーズ（第2弾完了）**: 最適化版の測定で boxtorrent 約140秒→**約32秒**、小さなアプリ約11秒→**2.3秒**、固定suite 5分50秒→**1分35秒**（2026-10-30、11/11）。効いた順: (a) 最終Preparedでの`corroborate_original_v1`の結果の再利用（同じ組で約187万回の再検証→1回。inventoryの版数はプロセス内で一意で、作成・書き換えごとに更新。seeds側のテスト差し替えは記憶を消す。固定点ループ中は書き換わるので対象外。83→33.7秒）、(b) `SourceNodeSiteV1`の`Rc<[..]>`化（約-20%）、(c) リンカをmoldに（約7秒→0.24秒。`HAKO_AOT_FUSE_LD=default|<name>`で上書き）、(d) C側lifecycle-v4のpredecessor前計算（約-10%）、(e) `nm --no-sort`（約-2秒）、(f) C側v2検証の関数ごとの索引（定義ブロック・支配関係の走査を関数ごとに1回へ。答えは走査と同一で、作れない形は従来の走査。v2 3.4→1.6秒。退役条件: lv4の`hako_lv4_function_index`と1つの関数索引へ統合した時に重複を消す）。検査は一つも減らしていない。**測って除外**: 一時マップのHashMap化（悪化）、重複確認のBTreeSet除去（誤差）、`validated_named_arrays`のメモ化（0.01%）、Cranelift（LLVMのオブジェクト生成は約0.5%）、C側v2検証の削減（C全体4%）。次の候補（設計が要る、この順）: Text End台帳の`SourceLoopCleanupPlanV2`への統合、サイトの整数ID化、`borrowed_formal_*`の再導出。速度の目安: 小さなアプリ3〜5秒（達成）、boxtorrent規模60秒以下（達成）→長期20秒以下、固定suite2分以下（達成）。測定は静かな状態で2回以上、`cargo quick-build --bin hakorune`の最適化版で。注意: `lv4_body_leaf_scalar`は「Map slotを無効化しない」の意味で純粋ではない。read/mutモデルへ持ち込まない。
2. [ ] 残る重い関数の整理（上の800行超）と、順序規約の一元化。
3. [ ] 次アプリへpointerを切り替える前に、finite_product_goalの未完（R6/R7・Loop・物理退役・最終pipeline）は別goalとして維持。

## 検証コマンド
- 選択smoke: `HAKO_ROOT=$PWD NYASH_ROOT=$PWD NYASH_BIN=$PWD/target/quick/hakorune NYASH_EMIT_EXE_NYRT=$PWD/target/quick BUILD_TIMEOUT_SECS=1200 bash tools/smokes/v2/profiles/integration/apps/boxtorrent_mini_exe.sh`（最適化版で約140秒。opt0版は`BUILD_TIMEOUT_SECS`既定300秒を超えるので使わない）。
- 固定suite: 上記の`run.sh ... --timeout 420`。ビルド: `cargo quick-build --bin hakorune`（最適化CLI）、`cargo quick-build-lifecycle-fast`（runtime）、`bash tools/build_hako_llvmc_ffi.sh`（C shim）。`cargo quick-build-fast`は開発用opt0で`target/quick/hakorune`を上書きする点に注意。
- 詳細履歴（Map借用整理series、Set receipt arm、Bool Local等）はGitに保持。
