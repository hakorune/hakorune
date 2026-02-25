#!/usr/bin/env bash
set -euo pipefail

# report_mir_hotops.sh — emit MIR(JSON) then print operation histogram for one function
#
# Usage:
#   tools/perf/report_mir_hotops.sh <bench-key|path/to/program.hako> [--function <name>] [--top <N>]
#
# Notes:
# - If first arg is not a file, it is treated as benchmark key:
#   <key> -> benchmarks/bench_<key>.hako
# - Uses binary-only direct emit by default for stable/fast route:
#   NYASH_STAGE1_BINARY_ONLY_DIRECT=1

ROOT_DIR=$(cd "$(dirname "$0")/../.." && pwd)
HAKORUNE_BIN="${ROOT_DIR}/target/release/hakorune"

if [[ $# -lt 1 ]]; then
  echo "Usage: $0 <bench-key|path/to/program.hako> [--function <name>] [--top <N>]" >&2
  exit 2
fi

INPUT_RAW="$1"
shift || true

FUNC_NAME=""
TOP_N="${PERF_MIR_SHAPE_TOP:-12}"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --function)
      FUNC_NAME="${2:-}"
      shift 2
      ;;
    --top)
      TOP_N="${2:-}"
      shift 2
      ;;
    -h|--help)
      echo "Usage: $0 <bench-key|path/to/program.hako> [--function <name>] [--top <N>]"
      exit 0
      ;;
    *)
      echo "[error] Unknown arg: $1" >&2
      exit 2
      ;;
  esac
done

if ! [[ "${TOP_N}" =~ ^[0-9]+$ ]]; then
  echo "[error] --top must be numeric: ${TOP_N}" >&2
  exit 2
fi

if [[ -f "${INPUT_RAW}" ]]; then
  INPUT_PATH="${INPUT_RAW}"
  INPUT_LABEL="${INPUT_RAW}"
else
  INPUT_PATH="${ROOT_DIR}/benchmarks/bench_${INPUT_RAW}.hako"
  INPUT_LABEL="${INPUT_RAW}"
fi

if [[ ! -f "${INPUT_PATH}" ]]; then
  echo "[error] input .hako not found: ${INPUT_PATH}" >&2
  exit 2
fi

if [[ ! -x "${HAKORUNE_BIN}" ]]; then
  echo "[error] hakorune not built: ${HAKORUNE_BIN}" >&2
  echo "[hint] Run: cargo build --release --bin hakorune" >&2
  exit 2
fi

TIMEOUT="${PERF_MIR_SHAPE_TIMEOUT:-40s}"
TMP_MIR=$(mktemp /tmp/perf_mir_hotops.XXXXXX.json)
cleanup() {
  rm -f "${TMP_MIR}" >/dev/null 2>&1 || true
}
trap cleanup EXIT

set +e
EMIT_OUT=$(
  env NYASH_STAGE1_BINARY_ONLY_DIRECT="${NYASH_STAGE1_BINARY_ONLY_DIRECT:-1}" \
      NYASH_SKIP_TOML_ENV="${NYASH_SKIP_TOML_ENV:-1}" \
      NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
      timeout "${TIMEOUT}" \
      "${HAKORUNE_BIN}" --hako-emit-mir-json "${TMP_MIR}" "${INPUT_PATH}" 2>&1
)
EMIT_RC=$?
set -e

if [[ "${EMIT_RC}" -ne 0 ]]; then
  echo "[error] emit mir failed: rc=${EMIT_RC} input=${INPUT_PATH}" >&2
  if [[ "${EMIT_RC}" -eq 124 || "${EMIT_OUT}" == *"timed out"* ]]; then
    echo "[hint] stage1 emit timeout. Try PERF_MIR_SHAPE_TIMEOUT=120s" >&2
    echo "[hint] current timeout=${TIMEOUT}" >&2
  fi
  printf '%s\n' "${EMIT_OUT}" | sed -n '1,20p' >&2
  exit 1
fi

python3 - "${TMP_MIR}" "${INPUT_LABEL}" "${FUNC_NAME}" "${TOP_N}" <<'PY'
import json
import sys
from collections import Counter

mir_path = sys.argv[1]
input_label = sys.argv[2]
wanted_fn = sys.argv[3]
top_n = int(sys.argv[4])

with open(mir_path, "r", encoding="utf-8") as f:
    mod = json.load(f)

funcs = mod.get("functions") or []
if not funcs:
    print("[error] no functions in MIR JSON", file=sys.stderr)
    sys.exit(1)

target = None
if wanted_fn:
    for fn in funcs:
        if fn.get("name") == wanted_fn:
            target = fn
            break
    if target is None:
        print(f"[error] function not found: {wanted_fn}", file=sys.stderr)
        sys.exit(1)
else:
    for cand in ("main", "Main.main/0", "Main.main"):
        for fn in funcs:
            if fn.get("name") == cand:
                target = fn
                break
        if target is not None:
            break
    if target is None:
        target = funcs[0]

blocks = target.get("blocks") or []
ops = Counter()
inst_total = 0

for bb in blocks:
    for inst in bb.get("instructions") or []:
        op = inst.get("op", "<unknown>")
        ops[op] += 1
        inst_total += 1

print(
    f"[mir-shape] input={input_label} function={target.get('name')} "
    f"blocks={len(blocks)} inst_total={inst_total} unique_ops={len(ops)}"
)

ranked = sorted(ops.items(), key=lambda x: (-x[1], x[0]))
for idx, (op, cnt) in enumerate(ranked[:top_n], start=1):
    pct = (cnt * 100.0 / inst_total) if inst_total else 0.0
    print(f"[mir-shape/op] rank={idx} op={op} count={cnt} pct={pct:.1f}")
PY
