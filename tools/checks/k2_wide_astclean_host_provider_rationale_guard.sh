#!/usr/bin/env bash
set -euo pipefail

file='src/host_providers/llvm_codegen.rs'
card='docs/development/current/main/phases/phase-293x/293x-305-ASTCLEAN-012-HOST-PROVIDER-COMPARE-BRIDGE-RATIONALE-GUARD.md'
ssot='docs/development/current/main/design/ast-cleanup-before-localtype-ssot.md'

if ! grep -q 'ASTCLEAN-012 host provider compare bridge dead_code rationale guard' "$ssot"; then
  echo '[astclean-host-provider] missing SSOT row' >&2
  exit 1
fi

if ! grep -q '293x-305 ASTCLEAN-012 host provider compare bridge dead_code rationale guard' "$card"; then
  echo '[astclean-host-provider] missing phase card' >&2
  exit 1
fi

if rg -n '#\[allow\(dead_code\)\]' "$file" | grep -v 'Phase 291x-126' >/dev/null; then
  echo '[astclean-host-provider] bare host provider dead_code allowance remains' >&2
  rg -n '#\[allow\(dead_code\)\]' "$file" | grep -v 'Phase 291x-126' >&2
  exit 1
fi

for module in ll_emit_compare_driver ll_emit_compare_source route transport_io transport_paths; do
  if ! rg -n "mod ${module};" "$file" >/dev/null; then
    echo "[astclean-host-provider] expected staged module missing: $module" >&2
    exit 1
  fi
done

count=$(rg -n '#\[allow\(dead_code\)\]' src | wc -l | tr -d ' ')
if [ "$count" -gt 174 ]; then
  echo "[astclean-host-provider] expected source allowance count <= 174, got $count" >&2
  exit 1
fi

echo "[astclean-host-provider] OK source_count=$count"
