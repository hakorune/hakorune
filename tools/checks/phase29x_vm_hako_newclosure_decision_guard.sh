#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC_X50="$ROOT_DIR/docs/development/current/main/phases/phase-29x/29x-77-newclosure-contract-lock-ssot.md"
DOC_X57="$ROOT_DIR/docs/development/current/main/phases/phase-29x/29x-83-vm-hako-newclosure-runtime-lane-decision-refresh-ssot.md"
ALLOWLIST="$ROOT_DIR/tools/checks/phase29x_vm_hako_s6_vocab_allowlist.txt"
GATE="$ROOT_DIR/tools/smokes/v2/profiles/integration/apps/phase29x_vm_hako_newclosure_decision_refresh_vm.sh"

cd "$ROOT_DIR"

echo "[vm-hako-newclosure-decision-guard] checking X57 decision lock"

if ! command -v rg >/dev/null 2>&1; then
  echo "[vm-hako-newclosure-decision-guard] ERROR: rg is required" >&2
  exit 2
fi

if [[ ! -f "$DOC_X50" ]]; then
  echo "[vm-hako-newclosure-decision-guard] ERROR: missing X50 SSOT: $DOC_X50"
  exit 1
fi
if [[ ! -f "$DOC_X57" ]]; then
  echo "[vm-hako-newclosure-decision-guard] ERROR: missing X57 SSOT: $DOC_X57"
  exit 1
fi
if [[ ! -f "$ALLOWLIST" ]]; then
  echo "[vm-hako-newclosure-decision-guard] ERROR: missing S6 allowlist: $ALLOWLIST"
  exit 1
fi
if [[ ! -x "$GATE" ]]; then
  echo "[vm-hako-newclosure-decision-guard] ERROR: gate missing or not executable: $GATE"
  exit 1
fi

if ! rg -q '^Decision: accepted$' "$DOC_X50"; then
  echo "[vm-hako-newclosure-decision-guard] ERROR: X50 decision drift (expected Decision: accepted)"
  exit 1
fi
if ! rg -q '^Decision: accepted$' "$DOC_X57"; then
  echo "[vm-hako-newclosure-decision-guard] ERROR: X57 decision drift (expected Decision: accepted)"
  exit 1
fi
if ! rg -q 'Decision owner: .*29x-77-newclosure-contract-lock-ssot.md' "$DOC_X57"; then
  echo "[vm-hako-newclosure-decision-guard] ERROR: X57 decision owner drift (expected 29x-77 owner)"
  exit 1
fi
if rg -q '^new_closure$' "$ALLOWLIST"; then
  echo "[vm-hako-newclosure-decision-guard] ERROR: allowlist drift (new_closure must stay non-promoted at X57)"
  exit 1
fi
if ! rg -q 'phase29x_vm_hako_s6_parity_gate_vm.sh' "$GATE"; then
  echo "[vm-hako-newclosure-decision-guard] ERROR: gate missing X56 parity step"
  exit 1
fi
if ! rg -q 'phase29x_vm_hako_newclosure_contract_vm.sh' "$GATE"; then
  echo "[vm-hako-newclosure-decision-guard] ERROR: gate missing NewClosure contract step"
  exit 1
fi

echo "[vm-hako-newclosure-decision-guard] ok"
