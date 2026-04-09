#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
source "$ROOT_DIR/tools/lib/ffi_contract.sh"

usage() {
  cat << USAGE
Usage: tools/run_llvm_harness.sh [--no-build] <input.hako> [-- <args...>]

Builds LLVM-harness prerequisites and runs the program via the explicit
compat/probe keep lane:
  NYASH_LLVM_USE_HARNESS=1 "$ROOT_DIR/target/release/hakorune" --backend llvm <input.hako>
  Rebuilds libhako_llvmc_ffi when shim sources are newer than the artifact.

Options:
  --no-build   Skip cargo builds and run with existing artifacts.
USAGE
}

if [[ $# -lt 1 ]]; then
  usage
  exit 1
fi

NO_BUILD=0
if [[ "${1:-}" == "--no-build" ]]; then
  NO_BUILD=1
  shift || true
fi

INPUT="${1:-}"
shift || true

if [[ "$INPUT" == "-h" || "$INPUT" == "--help" ]]; then
  usage
  exit 0
fi

if [[ ! -f "$INPUT" ]]; then
  echo "error: input file not found: $INPUT" >&2
  exit 1
fi

CARGO_TARGET_DIR_EFFECTIVE="${CARGO_TARGET_DIR:-$ROOT_DIR/target}"
BIN_DEFAULT="$CARGO_TARGET_DIR_EFFECTIVE/release/hakorune"
BIN="${NYASH_BIN:-$BIN_DEFAULT}"

if [[ "$NO_BUILD" != "1" ]]; then
  echo "[1/5] Building hakorune (llvm feature)..."
  cargo build --release -p nyash-rust --features llvm --bin hakorune -j 24

  echo "[2/5] Building ny-llvmc..."
  cargo build --release -p nyash-llvm-compiler -j 24

  echo "[3/5] Building nyash_kernel..."
  cargo build --release -p nyash_kernel -j 24
else
  # Fail-fast: avoid silently running with stale/missing artifacts.
  if [[ ! -x "$CARGO_TARGET_DIR_EFFECTIVE/release/ny-llvmc" ]]; then
    echo "error: --no-build requested but ny-llvmc is missing: $CARGO_TARGET_DIR_EFFECTIVE/release/ny-llvmc" >&2
    echo "hint: run without --no-build once to build LLVM harness prerequisites" >&2
    exit 1
  fi
  if [[ ! -f "$CARGO_TARGET_DIR_EFFECTIVE/release/libnyash_kernel.a" ]]; then
    echo "error: --no-build requested but nyash_kernel staticlib is missing: $CARGO_TARGET_DIR_EFFECTIVE/release/libnyash_kernel.a" >&2
    echo "hint: run without --no-build once to build LLVM harness prerequisites" >&2
    exit 1
  fi
fi

echo "[4/5] Ensuring libhako_llvmc_ffi freshness..."
if [[ "$NO_BUILD" == "1" ]]; then
  ffi_contract_require_fresh "$ROOT_DIR"
else
  ffi_contract_ensure_fresh "$ROOT_DIR"
fi

if [[ ! -x "$BIN" ]]; then
  if [[ -x "$CARGO_TARGET_DIR_EFFECTIVE/release/nyash" ]]; then
    BIN="$CARGO_TARGET_DIR_EFFECTIVE/release/nyash"
  else
    if [[ "$NO_BUILD" == "1" ]]; then
      echo "error: --no-build requested but compiler binary is missing/not executable: $BIN" >&2
      echo "hint: run without --no-build once, or set NYASH_BIN/CARGO_TARGET_DIR correctly" >&2
    else
      echo "error: compiler binary not found/executable after build: $BIN" >&2
      echo "hint: ensure NYASH_BIN points to an existing binary or set CARGO_TARGET_DIR correctly" >&2
    fi
    exit 1
  fi
fi

echo "[5/5] Running LLVM harness..."
NYASH_LLVM_USE_HARNESS=1 "$BIN" --backend llvm "$INPUT" "$@"
