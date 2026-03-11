#!/usr/bin/env bash
# run_stage1_cli.sh — helper to invoke Stage1 hakorune CLI with required env
#
# Responsibilities
# - Locate the Stage1 binary (default: target/selfhost/hakorune).
# - Ensure NyRT emits clean JSON/stdout by default (NYASH_NYRT_SILENT_RESULT=1).
# - Apply minimal runtime defaults so FileBox/hostbridge work without plugins.
# - Forward all remaining arguments to the Stage1 binary and propagate exit code.

set -euo pipefail

usage() {
  cat <<'USAGE' >&2
Usage: tools/selfhost/run_stage1_cli.sh [--bin <path>] [--] <hakorune-args...>

Defaults:
  --bin <path>   : default target/selfhost/hakorune
  env overrides  :
      NYASH_NYRT_SILENT_RESULT=1 (when unset)
      NYASH_DISABLE_PLUGINS=1    (when unset)
      NYASH_FILEBOX_MODE=core-ro (when unset)

Examples:
  tools/selfhost/run_stage1_cli.sh emit program-json apps/tests/minimal.hako
  tools/selfhost/run_stage1_cli.sh --bin /tmp/hakorune-dev emit mir-json apps/tests/minimal.hako
USAGE
}

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
BIN="${ROOT_DIR}/target/selfhost/hakorune"
source "${ROOT_DIR}/tools/selfhost/lib/stage1_contract.sh"

read_entry_source_text() {
  local entry="$1"
  if [[ -z "$entry" ]]; then
    echo "[run-stage1] source path is required" >&2
    exit 2
  fi
  if [[ ! -f "$entry" ]]; then
    echo "[run-stage1] source not found: $entry" >&2
    exit 2
  fi
  stage1_contract_source_text "$entry"
}

run_emit_program_json() {
  if [[ $# -ne 1 ]]; then
    echo "[run-stage1] usage: emit program-json <source.hako>" >&2
    exit 2
  fi
  local entry="$1"
  local source_text
  source_text="$(read_entry_source_text "$entry")"
  stage1_contract_exec_mode "$BIN" "emit-program" "$entry" "$source_text"
}

run_emit_mir_json() {
  local entry=""
  while [[ $# -gt 0 ]]; do
    case "$1" in
      --from-program-json)
        echo "[run-stage1] emit mir-json: --from-program-json is retired from this wrapper" >&2
        echo "             use tools/dev/phase29ch_program_json_compat_route_probe.sh or stage1_contract_exec_program_json_compat" >&2
        exit 2
        ;;
      *)
        if [[ -n "$entry" ]]; then
          echo "[run-stage1] emit mir-json: unexpected extra argument: $1" >&2
          exit 2
        fi
        entry="$1"
        shift
        ;;
    esac
  done

  if [[ -z "$entry" ]]; then
    echo "[run-stage1] emit mir-json: require <source.hako>" >&2
    exit 2
  fi

  run_emit_mir_json_from_source "$entry"
}

run_emit_mir_json_from_source() {
  local entry="$1"
  local source_text
  source_text="$(read_entry_source_text "$entry")"
  stage1_contract_exec_mode "$BIN" "emit-mir" "$entry" "$source_text"
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --bin)
      if [[ $# -lt 2 ]]; then
        echo "[run-stage1] --bin requires a path" >&2
        exit 2
      fi
      BIN="$2"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    --)
      shift
      break
      ;;
    *)
      break
      ;;
  esac
done

if [[ ! -x "$BIN" ]]; then
  echo "[run-stage1] Stage1 binary not found at $BIN" >&2
  echo "             Build via: tools/selfhost/build_stage1.sh" >&2
  exit 1
fi

if [[ $# -lt 1 ]]; then
  echo "[run-stage1] missing hakorune CLI arguments" >&2
  usage
  exit 2
fi

# Default env toggles for Stage1 CLI execution (shared contract SSOT)
stage1_contract_export_runner_defaults

if [[ "$1" == "emit" ]]; then
  if [[ $# -lt 2 ]]; then
    echo "[run-stage1] emit requires a subcommand" >&2
    exit 2
  fi
  subcmd="$2"
  shift 2
  case "$subcmd" in
    program-json)
      run_emit_program_json "$@"
      exit $?
      ;;
    mir-json)
      run_emit_mir_json "$@"
      exit $?
      ;;
  esac
fi

exec "$BIN" "$@"
