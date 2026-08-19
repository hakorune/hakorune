#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="s6c-pinned-corridor-meso-outline-smoke"
TEMP_DIR="$(mktemp -d /tmp/hako-s6c-meso-outline.XXXXXX)"
trap 'rm -rf -- "$TEMP_DIR"' EXIT

CC_CMD="${CC:-cc}"
CLANG_CMD="${CLANG_18:-clang-18}"
PROJECTOR="$ROOT_DIR/tools/perf/s6c_pinned_corridor_meso_outline.py"
DRIVER="$ROOT_DIR/lang/c-abi/tests/s6c_pinned_corridor_structural_zero_driver.c"
EXPECTED_SCAN="91b4bcc1aba8f08be7a31a046731f62636029133498751bf41795e97a1d371a3"

"$CC_CMD" -I"$ROOT_DIR/plugins/nyash-json-plugin/c/yyjson" \
  -o "$TEMP_DIR/structural-driver" "$DRIVER" \
  "$ROOT_DIR/lang/c-abi/shims/hako_aot.c" \
  "$ROOT_DIR/lang/c-abi/shims/hako_json_v1.c" \
  "$ROOT_DIR/plugins/nyash-json-plugin/c/yyjson/yyjson.c" -ldl

HAKO_PINNED_TEXT_REAL_CANDIDATE_JSON_OUT="$TEMP_DIR/real.json" \
  CARGO_BUILD_JOBS=4 cargo test --manifest-path "$ROOT_DIR/Cargo.toml" \
    --profile quick --lib -q \
    mir::builder::resolved_lowering::common_v2_s6c_cursor_cfg_tests::pinned_text_real_candidate_json_preserves_carrier_lineage \
    -- --exact >"$TEMP_DIR/cargo.stdout" 2>"$TEMP_DIR/cargo.stderr"
"$TEMP_DIR/structural-driver" "$TEMP_DIR/real.json" \
  "$TEMP_DIR/real.o" "$TEMP_DIR/final.ll"

python3 "$PROJECTOR" --ir "$TEMP_DIR/final.ll" \
  --expected-scan-sha256 "$EXPECTED_SCAN" \
  --output "$TEMP_DIR/meso.ll" --manifest "$TEMP_DIR/meso.json"
"$CLANG_CMD" -Werror -c -x ir "$TEMP_DIR/meso.ll" -o "$TEMP_DIR/meso.o"
if [[ "$(nm "$TEMP_DIR/meso.o" | awk '$2 == "T" {print $3}')" != "hako_s6c_meso" ]]; then
  echo "[$TAG] ERROR: outlined object symbol drift" >&2
  exit 1
fi

expect_reject() {
  local name="$1"
  local input="$TEMP_DIR/$name.ll"
  local output="$TEMP_DIR/$name.out.ll"
  local manifest="$TEMP_DIR/$name.json"
  if python3 "$PROJECTOR" --ir "$input" --expected-scan-sha256 "$EXPECTED_SCAN" \
      --output "$output" --manifest "$manifest" \
      >"$TEMP_DIR/$name.stdout" 2>"$TEMP_DIR/$name.stderr"; then
    echo "[$TAG] ERROR: $name drift was accepted" >&2
    exit 1
  fi
  if [[ -e "$output" || -e "$output.tmp" || -e "$manifest" || -e "$manifest.tmp" ]]; then
    echo "[$TAG] ERROR: $name published partial outline evidence" >&2
    exit 1
  fi
}

TEMP_DIR="$TEMP_DIR" python3 - <<'PY'
import os, pathlib
root = pathlib.Path(os.environ['TEMP_DIR'])
text = (root / 'final.ll').read_text()
mutations = {
    'entry-extra': text.replace('  %ptfc_pairs = alloca', '  %unexpected = add i64 %r0, 0\n  %ptfc_pairs = alloca', 1),
    'root-offset': text.replace('ptr %ptfc_frame, i64 32', 'ptr %ptfc_frame, i64 31', 1),
    'lane-dependency': text.replace('%r6 = add i64 %ptfc_subject_len, 0', '%r6 = add i64 %r0, 0', 1),
    'removed-phi': text.replace('[ 0, %bb1 ]', '[ 0, %bb0 ]', 1),
    'scan-drift': text.replace('%r14 = add i64 %r13, 1', '%r14 = add i64 %r13, 2', 1),
    'finish-order': text.replace(
        'call void @hako_text_formal_residence_finish_or_abort_v1(ptr %ptfc_frame)\n  ret i64 %r11',
        'ret i64 %r11\n  call void @hako_text_formal_residence_finish_or_abort_v1(ptr %ptfc_frame)', 1),
}
for name, value in mutations.items():
    (root / f'{name}.ll').write_text(value)
PY

for negative in entry-extra root-offset lane-dependency removed-phi scan-drift finish-order; do
  expect_reject "$negative"
done

python3 - "$TEMP_DIR/meso.json" <<'PY'
import json, sys
data = json.load(open(sys.argv[1]))
assert data['removed'] == {'blocks': 2, 'instructions': 25}
assert data['retained'] == {'blocks': 21, 'edges': 31, 'instructions': 59, 'phis': 6, 'returns': 2}
print('[s6c-pinned-corridor-meso-outline-smoke] ok '
      f"(scan graph {data['retained_graph_sha256'][:12]}; evidence-only)")
PY
