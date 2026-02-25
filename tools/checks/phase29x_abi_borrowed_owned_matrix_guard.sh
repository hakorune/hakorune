#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
CASES="$ROOT_DIR/tools/checks/phase29x_abi_borrowed_owned_matrix_cases.txt"
TESTS="$ROOT_DIR/crates/nyash_kernel/src/tests.rs"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-29x/29x-86-abi-borrowed-owned-conformance-extension-ssot.md"
GATE="$ROOT_DIR/tools/smokes/v2/profiles/integration/apps/phase29x_abi_borrowed_owned_conformance_vm.sh"

cd "$ROOT_DIR"

echo "[abi-borrowed-owned-matrix-guard] checking X59 borrowed/owned matrix wiring"

if ! command -v rg >/dev/null 2>&1; then
  echo "[abi-borrowed-owned-matrix-guard] ERROR: rg is required" >&2
  exit 2
fi

for required in "$CASES" "$TESTS" "$DOC"; do
  if [[ ! -f "$required" ]]; then
    echo "[abi-borrowed-owned-matrix-guard] ERROR: required file missing: $required"
    exit 1
  fi
done

if [[ ! -x "$GATE" ]]; then
  echo "[abi-borrowed-owned-matrix-guard] ERROR: gate missing or not executable: $GATE"
  exit 1
fi

if ! rg -q '^Decision: accepted$' "$DOC"; then
  echo "[abi-borrowed-owned-matrix-guard] ERROR: X59 SSOT decision drift (expected Decision: accepted)"
  exit 1
fi

while IFS='|' read -r case_id test_name; do
  if [[ -z "${case_id}" || "${case_id}" =~ ^# ]]; then
    continue
  fi
  if [[ -z "${test_name}" ]]; then
    echo "[abi-borrowed-owned-matrix-guard] ERROR: malformed matrix row (missing test name): $case_id"
    exit 1
  fi
  if ! rg -q "fn[[:space:]]+${test_name}[[:space:]]*\\(" "$TESTS"; then
    echo "[abi-borrowed-owned-matrix-guard] ERROR: missing nyash_kernel test for case ${case_id}: ${test_name}"
    exit 1
  fi
  if ! rg -q "\b${case_id}\b" "$DOC"; then
    echo "[abi-borrowed-owned-matrix-guard] ERROR: SSOT missing case id: ${case_id}"
    exit 1
  fi
  if ! rg -q "\b${test_name}\b" "$DOC"; then
    echo "[abi-borrowed-owned-matrix-guard] ERROR: SSOT missing mapped test: ${test_name}"
    exit 1
  fi
done <"$CASES"

if ! rg -q 'phase29x_core_cabi_delegation_guard_vm.sh' "$GATE"; then
  echo "[abi-borrowed-owned-matrix-guard] ERROR: gate missing X51 precondition step"
  exit 1
fi
if ! rg -q 'handle_abi_borrowed_owned_' "$GATE"; then
  echo "[abi-borrowed-owned-matrix-guard] ERROR: gate missing borrowed/owned matrix cargo filter"
  exit 1
fi
if ! rg -q 'phase29x_abi_borrowed_owned_matrix_cases.txt' "$GATE"; then
  echo "[abi-borrowed-owned-matrix-guard] ERROR: gate missing matrix cases source binding"
  exit 1
fi

echo "[abi-borrowed-owned-matrix-guard] ok"
