#!/usr/bin/env bash
set -euo pipefail

BASE_DIR="$(cd "$(dirname "$0")" && pwd)"

run() { local f="$1"; [[ -x "$f" ]] || chmod +x "$f"; bash "$f"; }

# 1) Stage‑B Program(JSON) shape canaries were archived in phase-29cv P35.
#    The stronger live contract keeper is the integration-side
#    phase29bq_hako_program_json_contract_pin_vm.sh proof.

# 2) loop_scan canaries（!=' + else Break/Continue）
run "$BASE_DIR/profiles/quick/core/phase2160/loop_scan_ne_else_break_canary_vm.sh" || true
run "$BASE_DIR/profiles/quick/core/phase2160/loop_scan_ne_else_continue_canary_vm.sh" || true

echo "[smokes/quick] done"
