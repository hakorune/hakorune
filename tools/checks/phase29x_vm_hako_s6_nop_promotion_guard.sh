#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
ALLOWLIST="$ROOT_DIR/tools/checks/phase29x_vm_hako_s6_vocab_allowlist.txt"
TARGET="$ROOT_DIR/src/runner/modes/vm_hako.rs"
GATE="$ROOT_DIR/tools/smokes/v2/profiles/integration/apps/phase29x_vm_hako_s6_nop_promotion_vm.sh"

cd "$ROOT_DIR"

echo "[vm-hako-s6-nop-promotion-guard] checking X58 nop vocabulary promotion wiring"

if ! command -v rg >/dev/null 2>&1; then
  echo "[vm-hako-s6-nop-promotion-guard] ERROR: rg is required" >&2
  exit 2
fi

if [[ ! -f "$ALLOWLIST" ]]; then
  echo "[vm-hako-s6-nop-promotion-guard] ERROR: allowlist missing: $ALLOWLIST"
  exit 1
fi
if [[ ! -f "$TARGET" ]]; then
  echo "[vm-hako-s6-nop-promotion-guard] ERROR: target missing: $TARGET"
  exit 1
fi
if [[ ! -x "$GATE" ]]; then
  echo "[vm-hako-s6-nop-promotion-guard] ERROR: gate missing or not executable: $GATE"
  exit 1
fi

if ! rg -q '^nop$' "$ALLOWLIST"; then
  echo "[vm-hako-s6-nop-promotion-guard] ERROR: allowlist missing promoted op: nop"
  exit 1
fi
if rg -q '^debug_log$' "$ALLOWLIST"; then
  echo "[vm-hako-s6-nop-promotion-guard] ERROR: allowlist drift: debug_log must stay non-promoted"
  exit 1
fi
if ! rg -q '"nop"\s*=>\s*\{\s*\}' "$TARGET"; then
  echo "[vm-hako-s6-nop-promotion-guard] ERROR: vm_hako subset checker missing nop acceptance arm"
  exit 1
fi
if ! rg -q 'phase29x_vm_hako_s6_parity_gate_vm.sh' "$GATE"; then
  echo "[vm-hako-s6-nop-promotion-guard] ERROR: gate missing X56 parity precondition"
  exit 1
fi
if ! rg -q 'phase29z_vm_hako_s3_nop_parity_vm.sh' "$GATE"; then
  echo "[vm-hako-s6-nop-promotion-guard] ERROR: gate missing nop parity fixture step"
  exit 1
fi
if ! rg -q 'run_with_vm_route_pin' "$GATE"; then
  echo "[vm-hako-s6-nop-promotion-guard] ERROR: gate missing route pin helper call"
  exit 1
fi

echo "[vm-hako-s6-nop-promotion-guard] ok"
