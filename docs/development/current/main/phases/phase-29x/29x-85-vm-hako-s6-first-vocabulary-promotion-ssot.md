---
Status: Active
Decision: accepted
Date: 2026-02-13
Scope: X58 vm-hako S6 first vocabulary promotion（`nop`）を 1語彙で固定する。
Related:
  - docs/development/current/main/phases/phase-29x/29x-80-postx53-runtime-core-sequencing-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-81-vm-hako-s6-vocabulary-inventory-guard-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-82-vm-hako-s6-dual-run-parity-gate-pack-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-83-vm-hako-newclosure-runtime-lane-decision-refresh-ssot.md
  - tools/checks/phase29x_vm_hako_s6_nop_promotion_guard.sh
  - tools/smokes/v2/profiles/integration/apps/phase29x_vm_hako_s6_nop_promotion_vm.sh
  - tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s3_nop_parity_vm.sh
---

# 29x-85: VM-Hako S6 First Vocabulary Promotion (SSOT)

## 0. Goal

- X58 の BoxCount を 1語彙だけ実施し、`nop` を S6 vocabulary へ昇格する。
- `new_closure` fail-fast decision（X57）は維持し、語彙昇格と混在させない。
- fixture+gate で再生可能な形に固定し、X59 へ進める前提を作る。

## 1. Contract

1. `check_vm_hako_subset_json` は `nop` を受理する（retired no-op shape）。
2. `tools/checks/phase29x_vm_hako_s6_vocab_allowlist.txt` は `nop` を含む。
3. `debug_log` は allowlist に昇格しない（legacy drift は継続 reject）。
4. X58 gate は X56 parity gate を前提 step として必ず再生する。
5. `phase29z_vm_hako_s3_nop_parity_vm.sh` は route pin helper 経由で実行し、nop parity を固定する。

## 2. Integration gate

- Guard:
  - `tools/checks/phase29x_vm_hako_s6_nop_promotion_guard.sh`
- Gate:
  - `tools/smokes/v2/profiles/integration/apps/phase29x_vm_hako_s6_nop_promotion_vm.sh`

Gate steps:
1. X58 guard（allowlist/subset-check/gate wiring）
2. `phase29x_vm_hako_s6_parity_gate_vm.sh`（X56 precondition）
3. `phase29z_vm_hako_s3_nop_parity_vm.sh`（route pin helper 経由）

## 3. Evidence command

- `bash tools/checks/phase29x_vm_hako_s6_nop_promotion_guard.sh`
- `bash tools/smokes/v2/profiles/integration/apps/phase29x_vm_hako_s6_nop_promotion_vm.sh`

## 4. Next step

- X59 は `29x-86-abi-borrowed-owned-conformance-extension-ssot.md` で完了。
- X60 は `29x-87-rc-insertion-phase2-queue-lock-ssot.md` で完了。
- X61 は `29x-88-observability-drift-guard-ssot.md` で完了。
- X62 は `29x-89-runtime-core-integrated-gate-ssot.md` で完了。
- X63 は `29x-92-optimization-allowlist-lock-ssot.md` で完了。
- 次タスクは X64（optimization parity fixtures/reject fixtures）。
