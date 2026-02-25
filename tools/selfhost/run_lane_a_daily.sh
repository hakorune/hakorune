#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"

SYNC_GUARD="$ROOT_DIR/tools/checks/phase29bq_joinir_port_sync_guard.sh"
FAST_GATE="$ROOT_DIR/tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh"

usage() {
  cat <<'EOF'
Usage:
  run_lane_a_daily.sh [--guards-only]

Options:
  --guards-only   Run joinir port sync/promotion guard only.
EOF
}

RUN_FAST_GATE=1
if [ "${1:-}" = "--guards-only" ]; then
  RUN_FAST_GATE=0
  shift
elif [ "${1:-}" = "-h" ] || [ "${1:-}" = "--help" ]; then
  usage
  exit 0
fi

if [ -n "${1:-}" ]; then
  echo "[FAIL] Unknown arg: $1" >&2
  usage >&2
  exit 2
fi

bash "$SYNC_GUARD"

if [ "$RUN_FAST_GATE" = "1" ]; then
  bash "$FAST_GATE" --only bq
fi

echo "[lane-a-daily] PASS"
