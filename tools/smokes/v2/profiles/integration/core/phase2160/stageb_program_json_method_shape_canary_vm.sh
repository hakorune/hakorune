#!/usr/bin/env bash
# Verify Stage‑B emits Program(JSON v0) with Method(length) for a simple snippet.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT_DIR="$ROOT_GIT"
else
  ROOT_DIR="$(cd "$SCRIPT_DIR/../../../../../../.." && pwd)"
fi
BIN="${ROOT_DIR}/target/release/hakorune"
if [[ ! -x "${BIN}" ]]; then echo "[SKIP] hakorune not built"; exit 0; fi

SRC='static box Main { method main(args) { local s = new StringBox("x"); return s.length() } }'
OUT=$(NYASH_JSON_ONLY=1 NYASH_DISABLE_NY_COMPILER=1 HAKO_DISABLE_NY_COMPILER=1 \
      NYASH_FEATURES=stage3 NYASH_PARSER_ALLOW_SEMICOLON=1 \
      "${BIN}" --backend vm "${ROOT_DIR}/lang/src/compiler/entry/compiler_stageb.hako" -- --source "${SRC}" 2>/dev/null | awk '/^{/,/^}$/')

if [[ -z "${OUT}" ]]; then echo "[FAIL] empty Program(JSON)"; exit 1; fi
echo "${OUT}" | grep -q '"Method"' || { echo "[FAIL] missing Method expr"; exit 1; }
echo "${OUT}" | grep -q '"method"\s*:\s*"length"' || { echo "[FAIL] missing method:length"; exit 1; }
echo "[PASS] stageb_program_json_method_shape"
exit 0
