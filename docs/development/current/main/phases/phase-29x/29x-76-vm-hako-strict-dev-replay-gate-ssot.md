---
Status: Active
Decision: provisional
Date: 2026-02-13
Scope: X49 の vm-hako strict/dev replay gate を固定し、route pin override なしで `backend=vm` が vm-hako lane を再生できることを契約化する。
Related:
  - docs/development/current/main/phases/phase-29x/29x-75-vm-route-pin-inventory-guard-ssot.md
  - tools/smokes/v2/profiles/integration/apps/phase29x_vm_hako_strict_dev_replay_vm.sh
---

# 29x-76: VM-Hako Strict/Dev Replay Gate (SSOT)

## 0. Conclusion

- strict/dev の `--backend vm` は route pin override なしで `lane=vm-hako` を選択する。
- replay gate は supported fixture と reject fixture の 2ケースで固定する。

## 1. Contract

1. supported fixture（`phase29z_vm_hako_s0_const_add_return_min.hako`）:
   - `rc=42`
   - first `[vm-route/select]` は `lane=vm-hako reason=strict-dev-prefer`
   - first `[derust-route/select]` は `source=hako-skeleton`
2. reject fixture（`phase29z_vm_hako_s0_reject_compare_ne_min.hako`）:
   - non-zero exit
   - first `[vm-route/select]` は `lane=vm-hako reason=strict-dev-prefer`
   - `[vm-hako/unimplemented]` tag を観測

## 2. Evidence command

- `bash tools/smokes/v2/profiles/integration/apps/phase29x_vm_hako_strict_dev_replay_vm.sh`

