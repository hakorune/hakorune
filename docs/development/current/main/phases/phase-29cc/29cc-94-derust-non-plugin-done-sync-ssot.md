---
Status: Active
Decision: accepted
Date: 2026-02-25
Scope: phase-29cc（non-plugin residue lane）の de-rust done 宣言を docs/evidence で固定する。
Related:
  - docs/development/current/main/design/de-rust-master-task-map-ssot.md
  - docs/development/current/main/design/de-rust-scope-decision-ssot.md
  - docs/development/current/main/phases/phase-29cc/README.md
  - docs/development/current/main/phases/phase-29cc/29cc-90-migration-execution-checklist.md
  - docs/development/current/main/phases/phase-29cc/29cc-92-non-plugin-rust-residue-task-set.md
  - docs/development/current/main/phases/phase-29cf/README.md
  - docs/development/current/main/phases/phase-29x/29x-62-derust-done-sync-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-45-rust-optional-done-sync-ssot.md
  - CURRENT_TASK.md
---

# Phase 29cc-94: De-Rust Non-Plugin Done Sync SSOT

## 0. Goal

phase-29cc（non-plugin residue lane）の done 宣言を 1 枚で固定し、
「monitor-only 継続」と「done 判定」を混同しない運用にする。

## 1. Done Criteria (29cc closeout)

次を全て満たした時に、phase-29cc non-plugin lane を done とする。

1. `29cc-92` fixed order（RNR-01..RNR-05）が完了している。
2. lane A/B/C の current blocker が `none`（monitor-only）である。
3. `phase29x_derust_done_matrix_vm.sh`（X32-X35 replay）が PASS。
4. `check_phase29x_x23_readiness.sh --strict` が `status=READY`。
5. done scope が `de-rust-scope-decision-ssot.md`（non-plugin done）で accepted。

## 2. Evidence Replay (2026-02-25)

1. `bash tools/smokes/v2/profiles/integration/apps/phase29x_derust_done_matrix_vm.sh`
   - PASS
2. `tools/selfhost/check_phase29x_x23_readiness.sh --strict`
   - `status=READY`
3. `cargo check --bin hakorune`
   - PASS
4. `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`
   - PASS

## 3. Operational Contract after Done

1. phase-29cc は `monitor-only` を維持し、fixed backlog は置かない。
2. reopen は failure-driven（gate FAIL 時のみ）で行う。
3. plugin 実装の全面 `.hako` 置換は separate lane として扱う（done 条件外）。
4. `VM fallback compat lane` / `bootstrap boundary reduction` の post-closeout follow-up は `phase-29cf` accepted monitor-only で扱い、この done 判定は再オープンしない。

## 4. Reopen Rule (failure-driven)

次のいずれかで phase-29cc blocker を reopen する。

1. `phase29x_derust_done_matrix_vm.sh` が FAIL。
2. `check_phase29x_x23_readiness.sh --strict` が non-READY。
3. `phase29bq_fast_gate_vm.sh --only bq` が FAIL（lane A 連動）。
4. scope decision が provisional/rejected へ変更された場合。

## 5. Decision

Decision: accepted

- phase-29cc non-plugin residue lane は done 宣言済み。
- 次段階は monitor-only 運用を継続し、failure-driven reopen のみ許可する。
- plugin 移植は separate lane（`29cc-95-plugin-lane-bootstrap-ssot.md`）で扱う。
