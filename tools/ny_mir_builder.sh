#!/usr/bin/env bash
# ny_mir_builder.sh — Minimal MIR Builder CLI (shell wrapper)
# Purpose: consume Nyash JSON IR and emit {obj|exe|ll|json} via the ny-llvmc mainline backend by default.
# Notes:
# - daily route is ny-llvmc(boundary) and remains the default entrypoint for object/exe emission.
# - llvmlite harness remains explicit compat/debug keep only; choose it with NYASH_LLVM_BACKEND=llvmlite when debugging.
# - native remains an explicit replay/canary lane only; choose it with NYASH_LLVM_BACKEND=native.

set -euo pipefail
[[ "${NYASH_CLI_VERBOSE:-0}" == "1" ]] && set -x

usage() {
  cat <<'USAGE'
Usage: tools/ny_mir_builder.sh [--in <file>|--stdin] [--emit {obj|exe|ll|json}] -o <out> [--target <triple>] [--nyrt <path>] [--quiet] [--verify-llvm]

Notes:
  - This wrapper defaults to the ny-llvmc mainline backend.
  - llvmlite remains an explicit compat/debug keep selected with NYASH_LLVM_BACKEND=llvmlite.
  - native remains an explicit replay/canary keep selected with NYASH_LLVM_BACKEND=native.
  - Input must be Nyash JSON IR (v0/v1). When --stdin is used, reads from stdin.
  - For --emit exe, kernel runtime must be built (crates/nyash_kernel). Use default paths if --nyrt omitted.
USAGE
}

llvm_route_trace_enabled() {
  case "${NYASH_LLVM_ROUTE_TRACE:-0}" in
    1|on|true|yes) return 0 ;;
    *) return 1 ;;
  esac
}

run_backend_quietly() {
  if llvm_route_trace_enabled; then
    "$@"
  else
    "$@" >/dev/null 2>&1
  fi
}

IN_MODE="stdin"   # stdin | file
IN_FILE=""
EMIT="obj"        # obj | exe | ll | json
OUT=""
TARGET=""
NYRT_DIR=""
VERIFY=0
QUIET=0
TMP_FILES=()
# Backend selection: default to the ny-llvmc mainline backend.
# llvmlite and native are explicit keep lanes selected with NYASH_LLVM_BACKEND.
if [[ -n "${NYASH_LLVM_BACKEND:-}" ]]; then
  BACKEND="${NYASH_LLVM_BACKEND}"
else
  if [[ -x "./target/release/ny-llvmc" ]]; then
    BACKEND="crate"
  else
    BACKEND="crate"  # keep for downstream case handling; will error gracefully later
  fi
fi

while [[ $# -gt 0 ]]; do
  case "$1" in
    -h|--help) usage; exit 0 ;;
    --in) IN_MODE="file"; IN_FILE="$2"; shift 2 ;;
    --stdin) IN_MODE="stdin"; shift ;;
    --emit) EMIT="$2"; shift 2 ;;
    -o) OUT="$2"; shift 2 ;;
    --target) TARGET="$2"; shift 2 ;;
    --nyrt) NYRT_DIR="$2"; shift 2 ;;
    --verify-llvm) VERIFY=1; shift ;;
    --quiet) QUIET=1; shift ;;
    *) echo "unknown arg: $1" >&2; usage; exit 2 ;;
  esac
done

if [[ -z "$OUT" ]]; then
  case "$EMIT" in
    obj) OUT="$(pwd)/target/aot_objects/a.o" ;;
    ll)  OUT="$(pwd)/target/aot_objects/a.ll" ;;
    exe) OUT="a.out" ;;
    json) OUT="/dev/stdout" ;;
    *) echo "error: invalid emit kind: $EMIT" >&2; exit 2 ;;
  esac
fi

# Require LLVM18 only for llvmlite backend (deprecated from auto-select)
if [[ "${NYASH_LLVM_BACKEND:-$BACKEND}" == "llvmlite" ]]; then
  if ! command -v llvm-config-18 >/dev/null 2>&1; then
    echo "error: llvm-config-18 not found (install LLVM 18 dev)" >&2
    exit 3
  fi
fi

# Build nyash + NyRT as needed（skip allowed）
LLVM_FEATURE=${NYASH_LLVM_FEATURE:-llvm}
SKIP_BUILD=${NYASH_LLVM_SKIP_BUILD:-0}
BUILD_TIMEOUT=${NYASH_LLVM_BUILD_TIMEOUT:-180}
if [[ "$SKIP_BUILD" != "1" ]]; then
  if [[ "$LLVM_FEATURE" == "llvm-inkwell-legacy" ]]; then
    _LLVMPREFIX=$(llvm-config-18 --prefix)
    timeout "$BUILD_TIMEOUT" \
      LLVM_SYS_181_PREFIX="${_LLVMPREFIX}" LLVM_SYS_180_PREFIX="${_LLVMPREFIX}" \
      cargo build --release -j 24 --features "${LLVM_FEATURE}" >/dev/null
  else
    timeout "$BUILD_TIMEOUT" cargo build --release -j 24 --features "${LLVM_FEATURE}" >/dev/null
  fi
  # Prebuild ny-llvmc when using the mainline backend
  if [[ "$BACKEND" == "crate" ]]; then
    (cd "$(dirname "$0")/.." && timeout "$BUILD_TIMEOUT" cargo build --release -j 24 -p nyash-llvm-compiler >/dev/null) || true
  fi
  if [[ "$EMIT" == "exe" ]]; then
    (cd crates/nyash_kernel && timeout "$BUILD_TIMEOUT" cargo build --release -j 24 >/dev/null)
  fi
fi

mkdir -p "$PWD/target/aot_objects"

# Prepare input
_STDIN_BUF=""
if [[ "$IN_MODE" == "stdin" ]]; then
  # Read all to a temp file to allow re-use
  _TMP_JSON=$(mktemp)
  cat > "$_TMP_JSON"
  IN_FILE="$_TMP_JSON"
fi

cleanup() {
  [[ -n "${_TMP_JSON:-}" && -f "$_TMP_JSON" ]] && rm -f "$_TMP_JSON" || true
  if ((${#TMP_FILES[@]} > 0)); then
    rm -f -- "${TMP_FILES[@]}" || true
  fi
}
trap cleanup EXIT

case "$EMIT" in
  json)
    # Normalization placeholder: currently pass-through
    cat "$IN_FILE" > "$OUT"
    [[ "$QUIET" == "0" ]] && echo "OK json:$OUT"
    ;;
  ll)
    # Ask nyash harness to dump LLVM IR (if supported via env)
    export NYASH_LLVM_DUMP_LL=1
    export NYASH_LLVM_LL_OUT="$OUT"
    if [[ "$VERIFY" == "1" ]]; then export NYASH_LLVM_VERIFY=1; fi
    # Prefer 'hakorune' binary if present (nyash is deprecated)
    BIN="./target/release/hakorune"
    [[ -x "$BIN" ]] || BIN="./target/release/nyash"
    if [[ "$LLVM_FEATURE" == "llvm-inkwell-legacy" ]]; then
      cat "$IN_FILE" | NYASH_LLVM_USE_HARNESS=1 LLVM_SYS_181_PREFIX="${_LLVMPREFIX}" LLVM_SYS_180_PREFIX="${_LLVMPREFIX}" \
        "$BIN" --backend llvm --ny-parser-pipe >/dev/null || true
    else
      cat "$IN_FILE" | NYASH_LLVM_USE_HARNESS=1 \
        "$BIN" --backend llvm --ny-parser-pipe >/dev/null || true
    fi
    if [[ ! -f "$OUT" ]]; then echo "error: failed to produce $OUT" >&2; exit 4; fi
    [[ "$QUIET" == "0" ]] && echo "OK ll:$OUT"
    ;;
  obj)
    case "$BACKEND" in
      crate)
        BIN_NYLLVMC="./target/release/ny-llvmc"
        if [[ ! -x "$BIN_NYLLVMC" ]]; then
          echo "error: ny-llvmc not found (cargo build -p nyash-llvm-compiler)" >&2; exit 4
        fi
        INPUT_FOR_CRATE="$IN_FILE"
        if [[ ! -f "$INPUT_FOR_CRATE" ]]; then
          echo "error: input not found: $INPUT_FOR_CRATE" >&2
          exit 2
        fi
        rm -f "$OUT"
        run_backend_quietly "$BIN_NYLLVMC" --in "$INPUT_FOR_CRATE" --emit obj --out "$OUT" || { echo "error: ny-llvmc failed" >&2; exit 4; }
        ;;
      native)
        if ! command -v llc >/dev/null 2>&1; then
          echo "error: llc not found (install LLVM tools)" >&2; exit 4
        fi
        rm -f "$OUT"
        # Optional verify: dump IR first and reject if PHI appears (simple guard)
        if [[ "${NYASH_LLVM_VERIFY_IR:-0}" == "1" ]]; then
          _TMP_LL=$(mktemp)
          if ! run_backend_quietly python3 "$PWD/tools/compat/native_llvm_builder.py" --in "$IN_FILE" --emit ll --out "$_TMP_LL"; then
            echo "error: native builder failed (ll)" >&2; rm -f "$_TMP_LL"; exit 4
          fi
          if grep -qE "[^a-zA-Z]phi[^a-zA-Z]" "$_TMP_LL"; then
            echo "error: IR verify failed (phi present)" >&2; rm -f "$_TMP_LL"; exit 4
          fi
          rm -f "$_TMP_LL"
        fi
        if ! run_backend_quietly python3 "$PWD/tools/compat/native_llvm_builder.py" --in "$IN_FILE" --emit obj --out "$OUT"; then
          echo "error: native builder failed" >&2; exit 4
        fi
        ;;
      llvmlite)
        # Directly use llvmlite harness with MIR v1 JSON input
        rm -f "$OUT"
        if ! run_backend_quietly python3 "$PWD/tools/llvmlite_harness.py" --in "$IN_FILE" --out "$OUT"; then
          echo "error: harness failed to produce $OUT" >&2; exit 4
        fi
        ;;
      *)
        echo "error: unsupported NYASH_LLVM_BACKEND=$BACKEND" >&2
        echo "hint: use 'crate' (mainline), 'llvmlite' (explicit keep), or 'native' (canary replay)" >&2
        exit 4
        ;;
    esac
    if [[ ! -f "$OUT" ]]; then echo "error: failed to produce $OUT" >&2; exit 4; fi
    [[ "$QUIET" == "0" ]] && echo "OK obj:$OUT"
    ;;
  exe)
    # Emit obj then link
    OBJ="$PWD/target/aot_objects/__tmp_builder.o"
    rm -f "$OBJ"
    case "$BACKEND" in
      crate)
        BIN_NYLLVMC="./target/release/ny-llvmc"
        if [[ ! -x "$BIN_NYLLVMC" ]]; then
          echo "error: ny-llvmc not found (cargo build -p nyash-llvm-compiler)" >&2; exit 4
        fi
        INPUT_FOR_CRATE="$IN_FILE"
        if [[ ! -f "$INPUT_FOR_CRATE" ]]; then
          echo "error: input not found: $INPUT_FOR_CRATE" >&2
          exit 2
        fi
        # Produce exe directly via ny-llvmc (lets ny-llvmc link)
        LIBS="${HAKO_AOT_LDFLAGS:-}"
        # Run and surface linker diagnostics on failure
        if ! "$BIN_NYLLVMC" --in "$INPUT_FOR_CRATE" --emit exe --nyrt target/release --libs "$LIBS" --out "$OUT"; then
          echo "error: ny-llvmc failed to link exe" >&2; exit 4
        fi
        ;;
      native)
        if ! command -v llc >/dev/null 2>&1; then
          echo "error: llc not found (install LLVM tools)" >&2; exit 4
        fi
        if [[ "${NYASH_LLVM_VERIFY_IR:-0}" == "1" ]]; then
          _TMP_LL=$(mktemp)
          if ! run_backend_quietly python3 "$PWD/tools/compat/native_llvm_builder.py" --in "$IN_FILE" --emit ll --out "$_TMP_LL"; then
            echo "error: native builder failed (ll)" >&2; rm -f "$_TMP_LL"; exit 4
          fi
          if grep -qE "[^a-zA-Z]phi[^a-zA-Z]" "$_TMP_LL"; then
            echo "error: IR verify failed (phi present)" >&2; rm -f "$_TMP_LL"; exit 4
          fi
          rm -f "$_TMP_LL"
        fi
        if ! run_backend_quietly python3 "$PWD/tools/compat/native_llvm_builder.py" --in "$IN_FILE" --emit obj --out "$OBJ"; then
          echo "error: native builder failed to produce object $OBJ" >&2; exit 4
        fi
        if [[ ! -f "$OBJ" ]]; then echo "error: failed to produce object $OBJ" >&2; exit 4; fi
        # Link with NyRT (same as llvmlite branch)
        NYRT_BASE=${NYRT_DIR:-"$PWD/crates/nyash_kernel"}
        cc "$OBJ" \
          -L target/release \
          -L "$NYRT_BASE/target/release" \
          -Wl,--whole-archive -lnyash_kernel -Wl,--no-whole-archive \
          -lpthread -ldl -lm -o "$OUT"
        ;;
      llvmlite)
        if ! run_backend_quietly python3 "$PWD/tools/llvmlite_harness.py" --in "$IN_FILE" --out "$OBJ"; then
          echo "error: harness failed to produce object $OBJ" >&2; exit 4
        fi
        if [[ ! -f "$OBJ" ]]; then echo "error: failed to produce object $OBJ" >&2; exit 4; fi
        # Link with NyRT
        NYRT_BASE=${NYRT_DIR:-"$PWD/crates/nyash_kernel"}
        cc "$OBJ" \
          -L target/release \
          -L "$NYRT_BASE/target/release" \
          -Wl,--whole-archive -lnyash_kernel -Wl,--no-whole-archive \
          -lpthread -ldl -lm -o "$OUT"
        ;;
      *)
        echo "error: unsupported NYASH_LLVM_BACKEND=$BACKEND" >&2
        echo "hint: use 'crate' (mainline), 'llvmlite' (explicit keep), or 'native' (canary replay)" >&2
        exit 4
        ;;
    esac
    [[ "$QUIET" == "0" ]] && echo "OK exe:$OUT"
    ;;
  *) echo "error: invalid emit kind: $EMIT" >&2; exit 2 ;;
esac

exit 0
