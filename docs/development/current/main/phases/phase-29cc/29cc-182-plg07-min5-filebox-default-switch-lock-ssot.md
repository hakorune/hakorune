---
Status: Done
Decision: accepted
Date: 2026-02-27
Scope: PLG-07-min5 として FileBox plugin route の既定を `.hako` parity route に切り替え、Rust route を compat 既定OFFへ縮退する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-178-plg07-plugin-derust-cutover-order-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-181-plg07-min4-filebox-binary-dualrun-gate-lock-ssot.md
  - tools/vm_plugin_smoke.sh
  - tools/checks/phase29cc_plg07_filebox_binary_default_switch_guard.sh
---

# 29cc-182 PLG-07-min5 FileBox Default Switch Lock

## Purpose
PLG-07-min4 で固定した dual-run parity を前提に、日常 plugin smoke の既定 route を `.hako` parity 側へ切り替える。

## Decision
1. `tools/vm_plugin_smoke.sh` の既定 manifest は `.hako` route のみ実行する。
2. Rust route は compat route として残し、既定OFFにする。
3. dual-run parity は milestone 判定用に既定OFFのオプトインへ移す。

## Contract
1. 既定（ENV未指定）では以下のみ実行:
   - `phase29cc_plg07_filebox_binary_hako_route_vm.sh`
2. compat route を確認する場合のみ:
   - `NYASH_PLG07_COMPAT_RUST=1`
3. dual-run parity を確認する場合のみ:
   - `NYASH_PLG07_DUALRUN=1`
4. default switch guard は manifest と toggle の整合を fail-fast で監査する。

## Acceptance
1. `cargo check --bin hakorune`
2. `bash tools/checks/phase29cc_plg07_filebox_binary_default_switch_guard.sh`
3. `bash tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg07_filebox_binary_hako_route_vm.sh`
4. `NYASH_PLG07_COMPAT_RUST=1 bash tools/vm_plugin_smoke.sh`（compat route 監査）
5. `NYASH_PLG07_DUALRUN=1 bash tools/vm_plugin_smoke.sh`（dual-run オプトイン監査）

## Next
1. `PLG-07-min6` は `29cc-183` で readiness lock 済み。
2. 次は `PLG-07-min7` retire execution lock（accepted-but-blocked）へ進む。
