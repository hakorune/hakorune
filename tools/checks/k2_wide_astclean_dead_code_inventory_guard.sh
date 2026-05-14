#!/usr/bin/env bash
set -euo pipefail

card='docs/development/current/main/phases/phase-293x/293x-297-ASTCLEAN-004-DEAD-CODE-ALLOWANCE-INVENTORY.md'
ssot='docs/development/current/main/design/ast-cleanup-before-localtype-ssot.md'

if ! grep -q 'ASTCLEAN-004 dead-code allowance inventory' "$ssot"; then
  echo '[astclean-dead-code-inventory] missing SSOT row' >&2
  exit 1
fi

if ! grep -q 'Source baseline count: 210' "$card"; then
  echo '[astclean-dead-code-inventory] missing baseline count' >&2
  exit 1
fi

count=$(rg -n '#\[allow\(dead_code\)\]' src | wc -l | tr -d ' ')
if [ "$count" -gt 210 ]; then
  echo "[astclean-dead-code-inventory] source dead_code allowance count grew: $count > 210" >&2
  exit 1
fi

for anchor in 'src/mir' 'src/runner' 'src/backend' 'ASTCLEAN-005 MIR dead_code allowance prune pilot'; do
  if ! grep -q "$anchor" "$card"; then
    echo "[astclean-dead-code-inventory] missing inventory anchor: $anchor" >&2
    exit 1
  fi
done

echo "[astclean-dead-code-inventory] OK source_count=$count"
