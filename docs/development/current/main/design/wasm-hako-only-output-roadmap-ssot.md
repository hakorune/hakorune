---
Status: Active
Decision: accepted
Date: 2026-02-26
Scope: WASM 出力経路を将来 `.hako` 単独（Rust thin runtime only）へ移行する順序と受け入れ基準を固定する。
Related:
  - docs/development/current/main/design/de-rust-master-task-map-ssot.md
  - docs/development/current/main/design/de-rust-lane-map-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-133-wsm-g2-browser-demo-task-plan.md
  - docs/development/current/main/phases/phase-29cc/29cc-118-wasm-grammar-compat-map-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-150-wsm-p1-min1-emit-wat-cli-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-151-wsm-p1-min2-wat-parity-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-152-wsm-p2-min1-wat2wasm-bridge-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-153-wsm-p3-min1-import-object-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-154-wsm-p4-min1-binary-writer-doc-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-155-wsm-p4-min2-binary-writer-skeleton-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-156-wsm-p4-min3-hako-writer-entry-parity-doc-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-157-wsm-p4-min4-hako-writer-const-parity-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-158-wsm-p4-min5-neg-const-parity-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-159-wsm-p4-min6-shape-table-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-177-wsm-p4-min7-buffer-file-binary-contract-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-160-wsm-p5-min1-default-cutover-doc-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-161-wsm-p5-min2-route-policy-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-162-wsm-p5-min3-default-hako-lane-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-163-wsm-p5-min4-hako-lane-bridge-shrink-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-164-wsm-p5-min5-native-helper-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-165-wsm-p5-min6-shape-expand-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-166-wsm-p5-min7-shape-trace-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-167-wsm-p5-min8-legacy-retire-readiness-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-168-wsm-p5-min9-legacy-retire-execution-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-169-wsm-p5-min10-legacy-hard-remove-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-170-wsm-p6-min1-route-policy-default-noop-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-171-wsm-g4-min1-nyash-playground-console-baseline-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-172-wsm-g4-min2-nyash-playground-canvas-primer-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-173-wsm-g4-min3-webcanvas-fixture-parity-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-174-wsm-g4-min4-canvas-advanced-fixture-parity-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-175-wsm-g4-min5-headless-two-example-parity-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-176-wsm-g4-min6-gate-promotion-closeout-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-184-wsm-p7-min1-hako-only-done-criteria-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-185-wsm-p7-min2-default-hako-only-guard-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-186-wsm-p7-min3-two-demo-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-187-wsm-p7-min4-compat-retention-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-188-wsm-p8-min1-bridge-retire-readiness-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-189-wsm-p9-min0-non-native-inventory-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-190-wsm-p9-min1-const-binop-native-shape-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-191-wsm-p9-min2-loop-canvas-primer-bridge-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-192-wsm-p9-min3-canvas-advanced-bridge-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-193-wsm-p9-min4-bridge-retire-refresh-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-194-wsm-p10-min1-loop-extern-native-emit-design-lock-ssot.md
  - src/backend/wasm/
  - projects/nyash-wasm/
---

# WASM `.hako`-Only Output Roadmap SSOT

## Goal
将来の主経路を「`.hako` が MIR から WASM を出力する」形へ寄せる。  
Rust 側はランナー/ポータビリティ維持の thin layer とし、WASM codegen/runtime 契約の正本は `.hako` 側へ移す。

## Current Boundary (as-is)
1. 現在の WASM codegen/runtime の主実装は Rust (`src/backend/wasm/*`)。
2. `.hako` 側は段階移行中で、契約固定（fixture/smoke/SSOT）が主役。
3. したがって現時点では Rust 依存は必要。無理に切り離さず、契約を先に固定する。

## Target Boundary (to-be)
1. `.hako` compiler が MIR -> WASM text/binary を生成する。
2. Rust 側は最小の実行ブリッジと配布保守（Windows/macOS/CI）に限定する。
3. contract/gate は維持し、backend 実装主体だけを Rust -> `.hako` に置換する。

## Migration Phases (fixed order)
1. **P0 Contract Lock (ongoing)**
   - 1 blocker = 1 shape で extern/boxcall 語彙を固定。
   - 受け入れは `phase29cc_wsm_g3_*_contract_vm.sh` と `tools/checks/dev_gate.sh wasm-demo-g3-*`。
2. **P1 WAT Emitter Parity**
   - `.hako` 側に WAT emitter（最小命令セット）を作り、Rust 出力と fixture 単位で同値化。
   - 受け入れは「同一 fixture の WAT 比較 + 既存 wasm smoke 緑」。
   - 入口ロック（done）: `--emit-wat`（`29cc-150`）。
3. **P2 Toolchain Bridge (`wat2wasm`)**
   - `.hako` が生成した WAT を外部ツール連結で `.wasm` 化する（Rust wasm codegen 本体を使わない）。
   - 受け入れは「WAT->WASM 変換後の browser demo/headless gate 緑」。
   - bridge lock（done）: `29cc-152`（`phase29cc_wsm_p2_min1_bridge_lock_vm.sh`）。
4. **P3 Runtime Contract Port**
   - JS import object 生成契約（supported list / fail-fast 文言）を `.hako` 側へ移植。
   - Rust runtime は fallback ではなく thin compatibility lane として残す。
   - runtime contract lock（done）: `29cc-153`（`phase29cc_wsm_p3_min1_import_object_lock_vm.sh`）。
5. **P4 Wasm Binary Writer (Rust-free output)**
   - `.hako` 側に wasm binary writer（section/LEB128）を実装し、WAT依存を外す。
   - 受け入れは「`.hako` 単独で `.wasm` 出力 + 既存 wasm smoke 緑」。
   - docs lock（done）: `29cc-154`（`phase29cc_wsm_p4_min1_docs_lock_vm.sh`）。
   - skeleton lock（done）: `29cc-155`（`phase29cc_wsm_p4_min2_binary_writer_lock_vm.sh`）。
   - `.hako` entry/parity docs lock（done）: `29cc-156`（`phase29cc_wsm_p4_min3_hako_writer_docs_lock_vm.sh`）。
   - const-return parity lock（done）: `29cc-157`（`phase29cc_wsm_p4_min4_hako_writer_const_parity_vm.sh`）。
   - negative const parity lock（done）: `29cc-158`（`phase29cc_wsm_p4_min5_hako_writer_neg_const_parity_vm.sh`）。
   - shape table lock（done）: `29cc-159`（`phase29cc_wsm_p4_min6_shape_table_lock_vm.sh`）。
   - Buffer/File binary contract lock（done）: `29cc-177`（typed BufferBox API + FileBox readBytes/writeBytes）。
6. **P5 Default Cutover**
   - 既定経路を `.hako` emitter/binary writer に切替。Rust backend は `--legacy-wasm-rust` 相当の互換 lane に縮退。
   - 連続マイルストーンで緑を確認後、互換 lane を retire 判定。
   - docs lock（done）: `29cc-160`（`phase29cc_wsm_p5_min1_default_cutover_docs_lock_vm.sh`）。
   - route policy lock（done）: `29cc-161`（`phase29cc_wsm_p5_min2_route_policy_lock_vm.sh`）。
   - default hako-lane lock（done）: `29cc-162`（`phase29cc_wsm_p5_min3_default_hako_lane_lock_vm.sh`）。
   - bridge shrink lock（done）: `29cc-163`（`phase29cc_wsm_p5_min4_hako_lane_bridge_shrink_vm.sh`）。
   - native helper lock（done）: `29cc-164`（`phase29cc_wsm_p5_min5_native_helper_lock_vm.sh`）。
   - shape expand lock（done）: `29cc-165`（`phase29cc_wsm_p5_min6_shape_expand_lock_vm.sh`）。
   - shape trace lock（done）: `29cc-166`（`phase29cc_wsm_p5_min7_shape_trace_lock_vm.sh`）。
   - legacy retire readiness lock（done）: `29cc-167`（`phase29cc_wsm_p5_min8_legacy_retire_readiness_lock_vm.sh`）。
   - legacy retire execution lock（done）: `29cc-168`（`phase29cc_wsm_p5_min9_legacy_retire_execution_lock_vm.sh`）。
   - legacy hard-remove lock（done）: `29cc-169`（`phase29cc_wsm_p5_min10_legacy_hard_remove_lock_vm.sh`）。
7. **P6 Route Policy Stabilization**
   - route policy env を default-only no-op として維持するか、完全撤去するかの判断を段階固定する。
   - default-only no-op lock（done）: `29cc-170`（`phase29cc_wsm_p6_min1_route_policy_default_noop_lock_vm.sh`）。
8. **G4 Project Migration Baseline**
   - `projects/nyash-wasm` の移植開始点として playground console baseline を lock する。
   - baseline lock（done）: `29cc-171`（`phase29cc_wsm_g4_min1_playground_console_baseline_vm.sh`）。
   - canvas primer lock（done）: `29cc-172`（`phase29cc_wsm_g4_min2_playground_canvas_primer_vm.sh`）。
   - webcanvas fixture parity lock（done）: `29cc-173`（`phase29cc_wsm_g4_min3_webcanvas_fixture_parity_vm.sh`）。
   - canvas_advanced fixture parity lock（done）: `29cc-174`（`phase29cc_wsm_g4_min4_canvas_advanced_fixture_parity_vm.sh`）。
   - headless two-example parity lock（done）: `29cc-175`（`phase29cc_wsm_g4_min5_headless_two_examples_vm.sh`）。
   - G4 closeout gate lock（done）: `29cc-176`（`phase29cc_wsm_g4_min6_gate_promotion_closeout_vm.sh`）。
9. **P7 Hako-Only Done Criteria Lock**
   - P6/G4 lock を束ねて `.hako`-only 完了判定を再起動し、default-only 契約を継続監査する。
   - done criteria lock（done）: `29cc-184`（`hako-only done criteria`）。
   - default hako-only guard lock（done）: `29cc-185`（`phase29cc_wsm_p7_min2_default_hako_only_guard_vm.sh`）。
   - two-demo lock（done）: `29cc-186`（`phase29cc_wsm_p7_min3_two_demo_lock_vm.sh`）。
   - compat retention lock（accepted-but-blocked done）: `29cc-187`（`phase29cc_wsm_p7_min4_compat_retention_lock_vm.sh`）。
10. **P8 Compat Bridge Retire Readiness Lock**
   - default-only 契約を維持しつつ、`BridgeRustBackend` 撤去判定を accepted-but-blocked で固定する。
   - bridge retire readiness lock（accepted-but-blocked done）: `29cc-188`（`phase29cc_wsm_p8_min1_bridge_retire_readiness_vm.sh`）。
11. **P9 Non-Native Shrink (shape-by-shape)**
   - min0 inventory lock（done）: `29cc-189`（`phase29cc_wsm_p9_min0_non_native_inventory_lock_vm.sh`）。
   - min1 const-binop native shape lock（done）: `29cc-190`（`phase29cc_wsm_p9_min1_const_binop_native_lock_vm.sh`）。
   - min2 loop/canvas primer bridge lock（accepted-but-blocked done）: `29cc-191`（`phase29cc_wsm_p9_min2_loop_canvas_primer_bridge_lock_vm.sh`）。
   - min3 canvas_advanced bridge lock（accepted-but-blocked done）: `29cc-192`（`phase29cc_wsm_p9_min3_canvas_advanced_bridge_lock_vm.sh`）。
   - min4 bridge retire refresh lock（accepted-but-blocked done）: `29cc-193`（`phase29cc_wsm_p9_bridge_retire_refresh_guard.sh`）。
12. **P10 Loop/Extern Native Emit (design-first)**
   - min1 design lock（accepted-but-blocked done）: `29cc-194`（`phase29cc_wsm_p10_loop_extern_native_emit_design_guard.sh`）。
   - next: `WSM-P10-min2`（loop+extern matcher inventory lock, analysis-only）。

## Non-Goals
1. 一括置換（big bang）で Rust 実装を即削除しない。
2. 既存 gate を bypass する temporary fallback を入れない。
3. `.hako` 側で未固定の語彙を silently accept しない（fail-fast 維持）。

## Acceptance Gates (minimum)
1. `tools/checks/dev_gate.sh wasm-demo-g3-core`
2. `tools/checks/dev_gate.sh wasm-demo-g3-full`
3. `tools/checks/dev_gate.sh portability`
4. `cargo check --features wasm-backend --bin hakorune`（移行期間のみ）
5. `tools/checks/dev_gate.sh wasm-boundary-lite`（P1/P2 境界ロック）

## Operation Rule
1. 新しい WASM 語彙追加は今まで通り docs-first（SSOT -> smoke -> code）。
2. `.hako` 移植タスクを切るときも「1語彙/1契約/1コミット」原則を維持する。
3. blocker 判定は `CURRENT_TASK.md` の wasm lane active next を正本にする。
