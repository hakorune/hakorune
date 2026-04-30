---
Status: Complete
Scope: Selfhost entry — hako_check (a.k.a. hack_check) bringup
Related:
- docs/development/current/main/phases/phase-29bf/SELFHOST_HANDOFF.md
- docs/development/current/main/design/selfhost-coreplan-unblocking-policy.md
- docs/development/current/main/design/selfhost-tools-loopless-subset-ssot.md
- docs/development/current/main/design/COREPLAN_GENERAL_LOOP_DECOMPOSITION_INQUIRY.md
- docs/development/current/main/design/coreplan-unknown-loop-strategy-ssot.md
- CLAUDE.md
- tools/hako_check/README.md
---

# Phase 29bg: Selfhost entry — hako_check bringup

## Goal

セルフホストへ戻る前に、`.hako` 品質チェック（`hako_check` / 旧称 `hack_check`）の導線を **確実に動く状態**で固定する。

## Non-goals

- 新しいルール追加（rules/*.hako の拡張）
- selfhost コンパイラの大規模改修
- 新しい env var の追加

## Plan

- P0: Run hako_check gate（docs-first）✅
- P1: Unblock hako_check (StringHelpers loops)（compiler change）✅（JoinIR gate greenのため対応不要）
- P2: Make hako_check loopless（tooling refactor）✅
- P3: closeout（docs-only）✅

## Instructions

- P0: `docs/development/current/main/phases/phase-29bg/P0-RUN-HAKO_CHECK-GATE-INSTRUCTIONS.md`
- P1: `docs/development/current/main/phases/phase-29bg/P1-UNBLOCK-HAKO_CHECK-STRINGHELPERS-LOOPS-INSTRUCTIONS.md`
- P2: `docs/development/current/main/phases/phase-29bg/P2-MAKE-HAKO_CHECK-LOOPLESS-INSTRUCTIONS.md`

## Gate (SSOT)

- `./tools/hako_check_loopless_gate.sh` (P2 gate wrapper)
- `./tools/hako_check/deadcode_smoke.sh`
- `bash tools/hako_check/run_tests.sh`

Default entry: `./tools/hako_check_loopless_gate.sh` (use `--only {quick|joinir|deadcode|run_tests}` for reruns)

## Gate expectations (SSOT)

| Command | RC | Notes |
| --- | --- | --- |
| `./tools/hako_check_loopless_gate.sh` | 0 | quick + JoinIR gate + deadcode + run_tests |
| `./tools/hako_check/deadcode_smoke.sh` | 0 | selfhost 前の最小ゲート |
| `bash tools/hako_check/run_tests.sh` | 0 | JSON 出力は filter 済み |

## Deadcode smoke contract

| Item | Expectation |
| --- | --- |
| HC019 detection | “unreachable method” と “dead static box” を必ず検出 |
| False positives | clean fixture で HC019 が出ない |
| Output | JSON-LSP 形式で安定（run_tests と同じ filter 方針） |
| Forbidden | flowbox タグ / plan freeze を出さない（release 既定のまま） |

## strict/dev failure classification (flowbox/freeze)

| code | Meaning | Next CorePlan part |
| --- | --- | --- |
| `planner_none` | Facts/Planner が対象外 | restricted-loop 正規化（tooling 側） |
| `composer_reject` | 合成入口で拒否 | generic loop v0.x の境界拡張 |
| `unstructured` | skeleton が一意化できない | loop 分解 / nested 排除 |
| `unsupported` | 語彙不足 | ExitIf/IfEffect の最小語彙追加 |

注: strict/dev では `flowbox/freeze` が出ることがある。release は挙動/ログ不変。
