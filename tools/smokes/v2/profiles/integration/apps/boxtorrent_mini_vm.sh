#!/bin/bash
# BoxTorrent mini real-app smoke (VM)
#
# Contract pin:
# - content-addressed chunks are stable
# - duplicate ingests reuse chunks and bump refcounts
# - manifest materialization reconstructs the original payload

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

APP="$NYASH_ROOT/apps/boxtorrent-mini/main.hako"

if [ ! -f "$APP" ]; then
  test_fail "boxtorrent_mini_vm: App not found: $APP"
  exit 2
fi

output=$(SMOKES_CLEAN_ENV=1 \
  NYASH_DISABLE_PLUGINS=1 \
  NYASH_VM_HAKO_PREFER_STRICT_DEV=0 \
  NYASH_VM_USE_FALLBACK=0 \
  NYASH_JOINIR_DEV=0 \
  HAKO_JOINIR_DEV=0 \
  NYASH_JOINIR_STRICT=0 \
  HAKO_JOINIR_STRICT=0 \
  run_nyash_vm "$APP")

expected=$(cat << 'TXT'
boxtorrent-mini
root=bt-59-458365
chunks=5
bytes=40
dedupe_hits=5
root_equal=true
roundtrip=true
ref_before_release=2
ref_after_release=1
summary=ok
TXT
)

compare_outputs "$expected" "$output" "boxtorrent_mini_vm" || exit 1

test_pass "boxtorrent_mini_vm"
