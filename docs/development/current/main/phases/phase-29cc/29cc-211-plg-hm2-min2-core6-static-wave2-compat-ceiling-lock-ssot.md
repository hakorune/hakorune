---
Status: Done
Decision: accepted
Date: 2026-02-28
Scope: PLG-HM2-min2 として plugin route matrix を固定し、Core6 static route と Wave2 compat ceiling を drift 監視する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-209-plg-hm1-core8-module-provider-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-210-plg-hm2-core-wave2-rust-recovery-line-lock-ssot.md
  - src/box_factory/plugin.rs
  - tools/checks/phase29cc_plg_hm2_min2_core6_wave2_ceiling_guard.sh
  - tools/checks/dev_gate.sh
---

# 29cc-211 PLG-HM2-min2 Core6 Static + Wave2 Compat Ceiling Lock

## Purpose
PLG-HM1 で導入した `module_first` route を HM2 で運用契約に昇格し、  
Core6 と Wave2 の責務境界を fail-fast で固定する。

## Route Matrix Contract
1. Core6（`ArrayBox/StringBox/MapBox/ConsoleBox/FileBox/PathBox`）は `module_first` で dynamic route を skip する。
2. Wave2 ceiling は `MathBox/NetClientBox` までに固定し、`module_first` でも dynamic compat を維持する。
3. `plugin-module-core8` gate は HM2 専用 guard を必須化し、route matrix drift を検知する。
4. HM2 では route 拡張を行わない（Wave3 以降は別 lock でのみ追加）。

## Acceptance
- `bash tools/checks/phase29cc_plg_hm2_min2_core6_wave2_ceiling_guard.sh`
- `bash tools/smokes/v2/profiles/integration/apps/phase29cc_plg_hm1_contract_tests_vm.sh`
- `tools/checks/dev_gate.sh plugin-module-core8`

## Next (fixed order)
1. `none`（monitor-only）
