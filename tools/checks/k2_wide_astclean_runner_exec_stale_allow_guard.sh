#!/usr/bin/env bash
set -euo pipefail

file='src/runner/modes/common_util/exec.rs'
card='docs/development/current/main/phases/phase-293x/293x-304-ASTCLEAN-011-RUNNER-EXEC-STALE-DEAD-CODE-ALLOWANCE-REMOVAL.md'
ssot='docs/development/current/main/design/ast-cleanup-before-localtype-ssot.md'

if ! grep -q 'ASTCLEAN-011 runner exec stale dead_code allowance removal' "$ssot"; then
  echo '[astclean-runner-exec] missing SSOT row' >&2
  exit 1
fi

if ! grep -q '293x-304 ASTCLEAN-011 runner exec stale dead_code allowance removal' "$card"; then
  echo '[astclean-runner-exec] missing phase card' >&2
  exit 1
fi

for symbol in ny_llvmc_emit_exe_lib ny_llvmc_emit_obj_lib ny_llvmc_emit_exe_bin run_executable; do
  if ! rg -n "pub fn ${symbol}\(" "$file" >/dev/null; then
    echo "[astclean-runner-exec] expected runner API missing: $symbol" >&2
    exit 1
  fi
done

if rg -n '#\[allow\(dead_code\)\]' "$file" >/dev/null; then
  echo '[astclean-runner-exec] stale runner exec dead_code allowance remains' >&2
  rg -n '#\[allow\(dead_code\)\]' "$file" >&2
  exit 1
fi

count=$(rg -n '#\[allow\(dead_code\)\]' src | wc -l | tr -d ' ')
if [ "$count" -gt 174 ]; then
  echo "[astclean-runner-exec] expected source allowance count <= 174, got $count" >&2
  exit 1
fi

cargo test -q parser_loopclean_while_stage3_normalizes_to_loop_ast

echo "[astclean-runner-exec] OK source_count=$count"
