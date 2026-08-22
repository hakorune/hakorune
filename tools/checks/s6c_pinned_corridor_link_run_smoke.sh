#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="s6c-pinned-corridor-link-run"
TEMP_DIR="$(mktemp -d /tmp/hako-s6c-link-run.XXXXXX)"
trap 'rm -rf -- "$TEMP_DIR"' EXIT

bash "$ROOT_DIR/tools/build_hako_llvmc_ffi.sh" >/dev/null
HAKO_PINNED_TEXT_REAL_CANDIDATE_JSON_OUT="$TEMP_DIR/real.json" \
  CARGO_BUILD_JOBS=4 \
  cargo test --manifest-path "$ROOT_DIR/Cargo.toml" --profile quick --lib -q \
    mir::builder::resolved_lowering::common_v2_s6c_cursor_cfg_tests::pinned_text_real_candidate_json_preserves_carrier_lineage \
    -- --exact >"$TEMP_DIR/cargo-test.stdout" 2>"$TEMP_DIR/cargo-test.stderr"

ROOT_DIR="$ROOT_DIR" TEMP_DIR="$TEMP_DIR" python3 - <<'PY'
import ctypes
import os
import pathlib

root = pathlib.Path(os.environ["ROOT_DIR"])
directory = pathlib.Path(os.environ["TEMP_DIR"])
library = ctypes.CDLL(str(root / "target/release/libhako_llvmc_ffi.so"))
compile_json = library.hako_llvmc_compile_json_pure_first
compile_json.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.POINTER(ctypes.c_void_p)]
compile_json.restype = ctypes.c_int
library.hako_mem_free.argtypes = [ctypes.c_void_p]
error = ctypes.c_void_p()
output = directory / "real.o"
rc = compile_json(
    str(directory / "real.json").encode(),
    str(output).encode(),
    ctypes.byref(error),
)
message = ctypes.string_at(error.value).decode(errors="replace") if error.value else ""
if error.value:
    library.hako_mem_free(error)
if rc != 0 or message or not output.is_file() or output.stat().st_size == 0:
    raise SystemExit(f"real candidate object emission failed: {message}")
PY

objcopy --redefine-sym 'ny_main=hako_s6c_candidate' \
  "$TEMP_DIR/real.o" "$TEMP_DIR/candidate.o"

if ! CARGO_BUILD_JOBS=4 cargo build \
    --manifest-path "$ROOT_DIR/Cargo.toml" \
    --profile quick -p nyash_kernel --features promotion-test-support -q \
    >"$TEMP_DIR/cargo-build.stdout" 2>"$TEMP_DIR/cargo-build.stderr"; then
  sed -n '1,240p' "$TEMP_DIR/cargo-build.stderr" >&2
  exit 1
fi

"${CC:-cc}" -O2 -no-pie \
  "$ROOT_DIR/lang/c-abi/tests/s6c_pinned_corridor_link_run.c" \
  "$TEMP_DIR/candidate.o" \
  -L "$ROOT_DIR/target/quick" \
  -lnyash_kernel \
  -lpthread -ldl -lm -o "$TEMP_DIR/link-run"

"$TEMP_DIR/link-run"
if find "$TEMP_DIR" -name '*.ptfb-tm-*.tmp' -print -quit | grep -q .; then
  echo "[$TAG] ERROR: TargetMachine temporary survived" >&2
  exit 1
fi
echo "[$TAG] ok (real object links to feature-gated NyRT and matches the independent oracle)"
