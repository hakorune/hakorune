#!/usr/bin/env bash
set -euo pipefail

targets=(
  src/tests/identical_exec_instance.rs
  src/tests/helpers/joinir_env.rs
  src/tests/helpers/joinir_frontend.rs
  src/tests/phase40_array_ext_filter_test.rs
  src/benchmarks/mod.rs
  src/runner/demos.rs
)
card='docs/development/current/main/phases/phase-293x/293x-301-ASTCLEAN-008-TEST-DEV-HELPER-DEAD-CODE-ALLOW-PRUNE.md'
ssot='docs/development/current/main/design/ast-cleanup-before-localtype-ssot.md'

if ! grep -q 'ASTCLEAN-008 test/dev helper dead_code allowance prune' "$ssot"; then
  echo '[astclean-test-dev-dead-code] missing SSOT row' >&2
  exit 1
fi

if ! grep -q '293x-301 ASTCLEAN-008 test/dev helper dead_code allow prune' "$card"; then
  echo '[astclean-test-dev-dead-code] missing phase card' >&2
  exit 1
fi

if rg -n '#\[!?allow\(dead_code\)\]' "${targets[@]}" | grep -v 'ASTCLEAN-008' >/dev/null; then
  echo '[astclean-test-dev-dead-code] bare test/dev dead_code allowance remains' >&2
  rg -n '#\[!?allow\(dead_code\)\]' "${targets[@]}" | grep -v 'ASTCLEAN-008' >&2
  exit 1
fi

if rg -n 'fn run_vm_benchmark' src/benchmarks/mod.rs >/dev/null; then
  echo '[astclean-test-dev-dead-code] legacy run_vm_benchmark stub remains' >&2
  exit 1
fi

count=$(rg -n '#\[allow\(dead_code\)\]' src | wc -l | tr -d ' ')
if [ "$count" -gt 202 ]; then
  echo "[astclean-test-dev-dead-code] expected source allowance count <= 202, got $count" >&2
  exit 1
fi

cargo test -q parser_loopclean_while_stage3_normalizes_to_loop_ast

echo "[astclean-test-dev-dead-code] OK source_count=$count"
