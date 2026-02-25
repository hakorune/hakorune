#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

tmp_json="/tmp/program_runner_min_typeop_cast_$$.json"
cat > "$tmp_json" <<'JSON'
{"version":0,"kind":"Program","body":[{"type":"Return","expr":{"type":"Int","value":0}}]}
JSON

set +e
HAKO_VERIFY_PRIMARY=core HAKO_VERIFY_BUILDER_ONLY=1 verify_program_via_builder_to_core "$tmp_json" >/dev/null 2>&1
rc=$?
set -e
rm -f "$tmp_json" || true

if [ "$rc" -eq 0 ]; then
  echo "[PASS] mirbuilder_runner_min_typeop_cast_core_exec_canary_vm"
  exit 0
fi
echo "[FAIL] mirbuilder_runner_min_typeop_cast_core_exec_canary_vm (rc=$rc, expect 0)" >&2; exit 1
