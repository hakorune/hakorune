#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-nyrt-startup-env-read-inventory"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

DOC="docs/development/current/main/design/nyrt-startup-env-read-inventory-ssot.md"
CARD="docs/development/current/main/phases/phase-296x/296x-651-HAKO-MIMALLOC-TYPED-OBJECT-EXACT-SLOT-ABI-SPLIT.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
ENV_REF="docs/reference/environment-variables.md"
ENTRY="crates/nyash_kernel/src/entry.rs"
PLUGIN_INIT="src/runner/plugin_init.rs"
PLUGIN_LOADER="src/runtime/plugin_loader_unified.rs"
ROOT_HELPER="src/runner/modes/common_util/resolve/root.rs"
MAIN="src/main.rs"
ENV_SSOT="src/config/env.rs"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_nyrt_startup_env_read_inventory_guard.sh"

echo "[$TAG] checking NyRT startup env read inventory"

guard_require_files "$TAG" \
  "$DOC" "$CARD" "$CURRENT_STATE" "$INDEX" "$ENV_REF" \
  "$ENTRY" "$PLUGIN_INIT" "$PLUGIN_LOADER" "$ROOT_HELPER" "$MAIN" "$ENV_SSOT" \
  "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" "NYRT-ENV-001" "$DOC" "inventory doc must name the row"
guard_expect_fixed_in_file "$TAG" "startup-before-ny_main" "$DOC" "inventory doc must classify the NyRT entry surface"
guard_expect_fixed_in_file "$TAG" "after-ny_main/result/metrics" "$DOC" "inventory doc must classify the result/metrics tail"
guard_expect_fixed_in_file "$TAG" "runtime helper" "$DOC" "inventory doc must include helper boundary rows"
guard_expect_fixed_in_file "$TAG" "src/config/env.rs" "$DOC" "inventory doc must name the env SSOT baseline"
guard_expect_fixed_in_file "$TAG" "docs/development/current/main/design/nyrt-startup-env-read-inventory-ssot.md" "$ENV_REF" "env reference must point at the inventory doc"
guard_expect_fixed_in_file "$TAG" "NYRT-ENV-001" "$CARD" "phase card must mention the inventory row"
guard_expect_fixed_in_file "$TAG" "NYRT-ENV-001" "$CURRENT_STATE" "current state must mention the inventory row"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"

guard_expect_fixed_in_file "$TAG" "HAKO_NYRT_PLUGIN_HOST" "$ENTRY" "entry must still read the plugin-host knob directly"
guard_expect_fixed_in_file "$TAG" "NYASH_NYRT_RUNTIME_HOOKS" "$ENTRY" "entry must still read runtime-hooks mode directly"
guard_expect_fixed_in_file "$TAG" "NYASH_NYRT_RUNTIME_BUILD" "$ENTRY" "entry must still read runtime-build mode directly"
guard_expect_fixed_in_file "$TAG" "NYASH_NYRT_ENTRY_PATH_PREP" "$ENTRY" "entry must still read entry-path prep mode directly"
guard_expect_fixed_in_file "$TAG" "NYASH_NYRT_RING0_INIT" "$ENTRY" "entry must still read ring0 init mode directly"
guard_expect_fixed_in_file "$TAG" "std::env::current_exe()" "$ENTRY" "entry must still probe the executable path"
guard_expect_fixed_in_file "$TAG" "std::env::current_dir()" "$ENTRY" "entry must still probe current_dir"
guard_expect_fixed_in_file "$TAG" "NYASH_NYRT_SILENT_RESULT" "$ENTRY" "entry must keep the result suppression knob"
guard_expect_fixed_in_file "$TAG" "NYASH_GC_METRICS_JSON" "$ENTRY" "entry must keep the GC metrics knob"
guard_expect_fixed_in_file "$TAG" "NYASH_GC_COLLECT_ALLOC" "$ENTRY" "entry must keep the GC allocation threshold knob"
guard_expect_fixed_in_file "$TAG" "resolve_plugin_toml" "$PLUGIN_INIT" "plugin init helper must remain in the inventory"
guard_expect_fixed_in_file "$TAG" "crate::config::env::hako_root()" "$PLUGIN_INIT" "plugin init helper must use the centralized root helper"
guard_expect_fixed_in_file "$TAG" "disable_plugins()" "$PLUGIN_LOADER" "plugin loader helper must stay in the inventory"
guard_expect_fixed_in_file "$TAG" "resolve_repo_root" "$ROOT_HELPER" "root helper must stay in the inventory"
guard_expect_fixed_in_file "$TAG" "std::env::current_exe()" "$ROOT_HELPER" "root helper must keep executable probing"
guard_expect_fixed_in_file "$TAG" "std::env::current_dir()" "$ROOT_HELPER" "root helper must keep cwd probing"
guard_expect_fixed_in_file "$TAG" "HAKO_PROGRAM_JSON" "$MAIN" "startup wrapper must keep the program-json knob"
guard_expect_fixed_in_file "$TAG" "HAKO_PROGRAM_JSON_FILE" "$MAIN" "startup wrapper must keep the program-json file knob"
guard_expect_fixed_in_file "$TAG" "NYASH_VERIFY_JSON" "$MAIN" "startup wrapper must keep the verify-json knob"
guard_expect_fixed_in_file "$TAG" "HAKO_VERIFY_V1_FORCE_HAKOVM" "$MAIN" "startup wrapper must keep the force-hakovm knob"
guard_expect_fixed_in_file "$TAG" "std::env::current_exe()" "$MAIN" "startup wrapper must keep executable probing"
guard_expect_fixed_in_file "$TAG" "bootstrap_from_toml_env" "$ENV_SSOT" "env SSOT baseline must stay in the inventory"

tmp_dir="$(mktemp -d /tmp/hakorune_nyrt_env_inventory.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"
cat >"$report" <<'REPORT'
output_contract=nyrt-startup-env-read-inventory-v0
input_contract=nyrt-startup-env-surface-v0
inventory_scope=nyrt_startup_env_reads
primary_owner=crates/nyash_kernel/src/entry.rs
primary_classification=startup-before-ny_main|after-ny_main/result/metrics
primary_access=direct_std_env+crate::env_flags+current_exe/current_dir
primary_env_keys=HAKO_NYRT_PLUGIN_HOST,NYASH_NYRT_RUNTIME_HOOKS,NYASH_NYRT_RUNTIME_BUILD,NYASH_NYRT_ENTRY_PATH_PREP,NYASH_NYRT_RING0_INIT,NYASH_NYRT_MINIMAL_STARTUP,NYASH_NYRT_SILENT_RESULT,NYASH_GC_METRICS_JSON,NYASH_GC_METRICS,NYASH_GC_COLLECT_SP,NYASH_GC_COLLECT_ALLOC,NYASH_LLVM_AUTO_SAFEPOINT,NYASH_GC_ALLOC_THRESHOLD,PATH,PYTHONHOME
adjacent_owner_0=src/runner/plugin_init.rs
adjacent_owner_0_classification=runtime_helper
adjacent_owner_0_access=crate::config::env+cwd/toml_path_fallback
adjacent_owner_1=src/runtime/plugin_loader_unified.rs
adjacent_owner_1_classification=runtime_helper
adjacent_owner_1_access=crate::config::env::disable_plugins
adjacent_owner_2=src/runner/modes/common_util/resolve/root.rs
adjacent_owner_2_classification=runtime_helper
adjacent_owner_2_access=crate::config::env::hako_root+current_dir/current_exe
adjacent_owner_3=src/main.rs
adjacent_owner_3_classification=startup_wrapper
adjacent_owner_3_access=direct_std_env+current_exe
central_env_ssot=src/config/env.rs
central_env_ssot_classification=env_control_surface
central_env_ssot_access=bootstrap_from_toml_env+env wrappers
summary=ok
REPORT
cat "$report"

guard_expect_fixed_in_file "$TAG" "output_contract=nyrt-startup-env-read-inventory-v0" "$report" "probe report must carry the inventory contract"
guard_expect_fixed_in_file "$TAG" "primary_owner=crates/nyash_kernel/src/entry.rs" "$report" "probe report must name the primary NyRT owner"
guard_expect_fixed_in_file "$TAG" "primary_classification=startup-before-ny_main|after-ny_main/result/metrics" "$report" "probe report must split the NyRT entry surface"
guard_expect_fixed_in_file "$TAG" "primary_env_keys=HAKO_NYRT_PLUGIN_HOST,NYASH_NYRT_RUNTIME_HOOKS,NYASH_NYRT_RUNTIME_BUILD,NYASH_NYRT_ENTRY_PATH_PREP,NYASH_NYRT_RING0_INIT,NYASH_NYRT_MINIMAL_STARTUP,NYASH_NYRT_SILENT_RESULT,NYASH_GC_METRICS_JSON,NYASH_GC_METRICS,NYASH_GC_COLLECT_SP,NYASH_GC_COLLECT_ALLOC,NYASH_LLVM_AUTO_SAFEPOINT,NYASH_GC_ALLOC_THRESHOLD,PATH,PYTHONHOME" "$report" "probe report must capture the direct entry keys"
guard_expect_fixed_in_file "$TAG" "adjacent_owner_0=src/runner/plugin_init.rs" "$report" "probe report must include plugin init"
guard_expect_fixed_in_file "$TAG" "adjacent_owner_1=src/runtime/plugin_loader_unified.rs" "$report" "probe report must include plugin loader"
guard_expect_fixed_in_file "$TAG" "adjacent_owner_2=src/runner/modes/common_util/resolve/root.rs" "$report" "probe report must include root helper"
guard_expect_fixed_in_file "$TAG" "adjacent_owner_3=src/main.rs" "$report" "probe report must include startup wrapper"
guard_expect_fixed_in_file "$TAG" "central_env_ssot=src/config/env.rs" "$report" "probe report must include the env SSOT baseline"
guard_expect_fixed_in_file "$TAG" "summary=ok" "$report" "probe report must finish cleanly"

echo "[$TAG] ok"
