#!/bin/bash
# stageb_bundle_vm.sh — Stage‑B: bundle emit (nyash-toml風の事前定義を模した結合) → ヘッダ検証

set -euo pipefail
if [ "${SMOKES_ENABLE_STAGEB:-0}" != "1" ]; then
  echo "[SKIP] stageb_bundle_vm (SMOKES_ENABLE_STAGEB=1 to enable)"
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

# Bundle snippets (pretend these came from modules resolved via nyash.toml)
b1='static box Util { method nop(args) { local z=0  return z } }'
b2='static box Pair { method of(a,b) { return a } }'

main='static box Main { method main(args) { return 0 } }'

json=$(stageb_compile_to_json_with_bundles "$main" "$b1" "$b2") || { echo "[FAIL] Stage‑B bundle emit failed" >&2; exit 1; }
if stageb_json_nonempty "$json"; then
  rm -f "$json"; echo "[PASS] stageb_bundle_vm"; exit 0
else
  echo "[FAIL] stageb_bundle_vm (emit json missing header)" >&2
  test -f "$json" && { echo "--- json ---" >&2; head -n1 "$json" >&2; }
  rm -f "$json"; exit 1
fi
