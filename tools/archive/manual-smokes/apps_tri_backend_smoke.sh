#!/usr/bin/env bash
set -euo pipefail

# Diagnostic / compat smoke:
# - compares interpreter / compat / JIT routes
# - not a mainline owner route

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
BIN="$ROOT_DIR/target/release/nyash"

default_apps=(
  "$ROOT_DIR/apps/tests/mir-branch-ret/main.hako"
  "$ROOT_DIR/apps/tests/semantics-unified/main.hako"
  "$ROOT_DIR/apps/tests/async-await-min/main.hako"
  "$ROOT_DIR/apps/tests/gc-sync-stress/main.hako"
)

if [ $# -gt 0 ]; then
  apps=("$@")
else
  apps=("${default_apps[@]}")
fi

echo "[build] nyash (release, cranelift-jit)"
cargo build --release --features cranelift-jit >/dev/null

run_case() {
  local app="$1"
  echo "\n=== $app ==="
  echo "[script] interpreter"
  timeout 15s "$BIN" "$app" >/tmp/ny_script.out || true
  echo "[compat route]"
  timeout 15s "$BIN" --backend vm "$app" >/tmp/ny_compat.out || true
  echo "[jit compat] compat route + jit-exec"
  timeout 15s "$BIN" --backend vm --jit-exec --jit-hostcall "$app" >/tmp/ny_jit.out || true
  # Summarize
  for mode in script compat jit; do
    local f="/tmp/ny_${mode}.out"
    local rc_line
    rc_line=$(rg -n "^Result: " -N "$f" || true)
    echo "  [$mode] ${rc_line:-no Result line}"
  done
}

for a in "${apps[@]}"; do
  if [ -f "$a" ]; then run_case "$a"; else echo "skip (not found): $a"; fi
done

echo "\n[done] tri-backend smoke complete"
