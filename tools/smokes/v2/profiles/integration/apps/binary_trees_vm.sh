#!/bin/bash
# binary-trees real-app smoke (VM)
#
# Contract pin:
# - recursive allocation and checksum route stays executable
# - short-lived and long-lived tree shapes stay deterministic

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

APP="$HAKO_ROOT/apps/binary-trees/main.hako"

if [ ! -f "$APP" ]; then
  test_fail "binary_trees_vm: App not found: $APP"
  exit 2
fi

output=$(run_hako_vm_release "$APP")

expected=$(cat << 'TXT'
binary-trees
stretch_depth=7 check=-1
long_lived_depth=6 check=-1
iterations_depth_4=64 check=-128
iterations_depth_6=16 check=-32
summary=ok
TXT
)

compare_outputs "$expected" "$output" "binary_trees_vm" || exit 1

test_pass "binary_trees_vm"
