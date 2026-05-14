#!/usr/bin/env bash
set -euo pipefail

file='src/mir/builder/loops.rs'
card='docs/development/current/main/phases/phase-293x/293x-300-ASTCLEAN-007-MIR-LOOPS-DUPLICATE-DEAD-CODE-ALLOW-PRUNE.md'
ssot='docs/development/current/main/design/ast-cleanup-before-localtype-ssot.md'

if ! grep -q 'ASTCLEAN-007 MIR loops duplicate dead_code allow prune' "$ssot"; then
  echo '[astclean-mir-loops-duplicate] missing SSOT row' >&2
  exit 1
fi

if ! grep -q '293x-300 ASTCLEAN-007 MIR loops duplicate dead_code allow prune' "$card"; then
  echo '[astclean-mir-loops-duplicate] missing phase card' >&2
  exit 1
fi

if awk 'prev && $0 ~ /#\[allow\(dead_code\)\]/ { found=1 } { prev = ($0 ~ /#\[allow\(dead_code\)\]/) } END { exit found ? 0 : 1 }' "$file"; then
  echo '[astclean-mir-loops-duplicate] duplicate adjacent dead_code allowance remains' >&2
  exit 1
fi

count=$(rg -n '#\[allow\(dead_code\)\]' src | wc -l | tr -d ' ')
if [ "$count" -gt 203 ]; then
  echo "[astclean-mir-loops-duplicate] expected source allowance count <= 203, got $count" >&2
  exit 1
fi

cargo test -q parser_loopclean_while_stage3_normalizes_to_loop_ast

echo "[astclean-mir-loops-duplicate] OK source_count=$count"
