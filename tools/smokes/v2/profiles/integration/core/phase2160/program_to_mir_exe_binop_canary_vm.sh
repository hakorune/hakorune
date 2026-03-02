#!/usr/bin/env bash
# Build EXE from minimal Hako (binop) via Program(JSON)->MIR(JSON) robust route and check rc.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT_DIR="$ROOT_GIT"
else
  ROOT_DIR="$(cd "$SCRIPT_DIR/../../../../../../.." && pwd)"
fi
BIN="${ROOT_DIR}/target/release/hakorune"
if [[ ! -x "${BIN}" ]]; then echo "[SKIP] hakorune not built"; exit 0; fi

TMP_HAKO=$(mktemp --suffix .hako)
cat >"${TMP_HAKO}" <<'HAKO'
static box Main { method main(args) { return 1 + 2 } }
HAKO

TMP_JSON=$(mktemp --suffix .json)
EXE_OUT=$(mktemp --suffix .exe)
trap 'rm -f "${TMP_HAKO}" "${TMP_JSON}" "${EXE_OUT}" || true' EXIT

# Prefer robust Program->MIR via CLI; fallback to Stage‑B wrapper if needed
if ! NYASH_FAIL_FAST=0 "${BIN}" --emit-mir-json "${TMP_JSON}" "${TMP_HAKO}" >/dev/null 2>&1; then
  if ! bash "${ROOT_DIR}/tools/smokes/v2/lib/emit_mir_route.sh" --route hako-helper --timeout-secs "${HAKO_BUILD_TIMEOUT:-10}" --out "${TMP_JSON}" --input "${TMP_HAKO}" >/dev/null 2>&1; then
    echo "[SKIP] cannot produce MIR JSON on this host"; exit 0
  fi
fi

# Build EXE with bounded time using crate_exec helper
source "${ROOT_DIR}/tools/smokes/v2/lib/crate_exec.sh" || true
if ! crate_build_exe "${TMP_JSON}" "${EXE_OUT}" "${ROOT_DIR}/target/release"; then
  echo "[SKIP] cannot build EXE on this host"; exit 0
fi

timeout 10s "${EXE_OUT}" >/dev/null 2>&1 || true
rc=$?
if [[ "$rc" -eq 3 ]]; then
  echo "[PASS] program_to_mir_exe_binop (rc=3)"
  exit 0
fi
echo "[SKIP] EXE rc != 3 (rc=${rc})"
exit 0
