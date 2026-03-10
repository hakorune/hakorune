#!/usr/bin/env bash
# selfhost_identity_check.sh — G1 gate: Stage1 vs Stage2 identity comparison
#
# Purpose:
#   Verify that Stage1 and Stage2 produce identical Program JSON v0 and MIR JSON v0
#   for the same input, proving selfhost correctness.
#
# Usage:
#   tools/selfhost_identity_check.sh [--mode full|smoke] [--skip-build] [--build-timeout-secs <secs>] [--cli-mode auto|stage1|stage0] [--allow-compat-route] [--bin-stage1 <path>] [--bin-stage2 <path>]
#
# Modes:
#   full  - Use compiler_stageb.hako (G1 done criteria)
#   smoke - Use hello_simple_llvm.hako (quick verification)
#
# Build options:
#   --skip-build     Skip Stage1/Stage2 build (use prebuilt binaries)
#   --build-timeout-secs Timeout for each Stage1/Stage2 build invocation (default: 900, 0 disables)
#   --cli-mode       Emit route for binaries:
#                    stage1=stage1 selfhost emit route (default; env-mainline for stage1-cli artifacts)
#                    stage0=direct flags (--emit-program-json-v0 / --emit-mir-json)
#                    auto=try stage1 selfhost route then fallback to stage0 (compat-only)
#   --allow-compat-route
#                    Required when --cli-mode is auto/stage0 (compat-only lane)
#   --bin-stage1     Path to prebuilt Stage1 binary (default: cli-mode依存)
#   --bin-stage2     Path to prebuilt Stage2 binary (default: cli-mode依存)
#
# Note: Building Stage1/Stage2 requires ~35GB+ RAM. For environments with less memory,
#       use --skip-build with prebuilt binaries from a larger machine.
#
# Exit codes:
#   0: PASS (Stage1 == Stage2)
#   1: FAIL (mismatch)
#   2: BUILD or SETUP error
#
# Evidence SSOT:
#   - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
#   - docs/development/current/main/design/coreplan-migration-roadmap-ssot.md (G1)

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
IDENTITY_LIB_DIR="${ROOT}/tools/selfhost/lib"
source "${IDENTITY_LIB_DIR}/identity_routes.sh"
source "${IDENTITY_LIB_DIR}/identity_compare.sh"

# Defaults
MODE="smoke"
SKIP_BUILD=0
BUILD_TIMEOUT_SECS=900
CLI_MODE="stage1"
ALLOW_COMPAT_ROUTE=0
STAGE1_BIN=""
STAGE2_BIN=""

# Entry points by mode
ENTRY_FULL="${ROOT}/lang/src/compiler/entry/compiler_stageb.hako"
ENTRY_SMOKE="${ROOT}/apps/tests/hello_simple_llvm.hako"

# Silence debug output
export HAKO_JOINIR_DEBUG=0
export NYASH_CLI_VERBOSE=0
export NYASH_JOINIR_DEBUG=0

usage() {
  cat <<'USAGE'
Usage: tools/selfhost_identity_check.sh [--mode full|smoke] [--skip-build] [--build-timeout-secs <secs>] [--cli-mode stage1|auto|stage0] [--allow-compat-route] [--bin-stage1 <path>] [--bin-stage2 <path>]

Options:
  --mode full|smoke  Test mode (default: smoke)
                     full  = compiler_stageb.hako (G1 done criteria)
                     smoke = hello_simple_llvm.hako (quick verification)
  --skip-build       Skip Stage1/Stage2 build (use prebuilt binaries)
  --build-timeout-secs <n>
                     Timeout for each Stage build (default: 900, 0 disables)
  --cli-mode <m>     CLI emit route (default: stage1)
                     stage1 = require stage1 selfhost emit route
                     auto   = try stage1 selfhost route then fallback to stage0 flags (compat-only)
                     stage0 = require stage0 direct flag route
  --allow-compat-route
                     Explicitly allow compat-only route (required for cli-mode=auto|stage0)
  --bin-stage1 <p>   Path to prebuilt Stage1 binary
  --bin-stage2 <p>   Path to prebuilt Stage2 binary
                     default(stage1/auto): target/selfhost/hakorune.stage1_cli(.stage2)
                     default(stage0):      target/selfhost/hakorune(.stage2)
  -h, --help         Show this help

Examples:
  # Build and compare (requires ~35GB+ RAM)
  tools/selfhost_identity_check.sh --mode full

  # Compare only with prebuilt binaries (recommended for <35GB environments)
  tools/selfhost_identity_check.sh --skip-build --bin-stage1 /path/to/stage1 --bin-stage2 /path/to/stage2

  # Smoke test with default binary locations
  tools/selfhost_identity_check.sh --mode smoke --skip-build

  # Compatibility probe (stage0 fallback path)
  tools/selfhost_identity_check.sh --mode smoke --skip-build --cli-mode auto --allow-compat-route --bin-stage1 target/release/hakorune --bin-stage2 target/release/hakorune
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --mode)
      MODE="$2"
      if [[ "$MODE" != "full" && "$MODE" != "smoke" ]]; then
        echo "[G1] invalid mode: $MODE (must be full or smoke)" >&2
        exit 2
      fi
      shift 2
      ;;
    --skip-build)
      SKIP_BUILD=1
      shift
      ;;
    --build-timeout-secs)
      BUILD_TIMEOUT_SECS="$2"
      shift 2
      ;;
    --cli-mode)
      CLI_MODE="$2"
      shift 2
      ;;
    --allow-compat-route)
      ALLOW_COMPAT_ROUTE=1
      shift
      ;;
    --bin-stage1)
      STAGE1_BIN="$2"
      shift 2
      ;;
    --bin-stage2)
      STAGE2_BIN="$2"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "[G1] unknown arg: $1" >&2
      usage
      exit 2
      ;;
  esac
done

if ! [[ "$BUILD_TIMEOUT_SECS" =~ ^[0-9]+$ ]]; then
  echo "[G1] invalid --build-timeout-secs: $BUILD_TIMEOUT_SECS (must be non-negative integer)" >&2
  exit 2
fi
if [[ "$CLI_MODE" != "auto" && "$CLI_MODE" != "stage1" && "$CLI_MODE" != "stage0" ]]; then
  echo "[G1] invalid --cli-mode: $CLI_MODE (must be auto|stage1|stage0)" >&2
  exit 2
fi
if [[ "$CLI_MODE" != "stage1" && "$ALLOW_COMPAT_ROUTE" -ne 1 ]]; then
  echo "[G1] compat route requires explicit opt-in: --allow-compat-route (cli-mode=${CLI_MODE})" >&2
  echo "     stage0/auto are compatibility-only and not accepted as main-route evidence" >&2
  exit 2
fi

# Set default binary paths if not specified.
# stage1/auto mode defaults to stage1-cli artifacts; stage0 mode keeps launcher artifacts.
if [[ -z "$STAGE1_BIN" ]]; then
  if [[ "$CLI_MODE" == "stage0" ]]; then
    STAGE1_BIN="${ROOT}/target/selfhost/hakorune"
  else
    STAGE1_BIN="${ROOT}/target/selfhost/hakorune.stage1_cli"
  fi
fi
if [[ -z "$STAGE2_BIN" ]]; then
  if [[ "$CLI_MODE" == "stage0" ]]; then
    STAGE2_BIN="${ROOT}/target/selfhost/hakorune.stage2"
  else
    STAGE2_BIN="${ROOT}/target/selfhost/hakorune.stage1_cli.stage2"
  fi
fi

# Select entry by mode
if [[ "$MODE" == "full" ]]; then
  ENTRY="$ENTRY_FULL"
  echo "[G1] Mode: full (G1 done criteria)" >&2
else
  ENTRY="$ENTRY_SMOKE"
  echo "[G1] Mode: smoke (quick verification only)" >&2
fi

if [[ ! -f "$ENTRY" ]]; then
  echo "[G1:FAIL] entry not found: $ENTRY" >&2
  exit 2
fi

# --- Build Stage1 and Stage2 ---
if [[ "$SKIP_BUILD" -eq 0 ]]; then
  BUILD_ARTIFACT_KIND="stage1-cli"
  if [[ "$CLI_MODE" == "stage0" ]]; then
    BUILD_ARTIFACT_KIND="launcher-exe"
  fi
  echo "[G1] Building Stage1 binary..." >&2
  echo "[G1] WARNING: Build requires ~35GB+ RAM. Use --skip-build for smaller environments." >&2
  echo "[G1] Build artifact-kind: ${BUILD_ARTIFACT_KIND} (cli-mode=${CLI_MODE})" >&2
  if ! bash "${ROOT}/tools/selfhost/build_stage1.sh" --artifact-kind "$BUILD_ARTIFACT_KIND" --out "$STAGE1_BIN" --timeout-secs "$BUILD_TIMEOUT_SECS" >/dev/null 2>&1; then
    echo "[G1:FAIL] Stage1 build failed (likely OOM - try --skip-build with prebuilt binaries)" >&2
    if [[ "$BUILD_TIMEOUT_SECS" -gt 0 ]]; then
      echo "          hint: increase --build-timeout-secs if this was a timeout" >&2
    fi
    exit 2
  fi

  echo "[G1] Building Stage2 binary (using Stage1 as bootstrap)..." >&2
  STAGE2_BUILD_PREFIX=(NYASH_BIN="$STAGE1_BIN")
  if [[ "$BUILD_ARTIFACT_KIND" == "stage1-cli" ]]; then
    echo "[G1] Stage2 build route: stage1-cli bridge-first bootstrap (reduced case)" >&2
  fi
  if ! env "${STAGE2_BUILD_PREFIX[@]}" bash "${ROOT}/tools/selfhost/build_stage1.sh" --artifact-kind "$BUILD_ARTIFACT_KIND" --out "$STAGE2_BIN" --timeout-secs "$BUILD_TIMEOUT_SECS" >/dev/null 2>&1; then
    echo "[G1:FAIL] Stage2 build failed (likely OOM - try --skip-build with prebuilt binaries)" >&2
    if [[ "$BUILD_TIMEOUT_SECS" -gt 0 ]]; then
      echo "          hint: increase --build-timeout-secs if this was a timeout" >&2
    fi
    exit 2
  fi
else
  echo "[G1] Skip build mode: using prebuilt binaries" >&2
  echo "[G1] Stage1: $STAGE1_BIN" >&2
  echo "[G1] Stage2: $STAGE2_BIN" >&2
fi

# Verify binaries exist
if [[ ! -x "$STAGE1_BIN" ]]; then
  echo "[G1:FAIL] Stage1 binary not found: $STAGE1_BIN" >&2
  if [[ "$SKIP_BUILD" -eq 1 ]]; then
    echo "         Provide prebuilt binary via --bin-stage1" >&2
  else
    echo "         Run: tools/selfhost/build_stage1.sh (requires ~35GB+ RAM)" >&2
  fi
  exit 2
fi

if [[ ! -x "$STAGE2_BIN" ]]; then
  echo "[G1:FAIL] Stage2 binary not found: $STAGE2_BIN" >&2
  if [[ "$SKIP_BUILD" -eq 1 ]]; then
    echo "         Provide prebuilt binary via --bin-stage2" >&2
  else
    echo "         Run: NYASH_BIN=$STAGE1_BIN tools/selfhost/build_stage1.sh --out $STAGE2_BIN" >&2
  fi
  exit 2
fi

# --- Temp files ---
TMP_DIR=$(mktemp -d)
cleanup() { rm -rf "$TMP_DIR" 2>/dev/null || true; }
trap cleanup EXIT

S1_PROG="${TMP_DIR}/s1_program.json"
S2_PROG="${TMP_DIR}/s2_program.json"
S1_MIR="${TMP_DIR}/s1_mir.json"
S2_MIR="${TMP_DIR}/s2_mir.json"

# Route evidence (auto-mode detection)
S1_ROUTE_FILE="${TMP_DIR}/stage1.route"
S2_ROUTE_FILE="${TMP_DIR}/stage2.route"

emit_and_validate_stage_payload() {
  local stage_label="$1"
  local bin="$2"
  local subcmd="$3"
  local entry="$4"
  local outfile="$5"
  local route_file="$6"

  if ! run_stage_cli "$bin" "$subcmd" "$entry" "$outfile" "$route_file"; then
    echo "[G1:FAIL] ${stage_label} emit ${subcmd} failed" >&2
    return 1
  fi
  validate_emit_payload "$subcmd" "$outfile" "$stage_label"
}


if [[ "$MODE" == "full" && "$CLI_MODE" != "stage0" ]]; then
  echo "[G1] Preflight: checking Stage1 CLI emit capability..." >&2
  if ! preflight_stage1_cli "Stage1" "$STAGE1_BIN"; then
    exit 2
  fi
  if ! preflight_stage1_cli "Stage2" "$STAGE2_BIN"; then
    exit 2
  fi
fi

# --- Program JSON v0 comparison ---
echo "[G1] Comparing Program JSON v0..." >&2

if ! emit_and_validate_stage_payload "Stage1" "$STAGE1_BIN" "program-json" "$ENTRY" "$S1_PROG" "$S1_ROUTE_FILE"; then
  exit 1
fi
if ! emit_and_validate_stage_payload "Stage2" "$STAGE2_BIN" "program-json" "$ENTRY" "$S2_PROG" "$S2_ROUTE_FILE"; then
  exit 1
fi

if ! compare_emit_outputs "Program JSON v0" "$S1_PROG" "$S2_PROG" "/tmp/g1_program_diff.txt"; then
  exit 1
fi

# Route snapshot after Program(JSON) comparison
S1_ROUTE="$(route_file_value "$S1_ROUTE_FILE")"
S2_ROUTE="$(route_file_value "$S2_ROUTE_FILE")"

# In full mode, require true stage1 CLI route for G1 evidence
if [[ "$MODE" == "full" ]]; then
  if ! require_stage1_route_for_full_mode "emit" "$S1_ROUTE" "$S2_ROUTE" \
    "use --cli-mode stage1 with real Stage1/Stage2 binaries for G1 evidence"; then
    exit 2
  fi
  if ! require_exact_stage1_route_for_full_mode "program-json" "stage1-env-program" "$S1_ROUTE" "$S2_ROUTE" \
    "current reduced authority pins program-json on stage1 env mainline"; then
    exit 2
  fi
fi

# In smoke mode, stage0 route is treated as compatibility check only.
# MIR JSON can differ in block-id numbering even for equivalent semantics.
if [[ "$MODE" == "smoke" && ( ! "$S1_ROUTE" =~ ^stage1 || ! "$S2_ROUTE" =~ ^stage1 ) ]]; then
  echo "[G1] smoke note: non-stage1 route detected (stage1_bin=$S1_ROUTE stage2_bin=$S2_ROUTE)" >&2
  echo "[G1] smoke note: skip MIR strict diff for compatibility probe" >&2
  echo "[G1:PASS] Stage1 == Stage2 identity verified (smoke mode - compatibility route)"
  exit 0
fi

# --- MIR JSON v0 comparison ---
echo "[G1] Comparing MIR JSON v0..." >&2

if ! emit_and_validate_stage_payload "Stage1" "$STAGE1_BIN" "mir-json" "$ENTRY" "$S1_MIR" "$S1_ROUTE_FILE"; then
  exit 1
fi
if ! emit_and_validate_stage_payload "Stage2" "$STAGE2_BIN" "mir-json" "$ENTRY" "$S2_MIR" "$S2_ROUTE_FILE"; then
  exit 1
fi

# In full mode, MIR must also stay on stage1 route (no stage0 compatibility fallback).
if [[ "$MODE" == "full" ]]; then
  S1_ROUTE="$(route_file_value "$S1_ROUTE_FILE")"
  S2_ROUTE="$(route_file_value "$S2_ROUTE_FILE")"
  if ! require_stage1_route_for_full_mode "mir-json" "$S1_ROUTE" "$S2_ROUTE" \
    "use --cli-mode stage1 with stage1-cli artifacts"; then
    exit 2
  fi
  if ! require_exact_stage1_route_for_full_mode "mir-json" "stage1-env-mir-source" "$S1_ROUTE" "$S2_ROUTE" \
    "current reduced authority pins mir-json on single-step source->MIR env mainline"; then
    exit 2
  fi
fi

if ! compare_emit_outputs "MIR JSON v0" "$S1_MIR" "$S2_MIR" "/tmp/g1_mir_diff.txt"; then
  exit 1
fi

# --- Success ---
if [[ "$MODE" == "full" ]]; then
  echo "[G1:PASS] Stage1 == Stage2 identity verified (full mode)"
else
  echo "[G1:PASS] Stage1 == Stage2 identity verified (smoke mode - not G1 done criteria)"
fi
exit 0
