#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
source "${ROOT}/tools/selfhost/lib/identity_routes.sh"
source "${ROOT}/tools/selfhost/lib/stage1_contract.sh"

BIN="${ROOT}/target/selfhost/hakorune.stage1_cli"
ENTRY="${ROOT}/apps/tests/hello_simple_llvm.hako"

usage() {
  cat <<'USAGE' >&2
Usage: tools/dev/phase29ch_program_json_compat_route_probe.sh [--bin <path>] [entry.hako]

Reports which explicit Program(JSON) compatibility route is actually
used for `emit mir-json` on the given compiled stage1-compatible artifact.
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --bin)
      if [[ $# -lt 2 ]]; then
        echo "[phase29ch/compat-probe] --bin requires a path" >&2
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
  echo "[phase29ch/compat-probe] binary not found: $BIN" >&2
  exit 2
fi
if [[ ! -f "$ENTRY" ]]; then
  echo "[phase29ch/compat-probe] entry not found: $ENTRY" >&2
  exit 2
fi

tmp_dir="$(mktemp -d)"
tmp_prog="${tmp_dir}/program.json"
out_file="${tmp_dir}/mir.json"
route_file="${tmp_dir}/route.txt"

if ! run_stage1_env_route "$BIN" "program-json" "$ENTRY" "$tmp_prog"; then
  cleanup_stage_temp_dir "$tmp_dir"
  echo "[phase29ch/compat-probe] failed to materialize Program(JSON)" >&2
  exit 1
fi

program_json_text="$(cat "$tmp_prog")"
if ! run_and_extract_stage_payload \
  "mir-json" \
  "$out_file" \
  stage1_contract_exec_program_json_compat "$BIN" "$program_json_text"; then
  cleanup_stage_temp_dir "$tmp_dir"
  echo "[phase29ch/compat-probe] compat route failed" >&2
  exit 1
fi

echo "stage1-env-mir-program" >"$route_file"

route="$(route_file_value "$route_file")"
echo "bin=${BIN}"
echo "entry=${ENTRY}"
echo "compat_route=${route}"

cleanup_stage_temp_dir "$tmp_dir"
