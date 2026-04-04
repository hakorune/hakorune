#!/bin/bash
# run.sh - unified selfhost facade (parser + thin dispatch)
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
NYASH_ROOT="${NYASH_ROOT:-$ROOT_DIR}"
NYASH_BIN="${NYASH_BIN:-$NYASH_ROOT/target/release/hakorune}"

mode=""
source_file=""
runtime_input="apps/examples/string_p0.hako"
runtime_mode="exe"
runtime_route=""
runtime_mode_explicit="0"
timeout_secs=""
timeout_ms=""
max_cases=""
case_filter=""
planner_required="${HAKO_JOINIR_PLANNER_REQUIRED:-1}"
route_id=""
jobs=""
steady_with_runtime_parity="0"
steady_no_collect_blocker="0"
steady_quiet="0"
steady_cleanup_old_logs="0"

RUN_HELPERS="$NYASH_ROOT/tools/selfhost/lib/selfhost_run_routes.sh"
if [ ! -f "$RUN_HELPERS" ]; then
  echo "[selfhost/run] helper file not found: $RUN_HELPERS" >&2
  exit 2
fi
source "$RUN_HELPERS"

usage() {
  cat <<'USAGE' >&2
Usage:
  run.sh --gate [--max-cases <n>] [--filter <substring>] [--jobs <n>] [--timeout-secs <n>] [--planner-required 0|1]
  run.sh --steady-state [--with-runtime-parity] [--no-collect-blocker] [--quiet] [--cleanup-old-logs]
  run.sh --runtime [--runtime-route <mainline|compat>] [--runtime-mode <exe|stage-a-compat>] [--input <file>] [--timeout-ms <n>] [--timeout-secs <n>]  # runtime-mode remains a compatibility alias; canonical route names are mainline|compat
  run.sh --direct --source-file <file> [--timeout-secs <n>] [--route-id <id>]

Examples:
  tools/selfhost/run.sh --gate --max-cases 5
  tools/selfhost/run.sh --gate --max-cases 20 --jobs 4
  tools/selfhost/run.sh --steady-state
  tools/selfhost/run.sh --steady-state --with-runtime-parity
  tools/selfhost/run.sh --steady-state --no-collect-blocker
  tools/selfhost/run.sh --steady-state --quiet
  tools/selfhost/run.sh --steady-state --cleanup-old-logs
  tools/selfhost/run.sh --runtime --input apps/examples/string_p0.hako
  tools/selfhost/run.sh --runtime --runtime-route mainline --input apps/examples/string_p0.hako
  tools/selfhost/run.sh --runtime --runtime-mode exe --input apps/examples/string_p0.hako
  tools/selfhost/run.sh --runtime --runtime-mode stage-a-compat --input apps/examples/string_p0.hako
  tools/selfhost/run.sh --direct --source-file apps/tests/phase29bq_selfhost_cleanup_only_min.hako
USAGE
}

set_mode() {
  local next_mode="$1"
  if [ -n "$mode" ] && [ "$mode" != "$next_mode" ]; then
    echo "[selfhost/run] mode already selected: $mode" >&2
    exit 2
  fi
  mode="$next_mode"
}

while [ $# -gt 0 ]; do
  case "$1" in
    --gate)
      set_mode "gate"
      shift
      ;;
    --runtime)
      set_mode "runtime"
      shift
      ;;
    --steady-state)
      set_mode "steady-state"
      shift
      ;;
    --direct)
      set_mode "direct"
      shift
      ;;
    --with-runtime-parity)
      steady_with_runtime_parity="1"
      shift
      ;;
    --no-collect-blocker)
      steady_no_collect_blocker="1"
      shift
      ;;
    --quiet)
      steady_quiet="1"
      shift
      ;;
    --cleanup-old-logs)
      steady_cleanup_old_logs="1"
      shift
      ;;
    --source-file)
      [ $# -ge 2 ] || { usage; exit 2; }
      source_file="$2"
      shift 2
      ;;
    --input)
      [ $# -ge 2 ] || { usage; exit 2; }
      runtime_input="$2"
      shift 2
      ;;
    --runtime-mode)
      [ $# -ge 2 ] || { usage; exit 2; }
      runtime_mode="$2"
      runtime_mode_explicit="1"
      shift 2
      ;;
    --runtime-route)
      [ $# -ge 2 ] || { usage; exit 2; }
      runtime_route="$2"
      shift 2
      ;;
    --timeout-secs)
      [ $# -ge 2 ] || { usage; exit 2; }
      timeout_secs="$2"
      shift 2
      ;;
    --timeout-ms)
      [ $# -ge 2 ] || { usage; exit 2; }
      timeout_ms="$2"
      shift 2
      ;;
    --max-cases)
      [ $# -ge 2 ] || { usage; exit 2; }
      max_cases="$2"
      shift 2
      ;;
    --filter)
      [ $# -ge 2 ] || { usage; exit 2; }
      case_filter="$2"
      shift 2
      ;;
    --planner-required)
      [ $# -ge 2 ] || { usage; exit 2; }
      planner_required="$2"
      shift 2
      ;;
    --jobs)
      [ $# -ge 2 ] || { usage; exit 2; }
      jobs="$2"
      shift 2
      ;;
    --route-id)
      [ $# -ge 2 ] || { usage; exit 2; }
      route_id="$2"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "[selfhost/run] unknown arg: $1" >&2
      usage
      exit 2
      ;;
  esac
done

if [ -z "$mode" ]; then
  echo "[selfhost/run] choose one mode: --gate | --steady-state | --runtime | --direct" >&2
  usage
  exit 2
fi

canonical_runtime_route() {
  case "$1" in
    mainline|exe)
      echo "mainline"
      ;;
    compat|stage-a|stage-a-compat)
      echo "compat"
      ;;
    *)
      return 1
      ;;
  esac
}

# Canonical runtime surface is route-first (`mainline|compat`).
# Keep `--runtime-mode exe|stage-a-compat` and `stage-a` as compatibility aliases only.
if [ "$mode" = "runtime" ]; then
  runtime_route_from_mode=""
  runtime_route_from_arg=""

  if [ "$runtime_mode_explicit" = "1" ] && [ -n "$runtime_mode" ]; then
    runtime_route_from_mode="$(canonical_runtime_route "$runtime_mode" 2>/dev/null || true)"
  fi
  if [ -n "$runtime_route" ]; then
    runtime_route_from_arg="$(canonical_runtime_route "$runtime_route" 2>/dev/null || true)"
  fi

  if [ -n "$runtime_route" ] && [ -z "$runtime_route_from_arg" ]; then
    echo "[selfhost/run] --runtime-route must be mainline|compat (compat aliases: stage-a|stage-a-compat; got: $runtime_route)" >&2
    exit 2
  fi
  if [ "$runtime_mode_explicit" = "1" ] && [ -z "$runtime_route_from_mode" ]; then
    echo "[selfhost/run] --runtime-mode must be exe|stage-a-compat (alias: stage-a; got: $runtime_mode)" >&2
    exit 2
  fi
  if [ -n "$runtime_route_from_arg" ] && [ -n "$runtime_route_from_mode" ] && [ "$runtime_route_from_arg" != "$runtime_route_from_mode" ]; then
    echo "[selfhost/run] --runtime-route and --runtime-mode disagree: route=$runtime_route mode=$runtime_mode" >&2
    exit 2
  fi

  runtime_route="${runtime_route_from_arg:-$runtime_route_from_mode}"
  if [ -z "$runtime_route" ]; then
    runtime_route="mainline"
  fi

  case "$runtime_route" in
    mainline)
      runtime_mode="exe"
      ;;
    compat)
      runtime_mode="stage-a-compat"
      ;;
  esac
fi

if [ "$mode" != "gate" ] && [ -n "$jobs" ]; then
  echo "[selfhost/run] --jobs is only valid with --gate" >&2
  exit 2
fi

if [ "$mode" != "steady-state" ] && [ "$steady_with_runtime_parity" = "1" ]; then
  echo "[selfhost/run] --with-runtime-parity is only valid with --steady-state" >&2
  exit 2
fi

if [ "$mode" != "steady-state" ] && [ "$steady_no_collect_blocker" = "1" ]; then
  echo "[selfhost/run] --no-collect-blocker is only valid with --steady-state" >&2
  exit 2
fi

if [ "$mode" != "steady-state" ] && [ "$steady_quiet" = "1" ]; then
  echo "[selfhost/run] --quiet is only valid with --steady-state" >&2
  exit 2
fi

if [ "$mode" != "steady-state" ] && [ "$steady_cleanup_old_logs" = "1" ]; then
  echo "[selfhost/run] --cleanup-old-logs is only valid with --steady-state" >&2
  exit 2
fi

if [ -n "$timeout_secs" ] && ! [[ "$timeout_secs" =~ ^[0-9]+$ ]]; then
  echo "[selfhost/run] --timeout-secs must be integer: $timeout_secs" >&2
  exit 2
fi

if [ -n "$timeout_ms" ] && ! [[ "$timeout_ms" =~ ^[0-9]+$ ]]; then
  echo "[selfhost/run] --timeout-ms must be integer: $timeout_ms" >&2
  exit 2
fi

case "$mode" in
  gate) run_gate ;;
  steady-state) run_steady_state ;;
  runtime) run_runtime ;;
  direct) run_direct ;;
  *)
    echo "[selfhost/run] internal error: unknown mode $mode" >&2
    exit 2
    ;;
esac
