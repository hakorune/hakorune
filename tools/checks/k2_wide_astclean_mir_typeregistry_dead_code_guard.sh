#!/usr/bin/env bash
set -euo pipefail

file='src/mir/builder/type_registry.rs'
card='docs/development/current/main/phases/phase-293x/293x-298-ASTCLEAN-005-MIR-TYPEREGISTRY-DEAD-CODE-ALLOW-PRUNE.md'
ssot='docs/development/current/main/design/ast-cleanup-before-localtype-ssot.md'

if ! grep -q 'ASTCLEAN-005 MIR TypeRegistry dead_code allow prune' "$ssot"; then
  echo '[astclean-typeregistry-dead-code] missing SSOT row' >&2
  exit 1
fi

if ! grep -q '293x-298 ASTCLEAN-005 MIR TypeRegistry dead_code allow prune' "$card"; then
  echo '[astclean-typeregistry-dead-code] missing phase card' >&2
  exit 1
fi

if rg -n '#\[allow\(dead_code\)\]' "$file" | grep -v 'ASTCLEAN-005' >/dev/null; then
  echo '[astclean-typeregistry-dead-code] bare dead_code allowance remains in TypeRegistry' >&2
  rg -n '#\[allow\(dead_code\)\]' "$file" | grep -v 'ASTCLEAN-005' >&2
  exit 1
fi

count=$(rg -n '#\[allow\(dead_code\)\]' src | wc -l | tr -d ' ')
if [ "$count" -gt 206 ]; then
  echo "[astclean-typeregistry-dead-code] expected source allowance count <= 206, got $count" >&2
  exit 1
fi

cargo test -q parser_loopclean_while_stage3_normalizes_to_loop_ast

echo "[astclean-typeregistry-dead-code] OK source_count=$count"
