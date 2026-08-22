#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="s6c-pinned-corridor-exact-bench-smoke"
TEMP_DIR="$(mktemp -d /tmp/hako-s6c-exact-bench.XXXXXX)"
trap 'rm -rf -- "$TEMP_DIR"' EXIT

CC_CMD="${CC:-cc}"
CLANG_CMD="${CLANG_18:-clang-18}"
CHECKER="$ROOT_DIR/tools/perf/s6c_pinned_corridor_exact_bench.py"
DRIVER="$ROOT_DIR/lang/c-abi/tests/s6c_pinned_corridor_exact_leaf_driver.c"
BENCH="$ROOT_DIR/lang/c-abi/tests/s6c_pinned_corridor_exact_leaf_bench.c"
REFERENCE="$ROOT_DIR/lang/c-abi/tests/s6c_pinned_corridor_exact_leaf_reference.c"

for command in "$CC_CMD" "$CLANG_CMD" taskset python3; do
  command -v "$command" >/dev/null || { echo "[$TAG] ERROR: missing $command" >&2; exit 1; }
done

"$CC_CMD" \
  -I"$ROOT_DIR/plugins/nyash-json-plugin/c/yyjson" \
  -o "$TEMP_DIR/exact-driver" "$DRIVER" \
  "$ROOT_DIR/lang/c-abi/shims/hako_aot.c" \
  "$ROOT_DIR/lang/c-abi/shims/hako_json_v1.c" \
  "$ROOT_DIR/plugins/nyash-json-plugin/c/yyjson/yyjson.c" -ldl

HAKO_PINNED_TEXT_REAL_CANDIDATE_JSON_OUT="$TEMP_DIR/real.json" \
  CARGO_BUILD_JOBS=4 \
  cargo test --manifest-path "$ROOT_DIR/Cargo.toml" --profile quick --lib -q \
    mir::builder::resolved_lowering::common_v2_s6c_cursor_cfg_tests::pinned_text_real_candidate_json_preserves_carrier_lineage \
    -- --exact >"$TEMP_DIR/cargo-test.stdout" 2>"$TEMP_DIR/cargo-test.stderr"

if "$TEMP_DIR/exact-driver" "$TEMP_DIR/real.json" "$TEMP_DIR/bad-real.o" \
    "$TEMP_DIR/missing/leaf.o" "$TEMP_DIR/bad-leaf.ll" \
    >"$TEMP_DIR/bad.stdout" 2>"$TEMP_DIR/bad.stderr"; then
  echo "[$TAG] ERROR: unavailable leaf output was accepted" >&2
  exit 1
fi
if [[ -e "$TEMP_DIR/bad-real.o" || -e "$TEMP_DIR/bad-leaf.ll" ]]; then
  echo "[$TAG] ERROR: failed leaf projection published evidence" >&2
  exit 1
fi

"$TEMP_DIR/exact-driver" "$TEMP_DIR/real.json" "$TEMP_DIR/real.o" \
  "$TEMP_DIR/exact-leaf.o" "$TEMP_DIR/exact-leaf.ll"

CARGO_BUILD_JOBS=4 cargo build --manifest-path "$ROOT_DIR/Cargo.toml" \
  --profile quick -p nyash_kernel --features promotion-test-support -q \
  >"$TEMP_DIR/cargo-build.stdout" 2>"$TEMP_DIR/cargo-build.stderr"

"$CLANG_CMD" -O3 -fno-lto -c "$REFERENCE" -o "$TEMP_DIR/reference.o"
"$CLANG_CMD" -O3 -fno-lto -no-pie "$BENCH" "$TEMP_DIR/reference.o" \
  "$TEMP_DIR/exact-leaf.o" -L"$ROOT_DIR/target/quick" -lnyash_kernel \
  -lpthread -ldl -lm -o "$TEMP_DIR/exact-bench"

CPU_ID="$(python3 - <<'PY'
allowed = open('/proc/self/status').read().split('Cpus_allowed_list:\t', 1)[1].splitlines()[0]
head = allowed.split(',', 1)[0]
print(head.split('-', 1)[0])
PY
)"
TOOLCHAIN="$($CLANG_CMD --version | head -1)"
taskset -c "$CPU_ID" "$TEMP_DIR/exact-bench" >"$TEMP_DIR/exact.csv"
python3 "$CHECKER" --csv "$TEMP_DIR/exact.csv" --ir "$TEMP_DIR/exact-leaf.ll" \
  --binary "$TEMP_DIR/exact-bench" --report "$TEMP_DIR/evidence.json" \
  --commit "$(git -C "$ROOT_DIR" rev-parse HEAD)" --cpu "$CPU_ID" --toolchain "$TOOLCHAIN"

expect_reject() {
  local name="$1"
  local csv="$2"
  local ir="$3"
  local report="$TEMP_DIR/$name.json"
  if python3 "$CHECKER" --csv "$csv" --ir "$ir" --binary "$TEMP_DIR/exact-bench" \
      --report "$report" --commit negative --cpu "$CPU_ID" --toolchain "$TOOLCHAIN" \
      >"$TEMP_DIR/$name.stdout" 2>"$TEMP_DIR/$name.stderr"; then
    echo "[$TAG] ERROR: $name negative was accepted" >&2
    exit 1
  fi
  [[ ! -e "$report" && ! -e "$report.tmp" ]] || {
    echo "[$TAG] ERROR: $name published negative evidence" >&2; exit 1;
  }
}

TEMP_DIR="$TEMP_DIR" python3 - <<'PY'
import csv, os, pathlib
root = pathlib.Path(os.environ['TEMP_DIR'])
rows = list(csv.DictReader((root / 'exact.csv').open()))
fields = rows[0].keys()
def write(name, changed):
    with (root / name).open('w', newline='') as f:
        out = csv.DictWriter(f, fieldnames=fields); out.writeheader(); out.writerows(changed)
write('missing-case.csv', [r for r in rows if r['case'] != 'w4-alias'])
short = [dict(r) for r in rows]; short[0]['hako_ns'] = '1'; write('short-sample.csv', short)
zero = [dict(r) for r in rows]; zero[0]['c_ns'] = '0'; write('zero-denominator.csv', zero)
red = [dict(r) for r in rows]
for r in red:
    if r['case'] == 'w1-equal': r['hako_ns'] = str(int(r['c_ns']) * 2)
write('threshold-red.csv', red)
ir = (root / 'exact-leaf.ll').read_text()
(root / 'eager.ll').write_text(ir.replace('br i1 %ptfc_c0_', 'br i1 %ptfc_c1_', 1))
PY

expect_reject missing-case "$TEMP_DIR/missing-case.csv" "$TEMP_DIR/exact-leaf.ll"
expect_reject short-sample "$TEMP_DIR/short-sample.csv" "$TEMP_DIR/exact-leaf.ll"
expect_reject zero-denominator "$TEMP_DIR/zero-denominator.csv" "$TEMP_DIR/exact-leaf.ll"
expect_reject threshold-red "$TEMP_DIR/threshold-red.csv" "$TEMP_DIR/exact-leaf.ll"
expect_reject eager-load "$TEMP_DIR/exact.csv" "$TEMP_DIR/eager.ll"

python3 - "$TEMP_DIR/evidence.json" <<'PY'
import json, sys
data = json.load(open(sys.argv[1]))
s = data['cases']['summary']
print('[s6c-pinned-corridor-exact-bench-smoke] ok '
      f"(ascii p50={s['ascii_max_p50']:.3f}, mixed p50={s['mixed_max_p50']:.3f}, p95={s['all_max_p95']:.3f})")
PY
