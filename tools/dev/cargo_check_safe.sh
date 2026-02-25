#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SRC="${ROOT_DIR}/tools/dev/exdev_rename_copy_fallback.c"
BUILD_DIR="${ROOT_DIR}/tools/tmp/exdev"
LIB="${BUILD_DIR}/librename_copy_fallback.so"

mkdir -p "${BUILD_DIR}"

if [[ ! -f "${LIB}" || "${SRC}" -nt "${LIB}" ]]; then
  if ! command -v gcc >/dev/null 2>&1; then
    echo "[cargo_check_safe] gcc is required to build ${LIB}" >&2
    exit 127
  fi
  gcc -shared -fPIC -O2 -o "${LIB}" "${SRC}" -ldl
fi

export LD_PRELOAD="${LIB}${LD_PRELOAD:+:${LD_PRELOAD}}"
export TMPDIR="${TMPDIR:-${ROOT_DIR}/target/tmp}"
mkdir -p "${TMPDIR}"

cd "${ROOT_DIR}"

if [[ $# -eq 0 ]]; then
  set -- --bin hakorune
fi

exec cargo check "$@"
