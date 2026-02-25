---
Status: Active
Decision: accepted
Date: 2026-02-13
Scope: X60 RC insertion phase2 queue（loop/call/early-exit）の挿入順序を gate-first で固定する。
Related:
  - docs/development/current/main/phases/phase-29x/29x-80-postx53-runtime-core-sequencing-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-20-early-exit-cleanup-ssot.md
  - docs/development/current/main/phases/phase-29y/20-RC-INSERTION-SSOT.md
  - src/mir/passes/rc_insertion.rs
  - src/bin/rc_insertion_selfcheck.rs
  - tools/checks/phase29x_rc_phase2_queue_cases.txt
  - tools/checks/phase29x_rc_phase2_queue_guard.sh
  - tools/smokes/v2/profiles/integration/apps/phase29x_rc_phase2_queue_lock_vm.sh
---

# 29x-87: RC Insertion Phase2 Queue Lock (SSOT)

## 0. Goal

- X60 として、RC insertion の phase2 queue 順序（loop/call/early-exit）を明示契約で固定する。
- 実装差分の有無に依存せず、`rc_insertion_selfcheck` の安定 marker を guard+gate で再生可能にする。
- runtime core lane の順序を維持するため、X59 ABI gate を前提 step に固定する。

## 1. Queue matrix SSOT

Matrix source of truth:
- `tools/checks/phase29x_rc_phase2_queue_cases.txt`

Fixed cases:

| Case ID | Marker | Contract focus |
| --- | --- | --- |
| `loop_early_exit_queue` | `[rc_phase2_queue/case] loop=ok` | break/continue 早期脱出 cleanup が Jump 前で安定挿入される |
| `call_overwrite_queue` | `[rc_phase2_queue/case] call=ok` | `Call -> Store` 上書きで release が Store(new) の直前に挿入される |
| `return_early_exit_queue` | `[rc_phase2_queue/case] early_exit=ok` | Return cleanup が Return 前 queue として安定挿入される |

Summary marker:
- `[rc_phase2_queue] loop=ok call=ok early_exit=ok`

## 2. Contract

1. `phase29x_rc_phase2_queue_cases.txt` の全 marker は `src/bin/rc_insertion_selfcheck.rs` に存在する。
2. `call` 経由の上書き release は instruction queue（`Store(new)` 直前）で固定する。
3. early-exit cleanup（Return/break/continue）は terminator queue（control transfer 直前）で固定する。
4. X60 gate は X59 gate（`phase29x_abi_borrowed_owned_conformance_vm.sh`）を前提実行する。
5. `rc-insertion-minimal` 実行で `[PASS] rc_insertion_selfcheck` と queue summary marker の両方が観測できる。

## 3. Integration gate

- Guard:
  - `tools/checks/phase29x_rc_phase2_queue_guard.sh`
- Gate:
  - `tools/smokes/v2/profiles/integration/apps/phase29x_rc_phase2_queue_lock_vm.sh`

Gate steps:
1. X60 guard（cases/docs/gate wiring）
2. X59 precondition（ABI borrowed/owned single-entry gate）
3. `cargo run -q --bin rc_insertion_selfcheck --features rc-insertion-minimal`
4. case markers + summary marker + PASS marker を検証

## 4. Evidence command

- `bash tools/checks/phase29x_rc_phase2_queue_guard.sh`
- `bash tools/smokes/v2/profiles/integration/apps/phase29x_rc_phase2_queue_lock_vm.sh`

## 5. Next step

- X61 は `29x-88-observability-drift-guard-ssot.md` で完了。
- X62 は `29x-89-runtime-core-integrated-gate-ssot.md` で完了。
- X63 は `29x-92-optimization-allowlist-lock-ssot.md` で完了。
- 次タスクは X64（optimization parity fixtures/reject fixtures）。
