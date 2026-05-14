#!/usr/bin/env bash
set -euo pipefail

if ! grep -q 'ASTCLEAN-003 parser depth no-op hook removal' docs/development/current/main/design/ast-cleanup-before-localtype-ssot.md; then
  echo '[astclean-parser-depth] missing SSOT row' >&2
  exit 1
fi

if grep -R 'update_depth_before_advance\|update_depth_after_advance' src/parser >/dev/null; then
  echo '[astclean-parser-depth] legacy depth hook reference remains' >&2
  grep -R 'update_depth_before_advance\|update_depth_after_advance' src/parser >&2
  exit 1
fi

cargo test -q parser_loopclean_while_stage3_normalizes_to_loop_ast

echo '[astclean-parser-depth] OK'
