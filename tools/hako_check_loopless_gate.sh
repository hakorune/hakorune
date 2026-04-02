#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
LOG_DIR="${HAKO_CHECK_LOOPLESS_LOG_DIR:-/tmp}"
ONLY="all"

usage() {
  cat <<'EOF'
Usage: tools/hako_check_loopless_gate.sh [--only {quick|joinir|deadcode|run_tests}]

Defaults to running all steps in order.
EOF
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --only=*)
      ONLY="${1#*=}"
      shift
      ;;
    --only)
      ONLY="${2:-}"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "Unknown argument: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

mkdir -p "$LOG_DIR"

run_step() {
  local name="$1"
  shift
  local stamp
  stamp="$(date +%Y%m%d_%H%M%S)"
  local log="$LOG_DIR/hako_check_loopless_${name}_${stamp}_$$.log"
  if ! "$@" 2>&1 | tee "$log"; then
    echo "LOG: $log" >&2
    exit 1
  fi
}

# Diagnostics fixtures (e.g. apps/tests/phase29aq_*_diag_min.hako) are out of gate scope.
case "$ONLY" in
  all)
    run_step quick "$ROOT/tools/smokes/v2/run.sh" --profile quick
    run_step joinir "$ROOT/tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh"
    run_step deadcode "$ROOT/tools/hako_check/deadcode_smoke.sh"
    run_step run_tests bash "$ROOT/tools/hako_check/run_tests.sh"
    ;;
  quick)
    run_step quick "$ROOT/tools/smokes/v2/run.sh" --profile quick
    ;;
  joinir)
    run_step joinir "$ROOT/tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh"
    ;;
  deadcode)
    run_step deadcode "$ROOT/tools/hako_check/deadcode_smoke.sh"
    ;;
  run_tests)
    run_step run_tests bash "$ROOT/tools/hako_check/run_tests.sh"
    ;;
  *)
    echo "Unknown --only target: $ONLY" >&2
    usage >&2
    exit 2
    ;;
esac
