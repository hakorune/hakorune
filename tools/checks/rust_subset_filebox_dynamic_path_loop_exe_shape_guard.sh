#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT"

PROBE="apps/rust-subset-to-hako/probes/regression/filebox_dynamic_path_loop_probe.hako"
MIR_OUT="${TMPDIR:-/tmp}/filebox_dynamic_path_loop_probe.mir.json"
EXE_OUT="${TMPDIR:-/tmp}/filebox_dynamic_path_loop_probe_exe"
RUN_OUT="${TMPDIR:-/tmp}/filebox_dynamic_path_loop_probe.out"

bash tools/build_hako_llvmc_ffi.sh >/dev/null

./target/release/hakorune --emit-mir-json "$MIR_OUT" "$PROBE" >/dev/null

NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-exe "$EXE_OUT" "$PROBE" >/dev/null

"$EXE_OUT" > "$RUN_OUT"

if ! grep -Eq '^[0-9]+$' "$RUN_OUT"; then
  echo "[FAIL] expected numeric byte total from FileBox dynamic loop probe" >&2
  cat "$RUN_OUT" >&2
  exit 1
fi

echo "output_contract=rust-subset-filebox-dynamic-path-loop-exe-shape-v0"
echo "focused_filebox_main_dynamic_loop_probe_mir_emit=green"
echo "focused_filebox_main_dynamic_loop_probe_exe=green"
echo "summary=ok"
