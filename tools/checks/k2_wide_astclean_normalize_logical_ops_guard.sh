#!/usr/bin/env bash
set -euo pipefail

if ! grep -q 'ASTCLEAN-002 normalize logical ops helper' docs/development/current/main/design/ast-cleanup-before-localtype-ssot.md; then
  echo '[astclean-normalize-logical-ops] missing SSOT row' >&2
  exit 1
fi

if ! grep -q '293x-295 ASTCLEAN-002 normalize logical ops helper' docs/development/current/main/phases/phase-293x/293x-295-ASTCLEAN-002-NORMALIZE-LOGICAL-OPS-HELPER.md; then
  echo '[astclean-normalize-logical-ops] missing phase card' >&2
  exit 1
fi

count=$(grep -c '^fn normalize_logical_ops(src: &str) -> String {' src/parser/mod.rs || true)
if [ "$count" -ne 1 ]; then
  echo "[astclean-normalize-logical-ops] expected one module helper, found $count" >&2
  exit 1
fi

nested_count=$(grep -c '^        fn normalize_logical_ops(src: &str) -> String {' src/parser/mod.rs || true)
if [ "$nested_count" -ne 0 ]; then
  echo '[astclean-normalize-logical-ops] nested duplicate helper remains' >&2
  exit 1
fi

cargo test -q parser_loopclean_while_stage3_normalizes_to_loop_ast

echo '[astclean-normalize-logical-ops] OK'
