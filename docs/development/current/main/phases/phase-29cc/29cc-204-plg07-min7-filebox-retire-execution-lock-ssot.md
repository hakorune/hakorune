---
Status: Done
Decision: accepted
Date: 2026-02-27
Scope: PLG-07-min7 として FileBox binary の Rust compat route retire execution を実行し、`.hako` 単一路線へ固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-183-plg07-min6-filebox-retire-readiness-lock-ssot.md
  - tools/vm_plugin_smoke.sh
  - tools/checks/phase29cc_plg07_filebox_binary_retire_execution_guard.sh
  - tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg07_filebox_binary_retire_execution_lock_vm.sh
  - tools/checks/dev_gate.sh
---

# 29cc-204 PLG-07-min7 FileBox Retire Execution Lock

## Purpose
`PLG-07-min6` で固定した readiness 条件を満たしたため、Rust compat route の実行導線を retire し、FileBox binary の plugin smoke を `.hako` route 単一路線へ固定する。

## Decision
1. `tools/vm_plugin_smoke.sh` から `NYASH_PLG07_COMPAT_RUST` / `NYASH_PLG07_DUALRUN` オプトイン導線を削除する。
2. default plugin smoke は `phase29cc_plg07_filebox_binary_hako_route_vm.sh` のみを維持する。
3. portability 監査は min6 readiness guard から min7 retire execution guard へ切り替える。
4. Rust/dual-run 用 smoke は archive に残すが、日常 gate からは呼ばない。

## Contract
1. `tools/vm_plugin_smoke.sh` に `NYASH_PLG07_COMPAT_RUST` / `NYASH_PLG07_DUALRUN` を残さない。
2. `tools/vm_plugin_smoke.sh` は `phase29cc_plg07_filebox_binary_hako_route_vm.sh` を default manifest に保持する。
3. `tools/checks/dev_gate.sh portability` は `phase29cc_plg07_filebox_binary_retire_execution_guard.sh` を実行する。
4. min7 smoke は `.hako` route と retire execution guard の両方を緑で固定する。

## Acceptance
1. `bash tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg07_filebox_binary_retire_execution_lock_vm.sh`
2. `bash tools/checks/phase29cc_plg07_filebox_binary_retire_execution_guard.sh`
3. `tools/checks/dev_gate.sh portability`

## Next
1. plugin lane active next は `none`（PLG-07 closeout complete; monitor-only）。
