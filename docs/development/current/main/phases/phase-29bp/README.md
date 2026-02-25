---
Status: Complete
Scope: planner-required master list + dev gate v4
Related:
- docs/development/current/main/10-Now.md
- docs/development/current/main/30-Backlog.md
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
- docs/development/current/main/design/joinir-planner-required-gates-ssot.md
- docs/development/current/main/phases/phase-29bo/README.md
- docs/development/current/main/phases/phase-29bn/README.md
---

# Phase 29bp: planner-required master list + dev gate v4

## Goal

- planner-required cases を **1つのTSV（master list）** に統合し、dev gate を v4 として一本化する。
- 既定挙動は不変（release default unchanged）。
- JoinIR regression pack と dev gate を常に緑維持。

## Non-goals

- by-name 分岐
- silent fallback の復活
- 新しい language feature の追加

## Plan (P0-P3)

- P0: docs-first（phase doc + master list SSOT）
- P1: master list の設計（列/注釈/ルール）
- P2: master list を導入、dev gate v4 に接続
- P3: closeout（post-change green + SSOT更新）

## Master list (SSOT)

- Path: `tools/smokes/v2/profiles/integration/joinir/planner_required_cases.tsv`
- Columns: `fixture` / `expected` / `allow_rc` / `planner_tag` / `reason`
- `__EMPTY__` は空stdoutのSSOTとして維持する

## Gate (SSOT)

- `./tools/hako_check_loopless_gate.sh`
- `./tools/smokes/v2/profiles/integration/joinir/phase29bp_planner_required_dev_gate_v4_vm.sh`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Acceptance criteria (RC)

- `./tools/hako_check_loopless_gate.sh` -> RC=0
- `./tools/smokes/v2/profiles/integration/joinir/phase29bp_planner_required_dev_gate_v4_vm.sh` -> RC=0
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` -> RC=0

## Policy

- master list は TSV で SSOT 化する（stdout SSOT + allow_rc）。
- 既存の pattern別 pack gate は **削除せず**、互換のため残す。
- provider/box/method の候補選択は順序非依存（priority 同点は lib 名の字句順で決定）。
- 観測ログは `NYASH_DEV_PROVIDER_TRACE=1` で出す（dev-only）。

## P2 blocker (resolved)

- ✅ Resolved: `phase29aq_string_parse_integer_sign_min_vm` の `-42` → `0` flake は、legacy mode の function lowering で `type_ctx` が関数境界を跨いで漏れて ValueId が衝突し、callee 解決が崩れる問題が原因だった。
- Fix: function lowering（static/instance）で legacy mode のとき `type_ctx`（`value_types`/`value_kinds`/`value_origin_newbox`）を save/restore して関数スコープを分離（`src/mir/builder/calls/lowering.rs`）。
- Verified: `./tools/smokes/v2/profiles/integration/joinir/phase29bp_planner_required_dev_gate_v4_vm.sh` / `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` / `./tools/hako_check_loopless_gate.sh` が PASS。

## Status note

- post-change green: `./tools/smokes/v2/profiles/integration/joinir/phase29bp_planner_required_dev_gate_v4_vm.sh` + `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` + `./tools/hako_check_loopless_gate.sh`
