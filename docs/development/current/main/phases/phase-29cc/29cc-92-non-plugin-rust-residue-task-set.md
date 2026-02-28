---
Status: Active (monitor-only)
Decision: accepted
Date: 2026-02-25
Scope: plugin 除外で残る Rust 実装を `.hako` へ段階移管するための fixed-order task set。
Related:
  - docs/development/current/main/phases/phase-29cc/README.md
  - docs/development/current/main/phases/phase-29cc/29cc-90-migration-execution-checklist.md
  - docs/development/current/main/phases/phase-29cc/29cc-93-rnr05-loop-scan-range-shape-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-94-derust-non-plugin-done-sync-ssot.md
  - docs/development/current/main/design/de-rust-lane-map-ssot.md
  - docs/development/current/main/design/de-rust-compiler-thin-rust-roadmap-ssot.md
  - docs/development/current/main/phases/phase-29y/60-NEXT-TASK-PLAN.md
  - CURRENT_TASK.md
---

# 29cc-92 Non-Plugin Rust Residue Task Set

## Purpose

- 「残りの Rust コードをどの順で薄くするか」を 1 枚で固定し、作業順の迷走を防ぐ。
- plugin 実装は本タスクセットの対象外とし、runtime/compiler の non-plugin 経路だけを扱う。
- 1 task = 1 commit = 1 acceptance gate を維持する。

## Inventory Snapshot (2026-02-24, non-plugin only)

| bucket | file | loc |
| --- | --- | ---: |
| runtime route | `src/runner/selfhost.rs` | 588 |
| runtime route | `src/runner/modes/common_util/selfhost/json.rs` | 211 |
| runtime vm-hako | `src/runner/modes/vm_hako.rs` | 193 |
| runtime vm-hako | `src/runner/modes/vm_hako/payload_normalize.rs` | 833 |
| runtime vm-hako | `src/runner/modes/vm_hako/subset_check.rs` | 727 |
| runtime vm-hako | `src/runner/modes/vm_hako/driver_spawn.rs` | 51 |
| runtime dispatch | `src/runner/route_orchestrator.rs` | 484 |
| runtime dispatch | `src/runner/dispatch.rs` | 402 |
| runtime core | `src/runtime/nyash_runtime.rs` | 153 |
| compiler plan | `src/mir/builder/control_flow/plan/pattern_pipeline.rs` | 496 |
| compiler plan | `src/mir/builder/control_flow/plan/loop_scan_v0/pipeline.rs` | 391 |
| compiler parser | `src/parser/expressions.rs` | 410 |
| compiler parser | `src/parser/statements/control_flow.rs` | 236 |
| total | non-plugin residue | 5175 |

## Fixed Order (active)

1. [done] `RNR-01` runtime seam: `vm_hako` compile bridge 分離（挙動不変, BoxShape, 2026-02-24）
2. [done] `RNR-02` runtime seam: `vm_hako` payload/subset 契約整理（判定SSOT集約, BoxShape, 2026-02-24）
3. [done] `RNR-03` runtime seam: `selfhost` JSON route 境界整理（Program/MIR ownership 固定, BoxShape, 2026-02-24）
4. [done] `RNR-04` runtime seam: dispatch/orchestrator の意味決定点の縮退（execution routing のみへ, BoxShape, 2026-02-24）
5. [done] `RNR-05` compiler seam: parser + plan pipeline の最小受理形 migration pack（BoxCount は 1 shape/commit, 2026-02-25）

Current active next:
- `none`（RNR-05 complete; monitor-only）
- closeout:
  - non-plugin de-rust done 宣言は `29cc-94-derust-non-plugin-done-sync-ssot.md` を正本とする。

## Monitor Refresh (2026-02-28)

non-plugin residue lane の monitor-only 状態を再確認した。

- `bash tools/checks/phase29y_derust_blocker_sync_guard.sh` -> PASS
- `bash tools/smokes/v2/profiles/integration/apps/phase29y_lane_gate_quick_vm.sh` -> PASS
- `tools/checks/dev_gate.sh plugin-module-core8` -> PASS（HM2-min2/min3 追加後も plugin lane 契約は維持）

判定:
- non-plugin residue は `none` 維持（failure-driven reopen 条件なし）

RNR-02 progress:
- min1 done（2026-02-24）: `subset_check` の shared helper 参照を `shape_contract` 経由に切替。
- min2 done（2026-02-24）: shared shape/canonicalization 実装を `shape_contract` へ移設し、`payload_normalize` は参照側へ縮退。
- min3 done（2026-02-24）: `compile_bridge` の MIR emit ENV override を scoped guard（Drop restore）へ置換。
- min4 done（2026-02-24）: vm-hako driver source を外部テンプレート化（`driver_main.hako` + `driver_source.rs`）。
- min5 done（2026-02-24）: handle同期 + `call(args=2)` 判定を `shape_contract` helper へ集約（payload/subset 双方で使用）。
- min6 done（2026-02-24）: `subset_control_misc` に `call(args=2)` 契約テストを追加（dynamic accept/reject・non-method reject）。

RNR-03 progress:
- min1 done（2026-02-24）: Stage-A child payload（MIR/Program/empty）の ownership 判定を `selfhost/json.rs` の `resolve_stage_a_payload` に集約。
- min2 done（2026-02-24）: `selfhost.rs` の route 配線を resolver 経由へ切替し、accepted-mir 処理を `accept_stage_a_mir_module` へ集約。
- min3 done（2026-02-24）: payload resolver 単体テスト追加 + gate 実行（`cargo check`, `compile_v0` test, `phase29y_lane_gate_vm`）で境界を固定。

RNR-04 progress:
- min1 done（2026-02-24）: Stage-A compat policy/guard を `route_orchestrator.rs` から `selfhost/stage_a_policy.rs` へ移設。
- min2 done（2026-02-24）: `selfhost.rs` の Stage-A guard 呼び出し先を `stage_a_policy` へ切替し、orchestrator を VM route 専用に縮退。
- min3 done（2026-02-24）: policy test を新モジュールへ移設 + runtime lane gate（`phase29y_lane_gate_vm`）で実行契約を再固定。
- min4 done（2026-02-24）: Stage-A spawn/compat ladder を `stage_a_spawn` / `stage_a_compat_bridge` へ分離し、`selfhost.rs` を route sequencing 中心へ縮退。

RNR-05 planned mins (docs-first):
- min1 done（2026-02-25）: parser 側の受理形を 1 つ固定（target shape + fixture + expected parse）し、Rust/.hako の同形観測契約を明記。
- min2 done（2026-02-25）: plan pipeline 側の対応点（`pattern_pipeline` / `loop_scan_v0` のどちらか）を 1 箇所だけ拡張し、rejected/accepted 条件を fail-fast で固定。
- min3 done（2026-02-25）: fast gate fixture を追加して shape pin を固定（`scan_loop_v0_lte_n_minus1_min`）。
- target shape SSOT:
  - `docs/development/current/main/phases/phase-29cc/29cc-93-rnr05-loop-scan-range-shape-ssot.md`
  - shape id: `rnr05.loop_scan.range_v0`（`i < n` + `i <= n - 1`）

RNR-05 progress:
- min1 done（2026-02-25）: parser AST pin を `src/tests/parser_loop_scan_range_shape.rs` に追加し、`while i <= n - 1` の AST 形（`LessEqual` + `Subtract(n,1)`）を固定。
- min2 done（2026-02-25）: `src/mir/builder/control_flow/plan/loop_scan_v0/facts.rs` の condition 受理を `i < n` に加えて `i <= n - 1` へ拡張。`i <= n` reject テストを追加して fail-fast 境界を固定。
- min3 done（2026-02-25）: fixture `apps/tests/phase29bq_joinir_scan_loop_range_lte_minus1_min.hako` を追加し、`phase29bq_fast_gate_cases.tsv` に case id `scan_loop_v0_lte_n_minus1_min` を登録。`phase29bq_fast_gate_vm.sh --only scan_loop_v0_lte_n_minus1_min` と `--only bq` を PASS。
- RNR-05 done（2026-02-25）: parser + plan pipeline minimal migration pack（1 shape）を完了。

## Acceptance Gate (per task)

- `cargo check --bin hakorune`
- `cargo test -q compile_v0_emits_boxcall_open_with_two_args -- --nocapture`
- runtime lane C を触る task では追加で:
  - `bash tools/smokes/v2/profiles/integration/apps/phase29y_lane_gate_vm.sh`

## Commit Boundary Lock

- BoxCount と BoxShape を同コミットで混在しない。
- fast gate FAIL の状態で `cases.tsv` / matrix を更新しない。
- 1 task 完了時に `README.md` / `29cc-90` / `CURRENT_TASK.md` の pointer を同期する。
