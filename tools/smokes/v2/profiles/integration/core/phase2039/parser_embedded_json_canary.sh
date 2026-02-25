#!/bin/bash
# Test: hv1 verify direct with env JSON (primary route)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR"/../../../../../../../../.. && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

# Build minimal Program(JSON v0) and verify via builder→Core driver (hv1 route inside)
tmp_prog="/tmp/test_prog_v0_$$.json"
cat > "$tmp_prog" <<'PROG'
{"version":0,"kind":"Program","body":[{"type":"Return","expr":{"type":"Int","value":0}}]}
PROG

set +e
if verify_program_via_builder_to_core "$tmp_prog"; then
  rc=0
else
  rc=1
fi
set -e
rm -f "$tmp_prog"

if [ "$rc" -eq 0 ]; then
  echo "[PASS] parser_embedded_json_canary"
  exit 0
fi
echo "[FAIL] parser_embedded_json_canary (builder→Core verify failed)" >&2
exit 1
