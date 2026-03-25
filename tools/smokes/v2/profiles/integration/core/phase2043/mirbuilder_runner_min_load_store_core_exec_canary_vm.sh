#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

tmp_json="/tmp/program_runner_min_load_store_$$.json"
cat > "$tmp_json" <<'JSON'
{"version":0,"kind":"Program","body":[
  {"type":"Return","expr":{"type":"Int","value":0}}
]}
JSON

trap 'rm -f "$tmp_json" || true' EXIT

run_verify_canary_and_expect_rc \
  run_verify_program_via_builder_only_to_core \
  "$tmp_json" \
  0 \
  "mirbuilder_runner_min_load_store_core_exec_canary_vm" \
  "mirbuilder_runner_min_load_store_core_exec_canary_vm"
