#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="s6c-pinned-corridor-structural-zero-smoke"
TEMP_DIR="$(mktemp -d /tmp/hako-s6c-structural-zero.XXXXXX)"
trap 'rm -rf -- "$TEMP_DIR"' EXIT

CC_CMD="${CC:-cc}"
OBJDUMP_CMD="${OBJDUMP:-objdump}"
CHECKER="$ROOT_DIR/tools/perf/s6c_pinned_corridor_structural_zero.py"
DRIVER="$ROOT_DIR/lang/c-abi/tests/s6c_pinned_corridor_structural_zero_driver.c"
RUNNER="$ROOT_DIR/lang/c-abi/tests/s6c_pinned_corridor_link_run.c"

"$CC_CMD" \
  -I"$ROOT_DIR/plugins/nyash-json-plugin/c/yyjson" \
  -o "$TEMP_DIR/structural-driver" \
  "$DRIVER" \
  "$ROOT_DIR/lang/c-abi/shims/hako_aot.c" \
  "$ROOT_DIR/lang/c-abi/shims/hako_json_v1.c" \
  "$ROOT_DIR/plugins/nyash-json-plugin/c/yyjson/yyjson.c" \
  -ldl

HAKO_PINNED_TEXT_REAL_CANDIDATE_JSON_OUT="$TEMP_DIR/real.json" \
  CARGO_BUILD_JOBS=4 \
  cargo test --manifest-path "$ROOT_DIR/Cargo.toml" --profile quick --lib -q \
    mir::builder::resolved_lowering::common_v2_s6c_cursor_cfg_tests::pinned_text_real_candidate_json_preserves_carrier_lineage \
    -- --exact >"$TEMP_DIR/cargo-test.stdout" 2>"$TEMP_DIR/cargo-test.stderr"

if "$TEMP_DIR/structural-driver" \
    "$TEMP_DIR/real.json" "$TEMP_DIR/capture-fail.o" \
    "$TEMP_DIR/missing/final.ll" \
    >"$TEMP_DIR/capture-fail.stdout" 2>"$TEMP_DIR/capture-fail.stderr"; then
  echo "[$TAG] ERROR: unavailable evidence output was accepted" >&2
  exit 1
fi
if [[ -e "$TEMP_DIR/capture-fail.o" ]] ||
   find "$TEMP_DIR" -name 'capture-fail.o.ptfb-tm-*.tmp' -print -quit | grep -q .; then
  echo "[$TAG] ERROR: failed evidence capture published an object" >&2
  exit 1
fi

"$TEMP_DIR/structural-driver" \
  "$TEMP_DIR/real.json" "$TEMP_DIR/real.o" "$TEMP_DIR/final.ll" \
  "$TEMP_DIR/origins.tsv"
PYTHONPATH="$ROOT_DIR/tools/hako_check" python3 - "$TEMP_DIR" <<'PY'
import sys
from pathlib import Path
from inspect_provenance_model import build_provenance

root = Path(sys.argv[1])
report = build_provenance(
    raw_path=root / "origins.tsv", mir_path=root / "real.json",
    llvm_path=root / "final.ll", mir_function="Main.find_ok/2",
    llvm_function="ny_main", issuer="selected_pinned_text_lowerer",
)
if len(report["relations"]) != 53:
    raise SystemExit("S6C provenance relation census mismatch")
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

"$CC_CMD" -O2 -no-pie \
  "$RUNNER" "$TEMP_DIR/candidate.o" \
  -L "$ROOT_DIR/target/quick" -lnyash_kernel \
  -lpthread -ldl -lm -o "$TEMP_DIR/link-run"
"$OBJDUMP_CMD" -d --disassemble=hako_s6c_candidate \
  "$TEMP_DIR/link-run" >"$TEMP_DIR/candidate.asm"

python3 "$CHECKER" \
  --ir "$TEMP_DIR/final.ll" \
  --assembly "$TEMP_DIR/candidate.asm" \
  --binary "$TEMP_DIR/link-run" \
  --report "$TEMP_DIR/evidence.json" \
  --commit "$(git -C "$ROOT_DIR" rev-parse HEAD)"

expect_reject() {
  local name="$1"
  local ir="$2"
  local assembly="$3"
  local report="$TEMP_DIR/$name.json"
  if python3 "$CHECKER" \
      --ir "$ir" --assembly "$assembly" --binary "$TEMP_DIR/link-run" \
      --report "$report" --commit negative \
      >"$TEMP_DIR/$name.stdout" 2>"$TEMP_DIR/$name.stderr"; then
    echo "[$TAG] ERROR: $name structural drift was accepted" >&2
    exit 1
  fi
  if [[ -e "$report" || -e "$report.tmp" ]]; then
    echo "[$TAG] ERROR: $name published negative evidence" >&2
    exit 1
  fi
}

TEMP_DIR="$TEMP_DIR" python3 - <<'PY'
import os
import pathlib
import re

root = pathlib.Path(os.environ["TEMP_DIR"])
ir = (root / "final.ll").read_text()
assembly = (root / "candidate.asm").read_text()
(root / "unexpected-call.ll").write_text(
    ir.replace("  %ptfc_byte_9 = load i8", "  call void @unexpected()\n  %ptfc_byte_9 = load i8", 1)
)
(root / "noalias.ll").write_text(ir.replace("i64 %r0", "i64 noalias %r0", 1))
(root / "wide-load.ll").write_text(ir.replace("load i8, ptr %ptfc_byte_ptr_9", "load i32, ptr %ptfc_byte_ptr_9", 1))
(root / "indirect.asm").write_text(
    re.sub(r"\bcallq?\s+[^\n]*<hako_text_formal_residence_enter_v1>", "call   *%rax", assembly, count=1)
)
PY

expect_reject unexpected-call "$TEMP_DIR/unexpected-call.ll" "$TEMP_DIR/candidate.asm"
expect_reject noalias "$TEMP_DIR/noalias.ll" "$TEMP_DIR/candidate.asm"
expect_reject wide-load "$TEMP_DIR/wide-load.ll" "$TEMP_DIR/candidate.asm"
expect_reject indirect-call "$TEMP_DIR/final.ll" "$TEMP_DIR/indirect.asm"

echo "[$TAG] ok (same final ModuleRef + final-linked candidate evidence; no compiler authority)"
