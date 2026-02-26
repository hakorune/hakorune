#!/bin/bash
# llvm_exe_runner.sh - Shared helpers for LLVM EXE parity smokes (integration)
#
# SSOT goals:
# - One place for "LLVM available?" SKIP logic
# - One place for "required plugins are dlopen-able" gating + conditional build-all
# - One place for "build_llvm.sh → run exe → numeric output compare"
#
# This file is meant to be sourced from smoke scripts that already source:
#   tools/smokes/v2/lib/test_runner.sh

set -uo pipefail

# Source centralized environment configuration (SSOT)
LIB_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
if [ -f "$LIB_DIR/env.sh" ]; then
  source "$LIB_DIR/env.sh"
fi

# ============================================================================
# Helper: require_joinir_dev
# ============================================================================
# Sets dev-only environment variables for JoinIR normalized shadow testing.
# Must be called BEFORE build/run operations if the fixture requires dev-only features.
#
# Usage:
#   require_joinir_dev
#
# NOTE: This now validates that env.sh has been sourced correctly.
#
require_joinir_dev() {
  # Verify env.sh provided the defaults
  if [ "${NYASH_JOINIR_DEV:-0}" != "1" ]; then
    export NYASH_JOINIR_DEV=1
  fi
  if [ "${HAKO_JOINIR_STRICT:-0}" != "1" ]; then
    export HAKO_JOINIR_STRICT=1
  fi
  echo "[INFO] JoinIR dev mode enabled (NYASH_JOINIR_DEV=1, HAKO_JOINIR_STRICT=1)"
}

# ============================================================================
# Helper: check_output_contract
# ============================================================================
# Unified output validation interface for exit_code, numeric, and substring checks.
#
# Args:
#   $1: contract_type - "exit_code" / "numeric" / "substring"
#   $2: expected      - Expected value (string/number)
#   $3: actual        - Actual value to compare
#   $4: context       - (optional) Context description for error messages
#
# Returns:
#   0 if contract satisfied, 1 otherwise
#
check_output_contract() {
  local contract_type="${1:-}"
  local expected="${2:-}"
  local actual="${3:-}"
  local context="${4:-output}"

  if [ -z "$contract_type" ] || [ -z "$expected" ]; then
    echo "[FAIL] check_output_contract: missing contract_type or expected value"
    return 1
  fi

  case "$contract_type" in
    exit_code)
      if [ "$actual" -ne "$expected" ] 2>/dev/null; then
        echo "[FAIL] OutputContract(exit_code): got $actual, expected $expected"
        return 1
      fi
      echo "[PASS] OutputContract(exit_code): $actual == $expected"
      return 0
      ;;

    numeric)
      if [ "$actual" != "$expected" ]; then
        echo "[FAIL] OutputContract(numeric): got '$actual', expected '$expected'"
        echo "[INFO] Context: $context"
        return 1
      fi
      echo "[PASS] OutputContract(numeric): $actual == $expected"
      return 0
      ;;

    substring)
      if ! printf "%s" "$actual" | grep -qF "$expected"; then
        echo "[FAIL] OutputContract(substring): '$expected' not found in $context"
        echo "[INFO] Actual output (first 200 chars):"
        printf "%s" "$actual" | head -c 200
        echo ""
        return 1
      fi
      echo "[PASS] OutputContract(substring): '$expected' found in $context"
      return 0
      ;;

    *)
      echo "[FAIL] check_output_contract: unknown contract_type '$contract_type'"
      return 1
      ;;
  esac
}

llvm_exe_strip_noise() {
  # SSOT for non-payload lines produced by runtime/plugin boot.
  grep -Ev '^\[|^Net plugin:|^$'
}

llvm_exe_first_payload_line() {
  local output="${1:-}"
  printf "%s\n" "$output" | llvm_exe_strip_noise | head -n 1 | tr -d '\r'
}

llvm_exe_cargo_target_dir() {
  # Use the workspace target dir by default so `NYASH_BIN` and plugin artifacts match local dev expectations.
  local target_dir="${LLVM_EXE_CARGO_TARGET_DIR:-$NYASH_ROOT/target}"
  mkdir -p "$target_dir"
  echo "$target_dir"
}

llvm_exe_preflight_or_skip() {
  if ! command -v llvm-config-18 &>/dev/null; then
    test_skip "llvm-config-18 not found"
    return 1
  fi

  if ! python3 -c "import llvmlite" 2>/dev/null; then
    test_skip "Python llvmlite not found"
    return 1
  fi

  return 0
}

llvm_exe_check_plugins() {
  # Args: repeated triples: <DisplayName> <SoPath> <CrateName>
  python3 - "$@" <<'PY'
import ctypes
import os
import sys

args = sys.argv[1:]
if len(args) % 3 != 0:
    print(f"[internal] expected triples, got {len(args)} args")
    sys.exit(2)

failures = []
for i in range(0, len(args), 3):
    display, path, _crate = args[i], args[i + 1], args[i + 2]
    if not os.path.isfile(path):
        failures.append(f"[plugin/missing] {display}: {path}")
        continue
    try:
        ctypes.CDLL(path)
    except Exception as e:  # noqa: BLE001
        failures.append(f"[plugin/dlopen] {display}: {path} ({e})")

if failures:
    print("\n".join(failures))
    sys.exit(1)
print("OK")
PY
}

llvm_exe_ensure_plugins_or_fail() {
  # Uses global array LLVM_REQUIRED_PLUGINS (triples encoded as "Display|SoPath|CrateName")
  if ! declare -p LLVM_REQUIRED_PLUGINS >/dev/null 2>&1; then
    return 0
  fi
  if [ "${#LLVM_REQUIRED_PLUGINS[@]}" -eq 0 ]; then
    return 0
  fi

  local -a triples=()
  local -a crate_names=()
  local CHECK_OUTPUT
  local item display so_path crate_name

  for item in "${LLVM_REQUIRED_PLUGINS[@]}"; do
    IFS='|' read -r display so_path crate_name <<<"$item"
    if [ -z "${display:-}" ] || [ -z "${so_path:-}" ] || [ -z "${crate_name:-}" ]; then
      echo "[FAIL] Invalid LLVM_REQUIRED_PLUGINS entry: '$item'"
      return 1
    fi
    triples+=("$display" "$so_path" "$crate_name")
    crate_names+=("$crate_name")
  done

  echo "[INFO] Checking plugin artifacts (LLVM EXE)"
  if CHECK_OUTPUT=$(llvm_exe_check_plugins "${triples[@]}" 2>&1); then
    return 0
  fi

  echo "$CHECK_OUTPUT"
  echo "[INFO] Missing/broken plugin detected, running build-all"

  local cargo_target_dir
  cargo_target_dir="$(llvm_exe_cargo_target_dir)"

  local build_log="${LLVM_PLUGIN_BUILD_LOG:-/tmp/llvm_exe_plugin_build.log}"
  if ! env CARGO_TARGET_DIR="$cargo_target_dir" bash "$NYASH_ROOT/tools/plugins/build-all.sh" "${crate_names[@]}" >"$build_log" 2>&1; then
    echo "[FAIL] tools/plugins/build-all.sh failed"
    tail -n 80 "$build_log"
    return 1
  fi

  if ! CHECK_OUTPUT=$(llvm_exe_check_plugins "${triples[@]}" 2>&1); then
    echo "$CHECK_OUTPUT"
    echo "[FAIL] Plugin artifacts still missing or unloadable after build-all"
    tail -n 80 "$build_log"
    return 1
  fi

  return 0
}

llvm_exe_build_and_run_numeric_smoke() {
  # Required globals:
  # - INPUT_HAKO
  # - OUTPUT_EXE
  # - EXPECTED (multiline)
  # - EXPECTED_LINES (number of numeric lines to compare)
  #
  # Fixture contract (LLVM EXE):
  # - Program should exit with code 0.
  # - Prefer `return 0` in fixture main() to avoid handle-id exit collisions.
  #
  # Optional:
  # - LLVM_BUILD_LOG
  # - RUN_TIMEOUT_SECS

  if [ -z "${INPUT_HAKO:-}" ] || [ -z "${OUTPUT_EXE:-}" ] || [ -z "${EXPECTED:-}" ]; then
    echo "[FAIL] llvm_exe_build_and_run_numeric_smoke: missing INPUT_HAKO/OUTPUT_EXE/EXPECTED"
    return 1
  fi
  if [ -z "${EXPECTED_LINES:-}" ]; then
    echo "[FAIL] llvm_exe_build_and_run_numeric_smoke: missing EXPECTED_LINES"
    return 1
  fi

  mkdir -p "$(dirname "$OUTPUT_EXE")"

  echo "[INFO] Building: $INPUT_HAKO → $OUTPUT_EXE"

  local cargo_target_dir
  cargo_target_dir="$(llvm_exe_cargo_target_dir)"

  # Ensure we use the compiler binary built in that target dir.
  local nyash_bin="$cargo_target_dir/release/hakorune"
  local obj_out="$cargo_target_dir/aot_objects/$(basename "$INPUT_HAKO").o"
  mkdir -p "$(dirname "$obj_out")"

  local build_log="${LLVM_BUILD_LOG:-/tmp/llvm_exe_build.log}"
  if ! env CARGO_TARGET_DIR="$cargo_target_dir" NYASH_BIN="$nyash_bin" NYASH_LLVM_OBJ_OUT="$obj_out" NYASH_DISABLE_PLUGINS=0 \
    "$NYASH_ROOT/tools/build_llvm.sh" "$INPUT_HAKO" -o "$OUTPUT_EXE" 2>&1 | tee "$build_log"; then
    echo "[FAIL] build_llvm.sh failed"
    tail -n 80 "$build_log"
    return 1
  fi

  if [ ! -x "$OUTPUT_EXE" ]; then
    echo "[FAIL] Executable not created or not executable: $OUTPUT_EXE"
    ls -la "$OUTPUT_EXE" 2>/dev/null || echo "File does not exist"
    return 1
  fi

  echo "[INFO] Executing: $OUTPUT_EXE"

  set +e
  local output
  output=$(timeout "${RUN_TIMEOUT_SECS:-10}" env NYASH_DISABLE_PLUGINS=0 "$OUTPUT_EXE" 2>&1)
  local exit_code=$?
  set -e

  if [ "$exit_code" -ne 0 ]; then
    echo "[FAIL] Execution failed with exit code $exit_code"
    echo "$output" | tail -n 80
    return 1
  fi

  local clean
  clean=$(printf "%s\n" "$output" | llvm_exe_strip_noise | grep -E '^-?[0-9]+$' | head -n "$EXPECTED_LINES" | tr -d '\r')

  echo "[INFO] CLEAN output:"
  echo "$clean"

  if check_output_contract "numeric" "$EXPECTED" "$clean" "numeric stdout (first $EXPECTED_LINES lines)"; then
    return 0
  fi

  echo "[INFO] Raw output (tail):"
  echo "$output" | tail -n 80
  return 1
}

llvm_exe_build_and_run_expect_exit_code() {
  # Required globals:
  # - INPUT_HAKO
  # - OUTPUT_EXE
  # - EXPECTED_EXIT_CODE
  #
  # Optional:
  # - LLVM_BUILD_LOG
  # - RUN_TIMEOUT_SECS

  if [ -z "${INPUT_HAKO:-}" ] || [ -z "${OUTPUT_EXE:-}" ] || [ -z "${EXPECTED_EXIT_CODE:-}" ]; then
    echo "[FAIL] llvm_exe_build_and_run_expect_exit_code: missing INPUT_HAKO/OUTPUT_EXE/EXPECTED_EXIT_CODE"
    return 1
  fi

  mkdir -p "$(dirname "$OUTPUT_EXE")"

  echo "[INFO] Building: $INPUT_HAKO → $OUTPUT_EXE"

  local cargo_target_dir
  cargo_target_dir="$(llvm_exe_cargo_target_dir)"

  # Ensure we use the compiler binary built in that target dir.
  local nyash_bin="$cargo_target_dir/release/hakorune"
  local obj_out="$cargo_target_dir/aot_objects/$(basename "$INPUT_HAKO").o"
  mkdir -p "$(dirname "$obj_out")"

  local build_log="${LLVM_BUILD_LOG:-/tmp/llvm_exe_build.log}"
  if ! env CARGO_TARGET_DIR="$cargo_target_dir" NYASH_BIN="$nyash_bin" NYASH_LLVM_OBJ_OUT="$obj_out" NYASH_DISABLE_PLUGINS=0 \
    "$NYASH_ROOT/tools/build_llvm.sh" "$INPUT_HAKO" -o "$OUTPUT_EXE" 2>&1 | tee "$build_log"; then
    echo "[FAIL] build_llvm.sh failed"
    tail -n 80 "$build_log"
    return 1
  fi

  if [ ! -x "$OUTPUT_EXE" ]; then
    echo "[FAIL] Executable not created or not executable: $OUTPUT_EXE"
    ls -la "$OUTPUT_EXE" 2>/dev/null || echo "File does not exist"
    return 1
  fi

  echo "[INFO] Executing: $OUTPUT_EXE"

  set +e
  local output
  output=$(timeout "${RUN_TIMEOUT_SECS:-10}" env NYASH_DISABLE_PLUGINS=0 "$OUTPUT_EXE" 2>&1)
  local exit_code=$?
  set -e

  if check_output_contract "exit_code" "$EXPECTED_EXIT_CODE" "$exit_code" "executable exit code"; then
    return 0
  fi

  echo "[INFO] Raw output (tail):"
  echo "$output" | tail -n 80
  return 1
}
