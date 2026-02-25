#!/usr/bin/env bash
# Phase 29x X39: Rust lane compatibility gate (opt-in only)
#
# Usage:
#   PHASE29X_ALLOW_RUST_LANE=1 tools/compat/phase29x_rust_lane_gate.sh [--dry-run]

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
DRY_RUN=0

if [[ "${1:-}" == "--dry-run" ]]; then
  DRY_RUN=1
fi

if [[ "${PHASE29X_ALLOW_RUST_LANE:-0}" != "1" ]]; then
  echo "[compat/optin-required] set PHASE29X_ALLOW_RUST_LANE=1 to run Rust lane gate" >&2
  exit 1
fi

if [[ "$DRY_RUN" == "1" ]]; then
  echo "[compat/optin] rust-lane gate dry-run (no heavy commands executed)"
  exit 0
fi

echo "[compat/optin] running rust-lane compatibility gate"
bash "$ROOT/tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh" --only bq
"$ROOT/tools/selfhost/run.sh" --gate --planner-required 1 --max-cases 5 --jobs 4

echo "[compat/optin] rust-lane compatibility gate PASS"
