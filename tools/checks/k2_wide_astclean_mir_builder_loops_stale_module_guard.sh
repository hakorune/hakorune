#!/usr/bin/env bash
set -euo pipefail

card='docs/development/current/main/phases/phase-293x/293x-306-ASTCLEAN-013-MIR-BUILDER-LOOPS-STALE-MODULE-REMOVAL.md'
ssot='docs/development/current/main/design/ast-cleanup-before-localtype-ssot.md'

if ! grep -q 'ASTCLEAN-013 MIR builder utility dead_code allowance prune' "$ssot"; then
  echo '[astclean-mir-builder-loops] missing SSOT row' >&2
  exit 1
fi

if ! grep -q '293x-306 ASTCLEAN-013 MIR builder loops stale module removal' "$card"; then
  echo '[astclean-mir-builder-loops] missing phase card' >&2
  exit 1
fi

if [ -e src/mir/builder/loops.rs ]; then
  echo '[astclean-mir-builder-loops] stale builder loops module returned' >&2
  exit 1
fi

if rg -n '^pub\(crate\) mod loops;' src/mir/builder.rs >/dev/null; then
  echo '[astclean-mir-builder-loops] stale builder loops module declaration remains' >&2
  exit 1
fi

if rg -n 'builder::loops' src/mir src/tests >/dev/null; then
  echo '[astclean-mir-builder-loops] stale builder::loops reference remains' >&2
  rg -n 'builder::loops' src/mir src/tests >&2
  exit 1
fi

count=$(rg -n '#\[allow\(dead_code\)\]' src | wc -l | tr -d ' ')
if [ "$count" -gt 167 ]; then
  echo "[astclean-mir-builder-loops] expected source allowance count <= 167, got $count" >&2
  exit 1
fi

cargo test -q parser_loopclean_while_stage3_normalizes_to_loop_ast

echo "[astclean-mir-builder-loops] OK source_count=$count"
