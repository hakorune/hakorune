---
Status: Active
Decision: accepted
Date: 2026-02-13
Scope: X57 NewClosure runtime lane decision（execute/fail-fast 境界）を再確認し、runtime fail-fast 継続を固定する。
Related:
  - docs/development/current/main/phases/phase-29x/29x-77-newclosure-contract-lock-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-80-postx53-runtime-core-sequencing-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-82-vm-hako-s6-dual-run-parity-gate-pack-ssot.md
  - tools/checks/phase29x_vm_hako_newclosure_decision_guard.sh
  - tools/smokes/v2/profiles/integration/apps/phase29x_vm_hako_newclosure_decision_refresh_vm.sh
  - tools/smokes/v2/profiles/integration/apps/phase29x_vm_hako_newclosure_contract_vm.sh
---

# 29x-83: VM-Hako NewClosure Runtime Lane Decision Refresh (SSOT)

## 0. Decision

- Decision: `accepted`（runtime fail-fast 継続）
- Decision owner: `29x-77-newclosure-contract-lock-ssot.md`（canonical）
- X57 では `new_closure` runtime execution を導入しない。
- 実行境界は X50 と同一（compiler-side shape 受理 + runtime fail-fast）を維持する。

## 1. Contract

1. `new_closure` は S6 vocabulary allowlist に昇格しない（X58 の対象候補として保留）。
2. rust-vm route は `unsupported op 'new_closure' in mir_json_v0 loader` を維持する。
3. hako-runner route は `[vm-hako/unimplemented op=new_closure]` で fail-fast 維持する。
4. X56 parity gate が green の前提で、X57 gate は decision drift を guard + smoke で固定する。

## 2. Integration gate

- Guard:
  - `tools/checks/phase29x_vm_hako_newclosure_decision_guard.sh`
- Gate:
  - `tools/smokes/v2/profiles/integration/apps/phase29x_vm_hako_newclosure_decision_refresh_vm.sh`

Gate steps:
1. decision guard（X50/X57 docs + S6 allowlist + gate wiring）
2. `phase29x_vm_hako_s6_parity_gate_vm.sh`（X56 parity precondition）
3. `phase29x_vm_hako_newclosure_contract_vm.sh`（runtime fail-fast contract replay）

## 3. Evidence command

- `bash tools/checks/phase29x_vm_hako_newclosure_decision_guard.sh`
- `bash tools/smokes/v2/profiles/integration/apps/phase29x_vm_hako_newclosure_decision_refresh_vm.sh`

## 4. Next step

- X58 は `29x-85-vm-hako-s6-first-vocabulary-promotion-ssot.md` で完了。
- X59 は `29x-86-abi-borrowed-owned-conformance-extension-ssot.md` で完了。
- X60 は `29x-87-rc-insertion-phase2-queue-lock-ssot.md` で完了。
- X61 は `29x-88-observability-drift-guard-ssot.md` で完了。
- X62 は `29x-89-runtime-core-integrated-gate-ssot.md` で完了。
- X63 は `29x-92-optimization-allowlist-lock-ssot.md` で完了。
- 次タスクは X64（optimization parity fixtures/reject fixtures）。
