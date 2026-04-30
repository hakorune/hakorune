#!/usr/bin/env bash
# Verify Stage-B emits sane Program(JSON v0) for a minimal program (no include/IO).
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT_DIR="$ROOT_GIT"
else
  ROOT_DIR="$(cd "$SCRIPT_DIR/../../../../../../.." && pwd)"
fi
source "${ROOT_DIR}/tools/selfhost/lib/stageb_program_json_capture.sh"
BIN="${ROOT_DIR}/target/release/hakorune"
if [[ ! -x "${BIN}" ]]; then
  echo "[SKIP] hakorune not built"; exit 0
fi

SRC='static box Main { method main(args) { local i = 0 loop(i < 3) { i = i + 1 } return i } }'
if ! OUT=$(NYASH_JSON_ONLY=1 NYASH_DISABLE_NY_COMPILER=1 HAKO_DISABLE_NY_COMPILER=1 \
      NYASH_FEATURES=stage3 NYASH_PARSER_ALLOW_SEMICOLON=1 \
      "${BIN}" --backend vm "${ROOT_DIR}/lang/src/compiler/entry/compiler_stageb.hako" -- --source "${SRC}" 2>/dev/null \
      | stageb_program_json_extract_from_stdin); then
  echo "[FAIL] failed to extract Program(JSON)"
  exit 1
fi

if [[ -z "${OUT}" ]]; then echo "[FAIL] empty Program(JSON)"; exit 1; fi
echo "${OUT}" | grep -q '"kind"\s*:\s*"Program"' || { echo "[FAIL] missing Program kind"; exit 1; }
echo "${OUT}" | grep -q '"Local"' || { echo "[FAIL] missing Local stmt"; exit 1; }
echo "${OUT}" | grep -q '"Loop"' || { echo "[FAIL] missing Loop stmt"; exit 1; }
echo "${OUT}" | grep -q '"Return"' || { echo "[FAIL] missing Return stmt"; exit 1; }
echo "[PASS] stageb_program_json_shape"
exit 0
