---
Status: Active
Decision: accepted
Date: 2026-02-13
Scope: X53 post-X46 runtime handoff done sync + rollback lock を固定する。
Related:
  - docs/development/current/main/phases/phase-29x/29x-73-postx46-runtime-handoff-sequencing-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-75-vm-route-pin-inventory-guard-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-76-vm-hako-strict-dev-replay-gate-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-77-newclosure-contract-lock-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-78-core-cabi-delegation-inventory-guard-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-79-runtime-handoff-gate-integration-ssot.md
  - docs/development/current/main/design/runtime-gc-policy-and-order-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-91-task-board.md
  - docs/development/current/main/phases/phase-29x/29x-90-integration-checklist.md
---

# 29x-74: Post-X46 Runtime Handoff Done Sync SSOT

## 0. Goal

- X47-X53 runtime handoff lane の完了判定を 1 枚で固定する。
- rollback 条件を明示し、strict/dev 既定の route 契約を壊さずに戻せる状態を保証する。
- GC optional（非意味論）の扱いを残課題として固定する。

## 1. Done Criteria (X53)

次をすべて満たした状態を post-X46 runtime handoff lane 完了とする。

1. route pin 増殖が guard で検出できる  
   (`phase29x_vm_route_pin_guard.sh` / `phase29x_vm_route_pin_guard_vm.sh`)
2. vm-hako replay と既存 daily gate を分離観測できる  
   (`phase29x_vm_hako_strict_dev_replay_vm.sh` + `phase29x_runtime_handoff_gate_vm.sh`)
3. Core C ABI boundary 逸脱が fail-fast で検出できる  
   (`phase29x_core_cabi_delegation_guard.sh` / `phase29x_core_cabi_delegation_guard_vm.sh`)
4. X48-X51 が X52 の single-entry gate から再生できる  
   (`phase29x_runtime_handoff_gate_guard.sh` / `phase29x_runtime_handoff_gate_vm.sh`)

## 2. Rollback Lock (X53)

rollback は次の明示スイッチだけを許可する。

1. strict/dev route pin override: `NYASH_VM_HAKO_PREFER_STRICT_DEV=0`
2. compat fallback opt-in: `NYASH_VM_USE_FALLBACK=1`
3. Rust lane compatibility opt-in: `PHASE29X_ALLOW_RUST_LANE=1 tools/compat/phase29x_rust_lane_gate.sh --dry-run`
4. backend 強制選択: `--backend vm-hako` / `--backend vm`

禁止:
- silent fallback（明示フラグなしで lane を切り替える）
- NewClosure の暗黙実装差し替え（X50 Decision を崩す）

## 3. Evidence (X53)

1. `bash tools/checks/phase29x_vm_route_pin_guard.sh`
2. `bash tools/smokes/v2/profiles/integration/apps/phase29x_vm_route_pin_guard_vm.sh`
3. `bash tools/smokes/v2/profiles/integration/apps/phase29x_vm_hako_strict_dev_replay_vm.sh`
4. `bash tools/smokes/v2/profiles/integration/apps/phase29x_vm_hako_newclosure_contract_vm.sh`
5. `bash tools/checks/phase29x_core_cabi_delegation_guard.sh`
6. `bash tools/smokes/v2/profiles/integration/apps/phase29x_core_cabi_delegation_guard_vm.sh`
7. `bash tools/checks/phase29x_runtime_handoff_gate_guard.sh`
8. `bash tools/smokes/v2/profiles/integration/apps/phase29x_runtime_handoff_gate_vm.sh`

## 4. Residual Scope / Non-goals

1. GC/cycle collector の新規実装はこの lane の対象外（optional/last を維持）。
2. runtime 意味論は GC ON/OFF で不変（回収タイミング差のみ）。

## 5. Next Step

post-X46 runtime handoff lane（X47-X53）は完了。
以後は `phase29x_llvm_only_daily_gate.sh` と `phase29x_runtime_handoff_gate_vm.sh` を節目運用し、
通常日は failure-driven の軽量ループへ戻る。
