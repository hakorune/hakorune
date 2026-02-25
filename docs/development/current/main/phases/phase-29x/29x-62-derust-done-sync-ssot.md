---
Status: Active
Decision: accepted
Date: 2026-02-25
Scope: Phase 29x X36 de-rust done 同期（完了判定 / rollback 条件 / 証跡リンク固定）。
Related:
  - docs/development/current/main/design/de-rust-master-task-map-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-58-derust-route-orchestrator-skeleton-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-59-derust-verifier-path-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-60-derust-safety-path-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-61-derust-strict-default-route-cutover-ssot.md
  - docs/development/current/main/design/de-rust-lane-map-ssot.md
  - docs/development/current/main/phases/phase-29y/50-LANE-GATE-SSOT.md
  - docs/development/current/main/design/joinir-planner-required-gates-ssot.md
  - docs/development/current/main/phases/phase-29bq/README.md
---

# Phase 29x X36: De-Rust Done Sync SSOT

## 0. Goal

X32-X35 で導入した `.hako` route/verifier/safety 契約を
「done 判定 + rollback 条件 + evidence 導線」として 1 枚に同期する。

## 1. Done Criteria (X32-X36)

次の 4 条件を満たすと、de-rust transfer lane を done とする。

1. X32: route selection parity が dual-run で一致
2. X33: verifier mismatch が fail-fast で停止
3. X34: lifecycle violation が fail-fast で停止
4. X35: strict/dev default route が `source=hako-skeleton`、Rust thin は明示時のみ

### 1.1 Done Criteria × Gate Coverage Matrix (operational SSOT)

| criterion | canonical evidence smoke | quick/fast gate wiring | gap status |
| --- | --- | --- | --- |
| X32 route dual-run parity | `phase29x_derust_route_dualrun_vm.sh` | `phase29x_derust_done_matrix_vm.sh`（manual + `dev_gate milestone-runtime` + `PHASE29Y_DERUST_DONE_MATRIX_CHECK=1`） | open（quick既定未統合） |
| X33 verifier fail-fast | `phase29x_derust_verifier_vm.sh` | `phase29x_derust_done_matrix_vm.sh`（manual + `dev_gate milestone-runtime` + `PHASE29Y_DERUST_DONE_MATRIX_CHECK=1`） | open（quick既定未統合） |
| X34 safety fail-fast | `phase29x_derust_safety_vm.sh` | `phase29x_derust_done_matrix_vm.sh`（manual + `dev_gate milestone-runtime` + `PHASE29Y_DERUST_DONE_MATRIX_CHECK=1`） | open（quick既定未統合） |
| X35 strict/dev default route | `phase29x_derust_strict_default_route_vm.sh` | `phase29x_derust_done_matrix_vm.sh`（manual + `dev_gate milestone-runtime` + `PHASE29Y_DERUST_DONE_MATRIX_CHECK=1`） | open（quick既定未統合） |

運用ルール:
- Done 宣言は上表 4 smoke を replay してから行う。
- `phase29bq` / `phase29y` / planner-required gate が緑でも、上表が未実行なら done 判定に使わない。
- quick/fast へ統合されるまで、`phase29x_derust_done_matrix_vm.sh` を manual evidence entry として維持する。
- `PHASE29Y_DERUST_DONE_MATRIX_CHECK=1` は quick への診断付加であり、daily quick の既定契約には含めない。
- de-rust 全体完了判定は `de-rust-master-task-map-ssot.md` の Completion Ladder（L4/L5）と同期する。

## 2. Rollback / Escape Hatches

段階切替中の rollback 条件は以下で固定する。

1. strict/dev で Rust thin に戻す: `NYASH_VM_HAKO_PREFER_STRICT_DEV=0`
2. compat fallback を明示許可: `NYASH_VM_USE_FALLBACK=1`
3. vm-hako lane を明示実行: `--backend vm-hako`

注記:
- 既定は strict/dev で `.hako` route 契約を使う。
- rollback は fail-fast の診断を崩さない範囲で明示時のみ許可する。

## 3. Evidence Commands

1. `cargo check -q --bin hakorune`
2. `bash tools/smokes/v2/profiles/integration/apps/phase29x_derust_route_dualrun_vm.sh`
3. `bash tools/smokes/v2/profiles/integration/apps/phase29x_derust_verifier_vm.sh`
4. `bash tools/smokes/v2/profiles/integration/apps/phase29x_derust_safety_vm.sh`
5. `bash tools/smokes/v2/profiles/integration/apps/phase29x_derust_strict_default_route_vm.sh`
6. `bash tools/smokes/v2/profiles/integration/apps/phase29x_derust_done_matrix_vm.sh`

### 3.1 Wiring Targets (docs-first)

次の SSOT に同じ matrix を参照する導線を追加し、done 判定の分散を防ぐ。

1. `docs/development/current/main/design/de-rust-lane-map-ssot.md`
   - lane A/B/C scoreboard へ X32-X35 matrix 参照を追加。
2. `docs/development/current/main/phases/phase-29y/50-LANE-GATE-SSOT.md`
   - lane gate quick/full は de-rust done と別判定であることを明記し、X32-X35 replay 手順を追記。
3. `docs/development/current/main/design/joinir-planner-required-gates-ssot.md`
   - planner-required gate は de-rust done の代替ではないことを明記し、post-check に X32-X35 を要求。
4. `docs/development/current/main/phases/phase-29bq/README.md`
   - selfhost/planner-required gate の closeout で X32-X35 を handshake として replay する導線を追記。

## 4. X36 Decision

Decision: accepted

- de-rust transfer lane（X32-X36）は docs + evidence で同期済み。
- 次レーンは X37（LLVM-only retirement lane 入口）へ進む。

## 5. Latest Replay Evidence (2026-02-25)

1. `bash tools/smokes/v2/profiles/integration/apps/phase29x_derust_done_matrix_vm.sh`
   - PASS（X32/X33/X34/X35 replay）
2. `tools/selfhost/check_phase29x_x23_readiness.sh --strict`
   - `status=READY`
