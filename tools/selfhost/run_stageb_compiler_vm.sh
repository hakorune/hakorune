#!/bin/bash
# run_stageb_compiler_vm.sh - shared Stage-B compiler route (VM)
#
# Contract:
# - Input: --source-file <path> (fixture/source file)
# - Output: Program(JSON v0) + diagnostics (stdout/stderr passthrough)
# - Route tag: stderr "[selfhost/route] id=..."
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
NYASH_ROOT="${NYASH_ROOT:-$ROOT_DIR}"
NYASH_BIN="${NYASH_BIN:-$NYASH_ROOT/target/release/hakorune}"
COMPILER="${SELFHOST_COMPILER_ENTRY:-$NYASH_ROOT/lang/src/compiler/entry/compiler_stageb.hako}"

SOURCE_FILE=""
TIMEOUT_SECS="${SMOKES_SELFHOST_STAGEB_TIMEOUT_SECS:-${RUN_TIMEOUT_SECS:-20}}"
ROUTE_ID="${SELFHOST_ROUTE_ID:-SH-GATE-STAGEB}"

usage() {
  cat <<'USAGE' >&2
Usage:
  run_stageb_compiler_vm.sh --source-file <path> [--timeout-secs <n>] [--route-id <id>]
USAGE
}

while [ $# -gt 0 ]; do
  case "$1" in
    --source-file)
      [ $# -ge 2 ] || { usage; exit 2; }
      SOURCE_FILE="$2"
      shift 2
      ;;
    --timeout-secs)
      [ $# -ge 2 ] || { usage; exit 2; }
      TIMEOUT_SECS="$2"
      shift 2
      ;;
    --route-id)
      [ $# -ge 2 ] || { usage; exit 2; }
      ROUTE_ID="$2"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "[selfhost/route] unknown arg: $1" >&2
      usage
      exit 2
      ;;
  esac
done

if [ -z "$SOURCE_FILE" ]; then
  echo "[selfhost/route] missing required --source-file" >&2
  usage
  exit 2
fi

if [[ "$SOURCE_FILE" != /* ]]; then
  SOURCE_FILE="$NYASH_ROOT/$SOURCE_FILE"
fi

if ! [[ "$TIMEOUT_SECS" =~ ^[0-9]+$ ]]; then
  echo "[selfhost/route] invalid --timeout-secs: $TIMEOUT_SECS" >&2
  exit 2
fi

if [ ! -f "$SOURCE_FILE" ]; then
  echo "[selfhost/route] source file not found: $SOURCE_FILE" >&2
  exit 2
fi

if [ ! -x "$NYASH_BIN" ]; then
  echo "[selfhost/route] nyash binary not found/executable: $NYASH_BIN" >&2
  exit 2
fi

if [ ! -f "$COMPILER" ]; then
  echo "[selfhost/route] compiler entry missing: $COMPILER" >&2
  exit 2
fi

if [ "${NYASH_SELFHOST_STAGEB_PROOF_ONLY:-0}" != "1" ]; then
  echo "[selfhost/route] stage-b VM route is proof-only; set NYASH_SELFHOST_STAGEB_PROOF_ONLY=1 to run" >&2
  exit 2
fi

SMOKE_ENV_SKIP_EXPORTS=1
source "$NYASH_ROOT/tools/smokes/v2/lib/env.sh"

if [ -z "${HAKO_STAGEB_MODULES_LIST:-}" ]; then
  HAKO_STAGEB_MODULES_LIST="$(collect_stageb_modules_list "$NYASH_ROOT")"
fi

if [ -z "${HAKO_STAGEB_MODULE_ROOTS_LIST:-}" ]; then
  HAKO_STAGEB_MODULE_ROOTS_LIST="$(collect_stageb_module_roots_list "$NYASH_ROOT")"
fi

echo "[selfhost/route] id=${ROUTE_ID} mode=stageb source=$(basename "$SOURCE_FILE") timeout_secs=${TIMEOUT_SECS}" >&2

# Phase 29x X22: Stage-B gate must stay on Rust VM core lane under strict/dev.
# vm-hako priority is disabled explicitly for this gate route.
HAKO_SRC="$(cat "$SOURCE_FILE")" \
  NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  NYASH_VM_HAKO_PREFER_STRICT_DEV="${NYASH_VM_HAKO_PREFER_STRICT_DEV:-0}" \
  NYASH_DEV="${NYASH_DEV:-0}" \
  NYASH_OPERATOR_BOX_ALL="${NYASH_OPERATOR_BOX_ALL:-0}" \
  NYASH_OPERATOR_BOX_STRINGIFY="${NYASH_OPERATOR_BOX_STRINGIFY:-0}" \
  NYASH_OPERATOR_BOX_COMPARE="${NYASH_OPERATOR_BOX_COMPARE:-0}" \
  NYASH_OPERATOR_BOX_ADD="${NYASH_OPERATOR_BOX_ADD:-0}" \
  NYASH_OPERATOR_BOX_COMPARE_ADOPT="${NYASH_OPERATOR_BOX_COMPARE_ADOPT:-0}" \
  NYASH_OPERATOR_BOX_ADD_ADOPT="${NYASH_OPERATOR_BOX_ADD_ADOPT:-0}" \
  NYASH_BUILDER_OPERATOR_BOX_ALL_CALL="${NYASH_BUILDER_OPERATOR_BOX_ALL_CALL:-0}" \
  NYASH_BUILDER_OPERATOR_BOX_ADD_CALL="${NYASH_BUILDER_OPERATOR_BOX_ADD_CALL:-0}" \
  HAKO_JOINIR_STRICT="${HAKO_JOINIR_STRICT:-1}" \
  HAKO_JOINIR_PLANNER_REQUIRED="${HAKO_JOINIR_PLANNER_REQUIRED:-1}" \
  NYASH_ALLOW_USING_FILE="${NYASH_ALLOW_USING_FILE:-1}" \
  HAKO_ALLOW_USING_FILE="${HAKO_ALLOW_USING_FILE:-1}" \
  NYASH_USING_AST="${NYASH_USING_AST:-1}" \
  NYASH_FEATURES="${NYASH_FEATURES:-stage3,no-try-compat}" \
  NYASH_PARSER_ALLOW_SEMICOLON="${NYASH_PARSER_ALLOW_SEMICOLON:-1}" \
  NYASH_VARMAP_GUARD_STRICT="${NYASH_VARMAP_GUARD_STRICT:-0}" \
  NYASH_BLOCK_SCHEDULE_VERIFY="${NYASH_BLOCK_SCHEDULE_VERIFY:-0}" \
  HAKO_STAGEB_MODULES_LIST="$HAKO_STAGEB_MODULES_LIST" \
  HAKO_STAGEB_MODULE_ROOTS_LIST="$HAKO_STAGEB_MODULE_ROOTS_LIST" \
  NYASH_QUIET="${NYASH_QUIET:-0}" \
  HAKO_QUIET="${HAKO_QUIET:-0}" \
  NYASH_CLI_VERBOSE="${NYASH_CLI_VERBOSE:-0}" \
  timeout "$TIMEOUT_SECS" \
  "$NYASH_BIN" --backend vm "$COMPILER" -- --stage-b --stage3
