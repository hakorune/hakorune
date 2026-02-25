#!/bin/bash
# hv1 inline: unop not on 0 -> rc=1
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

json='{"schema_version":"1.0","functions":[{"name":"main","blocks":[{"id":0,"instructions":[
  {"op":"const","dst":1, "value": {"type": "i64", "value": 0}},
  {"op":"unop","kind":"not","src":1,"dst":2},
  {"op":"ret","value":2}
]}]}]}'

tmp="/tmp/hv1_unop_not_$$.json"; printf '%s' "$json" > "$tmp"
set +e
rc=$(HAKO_PRIMARY_NO_FALLBACK=1 HAKO_VERIFY_PRIMARY=hakovm verify_v1_inline_file "$tmp" || true)
set -e
rm -f "$tmp"

if [[ "$rc" =~ ^-?[0-9]+$ ]] && [ "$rc" -eq 1 ]; then
  echo "[PASS] hv1_inline_unop_not_rc1_canary_vm"
  exit 0
fi
echo "[FAIL] hv1_inline_unop_not_rc1_canary_vm (rc=$rc, expect 1)" >&2
exit 1

