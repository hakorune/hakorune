---
Status: Active
Scope: Phase 29bq `.hako` mirbuilder の Post-M4 整理レーン（BoxShape, behavior-neutral）
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-29bq/29bq-91-mirbuilder-migration-progress-checklist.md
  - docs/development/current/main/phases/phase-29bq/29bq-109-hako-mirbuilder-handler-extraction-backlog.md
  - lang/src/compiler/mirbuilder/program_json_v0_phase_state_consumer_box.hako
  - lang/src/compiler/mirbuilder/stmt_handlers/print_stmt_handler.hako
  - lang/src/compiler/mirbuilder/stmt_handlers/local_stmt_handler.hako
  - lang/src/compiler/mirbuilder/stmt_handlers/assignment_stmt_handler.hako
  - lang/src/compiler/mirbuilder/stmt_handlers/return_stmt_handler.hako
---

# Phase 29bq — `.hako` MirBuilder Post-M4 Lane (P1-P5)

目的:
- M0-M4 で固定した「phase_state_box は scan/order、stmt 処理は handler/consumer」境界を、次段でさらに薄くする。
- 1コミットで 1 handler の責務移管だけを進め、挙動差分なしで consumer 層を縮退する。

固定ルール:
- 1コミット = 1項目（P1-P5 のいずれか 1つ）+ quick verify。
- grammar 拡張禁止（no-try/no-throw 方針維持）。
- fail-fast tag は `[freeze:contract][hako_mirbuilder]` を維持。
- parser handoff lane と混ぜない（Post-M4 lane 専用コミット）。

## Execution Order (must follow)

### P1: Print logic ownership move

- [x] `PrintStmtHandler` が Print 実装を直接持つ（`ProgramJsonV0ConsumerPrintBox` への単純委譲を解消）。
- [x] verify:
  - `HAKO_SELFHOST_BUILDER_FIRST=1 HAKO_SELFHOST_NO_DELEGATE=1 bash tools/hakorune_emit_mir.sh apps/tests/phase29bq_selfhost_cleanup_only_min.hako /tmp/phase29bq_cleanup_only_internal.mir.json`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`

### P2: Local logic ownership move

- [x] `LocalStmtHandler` が Local 実装を直接持つ（`ProgramJsonV0ConsumerLocalBox` への単純委譲を解消）。
- [x] verify:
  - `HAKO_SELFHOST_BUILDER_FIRST=1 HAKO_SELFHOST_NO_DELEGATE=1 bash tools/hakorune_emit_mir.sh apps/tests/phase29bq_selfhost_cleanup_only_min.hako /tmp/phase29bq_cleanup_only_internal.mir.json`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`

### P3: Assignment logic ownership move

- [x] `AssignmentStmtHandler` が Assignment 実装を直接持つ（`ProgramJsonV0ConsumerAssignmentBox` への単純委譲を解消）。
- [x] verify:
  - `HAKO_SELFHOST_BUILDER_FIRST=1 HAKO_SELFHOST_NO_DELEGATE=1 bash tools/hakorune_emit_mir.sh apps/tests/phase29bq_selfhost_cleanup_only_min.hako /tmp/phase29bq_cleanup_only_internal.mir.json`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`

### P4: Return logic ownership move

- [x] `ReturnStmtHandler` が Return 実装を直接持つ（`ProgramJsonV0ConsumerReturnBox` への単純委譲を解消）。
- [x] verify:
  - `HAKO_SELFHOST_BUILDER_FIRST=1 HAKO_SELFHOST_NO_DELEGATE=1 bash tools/hakorune_emit_mir.sh apps/tests/phase29bq_selfhost_cleanup_only_min.hako /tmp/phase29bq_cleanup_only_internal.mir.json`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`

### P5: Consumer layer residue cleanup

- [x] `program_json_v0_consumer_*_box.hako` の残骸を削除（参照ゼロを確認して撤去）。
- [x] `lang/src/compiler/mirbuilder/README.md` / `29bq-91` / `CURRENT_TASK.md` を同期。
- [x] verify:
  - `HAKO_SELFHOST_BUILDER_FIRST=1 HAKO_SELFHOST_NO_DELEGATE=1 bash tools/hakorune_emit_mir.sh lang/src/runner/stage1_cli.hako /tmp/stage1_cli_fullbuilder.mir.json`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`

## Done Definition

- P1-P5 を順序通り完了。
- `phase_state_box -> phase_state_consumer -> stmt_handler` の導線で、stmt 実装の主責務が handler 側に集約される。
- quick verify（internal-only + bq fast gate）が各コミットで緑。
