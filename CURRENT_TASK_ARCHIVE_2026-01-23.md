# CURRENT_TASK (root pointer)

Status: SSOT  
Scope: Repo root の旧リンク互換。現行の入口は `docs/development/current/main/10-Now.md`。

- Now (SSOT): `docs/development/current/main/10-Now.md`
- Backlog (SSOT): `docs/development/current/main/30-Backlog.md`
- Current phase (SSOT): `docs/development/current/main/phases/phase-29bq/README.md`

## Top Priority

- ✅ Phase 107 `balanced_depth_scan`: analysis-only view へ集約して selfhost/Stage‑B 形の揺れを解消（apps smokes PASS）
- loop_cond_break_continue: allow_extended=false のとき body_exit_allowed を作らず、if-assign が exit-if 経由で落ちる問題を回避（fixture+fast gate 追加済み）。
- Next: selfhost 実行で新しい freeze/reject が出た時だけ BoxCount/BoxShape を選んで拡張し、それ以外は gate 維持
- G1 identity check は `tools/selfhost_identity_check.sh --skip-build` で事前ビルド済み Stage1/Stage2 を使う（既定: `target/selfhost/hakorune`, `target/selfhost/hakorune.stage2`）
- G1 full failed (Stage1 build OOM): no diff files generated in this run (hakorune_emit_mir killed; direct emit failed)
- G1 OOM 根因候補: Stage‑B が巨大な AST/Program/MIR を **一括保持→最後に JSON 化**しており、ピークがデータ量比例で伸びる（出力 0 bytes のまま長時間）。
  - 提案（優先順）: (1) JSON 出力を to_writer でストリーミング化 / JSONL 化、(2) モジュール単位の分割コンパイル（吐いて捨てる）、(3) 最小 incremental キャッシュ、(4) Stage‑B を seed として固定する運用。
- Phase0 検証ブロッカー: Stage1 CLI 直呼び（STAGE1_SOURCE_TEXT, emit_program_json）でも 120s timeout（BuildBox 停滞疑い）。ログ採取が必要。
- BuildBox route freeze: UsingCollectorBox.collect/1（scan_with_quote_loop_min）。`/tmp/buildbox_program.json` は 0 bytes。LOG: `logs/selfhost/stageb_20260122_083846_720263.log`
- BuildBox strict freeze: ParserLiteralBox.parse_map/3。LOG: `logs/selfhost/stageb_20260122_104031_3743950.log`
- BuildBox strict freeze: ParserLiteralBox.parse_block_expr/3。LOG: `logs/selfhost/stageb_20260122_104801_3746507.log`
- BuildBox strict freeze: ParserLiteralBox.parse_map/3。LOG: `logs/selfhost/stageb_20260122_111725_3752762.log`
- BuildBox strict freeze: ParserPeekBox.parse/3（nested loop / strict_nested_loop_guard）。LOG: `logs/selfhost/stageb_20260122_133208_3793599.log` CMD: `HAKO_USE_BUILDBOX=1 NYASH_SELFHOST_KEEP_RAW=1 HAKO_JOINIR_STRICT=1 tools/selfhost/selfhost_build.sh --in lang/src/compiler/entry/compiler.hako --json /tmp/buildbox_program.json`
- fast gate failed: selfhost_blocker_scan_methods_loop_min → planner required, but planner returned None. LOG: `/tmp/phase29bq_fast_gate_3820693_bq_list.log` (last fn: `FuncScannerBox.mask_string_literals_keep_len/1`)
- fast gate failed: selfhost_blocker_rewriteknown_itoa_complex_step_min → output mismatch (expected 12, actual 1). LOG: `/tmp/phase29bq_fast_gate_3835797_bq_list.log`
- fast gate failed: phase29bq_selfhost_blocker_while_cap_min → output mismatch (expected 10, actual 4). LOG: `/tmp/phase29bq_fast_gate_3843893_bq_list.log`
- Next: ParserLiteralBox.parse_map/3 の実物形に合わせて fixture/shape を再調整（strict BuildBox）
- Priority shift: 構造改革（BoxShape）を優先して進める（BoxCount は parse_map 再調整で一旦区切り）。候補: generic_loop v1 の ShapeId/overlap-freeze・RejectReason の粒度改善・v0/v1 body_check 呼び分けの明文化。
- 予定: strict/dev + planner_required で **generic_loop v1-only フェーズ**に切り替え（v0 抽出は無効化）。release 既定は v0 維持。SSOT 更新先: `docs/development/current/main/10-Now.md`, `docs/development/current/main/design/generic-loop-v1-shape-ssot.md`, `docs/development/current/main/design/coreloop-generic-loop-v0-ssot.md`。
- Policy: .hako 側は **v1-only**（`generic_loop_v0` は Rust 互換レイヤ専用、移植しない）。
- Done: strict/dev + planner_required で `generic_loop_v0` 抽出を無効化（v1-only 強制）。
- Freeze tag SSOT: plan/normalize/joinir layers separated. SSOT: `docs/development/current/main/design/planfrag-freeze-taxonomy.md`.
- generic_loop v1 ShapeId: overlap/shape-required operational rules documented. SSOT: `docs/development/current/main/design/generic-loop-v1-shape-ssot.md`.
- ✅ C18: generic_loop v0/v1 overlap guard SSOT 化（policy + tests + docs）。
- C18 unit tests failed: `cargo test -p nyash-rust generic_loop_v1_shape_overlap_freezes` / `generic_loop_v0_rejects_v1_shape_effect_step_only` → `SkeletonFacts` missing `feature_slots` in test initializers (cargo test known issue).
- ✅ C19-B: CondCanon → CondProfile の観測線を generic_loop に接続（挙動不変、Boundは literal/var のみ）。
- ✅ C19-C: CondProfile を GenericLoopFacts に保持（観測のみ、挙動不変）。
- ✅ C19-D: ConditionShape/StepShape → CondProfile のアダプタ追加（観測のみ）。
- ✅ C19-E: CondProfile を Verifier まで観測で接続（受け取りのみ、挙動不変）。
- ✅ JoinIR は観測レイヤ（Recipe/Verifier/Lower の SSOT ではない）を SSOT 化。
- Recipe-only lowering: VerifiedRecipe-only boundary documented (type-level contract). SSOT: `docs/development/current/main/design/recipe-tree-and-parts-ssot.md`.
- Recipe-first entry contract SSOT added: `docs/development/current/main/design/recipe-first-entry-contract-ssot.md`.
- Recipe-first required updates list recorded in SSOT (entry contract).
- Recipe-first pilot: Pattern2Break recipe verification (planner_required only), debug tag `[recipe:pattern2]`.
- Recipe-first migration phased plan recorded (proposal only; deferred until VM bootstrap). SSOT: `docs/development/current/main/design/recipe-first-migration-phased-plan-proposal.md`.
- Recipe-first migration started as **structural refactor** (behavior-preserving, planner_required/dev only). Decision logged in `docs/development/current/main/20-Decisions.md`.
- ✅ Recipe-first Phase A 完了 (2026-01-23): contracts/matcher/feature slots 追加（挙動変更なし）。
- ✅ Recipe-first Phase B 完了 (2026-01-23): matcher 並行パス接続（planner_required/dev のみ、挙動変更なし）。
- ✅ Recipe-first Phase C 完了 (2026-01-23): Pattern2Break 入口で recipe_contract 必須化（planner_required/dev のみ、挙動変更なし）。
- ✅ Recipe-first Phase C2 完了 (2026-01-23): Pattern2Break の recipe 検証を matcher に集約（fail-fast, SSOT 化）。
- ✅ Recipe-first Phase C3 完了 (2026-01-23): Pattern2Break を RecipeComposer 経由で CorePlan 生成（planner_required/dev のみ）。
- ✅ Recipe-first Phase C4 完了 (2026-01-23): Pattern2Break を recipe-only に（planner_required/dev では DomainPlan 返さない）。
- ✅ Recipe-first Phase C5 完了 (2026-01-23): Pattern2Break recipe-only を gate で固定（fixture + planner_required gate）。
- ✅ Recipe-first Phase C6 完了 (2026-01-23): Pattern3IfPhi recipe 検証を matcher に追加（planner_required/dev のみ）。
- ✅ Recipe-first Phase C7 完了 (2026-01-23): Pattern3IfPhi を RecipeComposer 経由で CorePlan 生成（planner_required/dev のみ）。
- ✅ Recipe-first Phase C8 完了 (2026-01-23): Pattern3IfPhi を recipe-only に（planner_required/dev では DomainPlan 返さない）。
- ✅ Recipe-first Phase C9 完了 (2026-01-23): Pattern4Continue recipe-first migration (builder, matcher verification, composer, recipe-only gate)。
- ✅ Recipe-first Phase C10 完了 (2026-01-23): Pattern5InfiniteEarlyExit recipe-first migration (builder, matcher verification, composer, recipe-only gate)。
- ✅ Recipe-first Phase C11 完了 (2026-01-23): Pattern1SimpleWhile recipe-first migration (builder, matcher verification, composer, recipe-only gate)。
- ✅ Recipe-first Phase C12 完了 (2026-01-23): Pattern1CharMap recipe-first migration (builder, matcher verification, composer, recipe-only gate)。Body AST は Facts から再構築。root block は NoExit 契約（nested body は StmtOnly）。
- ✅ Recipe-first Phase C13 完了 (2026-01-23): Pattern1ArrayJoin recipe-first migration (builder with IfV2+Stmt×2, matcher verification, composer, recipe-only gate)。Body は IfV2（separator guard）+ Stmt×2（element append, increment）。body_contract は NoExit。
- ✅ Recipe-first Phase C14 完了 (2026-01-23): Pattern6–9 recipe-first migration (builder + matcher verification + composer + recipe-only gate)。
- ✅ Recipe-first Phase C15 完了 (2026-01-23): Scan loops (loop_scan_methods_v0 / loop_scan_methods_block_v0 / loop_scan_phi_vars_v0 / loop_scan_v0) recipe-first migration (verify + compose + recipe-only gate)。
  - planner_required_cases.tsv に scan cases を pin 済み。
- Planned (cleanup after pattern migration): generic_loop v0/v1 overlap guard を SSOT 化し、shape 判定の戻り値を Result 化して曖昧さを型で表現する（overlap freeze のタグ契約も統一）。
  - v0/v1 overlap guard を policy へ集約（候補: `src/mir/policies/generic_loop_overlap_policy.rs`）。
  - `generic-loop-v1-shape-ssot.md` に overlap guard / v1-only / freeze tag を1行明記。
  - 最小テスト: v1 shape overlap の freeze / v0 が v1 shape を reject を固定。
- Planned (design): CondProfile（parameterized condition skeleton）を並走追加し、ConditionShape の増殖を止める。最初は adapter で互換保持、段階移行（SSOT: `docs/development/current/main/design/condition-observation-ssot.md`）。
- Planned (design): Body=CFG skeleton enum / Condition=CondProfile の最終形へ移行（Verifier-only acceptance, no rewrite）。
- ✅ Recipe-first Phase C14 完了 (2026-01-23): Pattern6–9 (ScanWithInit / SplitScan / BoolPredicateScan / AccumConstLoop) recipe-first migration (builder, matcher verification, composer, recipe-only gate)。
- ✅ Recipe-first Phase C15 完了 (2026-01-23): Scan loop v0 family (loop_scan_methods_v0 / loop_scan_methods_block_v0 / loop_scan_phi_vars_v0 / loop_scan_v0) recipe-first migration (segment verification, composer, recipe-only gate)。
- ✅ Recipe-first Phase C16 完了 (2026-01-23): Collection loops (loop_collect_using_entries_v0 / loop_bundle_resolver_v0 / loop_true_break_continue) recipe-first migration (segment verification, composer, recipe-only gate)。
- ✅ Recipe-first Phase C17 完了 (2026-01-23): LoopCond* (break_continue / continue_only / continue_with_return / return_in_body) recipe-first migration (matcher verification, composer, recipe-only gate)。
- ✅ BoxCount: DivCountdownBy10 shape added to generic_loop_v1 for int_to_str selfhost blocker (fixture: `phase29bq_div_countdown_by10_min.hako`)。
- ✅ BoxCount: ScanWhilePredicate shape added to generic_loop_v1 for _extract_ident selfhost blocker (fixture: `phase29bq_funcscanner_extract_ident_min.hako`)。
- G1 full failed (Stage1 build OOM): no diff files generated in this run (hakorune_emit_mir killed; direct emit failed)
- G1 OOM 根因候補: Stage‑B が巨大な AST/Program/MIR を **一括保持→最後に JSON 化**しており、ピークがデータ量比例で伸びる（出力 0 bytes のまま長時間）。
  - 提案（優先順）: (1) JSON 出力を to_writer でストリーミング化 / JSONL 化、(2) モジュール単位の分割コンパイル（吐いて捨てる）、(3) 最小 incremental キャッシュ、(4) Stage‑B を seed として固定する運用。
- Phase0 検証ブロッカー: Stage1 CLI 直呼び（STAGE1_SOURCE_TEXT, emit_program_json）でも 120s timeout（BuildBox 停滞疑い）。ログ採取が必要。
- ✅ loop_cond_break_continue: ThenOnlyBreakIf（`if { break } else { non-exit }`）を受理。recipe-first lowering を使用し、body_exit_allowed はこの形がある場合に無効化（fixture+fast gate 追加済み）。

## Reboot / Resume (SSOT)

最短の再開手順（迷走防止・ここだけ見れば復帰できる）:

1) 作業ツリー確認 → commit/stash を先に確定
   - `git status -sb`
   - `git diff --stat`
   - 失敗中WIPなら: `git stash push -u -m "wip/<topic> (fails gate)"`

2) 入口ゲートで現状を再現（ログSSOTは /tmp）
   - fast: `./tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh`
   - selfhost(canary, health-check only): `HAKO_JOINIR_DEBUG=1 SMOKES_ENABLE_SELFHOST=1 ./tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh`
   - LOG: `/tmp/phase29bq_selfhost_*.log`
   - 失敗時は summary パスを **1行だけ** `CURRENT_TASK.md` に記録する（再開時の導線固定）
   - planner-required blocker capture (BoxCount seed): `./tools/smokes/v2/profiles/integration/joinir/phase29bq_collect_planner_required_blocker_vm.sh apps/tests/<fixture>.hako <label>` → `/tmp/phase29bq_joinir_blocker_<label>_*.summary`
   - latest capture: `/tmp/phase29bq_joinir_blocker_loop_if_return_local_1042332.summary` (first line: `fixture=apps/tests/phase29bq_selfhost_blocker_parse_program2_loop_if_return_local_min.hako`)
   - latest capture: `/tmp/phase29bq_joinir_blocker_loop_if_else_return_local_1386020.summary`
     - `fixture=apps/tests/phase29bq_selfhost_blocker_parse_program2_loop_if_else_return_local_min.hako`
     - `47:[ERROR] ❌ MIR compilation error: [plan/freeze:contract] planner required, but planner returned None (legacy fallback forbidden)`
   - latest capture: `/tmp/phase29bq_joinir_blocker_loop_if_else_if_return_1457605.summary`
     - `fixture=apps/tests/phase29bq_selfhost_blocker_parse_program2_loop_if_else_if_return_min.hako`
     - `47:[ERROR] ❌ MIR compilation error: [plan/freeze:contract] planner required, but planner returned None (legacy fallback forbidden)`
   - latest capture: `/tmp/phase29bq_joinir_blocker_nested_loop_if_else_return_1613831.summary`
     - `fixture=apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_return_min.hako`
     - `first_freeze_or_reject: not found`
     - `MIR compilation error: [normalizer] generic loop v0: nested loop has no plan`
   - latest capture: `/tmp/phase29bq_fast_gate_2101751_bq_list.log`
     - `fixture=apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_fallthrough_join_min.hako`
     - `first_freeze_or_reject: [ERROR] ❌ MIR compilation error: [plan/freeze:contract] planner required, but planner returned None (legacy fallback forbidden)`
     - resolved: ThenOnlyReturnIf 追加 + fixture pin（commit `d389309e2`）
   - latest capture: `/tmp/phase29bq_fast_gate_blocker_nested_loop_if_else_fallthrough_join_return_local_1768864049.summary`
     - `fixture=apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_fallthrough_join_return_local_min.hako`
     - `first_freeze_or_reject: [ERROR] ❌ MIR compilation error: [plan/freeze:contract] planner required, but planner returned None (legacy fallback forbidden)`
     - resolved: ThenOnlyReturnIf の return-local 受理 + fixture pin（commit `8d66b3f01`）
   - latest capture: `/tmp/phase29bq_fast_gate_blocker_nested_loop_if_else_fallthrough_join_return_blockexpr_1768866056.summary`
     - `fixture=apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_fallthrough_join_return_blockexpr_min.hako`
     - `first_freeze_or_reject: [normalizer] Unsupported value AST: BlockExpr ...`
     - resolved: normalizer + JSON v0 bridge で BlockExpr value 対応（commit `3a963d3d6`）
   - latest capture: `/tmp/phase29bq_fast_gate_blocker_nested_loop_if_else_fallthrough_join_return_blockexpr_local_1768871507.summary`
     - `fixture=apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_fallthrough_join_return_blockexpr_local_min.hako`
     - `first_freeze_or_reject: [ERROR] ❌ MIR compilation error: [plan/freeze:contract] planner required, but planner returned None (legacy fallback forbidden)`
     - resolved: ThenOnlyReturnIf で then 側 `local` + `return <value>` を受理（commit `9fd6ff8d4`）
   - latest capture: `/tmp/phase29bq_fast_gate_blocker_nested_loop_if_else_fallthrough_join_return_local_blockexpr_1768910831.summary`
     - `fixture=apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_fallthrough_join_return_local_blockexpr_min.hako`
     - `first_freeze_or_reject: [ERROR] ❌ MIR compilation error: [plan/freeze:contract] planner required, but planner returned None (legacy fallback forbidden)`
     - resolved: local-init pure 判定に BlockExpr を許可（commit `b7dbd8c6b`）
- selfhost canary flake (reproduced twice): `/tmp/phase29bq_selfhost_phase29bq_selfhost_blocker_scan_methods_loop_min.hako.log`
  - `summary: [diag/selfhost] steptree_root=1010:[phase132/debug] StepTree root for 'StageBBodyExtractorBox.build_body_src/2': ...`
  - `first_freeze_or_reject: not found`
  - `follow-up: HAKO_JOINIR_DEBUG=0 で単独実行は rc=0（PASS）。DEBUG=1 は trace 出力で expected=0 と不一致になるため、flake扱いで再現時のみ記録→停止。`
- planner_required dev gate FAIL: `phase29bq_selfhost_blocker_scan_methods_loop_min.hako` (expected 0, got 1). LOG: `/tmp/phase29bp_master_list.log`

3) 現状確認（selfhost）
   - 現状: `phase29bq_selfhost_planner_required_dev_gate_vm.sh` は PASS（subset 緑）
   - **Policy**: Loop RecipeBlockization v0（受理=RecipeBlock contract + parts）と `.hako mirbuilder` pin が揃うまで、selfhost “復帰作業”（fixture拡大・ブロッカー探索・受理形追加）は一時停止する（gate は “健康診断” としてのみ実行）。
   - selfhost 側で新しい freeze/reject を見つけても、原則ここでは直さない（Resume conditions を満たしてから BoxCount で扱う）。

4) SSA/PHI の次フェーズ設計（BoxShape）
   - 入口メモは `## Planned (SSA/PHI structure locks)` を参照（Copy SSOT → leaf-branch/JoinKey/sealing の順序を固定）

## Notes

- 詳細は `docs/development/current/main/phases/phase-29bq/README.md` に集約する。
- Status: if 語彙は `RecipeItem::IfV2 { contract: IfContractKind, .. }` に統一済み（旧 `RecipeItem::{If,IfJoin}` は撤去、drift check: `rg -n "RecipeItem::If(Join)?\\b" src/mir/builder/control_flow/plan --glob '!*.md'` → 0件）。
- Status: `parts::dispatch` は `lower_block_internal(kind, ...)` に集約済み（M20）、accept_kind の挙動分岐は Facts 側 field へ移設（M21, drift check: `rg -n "accept_kind == LoopCondBreakAcceptKind::" src/mir/builder/control_flow/plan/features` → 0件）。
- Status: ✅ loop recipe-first / Loop RecipeBlockization v0 closeout 完了（SSOT: `docs/development/current/main/design/recipe-tree-and-parts-ssot.md`）。
- Status: ✅ Verified entry SSOT（`parts::entry`）導入済み。RecipeBlock lowering は release でも常時 verify される。
- Status: ✅ 受理=Verifierのみ（Dispatch/Parts の contract 再判定は debug-only に縮退）
- Status: ✅ RecipeItem::LoopV0 は `parts::loop_::lower_loop_v0` 経由で lowering（AST 直下ろしを撤去）。
- Handoff: 次は selfhost 再開準備（`.hako mirbuilder` pin + restart condition 整合）へ進む。
- Bootstrap staging (SSOT): Stage0→Stage1→Stage2 の同一性テストを最優先で固定し、boundary shrink order（module_roots/using → AST JSON v0 → parser 最後）で移植を進める。
- Loop skeleton のソース側SSOTテンプレ（CoreLoop Skeleton）は `src/mir/builder/control_flow/plan/features/coreloop_skeleton/README.md` をSSOTとして運用する。
- StepBb（continue→step_bb→header）を既定としてテンプレ化済み。HeaderBb（legacy continue→header 直）は **Phase 11 で物理削除済み**（fast gate green 維持の上で撤去完了）。
- `collect_carrier_vars_from_recipe` や `[plan/trace]` の重複（trait/macro化）は BoxShape の追加リファクタになるため保留（挙動不変を維持）。
- 方針（最優先）: selfhost unblock よりも **compiler 側の表現力（CorePlan/Facts/Normalizer）の収束**を優先する（SSOT: `docs/development/current/main/design/compiler-expressivity-first-policy.md`）。
- “完成品キット”の増殖を避ける（BoxShape→BoxCount、共通部品SSOT優先）: `docs/development/current/main/design/lego-composability-policy.md`。
- キャンペーンSSOT: `docs/development/current/main/design/compiler-cleanliness-campaign-ssot.md`
- `.hako` mirbuilder migration: Phase-0〜の入口/契約/ピンは `docs/development/current/main/design/hako-mirbuilder-migration-phase0-entry-contract-ssot.md` をSSOTにする（未登録 `using` は `[freeze:contract][module_registry] missing module: ... hint=hako.toml:[modules]` で fail-fast）。

## Remaining Tasks (post M19–M21, BoxShape)

Selfhost “復帰作業” を再開する前に、BoxShape で「新しいコンパイラーの形」をもう一段だけ固める。

0) Loop RecipeBlockization v0 closeout（BoxShape, docs+drift）
   - SSOT: `docs/development/current/main/design/recipe-tree-and-parts-ssot.md`
   - Done: loop body の判定/構築が Facts SSOT（StmtOnly/ExitOnly/NoExit）へ集約され、features 側の手判定が戻らない（drift checks green）
   - Status: ✅ Completed（drift: `rg -n "count_control_flow|ControlFlowDetector" src/mir/builder/control_flow/plan/features --glob '!*.md'` → 0件）

1) RecipeBlock/Parts の適用範囲拡大（挙動不変）
   - 対象: `loop_cond_break_continue` 以外の if/exit/join 断片も `RecipeBlock -> parts::dispatch` 経由へ段階移行
   - 受け入れ基準: 入口が view-first（`CondBlockView` を producer が作って渡す）、consumer は AST を直読しない

1.1) Segmenter SSOT（線形区間専用）を確定
   - 境界: `Loop` / `Exit` / 「契約に収まらない If」
   - MixedBlock を増やさず、`Seq` に分割するのを SSOT 化
   - `NoExitBlockRecipe` は線形専用に戻し、`LoopV0` を構造ノードへ退避（BoxShape）

2) ✅ LoopV0 lowering の self-contained 化（最終核）
   - dispatch の `RecipeItem::LoopV0` で `lower_nested_loop_depth1_any` を呼ばない
   - `parts::loop_::lower_loop_v0` で VerifiedRecipeBlock 入口のみを通す
   - SSOT: `docs/development/current/main/design/recipe-tree-and-parts-ssot.md`

2) Verifier 運用の最終整理（dev/strict only のままか、適用範囲を増やすか）
   - いまの方針: `parts/verify` の RecipeBlock contract は dev/strict only
   - ToDo: “どの invariant は release 既定でも fail-fast すべきか” を Decision として固定（SSOT 追記）

3) CondBlockView prelude（plan-side lowering / Phase B4）
   - 現状: Phase B2 により `ASTNode::BlockExpr { prelude_stmts, tail_expr }` で **構造的に表現可能**になった。
   - ✅ Phase B4 完了: JoinIR/plan 側でも condition prelude を stmt-only effects として lower してから tail を評価する。
   - SSOT: `docs/development/current/main/design/cond-block-view-prelude-ssot.md`
   - Roadmap SSOT: `docs/development/current/main/design/map-literal-eviction-and-blockexpr-roadmap-ssot.md`（Phase B4）
   - Pinned: `apps/tests/phase29bq_cond_prelude_planner_required_min.hako`（fast gate）
   - Next: prelude 語彙の SSOT を中立モジュールへ集約（facts と normalizer の二重化を防止）

6) Final syntax (phase plan): MapLiteral eviction + BlockExpr
   - Goal: `{...}` を式ブロックに固定し、MapLiteral を `%{...}`（`=>`）へ退避して condition entry を “ただの式” に統一する
   - Spec (provisional): `docs/reference/language/block-expressions-and-map-literals.md`
   - Roadmap SSOT: `docs/development/current/main/design/map-literal-eviction-and-blockexpr-roadmap-ssot.md`

4) ExitKind depth>1 の方針決定（Recipe/Verifier/Parts の一貫性）
   - 現状: `ExitKind::{Break,Continue}{depth}` は語彙として存在（N>1 は未整備）
   - ToDo: RecipeBlock verifier と parts::exit 側の契約を SSOT 化してから受理範囲を増やす（BoxCount で pin してもよい）

5) CorePlan “意味論ゼロ” への縮退の次の一手（最終形の導線）
   - 現状: shrink criteria は docs に固定済み
   - ToDo: CorePlan が “機械のみ” になった段階で rename/責務縮退（`LoweredRecipe` 等）を検討

## Selfhost status

- Current: ✅ `SMOKES_ENABLE_SELFHOST=1 ./tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh` PASS
- Canary note (resolved): `phase29bq_selfhost_blocker_scan_methods_loop_min.hako` が `debug-fuel` 上限で `exit=1` になることがあるため、canary の `--json-file` 実行は `--debug-fuel unlimited`（timeoutガード付き）に固定
- Policy: RecipeTree+Parts の最終形へ収束するまで、selfhost “復帰作業” は一時停止（gate は維持、対象拡大もしない）
- Selfhost subset list: ✅ `apps/tests/phase29bq_selfhost_blocker_parse_program2_if_return_min.hako` を `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv` に追加済み
- Resume check: ✅ `.hako mirbuilder` pin smokes PASS（phase1/phase2）
- Recent fixes (high level):
  - `loop_cond_break_continue` に ElseGuardBreakIf 形を追加（selfhost loop pattern unblock）
  - Stage‑B defs 注入 + JSON v0 bridge で `BoxName.method()` を static 関数呼び出しへ解決（FuncScannerBox ident 抽出を MapBox ステートで安定化）

## Progress: .hako mirbuilder migration

Phase-0/1 status (SSOT pointers):
- Phase-0 entry contract SSOT: `docs/development/current/main/design/hako-mirbuilder-migration-phase0-entry-contract-ssot.md`
- Language v1 freeze SSOT: `docs/development/current/main/design/selfhost-language-v1-freeze-ssot.md`

✅ Phase-0 pin: `phase29bq_hako_mirbuilder_phase0_pin_vm.sh` PASS（入口SSOT + guard + env固定）
✅ Phase-1 (P1-0..P1-5): Program(JSON v0) → MIR JSON v0 (minimal Return(Int)) + pin smoke 追加済み
  - Code: `lang/src/compiler/mirbuilder/emit_mir_json_v0_from_program_json_v0.hako`
  - Pin smoke: `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase1_min_vm.sh`
  - Fixture: `apps/tests/phase29bq_hako_mirbuilder_phase1_literal_return_min.hako`
  - Note: `--emit-program-json-v0` は deterministic な Rust 側生成に固定（Stage-1 stub の JoinIR freeze を回避）

✅ Phase-2 (P2-1..P2-3): vocab expansion + pin smoke (3 cases)
  - Rust Program(JSON v0) generator expanded (Print/Local(init)/Assignment/Var/Binary(+)):
    - `src/runner/stage1_bridge/mod.rs`
  - .hako mirbuilder expanded:
    - Print(Int) / Local(Int)+Return(Var) / Local+Assignment(x=x+Int)+Return(Var)
    - `lang/src/compiler/mirbuilder/emit_mir_json_v0_from_program_json_v0.hako`
  - Pin smoke:
    - `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_phase2_min_vm.sh`
  - Fixtures:
    - `apps/tests/phase29bq_hako_mirbuilder_phase2_print_min.hako`
    - `apps/tests/phase29bq_hako_mirbuilder_phase2_local_return_min.hako`
    - `apps/tests/phase29bq_hako_mirbuilder_phase2_inc_return_min.hako`
  - ✅ Phase-3: Program(JSON v0) scanner SSOT（JoinIR-safe）を導入し、直スキャンの罠（read_digits misread 等）を call site から撤去:
    - `lang/src/compiler/mirbuilder/program_json_v0_scanner_box.hako`
  - Note: Phase-0/1/2 pin smoke は `HAKO_JOINIR_PLANNER_REQUIRED=1` を明示する（fallback で原因がブレるのを防止）。

## Local artifacts (untracked, do not commit)

- `chatgpt_pro_shortcircuit_joins_bundle.zip` / `CHATGPT_PRO_QUESTION_shortcircuit_joins.md`
- `joins_pushdown_review.zip`

Decision (avoid acceptance drift):
- Do not add a new “condition list” box directly.
- Prefer **Recipe SSOT** boundaries (Facts builds recipe; Lower consumes recipe only).
- Add **one new recipe vocabulary** per blocker (fixture+fast gate+SSOT).
- AcceptKind 契約は `loop_cond_break_continue` に閉じ、他箱への横展開は当面しない（Rust層肥大化防止のため、必要なら docs で補う）。

## BoxShape Focus (loop_cond unification)

現状: `loop_cond_*` 5モジュールは「同一アルゴリズムのパラメータ化」で、共通ロジックが多数重複している。
目的: 構造で重複を潰し、Facts/Recipe/Lower の責務と入口を明確化する（受理形の追加なし）。

対象:
- `loop_cond_break_continue`
- `loop_cond_continue_only`
- `loop_cond_continue_with_return`
- `loop_cond_return_in_body`
- `loop_true_break_continue`

重複ポイント（共通化対象）:
- `is_true_literal` / `is_supported_bool_expr`
- `count_control_flow` の呼び出しパターン
- Recipe 構築ボイラープレート
- Branch 分類ヘルパー

方針:
- BoxShape のみ（受理形は増やさない）
- 1フェーズ=1コミット、fast gate で確認
- 旧モジュールは段階的に deprecate

Phases:
1) helpers 抽出（共通関数を 1 箇所へ）
2) 統一 Recipe 型の導入（各箱の Facts/Lower は挙動不変）
3) facts の統合と variants 分割（新しい `loop_cond_unified/variants`）
4) 旧モジュールの deprecate（入口/README/REGISTRY を更新）

Progress:
- facts は `loop_cond_unified/variants` に移設済み。旧 `loop_cond_*::facts` は re-export のみ。

## Router Refactor (BoxShape, in-progress)

目的: `route_loop_pattern` の verify/tag/lower 重複を構造で潰し、入口SSOTを強化する（挙動不変）。

Step 1:
- `router.rs` に `lower_verified_core_plan(...)` / `freeze_expected_plan(...)` を追加
- shadow/release adopt の繰り返しを helper に統一
- gate/期待値の挙動は変更しない

Step 2:
- shadow/release adopt の順序と条件を composer 側に集約（router は 1 呼び出しに縮退）

Step 3:
- 入口SSOTを `plan/REGISTRY.md` に明記（router→planner→composer→lower の主導権/契約）

## Planned (anti-misstep guardrails)

- Facts 入口の reject を「理由SSOT（enum）」で返し、dev/strict で `[plan/reject]` を 1 行ログ固定する（Ok(None) の黙殺を減らす）。
- reject_reason → handoff（次に試す箱）をコード側SSOTのテーブルに集約し、順序依存で“たまたま通る”状態を減らす。
- CoreLoop Skeleton（テンプレ）側で fail-fast を強化し、PHI/terminator 系の失敗を同じ場所・同じメッセージで落とす。
- “数だけ増える軸”（clusterN 等）は、箱を増やす前に profile/table SSOT 化して追加点を 1 箇所に集約する（BoxShape）。
- else-only return（`if cond { <non-exit> } else { return ... }`）は `IfAny` に押し込まず、box-local recipe variant を増やして Facts→Recipe→Lower のズレを構造で潰す（BoxCount: fixture+fast gate+1語彙）。

### Structure locks (planned)

“気合いで守る”ではなく、構造で逸脱を起こしにくくするためのロック。

- **SSOT入口の型・可視性で固定**: `exit_branch` / `loop_carriers` / `edgecfg_stubs` / `carriers` のように「作り場」を `pub(in crate::mir::builder)` + private helper で囲い、features/** に直書きが再流入しない構造にする（SSOT: `docs/development/current/main/design/feature-helper-boundary-ssot.md`）。
- **Recipe共通化は shape-only を強制**: `recipes` は `RecipeBody` + `StmtIdx/StmtRange` + `refs`（StmtRef/StmtPair/StmtSpan 等）のみに限定し、意味語彙（例: `ExitIf`）は各箱の enum に閉じる（acceptance drift 防止）。
- **RecipeBody の統一スコープを固定**: recipe-first を採用した箱は **必ず** `recipes::RecipeBody` を所有する（Facts→Recipe→Lower の lifetimes を跨がせない）。recipe-first でない箱には強制しない（広域移行を避ける）。
- **棚卸しを機械化**: closeout checklist の `rg` を “1コマンドで回る” 形に固定し、逸脱が増えたら即 fail-fast できる導線を用意する（まず docs/手順SSOT → 必要なら tools へ昇格）。
- **アクセサ統一（読みやすさの構造ロック）**: `idx.0` 直書きを減らすため、`StmtRef::index()` / `RecipeBody::get_ref(StmtRef)` のような shape-only アクセサを追加し、recipe/pipeline の “参照取り出し” を統一する（意味語彙を増やさない）。
- **Recipe-first 強制の追加ロック**: compile_fail doctest で “features から parts::dispatch を呼べない” を固定し、`cargo xtask arch` 相当の drift lint を追加して禁則パターンをCIで落とす。

## Planned (design, compiler-first)

- “直交軸” は **分類レンズ（タグ）**として扱い、SSOTは Skeleton/FeatureSet の主語に固定する（SSOT: `docs/development/current/main/design/coreplan-skeleton-feature-model.md`）。
- 型システムは `MirType` / `RuntimeTypeTag` / `TypeView` に責務分離し、実行時fail-fastの真実は `docs/reference/language/types.md` に固定する（SSOT: `docs/development/current/main/design/type-system-policy-ssot.md`）。
- “完成品キット”の残りを潰す優先順（compiler-first）: Phase 12 `exit_map`/`exit_if_map` 共通化 → Phase 13 `nested_loop_depth1*` 統合（Kind enum）→ Phase 14 型の棚卸し（SSOT主語固定）→ Phase 15+ 型分離実装（慎重）。
- planner-first ケース表（TSV）から “担当/slot/Out-of-scope” のカタログを自動生成する（docs-only ではなく将来のツール化タスク）。
- pipeline の共通工程（carrier init / join payload / wires）を “helper” ではなく “pipeline step” としてSSOT化し、完成品キット（大箱複製）を抑制する（SSOT: `docs/development/current/main/design/lego-composability-policy.md`）。

## Planned (SSA/PHI structure locks)

- 短絡(`&&`/`||`) + joins の 3経路問題は、現状は `CoreEffectPlan::Copy` SSOT で畳む（契約は `docs/development/current/main/design/short-circuit-joins-ssot.md`）。
- Copy を削除したい場合は “構造” が先（BoxShape）:
  - Block params / edge args（PHIを join block の引数として表現）へ寄せる
  - CFG を seal してから SSA/PHI を確定（pred追加後のPHI入力不足を構造で禁止）
  - join の一意性を `JoinKey` で固定（outer/inner join の迷子を防ぐ）

### Planned (cond lowering: leaf-branch + consumer-join)

次フェーズの BoxShape 案（Copy の置き換え先の “構造”）。まずは設計SSOTを固める。

- **API 分離（SSOT）**:
  - branch 文脈: `lower_cond(expr, on_true, on_false)`（bool 値を作らない、leaf predicate でのみ branch）
  - value 文脈: `lower_bool(expr) -> ValueId`（内部で `Tdef/Fdef/J` を作り、PHI は join `J` に 1 回だけ）
- **Join は消費側が作る**:
  - `if (cond) { ... } else { ... }` の post-if 継続点（合流ブロック）でのみ joins/PHI を扱う
  - `&&/||` 展開の途中に “中間 join/phi” を作らない（3経路問題の温床になる）
- **fail-fast 不変条件（低コスト順）**:
  1. leaf-branch 制約（And/Or/Not を直接 branch していない）
  2. 出口完全性（`lower_cond` 領域からの出口は必ず `on_true` or `on_false`）
  3. join 一意性（value 文脈の join `J` は 1 個だけ）
  4. PHI 入力完全性（pred 数と phi 入力数が一致、重複/欠落なし）
  5. 支配（phi 入力の定義が対応 pred の末尾までで成立）
  6. critical edge 方針（edge に仕事を載せるなら必ず split）
  7. 到達可能性（死ブロックが phi 入力に混ざらない）

## Progress (compiler cleanliness campaign)

- ✅ Phase 5-6: reject_reason/handoff の入口ガード（`loop_cond_continue_only` / `loop_cond_break_continue`）
- ✅ Phase 5-6a: generic_loop reject reason SSOT 化 (4/N-11/N: NoValidLoopVarCandidates, AmbiguousLoopVarCandidates, ControlFlowAfterInBodyStep, ExitAfterInBodyStep, LoopVarUsedAfterInBodyStep, UnsupportedStmtAfterInBodyStep, ContinueIfStepRequiresTrailingExit, BreakElseStepMustBeFinalStmt)
- ✅ Phase 5-6b: Freeze message SSOT 化 (12/N-14/N: AmbiguousLoopVarCandidates, NoValidLoopVarCandidates(登録のみ), MultipleConditionalStepAssignments, MultipleStepAssignments)
  - 14/N: StepPlacementDecision.reject_reason 型置換（canon 層も RejectReason に）
- ✅ Phase 5-6c: loop_cond_continue_only reject SSOT 化 (15/N-16/N: MultipleNestedLoopsInContinueIfPrelude, ControlFlowInNestedLoopPreludePostlude)
- ✅ Phase 5-6d: plan/accept SSOT 化 (17/N: log_accept 導入 + loop_cond_break_continue 2箇所)
- ✅ Phase 5-6e: accept inventory + logging scope SSOT (18/N)
- ✅ Phase 5-6f: match_return_facts reject SSOT (19/N-24/N: scrutinee, too_few_arms, else_not_literal, arm_label, arm_not_literal, arm_literal_type)
- ✅ Phase 7: pipeline step 化（`features/steps/carrier_collect.rs` を導入し、2 pipeline で共用）
- ✅ Phase 8: pipeline step 化（`features/steps/join_payload.rs` を導入）
- ✅ Phase 9: pipeline step 化（`features/steps/loop_wiring_standard5.rs` を導入）
- ✅ Phase 10-11: HeaderBb legacy path 非推奨化→物理削除（warn-only → remove）
- ✅ Phase 15 T2/T4/T5: 型システム責務分離完了（MirType 意味論リーク 0 件、MirType 完全遮断）
  - T2: RuntimeTypeTag 導入（VMValue 入口分類）
  - T4: RuntimeTypeSpec 導入（意味論 SSOT）
  - T5: spec_from_mir_type を type_ops.rs に移動（runtime_type_spec.rs は MirType に依存しない）
  - `rg "MirType" src/backend/runtime_type_spec.rs` → 0 件
  - Decision: accepted（types.md）
  - Gate: phase29bq_fast_gate_vm.sh 全緑
- ✅ ExitBranch: direct-exit sites migrated（`branchn_return`, `exit_map`, `generic_loop_body`, `conditional_update_join` → `features/exit_branch.rs` helpers）
- ✅ ExitBranch: loop_cond_* ContinueWithPhiArgs migrated（`loop_cond_break_continue`, `loop_cond_return_in_body`, `loop_cond_continue_with_return`）
- ✅ ExitBranch: loop_cond/loop_true Break/Continue migrated (T15: Break/Continue 直書き移設完了)
- ✅ ExitBranch: loop_cond_* Return migrated (T16: Return(Option<ValueId>) 直書き移設完了)
- ✅ EdgeCFG stubs: loop_cond_break_continue Normal wires migrated (T17)
- ✅ EdgeCFG stubs: loop_cond_return_in_body Normal wires migrated (T18)
- ✅ EdgeCFG stubs: loop_wiring_standard5 Normal wires migrated (T19)
- ✅ EdgeCFG stubs: loop_cond_break_continue BranchStub migrated (T20)
- ✅ EdgeCFG stubs: loop_wiring_standard5 BranchStub migrated (T21)
- ✅ EdgeCFG stubs: exit_if_map boundary-clean confirmed (T23)
- ✅ CarrierSets SSOT: body/recipe/outer 入口を `features/carriers.rs` に固定（T24-T30 + docs inventory）
- ✅ loop_carriers.rs: build_expr_carrier_join_args を if_join から移設 (T32)
- ✅ RejectReason SSOT: match_return freeze messages migrated (T34)
- ✅ Planner entry guards SSOT: freeze string policy documented (T35)
- ✅ RejectReason SSOT: generic_loop freeze messages migrated (T36)
- ✅ Loop PHI SSOT: PHI/bindings を `loop_carriers` に集約（T37-T47）
- ✅ Loop PHI SSOT: loop_cond_continue_with_return は coreloop_skeleton 経由でPHI生成（`build_header_step_phis`、直書きなし）(T49 verify)
- ✅ Loop PHI SSOT: loop_cond_continue_with_return trace logs compacted (T50)
- ✅ Loop PHI SSOT: remove in_str-specific plan/trace debug (T51)
- ✅ Loop PHI SSOT: trace dump compacted in loop_cond_continue_with_return (T52)
- ✅ Cleanliness closeout: CorePhiInfo direct writes only in loop_carriers
- ✅ Cleanliness closeout: plan/trace dumps fully compacted (no map/vec dumps)
- ✅ T54: inventories verified and closeout checklist documented (features/** direct stub writes removed)
- ✅ Body lowering: lower_stmt_block moved to steps (1 pattern shared)
- ✅ Docs: PhiInputStrategy SSOT (design-only) added
- ✅ Docs: cross-pipeline helper map added
- ✅ Recipe-first (BoxShape): loop_cond_* migrated（Facts→Recipe→Lower only, no re-validation）
  - `loop_cond_continue_only` / `loop_cond_break_continue` / `loop_cond_return_in_body` / `loop_cond_continue_with_return`
- /tmp/phase29bq_fast_gate_blocker_else_return_local_20260121_013114.summary
- resolved: ElseOnlyReturnIf で else 側 local+return を受理（commit `22715b0e4`）
- P2 blocker phase107/balanced_depth_scan: .hako修正では解決不可（Rust JoinIR拡張必要）, LOG: `/tmp/phase107_find_balanced_array_end_vm_output.log`
- joinir/freeze: ParserStringScanBox.scan_with_quote/3 LOG: `logs/selfhost/stageb_20260122_084055_974019.log` CMD: `HAKO_USE_BUILDBOX=1 NYASH_SELFHOST_KEEP_RAW=1 tools/selfhost/selfhost_build.sh --in lang/src/compiler/entry/compiler.hako --json /tmp/buildbox_program.json`
- joinir/freeze: ParserStringScanBox.scan_with_quote/3 LOG: `logs/selfhost/stageb_20260122_090721_3721617.log` CMD: `HAKO_USE_BUILDBOX=1 NYASH_SELFHOST_KEEP_RAW=1 tools/selfhost/selfhost_build.sh --in lang/src/compiler/entry/compiler.hako --json /tmp/buildbox_program.json`
- joinir/freeze: UsingCollectorBox.collect/1 LOG: `logs/selfhost/stageb_20260122_091933_3722764.log` CMD: `HAKO_USE_BUILDBOX=1 NYASH_SELFHOST_KEEP_RAW=1 HAKO_JOINIR_STRICT=1 tools/selfhost/selfhost_build.sh --in lang/src/compiler/entry/compiler.hako --json /tmp/buildbox_program.json`
- joinir/freeze: UsingCollectorBox.collect/1 LOG: `logs/selfhost/stageb_20260122_094316_3725939.log` CMD: `HAKO_USE_BUILDBOX=1 NYASH_SELFHOST_KEEP_RAW=1 HAKO_JOINIR_STRICT=1 tools/selfhost/selfhost_build.sh --in lang/src/compiler/entry/compiler.hako --json /tmp/buildbox_program.json`
- joinir/freeze: ParserPeekBox.parse/3 LOG: `logs/selfhost/stageb_20260122_100352_3730507.log` CMD: `HAKO_USE_BUILDBOX=1 NYASH_SELFHOST_KEEP_RAW=1 HAKO_JOINIR_STRICT=1 HAKO_JOINIR_DEBUG=1 tools/selfhost/selfhost_build.sh --in lang/src/compiler/entry/compiler.hako --json /tmp/buildbox_program.json`
- joinir/freeze: ParserPeekBox.parse/3 LOG: `logs/selfhost/stageb_20260122_100624_3731252.log` CMD: `HAKO_USE_BUILDBOX=1 NYASH_SELFHOST_KEEP_RAW=1 HAKO_JOINIR_STRICT=1 HAKO_JOINIR_DEBUG=1 tools/selfhost/selfhost_build.sh --in lang/src/compiler/entry/compiler.hako --json /tmp/buildbox_program.json`
- joinir/freeze: ParserExprBox.parse_string2/3 LOG: `logs/selfhost/stageb_20260122_122439_3770944.log` CMD: `HAKO_USE_BUILDBOX=1 NYASH_SELFHOST_KEEP_RAW=1 HAKO_JOINIR_STRICT=1 tools/selfhost/selfhost_build.sh --in lang/src/compiler/entry/compiler.hako --json /tmp/buildbox_program.json`
- joinir/freeze: ParserExprBox.parse_factor2/3 LOG: `logs/selfhost/stageb_20260122_124501_3777106.log` CMD: `HAKO_USE_BUILDBOX=1 NYASH_SELFHOST_KEEP_RAW=1 HAKO_JOINIR_STRICT=1 tools/selfhost/selfhost_build.sh --in lang/src/compiler/entry/compiler.hako --json /tmp/buildbox_program.json`
- joinir/freeze: phase29bq_selfhost_blocker_stageb_bundle_mod_if_min.hako func=Stage1UsingResolverBox._collect_using_entries/1 LOG: /tmp/phase29bq_fast_gate_3936252_bq_list.log
