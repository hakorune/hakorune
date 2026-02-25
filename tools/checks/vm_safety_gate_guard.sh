#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

echo "[vm-safety-gate-guard] checking single-entry safety ownership"

if ! command -v rg >/dev/null 2>&1; then
  echo "[vm-safety-gate-guard] ERROR: rg is required" >&2
  exit 2
fi

for file in src/runner/modes/vm.rs src/runner/modes/vm_fallback.rs; do
  if ! rg -q "enforce_vm_source_safety_or_exit\\(" "$file"; then
    echo "[vm-safety-gate-guard] ERROR: source safety gate hook missing in $file"
    exit 1
  fi
done

for file in src/runner/modes/vm.rs src/runner/modes/vm_fallback.rs src/runner/modes/vm_hako.rs; do
  if ! rg -q "enforce_vm_lifecycle_safety_or_exit\\(" "$file"; then
    echo "[vm-safety-gate-guard] ERROR: lifecycle safety gate hook missing in $file"
    exit 1
  fi
done

direct_fail_fast_hits="$(rg -n "fail_fast_on_hako\\(|Hako-like source detected in Nyash VM path" src/runner/modes/vm.rs src/runner/modes/vm_fallback.rs -S || true)"
if [[ -n "$direct_fail_fast_hits" ]]; then
  echo "[vm-safety-gate-guard] ERROR: direct hako fail-fast logic remains in VM mode files"
  echo "$direct_fail_fast_hits"
  exit 1
fi

reason_hits="$(rg -n "release_strong-empty-values" src/runner/modes -S || true)"
if [[ -z "$reason_hits" ]]; then
  echo "[vm-safety-gate-guard] ERROR: lifecycle safety reason tag not found"
  exit 1
fi
reason_disallowed="$(printf "%s\n" "$reason_hits" | rg -v "src/runner/modes/common_util/safety_gate.rs" || true)"
if [[ -n "$reason_disallowed" ]]; then
  echo "[vm-safety-gate-guard] ERROR: lifecycle safety reason tag leaked outside safety_gate"
  echo "$reason_disallowed"
  exit 1
fi

echo "[vm-safety-gate-guard] ok"
