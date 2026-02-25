---
Status: Active
Decision: provisional
Date: 2026-02-13
Scope: Phase 29x X22（3日連続 gate green）の証跡台帳。
Related:
  - docs/development/current/main/phases/phase-29x/README.md
  - docs/development/current/main/phases/phase-29x/29x-90-integration-checklist.md
  - docs/development/current/main/phases/phase-29x/29x-91-task-board.md
  - tools/smokes/v2/profiles/integration/apps/phase29x_vm_route_observability_vm.sh
  - tools/smokes/v2/profiles/integration/apps/phase29x_vm_route_strict_dev_priority_vm.sh
  - tools/smokes/v2/profiles/integration/selfhost/phase29x_vm_route_non_strict_compat_boundary_vm.sh
  - tools/selfhost/run.sh
---

# Phase 29x X22: Three-Day Gate Evidence

## 0. Goal

X22 は `vm-route` lane の主要 gate を 3日連続で green にし、
cutover 後の運用安定性を証跡として固定する。

## 1. Gate Set (fixed)

1. `cargo check -q --bin hakorune`
2. `bash tools/smokes/v2/profiles/integration/apps/phase29x_vm_route_observability_vm.sh`
3. `bash tools/smokes/v2/profiles/integration/apps/phase29x_vm_route_strict_dev_priority_vm.sh`
4. `bash tools/smokes/v2/profiles/integration/selfhost/phase29x_vm_route_non_strict_compat_boundary_vm.sh`
5. `./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4`

実行補助:
- `tools/selfhost/record_phase29x_x22_evidence.sh <day> [YYYY-MM-DD]`
  - gate set を連続実行し、表に貼れる Markdown 1 行を出力する。
- `tools/selfhost/check_phase29x_x22_evidence.sh [--strict]`
  - 表の進捗/品質を検査する（`--strict` は Day1-3 PASS + 日付一意/昇順を必須化）。

補足:
- selfhost gate は strict/dev でも Rust VM core lane を使うため、
  `NYASH_VM_HAKO_PREFER_STRICT_DEV=0` を Stage-B gate wrapper 側で固定して実行する。

## 2. Daily Evidence Table

| Day | Date | Result | Notes |
| --- | --- | --- | --- |
| 1 | 2026-02-13 | PASS | route 3 smoke + 5-case selfhost gate PASS (`stageb_total_secs=18`, `avg_case_secs=3.60`) |
| 2 | 2026-02-14 | PASS | route 3 smoke + 5-case selfhost gate PASS (`stageb_total_secs=15`, `avg_case_secs=3.00`) |
| 3 | 2026-02-15 | PASS | route 3 smoke + 5-case selfhost gate PASS (`stageb_total_secs=15`, `avg_case_secs=3.00`) |

## 3. Completion Rule

- Day 1-3 が連続で PASS した時点で X22 を done とする。
- 途中で FAIL した場合は連続カウントをリセットし、原因を `CURRENT_TASK.md` に記録して再開する。
