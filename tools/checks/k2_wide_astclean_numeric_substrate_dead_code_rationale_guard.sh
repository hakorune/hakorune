#!/usr/bin/env bash
set -euo pipefail

file='src/mir/numeric_substrate.rs'
card='docs/development/current/main/phases/phase-293x/293x-299-ASTCLEAN-006-MIR-NUMERIC-SUBSTRATE-DEAD-CODE-RATIONALE-GUARD.md'
ssot='docs/development/current/main/design/ast-cleanup-before-localtype-ssot.md'

if ! grep -q 'ASTCLEAN-006 MIR numeric substrate dead_code rationale guard' "$ssot"; then
  echo '[astclean-numeric-substrate] missing SSOT row' >&2
  exit 1
fi

if ! grep -q '293x-299 ASTCLEAN-006 MIR numeric substrate dead_code rationale guard' "$card"; then
  echo '[astclean-numeric-substrate] missing phase card' >&2
  exit 1
fi

if rg -n '#\[allow\(dead_code\)\]' "$file" | grep -v '// .*row\|// 294x-\|// ASTCLEAN-' >/dev/null; then
  echo '[astclean-numeric-substrate] bare dead_code allowance remains in numeric substrate' >&2
  rg -n '#\[allow\(dead_code\)\]' "$file" | grep -v '// .*row\|// 294x-\|// ASTCLEAN-' >&2
  exit 1
fi

count=$(rg -n '#\[allow\(dead_code\)\]' src | wc -l | tr -d ' ')
if [ "$count" -gt 206 ]; then
  echo "[astclean-numeric-substrate] source allowance count grew: $count > 206" >&2
  exit 1
fi

echo "[astclean-numeric-substrate] OK source_count=$count"
