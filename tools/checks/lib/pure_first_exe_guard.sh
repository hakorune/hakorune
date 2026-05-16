#!/usr/bin/env bash
# pure_first_exe_guard.sh - shared pure-first EXE guard helpers

PURE_FIRST_EXE_GUARD_LIB_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PURE_FIRST_EXE_GUARD_ROOT="$(cd "${PURE_FIRST_EXE_GUARD_LIB_DIR}/../../.." && pwd)"
source "${PURE_FIRST_EXE_GUARD_LIB_DIR}/guard_common.sh"
source "${PURE_FIRST_EXE_GUARD_ROOT}/tools/selfhost/lib/selfhost_progress.sh"

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
  local emit_route="$root_dir/tools/smokes/v2/lib/emit_mir_route.sh"
  local rc=0

  selfhost_phase_start "selfhost.emit_mir"
  NYASH_FEATURES=rune \
  NYASH_DISABLE_PLUGINS=1 \
  NYASH_BIN="$root_dir/target/debug/hakorune" \
  "$emit_route" --route direct --out "$mir_json" --input "$app" >/dev/null || rc=$?
  if [ "$rc" -eq 0 ]; then
    selfhost_phase_done "selfhost.emit_mir"
  else
    selfhost_phase_fail "selfhost.emit_mir" "$rc"
  fi
  return "$rc"
}

pure_first_guard_print_progress_closeout() {
  local tag="$1"
  local progress_file="$2"

  if [ -s "$progress_file" ]; then
    echo "[$tag] last selfhost progress: $(cat "$progress_file")" >&2
  else
    echo "[$tag] last selfhost progress: unavailable" >&2
  fi
}

pure_first_guard_build_exe() {
  local tag="$1"
  local root_dir="$2"
  local app="$3"
  local mir_json="$4"
  local exe_out="$5"
  local build_log="$6"
  local mir_sha_before
  local mir_sha_after
  local preflight="$root_dir/tools/checks/pure_first_route_preflight.py"
  local progress_file="$build_log.progress"

  mir_sha_before="$(sha256sum "$mir_json" | awk '{print $1}')"
  rm -f "$progress_file" 2>/dev/null || true

  if ! (
    selfhost_phase_start "selfhost.route_preflight"
    if "$preflight" "$mir_json"; then
      selfhost_phase_done "selfhost.route_preflight"
    else
      rc=$?
      selfhost_phase_fail "selfhost.route_preflight" "$rc"
      exit "$rc"
    fi
  ) >"$build_log" 2>&1; then
    echo "[$tag] ERROR: pure-first route preflight failed" >&2
    sed -n '1,240p' "$build_log" >&2
    exit 1
  fi

  if ! HAKO_SELFHOST_PROGRESS_FILE="$progress_file" \
    NYASH_BIN="$root_dir/target/debug/hakorune" \
    NYASH_FEATURES=rune \
    NYASH_DISABLE_PLUGINS=1 \
    NYASH_LLVM_ROUTE_TRACE=1 \
    HAKO_BACKEND_COMPILE_RECIPE=pure-first \
    HAKO_BACKEND_COMPAT_REPLAY=none \
    timeout 120 tools/selfhost/selfhost_build.sh \
      --in "$app" \
      --mir-in "$mir_json" \
      --exe "$exe_out" >>"$build_log" 2>&1; then
    echo "[$tag] ERROR: pure-first build failed" >&2
    pure_first_guard_print_progress_closeout "$tag" "$progress_file"
    sed -n '1,240p' "$build_log" >&2
    exit 1
  fi

  mir_sha_after="$(sha256sum "$mir_json" | awk '{print $1}')"
  if [ "$mir_sha_before" != "$mir_sha_after" ]; then
    echo "[$tag] ERROR: pure-first EXE build rewrote preflight MIR artifact" >&2
    echo "[$tag] before=$mir_sha_before after=$mir_sha_after file=$mir_json" >&2
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
