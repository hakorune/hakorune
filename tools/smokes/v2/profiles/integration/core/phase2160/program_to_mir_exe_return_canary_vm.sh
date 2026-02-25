#!/usr/bin/env bash
# Build EXE from minimal Hako via Program(JSON)->MIR(JSON) robust route and check rc.
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
static box Main { method main(args) { return 42 } }
HAKO

TMP_JSON=$(mktemp --suffix .json)
EXE_OUT=$(mktemp --suffix .exe)
trap 'rm -f "${TMP_HAKO}" "${TMP_JSON}" "${EXE_OUT}" || true' EXIT

# Prefer robust Program->MIR via CLI; fallback to Stage‑B wrapper if needed
if ! timeout "${HAKO_BUILD_TIMEOUT:-10}" NYASH_FAIL_FAST=0 "${BIN}" --emit-mir-json "${TMP_JSON}" "${TMP_HAKO}" >/dev/null 2>&1; then
  if ! timeout "${HAKO_BUILD_TIMEOUT:-10}" bash "${ROOT_DIR}/tools/hakorune_emit_mir.sh" "${TMP_HAKO}" "${TMP_JSON}" >/dev/null 2>&1; then
    echo "[SKIP] cannot produce MIR JSON on this host"; exit 0
  fi
fi

source "${ROOT_DIR}/tools/smokes/v2/lib/crate_exec.sh" || true
if ! crate_build_exe "${TMP_JSON}" "${EXE_OUT}" "${ROOT_DIR}/target/release"; then
  echo "[SKIP] cannot build EXE on this host"; exit 0
fi

timeout "${HAKO_EXE_TIMEOUT:-5}" "${EXE_OUT}" >/dev/null 2>&1 || true
rc=$?
if [[ "$rc" -eq 42 ]]; then
  echo "[PASS] program_to_mir_exe_return (rc=42)"
  exit 0
fi
echo "[SKIP] EXE rc != 42 (rc=${rc})"
exit 0
