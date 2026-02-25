#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

echo "[vm-verifier-gate-guard] checking single-entry verifier ownership"

if ! command -v rg >/dev/null 2>&1; then
  echo "[vm-verifier-gate-guard] ERROR: rg is required" >&2
  exit 2
fi

verifier_hits="$(rg -n "MirVerifier::new\\(" src/runner/modes -S || true)"
if [[ -z "$verifier_hits" ]]; then
  echo "[vm-verifier-gate-guard] ERROR: MirVerifier callsite not found"
  exit 1
fi
verifier_disallowed="$(printf "%s\n" "$verifier_hits" | rg -v "src/runner/modes/common_util/verifier_gate.rs" || true)"
if [[ -n "$verifier_disallowed" ]]; then
  echo "[vm-verifier-gate-guard] ERROR: disallowed MirVerifier callsite detected"
  echo "$verifier_disallowed"
  exit 1
fi

for file in \
  src/runner/modes/vm.rs \
  src/runner/modes/vm_fallback.rs \
  src/runner/modes/vm_hako.rs; do
  if ! rg -q "enforce_vm_verify_gate_or_exit\\(" "$file"; then
    echo "[vm-verifier-gate-guard] ERROR: verifier gate hook missing in $file"
    exit 1
  fi
done

env_hits="$(rg -n "NYASH_VM_VERIFY_MIR" src/runner/modes/vm.rs src/runner/modes/vm_fallback.rs src/runner/modes/vm_hako.rs -S || true)"
if [[ -n "$env_hits" ]]; then
  echo "[vm-verifier-gate-guard] ERROR: direct NYASH_VM_VERIFY_MIR usage remains in VM mode files"
  echo "$env_hits"
  exit 1
fi

echo "[vm-verifier-gate-guard] ok"
