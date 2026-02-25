---
Status: Active
Scope: Phase 29bq `.hako` mirbuilder の stmt handler 抽出レーン（BoxShape, behavior-neutral）
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md
  - docs/development/current/main/phases/phase-29bq/29bq-91-mirbuilder-migration-progress-checklist.md
  - lang/src/compiler/mirbuilder/program_json_v0_phase_state_box.hako
  - lang/src/compiler/mirbuilder/program_json_v0_phase_state_consumer_box.hako
  - lang/src/compiler/mirbuilder/stmt_handlers/print_stmt_handler.hako
  - lang/src/compiler/mirbuilder/stmt_handlers/local_stmt_handler.hako
  - lang/src/compiler/mirbuilder/stmt_handlers/assignment_stmt_handler.hako
  - lang/src/compiler/mirbuilder/stmt_handlers/return_stmt_handler.hako
---

# Phase 29bq — .hako MirBuilder Handler Extraction Backlog (M0-M4)

目的:
- `program_json_v0_phase_state_box.hako` に残る stmt 処理の実体を `stmt_handlers/*` に段階抽出し、責務境界を固定する。
- parser handoff lane とは分離し、1コミットで 1 handler だけ進める。

前提:
- parser handoff Tier-18 が PROMOTE 済みであること。
- `cargo check --bin hakorune` と `phase29bq_fast_gate_vm.sh --only bq` が緑であること。

固定ルール:
- 1コミット = 1 handler = 1 quick verification。
- grammar 拡張禁止（no-try/no-throw 方針を維持）。
- fail-fast tag は `[freeze:contract][hako_mirbuilder]` を維持する。
- fallback を増やさない（挙動不明の吸収禁止）。

## Stub Inventory (current)

- `PrintStmtHandler`: `lang/src/compiler/mirbuilder/stmt_handlers/print_stmt_handler.hako`
- `LocalStmtHandler`: `lang/src/compiler/mirbuilder/stmt_handlers/local_stmt_handler.hako`
- `AssignmentStmtHandler`: `lang/src/compiler/mirbuilder/stmt_handlers/assignment_stmt_handler.hako`
- `ReturnStmtHandler`: `lang/src/compiler/mirbuilder/stmt_handlers/return_stmt_handler.hako`

## Execution Order (must follow)

### M0: Print handler extraction

- [x] `PrintStmtHandler.handle(...)` を STUB から実体化（既存 Print consumer ロジックを再利用して挙動中立で抽出）。
- [x] `program_json_v0_phase_state_consumer_box.hako` の配線を handler 呼び出しへ切替。
- [x] verify:
  - `HAKO_SELFHOST_BUILDER_FIRST=1 HAKO_SELFHOST_NO_DELEGATE=1 bash tools/hakorune_emit_mir.sh apps/tests/phase29bq_selfhost_cleanup_only_min.hako /tmp/phase29bq_cleanup_only_internal.mir.json`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`

### M1: Local handler extraction

- [x] `LocalStmtHandler.handle(...)` を STUB から実体化（既存 Local consumer ロジックを再利用して挙動中立で抽出）。
- [x] `program_json_v0_phase_state_consumer_box.hako` の Local 配線を handler 呼び出しへ切替。
- [x] verify:
  - `HAKO_SELFHOST_BUILDER_FIRST=1 HAKO_SELFHOST_NO_DELEGATE=1 bash tools/hakorune_emit_mir.sh apps/tests/phase29bq_selfhost_cleanup_only_min.hako /tmp/phase29bq_cleanup_only_internal.mir.json`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`

### M2: Assignment handler extraction

- [x] `AssignmentStmtHandler.handle(...)` を STUB から実体化（既存 Assignment consumer ロジックを再利用して挙動中立で抽出）。
- [x] `program_json_v0_phase_state_consumer_box.hako` の Assignment 配線を handler 呼び出しへ切替。
- [x] verify:
  - `HAKO_SELFHOST_BUILDER_FIRST=1 HAKO_SELFHOST_NO_DELEGATE=1 bash tools/hakorune_emit_mir.sh apps/tests/phase29bq_selfhost_cleanup_only_min.hako /tmp/phase29bq_cleanup_only_internal.mir.json`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`

### M3: Return handler extraction

- [x] `ReturnStmtHandler.handle(...)` を STUB から実体化（既存 Return consumer ロジックを再利用して挙動中立で抽出）。
- [x] `program_json_v0_phase_state_consumer_box.hako` の Return 配線を handler 呼び出しへ切替。
- [x] verify:
  - `HAKO_SELFHOST_BUILDER_FIRST=1 HAKO_SELFHOST_NO_DELEGATE=1 bash tools/hakorune_emit_mir.sh apps/tests/phase29bq_selfhost_cleanup_only_min.hako /tmp/phase29bq_cleanup_only_internal.mir.json`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`

### M4: Inline residue cleanup

- [x] `program_json_v0_phase_state_box.hako` 側の旧 inline 分岐残骸を最小差分で縮退（`_consume_stmt_in_body` pass-through を撤去し、consumer SSOT へ直結）。
- [x] `lang/src/compiler/mirbuilder/README.md` と `29bq-91` の進捗表記を同期。
- [x] verify:
  - `HAKO_SELFHOST_BUILDER_FIRST=1 HAKO_SELFHOST_NO_DELEGATE=1 bash tools/hakorune_emit_mir.sh lang/src/runner/stage1_cli.hako /tmp/stage1_cli_fullbuilder.mir.json`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`

## Done Definition (this backlog)

- M0-M4 が順序通りに完了し、全コミットで quick verify が緑。
- `stmt_handlers/*` が STUB ではなく実体を持ち、phase state 入口が handler 経由で一貫する。
