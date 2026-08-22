#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="s6c-native-hwcounter-smoke"
TEMP_DIR="$(mktemp -d /tmp/hako-s6c-native-counter.XXXXXX)"
trap 'rm -rf -- "$TEMP_DIR"' EXIT
CLANG_CMD="${CLANG_18:-clang-18}"
CPU_ID="${S6C_COUNTER_CPU:-$(python3 - <<'PY'
print(min(__import__('os').sched_getaffinity(0)))
PY
)}"
ITERATIONS="${S6C_COUNTER_ITERATIONS:-16777216}"
BUILDER="$ROOT_DIR/tools/perf/build_s6c_native_hwcounter_fixture.sh"
COLLECTOR="$ROOT_DIR/tools/perf/s6c_native_hwcounter_collect.py"

"$BUILDER" "$TEMP_DIR/build"
BIN="$TEMP_DIR/build/meso-bench"
ALIGNMENT="$TEMP_DIR/build/alignment.json"
if taskset -c "$CPU_ID" "$BIN" --arm wrong --case mixed/4096/first --iterations 1 \
    >"$TEMP_DIR/wrong-arm.out" 2>"$TEMP_DIR/wrong-arm.err"; then
  echo "[$TAG] ERROR: wrong arm accepted" >&2; exit 1
fi
if taskset -c "$CPU_ID" "$BIN" --arm hako --case mixed/4096/miss --iterations 1 \
    >"$TEMP_DIR/wrong-case.out" 2>"$TEMP_DIR/wrong-case.err"; then
  echo "[$TAG] ERROR: wrong case accepted" >&2; exit 1
fi
python3 "$COLLECTOR" --probe --binary "$BIN" --alignment-manifest "$ALIGNMENT" \
  --cpu "$CPU_ID" --iterations "$ITERATIONS" --clang "$CLANG_CMD" >"$TEMP_DIR/probe.json"
python3 - "$TEMP_DIR/probe.json" <<'PY'
import json, sys
row = json.load(open(sys.argv[1]))
hako, c = row['hako']['sample'], row['c']['sample']
assert hako['arm'] == 'hako' and c['arm'] == 'c'
assert hako['input_fingerprint'] == c['input_fingerprint']
assert hako['result'] == c['result']
assert 'arm_envelope' in hako and 'arm_envelope' in c
PY
echo "[$TAG] ok (positive native pair plus contract negatives)"
