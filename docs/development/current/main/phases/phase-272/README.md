Status: Active  
Date: 2025-12-22  
Scope: `scan_with_init` / `split_scan`（historical labels `6/7`）を `Frag + emit_frag()` へ段階吸収（numbered-label 列挙の増殖を止める）
Related:
- Design SSOT: `docs/development/current/main/design/edgecfg-fragments.md`
- Phase 269（BoolPredicateScan Frag; historical label: `8`）: `docs/development/current/main/phases/archive/phase-269/README.md`
- Phase 270（AccumConstLoop bridge; historical label: `9`）: `docs/development/current/main/phases/archive/phase-270/README.md`

# Phase 272（P0）: scan_with_init / split_scan を Frag+emit_frag へ吸収（段階適用）

## ステータス

- **P0.1（ScanWithInit; historical label: `6`）**: ✅ 完了（Frag+emit_frag 経路へ移行）
- **P0.2（SplitScan; historical label: `7`）**: ✅ 完了（Frag+emit_frag 経路へ移行）
- **2026-04-30 cleanup**: 旧 direct JoinIR lowerer shelf
  (`scan_with_init_minimal.rs` / `scan_with_init_reverse.rs` /
  `split_scan_minimal.rs` / `scan_bool_predicate_minimal.rs`) は `291x-734`
  で撤去済み。active owner は `joinir::route_entry` registry、
  `RecipeComposer`、Facts/Composer の plan 経路。

## 目的

- scan_with_init / split_scan（scan系）の CFG 構築を “numbered route label ごとの推測分岐” から外し、**EdgeCFG Frag 合成（ExitKind/wires/branches）**に収束させる。
- terminator emission を SSOT（`emit_frag()`）へ集約し、block の successors/preds 同期漏れを構造で防ぐ。

## スコープ境界

### ✅ 触る
- active route owners:
  - `src/mir/builder/control_flow/joinir/route_entry/registry/handlers/routes.rs`
  - `src/mir/builder/control_flow/plan/recipe_tree/scan_with_init_composer.rs`
  - `src/mir/builder/control_flow/plan/recipe_tree/split_scan_composer.rs`
  - `src/mir/builder/control_flow/plan/facts/loop_scan_with_init.rs`
  - `src/mir/builder/control_flow/plan/facts/loop_split_scan.rs`
- same historical path lane as scope boundary（scan_with_init / split_scan の old basename lane; old direct lowerer files were retired in `291x-734`）
- `src/mir/builder/emission/`（BoolPredicateScan と同じ “薄い入口” の追加; historical label: `8`）

### ❌ 触らない
- merge/EdgeCFG plumbing（Phase 260-268 の SSOT は維持）
- cf_loop の非JoinIR経路追加（JoinIR-only hard-freeze 維持）
- by-name ハードコード（Box名/Pattern名文字列での分岐増殖など）

## 入口SSOT（fixture/smoke）

### ScanWithInit（historical label: `6` / index_of）
- fixture: `apps/tests/phase254_p0_index_of_min.hako`（exit=1）
- smoke (VM): `tools/smokes/v2/profiles/integration/apps/archive/phase254_p0_index_of_vm.sh`

### SplitScan（historical label: `7` / split）
- fixture: `apps/tests/phase256_p0_split_min.hako`（exit=3）
- smoke (VM): `tools/smokes/v2/profiles/integration/apps/archive/phase256_p0_split_vm.sh`

## 方針（P0）

P0 は “両方一気に” ではなく、以下の順で段階適用する。

1. `ScanWithInit` を `Frag + emit_frag()` に切り替え（wiring を SSOT 化）
2. `SplitScan` を `Frag + emit_frag()` に切り替え（副作用 push を含む）
3. 旧 JoinIR 経路の撤去条件が満たせた時点で削除（本READMEに明記）

## 実装ガイド（共通）

- PHI は block 先頭（after existing phis）へ挿入し、入力を `[(pred_bb, val)]` の形で固定する:
  - `crate::mir::ssot::cf_common::insert_phi_at_head_spanned`
- terminator emission は `crate::mir::builder::control_flow::edgecfg::api::emit_frag` に集約する。
- `BoolPredicateScan`（historical label: `8`）の構造（参考）:
  - emission 入口: `src/mir/builder/emission/loop_predicate_scan.rs`

## P0.1: ScanWithInit（index_of）— Frag 化

### 狙い
- loop 骨格（header/body/step/after + early return）を Frag に落とし、Jump/Branch/Return を `emit_frag()` に集約する。

### 実装結果（✅ 完了）

- emission 入口を新設し、scan_with_init route の terminator emission を `emit_frag()`（SSOT）へ集約
  - 新規: `src/mir/builder/emission/loop_scan_with_init.rs`
  - 更新: `src/mir/builder/emission/mod.rs`
- scan_with_init route の JoinIRConversionPipeline 経路を撤去し、Frag/Recipe 経路へ切り替え
  - active route entry: `src/mir/builder/control_flow/joinir/route_entry/registry/handlers/routes.rs`
  - active composer: `src/mir/builder/control_flow/plan/recipe_tree/scan_with_init_composer.rs`
  - old direct lowerer file `src/mir/join_ir/lowering/scan_with_init_minimal.rs` was retired in `291x-734`
  - same historical path lane as scope boundary（scan_with_init old basename lane）
- P0 スコープ:
  - forward scan（`step=1`）のみ適用
  - reverse/dynamic needle 等は `Ok(None)` で不適用（既定挙動不変）
- 旧 DCE 対策（exit PHI 用の post-loop guard）を撤去（Frag が Return を直接 emit するため）

### 最小 CFG 形（forward scan）
- blocks: `header`, `body`, `step`, `after`, `ret_found`
- header:
  - `i_current = phi [i_init, preheader], [i_next, step_bb]`
  - `cond_loop = (i_current < len)`
  - branch: true→body, false→after
- body:
  - `ch = s.substring(i_current, i_current+1)`
  - `cond_match = (ch == needle)`
  - branch: true→ret_found, false→step
- step:
  - `i_next = i_current + 1`
  - jump: →header
- ret_found:
  - wire: `Return(i_current)`
- after:
  - P0 では `return -1` を既存 AST lowering に任せてもよい（after を current_block にする）

### 受け入れ
- `cargo test -p nyash-rust --lib --release`
- `HAKORUNE_BIN=./target/release/hakorune bash tools/smokes/v2/profiles/integration/apps/archive/phase254_p0_index_of_vm.sh`

### 追加の検証（推奨）

- `NYASH_VM_DUMP_MIR=1 ./target/release/hakorune --backend vm apps/tests/phase254_p0_index_of_min.hako` で PHI/terminator を確認（任意）

## P0.2: SplitScan（split）— Frag 化

### 狙い
- SplitScan（historical label: `7`）の terminator 配線（if/loop の遷移）を Frag に集約し、副作用（`result.push`）を含む形でも CFG を壊さない。

### 注意点（Fail-Fast）
- carriers が複数（`i`, `start`）なので、header の PHI を 2 本（以上）で SSA を閉じる必要がある。
- `result.push` は副作用なので、block 配置（評価順）を壊さない（P0は固定形のみ受理）。

### 実装結果（✅ 完了）

- split_scan route の JoinIRConversionPipeline 経路を撤去し、Frag/Recipe 経路へ切り替え
  - active route entry: `src/mir/builder/control_flow/joinir/route_entry/registry/handlers/routes.rs`
  - active composer: `src/mir/builder/control_flow/plan/recipe_tree/split_scan_composer.rs`
  - old direct lowerer file `src/mir/join_ir/lowering/split_scan_minimal.rs` was retired in `291x-734`
  - same historical path lane as scope boundary（split_scan old basename lane）
- emission 入口を新設し、terminator emission を `emit_frag()`（SSOT）へ集約
  - 新規: `src/mir/builder/emission/loop_split_scan.rs`
  - 更新: `src/mir/builder/emission/mod.rs`
- CFG 形（P0）:
  - blocks: `header`, `body`, `then`, `else`, `step`, `after`（+ 入口 preheader）
  - header: PHI（`i_current`, `start_current`） + loop condition
  - body: delimiter match check
  - then: `result.push(segment)` + `start_next_then` 計算（dominance 安全）
  - else: `i_next_else = i_current + 1`
  - step: PHI（`i_next`, `start_next`） + jump header
- Compare は `CompareOp::Le`（`i <= limit`）を使用（固定形）

### リファクタ結果（共通SSOT）

Phase 272 P0.2 完了後、`ScanWithInit` / `SplitScan` / `BoolPredicateScan`（historical labels: `6/7/8`）の重複を以下へ収束した（仕様不変）:

- PHI 挿入の薄いラッパ: `src/mir/builder/emission/phi.rs`
- variable_map の fail-fast 取得: `src/mir/builder/variable_context.rs`（`require(name, ctx)`）
- can_lower の戦略メモ: `src/mir/builder/control_flow/joinir/route_entry/router.rs`（`CanLowerStrategy`）
  - same historical route-entry lane as before (`router.rs`)

### 受け入れ
- `HAKORUNE_BIN=./target/release/hakorune bash tools/smokes/v2/profiles/integration/apps/archive/phase256_p0_split_vm.sh`
  - PASS（exit=3）
  - MIR 形（PHI/Branch/Jump/BoxCall(push)）を目視確認できること（任意）

## Next（planned）: Phase 273（design-first）— numbered route labelsを Plan Extractor に降格して裾広がりを止める

Phase 272（P0）で “terminator SSOT（emit_frag）へ寄せる” を完了したら、次は上流の収束（compiler flow の一本化）を行う。

- 相談メモ（外部レビュー用）: `docs/development/current/main/investigations/phase-272-frag-plan-architecture-consult.md`
- ねらい:
  - numbered route label = **Plan 抽出（pure）** に降格（builder を触らない）
  - Plan = `seq/if/loop/exit/effect/let` の固定語彙（増殖しない）
  - PlanLowerer が block/value/phi を作る唯一の箱（emit_frag は SSOT のまま）
- 受け入れ（最小）:
  - extractor が `next_block_id/next_value_id/insert_phi_*` を呼ばない（純関数）
  - Plan→Frag→emit_frag の本線が 1 本になる（pattern番号列挙を中心にしない）

## 旧 JoinIR 経路の撤去条件（SSOT）

Status: satisfied; old direct lowerer shelves were deleted in `291x-734`.

旧 `JoinIRConversionPipeline` 系の経路を削るのは、以下を満たした後に行う。

1. ScanWithInit / SplitScan（historical labels: `6/7`）の fixture/smoke が Frag 経路で PASS
2. `tools/smokes/v2/run.sh --profile quick` が悪化しない
3. router から該当 pattern の “旧経路” が消せる（最小差分で削除可能）

## テスト手順（固定）

1. `cargo build --release`
2. `cargo test -p nyash-rust --lib --release`
3. `HAKORUNE_BIN=./target/release/hakorune bash tools/smokes/v2/profiles/integration/apps/archive/phase254_p0_index_of_vm.sh`
4. `HAKORUNE_BIN=./target/release/hakorune bash tools/smokes/v2/profiles/integration/apps/archive/phase256_p0_split_vm.sh`
