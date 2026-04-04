#!/usr/bin/env bash
# Archived pre-promote gate for legacy-main removal.
# Historical engineering helper only; keep it frozen and non-growing.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../../.." && pwd)"
READINESS_SCRIPT="${ROOT}/tools/archive/legacy-selfhost/engineering/legacy_main_readiness.sh"

CLI_MODE="stage1"
BIN_STAGE1="${ROOT}/target/selfhost/hakorune.stage1_cli"
BIN_STAGE2="${ROOT}/target/selfhost/hakorune.stage1_cli.stage2"

usage() {
  cat <<'USAGE'
Usage: bash tools/archive/legacy-selfhost/engineering/pre_promote_legacy_main_removal.sh [options]

Options:
  --cli-mode <mode>       Pass-through to readiness helper (default: stage1)
  --bin-stage1 <path>     Stage1 binary path for smoke
  --bin-stage2 <path>     Stage2 binary path for smoke
  -h, --help              Show this help

Contract:
  - Calls `tools/archive/legacy-selfhost/engineering/legacy_main_readiness.sh --strict`
  - Exit 0 only when legacy-literal removal readiness is satisfied
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --cli-mode)
      [[ $# -ge 2 ]] || { usage; exit 2; }
      CLI_MODE="$2"
      shift 2
      ;;
    --bin-stage1)
      [[ $# -ge 2 ]] || { usage; exit 2; }
      BIN_STAGE1="$2"
      shift 2
      ;;
    --bin-stage2)
      [[ $# -ge 2 ]] || { usage; exit 2; }
      BIN_STAGE2="$2"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "[selfhost/pre-promote] unknown arg: $1" >&2
      usage
      exit 2
      ;;
  esac
done

if [[ ! -x "$READINESS_SCRIPT" ]]; then
  echo "[selfhost/pre-promote] missing readiness script: $READINESS_SCRIPT" >&2
  exit 2
fi

echo "[selfhost/pre-promote] running legacy-main readiness strict gate"
bash "$READINESS_SCRIPT" \
  --strict \
  --cli-mode "$CLI_MODE" \
  --bin-stage1 "$BIN_STAGE1" \
  --bin-stage2 "$BIN_STAGE2"
echo "[selfhost/pre-promote] PASS: legacy-main removal is ready"
