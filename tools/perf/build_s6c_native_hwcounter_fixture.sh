#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
OUTPUT_DIR="${1:-$ROOT_DIR/target/perf_evidence/s6c_native_hwcounter}"
CC_CMD="${CC:-cc}"
CLANG_CMD="${CLANG_18:-clang-18}"
PROJECTOR="$ROOT_DIR/tools/perf/s6c_pinned_corridor_meso_outline.py"
COLLECTOR="$ROOT_DIR/tools/perf/s6c_native_hwcounter_collect.py"
STRUCTURAL_DRIVER="$ROOT_DIR/lang/c-abi/tests/s6c_pinned_corridor_structural_zero_driver.c"
OBJECT_DRIVER="$ROOT_DIR/lang/c-abi/tests/s6c_pinned_corridor_meso_object_driver.c"
BENCH="$ROOT_DIR/lang/c-abi/tests/s6c_pinned_corridor_meso_bench.c"
REFERENCE="$ROOT_DIR/lang/c-abi/tests/s6c_pinned_corridor_meso_reference.c"
EXPECTED_SCAN="ea07b0aa8b57a37c3f18534bb3e29035d7cfbcbc7cea5a4cd6841aa9dad4c8d7"

mkdir -p "$OUTPUT_DIR"
compile_driver() {
  "$CC_CMD" -I"$ROOT_DIR/plugins/nyash-json-plugin/c/yyjson" -o "$OUTPUT_DIR/$1" "$2" \
    "$ROOT_DIR/lang/c-abi/shims/hako_aot.c" "$ROOT_DIR/lang/c-abi/shims/hako_json_v1.c" \
    "$ROOT_DIR/plugins/nyash-json-plugin/c/yyjson/yyjson.c" -ldl
}

compile_driver structural-driver "$STRUCTURAL_DRIVER"
compile_driver object-driver "$OBJECT_DRIVER"
HAKO_PINNED_TEXT_REAL_CANDIDATE_JSON_OUT="$OUTPUT_DIR/real.json" CARGO_BUILD_JOBS=4 \
  cargo test --manifest-path "$ROOT_DIR/Cargo.toml" --profile quick --lib -q \
    mir::builder::resolved_lowering::common_v2_s6c_cursor_cfg_tests::pinned_text_real_candidate_json_preserves_carrier_lineage \
    -- --exact >"$OUTPUT_DIR/cargo-test.stdout" 2>"$OUTPUT_DIR/cargo-test.stderr"
"$OUTPUT_DIR/structural-driver" "$OUTPUT_DIR/real.json" "$OUTPUT_DIR/structural.o" "$OUTPUT_DIR/final.ll"
python3 "$PROJECTOR" --ir "$OUTPUT_DIR/final.ll" --expected-scan-sha256 "$EXPECTED_SCAN" \
  --output "$OUTPUT_DIR/meso.ll" --manifest "$OUTPUT_DIR/outline.json"
"$OUTPUT_DIR/object-driver" "$OUTPUT_DIR/real.json" "$OUTPUT_DIR/real.o" \
  "$OUTPUT_DIR/meso.ll" "$OUTPUT_DIR/meso.o"
CARGO_BUILD_JOBS=4 cargo build --manifest-path "$ROOT_DIR/Cargo.toml" --profile quick \
  -p nyash_kernel --features promotion-test-support -q \
  >"$OUTPUT_DIR/cargo-build.stdout" 2>"$OUTPUT_DIR/cargo-build.stderr"
"$CLANG_CMD" -O3 -fno-lto -c "$REFERENCE" -o "$OUTPUT_DIR/reference.o"
"$CLANG_CMD" -O3 -fno-lto -no-pie "$BENCH" "$OUTPUT_DIR/reference.o" "$OUTPUT_DIR/meso.o" \
  -L"$ROOT_DIR/target/quick" -Wl,-rpath,"$ROOT_DIR/target/quick" \
  -lnyash_kernel -lpthread -ldl -lm -o "$OUTPUT_DIR/meso-bench"
python3 "$COLLECTOR" --binary "$OUTPUT_DIR/meso-bench" \
  --alignment-manifest "$OUTPUT_DIR/alignment.json" --write-alignment-manifest
echo "[s6c-native-hwcounter-build] ok: $OUTPUT_DIR/meso-bench"
