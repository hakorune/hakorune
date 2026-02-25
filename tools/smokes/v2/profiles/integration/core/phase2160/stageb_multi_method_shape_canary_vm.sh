#!/usr/bin/env bash
# Verify Stage‑B extracts main() body correctly when multiple methods exist in box Main.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT_DIR="$ROOT_GIT"
else
  ROOT_DIR="$(cd "$SCRIPT_DIR/../../../../../../.." && pwd)"
fi
BIN="${ROOT_DIR}/target/release/hakorune"
if [[ ! -x "${BIN}" ]]; then echo "[SKIP] hakorune not built"; exit 0; fi

# Code with helper() preceding main()
SRC='static box Main { static method helper() { return 9 } method main(args) { /* pre */ local x = 1; /* mid\nblock */ return x /* post */ } }'
OUT=$(NYASH_JSON_ONLY=1 NYASH_DISABLE_NY_COMPILER=1 HAKO_DISABLE_NY_COMPILER=1 \
      NYASH_FEATURES=stage3 NYASH_PARSER_ALLOW_SEMICOLON=1 \
      "${BIN}" --backend vm "${ROOT_DIR}/lang/src/compiler/entry/compiler_stageb.hako" -- --source "${SRC}" 2>/dev/null | awk '/^{/,/^}$/')

if [[ -z "${OUT}" ]]; then echo "[FAIL] empty Program(JSON)"; exit 1; fi
echo "${OUT}" | grep -q '"kind"\s*:\s*"Program"' || { echo "[FAIL] missing Program kind"; exit 1; }
echo "${OUT}" | grep -q '"Local"' || { echo "[FAIL] missing Local stmt (x)"; exit 1; }
echo "${OUT}" | grep -q '"Return"' || { echo "[FAIL] missing Return stmt"; exit 1; }
echo "${OUT}" | grep -q '"name"\s*:\s*"x"' || { echo "[FAIL] missing var name x"; exit 1; }
echo "[PASS] stageb_multi_method_shape"
exit 0

