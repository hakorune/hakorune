#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TMP_DIR="$(mktemp -d /tmp/hakorune_perf_aot_minimal_system_libs.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

REPORT="$TMP_DIR/report.out"
EXE="$TMP_DIR/ret0-minimal-system-libs.exe"
LDD_OUT="$TMP_DIR/ldd.out"
READELF_OUT="$TMP_DIR/readelf.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[perf-aot-minimal-system-libs] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

ROOT_DIR="$ROOT_DIR" bash -c '
  set -euo pipefail
  source "$ROOT_DIR/tools/perf/lib/aot_helpers.sh"
  HAKO_AOT_LDFLAGS="-static-libgcc" \
  NYASH_LLVM_LINK_SYSTEM_LIBS=minimal \
  NYASH_LLVM_LINK_WHOLE_ARCHIVE=0 \
  NYASH_LLVM_LINK_GC_SECTIONS=1 \
    perf_build_ret0_aot_exe "$ROOT_DIR" "$ROOT_DIR/target/release/hakorune" "$1" >/dev/null
' _ "$EXE"

"$EXE" >/dev/null
ldd "$EXE" >"$LDD_OUT" 2>&1 || true
readelf -d "$EXE" >"$READELF_OUT" 2>&1 || true

if grep -q 'libgcc_s' "$LDD_OUT" "$READELF_OUT"; then
  echo "[perf-aot-minimal-system-libs] libgcc_s must not be dynamically needed" >&2
  cat "$LDD_OUT" >&2
  cat "$READELF_OUT" >&2
  exit 1
fi

if grep -q 'libm.so' "$LDD_OUT" "$READELF_OUT"; then
  echo "[perf-aot-minimal-system-libs] libm must not be dynamically needed in minimal probe" >&2
  cat "$LDD_OUT" >&2
  cat "$READELF_OUT" >&2
  exit 1
fi

{
  echo "output_contract=perf-aot-minimal-system-libs-probe-v0"
  echo "input_contract=perf-aot-static-libgcc-probe-v0"
  echo "measurement_scope=exact_aot_minimal_system_libs_link_probe"
  echo "aot_ldflags=-static-libgcc"
  echo "link_system_libs=minimal"
  echo "ret0_exact_aot_build_status=ok"
  echo "ret0_exact_aot_run_status=ok"
  echo "dynamic_needed_libgcc_s=0"
  echo "dynamic_needed_libm=0"
  echo "dynamic_needed_libc=$(grep -q 'libc.so' "$LDD_OUT" && echo 1 || echo 0)"
  echo "dynamic_needed_ld_linux=$(grep -q 'ld-linux' "$LDD_OUT" && echo 1 || echo 0)"
  echo "default_link_mode_changed=0"
  echo "summary=ok"
} >"$REPORT"

require_line "$REPORT" "output_contract=perf-aot-minimal-system-libs-probe-v0"
require_line "$REPORT" "aot_ldflags=-static-libgcc"
require_line "$REPORT" "link_system_libs=minimal"
require_line "$REPORT" "ret0_exact_aot_build_status=ok"
require_line "$REPORT" "ret0_exact_aot_run_status=ok"
require_line "$REPORT" "dynamic_needed_libgcc_s=0"
require_line "$REPORT" "dynamic_needed_libm=0"
require_line "$REPORT" "default_link_mode_changed=0"
require_line "$REPORT" "summary=ok"

cat "$REPORT"
