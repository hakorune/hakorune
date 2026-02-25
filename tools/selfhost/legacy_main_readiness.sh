#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
IDENTITY_SCRIPT="${ROOT}/tools/selfhost_identity_check.sh"

PRODUCER_PATTERN='method\s+main\(args\)|static method main'
CONSUMER_PATTERN='findLegacyMainBody|tryLegacyPattern'

STRICT=0
RUN_SMOKE=1
CLI_MODE="stage1"
BIN_STAGE1="${ROOT}/target/selfhost/hakorune.stage1_cli"
BIN_STAGE2="${ROOT}/target/selfhost/hakorune.stage1_cli.stage2"

usage() {
  cat <<'USAGE'
Usage: tools/selfhost/legacy_main_readiness.sh [options]

Options:
  --strict                Exit 1 unless readiness is met
  --skip-smoke            Skip identity smoke (inventory-only run)
  --cli-mode <mode>       Pass-through to identity smoke (default: stage1)
  --bin-stage1 <path>     Stage1 binary for identity smoke
  --bin-stage2 <path>     Stage2 binary for identity smoke
  -h, --help              Show this help

Readiness rule:
  - producer inventory count == 0
  - identity smoke PASS
USAGE
}

filter_comment_only_hits() {
  awk '
    {
      text = $0
      sub(/^[^:]*:[0-9]+:/, "", text)
      if (text ~ /^[[:space:]]*\/\/+/) next
      if (text ~ /^[[:space:]]*$/) next
      print $0
    }
  '
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --strict)
      STRICT=1
      shift
      ;;
    --skip-smoke)
      RUN_SMOKE=0
      shift
      ;;
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
      echo "[selfhost/legacy-readiness] unknown arg: $1" >&2
      usage
      exit 2
      ;;
  esac
done

if [[ "$CLI_MODE" != "auto" && "$CLI_MODE" != "stage1" && "$CLI_MODE" != "stage0" ]]; then
  echo "[selfhost/legacy-readiness] invalid --cli-mode: $CLI_MODE" >&2
  exit 2
fi

producer_hits="$(rg -n "${PRODUCER_PATTERN}" "${ROOT}/lang/src/compiler" "${ROOT}/apps/tests" || true)"
producer_hits="$(printf '%s\n' "$producer_hits" | filter_comment_only_hits || true)"
producer_count="$(printf '%s\n' "$producer_hits" | sed '/^$/d' | wc -l | tr -d '[:space:]')"

consumer_hits="$(rg -n "${CONSUMER_PATTERN}" "${ROOT}/lang/src/compiler/entry/compiler_stageb.hako" "${ROOT}/lang/src/compiler/entry/compiler.hako" || true)"
consumer_hits="$(printf '%s\n' "$consumer_hits" | filter_comment_only_hits || true)"
consumer_count="$(printf '%s\n' "$consumer_hits" | sed '/^$/d' | wc -l | tr -d '[:space:]')"

echo "[selfhost/legacy-readiness] producer inventory (count=${producer_count})"
if [[ -n "$producer_hits" ]]; then
  printf '%s\n' "$producer_hits"
else
  echo "(none)"
fi

echo "[selfhost/legacy-readiness] consumer inventory (count=${consumer_count})"
if [[ -n "$consumer_hits" ]]; then
  printf '%s\n' "$consumer_hits"
else
  echo "(none)"
fi

smoke_ok=1
smoke_state="pass"
if [[ "$RUN_SMOKE" -eq 1 ]]; then
  if [[ ! -x "$IDENTITY_SCRIPT" ]]; then
    echo "[selfhost/legacy-readiness] missing identity script: $IDENTITY_SCRIPT" >&2
    exit 2
  fi
  echo "[selfhost/legacy-readiness] running identity smoke"
  identity_args=(
    --mode smoke
    --skip-build
    --cli-mode "$CLI_MODE"
    --bin-stage1 "$BIN_STAGE1"
    --bin-stage2 "$BIN_STAGE2"
  )
  if [[ "$CLI_MODE" != "stage1" ]]; then
    identity_args+=(--allow-compat-route)
  fi
  if ! "$IDENTITY_SCRIPT" "${identity_args[@]}"; then
    smoke_ok=0
    smoke_state="fail"
  fi
else
  smoke_state="skipped"
fi

ready=0
if [[ "$producer_count" -eq 0 && "$smoke_ok" -eq 1 ]]; then
  ready=1
fi

echo "[selfhost/legacy-readiness] summary producer_count=${producer_count} consumer_count=${consumer_count} smoke=${smoke_state} ready=${ready}"

if [[ "$RUN_SMOKE" -eq 1 && "$smoke_ok" -ne 1 ]]; then
  exit 2
fi

if [[ "$STRICT" -eq 1 && "$ready" -ne 1 ]]; then
  exit 1
fi

exit 0
