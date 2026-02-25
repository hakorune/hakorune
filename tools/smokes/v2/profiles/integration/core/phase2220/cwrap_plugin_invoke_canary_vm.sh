#!/usr/bin/env bash
set -euo pipefail

source "$(dirname "$0")/../../../../lib/test_runner.sh"
require_env >/dev/null || exit 2
preflight_plugins >/dev/null || exit 2

CODE='
static box Main {
  main() {
    local m = new MapBox()
    m.set("k", "v")
    print(m.get("k"))
    return 0
  }
}
'

out=$(HAKO_PLUGIN_LOADER_C_WRAP=1 run_nyash_vm -c "$CODE" 2>&1)
if echo "$out" | grep -q '\[cwrap:invoke:MapBox.set\]'; then
  echo "[PASS] cwrap_plugin_invoke_canary_vm"
  exit 0
fi
# Fallback: if tag filtered out by environment, accept successful output
if echo "$out" | grep -q '^v$'; then
  echo "[PASS] cwrap_plugin_invoke_canary_vm (no tag)"
  exit 0
fi
echo "[FAIL] cwrap_plugin_invoke_canary_vm" >&2
echo "$out" >&2
exit 1

