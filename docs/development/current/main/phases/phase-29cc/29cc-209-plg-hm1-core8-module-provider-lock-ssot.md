---
Status: Active
Decision: accepted
Date: 2026-02-28
Scope: PLG-HM1 として Core plugin de-Rust の入口を固定し、`module_first` 主経路（`.hako` module provider line）を段階導入する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-204-plg07-min7-filebox-retire-execution-lock-ssot.md
  - src/config/env/box_factory_flags.rs
  - src/box_factory/plugin.rs
  - tools/checks/dev_gate.sh
  - tools/smokes/v2/profiles/integration/apps/phase29cc_plg_hm1_min1_plugin_exec_mode_lock_vm.sh
  - tools/smokes/v2/profiles/integration/apps/phase29cc_plg_hm1_min2_core_module_route_skip_lock_vm.sh
  - tools/smokes/v2/profiles/integration/apps/phase29cc_plg_hm1_min3_file_path_module_first_lock_vm.sh
  - tools/smokes/v2/profiles/integration/apps/phase29cc_plg_hm1_min4_math_net_compat_inventory_lock_vm.sh
  - tools/smokes/v2/profiles/integration/apps/phase29cc_plg_hm1_contract_tests_vm.sh
---

# 29cc-209 PLG-HM1 Core Module Provider Lock

## Purpose
Core plugin の脱Rustラインを、`dynamic ABI` を互換レーンとして残しつつ `module_first` 主経路へ移す。  
この lock では、実行モードENVと route 縮退契約を fail-fast で固定する。

## Decision
1. `NYASH_PLUGIN_EXEC_MODE` を導入し、受理値を `module_first|dynamic_only|dynamic_first` に固定する。
2. default は `module_first`（将来の `.hako` module provider 主経路を既定にする）。
3. `module_first` では Core6（`ArrayBox/StringBox/MapBox/ConsoleBox/FileBox/PathBox`）を dynamic plugin route から縮退する。
4. Math/Net は `dynamic compat` を維持し、この lock では route 切替しない。
5. invalid mode は `[freeze:contract][plugin/exec-mode]` で fail-fast。

## Implemented (min1..min5)
1. `src/config/env/box_factory_flags.rs`
   - `PluginExecMode` parser + default + fail-fast を追加。
2. `src/box_factory/plugin.rs`
   - `module_first` 時の Core6 dynamic route skip を追加。
3. `tools/smokes/.../phase29cc_plg_hm1_min1_plugin_exec_mode_lock_vm.sh`
   - mode parser 契約を lock。
4. `tools/smokes/.../phase29cc_plg_hm1_min2_core_module_route_skip_lock_vm.sh`
   - Core route skip 契約を lock（初期）。
5. `tools/smokes/.../phase29cc_plg_hm1_min3_file_path_module_first_lock_vm.sh`
   - File/Path route skip 契約を lock。
6. `tools/smokes/.../phase29cc_plg_hm1_min4_math_net_compat_inventory_lock_vm.sh`
   - Math/Net compat inventory 契約を lock。
7. `tools/checks/dev_gate.sh`
   - `plugin-module-core8-light` は集約 smoke（`phase29cc_plg_hm1_contract_tests_vm.sh`）で min1..min4 を固定。
   - `plugin-module-core8` へ昇格。

## Acceptance
- `bash tools/smokes/v2/profiles/integration/apps/phase29cc_plg_hm1_min1_plugin_exec_mode_lock_vm.sh`
- `bash tools/smokes/v2/profiles/integration/apps/phase29cc_plg_hm1_min2_core_module_route_skip_lock_vm.sh`
- `bash tools/smokes/v2/profiles/integration/apps/phase29cc_plg_hm1_min3_file_path_module_first_lock_vm.sh`
- `bash tools/smokes/v2/profiles/integration/apps/phase29cc_plg_hm1_min4_math_net_compat_inventory_lock_vm.sh`
- `bash tools/smokes/v2/profiles/integration/apps/phase29cc_plg_hm1_contract_tests_vm.sh`
- `tools/checks/dev_gate.sh plugin-module-core8-light`
- `tools/checks/dev_gate.sh plugin-module-core8`

## Next (fixed order)
1. `none`（HM1 min1..min5 complete; monitor-only）
