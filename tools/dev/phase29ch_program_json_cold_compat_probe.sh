#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
source "${ROOT}/tools/selfhost/lib/identity_routes.sh"

BIN="${ROOT}/target/selfhost/hakorune.stage1_cli"
ENTRY="${ROOT}/apps/tests/hello_simple_llvm.hako"

usage() {
  cat <<'USAGE' >&2
Usage: tools/dev/phase29ch_program_json_cold_compat_probe.sh [--bin <path>] [entry.hako]

Reports whether the remaining cold Program(JSON) compatibility routes are still
accepted on the given compiled stage1-compatible artifact. This is diagnostics
only; current mainline compat uses text transport through `stage1-env-mir-program`.
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --bin)
      if [[ $# -lt 2 ]]; then
        echo "[phase29ch/cold-compat-probe] --bin requires a path" >&2
        exit 2
      fi
      BIN="$2"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      ENTRY="$1"
      shift
      ;;
  esac
done

if [[ ! -x "$BIN" ]]; then
  echo "[phase29ch/cold-compat-probe] binary not found: $BIN" >&2
  exit 2
fi
if [[ ! -f "$ENTRY" ]]; then
  echo "[phase29ch/cold-compat-probe] entry not found: $ENTRY" >&2
  exit 2
fi

tmp_dir="$(mktemp -d)"
trap 'cleanup_stage_temp_dir "$tmp_dir"' EXIT
out_file="${tmp_dir}/mir.json"
route_file="${tmp_dir}/route.txt"

echo "bin=${BIN}"
echo "entry=${ENTRY}"
if probe_stage1_env_mir_program_cold_compat_route "$BIN" "$ENTRY" "$out_file" "$route_file"; then
  echo "cold_compat_route=$(route_file_value "$route_file")"
else
  echo "cold_compat_route=none"
fi
