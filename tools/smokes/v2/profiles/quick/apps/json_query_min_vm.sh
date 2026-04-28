#!/bin/bash
# json_query_min_vm.sh — Minimal JSON query (VM)

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2
preflight_plugins || exit 2

APP_DIR="$NYASH_ROOT/apps/examples/json_query_min"
export NYASH_ALLOW_USING_FILE=1
# This fixture pins deterministic minimal JSON-query output and should not
# depend on unrelated JoinIR strict/dev lowering behavior.
output=$(
  NYASH_JOINIR_DEV=0 \
  HAKO_JOINIR_STRICT=0 \
  NYASH_JOINIR_STRICT=0 \
  HAKO_JOINIR_PLANNER_REQUIRED=0 \
  run_nyash_vm "$APP_DIR/main.hako" --dev
)

expected=$(cat << 'TXT'
2
TXT
)

compare_outputs "$expected" "$output" "json_query_min_vm" || exit 1
