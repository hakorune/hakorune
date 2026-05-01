#!/bin/bash

phase2120_boundary_pure_prepare() {
  local root="$1"
  local script_name="$2"

  # shellcheck disable=SC1090
  source "$root/tools/smokes/v2/lib/test_runner.sh"
  require_env || exit 2

  export NYASH_LLVM_USE_CAPI="${NYASH_LLVM_USE_CAPI:-1}"
  export HAKO_V1_EXTERN_PROVIDER_C_ABI="${HAKO_V1_EXTERN_PROVIDER_C_ABI:-1}"
  export HAKO_BACKEND_COMPILE_RECIPE="${HAKO_BACKEND_COMPILE_RECIPE:-pure-first}"
  export HAKO_BACKEND_COMPAT_REPLAY="${HAKO_BACKEND_COMPAT_REPLAY:-none}"
  unset HAKO_CAPI_PURE
  if [[ "${NYASH_LLVM_USE_CAPI}" != "1" ||
        "${HAKO_V1_EXTERN_PROVIDER_C_ABI}" != "1" ||
        "${HAKO_BACKEND_COMPILE_RECIPE}" != "pure-first" ||
        "${HAKO_BACKEND_COMPAT_REPLAY}" != "none" ]]; then
    echo "[SKIP] ${script_name} (toggles off)" >&2
    exit 0
  fi

  local ffi_candidates=(
    "$root/target/release/libhako_llvmc_ffi.so"
    "$root/lib/libhako_llvmc_ffi.so"
  )
  local ffi_found=0
  local candidate
  for candidate in "${ffi_candidates[@]}"; do
    if [[ -f "$candidate" ]]; then
      ffi_found=1
      break
    fi
  done
  if [[ "$ffi_found" != "1" ]]; then
    echo "[SKIP] ${script_name} (FFI library not found)" >&2
    exit 0
  fi

  PHASE2120_BOUNDARY_NY_LLVM_C="$root/target/release/ny-llvmc"
  if [[ ! -x "$PHASE2120_BOUNDARY_NY_LLVM_C" ]]; then
    echo "[SKIP] ${script_name} (ny-llvmc missing: $PHASE2120_BOUNDARY_NY_LLVM_C)" >&2
    exit 0
  fi
}

phase2120_boundary_pure_require_kernel_symbol() {
  local root="$1"
  local symbol="$2"
  local script_name="$3"
  local archive="$root/target/release/libnyash_kernel.a"

  if [[ ! -f "$archive" ]]; then
    echo "[SKIP] ${script_name} (kernel archive missing: $archive)" >&2
    exit 0
  fi
  if ! command -v grep >/dev/null 2>&1; then
    echo "[SKIP] ${script_name} (grep missing for kernel symbol check)" >&2
    exit 0
  fi
  if ! grep -aFq "$symbol" "$archive"; then
    echo "[FAIL] ${script_name} (stale kernel archive: missing symbol $symbol in $archive)" >&2
    echo "hint: cargo build --release -p nyash_kernel" >&2
    exit 1
  fi
}

phase2120_boundary_pure_get_size() {
  local path="$1"
  if stat -c %s "$path" >/dev/null 2>&1; then
    stat -c %s "$path"
  elif stat -f %z "$path" >/dev/null 2>&1; then
    stat -f %z "$path"
  else
    echo 0
  fi
}

phase2120_boundary_pure_sha_cmd() {
  if command -v sha1sum >/dev/null 2>&1; then
    echo "sha1sum"
  elif command -v shasum >/dev/null 2>&1; then
    echo "shasum"
  else
    echo ""
  fi
}

phase2120_boundary_pure_run() {
  local json="$1"
  local expect_rc="$2"
  local exe_prefix="$3"
  local sha_cmd
  local last_size=""
  local last_hash=""
  local i=""

  sha_cmd="$(phase2120_boundary_pure_sha_cmd)"
  for i in 1 2 3; do
    local exe="/tmp/${exe_prefix}_${$}_${i}"
    local tmp_json="/tmp/${exe_prefix}_${$}_${i}.json"
    local build_rc=""
    local rc=""
    local size=""

    printf '%s\n' "$json" > "$tmp_json"
    set +e
    (
      unset HAKO_CAPI_PURE
      NYASH_LLVM_USE_CAPI="${NYASH_LLVM_USE_CAPI}" \
      HAKO_V1_EXTERN_PROVIDER_C_ABI="${HAKO_V1_EXTERN_PROVIDER_C_ABI}" \
      HAKO_BACKEND_COMPILE_RECIPE="${HAKO_BACKEND_COMPILE_RECIPE}" \
      HAKO_BACKEND_COMPAT_REPLAY="${HAKO_BACKEND_COMPAT_REPLAY}" \
        "$PHASE2120_BOUNDARY_NY_LLVM_C" --driver boundary --emit exe --in "$tmp_json" --out "$exe"
    ) >/dev/null 2>&1
    build_rc=$?
    set -e
    rm -f "$tmp_json"
    if [[ "$build_rc" -ne 0 ]]; then
      echo "[FAIL] ny-llvmc boundary emit rc=$build_rc" >&2
      exit 1
    fi
    if [[ ! -f "$exe" ]]; then
      echo "[FAIL] exe not produced: $exe" >&2
      exit 1
    fi

    set +e
    "$exe" >/dev/null 2>&1
    rc=$?
    set -e
    if [[ "$rc" -ne "$expect_rc" ]]; then
      echo "[FAIL] rc=$rc (expect $expect_rc)" >&2
      exit 1
    fi

    if [[ -n "$sha_cmd" ]]; then
      "$sha_cmd" "$exe" | awk '{print "[hash] "$1}'
    fi
    size="$(phase2120_boundary_pure_get_size "$exe")"
    echo "[size] $size"
    if [[ -z "$last_size" ]]; then
      last_size="$size"
    elif [[ "$size" != "$last_size" ]]; then
      echo "[FAIL] size mismatch ($size != $last_size)" >&2
      exit 1
    fi

    if [[ "${NYASH_HASH_STRICT:-0}" == "1" && -n "$sha_cmd" ]]; then
      local hash
      hash="$($sha_cmd "$exe" | awk '{print $1}')"
      if [[ -z "$last_hash" ]]; then
        last_hash="$hash"
      elif [[ "$hash" != "$last_hash" ]]; then
        echo "[FAIL] hash mismatch ($hash != $last_hash)" >&2
        exit 1
      fi
    fi
  done
}
