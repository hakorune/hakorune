#!/bin/bash
# Phase 29cc PLG-05-min7: Net plugin wave-2 pilot smoke (VM, ResponseBox route)
# Contract:
# - ResponseBox is loaded via per-run nyash.toml.
# - setHeader/getHeader/write/readBody routes work in VM.
# - Fixture prints net_header=ok and net_body=body-123 and exits cleanly.

set -euo pipefail

source "$(dirname "$0")/../../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../../lib/plugin_pilot_common.sh"
require_env || exit 2

SMOKE_NAME="phase29cc_plg05_net_pilot_vm"
FIXTURE="$NYASH_ROOT/apps/tests/phase29cc_plg05_net_pilot_min.hako"
WORK_DIR="$(mktemp -d -t phase29cc_plg05_net.XXXXXX)"
OUTPUT_FILE="$WORK_DIR/output.log"

cleanup() {
  rm -rf "$WORK_DIR"
}
trap cleanup EXIT

require_fixture_file "$SMOKE_NAME" "$FIXTURE" || exit 1

EXT="$(detect_lib_ext)"
NET_LIB_NAME="$(lib_name_for nyash_net_plugin "$EXT")"
NET_LIB_PATH="$NYASH_ROOT/target/release/$NET_LIB_NAME"
STRING_LIB_NAME="$(lib_name_for nyash_string_plugin "$EXT")"
STRING_LIB_PATH="$NYASH_ROOT/target/release/$STRING_LIB_NAME"

build_plugin_release_checked "$SMOKE_NAME" "net" "$NET_LIB_PATH" -p nyash-net-plugin || exit 1
build_string_plugin_release_checked "$SMOKE_NAME" "$STRING_LIB_PATH" || exit 1

cat > "$WORK_DIR/nyash.toml" << EOF2
$(append_stringbox_toml "$STRING_LIB_NAME" "$STRING_LIB_PATH")

[libraries."$NET_LIB_NAME"]
boxes = ["ResponseBox"]
path = "$NET_LIB_PATH"

[libraries."$NET_LIB_NAME".ResponseBox]
type_id = 22
abi_version = 1
singleton = false

[libraries."$NET_LIB_NAME".ResponseBox.methods]
birth = { method_id = 0 }
setStatus = { method_id = 1 }
setHeader = { method_id = 2 }
write = { method_id = 3 }
readBody = { method_id = 4 }
getStatus = { method_id = 5 }
getHeader = { method_id = 6 }
fini = { method_id = 4294967295 }
EOF2

cp "$FIXTURE" "$WORK_DIR/main.hako"

set +e
(
  cd "$WORK_DIR"
  NYASH_DISABLE_PLUGINS=0 \
  NYASH_VM_HAKO_PREFER_STRICT_DEV=0 \
  NYASH_VM_USE_FALLBACK=0 \
  "$NYASH_BIN" --backend vm ./main.hako >"$OUTPUT_FILE" 2>&1
)
RC=$?
set -e

if [ "$RC" -ne 0 ]; then
  tail -n 120 "$OUTPUT_FILE" || true
  test_fail "$SMOKE_NAME: vm run failed rc=$RC"
  exit 1
fi

if ! grep -q 'net_header=ok' "$OUTPUT_FILE"; then
  tail -n 120 "$OUTPUT_FILE" || true
  test_fail "$SMOKE_NAME: expected output 'net_header=ok' not found"
  exit 1
fi

if ! grep -q 'net_body=body-123' "$OUTPUT_FILE"; then
  tail -n 120 "$OUTPUT_FILE" || true
  test_fail "$SMOKE_NAME: expected output 'net_body=body-123' not found"
  exit 1
fi

if grep -q 'Unknown Box type: ResponseBox' "$OUTPUT_FILE"; then
  tail -n 120 "$OUTPUT_FILE" || true
  test_fail "$SMOKE_NAME: stale unknown-box regression (ResponseBox)"
  exit 1
fi

test_pass "$SMOKE_NAME: PASS (Net plugin pilot locked)"
