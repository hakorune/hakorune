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
6. **P5 Default Cutover**
   - 既定経路を `.hako` emitter/binary writer に切替。Rust backend は `--legacy-wasm-rust` 相当の互換 lane に縮退。
   - 連続マイルストーンで緑を確認後、互換 lane を retire 判定。

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
