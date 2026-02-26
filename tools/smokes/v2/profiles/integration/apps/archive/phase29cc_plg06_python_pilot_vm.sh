#!/bin/bash
# Phase 29cc PLG-06-min2: Python plugin wave-3 rollout pilot smoke (VM)
# Contract:
# - PyRuntimeBox is loaded via per-run nyash.toml.
# - evalR route can be invoked in VM.
# - Fixture prints PyObjectBox handle text (pyr_out=PyObjectBox(...)) and exits cleanly.

set -euo pipefail

source "$(dirname "$0")/../../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../../lib/plugin_pilot_common.sh"
require_env || exit 2

SMOKE_NAME="phase29cc_plg06_python_pilot_vm"
FIXTURE="$NYASH_ROOT/apps/tests/phase29cc_plg06_python_pilot_min.hako"
WORK_DIR="$(mktemp -d -t phase29cc_plg06_python.XXXXXX)"
OUTPUT_FILE="$WORK_DIR/output.log"

cleanup() {
  rm -rf "$WORK_DIR"
}
trap cleanup EXIT

require_fixture_file "$SMOKE_NAME" "$FIXTURE" || exit 1

EXT="$(detect_lib_ext)"
PY_LIB_NAME="$(lib_name_for nyash_python_plugin "$EXT")"
PY_LIB_PATH="$NYASH_ROOT/target/release/$PY_LIB_NAME"
STRING_LIB_NAME="$(lib_name_for nyash_string_plugin "$EXT")"
STRING_LIB_PATH="$NYASH_ROOT/target/release/$STRING_LIB_NAME"

build_plugin_release_checked "$SMOKE_NAME" "python" "$PY_LIB_PATH" -p nyash-python-plugin || exit 1
build_string_plugin_release_checked "$SMOKE_NAME" "$STRING_LIB_PATH" || exit 1

cat > "$WORK_DIR/nyash.toml" << EOF2
$(append_stringbox_toml "$STRING_LIB_NAME" "$STRING_LIB_PATH")

[libraries."$PY_LIB_NAME"]
boxes = ["PyRuntimeBox", "PyObjectBox"]
path = "$PY_LIB_PATH"

[libraries."$PY_LIB_NAME".PyRuntimeBox]
type_id = 40
abi_version = 2
singleton = false

[libraries."$PY_LIB_NAME".PyRuntimeBox.methods]
birth = { method_id = 0 }
eval = { method_id = 1, args = ["code"] }
evalR = { method_id = 11, args = ["code"], returns_result = true }
import = { method_id = 2, args = ["module_name"] }
importR = { method_id = 12, args = ["module_name"], returns_result = true }
fini = { method_id = 4294967295 }

[libraries."$PY_LIB_NAME".PyObjectBox]
type_id = 41
abi_version = 2
singleton = false

[libraries."$PY_LIB_NAME".PyObjectBox.methods]
getattr = { method_id = 3, args = ["attr_name"] }
getAttr = { method_id = 3, args = ["attr_name"] }
getattrR = { method_id = 13, args = ["attr_name"], returns_result = true }
getAttrR = { method_id = 13, args = ["attr_name"], returns_result = true }
call = { method_id = 4, args = ["args_json"] }
callR = { method_id = 14, args = ["args_json"], returns_result = true }
callKw = { method_id = 5, args = ["args_json", "kwargs_json"] }
callKW = { method_id = 5, args = ["args_json", "kwargs_json"] }
call_kw = { method_id = 5, args = ["args_json", "kwargs_json"] }
callKwR = { method_id = 15, args = ["args_json", "kwargs_json"], returns_result = true }
callKWR = { method_id = 15, args = ["args_json", "kwargs_json"], returns_result = true }
str = { method_id = 6 }
toString = { method_id = 6 }
birth = { method_id = 0 }
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

if ! grep -Eq 'pyr_out=PyObjectBox\([0-9]+\)' "$OUTPUT_FILE"; then
  tail -n 120 "$OUTPUT_FILE" || true
  test_fail "$SMOKE_NAME: expected output matching 'pyr_out=PyObjectBox(<id>)' not found"
  exit 1
fi

if grep -q 'Unknown Box type: PyRuntimeBox' "$OUTPUT_FILE"; then
  tail -n 120 "$OUTPUT_FILE" || true
  test_fail "$SMOKE_NAME: stale unknown-box regression (PyRuntimeBox)"
  exit 1
fi

test_pass "$SMOKE_NAME: PASS (Python plugin pilot locked)"
