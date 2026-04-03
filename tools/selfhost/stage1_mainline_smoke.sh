#!/usr/bin/env bash
set -euo pipefail

# stage1_mainline_smoke.sh — current Stage1 mainline emit smoke
#
# Purpose:
#   - Exercise the current Stage1 shell contract via `run_stage1_cli.sh`.
#   - Verify `emit mir-json` on a stage1-cli artifact without touching the
#     legacy embedded bridge smoke.

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
RUN_STAGE1="$ROOT_DIR/tools/selfhost/run_stage1_cli.sh"

usage() {
  cat <<'USAGE' >&2
Usage: tools/selfhost/stage1_mainline_smoke.sh [--bin <path>] [<source.hako>]

Defaults:
  --bin      : prefer target/selfhost/hakorune.stage1_cli.stage2, then target/selfhost/hakorune.stage1_cli
  <source>   : apps/tests/hello_simple_llvm.hako

Behavior:
  - runs `tools/selfhost/run_stage1_cli.sh --bin <bin> emit mir-json <source>`
  - requires MIR(JSON) output to contain `"functions"`

This is the current mainline Stage1 smoke.
For the legacy embedded bridge smoke, use `tools/stage1_smoke.sh`.
USAGE
}

resolve_bin() {
  local candidates=(
    "$ROOT_DIR/target/selfhost/hakorune.stage1_cli.stage2"
    "$ROOT_DIR/target/selfhost/hakorune.stage1_cli"
  )
  local candidate
  for candidate in "${candidates[@]}"; do
    if [[ -x "$candidate" ]]; then
      printf '%s\n' "$candidate"
      return 0
    fi
  done
  return 1
}

BIN=""
ENTRY="$ROOT_DIR/apps/tests/hello_simple_llvm.hako"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --bin)
      if [[ $# -lt 2 ]]; then
        echo "[stage1-mainline-smoke] --bin requires a path" >&2
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
      ENTRY="$1"
      shift
      ;;
  esac
done

if [[ -z "$BIN" ]]; then
  if ! BIN="$(resolve_bin)"; then
    echo "[stage1-mainline-smoke] stage1-cli artifact not found" >&2
    echo "  build with: tools/selfhost/build_stage1.sh --artifact-kind stage1-cli" >&2
    exit 97
  fi
fi

if [[ "$BIN" != /* ]]; then
  BIN="$ROOT_DIR/$BIN"
fi
if [[ "$ENTRY" != /* ]]; then
  ENTRY="$ROOT_DIR/$ENTRY"
fi

if [[ ! -x "$BIN" ]]; then
  echo "[stage1-mainline-smoke] binary not found/executable: $BIN" >&2
  exit 97
fi
if [[ ! -f "$ENTRY" ]]; then
  echo "[stage1-mainline-smoke] source not found: $ENTRY" >&2
  exit 2
fi

OUT="$(mktemp --suffix .stage1_mainline_mir.json)"
ERR="$(mktemp --suffix .stage1_mainline_mir.err)"
trap 'rm -f "$OUT" "$ERR"' EXIT

echo "[stage1-mainline-smoke] emit mir-json: $ENTRY" >&2
if ! bash "$RUN_STAGE1" --bin "$BIN" emit mir-json "$ENTRY" >"$OUT" 2>"$ERR"; then
  echo "[stage1-mainline-smoke] emit mir-json failed" >&2
  sed -n '1,80p' "$ERR" >&2 || true
  exit 1
fi

if ! rg -q '"functions"' "$OUT"; then
  echo "[stage1-mainline-smoke] MIR(JSON) output missing functions marker" >&2
  sed -n '1,40p' "$OUT" >&2 || true
  sed -n '1,40p' "$ERR" >&2 || true
  exit 1
fi

echo "[stage1-mainline-smoke] PASS ($(basename "$BIN"))" >&2
