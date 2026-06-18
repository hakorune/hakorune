#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

PY_SELFTEST="apps/rust-subset-to-hako/selftest.py"
JSON_PROBE="apps/rust-subset-to-hako/probes/json_probe.hako"
CONVERTER="apps/rust-subset-to-hako/convert.hako"

echo "[rust-subset/smoke] python reference selftest"
python3 "$PY_SELFTEST"

echo "[rust-subset/smoke] ensure ny-llvmc FFI"
bash tools/build_hako_llvmc_ffi.sh >/dev/null

echo "[rust-subset/smoke] emit MIR JSON: json probe"
NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-mir-json /tmp/hako_json_probe.mir.json "$JSON_PROBE" >/tmp/hako_json_probe.emit.log

echo "[rust-subset/smoke] emit MIR JSON: converter"
NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-mir-json /tmp/rust_subset_convert.mir.json "$CONVERTER" >/tmp/rust_subset_convert.emit.log

echo "[rust-subset/smoke] EXE: json probe"
rm -f /tmp/hako_json_probe
NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-exe /tmp/hako_json_probe "$JSON_PROBE" \
  >/tmp/hako_json_probe.exe.log 2>&1
/tmp/hako_json_probe >/tmp/hako_json_probe.out 2>/tmp/hako_json_probe.err
grep -Fq "field.kind.value=Program" /tmp/hako_json_probe.out
grep -Fq "items.length=0" /tmp/hako_json_probe.out

echo "[rust-subset/smoke] EXE: converter parity"
rm -f /tmp/hako_rust_subset_convert
NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-exe /tmp/hako_rust_subset_convert "$CONVERTER" \
  >/tmp/hako_rust_subset_convert.exe.log 2>&1
/tmp/hako_rust_subset_convert \
  >/tmp/hako_rust_subset_convert.out.raw \
  2>/tmp/hako_rust_subset_convert.err
sed '/^Result: /d' /tmp/hako_rust_subset_convert.out.raw \
  >/tmp/hako_rust_subset_convert.out
diff -u apps/rust-subset-to-hako/examples/simple_expected.hako \
  /tmp/hako_rust_subset_convert.out

echo "summary=ok"
