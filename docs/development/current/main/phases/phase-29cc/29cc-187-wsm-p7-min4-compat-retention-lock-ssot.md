---
Status: Done
Decision: accepted-but-blocked
Date: 2026-02-27
Scope: WSM-P7-min4 として compat route の期限付き保持方針を lock し、P8 retire execution 条件へ接続する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-186-wsm-p7-min3-two-demo-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-170-wsm-p6-min1-route-policy-default-noop-lock-ssot.md
  - tools/smokes/v2/profiles/integration/phase29cc_wsm/p7/phase29cc_wsm_p7_min4_compat_retention_lock_vm.sh
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
---

# 29cc-187 WSM-P7-min4 Compat Retention Lock

## Purpose
`.hako` default route を主経路として維持しつつ、compat route を期限付きで保持する運用境界を固定する。  
P7 では即削除せず、P8 の retire execution へ判定条件を受け渡す。

## Decision
1. default route は不変（default-only）。
2. compat route は「内部互換責務」として期限付き保持する。
3. rollback は route 切替ではなく、既存 lock smoke/guard の再実行で検証する。
4. 実際の compat route retire execution は `WSM-P8-min1` で扱う（accepted-but-blocked）。

## Implemented
1. lock smoke 追加:
   - `tools/smokes/v2/profiles/integration/phase29cc_wsm/p7/phase29cc_wsm_p7_min4_compat_retention_lock_vm.sh`
2. smoke は次を連結して固定:
   - `phase29cc_wsm_p7_min2_default_hako_only_guard_vm.sh`
   - `phase29cc_wsm_p7_min3_two_demo_lock_vm.sh`

## Acceptance
1. `bash tools/smokes/v2/profiles/integration/phase29cc_wsm/p7/phase29cc_wsm_p7_min4_compat_retention_lock_vm.sh`
2. `tools/checks/dev_gate.sh portability`
3. `tools/checks/dev_gate.sh wasm-demo-g3-full`

## Next
1. `WSM-P8-min1`: compat route retire execution lock（accepted-but-blocked の解除判定）。
