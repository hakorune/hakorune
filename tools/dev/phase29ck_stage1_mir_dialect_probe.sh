#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage:
  phase29ck_stage1_mir_dialect_probe.sh --mir-json <file> [--strict-stage1]
  phase29ck_stage1_mir_dialect_probe.sh --route <direct|hako-mainline|hako-helper> --input <src.hako> [--timeout-secs <n>] [--strict-stage1]

Notes:
  - `--route/--input` materializes MIR(JSON) through `tools/smokes/v2/lib/emit_mir_route.sh`.
  - `--strict-stage1` fails if legacy Stage1 callsites (`boxcall` / `externcall`) are observed.
USAGE
}

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
ROUTE=""
INPUT=""
MIR_JSON=""
TIMEOUT_SECS=0
STRICT_STAGE1=0

while [ $# -gt 0 ]; do
  case "$1" in
    --route)
      ROUTE="${2:-}"
      shift 2
      ;;
    --input)
      INPUT="${2:-}"
      shift 2
      ;;
    --mir-json)
      MIR_JSON="${2:-}"
      shift 2
      ;;
    --timeout-secs)
      TIMEOUT_SECS="${2:-}"
      shift 2
      ;;
    --strict-stage1)
      STRICT_STAGE1=1
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "[stage1-mir/probe] unknown arg: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if [[ -n "$MIR_JSON" && -n "$ROUTE" ]]; then
  echo "[stage1-mir/probe] choose either --mir-json or --route/--input" >&2
  exit 2
fi

if [[ -z "$MIR_JSON" && -z "$ROUTE" ]]; then
  usage >&2
  exit 2
fi

if [[ -n "$ROUTE" && -z "$INPUT" ]]; then
  echo "[stage1-mir/probe] --input is required with --route" >&2
  exit 2
fi

if ! [[ "$TIMEOUT_SECS" =~ ^[0-9]+$ ]]; then
  echo "[stage1-mir/probe] --timeout-secs must be a non-negative integer: $TIMEOUT_SECS" >&2
  exit 2
fi

tmp_dir=""
cleanup() {
  if [[ -n "$tmp_dir" && -d "$tmp_dir" ]]; then
    rm -rf "$tmp_dir"
  fi
}
trap cleanup EXIT

SOURCE_LABEL=""
MIR_PATH=""

if [[ -n "$MIR_JSON" ]]; then
  if [[ ! -f "$MIR_JSON" ]]; then
    echo "[stage1-mir/probe] MIR JSON not found: $MIR_JSON" >&2
    exit 2
  fi
  MIR_PATH="$MIR_JSON"
  SOURCE_LABEL="mir-json:$MIR_JSON"
else
  tmp_dir="$(mktemp -d)"
  MIR_PATH="$tmp_dir/probe.mir.json"
  SOURCE_LABEL="route:$ROUTE input:$INPUT"
  EMIT_ROUTE="$ROOT/tools/smokes/v2/lib/emit_mir_route.sh"
  if [[ ! -x "$EMIT_ROUTE" ]]; then
    echo "[stage1-mir/probe] emit route helper missing: $EMIT_ROUTE" >&2
    exit 2
  fi
  cmd=(bash "$EMIT_ROUTE" --route "$ROUTE" --out "$MIR_PATH" --input "$INPUT")
  if [[ "$TIMEOUT_SECS" -gt 0 ]]; then
    cmd+=(--timeout-secs "$TIMEOUT_SECS")
  fi
  "${cmd[@]}"
fi

python3 - "$MIR_PATH" "$SOURCE_LABEL" "$STRICT_STAGE1" <<'PY'
import json
import pathlib
import sys

path = pathlib.Path(sys.argv[1])
source = sys.argv[2]
strict = sys.argv[3] == "1"

root = json.loads(path.read_text(encoding="utf-8"))
functions = root.get("functions", [])
counts = {
    "instructions": 0,
    "functions": len(functions),
    "blocks": 0,
    "boxcall": 0,
    "mir_call": 0,
    "newbox": 0,
    "copy": 0,
    "externcall": 0,
    "call": 0,
    "call_with_callee": 0,
    "call_without_callee": 0,
}

for func in functions:
    for block in func.get("blocks", []):
        counts["blocks"] += 1
        for inst in block.get("instructions", []):
            counts["instructions"] += 1
            op = inst.get("op")
            if op in counts:
                counts[op] += 1
            if op == "call":
                if "callee" in inst and inst["callee"] is not None:
                    counts["call_with_callee"] += 1
                else:
                    counts["call_without_callee"] += 1

print(
    "[stage1-mir/probe] "
    f"source={source} "
    f"functions={counts['functions']} "
    f"blocks={counts['blocks']} "
    f"instructions={counts['instructions']} "
    f"boxcall={counts['boxcall']} "
    f"mir_call={counts['mir_call']} "
    f"call={counts['call']} "
    f"call_with_callee={counts['call_with_callee']} "
    f"call_without_callee={counts['call_without_callee']} "
    f"externcall={counts['externcall']} "
    f"newbox={counts['newbox']} "
    f"copy={counts['copy']}"
)

if strict and (counts["boxcall"] > 0 or counts["externcall"] > 0):
    print(
        "[freeze:contract][stage1_mir_dialect/legacy_callsite_detected] "
        f"source={source} boxcall={counts['boxcall']} externcall={counts['externcall']}",
        file=sys.stderr,
    )
    sys.exit(1)
PY
