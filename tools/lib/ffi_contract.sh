#!/usr/bin/env bash
# ffi_contract.sh — Shared helpers for libhako_llvmc_ffi freshness checks.
# Library only; source from harness, perf, or gate scripts.

ffi_contract_artifact_candidates() {
  local root_dir=$1
  printf '%s\n' \
    "${root_dir}/target/release/libhako_llvmc_ffi.so" \
    "${root_dir}/target/release/libhako_llvmc_ffi.dylib" \
    "${root_dir}/target/release/hako_llvmc_ffi.dll" \
    "${root_dir}/lib/libhako_llvmc_ffi.so" \
    "${root_dir}/lib/libhako_llvmc_ffi.dylib" \
    "${root_dir}/lib/hako_llvmc_ffi.dll"
}

ffi_contract_source_files() {
  local root_dir=$1
  find "${root_dir}/lang/c-abi/shims" -maxdepth 1 -type f \
    \( -name '*.inc' -o -name 'hako_llvmc_ffi.c' -o -name 'hako_aot.c' -o -name 'hako_json_v1.c' -o -name 'hako_json_v1.h' -o -name 'hako_kernel.c' \) \
    -print0
  find "${root_dir}/plugins/nyash-json-plugin/c/yyjson" -maxdepth 1 -type f \
    \( -name '*.c' -o -name '*.h' \) \
    -print0
}

ffi_contract_artifact_path() {
  local root_dir=$1
  local candidate
  while IFS= read -r candidate; do
    if [[ -f "${candidate}" ]]; then
      printf '%s\n' "${candidate}"
      return 0
    fi
  done < <(ffi_contract_artifact_candidates "${root_dir}")
  return 1
}

ffi_contract_artifact_is_stale() {
  local root_dir=$1
  local artifact=$2
  local src
  while IFS= read -r -d '' src; do
    if [[ "${src}" -nt "${artifact}" ]]; then
      return 0
    fi
  done < <(ffi_contract_source_files "${root_dir}")
  return 1
}

ffi_contract_artifact_is_fresh() {
  local root_dir=$1
  local artifact
  artifact="$(ffi_contract_artifact_path "${root_dir}")" || return 1
  if ffi_contract_artifact_is_stale "${root_dir}" "${artifact}"; then
    return 1
  fi
  return 0
}

ffi_contract_require_fresh() {
  local root_dir=$1
  local artifact
  artifact="$(ffi_contract_artifact_path "${root_dir}")" || {
    echo "error: FFI library missing (run: bash tools/build_hako_llvmc_ffi.sh)" >&2
    return 1
  }
  if ffi_contract_artifact_is_stale "${root_dir}" "${artifact}"; then
    echo "error: FFI library is stale: ${artifact}" >&2
    echo "hint: run: bash tools/build_hako_llvmc_ffi.sh" >&2
    return 1
  fi
  return 0
}

ffi_contract_ensure_fresh() {
  local root_dir=$1
  if ffi_contract_artifact_is_fresh "${root_dir}"; then
    return 0
  fi
  echo "[ffi] rebuilding libhako_llvmc_ffi (missing or stale)"
  bash "${root_dir}/tools/build_hako_llvmc_ffi.sh"
}
