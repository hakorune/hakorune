#!/bin/bash
# Phase 29cc PLG-05-min6: Math plugin wave-2 pilot smoke (VM, TimeBox route)
# Contract:
# - TimeBox is loaded via per-run nyash.toml.
# - now method route works in VM and emits a numeric timestamp.

set -euo pipefail

source "$(dirname "$0")/../../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../../lib/plugin_pilot_common.sh"
require_env || exit 2

SMOKE_NAME="phase29cc_plg05_time_pilot_vm"
FIXTURE="$NYASH_ROOT/apps/tests/phase29cc_plg05_time_pilot_min.hako"
WORK_DIR="$(mktemp -d -t phase29cc_plg05_time.XXXXXX)"
OUTPUT_FILE="$WORK_DIR/output.log"

cleanup() {
  rm -rf "$WORK_DIR"
}
trap cleanup EXIT

require_fixture_file "$SMOKE_NAME" "$FIXTURE" || exit 1

EXT="$(detect_lib_ext)"
MATH_LIB_NAME="$(lib_name_for nyash_math_plugin "$EXT")"
MATH_LIB_PATH="$NYASH_ROOT/target/release/$MATH_LIB_NAME"
STRING_LIB_NAME="$(lib_name_for nyash_string_plugin "$EXT")"
STRING_LIB_PATH="$NYASH_ROOT/target/release/$STRING_LIB_NAME"

build_plugin_release_checked "$SMOKE_NAME" "math" "$MATH_LIB_PATH" -p nyash-math-plugin || exit 1
build_string_plugin_release_checked "$SMOKE_NAME" "$STRING_LIB_PATH" || exit 1

cat > "$WORK_DIR/nyash.toml" << EOF2
$(append_stringbox_toml "$STRING_LIB_NAME" "$STRING_LIB_PATH")

[libraries."$MATH_LIB_NAME"]
boxes = ["MathBox", "TimeBox"]
path = "$MATH_LIB_PATH"

[libraries."$MATH_LIB_NAME".MathBox]
type_id = 50
abi_version = 1
singleton = false

[libraries."$MATH_LIB_NAME".MathBox.methods]
birth = { method_id = 0 }
sqrt = { method_id = 1 }
sin = { method_id = 2 }
cos = { method_id = 3 }
round = { method_id = 4 }
fini = { method_id = 4294967295 }

[libraries."$MATH_LIB_NAME".TimeBox]
type_id = 51
abi_version = 1
singleton = false

[libraries."$MATH_LIB_NAME".TimeBox.methods]
birth = { method_id = 0 }
now = { method_id = 1 }
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

if ! grep -Eq '^time_now=[0-9]+$' "$OUTPUT_FILE"; then
  tail -n 120 "$OUTPUT_FILE" || true
  test_fail "$SMOKE_NAME: expected numeric output 'time_now=<digits>' not found"
  exit 1
fi

if grep -q 'Unknown Box type: TimeBox' "$OUTPUT_FILE"; then
  tail -n 120 "$OUTPUT_FILE" || true
  test_fail "$SMOKE_NAME: stale unknown-box regression (TimeBox)"
  exit 1
fi

test_pass "$SMOKE_NAME: PASS (Math/Time plugin pilot locked)"
