#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "$0")/.." && pwd)
cd "$ROOT_DIR"

declare -a VM_PLUGIN_SMOKES=(
  "tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg03_counterbox_pilot_vm.sh"
  "tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg04_arraybox_pilot_vm.sh"
  "tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg04_intcellbox_pilot_vm.sh"
  "tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg04_mapbox_pilot_vm.sh"
  "tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg04_stringbox_pilot_vm.sh"
  "tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg04_consolebox_pilot_vm.sh"
  "tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg04_filebox_pilot_vm.sh"
  "tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg07_filebox_binary_hako_route_vm.sh"
  "tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg05_json_pilot_vm.sh"
  "tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg05_toml_pilot_vm.sh"
  "tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg05_regex_pilot_vm.sh"
  "tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg05_encoding_pilot_vm.sh"
  "tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg05_path_pilot_vm.sh"
  "tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg05_time_pilot_vm.sh"
  "tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg05_net_pilot_vm.sh"
  "tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg06_pycompiler_pilot_vm.sh"
  "tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg06_python_pilot_vm.sh"
  "tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg06_pyparser_pilot_vm.sh"
  "tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg06_egui_pilot_vm.sh"
)

echo "[vm-plugin-smoke] delegate: phase29cc pilot manifest (${#VM_PLUGIN_SMOKES[@]} scripts)"
for smoke in "${VM_PLUGIN_SMOKES[@]}"; do
  bash "$smoke"
done

run_plg07_optional_smoke() {
  local env_name="$1"
  local script_path="$2"
  local label="$3"
  if [ "${!env_name:-0}" = "1" ]; then
    echo "[vm-plugin-smoke] $label"
    bash "$script_path"
  fi
}

# PLG-07-min5 default switch:
# - default: .hako route only (Rust compat OFF)
# - optional compat: enable NYASH_PLG07_COMPAT_RUST=1
# - optional dual-run parity check: enable NYASH_PLG07_DUALRUN=1
run_plg07_optional_smoke \
  "NYASH_PLG07_COMPAT_RUST" \
  "tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg07_filebox_binary_rust_route_vm.sh" \
  "PLG-07 compat route enabled (rust)"

run_plg07_optional_smoke \
  "NYASH_PLG07_DUALRUN" \
  "tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg07_filebox_binary_dualrun_vm.sh" \
  "PLG-07 dual-run parity enabled"
