---
Status: Active
Scope: planner-required master list を selfhost 入口へ接続（.hako 側の導線準備）
Related:
- docs/development/current/main/10-Now.md
- docs/development/current/main/30-Backlog.md
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
- docs/development/current/main/design/joinir-planner-required-gates-ssot.md
- docs/development/current/main/phases/phase-29bp/README.md
---

# Phase 29bq: planner-required → selfhost entry wiring

## Goal

- planner-required（strict/dev）を「selfhost 側からも 1 コマンドで回せる入口」に接続する。
- 既定挙動は不変（release default unchanged）。
- 既存 gate（29ae regression / planner-required v4）は緑維持。

## Non-goals

- 新しい language feature の追加
- by-name 分岐
- silent fallback の復活

## Plan (P0-P3)

- P0: docs-first（入口/SSOT の 1 枚化）
- P1: selfhost 入口（スクリプト/runner）設計と SSOT 固定
- P2: 実装（入口追加）→ 既存 gate green 維持
- P3: closeout（post-change green + SSOT 更新）

## Planned next (after 29bq)

- CorePlan cleanup after “Loop as structural box”（優先: CleanupWrap / Select / `loop(true|cond)` の小箱で混線を減らす）

## Gate (SSOT)

- Loopless subset: `./tools/hako_check_loopless_gate.sh`
- Planner-required dev gate v4: `./tools/smokes/v2/profiles/integration/joinir/phase29bp_planner_required_dev_gate_v4_vm.sh`
- JoinIR regression pack: `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
- Fast iteration (29bq): `./tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh`
- Selfhost planner-required entry: `SMOKES_ENABLE_SELFHOST=1 ./tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh`
- Baseline (implementation safety): `./tools/smokes/v2/profiles/integration/joinir/phase29bs_fast_gate_vm.sh --full`
- Loop(true) multi-break gate: `./tools/smokes/v2/profiles/integration/joinir/phase29bq_loop_true_multi_break_planner_required_vm.sh`
- Conditional update/join gate: `./tools/smokes/v2/profiles/integration/joinir/phase29bq_conditional_update_join_planner_required_vm.sh`
- Loop(cond) multi-exit gate: `./tools/smokes/v2/profiles/integration/joinir/phase29bq_loop_cond_multi_exit_planner_required_vm.sh`

## Acceptance criteria (RC)

- selfhost 入口（P2 で追加）を 1 コマンドで実行でき、RC=0 で通る（ログ導線あり）。
- subset TSV を 1 → 3 本まで段階的に増やして PASS（freeze が出たら増加停止し、README に LOG を固定）。
- master TSV は opt-in で少数だけ流せる（全件強制はしない）。
- `phase29bp_planner_required_dev_gate_v4_vm.sh` / `phase29ae_regression_pack_vm.sh` / `hako_check_loopless_gate.sh` が緑維持。

## Selfhost entry (SSOT)

- Entry command: `SMOKES_ENABLE_SELFHOST=1 ./tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh`
- Baseline check: `./tools/smokes/v2/profiles/integration/joinir/phase29bs_fast_gate_vm.sh --full`
- Opt-in: `SMOKES_ENABLE_SELFHOST=1` がない場合は SKIP（RC=0）。
- Log contract: 失敗時は最後に `LOG: /tmp/phase29bq_selfhost_<case>.log` を出力する。
- Execution path: `compiler.hako --stage-b --stage3` で Program(JSON v0) を生成し、`--json-file` で実行する（strict/dev + planner-required）。
- Default list: `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv`（selfhost 実行可能な subset）
- Master list: `SMOKES_SELFHOST_LIST=tools/smokes/v2/profiles/integration/joinir/planner_required_cases.tsv` で指定する。

## LoopTrueBreak gate (P2)

- Fixture: `apps/tests/phase29bq_loop_true_multi_break_parserish_min.hako`
- Tag: `[joinir/planner_first rule=LoopTrueBreak]`
- Note: `loop(true)` の break/continue 受理は **fallback-only**（他候補がある場合は既存パターンを優先し、競合で freeze しない）

## Conditional update/join gate (P2)

- Fixture: `apps/tests/phase29bq_conditional_update_join_min.hako`
- Tag: `[joinir/planner_first rule=Pattern1]`

## Loop(cond) multi-exit gate (P2)

- Fixture: `apps/tests/phase29bq_loop_cond_multi_exit_min.hako`
- Tag: `[joinir/planner_first rule=LoopCondBreak]`
- Note: break-only を既存SSOTとして維持しつつ、break+continue 混在も保守的に受理（continue-only は拒否）

## P2 blocker (current)

- Blocker: Stage-B compile が planner-required freeze（Loop の受理形不足）
  - Error: `[plan/freeze:unsupported] generic loop v0.2: control flow after in-body step`
  - Function: `ParserStringScanBox.scan_with_quote/3`（stage-b のログより）
  - Trigger shape: `loop(cond)` で `loop_increment` が body の末尾ではなく、かつ step の後ろに `exit-if`（例: `if (...) { break }`）が続く形
  - Why freeze: generic loop v0/v1 は「loop_increment を抽出して step_bb へ移動」前提のため、この形をそのまま受理すると意味論が変わり得る（AST rewrite 禁止のため fail-fast している）
- LOG: `/tmp/phase29bq_selfhost_phase118_pattern3_if_sum_min.hako.log`
- Repro: `SMOKES_ENABLE_SELFHOST=1 ./tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh` (rc=1)
- Next (compiler-side, no .hako workaround):
  - Facts に `StepPlacement` を保持し、「step を body に残す」専用の plan/lowerer（LoopFrame v1）を strict/dev + planner_required 限定で追加する
  - 最小 fixture + joinir gate を追加して契約固定 → selfhost gate を再実行して unblock を確認する
