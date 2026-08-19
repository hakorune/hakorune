#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="s6c-pinned-corridor-meso-outline-parity"
TEMP_DIR="$(mktemp -d /tmp/hako-s6c-meso-parity.XXXXXX)"
trap 'rm -rf -- "$TEMP_DIR"' EXIT

CC_CMD="${CC:-cc}"
PROJECTOR="$ROOT_DIR/tools/perf/s6c_pinned_corridor_meso_outline.py"
STRUCTURAL_DRIVER="$ROOT_DIR/lang/c-abi/tests/s6c_pinned_corridor_structural_zero_driver.c"
OBJECT_DRIVER="$ROOT_DIR/lang/c-abi/tests/s6c_pinned_corridor_meso_object_driver.c"
RUNNER="$ROOT_DIR/lang/c-abi/tests/s6c_pinned_corridor_meso_outline_parity.c"
EXPECTED_SCAN="91b4bcc1aba8f08be7a31a046731f62636029133498751bf41795e97a1d371a3"

compile_driver() {
  "$CC_CMD" -I"$ROOT_DIR/plugins/nyash-json-plugin/c/yyjson" \
    -o "$TEMP_DIR/$1" "$2" \
    "$ROOT_DIR/lang/c-abi/shims/hako_aot.c" \
    "$ROOT_DIR/lang/c-abi/shims/hako_json_v1.c" \
    "$ROOT_DIR/plugins/nyash-json-plugin/c/yyjson/yyjson.c" -ldl
}
compile_driver structural-driver "$STRUCTURAL_DRIVER"
compile_driver object-driver "$OBJECT_DRIVER"

HAKO_PINNED_TEXT_REAL_CANDIDATE_JSON_OUT="$TEMP_DIR/real.json" \
  CARGO_BUILD_JOBS=4 cargo test --manifest-path "$ROOT_DIR/Cargo.toml" \
    --profile quick --lib -q \
    mir::builder::resolved_lowering::common_v2_s6c_cursor_cfg_tests::pinned_text_real_candidate_json_preserves_carrier_lineage \
    -- --exact >"$TEMP_DIR/cargo.stdout" 2>"$TEMP_DIR/cargo.stderr"
"$TEMP_DIR/structural-driver" "$TEMP_DIR/real.json" \
  "$TEMP_DIR/structural-real.o" "$TEMP_DIR/final.ll"
python3 "$PROJECTOR" --ir "$TEMP_DIR/final.ll" \
  --expected-scan-sha256 "$EXPECTED_SCAN" \
  --output "$TEMP_DIR/meso.ll" --manifest "$TEMP_DIR/meso.json"
"$TEMP_DIR/object-driver" "$TEMP_DIR/real.json" "$TEMP_DIR/real.o" \
  "$TEMP_DIR/meso.ll" "$TEMP_DIR/meso.o"
objcopy --redefine-sym 'ny_main=hako_s6c_candidate' \
  "$TEMP_DIR/real.o" "$TEMP_DIR/candidate.o"

CARGO_BUILD_JOBS=4 cargo build --manifest-path "$ROOT_DIR/Cargo.toml" \
  --profile quick -p nyash_kernel --features promotion-test-support -q \
  >"$TEMP_DIR/build.stdout" 2>"$TEMP_DIR/build.stderr"
"$CC_CMD" -O2 -no-pie "$RUNNER" "$TEMP_DIR/candidate.o" "$TEMP_DIR/meso.o" \
  -L"$ROOT_DIR/target/quick" -lnyash_kernel -lpthread -ldl -lm \
  -o "$TEMP_DIR/parity"
"$TEMP_DIR/parity"

sed '0,/ret i64 %r11/s//ret i64 999/' "$TEMP_DIR/meso.ll" >"$TEMP_DIR/wrong.ll"
"$TEMP_DIR/object-driver" "$TEMP_DIR/real.json" "$TEMP_DIR/wrong-real.o" \
  "$TEMP_DIR/wrong.ll" "$TEMP_DIR/wrong.o"
objcopy --redefine-sym 'ny_main=hako_s6c_candidate' \
  "$TEMP_DIR/wrong-real.o" "$TEMP_DIR/wrong-candidate.o"
"$CC_CMD" -O2 -no-pie "$RUNNER" "$TEMP_DIR/wrong-candidate.o" "$TEMP_DIR/wrong.o" \
  -L"$ROOT_DIR/target/quick" -lnyash_kernel -lpthread -ldl -lm \
  -o "$TEMP_DIR/wrong-parity"
if "$TEMP_DIR/wrong-parity" >"$TEMP_DIR/wrong.stdout" 2>"$TEMP_DIR/wrong.stderr"; then
  echo "[$TAG] ERROR: wrong outlined Return was accepted" >&2
  exit 1
fi
if find "$TEMP_DIR" -name '*.ptfb-tm-*.tmp' -print -quit | grep -q .; then
  echo "[$TAG] ERROR: TargetMachine temporary survived" >&2
  exit 1
fi
echo "[$TAG] ok (unchanged whole candidate == digest-verified outline on shared roots)"
