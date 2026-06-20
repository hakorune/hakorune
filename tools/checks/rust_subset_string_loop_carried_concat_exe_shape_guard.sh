#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

PROBE="apps/rust-subset-to-hako/probes/regression/string_loop_carried_converter_concat_probe.hako"
TMP_BASE="/tmp/rust_subset_string_loop_carried_converter_concat"
MIR_JSON="${TMP_BASE}.mir.json"
EXE="${TMP_BASE}"
RAW_OUT="${TMP_BASE}.out.raw"
OUT="${TMP_BASE}.out"
EXPECTED="${TMP_BASE}.expected"

bash tools/build_hako_llvmc_ffi.sh >/dev/null

NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-mir-json "$MIR_JSON" "$PROBE" \
  >"${TMP_BASE}.emit.log" 2>&1

rm -f "$EXE"
NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-exe "$EXE" "$PROBE" \
  >"${TMP_BASE}.exe.log" 2>&1

"$EXE" >"$RAW_OUT" 2>"${TMP_BASE}.err"
sed '/^Result: /d' "$RAW_OUT" >"$OUT"

{
  echo "// module apps/rust-subset-to-hako/examples/simple_subset.json"
  cat apps/rust-subset-to-hako/examples/simple_expected.hako
  echo
  echo "// module apps/rust-subset-to-hako/examples/edge_subset.json"
  cat apps/rust-subset-to-hako/examples/edge_expected.hako
} >"$EXPECTED"

diff -u "$EXPECTED" "$OUT"

cat <<'REPORT'
output_contract=rust-subset-string-loop-carried-concat-exe-shape-v0
loop_carried_string_concat_mir_emit=green
loop_carried_string_concat_exe=green
output_matches_expected=green
summary=ok
REPORT
