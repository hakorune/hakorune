---
Status: Done
Decision: accepted
Date: 2026-02-28
Scope: PLG-HM2-min3 として plugin route policy matrix（exec_mode x factory_policy）を固定し、mainline 既定と compat 組み合わせの監査を fail-fast 化する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-210-plg-hm2-core-wave2-rust-recovery-line-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-211-plg-hm2-min2-core6-static-wave2-compat-ceiling-lock-ssot.md
  - src/config/env/box_factory_flags.rs
  - src/config/env/catalog.rs
  - src/box_factory/registry.rs
  - src/box_factory/plugin.rs
  - tools/checks/phase29cc_plg_hm2_min3_route_policy_matrix_guard.sh
  - tools/checks/dev_gate.sh
---

# 29cc-212 PLG-HM2-min3 Route Policy Matrix Lock

## Purpose
HM2-min1/min2 で固定した Core+Wave2 契約を保ったまま、  
plugin route policy の運用マトリクス（`NYASH_PLUGIN_EXEC_MODE` x `NYASH_BOX_FACTORY_POLICY`）を SSOT で固定する。

## Route Policy Matrix (fixed)

1. mainline default:
   - `NYASH_PLUGIN_EXEC_MODE=module_first`
   - `NYASH_BOX_FACTORY_POLICY=strict_plugin_first`
2. compat lane（monitor-only）:
   - `module_first + compat_plugin_first`（Core6 static route監査）
   - `dynamic_first + compat_plugin_first`（互換回帰確認）
3. diagnostic lane（monitor-only）:
   - `dynamic_only + strict_plugin_first`（plugin-only routing 点検）
4. mainline gate（`plugin-module-core8`）は 1. の既定を正本とし、2/3 は docs/guard で drift 監視する。

## Contracts

1. `NYASH_PLUGIN_EXEC_MODE` の受理値は `module_first|dynamic_only|dynamic_first` で固定。
2. `NYASH_BOX_FACTORY_POLICY` の受理値は `strict_plugin_first|compat_plugin_first|builtin_first` で固定。
3. `module_first` 時の Core6 static route と Wave2 compat ceiling（Math/Net）は HM2-min2 契約を継承する。
4. HM2-min3 では runtime 挙動の追加変更はしない（契約固定と監査導線のみ）。

## Acceptance

- `bash tools/checks/phase29cc_plg_hm2_min3_route_policy_matrix_guard.sh`
- `tools/checks/dev_gate.sh plugin-module-core8`
- `tools/checks/dev_gate.sh portability`

## Completion Evidence

- `bash tools/checks/phase29cc_plg_hm2_rust_recovery_line_guard.sh`
- `bash tools/checks/phase29cc_plg_hm2_min2_core6_wave2_ceiling_guard.sh`
- `bash tools/checks/phase29cc_plg_hm2_min3_route_policy_matrix_guard.sh`
- `tools/checks/dev_gate.sh plugin-module-core8`

## Next (fixed order)

1. `none`（monitor-only、failure-driven reopen）
