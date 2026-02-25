#!/usr/bin/env bash
set -euo pipefail

BASE_DIR="$(cd "$(dirname "$0")" && pwd)"

run() { local f="$1"; [[ -x "$f" ]] || chmod +x "$f"; bash "$f"; }

# 1) Stage‑B Program(JSON) shape（軽量・常時オン推奨）
run "$BASE_DIR/profiles/quick/core/phase2160/stageb_program_json_shape_canary_vm.sh" || true
run "$BASE_DIR/profiles/quick/core/phase2160/stageb_program_json_method_shape_canary_vm.sh" || true
run "$BASE_DIR/profiles/quick/core/phase2160/stageb_multi_method_shape_canary_vm.sh" || true

# 2) loop_scan canaries（!=' + else Break/Continue）
run "$BASE_DIR/profiles/quick/core/phase2160/loop_scan_ne_else_break_canary_vm.sh" || true
run "$BASE_DIR/profiles/quick/core/phase2160/loop_scan_ne_else_continue_canary_vm.sh" || true

echo "[smokes/quick] done"
