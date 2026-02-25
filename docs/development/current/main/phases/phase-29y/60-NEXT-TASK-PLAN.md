---
Status: Active (Y1/Y2/Y3 done, min1/min2/min3 done, RVP-0..RVP-5 done, RING1-CORE-06..09 done)
Decision: provisional
Date: 2026-02-19
Scope: Phase 29y runtime lane（lane C）の current/next と運用契約を短く維持する。
Related:
  - docs/development/current/main/design/de-rust-lane-map-ssot.md
  - docs/development/current/main/design/runtime-gc-policy-and-order-ssot.md
  - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
  - docs/development/current/main/phases/phase-29y/README.md
  - docs/development/current/main/phases/phase-29y/61-NEXT-TASK-HISTORY.md
  - docs/development/current/main/phases/phase-29y/70-APP-FIRST-OUTSOURCE-INSTRUCTIONS.md
  - docs/development/current/main/phases/phase-29y/80-RUST-VM-FEATURE-AUDIT-AND-HAKO-PORT-SSOT.md
  - docs/development/current/main/phases/phase-29y/81-RUST-VM-TO-HAKO-VM-FEATURE-MATRIX.md
  - docs/development/current/main/phases/phase-29y/85-RING1-PROMOTION-DRYRUN-TASK-PACKS.md
  - CURRENT_TASK.md
---

# Phase 29y Next Task Plan (Now/Next/Later)

## 0. Current Position

- Phase 29x runtime lane（X1-X66）は完了。
- Phase 29y docs-first（10/20/30/40/50）と optional GC queue（min1/min2/min3）は完了。
- APP-1（Gate Log Summarizer）acceptance は PASS。
- APP-2（Controlflow Probe）acceptance は PASS。
- APP-3（MIR Shape Guard）acceptance は PASS。
- Current blocker（runtime lane）は `none`（RVP-5-min12 complete; monitor-only）。
- runtime diagnostic pin（non-gating）:
  - `D-RVP-continue-assignment`
  - `tools/smokes/v2/profiles/integration/apps/phase29y_continue_assignment_in_continue_stale_guard_vm.sh`
- compiler pipeline diagnostic pin（non-gating）:
  - `D-B-stage1-emit-timeout`
  - `tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_continue_assignment_timeout_block_vm.sh`
  - `D-B-run-binary-only-ported`
  - `tools/smokes/v2/profiles/integration/apps/phase29y_hako_run_binary_only_ported_vm.sh`
  - `D-B-run-binary-only-backend-mismatch-block`
  - `tools/smokes/v2/profiles/integration/apps/phase29y_hako_run_binary_only_backend_mismatch_block_vm.sh`
  - `D-B-binary-only-selfhost-readiness-proxy`
  - `tools/smokes/v2/profiles/integration/apps/phase29y_hako_binary_only_selfhost_readiness_vm.sh`
  - `D-B-binary-only-ported`
  - `tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_binary_only_ported_vm.sh`
  - `D-B-mir-shape-guard`
  - `tools/smokes/v2/profiles/integration/apps/mir_shape_guard_vm.sh`
- ring1 promotion（`array/map/path/console`）は `RING1-CORE-09` まで昇格完了（accepted固定）。

## 0.1 Next-Task Doc Boundary (single source)

- `phase-29y` の Next 順序はこの文書だけを正本とする。
- `70/80/81/85` は補助資料であり、Next 順序の上書き権を持たない。
- 新規の next-task 文書は作成せず、この文書に追記する。

## 0.2 Lane Boundary (de-rust A/B/C)

- この文書は lane C（runtime port）専用の next plan。
- lane A（compiler meaning）/ lane B（compiler pipeline）の順序はここでは管理しない。
- lane B の `binary-only --hako-emit-mir-json` 固定順序は `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md` を正本とする。
- A/B/C の単一導線は `docs/development/current/main/design/de-rust-lane-map-ssot.md` を正本とする。

## 0.3 RVP Commit Boundary Lock (active rule)

- active status:
  - RVP open blocker は `none`。
  - fixed backlog は置かず monitor-only で維持する。
- 実装ルール（再発時のみ適用）:
  - `1 blocker = 1 fixture = 1 smoke = 1 commit`。
  - blocked pin -> ported 昇格の順でのみ更新する。
  - matrix row 更新は対象 capability row のみを同コミットで更新する。
- 過去の min-task 単位履歴・境界は `61-NEXT-TASK-HISTORY.md` を正本とする。

## 0.4 Plan Size Rule

- この文書は current/next と受け入れ条件のみ保持する（読み時間 30 秒以内）。
- 完了済みの詳細履歴は `61-NEXT-TASK-HISTORY.md` へ集約する。
- done項目の詳細をここへ再掲しない。

## 0.5 Debug Procedure Lock (lane A/B/C split)

- 目的:
  - 「Rust parser/mirbuilder/vm」と「.hako parser/mirbuilder/vm」の切り分けを毎回同じ手順で行い、場当たり対応を禁止する。
- 手順（固定）:
  1. blocker を fixture + blocked smoke で先に固定する。
  2. 同一入力から MIR を 2 経路で出力する。
     - Rust route: `--emit-mir-json`
     - .hako route: `--hako-emit-mir-json`
  3. MIR canonical compare を実行する（`jq -S .` + `diff -u`）。
  4. lane を確定してから修正する。
     - MIR 差分あり: lane B（parser/mirbuilder）
     - MIR 同一 + 実行差分: lane C（vm-hako runtime）
     - `[joinir/freeze]` / planner reject: lane A（compiler meaning）
- 禁止:
  - lane 未確定のまま複数層へ同時パッチを入れること。
  - blocked pin を作らずに ported smoke へ直接変更すること。

### 0.5.1 Known Parity Debt Lock (Rust parser vs `.hako` parser)

- 現状:
  - expression lowering（nested ternary family）は Rust 側先行で修正される可能性があり、lane B の non-gating debt として保持する。
  - probe fixture（`phase29y_hako_emit_mir_nested_ternary_probe_min.hako`）は strict parity lock 済み。未対応形は fail-fast（`[builder/selfhost-first:unsupported:ternary_no_lower]`）で固定する。
  - lane B の active fixed order は `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md` の
    `Lane-B Nested Ternary Debt Pack (B-TERNARY-01..03)` を正本とする。
- blocker 化トリガー（どれか1つで発火）:
  1. 同一 fixture で `--emit-mir-json` は green かつ `--hako-emit-mir-json` が NG。
  2. MIR canonical compare で lane B 差分が確定（`jq -S .` + `diff -u` で差分あり）。
  3. parser handoff 対象 fixture で `.hako` route が fail-fast（parse/lower）を返す。
- 発火時の固定手順:
  1. lane B blocker を `CURRENT_TASK.md` と本書に同期して起票する。
  2. 先に blocked pin を追加してから修正へ進む（ported 先行禁止）。
  3. 修正後は Rust/.hako 2経路の canonical compare 緑を確認して blocker を閉じる。
- monitor probe（non-gating, default）:
  - `bash tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_nested_ternary_debt_probe_vm.sh`
- blocker trigger check（strict, manual）:
  - `STRICT=1 bash tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_nested_ternary_debt_probe_vm.sh`

## 0.6 Stage1 Module Env Cache Contract (lane B, non-gating)

- 対象: `src/runner/stage1_bridge/modules.rs` の `collect_module_env_lists()`。
- cache path:
  - 既定: `target/.cache/stage1_module_env.json`
  - override: `NYASH_STAGE1_MODULES_CACHE=<path>`
- signature（invalidation key）:
  - root TOML metadata（path, len, mtime）
  - `modules.workspace.members[]`
  - 各 member `*_module.toml` metadata（path, len, mtime）
- invalidation:
  - signature mismatch / cache missing / parse failure で再生成。
- monitor smoke（non-gating）:
  - `bash tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_preemit_io_monitor_vm.sh`
  - strict triage（手動）: `bash tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_preemit_io_monitor_vm.sh --strict`

## 0.7 Ring1 Promotion Commit Boundary Lock (docs-first)

- 目的:
  - ring1 domain 昇格で `min1/min2/min3` を混ぜない。
- boundary（fixed）:
  - `min1`: provider 実装 + runtime wiring のみ。
  - `min2`: fixture + smoke + guard のみ。
  - `min3`: SSOT/README/CURRENT_TASK 同期 + lane gate 統合のみ。
- domain dry-run checklist（正本）:
  - `docs/development/current/main/design/ring1-core-provider-promotion-template-ssot.md`
  - `docs/development/current/main/phases/phase-29y/85-RING1-PROMOTION-DRYRUN-TASK-PACKS.md`
- 現状:
  - `array/map/path/console` は `accepted` へ昇格済み。
  - 追加昇格対象は現時点で `none`。

## 0.8 Lane-B emit-mir timeout diagnostic lock (non-gating)

- 対象 pin:
  - `tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_continue_assignment_timeout_block_vm.sh`
- timeout source SSOT:
  - `src/config/env/selfhost_flags.rs` の `ny_compiler_emit_timeout_ms()`（未指定時は `ny_compiler_timeout_ms()` へフォールバック）
  - `src/runner/stage1_bridge/mod.rs` の `spawn_with_timeout(...)`
- blocked contract:
  - marker: `[stage1-cli] emit-mir: stage1 stub timed out after ... ms`
  - 期待: blocked pin は marker を観測して PASS（non-gating diagnostics）
- repro (debug):
  - `NYASH_STAGE1_EMIT_TIMEOUT_MS=12000 bash tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_continue_assignment_timeout_block_vm.sh`
  - 2026-02-25 の確認では、`12000ms` に引き上げても marker 継続（internal timeout系の既知挙動）。
- promotion boundary:
  - marker が消え、`phase29y_hako_emit_mir_binary_only_ported_vm.sh` が green になった時だけ blocked→ported を昇格する。
  - blocked pin を先に削除しない（順序固定）。

## 1. Next Tasks (fixed order, 1 task = 1 commit)

- next-1: `none`
- 運用: monitor-only（failure-driven）。
- 再開トリガー（どれか1つでも失敗したら blocker 化）:
  - `phase29y_vm_hako_caps_gate_vm.sh`
  - `phase29y_continue_assignment_in_continue_stale_guard_vm.sh`
  - `phase29y_hako_emit_mir_continue_assignment_timeout_block_vm.sh`
  - `phase29y_hako_run_binary_only_ported_vm.sh`
  - `phase29y_hako_binary_only_selfhost_readiness_vm.sh`
  - `phase29y_hako_emit_mir_binary_only_ported_vm.sh`

## 2. Schedule (short)

- latest complete: `RING1-CORE-09-min3`（console accepted 同期）
- next-1: `none`
- full timeline archive:
  - `docs/development/current/main/phases/phase-29y/61-NEXT-TASK-HISTORY.md`

## 3. Ops Rule

- 日次固定:
  - `bash tools/checks/phase29y_derust_blocker_sync_guard.sh`
  - `bash tools/smokes/v2/profiles/integration/apps/phase29y_lane_gate_vm.sh`
- capability matrix 固定:
  - `bash tools/smokes/v2/profiles/integration/apps/phase29y_vm_hako_caps_gate_vm.sh`
- 診断固定（non-gating）:
  - `bash tools/smokes/v2/profiles/integration/apps/phase29y_continue_assignment_in_continue_stale_guard_vm.sh`
  - `bash tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_continue_assignment_timeout_block_vm.sh`
  - `bash tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_preemit_io_monitor_vm.sh`
  - `bash tools/smokes/v2/profiles/integration/apps/phase29y_hako_run_binary_only_ported_vm.sh`
  - `bash tools/smokes/v2/profiles/integration/apps/phase29y_hako_run_binary_only_backend_mismatch_block_vm.sh`
  - `bash tools/smokes/v2/profiles/integration/apps/phase29y_hako_binary_only_selfhost_readiness_vm.sh`
  - `bash tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_binary_only_ported_vm.sh`
  - `bash tools/smokes/v2/profiles/integration/apps/phase29y_hako_emit_mir_nested_ternary_debt_probe_vm.sh`
- push前/週次終端/回帰疑い:
  - `bash tools/smokes/v2/profiles/integration/apps/rc_gc_alignment_g2_fast_milestone_gate.sh`
- FAIL 時:
  - failure-driven 保守へ切替（上限60分）。
  - 60分で復旧しない場合は blocker メモを `CURRENT_TASK.md` に固定。

## 4. Detailed History (do not duplicate here)

- next-task timeline（phase-29y）:
  - `docs/development/current/main/phases/phase-29y/61-NEXT-TASK-HISTORY.md`
- X1-X66 契約履歴:
  - `docs/development/current/main/phases/phase-29x/README.md`
- optional GC queue 履歴:
  - `docs/development/current/main/phases/phase-29y/README.md`

## 5. App-first Pointer

- APP-1 指示書:
  - `docs/development/current/main/phases/phase-29y/70-APP-FIRST-OUTSOURCE-INSTRUCTIONS.md`
- APP-1 実装:
  - `apps/tools/gate_log_summarizer/README.md`
- APP-1 smoke:
  - `tools/smokes/v2/profiles/integration/apps/gate_log_summarizer_vm.sh`
- APP-2 実装:
  - `apps/tools/controlflow_probe/README.md`
- APP-2 smoke:
  - `tools/smokes/v2/profiles/integration/apps/controlflow_probe_vm.sh`
- APP-3 実装:
  - `apps/tools/mir_shape_guard/README.md`
- APP-3 smoke:
  - `tools/smokes/v2/profiles/integration/apps/mir_shape_guard_vm.sh`
