#!/usr/bin/env bash
set -euo pipefail

card='docs/development/current/main/phases/phase-293x/293x-308-ASTCLEAN-015-MIR-BUILDER-UTILITY-SHELF-PRUNE.md'
ssot='docs/development/current/main/design/ast-cleanup-before-localtype-ssot.md'

if ! grep -q 'ASTCLEAN-015 MIR builder utility shelf prune' "$ssot"; then
  echo '[astclean-mir-builder-utility] missing SSOT row' >&2
  exit 1
fi

if ! grep -q '293x-308 ASTCLEAN-015 MIR builder utility shelf prune' "$card"; then
  echo '[astclean-mir-builder-utility] missing phase card' >&2
  exit 1
fi

if [ -e src/mir/builder/utils/type_ops.rs ]; then
  echo '[astclean-mir-builder-utility] stale type_ops helper module returned' >&2
  exit 1
fi

if rg -n 'mod type_ops|emit_type_check|emit_cast|materialize_local|ensure_slotified_for_use' src/mir/builder >/dev/null; then
  echo '[astclean-mir-builder-utility] stale utility helper symbol returned' >&2
  rg -n 'mod type_ops|emit_type_check|emit_cast|materialize_local|ensure_slotified_for_use' src/mir/builder >&2
  exit 1
fi

if rg -n 'try_global_fallback_handlers|materialize_receiver_in_callee' src/mir/builder/calls/emit.rs >/dev/null; then
  echo '[astclean-mir-builder-utility] stale calls/emit wrapper returned' >&2
  rg -n 'try_global_fallback_handlers|materialize_receiver_in_callee' src/mir/builder/calls/emit.rs >&2
  exit 1
fi

if rg -n '#\[allow\(dead_code\)\]' src/mir/builder/utils/weak_ref.rs src/mir/builder/calls/emit.rs src/mir/builder/utils/mod.rs >/dev/null; then
  echo '[astclean-mir-builder-utility] target utility dead_code allowance remains' >&2
  rg -n '#\[allow\(dead_code\)\]' src/mir/builder/utils/weak_ref.rs src/mir/builder/calls/emit.rs src/mir/builder/utils/mod.rs >&2
  exit 1
fi

if rg -n '#\[allow\(dead_code\)\]' src/mir/builder/utils/pinning.rs src/mir/builder/schedule/block.rs | grep -v 'ASTCLEAN-015' >/dev/null; then
  echo '[astclean-mir-builder-utility] unreasoned schedule/pinning dead_code allowance remains' >&2
  rg -n '#\[allow\(dead_code\)\]' src/mir/builder/utils/pinning.rs src/mir/builder/schedule/block.rs | grep -v 'ASTCLEAN-015' >&2
  exit 1
fi

for live in emit_weak_new emit_weak_load emit_barrier_read emit_barrier_write pin_to_slot insert_copy_after_phis ensure_after_phis_copy emit_before_call_copy; do
  if ! rg -n "\b${live}\b" src/mir/builder >/dev/null; then
    echo "[astclean-mir-builder-utility] expected live helper missing: $live" >&2
    exit 1
  fi
done

count=$(rg -n '#\[allow\(dead_code\)\]' src | wc -l | tr -d ' ')
if [ "$count" -gt 146 ]; then
  echo "[astclean-mir-builder-utility] expected source allowance count <= 146, got $count" >&2
  exit 1
fi

cargo test -q parser_loopclean_while_stage3_normalizes_to_loop_ast

echo "[astclean-mir-builder-utility] OK source_count=$count"
