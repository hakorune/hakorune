#!/usr/bin/env bash
# emit_mir_route.sh
# Common emit entry for smoke scripts.
# Routes:
#   - direct        : hakorune --emit-mir-json
#   - hako-mainline : tools/hakorune_emit_mir.sh with selfhost-first + no-delegate + mainline-only
#   - hako-helper   : tools/hakorune_emit_mir.sh default route

set -euo pipefail

usage() {
  cat <<'USAGE'
Usage:
  emit_mir_route.sh --route <direct|hako-mainline|hako-helper> --out <mir.json> --input <src.hako> [--timeout-secs <n>] [-- <extra args>]

Notes:
  - route=direct uses:      <NYASH_BIN> --emit-mir-json <out> <input>
  - route=hako-mainline uses helper with:
      HAKO_SELFHOST_BUILDER_FIRST=1
      HAKO_SELFHOST_NO_DELEGATE=1
      HAKO_EMIT_MIR_MAINLINE_ONLY=1
  - route=hako-helper uses helper default behavior.
  - caller environment is inherited (joinir flags, fallback flags, etc.).
USAGE
}

ROOT="$(cd "$(dirname "$0")/../../../.." && pwd)"
ROUTE=""
OUT=""
INPUT=""
TIMEOUT_SECS=0
EXTRA_ARGS=()

while [ $# -gt 0 ]; do
  case "$1" in
    --route)
      ROUTE="${2:-}"
      shift 2
      ;;
    --out)
      OUT="${2:-}"
      shift 2
      ;;
    --input)
      INPUT="${2:-}"
      shift 2
      ;;
    --timeout-secs)
      TIMEOUT_SECS="${2:-}"
      shift 2
      ;;
    --)
      shift
      EXTRA_ARGS=("$@")
      break
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "[emit-route] unknown arg: $1" >&2
      usage
      exit 2
      ;;
  esac
done

if [ -z "$ROUTE" ] || [ -z "$OUT" ] || [ -z "$INPUT" ]; then
  usage >&2
  exit 2
fi

if ! [[ "$TIMEOUT_SECS" =~ ^[0-9]+$ ]]; then
  echo "[emit-route] --timeout-secs must be a non-negative integer: $TIMEOUT_SECS" >&2
  exit 2
fi

if [ ! -f "$INPUT" ]; then
  echo "[emit-route] input not found: $INPUT" >&2
  exit 2
fi

if [ -z "${NYASH_BIN:-}" ]; then
  if [ -x "$ROOT/target/release/hakorune" ]; then
    NYASH_BIN="$ROOT/target/release/hakorune"
  else
    NYASH_BIN="$ROOT/target/release/nyash"
  fi
fi

if [ ! -x "$NYASH_BIN" ]; then
  echo "[emit-route] nyash/hakorune binary not found: $NYASH_BIN" >&2
  exit 2
fi

HELPER="$ROOT/tools/hakorune_emit_mir.sh"
if [ "$ROUTE" != "direct" ] && [ ! -f "$HELPER" ]; then
  echo "[emit-route] helper missing: $HELPER" >&2
  exit 2
fi

CMD=()
case "$ROUTE" in
  direct)
    CMD=(env \
      HAKO_STAGE1_MODE=emit-mir \
      HAKO_EMIT_MIR_JSON=1 \
      STAGE1_EMIT_MIR_JSON=1 \
      "$NYASH_BIN" --emit-mir-json "$OUT" "$INPUT" "${EXTRA_ARGS[@]}")
    ;;
  hako-mainline)
    CMD=(env \
      HAKO_SELFHOST_BUILDER_FIRST=1 \
      HAKO_SELFHOST_NO_DELEGATE=1 \
      HAKO_EMIT_MIR_MAINLINE_ONLY=1 \
      bash "$HELPER" "$INPUT" "$OUT" "${EXTRA_ARGS[@]}")
    ;;
  hako-helper)
    CMD=(bash "$HELPER" "$INPUT" "$OUT" "${EXTRA_ARGS[@]}")
    ;;
  *)
    echo "[emit-route] unknown route: $ROUTE" >&2
    echo "[emit-route] allowed: direct | hako-mainline | hako-helper" >&2
    exit 2
    ;;
esac

if [ "$TIMEOUT_SECS" -gt 0 ]; then
  timeout --preserve-status "${TIMEOUT_SECS}s" "${CMD[@]}"
else
  "${CMD[@]}"
fi
