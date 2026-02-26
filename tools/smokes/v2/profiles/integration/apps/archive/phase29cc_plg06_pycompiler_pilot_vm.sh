#!/bin/bash
# Phase 29cc PLG-06-min1: PythonCompiler plugin wave-3 entry pilot smoke (VM)
# Contract:
# - PythonCompilerBox is loaded via per-run nyash.toml.
# - compile method route can be invoked in VM.
# - Fixture currently prints pyc_out=void (known bridge contract) and exits cleanly.
# Debt tag:
# - [plg06/pycompiler:return-bridge] compile result is not bridged to caller yet.
#   When fixed, replace this contract with non-void payload assertion.

set -euo pipefail

source "$(dirname "$0")/../../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../../lib/plugin_pilot_common.sh"
require_env || exit 2

SMOKE_NAME="phase29cc_plg06_pycompiler_pilot_vm"
FIXTURE="$NYASH_ROOT/apps/tests/phase29cc_plg06_pycompiler_pilot_min.hako"
WORK_DIR="$(mktemp -d -t phase29cc_plg06_pycompiler.XXXXXX)"
OUTPUT_FILE="$WORK_DIR/output.log"

cleanup() {
  rm -rf "$WORK_DIR"
}
trap cleanup EXIT

require_fixture_file "$SMOKE_NAME" "$FIXTURE" || exit 1

EXT="$(detect_lib_ext)"
PYCOMP_LIB_NAME="$(lib_name_for nyash_python_compiler_plugin "$EXT")"
PYCOMP_LIB_PATH="$NYASH_ROOT/target/release/$PYCOMP_LIB_NAME"
STRING_LIB_NAME="$(lib_name_for nyash_string_plugin "$EXT")"
STRING_LIB_PATH="$NYASH_ROOT/target/release/$STRING_LIB_NAME"

build_plugin_release_checked "$SMOKE_NAME" "python-compiler" "$PYCOMP_LIB_PATH" -p nyash-python-compiler-plugin || exit 1
build_string_plugin_release_checked "$SMOKE_NAME" "$STRING_LIB_PATH" || exit 1

cat > "$WORK_DIR/nyash.toml" << EOF2
$(append_stringbox_toml "$STRING_LIB_NAME" "$STRING_LIB_PATH")

[libraries."$PYCOMP_LIB_NAME"]
boxes = ["PythonCompilerBox"]
path = "$PYCOMP_LIB_PATH"

[libraries."$PYCOMP_LIB_NAME".PythonCompilerBox]
type_id = 61
abi_version = 1
singleton = false

[libraries."$PYCOMP_LIB_NAME".PythonCompilerBox.methods]
birth = { method_id = 0 }
compile = { method_id = 1, args = ["ir_json"], returns_result = true }
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

if ! grep -q 'pyc_out=void' "$OUTPUT_FILE"; then
  tail -n 120 "$OUTPUT_FILE" || true
  test_fail "$SMOKE_NAME: expected output 'pyc_out=void' not found"
  exit 1
fi

if grep -q 'Unknown Box type: PythonCompilerBox' "$OUTPUT_FILE"; then
  tail -n 120 "$OUTPUT_FILE" || true
  test_fail "$SMOKE_NAME: stale unknown-box regression (PythonCompilerBox)"
  exit 1
fi

test_pass "$SMOKE_NAME: PASS (PythonCompiler plugin pilot locked)"
