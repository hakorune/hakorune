#!/usr/bin/env bash
set -euo pipefail

card='docs/development/current/main/phases/phase-293x/293x-303-ASTCLEAN-010-RUNNER-BACKEND-JSON-BRIDGE-HELPER-PRUNE.md'
ssot='docs/development/current/main/design/ast-cleanup-before-localtype-ssot.md'

if ! grep -q 'ASTCLEAN-010 runner backend JSON bridge helper prune' "$ssot"; then
  echo '[astclean-runner-json-bridge] missing SSOT row' >&2
  exit 1
fi

if ! grep -q '293x-303 ASTCLEAN-010 runner backend JSON bridge helper prune' "$card"; then
  echo '[astclean-runner-json-bridge] missing phase card' >&2
  exit 1
fi

if rg -n 'has_numeric_core_boxcall|check_numeric_core_invariants' src/runner/mir_json_emit/helpers.rs >/dev/null; then
  echo '[astclean-runner-json-bridge] stale numeric-core JSON helper returned' >&2
  exit 1
fi

if rg -n 'pub\(crate\) fn emit_extern_call|pub\(crate\) fn emit_box_call' src/runner/mir_json_emit/emitters/calls.rs >/dev/null; then
  echo '[astclean-runner-json-bridge] stale MIR JSON call emitter wrapper returned' >&2
  exit 1
fi

if rg -n '^fn lower_expr\(|^fn lower_args\(|struct NoVars' src/runner/json_v0_bridge/lowering/expr.rs >/dev/null; then
  echo '[astclean-runner-json-bridge] stale JSON v0 no-vars lowering wrapper returned' >&2
  exit 1
fi

if rg -n 'parse_const_value|ConstValue' src/runner/json_v1_bridge/helpers.rs >/dev/null; then
  echo '[astclean-runner-json-bridge] stale JSON v1 const parser helper/import returned' >&2
  exit 1
fi

count=$(rg -n '#\[allow\(dead_code\)\]' src | wc -l | tr -d ' ')
if [ "$count" -gt 178 ]; then
  echo "[astclean-runner-json-bridge] expected source allowance count <= 178, got $count" >&2
  exit 1
fi

cargo test -q parser_loopclean_while_stage3_normalizes_to_loop_ast

echo "[astclean-runner-json-bridge] OK source_count=$count"
