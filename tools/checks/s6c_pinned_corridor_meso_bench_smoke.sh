#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="s6c-pinned-corridor-meso-bench"
TEMP_DIR="$(mktemp -d /tmp/hako-s6c-meso-bench.XXXXXX)"
trap 'rm -rf -- "$TEMP_DIR"' EXIT
CC_CMD="$(printenv CC 2>/dev/null || true)"; [[ -n "$CC_CMD" ]] || CC_CMD=cc
CLANG_CMD="$(printenv CLANG_18 2>/dev/null || true)"; [[ -n "$CLANG_CMD" ]] || CLANG_CMD=clang-18
PROJECTOR="$ROOT_DIR/tools/perf/s6c_pinned_corridor_meso_outline.py"
VALIDATOR="$ROOT_DIR/tools/perf/s6c_pinned_corridor_meso_bench.py"
STRUCTURAL_DRIVER="$ROOT_DIR/lang/c-abi/tests/s6c_pinned_corridor_structural_zero_driver.c"
OBJECT_DRIVER="$ROOT_DIR/lang/c-abi/tests/s6c_pinned_corridor_meso_object_driver.c"
BENCH="$ROOT_DIR/lang/c-abi/tests/s6c_pinned_corridor_meso_bench.c"
REFERENCE="$ROOT_DIR/lang/c-abi/tests/s6c_pinned_corridor_meso_reference.c"
EXPECTED_SCAN="91b4bcc1aba8f08be7a31a046731f62636029133498751bf41795e97a1d371a3"
compile_driver() {
  "$CC_CMD" -I"$ROOT_DIR/plugins/nyash-json-plugin/c/yyjson" -o "$TEMP_DIR/$1" "$2" \
    "$ROOT_DIR/lang/c-abi/shims/hako_aot.c" "$ROOT_DIR/lang/c-abi/shims/hako_json_v1.c" \
    "$ROOT_DIR/plugins/nyash-json-plugin/c/yyjson/yyjson.c" -ldl
}
compile_driver structural-driver "$STRUCTURAL_DRIVER"
compile_driver object-driver "$OBJECT_DRIVER"
HAKO_PINNED_TEXT_REAL_CANDIDATE_JSON_OUT="$TEMP_DIR/real.json" CARGO_BUILD_JOBS=4 \
  cargo test --manifest-path "$ROOT_DIR/Cargo.toml" --profile quick --lib -q \
    mir::builder::resolved_lowering::common_v2_s6c_cursor_cfg_tests::pinned_text_real_candidate_json_preserves_carrier_lineage \
    -- --exact >"$TEMP_DIR/cargo.stdout" 2>"$TEMP_DIR/cargo.stderr"
"$TEMP_DIR/structural-driver" "$TEMP_DIR/real.json" "$TEMP_DIR/structural.o" "$TEMP_DIR/final.ll"
python3 "$PROJECTOR" --ir "$TEMP_DIR/final.ll" --expected-scan-sha256 "$EXPECTED_SCAN" \
  --output "$TEMP_DIR/meso.ll" --manifest "$TEMP_DIR/outline.json"
"$TEMP_DIR/object-driver" "$TEMP_DIR/real.json" "$TEMP_DIR/real.o" "$TEMP_DIR/meso.ll" "$TEMP_DIR/meso.o"
CARGO_BUILD_JOBS=4 cargo build --manifest-path "$ROOT_DIR/Cargo.toml" --profile quick \
  -p nyash_kernel --features promotion-test-support -q >"$TEMP_DIR/build.stdout" 2>"$TEMP_DIR/build.stderr"
"$CLANG_CMD" -O3 -fno-lto -c "$REFERENCE" -o "$TEMP_DIR/reference.o"
"$CLANG_CMD" -O3 -fno-lto -no-pie "$BENCH" "$TEMP_DIR/reference.o" "$TEMP_DIR/meso.o" \
  -L"$ROOT_DIR/target/quick" -lnyash_kernel -lpthread -ldl -lm -o "$TEMP_DIR/meso-bench"
CPU_ID="$(python3 - <<'PY'
allowed = open('/proc/self/status').read().split('Cpus_allowed_list:\t', 1)[1].splitlines()[0]
print(allowed.split(',', 1)[0].split('-', 1)[0])
PY
)"
TOOLCHAIN="$($CLANG_CMD --version | head -1)"
taskset -c "$CPU_ID" "$TEMP_DIR/meso-bench" >"$TEMP_DIR/meso.csv"
python3 "$VALIDATOR" --csv "$TEMP_DIR/meso.csv" --outline-manifest "$TEMP_DIR/outline.json" \
  --binary "$TEMP_DIR/meso-bench" --report "$TEMP_DIR/evidence.json" \
  --commit "$(git -C "$ROOT_DIR" rev-parse HEAD)" --cpu "$CPU_ID" --toolchain "$TOOLCHAIN"
expect_reject() {
  local name="$1" csv="$2" manifest="$3" report="$TEMP_DIR/$name.json"
  if python3 "$VALIDATOR" --csv "$csv" --outline-manifest "$manifest" \
      --binary "$TEMP_DIR/meso-bench" --report "$report" --commit negative \
      --cpu "$CPU_ID" --toolchain "$TOOLCHAIN" >"$TEMP_DIR/$name.stdout" 2>"$TEMP_DIR/$name.stderr"; then
    echo "[$TAG] ERROR: $name negative was accepted" >&2; exit 1
  fi
  [[ ! -e "$report" && ! -e "$report.tmp" ]] || {
    echo "[$TAG] ERROR: $name published negative evidence" >&2; exit 1;
  }
}
TEMP_DIR="$TEMP_DIR" python3 - <<'PY'
import csv, json, os, pathlib
root = pathlib.Path(os.environ['TEMP_DIR'])
rows = list(csv.DictReader((root / 'meso.csv').open())); fields = rows[0].keys()
def write(name, changed):
    with (root / name).open('w', newline='') as f:
        out = csv.DictWriter(f, fieldnames=fields); out.writeheader(); out.writerows(changed)
write('missing-case.csv', [r for r in rows if not (r['family'] == 'mixed' and r['size'] == '1048576' and r['position'] == 'miss')])
short = [dict(r) for r in rows]; short[0]['hako_ns'] = '1'; write('short-arm.csv', short)
bad_shape = [dict(r) for r in rows]; bad_shape[0]['width1'] = '0'; write('shape-drift.csv', bad_shape)
red = [dict(r) for r in rows]
for r in red:
    if r['size'] == '4096' and r['family'] == 'ascii' and r['position'] == 'miss':
        r['hako_ns'] = str(int(r['c_ns']) * 2)
write('threshold-red.csv', red)
manifest = json.loads((root / 'outline.json').read_text()); manifest['schema'] = 'foreign-outline'
(root / 'foreign-outline.json').write_text(json.dumps(manifest))
PY
expect_reject missing-case "$TEMP_DIR/missing-case.csv" "$TEMP_DIR/outline.json"
expect_reject short-arm "$TEMP_DIR/short-arm.csv" "$TEMP_DIR/outline.json"
expect_reject shape-drift "$TEMP_DIR/shape-drift.csv" "$TEMP_DIR/outline.json"
expect_reject threshold-red "$TEMP_DIR/threshold-red.csv" "$TEMP_DIR/outline.json"
expect_reject foreign-outline "$TEMP_DIR/meso.csv" "$TEMP_DIR/foreign-outline.json"
python3 - "$TEMP_DIR/evidence.json" <<'PY'
import json, sys
summary = json.load(open(sys.argv[1]))['summary']
print('[s6c-pinned-corridor-meso-bench] ok '
      f"(4KiB+ max p50={summary['gated_4k_plus_max_p50']:.3f}; "
      f"informational max p95={summary['informational_all_max_p95']:.3f})")
PY
