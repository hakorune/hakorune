#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
source "${ROOT}/tools/selfhost/lib/identity_routes.sh"

BIN="${ROOT}/target/selfhost/hakorune.stage1_cli"
ENTRY="${ROOT}/apps/tests/hello_simple_llvm.hako"

usage() {
  cat <<'USAGE' >&2
Usage: tools/dev/phase29ch_program_json_cold_compat_probe.sh [--bin <path>] [entry.hako]

Reports how alternate supplied-Program(JSON) caller shapes collapse onto the
current live compat transport on a compiled stage1-compatible artifact. This is
diagnostics only; current mainline compat uses text transport through
`stage1-env-mir-program`, and both the old env shape and raw
`run_stage1_cli.sh --from-program-json` are tracked here as aliases/sugar
rather than as separate lanes.
USAGE
}

run_cold_legacy_env_mir_route() {
  local bin="$1"
  local entry="$2"
  local program_json_text="$3"

  env \
    "NYASH_NYRT_SILENT_RESULT=${NYASH_NYRT_SILENT_RESULT:-1}" \
    "NYASH_DISABLE_PLUGINS=${NYASH_DISABLE_PLUGINS:-1}" \
    "NYASH_FILEBOX_MODE=${NYASH_FILEBOX_MODE:-core-ro}" \
    "HAKO_SELFHOST_NO_DELEGATE=${HAKO_SELFHOST_NO_DELEGATE:-1}" \
    "HAKO_MIR_BUILDER_DELEGATE=${HAKO_MIR_BUILDER_DELEGATE:-0}" \
    "NYASH_USE_STAGE1_CLI=1" \
    "NYASH_STAGE1_MODE=emit-mir" \
    "HAKO_STAGE1_MODE=emit-mir" \
    "STAGE1_EMIT_PROGRAM_JSON=0" \
    "STAGE1_EMIT_MIR_JSON=1" \
    "HAKO_STAGE1_INPUT=${entry}" \
    "NYASH_STAGE1_INPUT=${entry}" \
    "STAGE1_SOURCE=${entry}" \
    "STAGE1_INPUT=${entry}" \
    "STAGE1_SOURCE_TEXT=${program_json_text}" \
    "STAGE1_PROGRAM_JSON_TEXT=${program_json_text}" \
    "$bin"
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
tmp_prog="${tmp_dir}/program.json"
out_file="${tmp_dir}/mir.json"
route_file="${tmp_dir}/route.txt"

if ! run_stage1_env_route "$BIN" "program-json" "$ENTRY" "$tmp_prog"; then
  echo "[phase29ch/cold-compat-probe] failed to materialize Program(JSON)" >&2
  exit 1
fi

program_json_text="$(cat "$tmp_prog")"

echo "bin=${BIN}"
echo "entry=${ENTRY}"
if run_and_extract_stage_payload \
  "mir-json" \
  "$out_file" \
  run_cold_legacy_env_mir_route "$BIN" "$ENTRY" "$program_json_text"; then
  echo "stage1-env-mir-program" >"$route_file"
  echo "legacy_env_program_json=$(route_file_value "$route_file")"
else
  echo "legacy_env_program_json=none"
fi

if ! run_and_extract_stage_payload \
  "mir-json" \
  "$out_file" \
  bash "${ROOT}/tools/selfhost/run_stage1_cli.sh" --bin "$BIN" emit mir-json --from-program-json "$tmp_prog"; then
  echo "[phase29ch/cold-compat-probe] raw wrapper sugar failed unexpectedly" >&2
  exit 1
fi
echo "raw_wrapper_program_json=stage1-env-mir-program"
