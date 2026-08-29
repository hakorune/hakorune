#!/usr/bin/env bash
# stage1_entry_shape_guard.sh — keep canonical Stage1 defaults at Main.main/0
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="stage1-entry-shape"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CANONICAL_LAUNCHER="$ROOT_DIR/lang/src/runner/entry/launcher_native_entry.hako"
CANONICAL_STAGE1="$ROOT_DIR/lang/src/runner/entry/stage1_cli_env_entry.hako"
LAUNCHER_WRAPPER="$ROOT_DIR/lang/src/runner/launcher_native_entry.hako"
STAGE1_WRAPPER="$ROOT_DIR/lang/src/runner/stage1_cli_env_entry.hako"
BUILD_SCRIPT="$ROOT_DIR/tools/selfhost/mainline/build_stage1.sh"
MODULE_MANIFEST="$ROOT_DIR/lang/src/runner/hako_module.toml"
STAGE1_ENV="$ROOT_DIR/src/runner/stage1_bridge/env.rs"
CARD="$ROOT_DIR/docs/development/current/main/investigations/mir-call-d1b-main-raw-cataloged-handoff-d0-2026-08-28.toml"

guard_require_command "$TAG" rg
guard_require_command "$TAG" awk
guard_require_files "$TAG" \
  "$CANONICAL_LAUNCHER" \
  "$CANONICAL_STAGE1" \
  "$LAUNCHER_WRAPPER" \
  "$STAGE1_WRAPPER" \
  "$BUILD_SCRIPT" \
  "$MODULE_MANIFEST" \
  "$STAGE1_ENV" \
  "$CARD"

check_canonical() {
  local file="$1"
  local label="$2"
  local main_count
  main_count="$(rg -c '^[[:space:]]*main\(\)[[:space:]]*\{' "$file" || true)"
  [[ "$main_count" == "1" ]] || \
    guard_fail "$TAG" "$label must expose exactly one Main.main/0 declaration"
  if rg -n '^[[:space:]]*main\([^)]+' "$file"; then
    guard_fail "$TAG" "$label contains a nonzero-arity main declaration"
  fi
}

check_canonical "$CANONICAL_LAUNCHER" "launcher canonical stub"
check_canonical "$CANONICAL_STAGE1" "stage1-cli canonical stub"

guard_expect_fixed_in_file "$TAG" \
  'ENTRY_DEFAULT_LAUNCHER="$ROOT/lang/src/runner/entry/launcher_native_entry.hako"' \
  "$BUILD_SCRIPT" "launcher default must use canonical entry stub"
guard_expect_fixed_in_file "$TAG" \
  'ENTRY_DEFAULT_STAGE1_CLI="$ROOT/lang/src/runner/entry/stage1_cli_env_entry.hako"' \
  "$BUILD_SCRIPT" "stage1-cli default must use canonical entry stub"
guard_expect_fixed_in_file "$TAG" \
  'entry.launcher_native_entry = "entry/launcher_native_entry.hako"' \
  "$MODULE_MANIFEST" "module manifest must export canonical launcher entry"
guard_expect_fixed_in_file "$TAG" \
  'entry.stage1_cli_env_entry = "entry/stage1_cli_env_entry.hako"' \
  "$MODULE_MANIFEST" "module manifest must export canonical stage1 entry"
guard_expect_fixed_in_file "$TAG" 'Main.main/0' "$STAGE1_ENV" \
  "Stage1 environment contract must retain Main.main/0"

if rg -n 'lang/src/runner/(launcher_native_entry|stage1_cli_env_entry)\.hako' \
  "$BUILD_SCRIPT" "$MODULE_MANIFEST"; then
  guard_fail "$TAG" "default artifact wiring must not name a top-level compatibility wrapper"
fi

guard_expect_fixed_in_file "$TAG" 'STAGE1-ENTRY-SHAPE-I0' "$CARD" \
  "active card must name the bounded entry-shape row"
guard_expect_fixed_in_file "$TAG" 'ParkedSealed' "$CARD" \
  "active card must keep wrapper route explicitly parked"

echo "[$TAG] canonical defaults=Main.main/0 wrappers=explicit-compat-only"
