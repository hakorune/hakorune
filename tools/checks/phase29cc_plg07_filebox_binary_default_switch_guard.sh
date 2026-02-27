#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

TARGET="tools/vm_plugin_smoke.sh"
MANIFEST_BLOCK="$(awk '
  /declare -a VM_PLUGIN_SMOKES=\(/ { in_block=1; next }
  in_block && /^\)/ { in_block=0; exit }
  in_block { print }
' "$TARGET")"

if ! rg -q "phase29cc_plg07_filebox_binary_hako_route_vm.sh" "$TARGET"; then
  echo "[plg07-default-switch-guard] missing default .hako route in $TARGET" >&2
  exit 1
fi

if printf '%s\n' "$MANIFEST_BLOCK" | rg -q 'phase29cc_plg07_filebox_binary_rust_route_vm.sh'; then
  echo "[plg07-default-switch-guard] rust route still present in default manifest list" >&2
  exit 1
fi

if printf '%s\n' "$MANIFEST_BLOCK" | rg -q 'phase29cc_plg07_filebox_binary_dualrun_vm.sh'; then
  echo "[plg07-default-switch-guard] dual-run route still present in default manifest list" >&2
  exit 1
fi

if ! rg -q "NYASH_PLG07_COMPAT_RUST" "$TARGET"; then
  echo "[plg07-default-switch-guard] compat toggle NYASH_PLG07_COMPAT_RUST missing" >&2
  exit 1
fi

if ! rg -q "NYASH_PLG07_DUALRUN" "$TARGET"; then
  echo "[plg07-default-switch-guard] dual-run toggle NYASH_PLG07_DUALRUN missing" >&2
  exit 1
fi

echo "[plg07-default-switch-guard] ok"
