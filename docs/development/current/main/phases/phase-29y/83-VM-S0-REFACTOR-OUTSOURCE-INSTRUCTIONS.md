---
Status: Active
Decision: provisional
Date: 2026-02-18
Scope: `mir_vm_s0.hako` など 1000行超ファイルを BoxShape 方針で分割し、挙動不変で保守性を上げるための外部AI向け固定指示書。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-29y/60-NEXT-TASK-PLAN.md
  - docs/development/current/main/phases/phase-29y/81-RUST-VM-TO-HAKO-VM-FEATURE-MATRIX.md
  - docs/development/current/main/phases/phase-29y/82-VM-HAKO-BOXCALL-CONTRACT-SSOT.md
  - docs/development/current/main/design/compiler-expressivity-first-policy.md
---

# VM S0 Refactor Outsource Instructions (BoxShape, behavior-preserving)

## 0. Goal

- `lang/src/vm/boxes/mir_vm_s0.hako` の責務混在を解消し、導線を薄くする。
- `mir_vm_s0.hako` は facade-only を目標にし、const lookup helpers のような局所ロジックは helper box 側へ寄せる。
- 機能追加ではなく、**挙動不変の構造改善**だけを行う。
- `.hako VM` の現在契約（RVP-C14 ported / C15 blocked）を壊さない。

## 1. Task Type (fixed)

- タスク種別は **BoxShape**（責務分割・入口集約・SSOT化）。
- **BoxCount（新しい受理形追加）を混ぜない**。
- refactor series は許可するが、シリーズ全体で目的は 1 つだけに固定する。

## 2. Target Scope (must keep)

- 主対象:
  - `lang/src/vm/boxes/mir_vm_s0.hako`
- 補助対象:
  - `lang/src/vm/boxes/mir_vm_s0_*.hako`（新規分割先）
  - `lang/src/vm/hako_module.toml`（モジュール配線）
  - 必要最小限の docs（本指示書と整合する範囲）

禁止:
- `src/**` Rust コードの仕様変更
- capability row の意味変更（blocked/ported の意味を書き換えない）
- fallback 追加、silent pass 追加
- smoke の expected 契約を都合で緩める変更

## 3. Required Deliverables

1. `mir_vm_s0.hako` を「薄い入口」にする
   - 目安: `run` と top-level dispatch、最小の glue だけ残す
2. 責務分離された helper box を追加する
   - 最低 3 分割（例: `*_json_scan`, `*_value_ops`, `*_block_runner`）
3. 各 helper の先頭に Responsibility コメントを追加する
4. `hako_module.toml` の配線を更新し、`using selfhost.vm.helpers.*` を壊さない
5. `mini_vm_s0_entry.hako` を thin stable entry として保ち、`run_min` を helper runner 側へ寄せる
6. 既存 gate/smoke が全て PASS する状態で提出する

## 4. Contract (behavior lock)

次は必ず維持すること:

- C14 ported 契約:
  - `vm_hako_caps_app1_summary_contract_ported_vm.sh` が PASS
- C15 blocked 契約:
  - `vm_hako_caps_app1_summary_contract_block_vm.sh` が PASS（full fixture timeout blocker）
- vm-hako capability gate:
  - `phase29y_vm_hako_caps_gate_vm.sh` が PASS
- mainline/lane gate:
  - `phase29y_no_compat_mainline_vm.sh` が PASS
  - `phase29y_lane_gate_vm.sh` が PASS

追加ルール:
- 出力タグ（`[vm-hako/contract]`, `[vm-hako/unimplemented]`）の文言をむやみに変えない
- debug 出力を増やす場合は既定OFFで、既存契約を壊さない

## 5. Implementation Order (recommended)

1. `mir_vm_s0.hako` の責務棚卸しをコメントで固定
2. 純粋ヘルパから順に分離
   - 文字列/JSON scan
   - 値/型/handle演算
   - block walk/branch/jump/phi
3. call/externcall/boxcall の実行層を分離
   - `mir_vm_s0_exec_dispatch.hako`
4. block payload traversal / cache assembly / run orchestration を分離
   - `mir_vm_s0_block_runner.hako`
5. 最後に入口ファイルを薄くして配線整理

## 6. Commit Policy

- 原則: 1コミット1目的
- Refactor Series Mode を使う場合:
  - 2〜5コミット程度に分割
  - 各コミットでビルド可能
  - 挙動変更は入れない

## 7. Acceptance Commands (must report)

1. `bash tools/smokes/v2/profiles/integration/apps/vm_hako_caps_app1_summary_contract_ported_vm.sh`
2. `bash tools/smokes/v2/profiles/integration/apps/archive/vm_hako_caps_app1_summary_contract_block_vm.sh`
3. `bash tools/smokes/v2/profiles/integration/apps/phase29y_vm_hako_caps_gate_vm.sh`
4. `bash tools/smokes/v2/profiles/integration/apps/phase29y_no_compat_mainline_vm.sh`
5. `bash tools/smokes/v2/profiles/integration/apps/phase29y_lane_gate_vm.sh`
6. `bash tools/checks/phase29y_derust_blocker_sync_guard.sh`

## 8. Reviewer Checklist

- `mir_vm_s0.hako` が facade-only になっている
- const lookup helpers が `mir_vm_s0_exec_dispatch.hako` 側へ寄っている
- `mini_vm_s0_entry.hako` が薄い entry wrapper になっている
- 新規 helper が責務ごとに分割され、重複ロジックが減っている
- 受理形や契約文言を変えていない
- acceptance 6コマンドが PASS
- 変更範囲が runtime lane C（`.hako VM`）から逸脱していない

## 9. Copy-Paste Request (for external AI)

```text
Implement BoxShape refactor from:
docs/development/current/main/phases/phase-29y/83-VM-S0-REFACTOR-OUTSOURCE-INSTRUCTIONS.md

Must follow exactly:
- Behavior-preserving refactor only (no new capability, no fallback)
- Split mir_vm_s0.hako into helper boxes with clear responsibilities
- Keep C14 ported + C15 blocked contracts unchanged
- Run and report all 6 acceptance commands
```
