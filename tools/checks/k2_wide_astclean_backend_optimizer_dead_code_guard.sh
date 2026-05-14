#!/usr/bin/env bash
set -euo pipefail

card='docs/development/current/main/phases/phase-293x/293x-302-ASTCLEAN-009-BACKEND-OPTIMIZER-UTILITY-DEAD-CODE-PRUNE.md'
ssot='docs/development/current/main/design/ast-cleanup-before-localtype-ssot.md'
backend_targets=(
  src/backend/mir_interpreter/utils/error_helpers.rs
  src/backend/mir_interpreter/utils/conversion_helpers.rs
  src/backend/mir_interpreter/utils/arg_validation.rs
  src/backend/mir_interpreter/utils/destination_helpers.rs
  src/backend/mir_interpreter/utils/receiver_helpers.rs
)

if ! grep -q 'ASTCLEAN-009 backend/optimizer utility dead_code allowance prune' "$ssot"; then
  echo '[astclean-backend-optimizer] missing SSOT row' >&2
  exit 1
fi

if ! grep -q '293x-302 ASTCLEAN-009 backend/optimizer utility dead_code prune' "$card"; then
  echo '[astclean-backend-optimizer] missing phase card' >&2
  exit 1
fi

if [ -e src/mir/optimizer/diagnostics.rs ]; then
  echo '[astclean-backend-optimizer] stale optimizer diagnostics module remains' >&2
  exit 1
fi

if grep -q '^mod diagnostics;' src/mir/optimizer.rs; then
  echo '[astclean-backend-optimizer] stale optimizer diagnostics module declaration remains' >&2
  exit 1
fi

if rg -n '#\[allow\(dead_code\)\]' "${backend_targets[@]}" | grep -v 'ASTCLEAN-009' >/dev/null; then
  echo '[astclean-backend-optimizer] bare backend utility dead_code allowance remains' >&2
  rg -n '#\[allow\(dead_code\)\]' "${backend_targets[@]}" | grep -v 'ASTCLEAN-009' >&2
  exit 1
fi

if rg -n '#\[allow\(dead_code\)\]' src/mir/optimizer/core.rs | grep -v 'ASTCLEAN-009' >/dev/null; then
  echo '[astclean-backend-optimizer] bare optimizer core dead_code allowance remains' >&2
  rg -n '#\[allow\(dead_code\)\]' src/mir/optimizer/core.rs | grep -v 'ASTCLEAN-009' >&2
  exit 1
fi

for removed in 'receiver_type_error' 'arg_count_min' 'from_error' 'err_out_of_bounds' 'fn out_of_bounds'; do
  if rg -n "$removed" src/backend/mir_interpreter/utils/error_helpers.rs >/dev/null; then
    echo "[astclean-backend-optimizer] removed error helper returned: $removed" >&2
    exit 1
  fi
done

for removed in 'pub(crate) fn normalize_python_helper_calls' 'pub(crate) fn normalize_legacy_instructions' 'pub(crate) fn normalize_ref_field_access'; do
  if rg -n "$removed" src/mir/optimizer/core.rs >/dev/null; then
    echo "[astclean-backend-optimizer] stale optimizer wrapper returned: $removed" >&2
    exit 1
  fi
done

count=$(rg -n '#\[allow\(dead_code\)\]' src | wc -l | tr -d ' ')
if [ "$count" -gt 185 ]; then
  echo "[astclean-backend-optimizer] expected source allowance count <= 185, got $count" >&2
  exit 1
fi

cargo test -q parser_loopclean_while_stage3_normalizes_to_loop_ast

echo "[astclean-backend-optimizer] OK source_count=$count"
