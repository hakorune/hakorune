#!/usr/bin/env bash
set -euo pipefail

if [[ "${NYASH_CLI_VERBOSE:-0}" == "1" ]]; then
  set -x
fi

usage() {
  cat << USAGE
Usage: tools/build_llvm.sh <input.hako> [-o <output>]

Compiles a Nyash program with the LLVM backend to an object (.o),
links it with the NyRT static runtime, and produces a native executable.

Options:
  -o <output>   Output executable path (default: tmp/app)

Requirements:
  - LLVM 18 development (llvm-config-18)
  - Nyash Kernel static runtime (crates/nyash_kernel)

Implementation detail:
  - `NYASH_LLVM_COMPILER=crate` keeps the ny-llvmc route.
  - `NYASH_LLVM_BACKEND=native` switches that crate route to `ny-llvmc --driver native`.
USAGE
}

if [[ $# -lt 1 ]]; then usage; exit 1; fi

INPUT=""
OUT="tmp/app"
while [[ $# -gt 0 ]]; do
  case "$1" in
    -h|--help) usage; exit 0 ;;
    -o) OUT="$2"; shift 2 ;;
    *) INPUT="$1"; shift ;;
  esac
done

if [[ ! -f "$INPUT" ]]; then
  echo "error: input file not found: $INPUT" >&2
  exit 1
fi

if ! command -v llvm-config-18 >/dev/null 2>&1; then
  echo "error: llvm-config-18 not found (install LLVM 18 dev)." >&2
  exit 2
fi

# Global build lock (SSOT):
# - Protects shared outputs (target/aot_objects, tmp exe names) from parallel smoke collisions.
# - Can be disabled explicitly via NYASH_LLVM_BUILD_LOCK=0.
# - Nested callers that already hold the lock should set NYASH_LLVM_BUILD_LOCK_HELD=1.
if [[ "${NYASH_LLVM_BUILD_LOCK:-1}" != "0" ]] \
  && [[ "${NYASH_LLVM_BUILD_LOCK_HELD:-0}" != "1" ]] \
  && command -v flock >/dev/null 2>&1; then
  LOCK_TARGET_DIR="${CARGO_TARGET_DIR:-$PWD/target}"
  mkdir -p "$LOCK_TARGET_DIR"
  LOCK_FILE="${NYASH_LLVM_BUILD_LOCK_FILE:-$LOCK_TARGET_DIR/.build_llvm.lock}"
  LOCK_TIMEOUT_SECS="${NYASH_LLVM_BUILD_LOCK_TIMEOUT_SECS:-120}"

  exec {NYASH_LLVM_LOCK_FD}> "$LOCK_FILE"
  if ! flock -w "$LOCK_TIMEOUT_SECS" "$NYASH_LLVM_LOCK_FD"; then
    echo "error: build_llvm lock timeout (${LOCK_TIMEOUT_SECS}s): $LOCK_FILE" >&2
    eval "exec ${NYASH_LLVM_LOCK_FD}>&-"
    exit 3
  fi
  trap 'flock -u "$NYASH_LLVM_LOCK_FD" 2>/dev/null || true; eval "exec ${NYASH_LLVM_LOCK_FD}>&-" 2>/dev/null || true' EXIT
fi

# Use the cargo target dir when set (helps LLVM EXE smokes that build under /tmp).
CARGO_TARGET_DIR_EFFECTIVE="${CARGO_TARGET_DIR:-$PWD/target}"

# Rust builds (especially rmeta/rlib finalization) may fail with EXDEV when the temp dir
# is not compatible with the output directory. Prefer a temp dir under the final output
# folder so rustc can atomically persist artifacts without cross-device rename issues.
#
# NOTE: release/deps may not exist yet on first build, so create it eagerly.
# TMPDIR configuration (SSOT: tools/smokes/v2/lib/env.sh sets TARGET_TMPDIR)
# Use TARGET_TMPDIR if set by smoke framework, otherwise fallback to cargo deps dir
TMPDIR_EFFECTIVE="${TMPDIR:-${TARGET_TMPDIR:-$CARGO_TARGET_DIR_EFFECTIVE/release/deps}}"
mkdir -p "$TMPDIR_EFFECTIVE"
export TMPDIR="$TMPDIR_EFFECTIVE"

BIN_DEFAULT="$CARGO_TARGET_DIR_EFFECTIVE/release/hakorune"
BIN="${NYASH_BIN:-$BIN_DEFAULT}"

echo "[1/4] Building hakorune (feature selectable) ..."
# Select LLVM feature: default harness (llvm), or legacy inkwell when NYASH_LLVM_FEATURE=llvm-inkwell-legacy
LLVM_FEATURE=${NYASH_LLVM_FEATURE:-llvm}

# Use 24 threads for parallel build
if [[ "$LLVM_FEATURE" == "llvm-inkwell-legacy" ]]; then
  # Legacy inkwell需要LLVM_SYS_180_PREFIX
  _LLVMPREFIX=$(llvm-config-18 --prefix)
  LLVM_SYS_181_PREFIX="${_LLVMPREFIX}" LLVM_SYS_180_PREFIX="${_LLVMPREFIX}" \
    CARGO_INCREMENTAL=0 cargo build --release -j 24 -p nyash-rust --features "$LLVM_FEATURE" >/dev/null
else
  # llvm-harness（デフォルト）はLLVM_SYS_180_PREFIX不要
  CARGO_INCREMENTAL=0 cargo build --release -j 24 -p nyash-rust --features "$LLVM_FEATURE" >/dev/null
fi

if [[ ! -x "$BIN" ]]; then
  # Backward compatible fallback for older layouts.
  if [[ -x "$CARGO_TARGET_DIR_EFFECTIVE/release/nyash" ]]; then
    BIN="$CARGO_TARGET_DIR_EFFECTIVE/release/nyash"
  else
    echo "error: compiler binary not found/executable after build: $BIN" >&2
    echo "hint: ensure NYASH_BIN points to an existing binary or set CARGO_TARGET_DIR correctly" >&2
    exit 1
  fi
fi

echo "[2/4] Emitting object (.o) via LLVM backend ..."
# Default object output path under $CARGO_TARGET_DIR/aot_objects
mkdir -p "$CARGO_TARGET_DIR_EFFECTIVE/aot_objects"
stem=$(basename "$INPUT")
stem=${stem%.hako}
OBJ="${NYASH_LLVM_OBJ_OUT:-$CARGO_TARGET_DIR_EFFECTIVE/aot_objects/${stem}.o}"
if [[ "${NYASH_LLVM_SKIP_EMIT:-0}" != "1" ]]; then
  rm -f "$OBJ"
  COMPILER_MODE=${NYASH_LLVM_COMPILER:-harness}
  case "$COMPILER_MODE" in
    crate)
      NYLLVMC_ARGS=()
      if [[ "${NYASH_LLVM_BACKEND:-}" == "native" ]]; then
        NYLLVMC_ARGS+=(--driver native)
      fi
      # Use crates/nyash-llvm-compiler (ny-llvmc): requires pre-generated MIR JSON path in NYASH_LLVM_MIR_JSON
      if [[ -z "${NYASH_LLVM_MIR_JSON:-}" ]]; then
        # Auto‑emit MIR JSON via nyash CLI flag
        mkdir -p tmp
        NYASH_LLVM_MIR_JSON="tmp/nyash_crate_mir.json"
        echo "    emitting MIR JSON: $NYASH_LLVM_MIR_JSON" >&2
        "$BIN" --emit-mir-json "$NYASH_LLVM_MIR_JSON" --backend mir "$INPUT" >/dev/null
      fi
      echo "    using ny-llvmc (crate) with JSON: $NYASH_LLVM_MIR_JSON" >&2
      cargo build --release -p nyash-llvm-compiler >/dev/null
      # Optional schema validation when explicitly requested
        if [[ "${NYASH_LLVM_VALIDATE_JSON:-0}" == "1" ]]; then
        if [[ -f tools/validate_mir_json.py ]]; then
          if ! python3 -m jsonschema --version >/dev/null 2>&1; then
            echo "[warn] jsonschema not available; skipping schema validation" >&2
          else
            echo "    validating MIR JSON schema ..." >&2
            python3 tools/validate_mir_json.py "$NYASH_LLVM_MIR_JSON" || {
              echo "error: MIR JSON validation failed" >&2; exit 3; }
          fi
        fi
      fi
        if [[ "${NYASH_LLVM_EMIT:-obj}" == "exe" ]]; then
          echo "    emitting EXE via ny-llvmc (crate) ..." >&2
        # Ensure Nyash Kernel is built (for libnyash_kernel.a)
        if [[ ! -f crates/nyash_kernel/target/release/libnyash_kernel.a && "${NYASH_LLVM_SKIP_NYRT_BUILD:-0}" != "1" ]]; then
          ( cd crates/nyash_kernel && cargo build --release -j 24 >/dev/null )
        fi
        NYRT_DIR_HINT="${NYASH_LLVM_NYRT:-crates/nyash_kernel/target/release}"
        ./target/release/ny-llvmc "${NYLLVMC_ARGS[@]}" --in "$NYASH_LLVM_MIR_JSON" --out "$OUT" --emit exe --nyrt "$NYRT_DIR_HINT" ${NYASH_LLVM_LIBS:+--libs "$NYASH_LLVM_LIBS"}
        echo "✅ Done: $OUT"; echo "   (runtime may require nyash.toml and plugins depending on app)"; exit 0
      else
        ./target/release/ny-llvmc "${NYLLVMC_ARGS[@]}" --in "$NYASH_LLVM_MIR_JSON" --out "$OBJ"
      fi
      ;;
  esac
  if [[ "$COMPILER_MODE" == "harness" ]]; then
    if [[ "${NYASH_LLVM_FEATURE:-llvm}" == "llvm-inkwell-legacy" ]]; then
      # Legacy path: do not use harness (LLVM_SYS_180_PREFIX needed)
      _LLVMPREFIX=$(llvm-config-18 --prefix)
      NYASH_LLVM_OBJ_OUT="$OBJ" LLVM_SYS_181_PREFIX="${_LLVMPREFIX}" LLVM_SYS_180_PREFIX="${_LLVMPREFIX}" \
        "$BIN" --backend llvm "$INPUT" >/dev/null || true
    else
      # Harness path (Python llvmlite - LLVM_SYS_180_PREFIX不要)
      NYASH_LLVM_OBJ_OUT="$OBJ" NYASH_LLVM_USE_HARNESS=1 \
        "$BIN" --backend llvm "$INPUT" >/dev/null || true
    fi
  fi
fi
if [[ ! -f "$OBJ" ]]; then
  echo "error: object not generated: $OBJ" >&2
  echo "hint: you can pre-generate it (e.g. via --run-task smoke_obj_*) and set NYASH_LLVM_SKIP_EMIT=1" >&2
  exit 3
fi

if [[ "${NYASH_LLVM_ONLY_OBJ:-0}" == "1" ]]; then
  echo "[3/4] Skipping link: object generated at $OBJ (NYASH_LLVM_ONLY_OBJ=1)"
  exit 0
fi

echo "[3/4] Building Nyash Kernel static runtime ..."
if [[ "${NYASH_LLVM_SKIP_NYRT_BUILD:-0}" == "1" ]]; then
  echo "    Skipping Nyash Kernel build (NYASH_LLVM_SKIP_NYRT_BUILD=1)"
else
  NYRT_LIB_PRIMARY="$CARGO_TARGET_DIR_EFFECTIVE/release/libnyash_kernel.a"
  NYRT_LIB_ALT="crates/nyash_kernel/target/release/libnyash_kernel.a"
  if [[ ( -f "$NYRT_LIB_PRIMARY" || -f "$NYRT_LIB_ALT" ) && "${NYASH_LLVM_FORCE_NYRT_BUILD:-0}" != "1" ]]; then
    echo "    Using cached Nyash Kernel runtime (set NYASH_LLVM_FORCE_NYRT_BUILD=1 to rebuild)"
  else
  # Use 24 threads for parallel build
  ( cd crates/nyash_kernel && cargo build --release -j 24 >/dev/null )
  fi
fi

# Ensure output directory exists
mkdir -p "$(dirname "$OUT")"
echo "[4/4] Linking $OUT ..."
cc "$OBJ" \
  -L "$CARGO_TARGET_DIR_EFFECTIVE/release" \
  -L crates/nyash_kernel/target/release \
  -Wl,--whole-archive -lnyash_kernel -Wl,--no-whole-archive \
  -lpthread -ldl -lm -o "$OUT"

echo "✅ Done: $OUT"
echo "   (runtime requires nyash.toml and plugin .so per config)"
