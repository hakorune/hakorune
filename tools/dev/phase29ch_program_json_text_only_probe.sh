#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
source "${ROOT}/tools/selfhost/lib/stage1_contract.sh"

BIN="${ROOT}/target/selfhost/hakorune.stage1_cli"
ENTRY="${ROOT}/apps/tests/hello_simple_llvm.hako"

usage() {
  cat <<'USAGE' >&2
Usage: tools/dev/phase29ch_program_json_text_only_probe.sh [--bin <path>] [entry.hako]

Builds Program(JSON) once, then tries the compat emit-mir contract with only
`*_PROGRAM_JSON_TEXT` populated and no `*_PROGRAM_JSON` path. The current
expected result is nonzero because the compat resolver still needs the path lane.
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --bin)
      if [[ $# -lt 2 ]]; then
        echo "[phase29ch/text-only-probe] --bin requires a path" >&2
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
  echo "[phase29ch/text-only-probe] binary not found: $BIN" >&2
  exit 2
fi
if [[ ! -f "$ENTRY" ]]; then
  echo "[phase29ch/text-only-probe] entry not found: $ENTRY" >&2
  exit 2
fi

tmp_prog="$(mktemp)"
trap 'rm -f "$tmp_prog"' EXIT

bash "${ROOT}/tools/selfhost/run_stage1_cli.sh" --bin "$BIN" emit program-json "$ENTRY" >"$tmp_prog"
program_json_text="$(cat "$tmp_prog")"

set +e
stage1_contract_export_runner_defaults
stage1_contract_run_bin_with_env \
  "$BIN" \
  "emit-mir" \
  "__stage1_program_json__" \
  "$program_json_text" \
  0 \
  1 \
  "" \
  "$program_json_text" \
  /dev/null \
  /dev/null
rc=$?
set -e

echo "bin=${BIN}"
echo "entry=${ENTRY}"
echo "text_only_rc=${rc}"

if [[ "$rc" -eq 0 ]]; then
  echo "[phase29ch/text-only-probe] expected current text-only compat lane to stay nonzero" >&2
  exit 1
fi
