---
Status: Done
Decision: accepted
Date: 2026-02-27
Scope: PLG-07-min6 として FileBox binary route の Rust plugin retire readiness 判定基準を docs-first で固定し、portability gate への接続を SSOT 化する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-178-plg07-plugin-derust-cutover-order-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-182-plg07-min5-filebox-default-switch-lock-ssot.md
  - tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg07_filebox_binary_retire_readiness_lock_vm.sh
  - tools/checks/phase29cc_plg07_filebox_binary_retire_readiness_guard.sh
  - tools/checks/dev_gate.sh
---

# 29cc-183 PLG-07-min6 FileBox Retire Readiness Lock

## Purpose
Rust plugin route を即撤去せず、先に「どの条件が揃えば retire 実行へ進めるか」を lock する。  
PLG-07-min5 で既定を `.hako` route 化した状態を前提に、判定根拠を guard/smoke で再現可能にする。

## Decision
1. `PLG-07-min6` は docs-first の retire readiness lock とし、code behavior は変更しない。
2. retire readiness 判定の根拠は次を維持する:
   - default switch 契約（`.hako` route default、Rust route compat）
   - dual-run parity 契約（Rust route / `.hako` route 同値）
   - `.hako` route の実 fixture 成功（FileBox binary API）
3. portability milestone 監査に readiness guard を接続する:
   - `tools/checks/dev_gate.sh portability`
   - `tools/checks/phase29cc_plg07_filebox_binary_retire_readiness_guard.sh`
4. 実際の retire 実行（Rust compat route の撤去）は `PLG-07-min7` で扱う。

## Implemented
1. smoke 追加:
   - `tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg07_filebox_binary_retire_readiness_lock_vm.sh`
   - lock doc keyword と readiness 証跡（default-switch/dual-run/.hako route）を固定。
2. guard 追加:
   - `tools/checks/phase29cc_plg07_filebox_binary_retire_readiness_guard.sh`
3. gate 接続:
   - `tools/checks/dev_gate.sh portability` に `PLG-07 retire readiness guard` step を追加。

## Rollback contract
1. default route rollback は `NYASH_PLG07_COMPAT_RUST=1` で Rust compat route を再有効化する。
2. parity rollback/監査は `NYASH_PLG07_DUALRUN=1` で dual-run を再有効化する。
3. rollback は route 切替のみで行い、コードコピー退避は作らない（git history を正本）。

## Acceptance
1. `cargo check --bin hakorune`
2. `bash tools/checks/phase29cc_plg07_filebox_binary_retire_readiness_guard.sh`
3. `bash tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg07_filebox_binary_retire_readiness_lock_vm.sh`
4. `tools/checks/dev_gate.sh portability`

## Next
1. `PLG-07-min7`: Rust compat route retire execution lock（accepted-but-blocked; 連続2 milestone 判定を満たした後に縮退実行）。
