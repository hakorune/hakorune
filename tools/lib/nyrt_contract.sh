#!/usr/bin/env bash
# nyrt_contract.sh - Shared libnyash_kernel.a freshness checks.
# Library only; source from LLVM harness/build scripts.

nyrt_contract_source_files() {
  local root_dir=$1
  find "${root_dir}/crates/nyash_kernel/src" -type f -name '*.rs' -print0
  printf '%s\0' \
    "${root_dir}/crates/nyash_kernel/Cargo.toml" \
    "${root_dir}/Cargo.lock"
}

nyrt_contract_newer_source() {
  local root_dir=$1
  local artifact=$2
  local src
  while IFS= read -r -d '' src; do
    if [[ -f "${src}" && "${src}" -nt "${artifact}" ]]; then
      printf '%s\n' "${src}"
      return 0
    fi
  done < <(nyrt_contract_source_files "${root_dir}")
  return 1
}

nyrt_contract_artifact_is_fresh() {
  local root_dir=$1
  local artifact=$2
  [[ -f "${artifact}" ]] || return 1
  if nyrt_contract_newer_source "${root_dir}" "${artifact}" >/dev/null; then
    return 1
  fi
  return 0
}

nyrt_contract_require_fresh_artifact() {
  local root_dir=$1
  local artifact=$2
  if [[ ! -f "${artifact}" ]]; then
    echo "error: NyRT artifact missing: ${artifact}" >&2
    echo "hint: cargo build -p nyash_kernel --release" >&2
    return 1
  fi
  local newer_source
  if newer_source="$(nyrt_contract_newer_source "${root_dir}" "${artifact}")"; then
    echo "error: NyRT is stale" >&2
    echo "artifact=${artifact}" >&2
    echo "newer_source=${newer_source}" >&2
    echo "hint: cargo build -p nyash_kernel --release" >&2
    return 1
  fi
  return 0
}
