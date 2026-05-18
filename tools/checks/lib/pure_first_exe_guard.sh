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

pure_first_guard_build_hakorune_debug() {
  cargo build -q --bin hakorune
}

pure_first_guard_hakorune_bin_for_mode() {
  local root_dir="$1"
  local mode="$2"
  local bin=""
  local rc=0

  case "$mode" in
    release)
      cargo build --release -q --bin hakorune || {
        rc=$?
        return "$rc"
      }
      bin="$root_dir/target/release/hakorune"
      ;;
    debug)
      cargo build -q --bin hakorune || {
        rc=$?
        return "$rc"
      }
      bin="$root_dir/target/debug/hakorune"
      ;;
    *)
      echo "[pure-first] ERROR: hakorune binary mode must be release or debug, got: $mode" >&2
      return 2
      ;;
  esac

  printf '%s\n' "$bin"
}

pure_first_guard_run_vm() {
  local tag="$1"
  local root_dir="$2"
  local app="$3"
  local vm_log="$4"
  local vm_bin_mode="${PURE_FIRST_VM_BIN:-release}"
  local vm_bin=""

  if ! vm_bin="$(pure_first_guard_hakorune_bin_for_mode "$root_dir" "$vm_bin_mode")"; then
    echo "[$tag] ERROR: failed to build hakorune VM binary mode=$vm_bin_mode" >&2
    return 1
  fi

  if ! NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 120 \
    "$vm_bin" --backend vm "$app" >"$vm_log" 2>&1; then
    echo "[$tag] ERROR: VM run failed" >&2
    sed -n '1,180p' "$vm_log" >&2
    return 1
  fi
}

pure_first_guard_parse_level() {
  local tag="$1"
  shift
  local level="L3"

  while [ "$#" -gt 0 ]; do
    case "$1" in
      --level)
        if [ "$#" -lt 2 ]; then
          echo "[$tag] ERROR: --level requires a value" >&2
          exit 2
        fi
        level="$2"
        shift 2
        ;;
      --level=*)
        level="${1#--level=}"
        shift
        ;;
      *)
        echo "[$tag] ERROR: unknown argument: $1" >&2
        exit 2
        ;;
    esac
  done

  case "$level" in
    L0|L1|L2|L3|L4) ;;
    *)
      echo "[$tag] ERROR: unsupported validation level: $level" >&2
      exit 2
      ;;
  esac
  printf '%s\n' "$level"
}

pure_first_guard_level_allows_vm() {
  case "$1" in
    L1|L2|L3|L4) return 0 ;;
    *) return 1 ;;
  esac
}

pure_first_guard_level_allows_mir() {
  case "$1" in
    L2|L3|L4) return 0 ;;
    *) return 1 ;;
  esac
}

pure_first_guard_level_allows_exe() {
  case "$1" in
    L3|L4) return 0 ;;
    *) return 1 ;;
  esac
}

pure_first_guard_emit_mir() {
  local root_dir="$1"
  local app="$2"
  local mir_json="$3"
  local emit_route="$root_dir/tools/smokes/v2/lib/emit_mir_route.sh"
  local emit_bin_mode="${PURE_FIRST_MIR_EMIT_BIN:-release}"
  local emit_bin=""
  local rc=0

  selfhost_phase_start "selfhost.emit_mir"

  case "$emit_bin_mode" in
    release)
      emit_bin="$(pure_first_guard_hakorune_bin_for_mode "$root_dir" release)" || {
        rc=$?
        selfhost_phase_fail "selfhost.emit_mir" "$rc"
        return "$rc"
      }
      ;;
    debug)
      emit_bin="$(pure_first_guard_hakorune_bin_for_mode "$root_dir" debug)" || {
        rc=$?
        selfhost_phase_fail "selfhost.emit_mir" "$rc"
        return "$rc"
      }
      ;;
    *)
      echo "[pure-first] ERROR: PURE_FIRST_MIR_EMIT_BIN must be release or debug, got: $emit_bin_mode" >&2
      selfhost_phase_fail "selfhost.emit_mir" 2
      return 2
      ;;
  esac

  NYASH_FEATURES=rune \
  NYASH_DISABLE_PLUGINS=1 \
  NYASH_BIN="$emit_bin" \
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
  local progress_file="$build_log.progress"

  mir_sha_before="$(sha256sum "$mir_json" | awk '{print $1}')"
  rm -f "$progress_file" 2>/dev/null || true

  pure_first_guard_route_preflight "$tag" "$root_dir" "$mir_json" "$build_log"

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

pure_first_guard_route_preflight() {
  local tag="$1"
  local root_dir="$2"
  local mir_json="$3"
  local build_log="$4"
  local preflight="$root_dir/tools/checks/pure_first_route_preflight.py"

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
