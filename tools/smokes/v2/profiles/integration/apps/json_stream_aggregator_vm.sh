#!/bin/bash
# JSON stream aggregator real-app smoke (VM)
#
# Contract pin:
# - narrow JSONL scanner stays deterministic
# - map-backed per-user aggregation keeps stable accounting

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

APP="$HAKO_ROOT/apps/json-stream-aggregator/main.hako"

if [ ! -f "$APP" ]; then
  test_fail "json_stream_aggregator_vm: App not found: $APP"
  exit 2
fi

output=$(run_hako_vm_release "$APP")

expected=$(cat << 'TXT'
json-stream-aggregator
events=5
users=3
ana_bytes=42 ok=2 fail=0
bob_bytes=27 ok=1 fail=1
cy_bytes=9 ok=1 fail=0
total_bytes=78
ok=4 fail=1
summary=ok
TXT
)

compare_outputs "$expected" "$output" "json_stream_aggregator_vm" || exit 1

test_pass "json_stream_aggregator_vm"
