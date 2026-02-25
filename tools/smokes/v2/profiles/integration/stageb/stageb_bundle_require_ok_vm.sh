#!/bin/bash
# stageb_bundle_require_ok_vm.sh — Stage‑B: require-mod satisfied → ヘッダ検証（helpers経由）

set -euo pipefail
if [ "${SMOKES_ENABLE_STAGEB:-0}" != "1" ]; then
  echo "[SKIP] stageb_bundle_require_ok_vm (SMOKES_ENABLE_STAGEB=1 to enable)"
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
util='static box Util { method nop(a) { return a } }'

json_path=$(stageb_compile_to_json_with_bundles "$main" "$util") || { echo "[FAIL] stageb_bundle_require_ok_vm (emit failed)" >&2; exit 1; }
if [ -s "$json_path" ] && head -n1 "$json_path" | grep -q '"version":0' && head -n1 "$json_path" | grep -q '"kind":"Program"'; then
  rm -f "$json_path"; echo "[PASS] stageb_bundle_require_ok_vm"; exit 0
else
  echo "[FAIL] stageb_bundle_require_ok_vm (missing header)" >&2
  test -f "$json_path" && head -n1 "$json_path" >&2 || true
  rm -f "$json_path"; exit 1
fi
