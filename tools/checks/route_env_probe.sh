#!/usr/bin/env bash

# route_env_probe.sh
# Print emit route + environment trace in one place.
# Purpose:
#  - make current emit path explicit (direct/hako-mainline/hako-helper)
#  - expose fallback/feature env before running

set -euo pipefail

usage() {
  cat <<'USAGE'
Usage:
  route_env_probe.sh --route <direct|hako-mainline|hako-helper> [--source <path>] [--run] [--require-no-fallback]

Examples:
  route_env_probe.sh --route direct --source apps/tests/phase29bq_hako_mirbuilder_phase1_literal_return_min.hako
  route_env_probe.sh --route hako-mainline --source apps/tests/phase29bq_hako_mirbuilder_phase1_literal_return_min.hako --run
  route_env_probe.sh --route direct --require-no-fallback
USAGE
}

ROUTE=""
SOURCE=""
RUN_EMIT=0
REQUIRE_NO_FALLBACK=0

while [[ $# -gt 0 ]]; do
  case "$1" in
    --route)
      ROUTE="${2:-}"
      shift 2
      ;;
    --source)
      SOURCE="${2:-}"
      shift 2
      ;;
    --run)
      RUN_EMIT=1
      shift
      ;;
    --require-no-fallback)
      REQUIRE_NO_FALLBACK=1
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "[route_env_probe] unknown arg: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if [[ -z "$ROUTE" ]]; then
  usage >&2
  exit 2
fi

case "$ROUTE" in
  direct|hako-mainline|hako-helper) ;;
  *)
    echo "[route_env_probe] invalid route: $ROUTE" >&2
    echo "[route_env_probe] allowed: direct | hako-mainline | hako-helper" >&2
    exit 2
    ;;
esac

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
NYASH_BIN="${NYASH_BIN:-$ROOT/target/release/hakorune}"
if [[ ! -x "$NYASH_BIN" ]]; then
  FALLBACK_NYASH_BIN="$ROOT/target/release/nyash"
  if [[ -x "$FALLBACK_NYASH_BIN" ]]; then
    NYASH_BIN="$FALLBACK_NYASH_BIN"
  fi
fi

declare -a KEYS=(
  NYASH_BIN
  NYASH_ROOT
  NYASH_VM_USE_FALLBACK
  PERF_AOT_PREFER_HELPER
  PERF_AOT_HELPER_ONLY
  NYASH_FAIL_FAST
  NYASH_MIR_CONCAT3_CANON
  NYASH_STRING_SPAN_CACHE_POLICY
  HAKO_SELFHOST_BUILDER_FIRST
  HAKO_SELFHOST_NO_DELEGATE
  HAKO_EMIT_MIR_MAINLINE_ONLY
  HAKO_MIR_BUILDER_DELEGATE
  NYASH_USE_STAGE1_CLI
  NYASH_STAGE1_MODE
  HAKO_STAGE1_MODE
  STAGE1_EMIT_PROGRAM_JSON
  STAGE1_EMIT_MIR_JSON
  NYASH_STAGE1_BINARY_ONLY_DIRECT
  NYASH_STAGE1_BINARY_ONLY_RUN_DIRECT
  STAGE1_SOURCE
)

is_enabled_env() {
  local raw="${1:-}"
  local lower="${raw,,}"
  case "$lower" in
    ""|0|false|off|no|unset)
      return 1
      ;;
    *)
      return 0
      ;;
  esac
}

require_disabled_env() {
  local key="$1"
  local val="${!key-}"
  if is_enabled_env "$val"; then
    echo "[route_env_probe] [FAIL] require-no-fallback: ${key}=${val} must be unset/0/false/off/no" >&2
    return 1
  fi
  return 0
}

if [[ "$REQUIRE_NO_FALLBACK" -eq 1 ]]; then
  if [[ "$ROUTE" == "hako-helper" ]]; then
    echo "[route_env_probe] [FAIL] require-no-fallback forbids route=hako-helper" >&2
    exit 2
  fi
  require_disabled_env NYASH_VM_USE_FALLBACK
  require_disabled_env HAKO_MIR_BUILDER_DELEGATE
  require_disabled_env PERF_AOT_PREFER_HELPER
  require_disabled_env PERF_AOT_HELPER_ONLY
fi

echo "[route_env_probe] route = $ROUTE"
echo "[route_env_probe] NYASH_BIN = $NYASH_BIN"
if [[ -x "$NYASH_BIN" ]]; then
  echo "[route_env_probe] binary  = available"
else
  echo "[route_env_probe] binary  = unavailable"
fi

for key in "${KEYS[@]}"; do
  value="${!key-<unset>}"
  if [[ "$value" == "<unset>" ]]; then
    printf "[route_env_probe] %-35s <unset>\n" "$key"
  else
    printf "[route_env_probe] %-35s %s\n" "$key" "$value"
  fi
done

echo "[route_env_probe] resolved route command:"
case "$ROUTE" in
  direct)
    echo "  $NYASH_BIN --emit-mir-json <out> <source>"
    ;;
  hako-mainline)
    echo "  bash tools/smokes/v2/lib/emit_mir_route.sh --route hako-mainline --out <out> --input <source>"
    ;;
  hako-helper)
    echo "  bash tools/smokes/v2/lib/emit_mir_route.sh --route hako-helper --out <out> --input <source>"
    ;;
esac

if [[ "$RUN_EMIT" -eq 1 ]]; then
  if [[ -z "$SOURCE" ]]; then
    echo "[route_env_probe] --run requires --source" >&2
    exit 2
  fi

  if [[ "$SOURCE" != /* ]]; then
    SOURCE="$ROOT/$SOURCE"
  fi

  if [[ ! -f "$SOURCE" ]]; then
    echo "[route_env_probe] source not found: $SOURCE" >&2
    exit 2
  fi

  TEMP_OUT="$(mktemp)"
  ROUTE_SCRIPT="$ROOT/tools/smokes/v2/lib/emit_mir_route.sh"
  if [[ ! -x "$ROUTE_SCRIPT" ]]; then
    echo "[route_env_probe] emit route script not found: $ROUTE_SCRIPT" >&2
    exit 2
  fi
  case "$ROUTE" in
    hako-mainline|hako-helper)
      if [[ ! -f "$ROOT/tools/hakorune_emit_mir.sh" ]]; then
        echo "[route_env_probe] helper script missing: $ROOT/tools/hakorune_emit_mir.sh" >&2
        exit 2
      fi
      ;;
  esac

  echo "[route_env_probe] run command:"
  echo "  bash \"$ROUTE_SCRIPT\" --route \"$ROUTE\" --out \"$TEMP_OUT\" --input \"$SOURCE\""
  NYASH_BIN="$NYASH_BIN" bash "$ROUTE_SCRIPT" --route "$ROUTE" --out "$TEMP_OUT" --input "$SOURCE" >/dev/null
  rc=$?
  echo "[route_env_probe] emit exit code: $rc"
  rm -f "$TEMP_OUT"
  exit "$rc"
fi
