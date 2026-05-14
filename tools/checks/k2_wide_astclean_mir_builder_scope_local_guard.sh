#!/usr/bin/env bash
set -euo pipefail

card='docs/development/current/main/phases/phase-293x/293x-307-ASTCLEAN-014-MIR-BUILDER-SCOPE-LOCAL-UTILITY-DEAD-CODE-PRUNE.md'
ssot='docs/development/current/main/design/ast-cleanup-before-localtype-ssot.md'

if ! grep -q 'ASTCLEAN-014 MIR builder scope/local utility dead_code prune' "$ssot"; then
  echo '[astclean-mir-scope-local] missing SSOT row' >&2
  exit 1
fi

if ! grep -q '293x-307 ASTCLEAN-014 MIR builder scope/local utility dead_code prune' "$card"; then
  echo '[astclean-mir-scope-local] missing phase card' >&2
  exit 1
fi

for removed in push_loop_header pop_loop_header current_loop_header push_loop_exit pop_loop_exit local_cmp_operand; do
  if rg -n "\b${removed}\b" src/mir/builder/scope_context.rs src/mir/builder/utils/local_ssa.rs >/dev/null; then
    echo "[astclean-mir-scope-local] stale helper returned: $removed" >&2
    exit 1
  fi
done

if rg -n '#\[allow\(dead_code\)\]' src/mir/builder/scope_context.rs src/mir/builder/utils/local_ssa.rs >/dev/null; then
  echo '[astclean-mir-scope-local] dead_code allowance remains in scope/local targets' >&2
  rg -n '#\[allow\(dead_code\)\]' src/mir/builder/scope_context.rs src/mir/builder/utils/local_ssa.rs >&2
  exit 1
fi

for live in local_recv local_arg local_field_base local_cond local_ssa_ensure; do
  if ! rg -n "\b${live}\b" src/mir/builder/utils/local_ssa.rs >/dev/null; then
    echo "[astclean-mir-scope-local] expected LocalSSA wrapper missing: $live" >&2
    exit 1
  fi
done

count=$(rg -n '#\[allow\(dead_code\)\]' src | wc -l | tr -d ' ')
if [ "$count" -gt 156 ]; then
  echo "[astclean-mir-scope-local] expected source allowance count <= 156, got $count" >&2
  exit 1
fi

cargo test -q parser_loopclean_while_stage3_normalizes_to_loop_ast

echo "[astclean-mir-scope-local] OK source_count=$count"
