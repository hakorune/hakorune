#!/usr/bin/env bash
set -euo pipefail

file='src/mir/builder/call_resolution.rs'
card='docs/development/current/main/phases/phase-293x/293x-309-ASTCLEAN-016-MIR-BUILDER-CALL-RESOLUTION-DUPLICATE-HELPER-PRUNE.md'
ssot='docs/development/current/main/design/ast-cleanup-before-localtype-ssot.md'

if ! grep -q 'ASTCLEAN-016 MIR builder call resolution duplicate helper prune' "$ssot"; then
  echo '[astclean-call-resolution] missing SSOT row' >&2
  exit 1
fi

if ! grep -q '293x-309 ASTCLEAN-016 MIR builder call resolution duplicate helper prune' "$card"; then
  echo '[astclean-call-resolution] missing phase card' >&2
  exit 1
fi

if rg -n 'is_commonly_shadowed_method|generate_self_recursion_warning|test_shadowed_method_detection|test_warning_generation' "$file" >/dev/null; then
  echo '[astclean-call-resolution] duplicate helper/test returned' >&2
  rg -n 'is_commonly_shadowed_method|generate_self_recursion_warning|test_shadowed_method_detection|test_warning_generation' "$file" >&2
  exit 1
fi

for live in is_builtin_function is_extern_function suggest_resolution; do
  if ! rg -n "\b${live}\b" "$file" >/dev/null; then
    echo "[astclean-call-resolution] expected live helper missing: $live" >&2
    exit 1
  fi
done

if rg -n '#\[allow\(dead_code\)\]' "$file" >/dev/null; then
  echo '[astclean-call-resolution] dead_code allowance remains in call_resolution.rs' >&2
  exit 1
fi

count=$(rg -n '#\[allow\(dead_code\)\]' src | wc -l | tr -d ' ')
if [ "$count" -gt 144 ]; then
  echo "[astclean-call-resolution] expected source allowance count <= 144, got $count" >&2
  exit 1
fi

cargo test -q parser_loopclean_while_stage3_normalizes_to_loop_ast

echo "[astclean-call-resolution] OK source_count=$count"
