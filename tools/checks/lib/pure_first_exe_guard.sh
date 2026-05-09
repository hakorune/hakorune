#!/usr/bin/env bash
# pure_first_exe_guard.sh - shared pure-first EXE guard helpers

PURE_FIRST_EXE_GUARD_LIB_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "${PURE_FIRST_EXE_GUARD_LIB_DIR}/guard_common.sh"

pure_first_guard_build_toolchain() {
  cargo build -q --bin hakorune
  cargo build --release -q -p nyash_kernel
  cargo build --release -q -p nyash-llvm-compiler --bin ny-llvmc
  bash tools/build_hako_llvmc_ffi.sh >/dev/null
}

pure_first_guard_emit_mir() {
  local root_dir="$1"
  local app="$2"
  local mir_json="$3"

  NYASH_FEATURES=rune \
  NYASH_DISABLE_PLUGINS=1 \
  "$root_dir/target/debug/hakorune" --emit-mir-json "$mir_json" "$app" >/dev/null
}

pure_first_guard_build_exe() {
  local tag="$1"
  local root_dir="$2"
  local app="$3"
  local mir_json="$4"
  local exe_out="$5"
  local build_log="$6"

  if ! NYASH_BIN="$root_dir/target/debug/hakorune" \
    NYASH_FEATURES=rune \
    NYASH_DISABLE_PLUGINS=1 \
    NYASH_LLVM_ROUTE_TRACE=1 \
    HAKO_BACKEND_COMPILE_RECIPE=pure-first \
    HAKO_BACKEND_COMPAT_REPLAY=none \
    timeout 120 tools/selfhost/selfhost_build.sh \
      --in "$app" \
      --mir "$mir_json" \
      --exe "$exe_out" >"$build_log" 2>&1; then
    echo "[$tag] ERROR: pure-first build failed" >&2
    sed -n '1,240p' "$build_log" >&2
    exit 1
  fi
}

pure_first_guard_assert_clean_build_log() {
  local tag="$1"
  local build_log="$2"

  if rg -F -q 'unsupported_pure_shape' "$build_log"; then
    echo "[$tag] ERROR: pure-first reported unsupported shape" >&2
    sed -n '1,220p' "$build_log" >&2
    exit 1
  fi

  if rg -F -q 'compat_replay=harness' "$build_log"; then
    echo "[$tag] ERROR: compat replay must stay disabled" >&2
    sed -n '1,180p' "$build_log" >&2
    exit 1
  fi
}

pure_first_guard_run_exe() {
  local tag="$1"
  local exe_out="$2"
  local run_log="$3"

  if ! "$exe_out" >"$run_log" 2>&1; then
    echo "[$tag] ERROR: EXE run failed" >&2
    sed -n '1,160p' "$run_log" >&2
    exit 1
  fi
}
