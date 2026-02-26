#!/bin/bash
# Phase 29cc PLG-05-min1: Json plugin wave-2 pilot smoke (VM)
# Contract:
# - JsonDocBox/JsonNodeBox are loaded via per-run nyash.toml.
# - StringBox is also loaded as dependency for string argument marshalling.
# - parse/root/get/int/str method route works in VM.
# - Fixture prints json_kind=Program and json_stmt_size=0 and exits cleanly.

set -euo pipefail

source "$(dirname "$0")/../../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../../lib/plugin_pilot_common.sh"
require_env || exit 2

SMOKE_NAME="phase29cc_plg05_json_pilot_vm"
FIXTURE="$NYASH_ROOT/apps/tests/phase29cc_plg05_json_pilot_min.hako"
WORK_DIR="$(mktemp -d -t phase29cc_plg05_json.XXXXXX)"
OUTPUT_FILE="$WORK_DIR/output.log"

cleanup() {
  rm -rf "$WORK_DIR"
}
trap cleanup EXIT

require_fixture_file "$SMOKE_NAME" "$FIXTURE" || exit 1

EXT="$(detect_lib_ext)"
JSON_LIB_NAME="$(lib_name_for nyash_json_plugin "$EXT")"
JSON_LIB_PATH="$NYASH_ROOT/target/release/$JSON_LIB_NAME"
STRING_LIB_NAME="$(lib_name_for nyash_string_plugin "$EXT")"
STRING_LIB_PATH="$NYASH_ROOT/target/release/$STRING_LIB_NAME"

build_plugin_release_checked "$SMOKE_NAME" "json" "$JSON_LIB_PATH" -p nyash-json-plugin || exit 1
build_string_plugin_release_checked "$SMOKE_NAME" "$STRING_LIB_PATH" || exit 1

cat > "$WORK_DIR/nyash.toml" << EOF2
$(append_stringbox_toml "$STRING_LIB_NAME" "$STRING_LIB_PATH")

[libraries."$JSON_LIB_NAME"]
boxes = ["JsonDocBox", "JsonNodeBox"]
path = "$JSON_LIB_PATH"

[libraries."$JSON_LIB_NAME".JsonDocBox]
type_id = 70
abi_version = 1
singleton = false

[libraries."$JSON_LIB_NAME".JsonDocBox.methods]
birth = { method_id = 0 }
parse = { method_id = 1 }
root = { method_id = 2 }
error = { method_id = 3 }
fini = { method_id = 4294967295 }

[libraries."$JSON_LIB_NAME".JsonNodeBox]
type_id = 71
abi_version = 1
singleton = false

[libraries."$JSON_LIB_NAME".JsonNodeBox.methods]
birth = { method_id = 0 }
kind = { method_id = 1 }
get = { method_id = 2 }
size = { method_id = 3 }
at = { method_id = 4 }
str = { method_id = 5 }
int = { method_id = 6 }
bool = { method_id = 7 }
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

if ! grep -q 'json_kind=Program' "$OUTPUT_FILE"; then
  tail -n 120 "$OUTPUT_FILE" || true
  test_fail "$SMOKE_NAME: expected output 'json_kind=Program' not found"
  exit 1
fi

if ! grep -q 'json_stmt_size=0' "$OUTPUT_FILE"; then
  tail -n 120 "$OUTPUT_FILE" || true
  test_fail "$SMOKE_NAME: expected output 'json_stmt_size=0' not found"
  exit 1
fi

if grep -q 'Unknown Box type: JsonDocBox' "$OUTPUT_FILE"; then
  tail -n 120 "$OUTPUT_FILE" || true
  test_fail "$SMOKE_NAME: stale unknown-box regression (JsonDocBox)"
  exit 1
fi

if grep -q 'Unknown Box type: JsonNodeBox' "$OUTPUT_FILE"; then
  tail -n 120 "$OUTPUT_FILE" || true
  test_fail "$SMOKE_NAME: stale unknown-box regression (JsonNodeBox)"
  exit 1
fi

test_pass "$SMOKE_NAME: PASS (Json plugin pilot locked)"
