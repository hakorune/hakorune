#!/bin/bash
# stageb_bundle_mix_emit_vm.sh — Stage‑B: mix of --bundle-src and --bundle-mod emits valid header

set -euo pipefail
if [ "${SMOKES_ENABLE_STAGEB:-0}" != "1" ]; then
  echo "[SKIP] stageb_bundle_mix_emit_vm (SMOKES_ENABLE_STAGEB=1 to enable)"
  exit 0
fi
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"
fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
source "$ROOT/tools/smokes/v2/lib/stageb_helpers.sh"
require_env || exit 2

main='static box Main { method main(args) { return 0 } }'
u_src='static box Ux { method id(a){ return a } }'
u_mod='static box Vy { method id(a){ return a } }'

json=$(stageb_compile_to_json_with_bundles "$main" "$u_src") || { echo "[FAIL] stageb_bundle_mix_emit_vm (emit failed)" >&2; exit 1; }
if [ -s "$json" ] && head -n1 "$json" | grep -q '"version":0' && head -n1 "$json" | grep -q '"kind":"Program"'; then
  rm -f "$json"; echo "[PASS] stageb_bundle_mix_emit_vm"; exit 0
else
  echo "[FAIL] stageb_bundle_mix_emit_vm (missing header)" >&2
  test -f "$json" && head -n1 "$json" >&2 || true
  rm -f "$json"; exit 1
fi
