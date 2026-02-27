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

# PLG-07-min5 default switch:
# - default: .hako route only (Rust compat OFF)
# - optional compat: enable NYASH_PLG07_COMPAT_RUST=1
# - optional dual-run parity check: enable NYASH_PLG07_DUALRUN=1
if [ "${NYASH_PLG07_COMPAT_RUST:-0}" = "1" ]; then
  echo "[vm-plugin-smoke] PLG-07 compat route enabled (rust)"
  bash "tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg07_filebox_binary_rust_route_vm.sh"
fi

if [ "${NYASH_PLG07_DUALRUN:-0}" = "1" ]; then
  echo "[vm-plugin-smoke] PLG-07 dual-run parity enabled"
  bash "tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg07_filebox_binary_dualrun_vm.sh"
fi
